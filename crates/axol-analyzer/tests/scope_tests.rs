// Owner: PascalElixir / axolrs (GitHub org)
// File: Scope-resolution integration tests for the Gills analysis engine: shadowing, locals, params, fields, methods, variants, closures and cross-document queries with exact position assertions.

use tower_lsp::lsp_types::{HoverContents, Position, Range, Url};

use axol_analyzer::analysis::{
    analyze, definition_in_docs, references_in_docs, rename_in_docs,
};
use axol_analyzer::scope::{Container, DefKind};
use axol_analyzer::Analysis;

const SHADOW_SRC: &str = "\
fn main()
    let x = 1
    if x > 0 then
        let x = 2
        print(x)
    end
    print(x)
end
";

const BRANCH_SRC: &str = "\
fn main()
    let y = 1
    if y > 0 then
        let y = 2
        print(y)
    elseif y < 0 then
        let y = 3
        print(y)
    else
        print(y)
    end
    print(y)
end
";

const WHILE_SRC: &str = "\
fn main()
    let w = 0
    while w < 3 do
        let w = w + 1
        print(w)
    end
    print(w)
end
";

const SEQ_SRC: &str = "\
fn main()
    let z = 1
    let z = z + 1
    print(z)
end
";

const PARAM_LOCAL_SRC: &str = "\
fn f(x: Int)
    print(x)
    let x = 2
    print(x)
end
";

const METHODS_SRC: &str = "\
Player = struct
    hp: Int
    name: String
end

Player.attack = fn(self, dmg: Int) -> Int
    self.hp = self.hp - dmg
    return self.hp
end

fn main()
    let hero = Player { hp = 30, name = \"Hero\" }
    let result = hero:attack(12)
    print(hero.hp)
    print(result)
end
";

const STATIC_SRC: &str = "\
Player = struct
    hp: Int
end

Player.new = fn(hp: Int) -> Player
    return Player { hp = hp }
end

fn main()
    let fresh = Player.new(5)
    print(fresh)
end
";

const DOT_METHOD_SRC: &str = "\
Enemy = struct
    hp: Int
end

Enemy.attack = fn(self, dmg: Int) -> Int
    return self.hp - dmg
end

fn main()
    let foe = Enemy { hp = 9 }
    let r = foe.attack(2)
    print(r)
end
";

const ENUM_SRC: &str = "\
Color = enum
    Red
    Green
end

fn main()
    let c = Color.Red
    match c
        Color.Red => print(\"r\")
        Green => print(\"g\")
        other => print(other)
    end
end
";

const CONST_SRC: &str = "\
const MAX: Int = 100

fn main()
    print(MAX)
end
";

const CLOSURE_SRC: &str = "\
fn main()
    let base = 10
    let add = fn(n: Int) -> Int
        let deep = fn(m: Int) -> Int
            return m + n + base
        end
        return deep(2)
    end
    print(add(1))
end
";

const IFACE_SRC: &str = "\
Shape = interface
    area(self) -> Float
end
";

const STRUCT_PATTERN_SRC: &str = "\
Point = struct
    x: Int
    y: Int
end

fn f(p: Point)
    match p.x
        { Point x = px, y } => print(px + y)
        _ => print(\"w\")
    end
end
";

const SAME_NAME_SRC: &str = "\
fn damage(x: Int)
    print(x)
end

fn main()
    let damage = 5
    print(damage)
end
";

const TWO_FN_SRC: &str = "\
fn a()
    let v = 1
    print(v)
end

fn b()
    let v = 2
    print(v)
end
";

const UNRESOLVED_SRC: &str = "\
fn main()
    mystery(1)
    print(mystery)
end
";

const FOR_SRC: &str = "\
fn main()
    for i in 0..3 do
        print(i)
    end
end
";

const FALLBACK_SRC: &str = "\
Solo = struct
    hp: Int
end

Solo.attack = fn(self, dmg: Int) -> Int
    return self.hp - dmg
end

fn poke(thing)
    thing:attack(1)
    print(thing.hp)
end
";

const AMBIG_SRC: &str = "\
Left = struct
    hp: Int
end

Right = struct
    hp: Int
end

Left.attack = fn(self, dmg: Int) -> Int
    return self.hp - dmg
end

Right.attack = fn(self, dmg: Int) -> Int
    return self.hp - dmg
end

fn poke(thing)
    thing:attack(1)
    print(thing.hp)
end
";

const CD_LOCAL_A: &str = "\
fn main()
    let p = 1
    print(p)
end
";

const CD_LOCAL_B: &str = "\
const p: Int = 2

fn use_it()
    print(p)
end
";

const CD_DEF_A: &str = "\
fn damage(x: Int)
    print(x)
end

fn main()
    damage(1)
end
";

const CD_DEF_B: &str = "fn damage(y: Int)\n    print(y)\nend\n";

/// Build an LSP position.
fn p(line: u32, character: u32) -> Position {
    Position { line, character }
}

/// Convert an LSP range back to byte offsets using the analysis line index.
fn bytes(a: &Analysis, r: &Range) -> (u32, u32) {
    (
        a.index.offset_of(&a.src, r.start),
        a.index.offset_of(&a.src, r.end),
    )
}

/// Return the byte offsets of every whole-word occurrence of a name.
fn occ(a: &Analysis, name: &str) -> Vec<(u32, u32)> {
    a.occurrences(name).iter().map(|r| bytes(a, r)).collect()
}

/// Return the byte offset of the first occurrence of a needle.
fn at(src: &str, needle: &str) -> u32 {
    src.find(needle).unwrap_or_else(|| panic!("missing {needle:?}")) as u32
}

/// Return the byte offset of the first occurrence of a needle at or after `from`.
fn at_from(src: &str, needle: &str, from: u32) -> u32 {
    src[from as usize..]
        .find(needle)
        .unwrap_or_else(|| panic!("missing {needle:?} after {from}")) as u32
        + from
}

/// Convert a cursor (line, character) in an analysis to a byte offset.
fn off(a: &Analysis, line: u32, character: u32) -> u32 {
    a.index.offset_of(&a.src, p(line, character))
}

/// Extract the markdown value of a hover.
fn hover_value(a: &Analysis, offset: u32) -> Option<String> {
    a.hover(offset).map(|h| match h.contents {
        HoverContents::Markup(m) => m.value,
        _ => panic!("expected markdown hover"),
    })
}

/// A multi-document fixture for cross-document queries.
struct Fixture {
    docs: Vec<(Url, String)>,
}

/// Build a fixture from (uri, source) pairs.
fn fx(pairs: &[(&str, &str)]) -> Fixture {
    Fixture {
        docs: pairs
            .iter()
            .map(|(u, s)| (Url::parse(u).unwrap(), s.to_string()))
            .collect(),
    }
}

impl Fixture {
    /// Resolve a definition to (uri, byte range).
    fn def(&self, uri: &str, offset: u32) -> Option<(String, (u32, u32))> {
        definition_in_docs(&self.docs, &Url::parse(uri).unwrap(), offset).map(|loc| {
            let text = self
                .docs
                .iter()
                .find(|(u, _)| *u == loc.uri)
                .map(|(_, t)| t.clone())
                .unwrap();
            let a = analyze(&text);
            (loc.uri.to_string(), bytes(&a, &loc.range))
        })
    }

    /// Resolve references to (uri, byte range) pairs.
    fn refs(&self, uri: &str, offset: u32, include: bool) -> Option<Vec<(String, (u32, u32))>> {
        references_in_docs(&self.docs, &Url::parse(uri).unwrap(), offset, include).map(|locs| {
            locs
                .iter()
                .map(|loc| {
                    let text = self
                        .docs
                        .iter()
                        .find(|(u, _)| *u == loc.uri)
                        .map(|(_, t)| t.clone())
                        .unwrap();
                    let a = analyze(&text);
                    (loc.uri.to_string(), bytes(&a, &loc.range))
                })
                .collect()
        })
    }

    /// Resolve a rename to per-uri edit ranges.
    fn renames(&self, uri: &str, offset: u32, new_name: &str) -> Option<Vec<(String, Vec<(u32, u32)>)>> {
        rename_in_docs(&self.docs, &Url::parse(uri).unwrap(), offset, new_name).map(|edit| {
            let mut out = Vec::new();
            for (u, edits) in edit.changes.unwrap() {
                let text = self
                    .docs
                    .iter()
                    .find(|(d, _)| *d == u)
                    .map(|(_, t)| t.clone())
                    .unwrap();
                let a = analyze(&text);
                let spans = edits.iter().map(|e| bytes(&a, &e.range)).collect();
                out.push((u.to_string(), spans));
            }
            out
        })
    }
}

/// Every test source parses without diagnostics, so spans are trustworthy.
#[test]
fn all_test_sources_parse_clean() {
    for (name, src) in [
        ("shadow", SHADOW_SRC),
        ("branch", BRANCH_SRC),
        ("while", WHILE_SRC),
        ("seq", SEQ_SRC),
        ("param_local", PARAM_LOCAL_SRC),
        ("methods", METHODS_SRC),
        ("static", STATIC_SRC),
        ("dot_method", DOT_METHOD_SRC),
        ("enum", ENUM_SRC),
        ("const", CONST_SRC),
        ("closure", CLOSURE_SRC),
        ("iface", IFACE_SRC),
        ("struct_pattern", STRUCT_PATTERN_SRC),
        ("same_name", SAME_NAME_SRC),
        ("two_fn", TWO_FN_SRC),
        ("unresolved", UNRESOLVED_SRC),
        ("for", FOR_SRC),
        ("fallback", FALLBACK_SRC),
        ("ambig", AMBIG_SRC),
        ("cd_local_a", CD_LOCAL_A),
        ("cd_local_b", CD_LOCAL_B),
        ("cd_def_a", CD_DEF_A),
        ("cd_def_b", CD_DEF_B),
    ] {
        let a = analyze(src);
        assert!(
            a.lsp_diagnostics().is_empty(),
            "{name} has diagnostics: {:?}",
            a.report_lines(name)
        );
    }
}

/// A use inside the shadowing block resolves to the inner let.
#[test]
fn shadow_definition_on_inner_use_returns_inner_let() {
    let a = analyze(SHADOW_SRC);
    let xs = occ(&a, "x");
    assert_eq!(xs.len(), 5);
    let range = a.definition_range(xs[3].0 + 1).unwrap();
    assert_eq!(bytes(&a, &range), xs[2]);
}

/// A use after the shadowing block resolves to the outer let.
#[test]
fn shadow_definition_on_outer_use_returns_outer_let() {
    let a = analyze(SHADOW_SRC);
    let xs = occ(&a, "x");
    let range = a.definition_range(xs[4].0 + 1).unwrap();
    assert_eq!(bytes(&a, &range), xs[0]);
}

/// The condition of the shadowing if resolves to the outer let.
#[test]
fn shadow_definition_on_condition_returns_outer_let() {
    let a = analyze(SHADOW_SRC);
    let xs = occ(&a, "x");
    let range = a.definition_range(xs[1].0 + 1).unwrap();
    assert_eq!(bytes(&a, &range), xs[0]);
}

/// Hovering the inner declaration token resolves to the inner binding itself.
#[test]
fn shadow_definition_on_inner_decl_returns_inner_decl() {
    let a = analyze(SHADOW_SRC);
    let xs = occ(&a, "x");
    let range = a.definition_range(xs[2].0 + 1).unwrap();
    assert_eq!(bytes(&a, &range), xs[2]);
}

/// References of the inner binding cover only the inner declaration and use.
#[test]
fn shadow_references_inner_exclude_outer() {
    let a = analyze(SHADOW_SRC);
    let xs = occ(&a, "x");
    let with_decl = a.reference_ranges(xs[3].0 + 1, true).unwrap();
    assert_eq!(with_decl.iter().map(|r| bytes(&a, r)).collect::<Vec<_>>(), vec![xs[2], xs[3]]);
    let without_decl = a.reference_ranges(xs[3].0 + 1, false).unwrap();
    assert_eq!(without_decl.iter().map(|r| bytes(&a, r)).collect::<Vec<_>>(), vec![xs[3]]);
}

/// References of the outer binding cover only the outer declaration, condition and use.
#[test]
fn shadow_references_outer_exclude_inner() {
    let a = analyze(SHADOW_SRC);
    let xs = occ(&a, "x");
    let ranges = a.reference_ranges(xs[4].0 + 1, true).unwrap();
    let got: Vec<(u32, u32)> = ranges.iter().map(|r| bytes(&a, r)).collect();
    assert_eq!(got, vec![xs[0], xs[1], xs[4]]);
}

/// Renaming the inner binding edits only the inner declaration and use.
#[test]
fn shadow_rename_inner_edits_only_inner() {
    let a = analyze(SHADOW_SRC);
    let xs = occ(&a, "x");
    let edits = a.rename_edits(xs[3].0 + 1, "y").unwrap();
    let got: Vec<(u32, u32)> = edits.iter().map(|e| bytes(&a, &e.range)).collect();
    assert_eq!(got, vec![xs[2], xs[3]]);
    assert!(edits.iter().all(|e| e.new_text == "y"));
}

/// Renaming the outer binding edits only the outer declaration, condition and use.
#[test]
fn shadow_rename_outer_edits_outer_only() {
    let a = analyze(SHADOW_SRC);
    let xs = occ(&a, "x");
    let edits = a.rename_edits(xs[4].0 + 1, "y").unwrap();
    let got: Vec<(u32, u32)> = edits.iter().map(|e| bytes(&a, &e.range)).collect();
    assert_eq!(got, vec![xs[0], xs[1], xs[4]]);
}

/// The resolved inner and outer uses carry Let kind with distinct containers.
#[test]
fn shadow_resolved_kinds_are_locals() {
    let a = analyze(SHADOW_SRC);
    let xs = occ(&a, "x");
    let inner = a.resolved_def(xs[3].0 + 1).unwrap();
    assert_eq!(inner.kind, DefKind::Let);
    assert_eq!(inner.name, "x");
    assert_eq!(inner.container, Container::Function("main".to_string()));
    let outer = a.resolved_def(xs[4].0 + 1).unwrap();
    assert_eq!(outer.kind, DefKind::Let);
    assert_ne!(a.binding_id_at(xs[3].0 + 1), a.binding_id_at(xs[4].0 + 1));
}

/// LSP line/character positions resolve the shadowing uses the same way.
#[test]
fn shadow_positions_by_lsp_line_and_character() {
    let a = analyze(SHADOW_SRC);
    let inner = a.definition_range(off(&a, 4, 14)).unwrap();
    assert_eq!(bytes(&a, &inner), (off(&a, 3, 12), off(&a, 3, 13)));
    let outer = a.definition_range(off(&a, 6, 10)).unwrap();
    assert_eq!(bytes(&a, &outer), (off(&a, 1, 8), off(&a, 1, 9)));
}

/// Cross-document definition of the shadowed inner use stays in its own document.
#[test]
fn shadow_definition_in_docs_inner() {
    let f = fx(&[("file:///shadow.axol", SHADOW_SRC)]);
    let a = analyze(SHADOW_SRC);
    let xs = occ(&a, "x");
    let got = f.def("file:///shadow.axol", xs[3].0 + 1).unwrap();
    assert_eq!(got, ("file:///shadow.axol".to_string(), xs[2]));
}

/// Cross-document rename of the inner binding edits only inner positions.
#[test]
fn shadow_rename_in_docs_inner_only() {
    let f = fx(&[("file:///shadow.axol", SHADOW_SRC)]);
    let a = analyze(SHADOW_SRC);
    let xs = occ(&a, "x");
    let got = f.renames("file:///shadow.axol", xs[3].0 + 1, "y").unwrap();
    assert_eq!(got.len(), 1);
    assert_eq!(got[0].0, "file:///shadow.axol");
    assert_eq!(got[0].1, vec![xs[2], xs[3]]);
}

/// Shadowing in then and elseif branches splits references per branch.
#[test]
fn branch_shadowing_splits_references() {
    let a = analyze(BRANCH_SRC);
    let ys = occ(&a, "y");
    assert_eq!(ys.len(), 9);
    let then_refs = a.reference_ranges(ys[3].0 + 1, true).unwrap();
    assert_eq!(
        then_refs.iter().map(|r| bytes(&a, r)).collect::<Vec<_>>(),
        vec![ys[2], ys[3]]
    );
    let elseif_refs = a.reference_ranges(ys[6].0 + 1, true).unwrap();
    assert_eq!(
        elseif_refs.iter().map(|r| bytes(&a, r)).collect::<Vec<_>>(),
        vec![ys[5], ys[6]]
    );
}

/// The elseif condition and else branch resolve to the outer binding.
#[test]
fn branch_condition_and_else_resolve_outer() {
    let a = analyze(BRANCH_SRC);
    let ys = occ(&a, "y");
    let cond = a.definition_range(ys[4].0 + 1).unwrap();
    assert_eq!(bytes(&a, &cond), ys[0]);
    let else_use = a.definition_range(ys[7].0 + 1).unwrap();
    assert_eq!(bytes(&a, &else_use), ys[0]);
    let ranges = a.reference_ranges(ys[7].0 + 1, true).unwrap();
    let got: Vec<(u32, u32)> = ranges.iter().map(|r| bytes(&a, r)).collect();
    assert_eq!(got, vec![ys[0], ys[1], ys[4], ys[7], ys[8]]);
}

/// Shadowing inside a while body splits references from the loop condition.
#[test]
fn while_shadowing_splits_references() {
    let a = analyze(WHILE_SRC);
    let ws = occ(&a, "w");
    assert_eq!(ws.len(), 6);
    let inner = a.definition_range(ws[4].0 + 1).unwrap();
    assert_eq!(bytes(&a, &inner), ws[2]);
    let body_refs = a.reference_ranges(ws[4].0 + 1, true).unwrap();
    assert_eq!(
        body_refs.iter().map(|r| bytes(&a, r)).collect::<Vec<_>>(),
        vec![ws[2], ws[4]]
    );
}

/// The initializer of the inner while-body let resolves to the outer binding.
#[test]
fn while_initializer_resolves_outer_binding() {
    let a = analyze(WHILE_SRC);
    let ws = occ(&a, "w");
    let rhs = a.definition_range(ws[3].0 + 1).unwrap();
    assert_eq!(bytes(&a, &rhs), ws[0]);
    let outer_refs = a.reference_ranges(ws[3].0 + 1, true).unwrap();
    assert_eq!(
        outer_refs.iter().map(|r| bytes(&a, r)).collect::<Vec<_>>(),
        vec![ws[0], ws[1], ws[3], ws[5]]
    );
}

/// Sequential lets with the same name in one scope shadow each other.
#[test]
fn sequential_lets_shadow_in_same_scope() {
    let a = analyze(SEQ_SRC);
    let zs = occ(&a, "z");
    assert_eq!(zs.len(), 4);
    let rhs = a.definition_range(zs[2].0 + 1).unwrap();
    assert_eq!(bytes(&a, &rhs), zs[0]);
    let later = a.definition_range(zs[3].0 + 1).unwrap();
    assert_eq!(bytes(&a, &later), zs[1]);
    let second_refs = a.reference_ranges(zs[3].0 + 1, true).unwrap();
    assert_eq!(
        second_refs.iter().map(|r| bytes(&a, r)).collect::<Vec<_>>(),
        vec![zs[1], zs[3]]
    );
}

/// A let local resolves with Let kind, inferred type and function container.
#[test]
fn let_local_resolves_with_kind_type_and_container() {
    let a = analyze(SHADOW_SRC);
    let xs = occ(&a, "x");
    let def = a.resolved_def(xs[0].0 + 1).unwrap();
    assert_eq!(def.kind, DefKind::Let);
    assert!(!def.is_var);
    assert_eq!(def.ty.as_deref(), Some("Int"));
    assert_eq!(def.container, Container::Function("main".to_string()));
}

/// A var local resolves as mutable and its hover says `var`.
#[test]
fn var_local_hover_shows_var_keyword() {
    let src = "fn main()\n    var total = 10\n    print(total)\nend\n";
    let a = analyze(src);
    let value = hover_value(&a, at(src, "var total") + 5).unwrap();
    assert_eq!(
        value,
        "```axol\ntotal: Int\n```\n\n**local** - `var` binding in `main`"
    );
}

/// A let local hover shows its inferred type and scope kind.
#[test]
fn let_local_hover_shows_type_and_scope() {
    let a = analyze(SHADOW_SRC);
    let xs = occ(&a, "x");
    let value = hover_value(&a, xs[4].0 + 1).unwrap();
    assert_eq!(
        value,
        "```axol\nx: Int\n```\n\n**local** - `let` binding in `main`"
    );
}

/// Assignment targets resolve to the assigned local.
#[test]
fn assignment_target_resolves_to_local() {
    let src = "fn main()\n    let count = 0\n    count = count + 1\n    print(count)\nend\n";
    let a = analyze(src);
    let cs = occ(&a, "count");
    assert_eq!(cs.len(), 4);
    let target = a.definition_range(cs[1].0 + 1).unwrap();
    assert_eq!(bytes(&a, &target), cs[0]);
    let refs = a.reference_ranges(cs[1].0 + 1, true).unwrap();
    assert_eq!(refs.iter().map(|r| bytes(&a, r)).collect::<Vec<_>>(), cs);
}

/// Uses inside interpolated strings resolve to the enclosing locals.
#[test]
fn interpolation_uses_resolve_to_locals() {
    let src = "fn main()\n    let hp = 9\n    let msg = \"hp ${hp}\"\n    print(msg)\nend\n";
    let a = analyze(src);
    let hps = occ(&a, "hp");
    assert_eq!(hps.len(), 2);
    let inner = a.definition_range(hps[1].0 + 1).unwrap();
    assert_eq!(bytes(&a, &inner), hps[0]);
    let value = hover_value(&a, hps[1].0 + 1).unwrap();
    assert!(value.contains("hp: Int"));
}

/// Hovering string text outside interpolation yields nothing.
#[test]
fn hover_on_string_text_is_none() {
    let src = "fn main()\n    let msg = \"plain text\"\nend\n";
    let a = analyze(src);
    let quote = at(src, "\"plain");
    assert!(a.hover(quote + 3).is_none());
}

/// The for-loop variable binds its body uses.
#[test]
fn for_loop_variable_binds_body_uses() {
    let a = analyze(FOR_SRC);
    let is = occ(&a, "i");
    assert_eq!(is.len(), 2);
    let use_range = a.definition_range(is[1].0 + 1).unwrap();
    assert_eq!(bytes(&a, &use_range), is[0]);
    let refs = a.reference_ranges(is[1].0 + 1, true).unwrap();
    assert_eq!(refs.iter().map(|r| bytes(&a, r)).collect::<Vec<_>>(), is);
    let value = hover_value(&a, is[1].0 + 1).unwrap();
    assert_eq!(
        value,
        "```axol\ni\n```\n\n**local** - `let` binding in `main`"
    );
}

/// A match-arm binding binds its arm body and hover shows the local line.
#[test]
fn match_arm_binding_binds_body_uses() {
    let a = analyze(ENUM_SRC);
    let others = occ(&a, "other");
    assert_eq!(others.len(), 2);
    let use_range = a.definition_range(others[1].0 + 1).unwrap();
    assert_eq!(bytes(&a, &use_range), others[0]);
    let refs = a.reference_ranges(others[1].0 + 1, true).unwrap();
    assert_eq!(refs.iter().map(|r| bytes(&a, r)).collect::<Vec<_>>(), others);
}

/// A sub-pattern binding of an enum variant pattern binds its uses.
#[test]
fn enum_pattern_sub_binding_resolves() {
    let src = "Color = enum\n    Green\n    Rgb(Int, Int, Int)\nend\n\nfn f()\n    match Color.Rgb\n        Color.Rgb(r, g, b) => print(r + g + b)\n    end\nend\n";
    let a = analyze(src);
    let rs = occ(&a, "r");
    assert_eq!(rs.len(), 2);
    let use_range = a.definition_range(rs[1].0 + 1).unwrap();
    assert_eq!(bytes(&a, &use_range), rs[0]);
}

/// A fn parameter resolves with Param kind and its annotated base type.
#[test]
fn param_resolves_with_kind_and_type() {
    let a = analyze(METHODS_SRC);
    let dmgs = occ(&a, "dmg");
    assert_eq!(dmgs.len(), 2);
    let def = a.resolved_def(dmgs[1].0 + 1).unwrap();
    assert_eq!(def.kind, DefKind::Param);
    assert_eq!(def.ty.as_deref(), Some("Int"));
    assert_eq!(
        def.container,
        Container::Method { receiver: "Player".to_string(), name: "attack".to_string() }
    );
}

/// A parameter use resolves to the parameter declaration.
#[test]
fn param_definition_returns_param_decl() {
    let a = analyze(METHODS_SRC);
    let dmgs = occ(&a, "dmg");
    let range = a.definition_range(dmgs[1].0 + 1).unwrap();
    assert_eq!(bytes(&a, &range), dmgs[0]);
}

/// A method parameter hover shows its type and owning method.
#[test]
fn method_param_hover_shows_parameter_of_method() {
    let a = analyze(METHODS_SRC);
    let dmgs = occ(&a, "dmg");
    let value = hover_value(&a, dmgs[1].0 + 1).unwrap();
    assert_eq!(
        value,
        "```axol\ndmg: Int\n```\n\n**parameter** of `Player.attack`"
    );
}

/// A same-named parameter and local stay separate bindings.
#[test]
fn param_and_same_named_local_stay_separate() {
    let a = analyze(PARAM_LOCAL_SRC);
    let xs = occ(&a, "x");
    assert_eq!(xs.len(), 4);
    let early = a.definition_range(xs[1].0 + 1).unwrap();
    assert_eq!(bytes(&a, &early), xs[0]);
    let late = a.definition_range(xs[3].0 + 1).unwrap();
    assert_eq!(bytes(&a, &late), xs[2]);
    let param_refs = a.reference_ranges(xs[1].0 + 1, true).unwrap();
    assert_eq!(
        param_refs.iter().map(|r| bytes(&a, r)).collect::<Vec<_>>(),
        vec![xs[0], xs[1]]
    );
    let local_refs = a.reference_ranges(xs[3].0 + 1, true).unwrap();
    assert_eq!(
        local_refs.iter().map(|r| bytes(&a, r)).collect::<Vec<_>>(),
        vec![xs[2], xs[3]]
    );
}

/// The fn parameter hover preserves the resolved ownership-mode format.
#[test]
fn fn_param_hover_preserves_ownership_mode() {
    let src = "Player = struct\n    hp: Int\nend\n\nfn damage(player: & var Player, amount: Int)\n    player.hp = player.hp - amount\nend\n";
    let a = analyze(src);
    let players = occ(&a, "player");
    let value = hover_value(&a, players[1].0 + 1).unwrap();
    assert_eq!(
        value,
        "```axol\nplayer: &mut Player\n```\n\n**ownership**: `&mut` - parameter of `damage`"
    );
}

/// The `self` expression resolves to the method's self parameter.
#[test]
fn self_expr_resolves_to_self_param() {
    let a = analyze(METHODS_SRC);
    let selves = occ(&a, "self");
    assert_eq!(selves.len(), 4);
    let def = a.resolved_def(selves[1].0 + 2).unwrap();
    assert_eq!(def.kind, DefKind::Param);
    assert!(def.self_param);
    assert_eq!(def.ty.as_deref(), Some("Player"));
    let range = a.definition_range(selves[3].0 + 2).unwrap();
    assert_eq!(bytes(&a, &range), selves[0]);
}

/// The `self` hover shows the receiver type and owning method.
#[test]
fn self_hover_shows_receiver_of_method() {
    let a = analyze(METHODS_SRC);
    let selves = occ(&a, "self");
    let value = hover_value(&a, selves[1].0 + 2).unwrap();
    assert_eq!(
        value,
        "```axol\nself: Player\n```\n\n**receiver** of `Player.attack`"
    );
}

/// `self.hp` resolves to the struct field declaration.
#[test]
fn self_field_resolves_to_struct_field() {
    let a = analyze(METHODS_SRC);
    let hps = occ(&a, "hp");
    assert_eq!(hps.len(), 6);
    for idx in [1, 2, 3] {
        let range = a.definition_range(hps[idx].0 + 1).unwrap();
        assert_eq!(bytes(&a, &range), hps[0], "field use {idx} must resolve to the field decl");
    }
}

/// A struct-literal field key resolves to the struct field declaration.
#[test]
fn struct_literal_field_key_resolves_to_field() {
    let a = analyze(METHODS_SRC);
    let hps = occ(&a, "hp");
    let key = a.definition_range(hps[4].0 + 1).unwrap();
    assert_eq!(bytes(&a, &key), hps[0]);
}

/// A struct-literal type name resolves to the struct declaration.
#[test]
fn struct_literal_type_resolves_to_struct() {
    let a = analyze(METHODS_SRC);
    let players = occ(&a, "Player");
    assert_eq!(players.len(), 3);
    let lit = a.definition_range(players[2].0 + 2).unwrap();
    assert_eq!(bytes(&a, &lit), players[0]);
    let def = a.resolved_def(players[2].0 + 2).unwrap();
    assert_eq!(def.kind, DefKind::Struct);
}

/// A field read on a typed local resolves to the struct field declaration.
#[test]
fn field_read_on_local_resolves_to_field() {
    let a = analyze(METHODS_SRC);
    let hps = occ(&a, "hp");
    let read = a.definition_range(hps[5].0 + 1).unwrap();
    assert_eq!(bytes(&a, &read), hps[0]);
}

/// A field hover shows the field's type and the struct it belongs to.
#[test]
fn field_hover_shows_struct_owner() {
    let a = analyze(METHODS_SRC);
    let hps = occ(&a, "hp");
    let value = hover_value(&a, hps[1].0 + 1).unwrap();
    assert_eq!(
        value,
        "```axol\nhp: Int\n```\n\n**field** of struct `Player`"
    );
}

/// Field references cover the declaration, self reads/writes, literal keys and local reads.
#[test]
fn field_references_cover_all_field_uses() {
    let a = analyze(METHODS_SRC);
    let hps = occ(&a, "hp");
    let ranges = a.reference_ranges(hps[1].0 + 1, true).unwrap();
    assert_eq!(ranges.iter().map(|r| bytes(&a, r)).collect::<Vec<_>>(), hps);
}

/// Renaming a field edits the declaration and every field use.
#[test]
fn field_rename_edits_all_field_uses() {
    let a = analyze(METHODS_SRC);
    let hps = occ(&a, "hp");
    let edits = a.rename_edits(hps[1].0 + 1, "health").unwrap();
    let got: Vec<(u32, u32)> = edits.iter().map(|e| bytes(&a, &e.range)).collect();
    assert_eq!(got, hps);
    assert!(edits.iter().all(|e| e.new_text == "health"));
}

/// A colon method call resolves to the method declaration through receiver type inference.
#[test]
fn colon_method_call_resolves_to_method_decl() {
    let a = analyze(METHODS_SRC);
    let attacks = occ(&a, "attack");
    assert_eq!(attacks.len(), 2);
    let range = a.definition_range(attacks[1].0 + 1).unwrap();
    assert_eq!(bytes(&a, &range), attacks[0]);
    let def = a.resolved_def(attacks[1].0 + 1).unwrap();
    assert_eq!(def.kind, DefKind::Method);
    assert_eq!(def.owner.as_deref(), Some("Player"));
}

/// A dot method call resolves to the method declaration as well.
#[test]
fn dot_method_call_resolves_to_method_decl() {
    let a = analyze(DOT_METHOD_SRC);
    let attacks = occ(&a, "attack");
    assert_eq!(attacks.len(), 2);
    let range = a.definition_range(attacks[1].0 + 1).unwrap();
    assert_eq!(bytes(&a, &range), attacks[0]);
}

/// A method-call hover shows the receiver-qualified signature.
#[test]
fn method_call_hover_shows_signature() {
    let a = analyze(METHODS_SRC);
    let attacks = occ(&a, "attack");
    let value = hover_value(&a, attacks[1].0 + 1).unwrap();
    assert_eq!(value, "```axol\nfn Player.attack(self, dmg: Int) -> Int\n```");
}

/// Method references cover the declaration and every call site.
#[test]
fn method_references_cover_calls() {
    let a = analyze(METHODS_SRC);
    let attacks = occ(&a, "attack");
    let ranges = a.reference_ranges(attacks[1].0 + 1, true).unwrap();
    assert_eq!(ranges.iter().map(|r| bytes(&a, r)).collect::<Vec<_>>(), attacks);
}

/// Renaming a method edits the declaration and the call site.
#[test]
fn method_rename_edits_decl_and_calls() {
    let a = analyze(METHODS_SRC);
    let attacks = occ(&a, "attack");
    let edits = a.rename_edits(attacks[1].0 + 1, "strike").unwrap();
    let got: Vec<(u32, u32)> = edits.iter().map(|e| bytes(&a, &e.range)).collect();
    assert_eq!(got, attacks);
}

/// A local's type is inferred from a struct literal.
#[test]
fn local_type_inferred_from_struct_literal() {
    let a = analyze(METHODS_SRC);
    let heroes = occ(&a, "hero");
    let def = a.resolved_def(heroes[0].0 + 1).unwrap();
    assert_eq!(def.ty.as_deref(), Some("Player"));
    let value = hover_value(&a, heroes[0].0 + 1).unwrap();
    assert_eq!(
        value,
        "```axol\nhero: Player\n```\n\n**local** - `let` binding in `main`"
    );
}

/// A local's type is inferred from a method call's return type.
#[test]
fn local_type_inferred_from_method_return() {
    let a = analyze(METHODS_SRC);
    let results = occ(&a, "result");
    let def = a.resolved_def(results[0].0 + 1).unwrap();
    assert_eq!(def.ty.as_deref(), Some("Int"));
}

/// A static method call through the struct name resolves to the static method.
#[test]
fn static_method_call_resolves_to_static_method() {
    let a = analyze(STATIC_SRC);
    let news = occ(&a, "new");
    assert_eq!(news.len(), 2);
    let range = a.definition_range(news[1].0 + 1).unwrap();
    assert_eq!(bytes(&a, &range), news[0]);
    let def = a.resolved_def(news[1].0 + 1).unwrap();
    assert_eq!(def.kind, DefKind::Method);
    assert_eq!(def.owner.as_deref(), Some("Player"));
}

/// A static-method call infers the returned struct type for its local.
#[test]
fn static_method_return_infers_local_type() {
    let a = analyze(STATIC_SRC);
    let freshes = occ(&a, "fresh");
    let def = a.resolved_def(freshes[0].0 + 1).unwrap();
    assert_eq!(def.ty.as_deref(), Some("Player"));
}

/// A struct-literal key in a static method resolves to the field, its value to the parameter.
#[test]
fn static_method_literal_key_and_param_separate() {
    let a = analyze(STATIC_SRC);
    let hps = occ(&a, "hp");
    assert_eq!(hps.len(), 4);
    let key = a.definition_range(hps[2].0 + 1).unwrap();
    assert_eq!(bytes(&a, &key), hps[0]);
    let value_use = a.definition_range(hps[3].0 + 1).unwrap();
    assert_eq!(bytes(&a, &value_use), hps[1]);
}

/// An enum variant use in an expression resolves to the variant declaration.
#[test]
fn enum_variant_expression_resolves_to_variant() {
    let a = analyze(ENUM_SRC);
    let reds = occ(&a, "Red");
    assert_eq!(reds.len(), 3);
    let range = a.definition_range(reds[1].0 + 1).unwrap();
    assert_eq!(bytes(&a, &range), reds[0]);
    let def = a.resolved_def(reds[1].0 + 1).unwrap();
    assert_eq!(def.kind, DefKind::Variant);
    assert_eq!(def.owner.as_deref(), Some("Color"));
}

/// An enum variant use in a pattern resolves to the variant declaration.
#[test]
fn enum_variant_pattern_resolves_to_variant() {
    let a = analyze(ENUM_SRC);
    let reds = occ(&a, "Red");
    let pattern = a.definition_range(reds[2].0 + 1).unwrap();
    assert_eq!(bytes(&a, &pattern), reds[0]);
}

/// The enum name in `Color.Red` resolves to the enum declaration.
#[test]
fn enum_name_use_resolves_to_enum() {
    let a = analyze(ENUM_SRC);
    let colors = occ(&a, "Color");
    assert_eq!(colors.len(), 3);
    for idx in [1, 2] {
        let range = a.definition_range(colors[idx].0 + 1).unwrap();
        assert_eq!(bytes(&a, &range), colors[0]);
    }
}

/// A variant hover shows the variant and its enum.
#[test]
fn variant_hover_shows_enum_owner() {
    let a = analyze(ENUM_SRC);
    let reds = occ(&a, "Red");
    let value = hover_value(&a, reds[1].0 + 1).unwrap();
    assert_eq!(
        value,
        "```axol\nRed\n```\n\n**variant** of enum `Color`"
    );
}

/// A same-named variant and pattern binding stay separate.
#[test]
fn variant_and_pattern_binding_stay_separate() {
    let a = analyze(ENUM_SRC);
    let greens = occ(&a, "Green");
    assert_eq!(greens.len(), 2);
    let def = a.resolved_def(greens[1].0 + 1).unwrap();
    assert_eq!(def.kind, DefKind::Let);
    let variant = a.resolved_def(greens[0].0 + 1).unwrap();
    assert_eq!(variant.kind, DefKind::Variant);
}

/// Variant references cover the declaration and both use forms.
#[test]
fn variant_references_cover_expression_and_pattern() {
    let a = analyze(ENUM_SRC);
    let reds = occ(&a, "Red");
    let ranges = a.reference_ranges(reds[1].0 + 1, true).unwrap();
    assert_eq!(ranges.iter().map(|r| bytes(&a, r)).collect::<Vec<_>>(), reds);
}

/// A const use resolves to the const declaration and hovers its signature.
#[test]
fn const_use_resolves_to_const_decl() {
    let a = analyze(CONST_SRC);
    let maxes = occ(&a, "MAX");
    assert_eq!(maxes.len(), 2);
    let range = a.definition_range(maxes[1].0 + 1).unwrap();
    assert_eq!(bytes(&a, &range), maxes[0]);
    let def = a.resolved_def(maxes[1].0 + 1).unwrap();
    assert_eq!(def.kind, DefKind::Const);
    let value = hover_value(&a, maxes[1].0 + 1).unwrap();
    assert_eq!(value, "```axol\nconst MAX: Int\n```");
}

/// A type annotation use resolves to the struct declaration.
#[test]
fn type_annotation_use_resolves_to_struct() {
    let src = "Point = struct\n    x: Int\n    y: Int\nend\n\nfn f(p: Point)\n    print(p.x)\nend\n";
    let a = analyze(src);
    let points = occ(&a, "Point");
    assert_eq!(points.len(), 2);
    let range = a.definition_range(points[1].0 + 1).unwrap();
    assert_eq!(bytes(&a, &range), points[0]);
    let def = a.resolved_def(points[1].0 + 1).unwrap();
    assert_eq!(def.kind, DefKind::Struct);
}

/// Struct references include type annotations and literal uses.
#[test]
fn struct_references_include_type_uses() {
    let src = "Point = struct\n    x: Int\n    y: Int\nend\n\nfn f(p: Point)\n    let q = Point { x = 1, y = 2 }\n    print(q.x)\nend\n";
    let a = analyze(src);
    let points = occ(&a, "Point");
    assert_eq!(points.len(), 3);
    let ranges = a.reference_ranges(points[1].0 + 1, true).unwrap();
    assert_eq!(ranges.iter().map(|r| bytes(&a, r)).collect::<Vec<_>>(), points);
}

/// A closure parameter resolves with Closure container.
#[test]
fn closure_param_resolves_with_closure_container() {
    let a = analyze(CLOSURE_SRC);
    let ns = occ(&a, "n");
    assert_eq!(ns.len(), 2);
    let def = a.resolved_def(ns[1].0 + 1).unwrap();
    assert_eq!(def.kind, DefKind::Param);
    assert_eq!(def.container, Container::Closure);
    let range = a.definition_range(ns[1].0 + 1).unwrap();
    assert_eq!(bytes(&a, &range), ns[0]);
}

/// A closure parameter hover says it belongs to a closure.
#[test]
fn closure_param_hover_says_closure() {
    let a = analyze(CLOSURE_SRC);
    let ns = occ(&a, "n");
    let value = hover_value(&a, ns[1].0 + 1).unwrap();
    assert_eq!(
        value,
        "```axol\nn: Int\n```\n\n**parameter** of a closure"
    );
}

/// A use inside a nested closure resolves transitively to the outer local.
#[test]
fn nested_closure_captures_outer_local() {
    let a = analyze(CLOSURE_SRC);
    let bases = occ(&a, "base");
    assert_eq!(bases.len(), 2);
    let range = a.definition_range(bases[1].0 + 1).unwrap();
    assert_eq!(bytes(&a, &range), bases[0]);
    let def = a.resolved_def(bases[1].0 + 1).unwrap();
    assert_eq!(def.container, Container::Function("main".to_string()));
}

/// A local declared inside a closure has the Closure container in hover.
#[test]
fn closure_local_hover_says_closure() {
    let a = analyze(CLOSURE_SRC);
    let deeps = occ(&a, "deep");
    assert_eq!(deeps.len(), 2);
    let value = hover_value(&a, deeps[0].0 + 1).unwrap();
    assert_eq!(
        value,
        "```axol\ndeep\n```\n\n**local** - `let` binding in a closure"
    );
}

/// Calls of closure locals resolve to the closure bindings.
#[test]
fn closure_call_resolves_to_closure_local() {
    let a = analyze(CLOSURE_SRC);
    let adds = occ(&a, "add");
    assert_eq!(adds.len(), 2);
    let range = a.definition_range(adds[1].0 + 1).unwrap();
    assert_eq!(bytes(&a, &range), adds[0]);
    let def = a.resolved_def(adds[1].0 + 1).unwrap();
    assert_eq!(def.kind, DefKind::Let);
}

/// An interface method declaration resolves with Method kind owned by the interface.
#[test]
fn interface_method_decl_resolves() {
    let a = analyze(IFACE_SRC);
    let areas = occ(&a, "area");
    assert_eq!(areas.len(), 1);
    let def = a.resolved_def(areas[0].0 + 1).unwrap();
    assert_eq!(def.kind, DefKind::Method);
    assert_eq!(def.owner.as_deref(), Some("Shape"));
}

/// An interface method hover shows the interface-qualified signature.
#[test]
fn interface_method_hover_shows_signature() {
    let a = analyze(IFACE_SRC);
    let areas = occ(&a, "area");
    let value = hover_value(&a, areas[0].0 + 1).unwrap();
    assert_eq!(value, "```axol\nfn Shape.area(self) -> Float\n```");
}

/// The self parameter of an interface method resolves as the receiver.
#[test]
fn interface_self_param_resolves() {
    let a = analyze(IFACE_SRC);
    let selves = occ(&a, "self");
    let def = a.resolved_def(selves[0].0 + 1).unwrap();
    assert_eq!(def.kind, DefKind::Param);
    assert!(def.self_param);
    assert_eq!(
        def.container,
        Container::Method { receiver: "Shape".to_string(), name: "area".to_string() }
    );
}

/// A struct pattern field key resolves to the struct field declaration.
#[test]
fn struct_pattern_field_resolves_to_field_decl() {
    let a = analyze(STRUCT_PATTERN_SRC);
    let xs = occ(&a, "x");
    assert_eq!(xs.len(), 3);
    let pattern_key = a.definition_range(xs[1].0 + 1).unwrap();
    assert_eq!(bytes(&a, &pattern_key), xs[0]);
    let field_read = a.definition_range(xs[2].0 + 1).unwrap();
    assert_eq!(bytes(&a, &field_read), xs[0]);
}

/// A struct pattern shorthand field is a binding, not the field.
#[test]
fn struct_pattern_shorthand_is_binding() {
    let a = analyze(STRUCT_PATTERN_SRC);
    let ys = occ(&a, "y");
    assert_eq!(ys.len(), 3);
    let shorthand = a.resolved_def(ys[1].0 + 1).unwrap();
    assert_eq!(shorthand.kind, DefKind::Let);
    let use_def = a.definition_range(ys[2].0 + 1).unwrap();
    assert_eq!(bytes(&a, &use_def), ys[1]);
}

/// A same-named function and local stay separate bindings.
#[test]
fn same_name_fn_and_local_stay_separate() {
    let a = analyze(SAME_NAME_SRC);
    let damages = occ(&a, "damage");
    assert_eq!(damages.len(), 3);
    let use_def = a.definition_range(damages[2].0 + 1).unwrap();
    assert_eq!(bytes(&a, &use_def), damages[1]);
    let use_kind = a.resolved_def(damages[2].0 + 1).unwrap().kind;
    assert_eq!(use_kind, DefKind::Let);
    let fn_kind = a.resolved_def(damages[0].0 + 1).unwrap().kind;
    assert_eq!(fn_kind, DefKind::Fn);
    let local_refs = a.reference_ranges(damages[2].0 + 1, true).unwrap();
    assert_eq!(
        local_refs.iter().map(|r| bytes(&a, r)).collect::<Vec<_>>(),
        vec![damages[1], damages[2]]
    );
    let fn_refs = a.reference_ranges(damages[0].0 + 1, true).unwrap();
    assert_eq!(
        fn_refs.iter().map(|r| bytes(&a, r)).collect::<Vec<_>>(),
        vec![damages[0]]
    );
}

/// Renaming the same-named local does not touch the function declaration.
#[test]
fn same_name_rename_local_excludes_fn() {
    let a = analyze(SAME_NAME_SRC);
    let damages = occ(&a, "damage");
    let edits = a.rename_edits(damages[2].0 + 1, "hurt").unwrap();
    let got: Vec<(u32, u32)> = edits.iter().map(|e| bytes(&a, &e.range)).collect();
    assert_eq!(got, vec![damages[1], damages[2]]);
}

/// Same-named locals in different functions stay separate.
#[test]
fn same_name_locals_in_two_fns_stay_separate() {
    let a = analyze(TWO_FN_SRC);
    let vs = occ(&a, "v");
    assert_eq!(vs.len(), 4);
    let a_use = a.definition_range(vs[1].0 + 1).unwrap();
    assert_eq!(bytes(&a, &a_use), vs[0]);
    let b_use = a.definition_range(vs[3].0 + 1).unwrap();
    assert_eq!(bytes(&a, &b_use), vs[2]);
    let a_refs = a.reference_ranges(vs[1].0 + 1, true).unwrap();
    assert_eq!(a_refs.iter().map(|r| bytes(&a, r)).collect::<Vec<_>>(), vec![vs[0], vs[1]]);
    let b_refs = a.reference_ranges(vs[3].0 + 1, true).unwrap();
    assert_eq!(b_refs.iter().map(|r| bytes(&a, r)).collect::<Vec<_>>(), vec![vs[2], vs[3]]);
}

/// Unresolvable identifiers yield no definition, references, rename or hover.
#[test]
fn unresolvable_identifier_yields_nothing() {
    let a = analyze(UNRESOLVED_SRC);
    let mystery = at(UNRESOLVED_SRC, "mystery");
    assert!(a.binding_id_at(mystery + 1).is_none());
    assert!(a.definition_range(mystery + 1).is_none());
    assert!(a.reference_ranges(mystery + 1, true).is_none());
    assert!(a.rename_edits(mystery + 1, "x").is_none());
    assert!(hover_value(&a, mystery + 1).is_none());
    let f = fx(&[("file:///u.axol", UNRESOLVED_SRC)]);
    assert!(f.def("file:///u.axol", mystery + 1).is_none());
    assert!(f.refs("file:///u.axol", mystery + 1, true).is_none());
    assert!(f.renames("file:///u.axol", mystery + 1, "x").is_none());
}

/// Unresolved identifiers fall back to a declaration in another open document.
#[test]
fn unresolvable_falls_back_to_other_document() {
    let f = fx(&[
        ("file:///use.axol", "fn helper()\n    damage(1)\nend\n"),
        ("file:///decl.axol", "fn damage(amount: Int)\n    print(amount)\nend\n"),
    ]);
    let use_src = "fn helper()\n    damage(1)\nend\n";
    let decl_src = "fn damage(amount: Int)\n    print(amount)\nend\n";
    let got = f.def("file:///use.axol", at(use_src, "damage") + 1).unwrap();
    assert_eq!(
        got,
        ("file:///decl.axol".to_string(), (at(decl_src, "damage"), at(decl_src, "damage") + 6))
    );
    let refs = f.refs("file:///use.axol", at(use_src, "damage") + 1, true).unwrap();
    assert_eq!(refs.len(), 2);
    assert!(refs.contains(&("file:///decl.axol".to_string(), (at(decl_src, "damage"), at(decl_src, "damage") + 6))));
    assert!(refs.contains(&("file:///use.axol".to_string(), (at(use_src, "damage"), at(use_src, "damage") + 6))));
    let renames = f.renames("file:///use.axol", at(use_src, "damage") + 1, "hurt").unwrap();
    assert_eq!(renames.len(), 2);
}

/// A local shadowing an imported module-level name in another document wins locally.
#[test]
fn cross_doc_local_shadowing_wins_over_other_doc_decl() {
    let use_src = "fn helper()\n    let damage = 1\n    damage(1)\nend\n";
    let f = fx(&[
        ("file:///use.axol", use_src),
        ("file:///decl.axol", "fn damage(amount: Int)\n    print(amount)\nend\n"),
    ]);
    let local_use = at(use_src, "damage(1)");
    let got = f.def("file:///use.axol", local_use + 1).unwrap();
    assert_eq!(
        got,
        ("file:///use.axol".to_string(), (at(use_src, "let damage") + 4, at(use_src, "let damage") + 10))
    );
    let refs = f.refs("file:///use.axol", local_use + 1, true).unwrap();
    assert_eq!(
        refs,
        vec![
            ("file:///use.axol".to_string(), (at(use_src, "let damage") + 4, at(use_src, "let damage") + 10)),
            ("file:///use.axol".to_string(), (local_use, local_use + 6)),
        ]
    );
}

/// Local references never cross documents even when the same name is declared elsewhere.
#[test]
fn local_references_stay_in_own_document() {
    let f = fx(&[("file:///a.axol", CD_LOCAL_A), ("file:///b.axol", CD_LOCAL_B)]);
    let got = f.refs("file:///a.axol", at(CD_LOCAL_A, "print(p)") + 7, true).unwrap();
    assert_eq!(
        got,
        vec![
            ("file:///a.axol".to_string(), (at(CD_LOCAL_A, "let p") + 4, at(CD_LOCAL_A, "let p") + 5)),
            ("file:///a.axol".to_string(), (at(CD_LOCAL_A, "print(p)") + 6, at(CD_LOCAL_A, "print(p)") + 7)),
        ]
    );
    let b_got = f.refs("file:///b.axol", at(CD_LOCAL_B, "print(p)") + 7, true).unwrap();
    assert_eq!(b_got.len(), 2);
    assert!(b_got.contains(&("file:///b.axol".to_string(), (at(CD_LOCAL_B, "const p") + 6, at(CD_LOCAL_B, "const p") + 7))));
}

/// Definition prefers the declaration in the cursor's own document.
#[test]
fn definition_prefers_own_document() {
    let f = fx(&[("file:///a.axol", CD_DEF_A), ("file:///b.axol", CD_DEF_B)]);
    let got = f.def("file:///a.axol", at(CD_DEF_A, "damage(1)") + 1).unwrap();
    assert_eq!(
        got,
        (
            "file:///a.axol".to_string(),
            (at(CD_DEF_A, "fn damage") + 3, at(CD_DEF_A, "fn damage") + 9)
        )
    );
}

/// Method calls on receivers with unknown types fall back to a unique method name.
#[test]
fn unknown_receiver_falls_back_to_unique_method() {
    let a = analyze(FALLBACK_SRC);
    let attacks = occ(&a, "attack");
    assert_eq!(attacks.len(), 2);
    let range = a.definition_range(attacks[1].0 + 1).unwrap();
    assert_eq!(bytes(&a, &range), attacks[0]);
}

/// Field accesses on receivers with unknown types fall back to a unique field name.
#[test]
fn unknown_receiver_falls_back_to_unique_field() {
    let a = analyze(FALLBACK_SRC);
    let hps = occ(&a, "hp");
    let range = a.definition_range(hps[2].0 + 1).unwrap();
    assert_eq!(bytes(&a, &range), hps[0]);
}

/// Ambiguous method and field names on unknown receivers do not resolve.
#[test]
fn ambiguous_names_do_not_resolve() {
    let a = analyze(AMBIG_SRC);
    let attacks = occ(&a, "attack");
    assert_eq!(attacks.len(), 3);
    assert!(a.binding_id_at(attacks[2].0 + 1).is_none());
    let thing_hp = at_from(AMBIG_SRC, "thing.hp", at(AMBIG_SRC, "fn poke")) + 6;
    assert!(a.binding_id_at(thing_hp + 1).is_none());
}

/// Same-named fields on two structs stay separate bindings.
#[test]
fn same_name_fields_on_two_structs_stay_separate() {
    let a = analyze(AMBIG_SRC);
    let hps = occ(&a, "hp");
    let left = a.resolved_def(hps[0].0 + 1).unwrap();
    assert_eq!(left.owner.as_deref(), Some("Left"));
    let right = a.resolved_def(hps[1].0 + 1).unwrap();
    assert_eq!(right.owner.as_deref(), Some("Right"));
    let left_refs = a.reference_ranges(hps[0].0 + 1, true).unwrap();
    assert_eq!(
        left_refs.iter().map(|r| bytes(&a, r)).collect::<Vec<_>>(),
        vec![hps[0], hps[2]]
    );
}

/// Self fields on two same-named methods of different receivers stay separate.
#[test]
fn self_fields_on_two_receivers_stay_separate() {
    let a = analyze(AMBIG_SRC);
    let selves = occ(&a, "self");
    assert_eq!(selves.len(), 4);
    let left = a.resolved_def(selves[1].0 + 2).unwrap();
    assert_eq!(left.ty.as_deref(), Some("Left"));
    let right = a.resolved_def(selves[3].0 + 2).unwrap();
    assert_eq!(right.ty.as_deref(), Some("Right"));
    let left_hover = hover_value(&a, selves[1].0 + 2).unwrap();
    assert!(left_hover.contains("**receiver** of `Left.attack`"));
    let right_hover = hover_value(&a, selves[3].0 + 2).unwrap();
    assert!(right_hover.contains("**receiver** of `Right.attack`"));
}

/// Hovering a keyword or non-identifier token yields nothing.
#[test]
fn hover_on_keyword_is_none() {
    let a = analyze(SHADOW_SRC);
    assert!(a.hover(off(&a, 1, 4)).is_none());
    assert!(a.hover(off(&a, 1, 7)).is_none());
}

/// References on the declaration include it first, and exclude it when asked.
#[test]
fn references_include_declaration_flag_is_respected() {
    let a = analyze(SHADOW_SRC);
    let xs = occ(&a, "x");
    let included = a.reference_ranges(xs[0].0 + 1, true).unwrap();
    let excluded = a.reference_ranges(xs[0].0 + 1, false).unwrap();
    assert_eq!(
        included.iter().map(|r| bytes(&a, r)).collect::<Vec<_>>(),
        vec![xs[0], xs[1], xs[4]]
    );
    assert_eq!(
        excluded.iter().map(|r| bytes(&a, r)).collect::<Vec<_>>(),
        vec![xs[1], xs[4]]
    );
}

/// Definition on a use and on its declaration agree on the binding.
#[test]
fn definition_on_use_and_decl_agree() {
    let a = analyze(METHODS_SRC);
    let heroes = occ(&a, "hero");
    let from_use = a.definition_range(heroes[1].0 + 1).unwrap();
    let from_decl = a.definition_range(heroes[0].0 + 1).unwrap();
    assert_eq!(bytes(&a, &from_use), bytes(&a, &from_decl));
}

/// Rename produces edits whose new text is the requested name.
#[test]
fn rename_edits_carry_new_name() {
    let a = analyze(METHODS_SRC);
    let heroes = occ(&a, "hero");
    let edits = a.rename_edits(heroes[1].0 + 1, "champ").unwrap();
    assert_eq!(edits.len(), 3);
    assert!(edits.iter().all(|e| e.new_text == "champ"));
    let got: Vec<(u32, u32)> = edits.iter().map(|e| bytes(&a, &e.range)).collect();
    assert_eq!(got, heroes);
}

/// Hover over a method-call receiver still resolves the local binding.
#[test]
fn hover_over_method_receiver_resolves_local() {
    let a = analyze(METHODS_SRC);
    let heroes = occ(&a, "hero");
    let value = hover_value(&a, heroes[1].0 + 1).unwrap();
    assert!(value.contains("hero: Player"));
    assert!(value.contains("**local** - `let` binding in `main`"));
}

/// The method receiver token in the declaration resolves to the struct.
#[test]
fn method_receiver_token_resolves_to_struct() {
    let a = analyze(METHODS_SRC);
    let players = occ(&a, "Player");
    let range = a.definition_range(players[1].0 + 1).unwrap();
    assert_eq!(bytes(&a, &range), players[0]);
}

/// Struct references include the method receiver, literal and type uses.
#[test]
fn struct_references_include_receiver_uses() {
    let a = analyze(METHODS_SRC);
    let players = occ(&a, "Player");
    let ranges = a.reference_ranges(players[1].0 + 1, true).unwrap();
    assert_eq!(ranges.iter().map(|r| bytes(&a, r)).collect::<Vec<_>>(), players);
}

/// Text edits for rename are ordered by position.
#[test]
fn rename_edits_are_ordered_by_position() {
    let a = analyze(SAME_NAME_SRC);
    let damages = occ(&a, "damage");
    let edits = a.rename_edits(damages[1].0 + 1, "hurt").unwrap();
    let got: Vec<(u32, u32)> = edits.iter().map(|e| bytes(&a, &e.range)).collect();
    assert_eq!(got, vec![damages[1], damages[2]]);
    assert!(got[0].0 < got[1].0);
}

/// Occurrences inside strings and comments are not resolved as references.
#[test]
fn references_skip_string_and_comment_text() {
    let src = "-- damage is mentioned here\nfn damage(x: Int)\n    print(\"damage\")\nend\n";
    let a = analyze(src);
    let damages = occ(&a, "damage");
    assert_eq!(damages.len(), 1);
    let ranges = a.reference_ranges(damages[0].0 + 1, true).unwrap();
    assert_eq!(ranges.iter().map(|r| bytes(&a, r)).collect::<Vec<_>>(), damages);
}

/// Definition at the token end (completion position) still resolves.
#[test]
fn definition_at_token_end_resolves() {
    let a = analyze(METHODS_SRC);
    let heroes = occ(&a, "hero");
    let range = a.definition_range(heroes[1].0 + heroes[1].1 - heroes[1].0).unwrap();
    assert_eq!(bytes(&a, &range), heroes[0]);
}
