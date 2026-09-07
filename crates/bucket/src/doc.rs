// Owner: PascalElixir / axolrs (GitHub org)
// File: crates/bucket/src/doc.rs - the Ambystoma documentation generator for .axol projects.

use crate::build::collect_axol_files;
use axolc_core::ast::{
    ConstItem, EnumItem, EnumVariant, Expr, FnItem, InterfaceItem, Item, MethodItem, MethodSig,
    Module, Param, StructField, StructItem, TypeAliasItem, TypeAnnot, TypeAnnotKind,
};
use axolc_core::lexer::token::TokenKind;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

/// Generate markdown documentation for every .axol file in the project with Ambystoma.
pub fn doc_cmd() {
    let dir = std::env::current_dir().unwrap();
    let files = collect_axol_files(&dir);
    if files.is_empty() {
        eprintln!("Ambystoma: no .axol sources found");
        std::process::exit(1);
    }
    let mut stem_counts: HashMap<String, usize> = HashMap::new();
    for f in &files {
        let stem = stem_of(f);
        *stem_counts.entry(stem).or_insert(0) += 1;
    }
    fs::create_dir_all("doc").unwrap();
    let doc_dir = Path::new("doc");
    for f in &files {
        let src = fs::read_to_string(f).unwrap_or_default();
        let rel = relative_path_string(&dir, f);
        let (module, diags) = axolc_core::parse(&src, 0);
        if diags.has_errors() {
            eprintln!("Ambystoma: {} has parse errors; documenting the items that parsed", rel);
        }
        let name = module_file_name(f, &dir, &stem_counts);
        let md = generate_module_md(&name, &rel, &src, &module);
        fs::write(doc_dir.join(format!("{}.md", name)), md).unwrap();
        println!("Ambystoma: doc/{}.md <- {}", name, rel);
    }
    println!("Ambystoma: wrote {} doc file(s) under doc/", files.len());
}

/// Render the complete Ambystoma markdown page for one parsed module.
pub fn generate_module_md(module_name: &str, rel_path: &str, src: &str, module: &Module) -> String {
    let mut b = MdBuilder::new();
    b.line(&format!("# {}", module_name));
    b.blank();
    b.line(&format!("> Source: `{}`", rel_path));
    b.blank();
    let mut fns: Vec<&FnItem> = Vec::new();
    let mut structs: Vec<&StructItem> = Vec::new();
    let mut enums: Vec<&EnumItem> = Vec::new();
    let mut interfaces: Vec<&InterfaceItem> = Vec::new();
    let mut consts: Vec<&ConstItem> = Vec::new();
    let mut methods: Vec<&MethodItem> = Vec::new();
    let mut aliases: Vec<&TypeAliasItem> = Vec::new();
    for item in &module.items {
        match item {
            Item::Fn(f) => fns.push(f),
            Item::Struct(s) => structs.push(s),
            Item::Enum(e) => enums.push(e),
            Item::Interface(i) => interfaces.push(i),
            Item::Const(c) => consts.push(c),
            Item::Method(m) => methods.push(m),
            Item::StaticMethod(m) => methods.push(m),
            Item::TypeAlias(t) => aliases.push(t),
            _ => {}
        }
    }
    if !fns.is_empty() {
        b.section("Functions");
        for f in fns {
            b.item(&fn_signature(f), doc_comment_above(src, f.span.start as usize), Vec::new());
        }
    }
    if !structs.is_empty() {
        b.section("Structs");
        for s in structs {
            let details = s.fields.iter().map(field_line).collect();
            b.item(&struct_signature(s), doc_comment_above(src, s.span.start as usize), details);
        }
    }
    if !enums.is_empty() {
        b.section("Enums");
        for e in enums {
            let details = e.variants.iter().map(enum_variant_line).collect();
            b.item(&enum_signature(e), doc_comment_above(src, e.span.start as usize), details);
        }
    }
    if !interfaces.is_empty() {
        b.section("Interfaces");
        for i in interfaces {
            let details = i.methods.iter().map(interface_method_line).collect();
            b.item(&interface_signature(i), doc_comment_above(src, i.span.start as usize), details);
        }
    }
    if !consts.is_empty() {
        b.section("Constants");
        for c in consts {
            b.item(&const_signature(c), doc_comment_above(src, c.span.start as usize), Vec::new());
        }
    }
    if !methods.is_empty() {
        b.section("Methods");
        for m in methods {
            b.item(&method_signature(m), doc_comment_above(src, m.span.start as usize), Vec::new());
        }
    }
    if !aliases.is_empty() {
        b.section("Type Aliases");
        for t in aliases {
            b.item(&type_alias_signature(t), doc_comment_above(src, t.span.start as usize), Vec::new());
        }
    }
    b.finish()
}

/// Render a type annotation as Axolotl source syntax.
pub fn render_type(t: &TypeAnnot) -> String {
    match &t.kind {
        TypeAnnotKind::Named(id, args) => {
            if args.is_empty() {
                id.name.clone()
            } else {
                let rendered: Vec<String> = args.iter().map(render_type).collect();
                format!("{}<{}>", id.name, rendered.join(", "))
            }
        }
        TypeAnnotKind::Primitive(tk) => primitive_name(*tk),
        TypeAnnotKind::Optional(inner) => format!("{}?", render_type(inner)),
        TypeAnnotKind::Array(inner) => format!("[{}]", render_type(inner)),
        TypeAnnotKind::Map(k, v) => format!("Map({}, {})", render_type(k), render_type(v)),
        TypeAnnotKind::Set(inner) => format!("Set({})", render_type(inner)),
        TypeAnnotKind::Tuple(elems) => {
            let rendered: Vec<String> = elems.iter().map(render_type).collect();
            format!("({})", rendered.join(", "))
        }
        TypeAnnotKind::Function(params, ret) => {
            let rendered: Vec<String> = params.iter().map(render_type).collect();
            format!("fn({}) -> {}", rendered.join(", "), render_type(ret))
        }
        TypeAnnotKind::Borrow(inner, true) => format!("&mut {}", render_type(inner)),
        TypeAnnotKind::Borrow(inner, false) => format!("& {}", render_type(inner)),
        TypeAnnotKind::Move(inner) => format!("move {}", render_type(inner)),
        TypeAnnotKind::Infer => "_".to_string(),
        TypeAnnotKind::SelfType => "Self".to_string(),
    }
}

/// Map a primitive token kind to its source-level type name.
fn primitive_name(tk: TokenKind) -> String {
    match tk {
        TokenKind::KwInt => "Int",
        TokenKind::KwUInt => "UInt",
        TokenKind::KwFloat => "Float",
        TokenKind::KwDouble => "Double",
        TokenKind::KwBool => "Bool",
        TokenKind::KwString => "String",
        TokenKind::KwChar => "Char",
        TokenKind::KwByte => "Byte",
        TokenKind::KwI8 => "I8",
        TokenKind::KwI16 => "I16",
        TokenKind::KwI32 => "I32",
        TokenKind::KwI64 => "I64",
        TokenKind::KwI128 => "I128",
        TokenKind::KwU8 => "U8",
        TokenKind::KwU16 => "U16",
        TokenKind::KwU32 => "U32",
        TokenKind::KwU64 => "U64",
        TokenKind::KwU128 => "U128",
        TokenKind::KwF32 => "F32",
        TokenKind::KwF64 => "F64",
        TokenKind::KwStr => "&str",
        _ => "?",
    }
    .to_string()
}

/// Render a top-level function's full signature.
pub fn fn_signature(f: &FnItem) -> String {
    let mut s = String::new();
    if f.is_pub {
        s.push_str("pub ");
    }
    if f.is_async {
        s.push_str("async ");
    }
    s.push_str("fn ");
    s.push_str(&f.name.name);
    s.push_str(&generics_text(&f.generics));
    s.push('(');
    s.push_str(&render_params(&f.params));
    s.push(')');
    s.push_str(&ret_suffix(f.ret.as_ref()));
    if f.is_extern {
        s.push(';');
    }
    s
}

/// Render an impl method or static method signature.
pub fn method_signature(m: &MethodItem) -> String {
    let mut s = String::new();
    if m.is_async {
        s.push_str("async ");
    }
    s.push_str(&m.receiver.name);
    s.push('.');
    s.push_str(&m.name.name);
    s.push('(');
    s.push_str(&render_params(&m.params));
    s.push(')');
    s.push_str(&ret_suffix(m.ret.as_ref()));
    s
}

/// Render one interface method signature.
pub fn interface_method_signature(m: &MethodSig) -> String {
    format!("{}({}){}", m.name.name, render_params(&m.params), ret_suffix(m.ret.as_ref()))
}

/// Render a struct declaration signature.
pub fn struct_signature(s: &StructItem) -> String {
    format!("{}struct {}{}", pub_prefix(s.is_pub), s.name.name, generics_text(&s.generics))
}

/// Render an enum declaration signature.
pub fn enum_signature(e: &EnumItem) -> String {
    format!("{}enum {}{}", pub_prefix(e.is_pub), e.name.name, generics_text(&e.generics))
}

/// Render an interface declaration signature.
pub fn interface_signature(i: &InterfaceItem) -> String {
    format!("{}interface {}{}", pub_prefix(i.is_pub), i.name.name, generics_text(&i.generics))
}

/// Render a const declaration with its literal value when it is a literal.
pub fn const_signature(c: &ConstItem) -> String {
    let mut s = format!("{}const {}", pub_prefix(c.is_pub), c.name.name);
    if let Some(t) = &c.ty {
        s.push_str(": ");
        s.push_str(&render_type(t));
    }
    if let Some(lit) = literal_text(&c.value) {
        s.push_str(" = ");
        s.push_str(&lit);
    }
    s
}

/// Render a type alias declaration signature.
pub fn type_alias_signature(t: &TypeAliasItem) -> String {
    format!("{}type {}{} = {}", pub_prefix(t.is_pub), t.name.name, generics_text(&t.generics), render_type(&t.ty))
}

/// Render one struct field as a documentation list line.
pub fn field_line(f: &StructField) -> String {
    format!("- `{}`: {}", f.name.name, render_type(&f.ty))
}

/// Render one enum variant as a documentation list line.
pub fn enum_variant_line(v: &EnumVariant) -> String {
    if v.fields.is_empty() {
        format!("- `{}`", v.name.name)
    } else {
        let parts: Vec<String> = v
            .fields
            .iter()
            .map(|f| match &f.name {
                Some(n) => format!("{}: {}", n.name, render_type(&f.ty)),
                None => render_type(&f.ty),
            })
            .collect();
        format!("- `{}({})`", v.name.name, parts.join(", "))
    }
}

/// Render one interface method as a documentation list line.
fn interface_method_line(m: &MethodSig) -> String {
    format!("- `{}`", interface_method_signature(m))
}

/// Render an expression's literal text when it is a simple literal.
pub fn literal_text(e: &Expr) -> Option<String> {
    match e {
        Expr::IntLit(s, _) => Some(s.clone()),
        Expr::FloatLit(s, _) => Some(s.clone()),
        Expr::StrLit(s, _) => Some(format!("{:?}", s)),
        Expr::CharLit(s, _) => Some(format!("'{}'", s)),
        Expr::BoolLit(b, _) => Some(b.to_string()),
        Expr::RawStrLit(s, _) => Some(format!("`{}`", s)),
        Expr::Null(_) => Some("null".to_string()),
        _ => None,
    }
}

/// Extract the `--` doc comment lines immediately above a byte offset, joined with spaces.
pub fn doc_comment_above(src: &str, start: usize) -> Option<String> {
    let start = start.min(src.len());
    let prefix = &src[..start];
    let line_start = prefix.rfind('\n').map(|i| i + 1).unwrap_or(0);
    if !prefix[line_start..].trim().is_empty() {
        return None;
    }
    if line_start == 0 {
        return None;
    }
    let above = &src[..line_start - 1];
    let lines: Vec<&str> = above.split('\n').collect();
    let mut collected: Vec<&str> = Vec::new();
    for line in lines.iter().rev() {
        let t = line.trim();
        if t.starts_with("--[[") {
            break;
        }
        if let Some(rest) = t.strip_prefix("--") {
            collected.push(rest.trim());
        } else {
            break;
        }
    }
    let joined = collected
        .iter()
        .rev()
        .filter(|l| !l.is_empty())
        .cloned()
        .collect::<Vec<_>>()
        .join(" ");
    if joined.is_empty() {
        None
    } else {
        Some(joined)
    }
}

/// Render a parameter list with names and optional types.
pub fn render_params(params: &[Param]) -> String {
    params
        .iter()
        .map(|p| match &p.ty {
            Some(t) => format!("{}: {}", p.name.name, render_type(t)),
            None => p.name.name.clone(),
        })
        .collect::<Vec<_>>()
        .join(", ")
}

/// Render generic parameter names as `<T, U>` or an empty string.
fn generics_text(idents: &[axolc_core::ast::Ident]) -> String {
    if idents.is_empty() {
        String::new()
    } else {
        let names: Vec<&str> = idents.iter().map(|g| g.name.as_str()).collect();
        format!("<{}>", names.join(", "))
    }
}

/// Render an optional return type as ` -> T` or an empty string.
fn ret_suffix(ret: Option<&TypeAnnot>) -> String {
    match ret {
        Some(t) => format!(" -> {}", render_type(t)),
        None => String::new(),
    }
}

/// Render the `pub ` prefix for visibility.
fn pub_prefix(is_pub: bool) -> &'static str {
    if is_pub {
        "pub "
    } else {
        ""
    }
}

/// A line-oriented markdown builder used by the Ambystoma generator.
pub struct MdBuilder {
    lines: Vec<String>,
}

impl MdBuilder {
    /// Create an empty markdown builder.
    pub fn new() -> MdBuilder {
        MdBuilder { lines: Vec::new() }
    }

    /// Append one content line.
    pub fn line(&mut self, text: &str) {
        self.lines.push(text.to_string());
    }

    /// Append a blank separator line.
    pub fn blank(&mut self) {
        self.lines.push(String::new());
    }

    /// Append a section heading.
    pub fn section(&mut self, title: &str) {
        self.line(&format!("## {}", title));
        self.blank();
    }

    /// Append an item heading with its optional description and detail lines.
    pub fn item(&mut self, signature: &str, description: Option<String>, details: Vec<String>) {
        self.line(&format!("### `{}`", signature));
        self.blank();
        if let Some(d) = description {
            self.line(&d);
            self.blank();
        }
        let has_details = !details.is_empty();
        for det in details {
            self.line(&det);
        }
        if has_details {
            self.blank();
        }
    }

    /// Finish the document: strip trailing blanks, join lines, end with a newline.
    pub fn finish(mut self) -> String {
        while self.lines.last().is_some_and(|l| l.is_empty()) {
            self.lines.pop();
        }
        let mut out = self.lines.join("\n");
        out.push('\n');
        out
    }
}

/// Choose the doc file name for one .axol source file, avoiding stem collisions.
pub fn module_file_name(file: &Path, root: &Path, stem_counts: &HashMap<String, usize>) -> String {
    let stem = stem_of(file);
    if stem_counts.get(&stem).copied().unwrap_or(0) > 1 {
        slug_path(file, root)
    } else {
        stem
    }
}

/// Extract a file's stem as a lossy string.
fn stem_of(file: &Path) -> String {
    file.file_stem().map(|s| s.to_string_lossy().to_string()).unwrap_or_default()
}

/// Render a file's path relative to root with separators replaced by dashes.
fn slug_path(file: &Path, root: &Path) -> String {
    let rel = file.strip_prefix(root).unwrap_or(file);
    let text = rel.to_string_lossy().replace(['/', '\\'], "-");
    text.strip_suffix(".axol").unwrap_or(&text).to_string()
}

/// Render a path relative to root, falling back to the full display path.
pub fn relative_path_string(root: &Path, path: &Path) -> String {
    match path.strip_prefix(root) {
        Ok(rel) => rel.to_string_lossy().to_string(),
        Err(_) => path.display().to_string(),
    }
}

/// Collect the .axol files under a directory root for external callers.
pub fn axol_files_under(dir: &Path) -> Vec<PathBuf> {
    collect_axol_files(dir)
}
