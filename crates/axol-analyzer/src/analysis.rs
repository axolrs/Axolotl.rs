// Owner: PascalElixir / axolrs (GitHub org)
// File: Analysis - the pure Gills engine: parses, lowers and indexes one document, then answers hover, completion, symbols, diagnostics and cross-document queries without any transport.

use std::collections::HashMap;

use axolc_core::ast;
use axolc_core::diag::{Diagnostic as CoreDiagnostic, Severity};
use axolc_core::lexer::token::{Token, TokenKind};
use axolc_core::ownership::ParamMode;
use axolc_core::span::Span;
use tower_lsp::lsp_types::{
    CompletionItem, CompletionItemKind, CompletionTextEdit, Diagnostic, DiagnosticSeverity,
    DocumentSymbol, Hover, HoverContents, Location, MarkupContent, MarkupKind, NumberOrString,
    Range, SymbolKind, TextEdit, Url, WorkspaceEdit,
};

use crate::line_index::LineIndex;
use crate::scope::{Container, Def, DefKind, ScopeIndex};

/// The Axolotl keyword set offered by completion, in parser-accepted order.
pub const KEYWORDS: &[&str] = &[
    "fn", "let", "var", "if", "then", "elseif", "else", "end", "while", "for", "do", "match",
    "case", "struct", "enum", "interface", "impl", "const", "type", "use", "return", "break",
    "continue", "repeat", "until", "loop", "pub", "async", "await", "spawn", "unsafe", "borrow",
    "mut", "move",
];

/// The kind of a top-level declaration exposed to editors.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeclKind {
    /// A free function.
    Fn,
    /// A method attached to a receiver type.
    Method,
    /// A struct type.
    Struct,
    /// An enum type.
    Enum,
    /// An interface (trait-like) type.
    Interface,
    /// A named constant.
    Const,
}

/// A top-level declaration found in a document.
#[derive(Debug, Clone)]
pub struct ItemDecl {
    /// The declared name.
    pub name: String,
    /// What kind of item this is.
    pub kind: DeclKind,
    /// Span of the declaring identifier token.
    pub ident_span: Span,
    /// Span of the whole item.
    pub item_span: Span,
}

/// A complete analysis of one document: AST, HIR with ownership, tokens, line index and scope index.
#[derive(Debug, Clone)]
pub struct Analysis {
    /// The document body this analysis was built from.
    pub src: String,
    /// The parsed AST module.
    pub module: ast::Module,
    /// The lowered HIR module carrying the ownership table.
    pub hir: axolc_core::hir::Module,
    /// All parse and ownership diagnostics merged in source order.
    pub diags: axolc_core::diag::Diagnostics,
    /// The token stream used for word-level queries.
    pub tokens: Vec<Token>,
    /// The line index used for position conversion.
    pub index: LineIndex,
    /// The scope-aware name resolution index.
    pub scope: ScopeIndex,
}

/// Parse, lower and index one document body.
pub fn analyze(src: &str) -> Analysis {
    let (module, parse_diags) = axolc_core::parse(src, 0);
    let hir = axolc_core::hir::lower(&module);
    let (tokens, _) = axolc_core::tokenize(src, 0);
    let mut diags = parse_diags;
    diags.extend(hir.diags.clone());
    let index = LineIndex::new(src);
    let scope = ScopeIndex::build(&module, src);
    Analysis { src: src.to_string(), module, hir, diags, tokens, index, scope }
}

impl Analysis {
    /// Convert all core diagnostics into LSP diagnostics with correct ranges.
    pub fn lsp_diagnostics(&self) -> Vec<Diagnostic> {
        self.diags
            .items
            .iter()
            .map(|d| self.to_lsp_diagnostic(d))
            .collect()
    }

    /// Return the top-level declarations in source order.
    pub fn item_decls(&self) -> Vec<ItemDecl> {
        self.module
            .items
            .iter()
            .filter_map(|item| {
                let (name, kind, ident_span, item_span) = match item {
                    ast::Item::Fn(f) => (&f.name.name, DeclKind::Fn, f.name.span, f.span),
                    ast::Item::Method(m) => (&m.name.name, DeclKind::Method, m.name.span, m.span),
                    ast::Item::StaticMethod(m) => {
                        (&m.name.name, DeclKind::Method, m.name.span, m.span)
                    }
                    ast::Item::Struct(s) => (&s.name.name, DeclKind::Struct, s.name.span, s.span),
                    ast::Item::Enum(e) => (&e.name.name, DeclKind::Enum, e.name.span, e.span),
                    ast::Item::Interface(i) => {
                        (&i.name.name, DeclKind::Interface, i.name.span, i.span)
                    }
                    ast::Item::Const(c) => (&c.name.name, DeclKind::Const, c.name.span, c.span),
                    _ => return None,
                };
                Some(ItemDecl {
                    name: name.clone(),
                    kind,
                    ident_span,
                    item_span,
                })
            })
            .collect()
    }

    /// Return the identifier token touching the cursor offset, if any.
    ///
    /// "Touching" prefers full containment, then a token ending exactly at the
    /// cursor (the usual completion position).
    pub fn token_at(&self, offset: u32) -> Option<&Token> {
        self.tokens
            .iter()
            .find(|t| t.span.start <= offset && offset < t.span.end)
            .or_else(|| {
                self.tokens
                    .iter()
                    .find(|t| t.span.end == offset && t.span.start < t.span.end)
            })
    }

    /// Return the identifier name at the cursor offset, if any.
    pub fn identifier_at(&self, offset: u32) -> Option<String> {
        let token = self.token_at(offset)?;
        if token.kind == TokenKind::Ident {
            Some(token.text.clone())
        } else {
            None
        }
    }

    /// Return the LSP range of the identifier at the cursor offset, if any.
    pub fn identifier_range_at(&self, offset: u32) -> Option<Range> {
        let token = self.token_at(offset)?;
        if token.kind != TokenKind::Ident {
            return None;
        }
        Some(self.range(token.span.start, token.span.end))
    }

    /// Return the markdown hover for the identifier at the cursor offset, resolved through the scope tree.
    pub fn hover(&self, offset: u32) -> Option<Hover> {
        let token = self.name_token_at(offset)?;
        let id = self.scope.resolve_span(token.span)?;
        let def = self.scope.def(id);
        let value = self.def_hover_markdown(def)?;
        let range = self.range(token.span.start, token.span.end);
        Some(Hover {
            contents: HoverContents::Markup(MarkupContent { kind: MarkupKind::Markdown, value }),
            range: Some(range),
        })
    }

    /// Return the identifier or `self` token touching the cursor offset, preferring the name token to its left at a boundary.
    fn name_token_at(&self, offset: u32) -> Option<&Token> {
        let contained = self
            .tokens
            .iter()
            .find(|t| t.span.start <= offset && offset < t.span.end);
        if let Some(token) = contained {
            if is_name_token(token) {
                return Some(token);
            }
        }
        let ending = self
            .tokens
            .iter()
            .find(|t| t.span.end == offset && t.span.start < t.span.end);
        if let Some(token) = ending {
            if is_name_token(token) {
                return Some(token);
            }
        }
        contained.filter(|t| is_name_token(t))
    }

    /// Return the binding id resolved at the cursor offset, if any.
    pub fn binding_id_at(&self, offset: u32) -> Option<usize> {
        let token = self.name_token_at(offset)?;
        self.scope.resolve_span(token.span)
    }

    /// Return the resolved declaration at the cursor offset, if any.
    pub fn resolved_def(&self, offset: u32) -> Option<&Def> {
        let id = self.binding_id_at(offset)?;
        Some(self.scope.def(id))
    }

    /// Return the declaration range of the binding resolved at the cursor offset.
    pub fn definition_range(&self, offset: u32) -> Option<Range> {
        let id = self.binding_id_at(offset)?;
        let def = self.scope.def(id);
        Some(self.range(def.span.start, def.span.end))
    }

    /// Return the reference ranges of the binding resolved at the cursor offset.
    ///
    /// The declaration is included only when requested; shadowing keeps same-named
    /// bindings in different scopes separate.
    pub fn reference_ranges(&self, offset: u32, include_declaration: bool) -> Option<Vec<Range>> {
        let id = self.binding_id_at(offset)?;
        let def = self.scope.def(id);
        let mut ranges: Vec<Range> = self
            .scope
            .binding_uses(id)
            .into_iter()
            .map(|span| self.range(span.start, span.end))
            .collect();
        if include_declaration {
            ranges.insert(0, self.range(def.span.start, def.span.end));
        }
        Some(ranges)
    }

    /// Return rename edits for the binding resolved at the cursor offset.
    pub fn rename_edits(&self, offset: u32, new_name: &str) -> Option<Vec<TextEdit>> {
        let id = self.binding_id_at(offset)?;
        let def = self.scope.def(id);
        let mut edits: Vec<TextEdit> = self
            .scope
            .binding_uses(id)
            .into_iter()
            .map(|span| TextEdit {
                range: self.range(span.start, span.end),
                new_text: new_name.to_string(),
            })
            .collect();
        edits.insert(
            0,
            TextEdit {
                range: self.range(def.span.start, def.span.end),
                new_text: new_name.to_string(),
            },
        );
        Some(edits)
    }

    /// Return the name at the cursor when it is an unresolved plain name or type use.
    pub fn unresolved_module_name_at(&self, offset: u32) -> Option<String> {
        let token = self.name_token_at(offset)?;
        if self.binding_id_at(offset).is_some() {
            return None;
        }
        self.scope.unresolved_name_at(token.span)
    }

    /// Return keyword and document-item completions with insert text edits.
    pub fn completions(&self, offset: u32) -> Vec<CompletionItem> {
        let insert_range = self.completion_range(offset);
        let mut items: Vec<CompletionItem> = KEYWORDS
            .iter()
            .map(|kw| CompletionItem {
                label: (*kw).to_string(),
                kind: Some(CompletionItemKind::KEYWORD),
                detail: Some("Axolotl keyword".to_string()),
                sort_text: Some(format!("0_{kw}")),
                text_edit: Some(CompletionTextEdit::Edit(TextEdit {
                    range: insert_range,
                    new_text: (*kw).to_string(),
                })),
                ..Default::default()
            })
            .collect();
        for decl in self.item_decls() {
            let (kind, detail) = self.decl_completion(&decl);
            items.push(CompletionItem {
                label: decl.name.clone(),
                kind: Some(kind),
                detail: Some(detail),
                sort_text: Some(format!("1_{}", decl.name)),
                text_edit: Some(CompletionTextEdit::Edit(TextEdit {
                    range: insert_range,
                    new_text: decl.name.clone(),
                })),
                ..Default::default()
            });
        }
        items
    }

    /// Return nested document symbols for all top-level items.
    pub fn document_symbols(&self) -> Vec<DocumentSymbol> {
        self.module
            .items
            .iter()
            .filter_map(|item| self.item_symbol(item))
            .collect()
    }

    /// Return the identifier range of the first top-level declaration with the given name.
    pub fn decl_range(&self, name: &str) -> Option<Range> {
        self.item_decls()
            .iter()
            .find(|d| d.name == name)
            .map(|d| self.range(d.ident_span.start, d.ident_span.end))
    }

    /// Return the whole-word name occurrences of a name in this document.
    ///
    /// Matching runs on the token stream, so occurrences inside strings or
    /// comments are never reported; the `self` keyword counts as a name.
    pub fn occurrences(&self, name: &str) -> Vec<Range> {
        self.tokens
            .iter()
            .filter(|t| is_name_token(t) && t.text == name)
            .map(|t| self.range(t.span.start, t.span.end))
            .collect()
    }

    /// Render a human-readable `file:line:col: severity[code]: message` report.
    pub fn report_lines(&self, file: &str) -> Vec<String> {
        self.diags
            .items
            .iter()
            .flat_map(|d| self.report_lines_for(file, d))
            .collect()
    }

    /// Return true when the analysis contains at least one error diagnostic.
    pub fn has_errors(&self) -> bool {
        self.diags.has_errors()
    }

    /// Convert a byte span into an LSP range.
    fn range(&self, start: u32, end: u32) -> Range {
        self.index.range_of(&self.src, start, end)
    }

    /// Build one LSP diagnostic from one core diagnostic.
    fn to_lsp_diagnostic(&self, d: &CoreDiagnostic) -> Diagnostic {
        let message = if d.notes.is_empty() {
            d.message.clone()
        } else {
            let mut message = d.message.clone();
            for note in &d.notes {
                message.push_str("\nnote: ");
                message.push_str(note);
            }
            message
        };
        Diagnostic {
            range: self.range(d.span.start, d.span.end),
            severity: Some(self.severity(d.severity)),
            code: d.code.clone().map(NumberOrString::String),
            source: Some("axolc".to_string()),
            message,
            ..Default::default()
        }
    }

    /// Map a core severity onto the LSP severity numbering.
    fn severity(&self, severity: Severity) -> DiagnosticSeverity {
        match severity {
            Severity::Error => DiagnosticSeverity::ERROR,
            Severity::Warning => DiagnosticSeverity::WARNING,
            Severity::Info => DiagnosticSeverity::INFORMATION,
            Severity::Hint => DiagnosticSeverity::HINT,
        }
    }

    /// Render the report lines (message plus notes) for one core diagnostic.
    fn report_lines_for(&self, file: &str, d: &CoreDiagnostic) -> Vec<String> {
        let pos = self.index.position_of(&self.src, d.span.start);
        let sev = match d.severity {
            Severity::Error => "error",
            Severity::Warning => "warning",
            Severity::Info => "info",
            Severity::Hint => "hint",
        };
        let code = d.code.as_deref().map(|c| format!("[{c}]")).unwrap_or_default();
        let mut lines = vec![format!(
            "{}:{}:{}: {}{}: {}",
            file,
            pos.line + 1,
            pos.character + 1,
            sev,
            code,
            d.message
        )];
        for note in &d.notes {
            lines.push(format!("  note: {note}"));
        }
        lines
    }

    /// Build the markdown body for a hover over one resolved declaration.
    fn def_hover_markdown(&self, def: &Def) -> Option<String> {
        match def.kind {
            DefKind::Fn => self.fn_decl_hover(&def.name),
            DefKind::Method => self.method_decl_hover(def.owner.as_deref()?, &def.name),
            DefKind::Struct => self.struct_decl_hover(&def.name),
            DefKind::Enum => self.enum_decl_hover(&def.name),
            DefKind::Interface => self.interface_decl_hover(&def.name),
            DefKind::Const => self.const_decl_hover(&def.name),
            DefKind::Field => self.field_hover_markdown(def),
            DefKind::Variant => self.variant_hover_markdown(def),
            DefKind::Param => self.param_hover_markdown(def),
            DefKind::Let => Some(self.local_hover_markdown(def)),
        }
    }

    /// Build the markdown body for a hover over a function name.
    fn fn_decl_hover(&self, name: &str) -> Option<String> {
        let f = self.find_fn(name)?;
        let sig = self.fn_signature(f);
        let mut value = fenced(&sig);
        let ownership = self.ownership_summary(name);
        if !ownership.is_empty() {
            value.push_str("\n\n");
            value.push_str(&ownership);
        }
        Some(value)
    }

    /// Build the markdown body for a hover over a struct name.
    fn struct_decl_hover(&self, name: &str) -> Option<String> {
        let s = self.find_struct(name)?;
        Some(fenced(&self.struct_signature(s)))
    }

    /// Build the markdown body for a hover over an enum name.
    fn enum_decl_hover(&self, name: &str) -> Option<String> {
        let e = self.find_enum(name)?;
        Some(fenced(&self.enum_signature(e)))
    }

    /// Build the markdown body for a hover over an interface name.
    fn interface_decl_hover(&self, name: &str) -> Option<String> {
        let i = self.find_interface(name)?;
        Some(fenced(&format!("interface {}", i.name.name)))
    }

    /// Build the markdown body for a hover over a constant name.
    fn const_decl_hover(&self, name: &str) -> Option<String> {
        let c = self.find_const(name)?;
        Some(fenced(&self.const_signature(c)))
    }

    /// Build the markdown body for a hover over a method name, qualified by its receiver.
    fn method_decl_hover(&self, owner: &str, name: &str) -> Option<String> {
        if let Some(m) = self.find_method_in(owner, name) {
            return Some(fenced(&self.method_signature(m)));
        }
        if let Some(sig) = self.find_interface_method(owner, name) {
            return Some(fenced(&self.interface_method_signature(owner, sig)));
        }
        None
    }

    /// Render the signature of an interface method qualified by the interface name.
    fn interface_method_signature(&self, owner: &str, sig: &ast::MethodSig) -> String {
        let params: Vec<String> = sig
            .params
            .iter()
            .map(|p| {
                if p.name.name == "self" {
                    p.name.name.clone()
                } else {
                    format!("{}: {}", p.name.name, self.type_text(p.ty.as_ref()))
                }
            })
            .collect();
        let ret = sig
            .ret
            .as_ref()
            .map(|r| format!(" -> {}", self.type_text(Some(r))))
            .unwrap_or_default();
        format!("fn {owner}.{}({}){}", sig.name.name, params.join(", "), ret)
    }

    /// Build the markdown body for a hover over a struct field, naming its struct.
    fn field_hover_markdown(&self, def: &Def) -> Option<String> {
        let owner = def.owner.as_deref()?;
        let s = self.find_struct(owner)?;
        let field = s.fields.iter().find(|f| f.name.name == def.name)?;
        let sig = format!("{}: {}", def.name, self.type_text(Some(&field.ty)));
        Some(format!("{}\n\n**field** of struct `{owner}`", fenced(&sig)))
    }

    /// Build the markdown body for a hover over an enum variant, naming its enum.
    fn variant_hover_markdown(&self, def: &Def) -> Option<String> {
        let owner = def.owner.as_deref()?;
        let e = self.find_enum(owner)?;
        let variant = e.variants.iter().find(|v| v.name.name == def.name)?;
        let fields: Vec<String> =
            variant.fields.iter().map(|f| self.type_text(Some(&f.ty))).collect();
        let sig = if fields.is_empty() {
            def.name.clone()
        } else {
            format!("{}({})", def.name, fields.join(", "))
        };
        Some(format!("{}\n\n**variant** of enum `{owner}`", fenced(&sig)))
    }

    /// Build the markdown body for a hover over a local binding.
    fn local_hover_markdown(&self, def: &Def) -> String {
        let base = def.ty.clone().unwrap_or_else(|| "_".to_string());
        let header = if base == "_" {
            def.name.clone()
        } else {
            format!("{}: {}", def.name, base)
        };
        let kw = if def.is_var { "var" } else { "let" };
        let scope = match &def.container {
            Container::Closure => "a closure".to_string(),
            other => format!("`{}`", other.describe()),
        };
        format!("{}\n\n**local** - `{kw}` binding in {scope}", fenced(&header))
    }

    /// Build the markdown body for a hover over a parameter, preserving the ownership-mode content.
    fn param_hover_markdown(&self, def: &Def) -> Option<String> {
        let base = def.ty.clone().unwrap_or_else(|| "_".to_string());
        if let Container::Function(fname) = &def.container {
            let modes = self.hir.ownership.modes_for(fname);
            let resolved = modes
                .iter()
                .find(|(pname, _)| pname == &def.name)
                .map(|(_, mode)| *mode);
            let header = match resolved {
                Some(mode) => render_typed(&def.name, mode_sigil(mode), &base),
                None => {
                    if base == "_" {
                        def.name.clone()
                    } else {
                        format!("{}: {}", def.name, base)
                    }
                }
            };
            let mut value = fenced(&header);
            if let Some(mode) = resolved {
                let hints = self.hir.ownership.hints_for(fname);
                let hint = hints
                    .iter()
                    .find(|(pname, _)| pname == &def.name)
                    .map(|(_, m)| *m);
                let line = ownership_entry(mode, hint);
                value.push_str("\n\n");
                value.push_str(&format!("**ownership**: {line} - parameter of `{fname}`"));
            }
            return Some(value);
        }
        let header = if def.self_param {
            render_typed(&def.name, "", &base)
        } else if base == "_" {
            def.name.clone()
        } else {
            format!("{}: {}", def.name, base)
        };
        let role = if def.self_param { "receiver" } else { "parameter" };
        let container = match &def.container {
            Container::Closure => "a closure".to_string(),
            other => format!("`{}`", other.describe()),
        };
        Some(format!("{}\n\n**{role}** of {container}", fenced(&header)))
    }

    /// Find the top-level function with the given name.
    fn find_fn(&self, name: &str) -> Option<&ast::FnItem> {
        self.module
            .items
            .iter()
            .find_map(|i| match i {
                ast::Item::Fn(f) if f.name.name == name => Some(f),
                _ => None,
            })
    }

    /// Find the struct with the given name.
    fn find_struct(&self, name: &str) -> Option<&ast::StructItem> {
        self.module
            .items
            .iter()
            .find_map(|i| match i {
                ast::Item::Struct(s) if s.name.name == name => Some(s),
                _ => None,
            })
    }

    /// Find the enum with the given name.
    fn find_enum(&self, name: &str) -> Option<&ast::EnumItem> {
        self.module
            .items
            .iter()
            .find_map(|i| match i {
                ast::Item::Enum(e) if e.name.name == name => Some(e),
                _ => None,
            })
    }

    /// Find the interface with the given name.
    fn find_interface(&self, name: &str) -> Option<&ast::InterfaceItem> {
        self.module
            .items
            .iter()
            .find_map(|i| match i {
                ast::Item::Interface(t) if t.name.name == name => Some(t),
                _ => None,
            })
    }

    /// Find the constant with the given name.
    fn find_const(&self, name: &str) -> Option<&ast::ConstItem> {
        self.module
            .items
            .iter()
            .find_map(|i| match i {
                ast::Item::Const(c) if c.name.name == name => Some(c),
                _ => None,
            })
    }

    /// Find the method with the given name across receivers.
    fn find_method(&self, name: &str) -> Option<&ast::MethodItem> {
        self.module
            .items
            .iter()
            .find_map(|i| match i {
                ast::Item::Method(m) | ast::Item::StaticMethod(m) if m.name.name == name => {
                    Some(m)
                }
                _ => None,
            })
    }

    /// Find the method with the given name declared for one receiver type.
    fn find_method_in(&self, owner: &str, name: &str) -> Option<&ast::MethodItem> {
        self.module
            .items
            .iter()
            .find_map(|i| match i {
                ast::Item::Method(m) | ast::Item::StaticMethod(m)
                    if m.receiver.name == owner && m.name.name == name =>
                {
                    Some(m)
                }
                _ => None,
            })
    }

    /// Find the method signature with the given name declared in one interface.
    fn find_interface_method(&self, owner: &str, name: &str) -> Option<&ast::MethodSig> {
        self.find_interface(owner)?.methods.iter().find(|m| m.name.name == name)
    }

    /// Render the signature of a free function, overlaying resolved ownership modes.
    pub fn fn_signature(&self, f: &ast::FnItem) -> String {
        let mode_list = self.hir.ownership.modes_for(&f.name.name);
        let modes: HashMap<&str, ParamMode> =
            mode_list.iter().map(|(n, m)| (n.as_str(), *m)).collect();
        let params: Vec<String> = f
            .params
            .iter()
            .map(|p| {
                let base = self.base_type_text(p.ty.as_ref());
                match modes.get(p.name.name.as_str()) {
                    Some(mode) => render_typed(&p.name.name, mode_sigil(*mode), &base),
                    None => format!("{}: {}", p.name.name, self.type_text(p.ty.as_ref())),
                }
            })
            .collect();
        let ret = f
            .ret
            .as_ref()
            .map(|r| format!(" -> {}", self.type_text(Some(r))))
            .unwrap_or_default();
        let prefix = if f.is_pub { "pub " } else { "" };
        let async_prefix = if f.is_async { "async " } else { "" };
        format!("{prefix}{async_prefix}fn {}({}){ret}", f.name.name, params.join(", "))
    }

    /// Render the signature of a struct with its fields.
    fn struct_signature(&self, s: &ast::StructItem) -> String {
        let fields: Vec<String> = s
            .fields
            .iter()
            .map(|f| format!("{}: {}", f.name.name, self.type_text(Some(&f.ty))))
            .collect();
        format!("struct {} {{ {} }}", s.name.name, fields.join(", "))
    }

    /// Render the signature of an enum with its variants.
    fn enum_signature(&self, e: &ast::EnumItem) -> String {
        let variants: Vec<String> = e
            .variants
            .iter()
            .map(|v| {
                if v.fields.is_empty() {
                    v.name.name.clone()
                } else {
                    let fields: Vec<String> =
                        v.fields.iter().map(|f| self.type_text(Some(&f.ty))).collect();
                    format!("{}({})", v.name.name, fields.join(", "))
                }
            })
            .collect();
        format!("enum {} {{ {} }}", e.name.name, variants.join(", "))
    }

    /// Render the signature of a constant.
    fn const_signature(&self, c: &ast::ConstItem) -> String {
        let ty = self.type_text(c.ty.as_ref());
        format!("const {}: {}", c.name.name, ty)
    }

    /// Render the signature of a method qualified by its receiver type.
    fn method_signature(&self, m: &ast::MethodItem) -> String {
        let params: Vec<String> = m
            .params
            .iter()
            .map(|p| {
                let ty = self.type_text(p.ty.as_ref());
                if p.name.name == "self" {
                    p.name.name.clone()
                } else {
                    format!("{}: {}", p.name.name, ty)
                }
            })
            .collect();
        let ret = m
            .ret
            .as_ref()
            .map(|r| format!(" -> {}", self.type_text(Some(r))))
            .unwrap_or_default();
        let async_prefix = if m.is_async { "async " } else { "" };
        format!("{async_prefix}fn {}.{}({}){ret}", m.receiver.name, m.name.name, params.join(", "))
    }

    /// Return the source text of a type annotation, or `_` when absent.
    fn type_text(&self, ty: Option<&ast::TypeAnnot>) -> String {
        match ty {
            Some(t) => self.slice_span(t.span),
            None => "_".to_string(),
        }
    }

    /// Return the base type text with any borrow/move wrapper unwrapped.
    fn base_type_text(&self, ty: Option<&ast::TypeAnnot>) -> String {
        let Some(t) = ty else { return "_".to_string() };
        match &t.kind {
            ast::TypeAnnotKind::Borrow(inner, _) | ast::TypeAnnotKind::Move(inner) => {
                self.slice_span(inner.span)
            }
            _ => self.type_text(Some(t)),
        }
    }

    /// Slice the document at a span, clamped and boundary-checked.
    fn slice_span(&self, span: Span) -> String {
        let start = (span.start as usize).min(self.src.len());
        let end = (span.end as usize).min(self.src.len()).max(start);
        if self.src.is_char_boundary(start) && self.src.is_char_boundary(end) {
            self.src[start..end].to_string()
        } else {
            "_".to_string()
        }
    }

    /// Render the ownership summary line for one function's resolved mode table.
    fn ownership_summary(&self, fn_name: &str) -> String {
        let modes = self.hir.ownership.modes_for(fn_name);
        if modes.is_empty() {
            return String::new();
        }
        let hints = self.hir.ownership.hints_for(fn_name);
        let entries: Vec<String> = modes
            .iter()
            .map(|(name, mode)| {
                let hint = hints
                    .iter()
                    .find(|(pname, _)| pname == name)
                    .map(|(_, m)| *m);
                format!("`{name}`: {}", ownership_entry(*mode, hint))
            })
            .collect();
        format!("**ownership** - {}", entries.join(", "))
    }

    /// Return the range a completion text edit should replace.
    fn completion_range(&self, offset: u32) -> Range {
        match self.token_at(offset) {
            Some(t) if t.kind == TokenKind::Ident => self.range(t.span.start, t.span.end),
            _ => self.range(offset, offset),
        }
    }

    /// Return the completion kind and detail for one declaration.
    fn decl_completion(&self, decl: &ItemDecl) -> (CompletionItemKind, String) {
        match decl.kind {
            DeclKind::Fn => {
                let f = self.find_fn(&decl.name);
                (
                    CompletionItemKind::FUNCTION,
                    f.map(|f| self.fn_signature(f)).unwrap_or_else(|| "fn".to_string()),
                )
            }
            DeclKind::Method => {
                let m = self.find_method(&decl.name);
                (
                    CompletionItemKind::METHOD,
                    m.map(|m| self.method_signature(m)).unwrap_or_else(|| "method".to_string()),
                )
            }
            DeclKind::Struct => {
                let s = self.find_struct(&decl.name);
                (
                    CompletionItemKind::STRUCT,
                    s.map(|s| self.struct_signature(s))
                        .unwrap_or_else(|| "struct".to_string()),
                )
            }
            DeclKind::Enum => (
                CompletionItemKind::ENUM,
                format!("enum {}", decl.name),
            ),
            DeclKind::Interface => (
                CompletionItemKind::INTERFACE,
                format!("interface {}", decl.name),
            ),
            DeclKind::Const => (
                CompletionItemKind::CONSTANT,
                format!("const {}", decl.name),
            ),
        }
    }

    /// Build one document symbol from one AST item.
    #[allow(deprecated)]
    fn item_symbol(&self, item: &ast::Item) -> Option<DocumentSymbol> {
        let symbol = match item {
            ast::Item::Fn(f) => DocumentSymbol {
                name: f.name.name.clone(),
                detail: Some(self.fn_signature(f)),
                kind: SymbolKind::FUNCTION,
                range: self.range(f.span.start, f.span.end),
                selection_range: self.range(f.name.span.start, f.name.span.end),
                children: None,
                tags: None,
                deprecated: None,
            },
            ast::Item::Method(m) | ast::Item::StaticMethod(m) => DocumentSymbol {
                name: format!("{}.{}", m.receiver.name, m.name.name),
                detail: Some(self.method_signature(m)),
                kind: SymbolKind::METHOD,
                range: self.range(m.span.start, m.span.end),
                selection_range: self.range(m.name.span.start, m.name.span.end),
                children: None,
                tags: None,
                deprecated: None,
            },
            ast::Item::Struct(s) => DocumentSymbol {
                name: s.name.name.clone(),
                detail: Some(self.struct_signature(s)),
                kind: SymbolKind::STRUCT,
                range: self.range(s.span.start, s.span.end),
                selection_range: self.range(s.name.span.start, s.name.span.end),
                children: Some(
                    s.fields
                        .iter()
                        .map(|f| DocumentSymbol {
                            name: f.name.name.clone(),
                            detail: Some(self.type_text(Some(&f.ty))),
                            kind: SymbolKind::FIELD,
                            range: self.range(f.span.start, f.span.end),
                            selection_range: self.range(f.name.span.start, f.name.span.end),
                            children: None,
                            tags: None,
                            deprecated: None,
                        })
                        .collect(),
                ),
                tags: None,
                deprecated: None,
            },
            ast::Item::Enum(e) => DocumentSymbol {
                name: e.name.name.clone(),
                detail: Some(self.enum_signature(e)),
                kind: SymbolKind::ENUM,
                range: self.range(e.span.start, e.span.end),
                selection_range: self.range(e.name.span.start, e.name.span.end),
                children: Some(
                    e.variants
                        .iter()
                        .map(|v| DocumentSymbol {
                            name: v.name.name.clone(),
                            detail: None,
                            kind: SymbolKind::ENUM_MEMBER,
                            range: self.range(v.span.start, v.span.end),
                            selection_range: self.range(v.name.span.start, v.name.span.end),
                            children: None,
                            tags: None,
                            deprecated: None,
                        })
                        .collect(),
                ),
                tags: None,
                deprecated: None,
            },
            ast::Item::Interface(i) => DocumentSymbol {
                name: i.name.name.clone(),
                detail: None,
                kind: SymbolKind::INTERFACE,
                range: self.range(i.span.start, i.span.end),
                selection_range: self.range(i.name.span.start, i.name.span.end),
                children: Some(
                    i.methods
                        .iter()
                        .map(|m| DocumentSymbol {
                            name: m.name.name.clone(),
                            detail: None,
                            kind: SymbolKind::METHOD,
                            range: self.range(m.span.start, m.span.end),
                            selection_range: self.range(m.name.span.start, m.name.span.end),
                            children: None,
                            tags: None,
                            deprecated: None,
                        })
                        .collect(),
                ),
                tags: None,
                deprecated: None,
            },
            ast::Item::Const(c) => DocumentSymbol {
                name: c.name.name.clone(),
                detail: Some(self.const_signature(c)),
                kind: SymbolKind::CONSTANT,
                range: self.range(c.span.start, c.span.end),
                selection_range: self.range(c.name.span.start, c.name.span.end),
                children: None,
                tags: None,
                deprecated: None,
            },
            _ => return None,
        };
        Some(symbol)
    }
}

/// Resolve the definition location of the identifier at the cursor across open documents.
///
/// In-document bindings resolve to their declaration in the same document; unresolved
/// module-level names fall back to the first open document that declares them.
pub fn definition_in_docs(docs: &[(Url, String)], uri: &Url, offset: u32) -> Option<Location> {
    let text = docs.iter().find(|(u, _)| u == uri).map(|(_, t)| t.as_str())?;
    let source = analyze(text);
    if let Some(range) = source.definition_range(offset) {
        return Some(Location { uri: uri.clone(), range });
    }
    let name = source.unresolved_module_name_at(offset)?;
    for (doc_uri, doc_text) in docs {
        let other = analyze(doc_text);
        if let Some(id) = other.scope.module_decl(&name) {
            let def = other.scope.def(id);
            return Some(Location {
                uri: doc_uri.clone(),
                range: other.range(def.span.start, def.span.end),
            });
        }
    }
    None
}

/// Resolve all references of the binding at the cursor across open documents.
///
/// Locals, params, fields and variants stay document-local; module-level items
/// collect their same-named declaration and resolved or unresolved uses in every
/// open document. Returns `None` when the identifier resolves nowhere, including
/// module-level names no open document declares.
pub fn references_in_docs(
    docs: &[(Url, String)],
    uri: &Url,
    offset: u32,
    include_declaration: bool,
) -> Option<Vec<Location>> {
    let text = docs.iter().find(|(u, _)| u == uri).map(|(_, t)| t.as_str())?;
    let source = analyze(text);
    if let Some(id) = source.binding_id_at(offset) {
        let def = source.scope.def(id);
        if def.kind.is_module_level() {
            return Some(module_binding_locations(
                docs,
                &def.name,
                include_declaration,
            ));
        }
        let ranges = source.reference_ranges(offset, include_declaration)?;
        return Some(
            ranges
                .into_iter()
                .map(|range| Location { uri: uri.clone(), range })
                .collect(),
        );
    }
    let name = source.unresolved_module_name_at(offset)?;
    if !declared_in_docs(docs, &name) {
        return None;
    }
    Some(module_binding_locations(docs, &name, include_declaration))
}

/// Compute a rename across open documents for the binding at the cursor.
///
/// Locals, params, fields and variants rename only within their document;
/// module-level items rename in every document that declares or uses the name.
/// Returns `None` when the identifier resolves nowhere, including module-level
/// names no open document declares.
pub fn rename_in_docs(
    docs: &[(Url, String)],
    uri: &Url,
    offset: u32,
    new_name: &str,
) -> Option<WorkspaceEdit> {
    let text = docs.iter().find(|(u, _)| u == uri).map(|(_, t)| t.as_str())?;
    let source = analyze(text);
    if let Some(id) = source.binding_id_at(offset) {
        let def = source.scope.def(id);
        if def.kind.is_module_level() {
            let mut changes: HashMap<Url, Vec<TextEdit>> = HashMap::new();
            for (doc_uri, doc_text) in docs {
                let edits = module_binding_edits(doc_text, &def.name, new_name);
                if !edits.is_empty() {
                    changes.insert(doc_uri.clone(), edits);
                }
            }
            return Some(WorkspaceEdit { changes: Some(changes), ..Default::default() });
        }
        let edits = source.rename_edits(offset, new_name)?;
        let mut changes: HashMap<Url, Vec<TextEdit>> = HashMap::new();
        changes.insert(uri.clone(), edits);
        return Some(WorkspaceEdit { changes: Some(changes), ..Default::default() });
    }
    let name = source.unresolved_module_name_at(offset)?;
    if !declared_in_docs(docs, &name) {
        return None;
    }
    let mut changes: HashMap<Url, Vec<TextEdit>> = HashMap::new();
    for (doc_uri, doc_text) in docs {
        let edits = module_binding_edits(doc_text, &name, new_name);
        if !edits.is_empty() {
            changes.insert(doc_uri.clone(), edits);
        }
    }
    if changes.is_empty() {
        return None;
    }
    Some(WorkspaceEdit { changes: Some(changes), ..Default::default() })
}

/// Collect the locations of one module-level binding's declaration and uses across open documents.
fn module_binding_locations(
    docs: &[(Url, String)],
    name: &str,
    include_declaration: bool,
) -> Vec<Location> {
    let mut locations = Vec::new();
    for (doc_uri, doc_text) in docs {
        let other = analyze(doc_text);
        let decl = other.scope.module_decl(name).map(|id| other.scope.def(id).span);
        if let Some(span) = decl {
            if include_declaration {
                locations.push(Location {
                    uri: doc_uri.clone(),
                    range: other.range(span.start, span.end),
                });
            }
            for span in other.scope.binding_uses_of(name) {
                locations.push(Location {
                    uri: doc_uri.clone(),
                    range: other.range(span.start, span.end),
                });
            }
        } else {
            for span in other.scope.unresolved_name_uses(name) {
                locations.push(Location {
                    uri: doc_uri.clone(),
                    range: other.range(span.start, span.end),
                });
            }
        }
    }
    locations
}

/// Compute the rename edits for one module-level binding in one document.
fn module_binding_edits(doc_text: &str, name: &str, new_name: &str) -> Vec<TextEdit> {
    let other = analyze(doc_text);
    let mut edits = Vec::new();
    if let Some(id) = other.scope.module_decl(name) {
        let span = other.scope.def(id).span;
        edits.push(TextEdit {
            range: other.range(span.start, span.end),
            new_text: new_name.to_string(),
        });
        for span in other.scope.binding_uses_of(name) {
            edits.push(TextEdit {
                range: other.range(span.start, span.end),
                new_text: new_name.to_string(),
            });
        }
    } else {
        for span in other.scope.unresolved_name_uses(name) {
            edits.push(TextEdit {
                range: other.range(span.start, span.end),
                new_text: new_name.to_string(),
            });
        }
    }
    edits
}

/// Return true when any open document declares the module-level binding with the given name.
fn declared_in_docs(docs: &[(Url, String)], name: &str) -> bool {
    docs.iter().any(|(_, text)| analyze(text).scope.module_decl(name).is_some())
}

/// Return true when a token is an identifier or the `self` keyword.
fn is_name_token(token: &Token) -> bool {
    token.kind == TokenKind::Ident
        || (token.kind == TokenKind::KwSelfType && token.text == "self")
}

/// Render a parameter with its resolved ownership sigil and base type.
fn render_typed(name: &str, sigil: &str, base: &str) -> String {
    if base == "_" {
        name.to_string()
    } else if sigil.is_empty() {
        format!("{name}: {base}")
    } else {
        format!("{name}: {sigil} {base}")
    }
}

/// Render the resolved mode for prose, including the inferred hint when it differs.
fn ownership_entry(resolved: ParamMode, hint: Option<ParamMode>) -> String {
    let display = mode_name(resolved);
    match hint {
        Some(h) if h != resolved => format!("{display} (inferred {})", mode_name(h)),
        _ => display,
    }
}

/// Return the Rust-style rendering of a resolved parameter mode.
fn mode_sigil(mode: ParamMode) -> &'static str {
    match mode {
        ParamMode::Borrow => "&",
        ParamMode::BorrowMut => "&mut",
        ParamMode::Move => "move",
        ParamMode::Value => "",
    }
}

/// Return the prose name of a resolved parameter mode.
fn mode_name(mode: ParamMode) -> String {
    let sigil = mode_sigil(mode);
    if sigil.is_empty() {
        "value (Copy)".to_string()
    } else {
        format!("`{sigil}`")
    }
}

/// Wrap one signature line in an axol-fenced markdown block.
fn fenced(signature: &str) -> String {
    format!("```axol\n{signature}\n```")
}
