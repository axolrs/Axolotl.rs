// Owner: PascalElixir / axolrs (GitHub org)
// File: Integration tests for the Gills analysis engine: positions, diagnostics, hover, completion, symbols, definition, references and rename.

use std::collections::HashMap;

use tower_lsp::lsp_types::{
    CompletionItem, CompletionItemKind, Diagnostic, Position, Range, SymbolKind, TextEdit, Url,
};

use axol_analyzer::analysis::{
    analyze, definition_in_docs, references_in_docs, rename_in_docs,
};
use axol_analyzer::line_index::LineIndex;
use axol_analyzer::Analysis;

const GAME_SRC: &str = "\
Player = struct
    hp: Int
end

fn damage(player: & var Player, amount: Int)
    player.hp = player.hp - amount
end

fn take(x: move Int)
    print(x)
end

fn main()
    let p = Player { hp = 100 }
    damage(p, 10)
    let a = 5
    take(a)
    print(a)
end
";

/// Build an LSP position.
fn p(line: u32, character: u32) -> Position {
    Position { line, character }
}

/// Build an LSP range from line/character pairs.
fn rng(start: (u32, u32), end: (u32, u32)) -> Range {
    Range { start: p(start.0, start.1), end: p(end.0, end.1) }
}

/// Parse a test file URI.
fn uri(path: &str) -> Url {
    Url::parse(path).unwrap()
}

/// Convert a line/character pair in an analysis to a byte offset.
fn offset_of(a: &Analysis, line: u32, character: u32) -> u32 {
    a.index.offset_of(&a.src, p(line, character))
}

/// Analyze the shared game document.
fn game() -> Analysis {
    analyze(GAME_SRC)
}

/// Find the diagnostic carrying the given code, if present.
fn find_code<'a>(diags: &'a [Diagnostic], code: &str) -> Option<&'a Diagnostic> {
    diags.iter().find(|d| {
        matches!(&d.code, Some(tower_lsp::lsp_types::NumberOrString::String(c)) if c == code)
    })
}

/// Find a completion item by label.
fn find_item<'a>(items: &'a [CompletionItem], label: &str) -> &'a CompletionItem {
    items.iter().find(|i| i.label == label).unwrap_or_else(|| panic!("missing item {label}"))
}

/// Byte offsets and LSP positions round-trip on a multi-line ASCII document.
#[test]
fn position_round_trip_multiline_ascii() {
    let idx = LineIndex::new(GAME_SRC);
    for offset in [0u32, 4, 19, 21, 50, 55, 68, 95, 130, 150] {
        if offset as usize > GAME_SRC.len() {
            continue;
        }
        let pos = idx.position_of(GAME_SRC, offset);
        assert_eq!(idx.offset_of(GAME_SRC, pos), offset);
    }
}

/// Multi-byte characters convert to UTF-16 code-unit columns.
#[test]
fn position_counts_utf16_units_for_multibyte() {
    let text = "let s = \"héllo→😀\"";
    let idx = LineIndex::new(text);
    assert_eq!(idx.position_of(text, text.find('é').unwrap() as u32), p(0, 10));
    assert_eq!(idx.position_of(text, text.find('→').unwrap() as u32), p(0, 14));
    let emoji = text.find('😀').unwrap() as u32;
    assert_eq!(idx.position_of(text, emoji), p(0, 15));
    assert_eq!(idx.offset_of(text, p(0, 17)), emoji + 4);
}

/// CRLF documents index the carriage return onto the earlier line.
#[test]
fn position_handles_crlf_line_breaks() {
    let text = "one\r\ntwo\r\nthree";
    let idx = LineIndex::new(text);
    assert_eq!(idx.line_count(), 3);
    assert_eq!(idx.position_of(text, 5), p(1, 0));
    assert_eq!(idx.position_of(text, 3), p(0, 3));
    assert_eq!(idx.offset_of(text, p(2, 0)), 10);
}

/// Positions in the analysis line index convert back to the same offsets.
#[test]
fn analysis_position_conversion_is_exact() {
    let a = game();
    assert_eq!(offset_of(&a, 4, 7), 40);
    assert_eq!(a.index.position_of(&a.src, 40), p(4, 7));
}

/// Parse errors surface as LSP diagnostics with correct ranges.
#[test]
fn diagnostics_carry_parse_error_range() {
    let a = analyze("fn main()\n    let x =\nend\n");
    assert_eq!(a.lsp_diagnostics().len(), 1);
    let d = &a.lsp_diagnostics()[0];
    assert_eq!(d.range, rng((2, 0), (2, 3)));
    assert_eq!(d.severity, Some(tower_lsp::lsp_types::DiagnosticSeverity::ERROR));
    assert_eq!(d.source.as_deref(), Some("axolc"));
    assert!(d.message.contains("KwEnd"));
}

/// Use-after-move (E0101) surfaces with its exact use-site range, code and note.
#[test]
fn diagnostics_report_e0101_use_after_move() {
    let a = game();
    let diags = a.lsp_diagnostics();
    let d = find_code(&diags, "E0101").expect("E0101 present");
    assert_eq!(d.range, rng((17, 10), (17, 11)));
    assert_eq!(d.severity, Some(tower_lsp::lsp_types::DiagnosticSeverity::ERROR));
    assert!(d.message.contains("use of moved value `a`"));
    assert!(d.message.contains("note: value moved into `take`"));
}

/// Writing through a `borrow` parameter surfaces E0104.
#[test]
fn diagnostics_report_e0104_write_through_borrow() {
    let a = analyze("fn f(x: borrow Int)\n    x = 1\nend");
    assert!(find_code(&a.lsp_diagnostics(), "E0104").is_some());
}

/// Passing a borrowed value to a move-taking function surfaces E0102.
#[test]
fn diagnostics_report_e0102_move_out_of_borrow() {
    let a = analyze("fn take(x: move Int)\n    print(x)\nend\n\nfn f(y: borrow Int)\n    take(y)\nend");
    assert!(find_code(&a.lsp_diagnostics(), "E0102").is_some());
}

/// A clean document produces no diagnostics.
#[test]
fn diagnostics_clean_document_is_empty() {
    let a = analyze("fn main()\n    print(1)\nend\n");
    assert!(a.lsp_diagnostics().is_empty());
    assert!(!a.has_errors());
}

/// The moved-value document is flagged as having errors.
#[test]
fn has_errors_detects_ownership_error() {
    assert!(game().has_errors());
}

/// The check report renders file:line:col with severity, code and notes.
#[test]
fn report_lines_render_location_and_code() {
    let a = game();
    let lines = a.report_lines("game.axol");
    assert_eq!(
        lines[0],
        "game.axol:18:11: error[E0101]: use of moved value `a`"
    );
    assert_eq!(
        lines[1],
        "  note: value moved into `take` earlier in this function"
    );
}

/// Hovering a function name renders its signature with resolved ownership modes.
#[test]
fn hover_over_function_shows_signature_with_modes() {
    let a = game();
    let hover = a.hover(offset_of(&a, 4, 7)).unwrap();
    match hover.contents {
        tower_lsp::lsp_types::HoverContents::Markup(m) => {
            assert_eq!(
                m.value,
                "```axol\nfn damage(player: &mut Player, amount: Int)\n```\n\n**ownership** - `player`: `&mut`, `amount`: value (Copy) (inferred `&`)"
            );
        }
        _ => panic!("expected markdown hover"),
    }
}

/// Hovering a mutated parameter renders its type and inferred `&mut` mode.
#[test]
fn hover_over_param_shows_inferred_mut() {
    let a = game();
    let hover = a.hover(offset_of(&a, 5, 7)).unwrap();
    match hover.contents {
        tower_lsp::lsp_types::HoverContents::Markup(m) => {
            assert_eq!(
                m.value,
                "```axol\nplayer: &mut Player\n```\n\n**ownership**: `&mut` - parameter of `damage`"
            );
        }
        _ => panic!("expected markdown hover"),
    }
    assert_eq!(hover.range, Some(rng((5, 4), (5, 10))));
}

/// Hovering a read-only non-primitive parameter renders a borrow mode.
#[test]
fn hover_over_readonly_param_shows_borrow() {
    let src = "Thing = struct\n    v: Int\nend\n\nfn peek(t: Thing)\n    print(t)\nend\n";
    let a = analyze(src);
    let hover = a.hover(offset_of(&a, 5, 10)).unwrap();
    match hover.contents {
        tower_lsp::lsp_types::HoverContents::Markup(m) => {
            assert!(m.value.contains("t: & Thing"));
            assert!(m.value.contains("**ownership**: `&`"));
        }
        _ => panic!("expected markdown hover"),
    }
}

/// Hovering a primitive parameter renders the Copy value mode plus the borrow hint.
#[test]
fn hover_over_primitive_param_shows_value_and_hint() {
    let a = game();
    let hover = a.hover(offset_of(&a, 4, 34)).unwrap();
    match hover.contents {
        tower_lsp::lsp_types::HoverContents::Markup(m) => {
            assert!(m.value.contains("amount: Int"));
            assert!(m.value.contains("**ownership**: value (Copy) (inferred `&`)"));
        }
        _ => panic!("expected markdown hover"),
    }
}

/// Hovering a struct name renders its fields.
#[test]
fn hover_over_struct_shows_fields() {
    let a = game();
    let hover = a.hover(offset_of(&a, 0, 1)).unwrap();
    match hover.contents {
        tower_lsp::lsp_types::HoverContents::Markup(m) => {
            assert!(m.value.contains("struct Player { hp: Int }"));
        }
        _ => panic!("expected markdown hover"),
    }
}

/// Hovering an enum name renders its variants.
#[test]
fn hover_over_enum_shows_variants() {
    let src = "Color = enum\n    Red\n    Green\n    Blue\nend\n\nfn main()\n    print(1)\nend\n";
    let a = analyze(src);
    let hover = a.hover(offset_of(&a, 0, 1)).unwrap();
    match hover.contents {
        tower_lsp::lsp_types::HoverContents::Markup(m) => {
            assert!(m.value.contains("enum Color { Red, Green, Blue }"));
        }
        _ => panic!("expected markdown hover"),
    }
}

/// Hovering a constant renders its type.
#[test]
fn hover_over_const_shows_type() {
    let src = "const MAX: Int = 100\n\nfn main()\n    print(MAX)\nend\n";
    let a = analyze(src);
    let hover = a.hover(offset_of(&a, 0, 7)).unwrap();
    match hover.contents {
        tower_lsp::lsp_types::HoverContents::Markup(m) => {
            assert!(m.value.contains("const MAX: Int"));
        }
        _ => panic!("expected markdown hover"),
    }
}

/// Hovering an interface renders its name.
#[test]
fn hover_over_interface_shows_name() {
    let src = "Shape = interface\n    area(self) -> Float\nend\n";
    let a = analyze(src);
    let hover = a.hover(offset_of(&a, 0, 1)).unwrap();
    match hover.contents {
        tower_lsp::lsp_types::HoverContents::Markup(m) => {
            assert!(m.value.contains("interface Shape"));
        }
        _ => panic!("expected markdown hover"),
    }
}

/// Hovering a method renders the receiver-qualified signature.
#[test]
fn hover_over_method_shows_receiver_qualified_signature() {
    let src = "\
Player = struct
    hp: Int
end

Player.attack = fn(self, damage: Int) -> Int
    self.hp = self.hp - damage
    return self.hp
end
";
    let a = analyze(src);
    let hover = a.hover(offset_of(&a, 4, 10)).unwrap();
    match hover.contents {
        tower_lsp::lsp_types::HoverContents::Markup(m) => {
            assert!(m.value.contains("fn Player.attack(self, damage: Int) -> Int"));
        }
        _ => panic!("expected markdown hover"),
    }
}

/// Hovering a non-declared identifier yields no hover.
#[test]
fn hover_over_builtin_is_none() {
    let a = game();
    assert!(a.hover(offset_of(&a, 9, 5)).is_none());
}

/// Hovering a non-identifier token yields no hover.
#[test]
fn hover_over_operator_is_none() {
    let a = game();
    assert!(a.hover(offset_of(&a, 4, 18)).is_none());
}

/// Completion offers every Axolotl keyword.
#[test]
fn completion_includes_all_keywords() {
    let a = game();
    let items = a.completions(offset_of(&a, 14, 6));
    let keywords: Vec<&str> = items
        .iter()
        .filter(|i| i.kind == Some(CompletionItemKind::KEYWORD))
        .map(|i| i.label.as_str())
        .collect();
    assert_eq!(keywords.len(), 34);
    for kw in ["fn", "let", "var", "repeat", "until", "impl", "spawn", "unsafe", "move"] {
        assert!(keywords.contains(&kw), "missing keyword {kw}");
    }
}

/// Completion offers the document's own items with proper kinds.
#[test]
fn completion_includes_document_items() {
    let a = game();
    let items = a.completions(offset_of(&a, 14, 6));
    assert_eq!(find_item(&items, "Player").kind, Some(CompletionItemKind::STRUCT));
    assert_eq!(find_item(&items, "damage").kind, Some(CompletionItemKind::FUNCTION));
    assert_eq!(find_item(&items, "take").kind, Some(CompletionItemKind::FUNCTION));
    assert_eq!(find_item(&items, "main").kind, Some(CompletionItemKind::FUNCTION));
}

/// Function completions carry the ownership-aware signature as detail.
#[test]
fn completion_fn_detail_is_signature() {
    let a = game();
    let items = a.completions(offset_of(&a, 14, 6));
    assert_eq!(
        find_item(&items, "damage").detail.as_deref(),
        Some("fn damage(player: &mut Player, amount: Int)")
    );
}

/// Completion text edits replace the partial word under the cursor.
#[test]
fn completion_text_edit_replaces_partial_word() {
    let a = game();
    let items = a.completions(offset_of(&a, 14, 6));
    let edit = match &find_item(&items, "damage").text_edit {
        Some(tower_lsp::lsp_types::CompletionTextEdit::Edit(e)) => e.clone(),
        _ => panic!("expected text edit"),
    };
    assert_eq!(edit.range, rng((14, 4), (14, 10)));
    assert_eq!(edit.new_text, "damage");
}

/// Completion at whitespace inserts at the cursor with an empty range.
#[test]
fn completion_at_whitespace_inserts_at_cursor() {
    let a = game();
    let items = a.completions(offset_of(&a, 1, 2));
    let edit = match &items[0].text_edit {
        Some(tower_lsp::lsp_types::CompletionTextEdit::Edit(e)) => e.clone(),
        _ => panic!("expected text edit"),
    };
    assert_eq!(edit.range, rng((1, 2), (1, 2)));
}

/// The identifier at the exact end of a word resolves (completion position).
#[test]
fn identifier_at_token_end_resolves() {
    let a = game();
    assert_eq!(a.identifier_at(offset_of(&a, 13, 9)).as_deref(), Some("p"));
}

/// Occurrences skip identifiers inside strings and comments.
#[test]
fn occurrences_skip_strings_and_comments() {
    let src = "-- damage is mentioned here\nfn damage(x: Int)\n    print(\"damage\")\nend\n";
    let a = analyze(src);
    assert_eq!(a.occurrences("damage").len(), 1);
}

/// Definition resolves a same-document usage to the declaration range.
#[test]
fn definition_resolves_within_document() {
    let a = game();
    let docs = vec![(uri("file:///game.axol"), GAME_SRC.to_string())];
    let loc = definition_in_docs(&docs, &uri("file:///game.axol"), offset_of(&a, 14, 7)).unwrap();
    assert_eq!(loc.uri, uri("file:///game.axol"));
    assert_eq!(loc.range, rng((4, 3), (4, 9)));
}

/// Definition resolves a usage in one document to a declaration in another.
#[test]
fn definition_resolves_across_documents() {
    let other = "fn helper()\n    damage(1, 2)\nend\n";
    let docs = vec![
        (uri("file:///game.axol"), GAME_SRC.to_string()),
        (uri("file:///other.axol"), other.to_string()),
    ];
    let other_analysis = analyze(other);
    let offset = other_analysis.index.offset_of(other, p(1, 7));
    let loc = definition_in_docs(&docs, &uri("file:///other.axol"), offset).unwrap();
    assert_eq!(loc.uri, uri("file:///game.axol"));
    assert_eq!(loc.range, rng((4, 3), (4, 9)));
}

/// Definition of a local variable resolves to its `let` declaration range.
#[test]
fn definition_of_local_returns_let_range() {
    let a = game();
    let docs = vec![(uri("file:///game.axol"), GAME_SRC.to_string())];
    let loc = definition_in_docs(&docs, &uri("file:///game.axol"), offset_of(&a, 13, 8)).unwrap();
    assert_eq!(loc.uri, uri("file:///game.axol"));
    assert_eq!(loc.range, rng((13, 8), (13, 9)));
}

/// References include the declaration and every whole-word usage across documents.
#[test]
fn references_include_declaration_across_documents() {
    let other = "fn helper()\n    damage(1, 2)\nend\n";
    let other_analysis = analyze(other);
    let offset = other_analysis.index.offset_of(other, p(1, 7));
    let docs = vec![
        (uri("file:///game.axol"), GAME_SRC.to_string()),
        (uri("file:///other.axol"), other.to_string()),
    ];
    let locs =
        references_in_docs(&docs, &uri("file:///other.axol"), offset, true).unwrap();
    assert_eq!(locs.len(), 3);
    assert!(locs.contains(&tower_lsp::lsp_types::Location {
        uri: uri("file:///game.axol"),
        range: rng((4, 3), (4, 9))
    }));
    assert!(locs.contains(&tower_lsp::lsp_types::Location {
        uri: uri("file:///game.axol"),
        range: rng((14, 4), (14, 10))
    }));
    assert!(locs.contains(&tower_lsp::lsp_types::Location {
        uri: uri("file:///other.axol"),
        range: rng((1, 4), (1, 10))
    }));
}

/// References without the declaration return only the usages.
#[test]
fn references_exclude_declaration() {
    let game_analysis = game();
    let docs = vec![(uri("file:///game.axol"), GAME_SRC.to_string())];
    let locs = references_in_docs(
        &docs,
        &uri("file:///game.axol"),
        offset_of(&game_analysis, 4, 7),
        false,
    )
    .unwrap();
    assert_eq!(locs.len(), 1);
    assert_eq!(locs[0].range, rng((14, 4), (14, 10)));
}

/// References of a local variable return its declaration and every resolved use.
#[test]
fn references_of_local_include_declaration_and_uses() {
    let a = game();
    let docs = vec![(uri("file:///game.axol"), GAME_SRC.to_string())];
    let locs = references_in_docs(&docs, &uri("file:///game.axol"), offset_of(&a, 13, 8), true)
        .unwrap();
    assert_eq!(locs.len(), 2);
    assert!(locs.contains(&tower_lsp::lsp_types::Location {
        uri: uri("file:///game.axol"),
        range: rng((13, 8), (13, 9))
    }));
    assert!(locs.contains(&tower_lsp::lsp_types::Location {
        uri: uri("file:///game.axol"),
        range: rng((14, 11), (14, 12))
    }));
}

/// Document symbols expose functions, structs, enums and consts with details.
#[test]
fn document_symbols_kinds_and_details() {
    let src = "\
Color = enum
    Red
    Green
end

const MAX: Int = 100

Player = struct
    hp: Int
end

fn main()
    print(1)
end
";
    let a = analyze(src);
    let symbols = a.document_symbols();
    assert_eq!(symbols.len(), 4);
    assert_eq!(symbols[0].name, "Color");
    assert_eq!(symbols[0].kind, SymbolKind::ENUM);
    assert_eq!(symbols[1].name, "MAX");
    assert_eq!(symbols[1].kind, SymbolKind::CONSTANT);
    assert_eq!(symbols[2].name, "Player");
    assert_eq!(symbols[2].kind, SymbolKind::STRUCT);
    assert_eq!(symbols[3].name, "main");
    assert_eq!(symbols[3].kind, SymbolKind::FUNCTION);
    assert_eq!(symbols[3].detail.as_deref(), Some("fn main()"));
}

/// Document symbol selection ranges point at the declaring identifier.
#[test]
fn document_symbols_selection_range_is_ident() {
    let a = game();
    let symbols = a.document_symbols();
    assert_eq!(symbols[0].name, "Player");
    assert_eq!(symbols[0].selection_range, rng((0, 0), (0, 6)));
    assert_eq!(symbols[1].name, "damage");
    assert_eq!(symbols[1].selection_range, rng((4, 3), (4, 9)));
}

/// Struct symbols carry their fields as children.
#[test]
fn document_symbols_struct_children_are_fields() {
    let a = game();
    let symbols = a.document_symbols();
    let children = symbols[0].children.as_ref().unwrap();
    assert_eq!(children.len(), 1);
    assert_eq!(children[0].name, "hp");
    assert_eq!(children[0].kind, SymbolKind::FIELD);
}

/// Enum symbols carry their variants as children.
#[test]
fn document_symbols_enum_children_are_variants() {
    let src = "Color = enum\n    Red\n    Green\nend\n";
    let a = analyze(src);
    let symbols = a.document_symbols();
    let children = symbols[0].children.as_ref().unwrap();
    let names: Vec<&str> = children.iter().map(|c| c.name.as_str()).collect();
    assert_eq!(names, vec!["Red", "Green"]);
    assert_eq!(children[0].kind, SymbolKind::ENUM_MEMBER);
}

/// Methods surface as METHOD symbols named receiver.name.
#[test]
fn document_symbols_method_symbol() {
    let src = "\
Player = struct
    hp: Int
end

Player.attack = fn(self, damage: Int) -> Int
    self.hp = self.hp - damage
    return self.hp
end
";
    let a = analyze(src);
    let symbols = a.document_symbols();
    assert_eq!(symbols.len(), 2);
    assert_eq!(symbols[1].name, "Player.attack");
    assert_eq!(symbols[1].kind, SymbolKind::METHOD);
    assert_eq!(symbols[1].selection_range, rng((4, 7), (4, 13)));
    assert!(symbols[1].detail.as_deref().unwrap().contains("fn Player.attack"));
}

/// Renaming produces one text edit per whole-word occurrence in the document.
#[test]
fn rename_edits_all_occurrences_in_document() {
    let game_analysis = game();
    let docs = vec![(uri("file:///game.axol"), GAME_SRC.to_string())];
    let edit =
        rename_in_docs(&docs, &uri("file:///game.axol"), offset_of(&game_analysis, 4, 7), "hurt")
            .unwrap();
    let changes = edit.changes.unwrap();
    let edits = changes.get(&uri("file:///game.axol")).unwrap();
    assert_eq!(edits.len(), 2);
    assert!(edits.contains(&TextEdit { range: rng((4, 3), (4, 9)), new_text: "hurt".into() }));
    assert!(edits.contains(&TextEdit { range: rng((14, 4), (14, 10)), new_text: "hurt".into() }));
}

/// Renaming spans documents with edits for each file.
#[test]
fn rename_edits_span_documents() {
    let other = "fn helper()\n    damage(1, 2)\nend\n";
    let other_analysis = analyze(other);
    let offset = other_analysis.index.offset_of(other, p(1, 7));
    let docs = vec![
        (uri("file:///game.axol"), GAME_SRC.to_string()),
        (uri("file:///other.axol"), other.to_string()),
    ];
    let edit = rename_in_docs(&docs, &uri("file:///other.axol"), offset, "hurt").unwrap();
    let changes = edit.changes.unwrap();
    assert_eq!(changes.len(), 2);
    assert_eq!(changes[&uri("file:///game.axol")].len(), 2);
    assert_eq!(changes[&uri("file:///other.axol")].len(), 1);
    assert_eq!(
        changes[&uri("file:///other.axol")][0].range,
        rng((1, 4), (1, 10))
    );
}

/// Renaming a local identifier edits exactly its declaration and resolved uses.
#[test]
fn rename_of_local_edits_declaration_and_uses() {
    let a = game();
    let docs = vec![(uri("file:///game.axol"), GAME_SRC.to_string())];
    let edit =
        rename_in_docs(&docs, &uri("file:///game.axol"), offset_of(&a, 13, 8), "hero").unwrap();
    let changes = edit.changes.unwrap();
    let edits = changes.get(&uri("file:///game.axol")).unwrap();
    assert_eq!(edits.len(), 2);
    assert!(edits.contains(&TextEdit { range: rng((13, 8), (13, 9)), new_text: "hero".into() }));
    assert!(edits.contains(&TextEdit { range: rng((14, 11), (14, 12)), new_text: "hero".into() }));
}

/// The keyword list is stable and deduplicated.
#[test]
fn keyword_list_is_unique() {
    let mut sorted = axol_analyzer::analysis::KEYWORDS.to_vec();
    sorted.sort_unstable();
    let before = sorted.len();
    sorted.dedup();
    assert_eq!(sorted.len(), before);
}

/// Analyze exposes the ownership table for downstream queries.
#[test]
fn analysis_exposes_ownership_modes() {
    let a = game();
    let modes = a.hir.ownership.modes_for("damage");
    assert_eq!(
        modes,
        vec![
            ("player".to_string(), axolc_core::ownership::ParamMode::BorrowMut),
            ("amount".to_string(), axolc_core::ownership::ParamMode::Value),
        ]
    );
}

/// Item declarations enumerate every top-level item with spans.
#[test]
fn item_decls_cover_all_items() {
    let a = game();
    let decls = a.item_decls();
    let names: Vec<&str> = decls.iter().map(|d| d.name.as_str()).collect();
    assert_eq!(names, vec!["Player", "damage", "take", "main"]);
    assert_eq!(decls[0].kind, axol_analyzer::analysis::DeclKind::Struct);
    assert_eq!(decls[1].kind, axol_analyzer::analysis::DeclKind::Fn);
    assert_eq!(decls[1].ident_span.start, 36);
}

/// The document snapshot map used by rename is keyed by URI.
#[test]
fn workspace_edit_serializes_changes_map() {
    let edit = tower_lsp::lsp_types::WorkspaceEdit {
        changes: Some(HashMap::from([(
            uri("file:///a.axol"),
            vec![TextEdit { range: rng((0, 0), (0, 1)), new_text: "b".into() }],
        )])),
        ..Default::default()
    };
    let json = serde_json::to_value(&edit).unwrap();
    assert!(json["changes"]["file:///a.axol"][0]["newText"].as_str() == Some("b"));
}
