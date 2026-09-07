// Owner: PascalElixir / axolrs (GitHub org)
// File: crates/bucket/tests/doc_tests.rs - Ambystoma documentation generator tests.

use bucket::doc::{
    doc_comment_above, enum_signature, enum_variant_line, const_signature, field_line,
    fn_signature, generate_module_md, interface_method_signature, interface_signature,
    literal_text, method_signature, module_file_name, MdBuilder, render_type, struct_signature,
    type_alias_signature,
};
use axolc_core::ast::{Item, Module, TypeAnnot, TypeAnnotKind};
use axolc_core::span::Span;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn bucket_binary() -> String {
    let mut path = std::env::current_exe().unwrap();
    path.pop();
    path.pop();
    path.join("bucket").to_string_lossy().to_string()
}

/// Run the bucket binary with arguments in a working directory.
fn run_bucket_in(dir: &Path, args: &[&str]) -> (String, String, i32) {
    let output = Command::new(bucket_binary())
        .args(args)
        .current_dir(dir)
        .output()
        .expect("run bucket");
    (
        String::from_utf8_lossy(&output.stdout).to_string(),
        String::from_utf8_lossy(&output.stderr).to_string(),
        output.status.code().unwrap_or(-1),
    )
}

/// Create a unique temp directory for a test scenario.
fn temp_dir(tag: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("bucket-doc-{}-{}", tag, std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    dir
}

/// Parse a source string and assert it produced no diagnostics.
fn parse_clean(src: &str) -> Module {
    let (module, diags) = axolc_core::parse(src, 0);
    assert!(!diags.has_errors(), "fixture should parse: {src}\n{:?}", diags.items);
    module
}

/// Return the source span of a module item.
fn item_span(item: &Item) -> Span {
    match item {
        Item::Fn(f) => f.span,
        Item::Struct(s) => s.span,
        Item::Enum(e) => e.span,
        Item::Interface(i) => i.span,
        Item::Const(c) => c.span,
        Item::Method(m) => m.span,
        Item::StaticMethod(m) => m.span,
        Item::TypeAlias(t) => t.span,
        _ => Span::new(0, 0, 0),
    }
}

/// Return the rendered type of the first parameter of a one-parameter function.
fn param_type_of(src: &str) -> String {
    let module = parse_clean(&format!("fn probe(x: {src})\nend"));
    match &module.items[0] {
        Item::Fn(f) => render_type(f.params[0].ty.as_ref().expect("typed param")),
        _ => panic!("expected a fn item"),
    }
}

/// Return the rendered return type of a function fixture.
fn ret_type_of(src: &str) -> String {
    let module = parse_clean(&format!("fn probe() -> {src}\nend"));
    match &module.items[0] {
        Item::Fn(f) => render_type(f.ret.as_ref().expect("typed return")),
        _ => panic!("expected a fn item"),
    }
}

/// Return the first fn item of a parsed module.
fn first_fn(module: &Module) -> &axolc_core::ast::FnItem {
    for item in &module.items {
        if let Item::Fn(f) = item {
            return f;
        }
    }
    panic!("no fn item found");
}

/// Return the first struct item of a parsed module.
fn first_struct(module: &Module) -> &axolc_core::ast::StructItem {
    for item in &module.items {
        if let Item::Struct(s) = item {
            return s;
        }
    }
    panic!("no struct item found");
}

/// Return the first enum item of a parsed module.
fn first_enum(module: &Module) -> &axolc_core::ast::EnumItem {
    for item in &module.items {
        if let Item::Enum(e) = item {
            return e;
        }
    }
    panic!("no enum item found");
}

/// Return the first interface item of a parsed module.
fn first_interface(module: &Module) -> &axolc_core::ast::InterfaceItem {
    for item in &module.items {
        if let Item::Interface(i) = item {
            return i;
        }
    }
    panic!("no interface item found");
}

/// Return the first const item of a parsed module.
fn first_const(module: &Module) -> &axolc_core::ast::ConstItem {
    for item in &module.items {
        if let Item::Const(c) = item {
            return c;
        }
    }
    panic!("no const item found");
}

/// Return the first method item of a parsed module.
fn first_method(module: &Module) -> &axolc_core::ast::MethodItem {
    for item in &module.items {
        if let Item::Method(m) = item {
            return m;
        }
    }
    panic!("no method item found");
}

/// Build a type annotation by hand for kinds the parser cannot produce.
fn manual_type(kind: TypeAnnotKind) -> TypeAnnot {
    TypeAnnot { kind, span: Span::new(0, 0, 0) }
}

/// Build a named type by hand.
fn manual_named(name: &str) -> TypeAnnot {
    manual_type(TypeAnnotKind::Named(
        axolc_core::ast::Ident { name: name.to_string(), span: Span::new(0, 0, 0) },
        vec![],
    ))
}

/// Parameter types render exactly as their source-level Axolotl syntax.
#[test]
fn render_type_table_from_parsed_sources() {
    let cases: Vec<(&str, &str)> = vec![
        ("Int", "Int"),
        ("UInt", "UInt"),
        ("Float", "Float"),
        ("Double", "Float"),
        ("Bool", "Bool"),
        ("String", "String"),
        ("Char", "Char"),
        ("Byte", "Byte"),
        ("I8", "I8"),
        ("I16", "I16"),
        ("I32", "I32"),
        ("I64", "I64"),
        ("I128", "I128"),
        ("U8", "U8"),
        ("U16", "U16"),
        ("U32", "U32"),
        ("U64", "U64"),
        ("U128", "U128"),
        ("F32", "F32"),
        ("F64", "F64"),
        ("Player", "Player"),
        ("Vec<Int>", "Vec<Int>"),
        ("Vec<Vec<Int> >", "Vec<Vec<Int>>"),
        ("HashMap<String, Int>", "HashMap<String, Int>"),
        ("Int?", "Int?"),
        ("Player?", "Player?"),
        ("[Int]", "[Int]"),
        ("[[Int]]", "[[Int]]"),
        ("[Player?]", "[Player?]"),
        ("(Int, String)", "(Int, String)"),
        ("(Int)", "(Int)"),
        ("& Player", "& Player"),
        ("& var Player", "&mut Player"),
        ("borrow Int", "& Int"),
        ("borrow var Int", "&mut Int"),
        ("move Int", "move Int"),
        ("move [String]", "move [String]"),
        ("& var & var Player", "&mut &mut Player"),
        ("fn(Int) -> Int", "fn(Int) -> Int"),
        ("fn() -> String", "fn() -> String"),
        ("fn(Int, Int) -> Bool", "fn(Int, Int) -> Bool"),
        ("fn(Int) -> fn(Int) -> Int", "fn(Int) -> fn(Int) -> Int"),
        ("Self", "Self"),
        ("[Self]", "[Self]"),
        ("& var Self", "&mut Self"),
    ];
    for (src, expected) in cases {
        assert_eq!(param_type_of(src), expected, "param type {src}");
    }
}

/// Return types render exactly as their source-level Axolotl syntax.
#[test]
fn render_type_table_from_return_positions() {
    let cases: Vec<(&str, &str)> = vec![
        ("Int", "Int"),
        ("String", "String"),
        ("Player", "Player"),
        ("[Int]", "[Int]"),
        ("& var Player", "&mut Player"),
        ("& Player", "& Player"),
        ("move Int", "move Int"),
        ("Int?", "Int?"),
        ("fn(Int) -> Int", "fn(Int) -> Int"),
        ("(Int, Bool)", "(Int, Bool)"),
        ("Vec<String>", "Vec<String>"),
    ];
    for (src, expected) in cases {
        assert_eq!(ret_type_of(src), expected, "return type {src}");
    }
}

/// Map, Set, Infer, and SelfType render through hand-built annotations.
#[test]
fn render_type_manual_kinds() {
    assert_eq!(
        render_type(&manual_type(TypeAnnotKind::Map(
            Box::new(manual_named("String")),
            Box::new(manual_named("Int"))
        ))),
        "Map(String, Int)"
    );
    assert_eq!(
        render_type(&manual_type(TypeAnnotKind::Set(Box::new(manual_named("Int"))))),
        "Set(Int)"
    );
    assert_eq!(render_type(&manual_type(TypeAnnotKind::Infer)), "_");
    assert_eq!(render_type(&manual_type(TypeAnnotKind::SelfType)), "Self");
    assert_eq!(
        render_type(&manual_type(TypeAnnotKind::Optional(Box::new(manual_named("Player"))))),
        "Player?"
    );
    assert_eq!(
        render_type(&manual_type(TypeAnnotKind::Array(Box::new(manual_type(
            TypeAnnotKind::Optional(Box::new(manual_named("Int")))
        ))))),
        "[Int?]"
    );
    assert_eq!(
        render_type(&manual_type(TypeAnnotKind::Tuple(vec![
            manual_named("Int"),
            manual_type(TypeAnnotKind::Array(Box::new(manual_named("String")))),
        ]))),
        "(Int, [String])"
    );
}

/// Function signatures render with names, params, returns, and modifiers.
#[test]
fn fn_signature_table() {
    let cases: Vec<(&str, &str)> = vec![
        ("fn main()\nend", "fn main()"),
        ("fn add(a: Int, b: Int) -> Int\n    return a\nend", "fn add(a: Int, b: Int) -> Int"),
        ("fn ident(x: Int) -> Int\n    return x\nend", "fn ident(x: Int) -> Int"),
        ("fn bare(x)\n    print(x)\nend", "fn bare(x)"),
        ("fn three(a: Int, b: String, c: Bool)\nend", "fn three(a: Int, b: String, c: Bool)"),
        ("fn ret() -> [Int]\nend", "fn ret() -> [Int]"),
        ("fn refret() -> & var Player\nend", "fn refret() -> &mut Player"),
        ("fn opt(x: Int?) -> Int?\nend", "fn opt(x: Int?) -> Int?"),
        ("pub fn visible()\nend", "pub fn visible()"),
        ("pub fn vis(x: Int) -> String\nend", "pub fn vis(x: Int) -> String"),
        ("async fn later()\nend", "async fn later()"),
        ("async fn later2(x: Int)\nend", "async fn later2(x: Int)"),
        ("fn gen<T>(x: T) -> T\n    return x\nend", "fn gen<T>(x: T) -> T"),
        ("fn gen2<T, U>(t: T, u: U)\nend", "fn gen2<T, U>(t: T, u: U)"),
        ("fn ext(x: Int);", "fn ext(x: Int);"),
        ("fn defaulted(x: Int = 5)\nend", "fn defaulted(x: Int)"),
        ("fn fnty(cb: fn(Int) -> Int)\nend", "fn fnty(cb: fn(Int) -> Int)"),
        ("fn tuple(x: (Int, String))\nend", "fn tuple(x: (Int, String))"),
        ("fn many(a: Int, b: Int, c: Int, d: Int)\nend", "fn many(a: Int, b: Int, c: Int, d: Int)"),
        ("fn selfty(x: Self)\nend", "fn selfty(x: Self)"),
    ];
    for (src, expected) in cases {
        let module = parse_clean(src);
        assert_eq!(fn_signature(first_fn(&module)), expected, "signature of {src}");
    }
}

/// Method and static method signatures render with receiver-qualified names.
#[test]
fn method_signature_table() {
    let cases: Vec<(&str, &str)> = vec![
        (
            "Player.damage = fn(self, amount: Int)\n    print(1)\nend",
            "Player.damage(self, amount: Int)",
        ),
        (
            "Player.alive = fn(self) -> Bool\n    return true\nend",
            "Player.alive(self) -> Bool",
        ),
        (
            "Player.create = fn(name: String) -> Player\n    return Player { }\nend",
            "Player.create(name: String) -> Player",
        ),
        (
            "Player.reset = fn(self)\nend",
            "Player.reset(self)",
        ),
        (
            "Rect.area = fn(self) -> Float\nend",
            "Rect.area(self) -> Float",
        ),
        (
            "Game.start = fn()\nend",
            "Game.start()",
        ),
    ];
    for (src, expected) in cases {
        let module = parse_clean(src);
        assert_eq!(method_signature(first_method(&module)), expected, "method signature of {src}");
    }
}

/// Struct signatures render the declaration without fields.
#[test]
fn struct_signature_table() {
    let cases: Vec<(&str, &str)> = vec![
        ("Point = struct\n    x: Int\nend", "struct Point"),
        ("Empty = struct\nend", "struct Empty"),
        ("Pair = struct<T>\n    a: T\nend", "struct Pair<T>"),
        ("Pair = struct<T, U>\n    a: T\nend", "struct Pair<T, U>"),
    ];
    for (src, expected) in cases {
        let module = parse_clean(src);
        assert_eq!(struct_signature(first_struct(&module)), expected, "struct signature of {src}");
    }
}

/// Struct fields render as list lines with types.
#[test]
fn field_line_table() {
    let module = parse_clean("P = struct\n    x: Int\n    name: String\n    tags: [String]\nend");
    let s = first_struct(&module);
    assert_eq!(field_line(&s.fields[0]), "- `x`: Int");
    assert_eq!(field_line(&s.fields[1]), "- `name`: String");
    assert_eq!(field_line(&s.fields[2]), "- `tags`: [String]");
}

/// Enum signatures and variants render with and without payload fields.
#[test]
fn enum_signature_and_variant_table() {
    let module = parse_clean(
        "Shape = enum\n    Circle\n    Rect(w: Int, h: Int)\n    Named(String, Int)\nend",
    );
    let e = first_enum(&module);
    assert_eq!(enum_signature(e), "enum Shape");
    assert_eq!(enum_variant_line(&e.variants[0]), "- `Circle`");
    assert_eq!(enum_variant_line(&e.variants[1]), "- `Rect(w: Int, h: Int)`");
    assert_eq!(enum_variant_line(&e.variants[2]), "- `Named(String, Int)`");
    let empty = parse_clean("Unit = enum\n    A\nend");
    let empty_enum = first_enum(&empty);
    assert_eq!(enum_signature(empty_enum), "enum Unit");
    assert_eq!(empty_enum.variants.len(), 1);
    assert_eq!(enum_variant_line(&empty_enum.variants[0]), "- `A`");
}

/// Interface signatures and method lines render.
#[test]
fn interface_signature_table() {
    let module = parse_clean(
        "Drawable = interface\n    draw(self, x: Int) -> Bool\n    area(self) -> Float\nend",
    );
    let i = first_interface(&module);
    assert_eq!(interface_signature(i), "interface Drawable");
    assert_eq!(interface_method_signature(&i.methods[0]), "draw(self, x: Int) -> Bool");
    assert_eq!(interface_method_signature(&i.methods[1]), "area(self) -> Float");
    assert_eq!(i.methods.len(), 2);
    let bare = parse_clean("Runnable = interface\n    run(self)\nend");
    let bare_iface = first_interface(&bare);
    assert_eq!(interface_signature(bare_iface), "interface Runnable");
    assert_eq!(interface_method_signature(&bare_iface.methods[0]), "run(self)");
}

/// Const signatures render types and literal values.
#[test]
fn const_signature_table() {
    let cases: Vec<(&str, &str)> = vec![
        ("const MAX: Int = 3", "const MAX: Int = 3"),
        ("const GREETING: String = \"hi\"", "const GREETING: String = \"hi\""),
        ("const RATIO: Float = 0.5", "const RATIO: Float = 0.5"),
        ("const FLAG: Bool = true", "const FLAG: Bool = true"),
        ("const LETTER: Char = 'a'", "const LETTER: Char = 'a'"),
        ("const UNTYPED = 7", "const UNTYPED = 7"),
        ("const CHANGED: Int = 1 + 2", "const CHANGED: Int"),
        ("const NAMED = MAX", "const NAMED"),
    ];
    for (src, expected) in cases {
        let module = parse_clean(src);
        assert_eq!(const_signature(first_const(&module)), expected, "const signature of {src}");
    }
}

/// Literal extraction recognizes simple literals and rejects expressions.
#[test]
fn literal_text_table() {
    let cases: Vec<(&str, Option<&str>)> = vec![
        ("const A: Int = 3", Some("3")),
        ("const B: Float = 0.25", Some("0.25")),
        ("const C: String = \"word\"", Some("\"word\"")),
        ("const D: Bool = false", Some("false")),
        ("const E: Char = 'z'", Some("'z'")),
        ("const F: Int = 1 + 2", None),
        ("const G = OTHER", None),
    ];
    for (src, expected) in cases {
        let module = parse_clean(src);
        let actual = literal_text(&first_const(&module).value);
        assert_eq!(
            actual.as_deref(),
            expected,
            "literal text of {src}"
        );
    }
}

/// Type alias signatures render the aliased type.
#[test]
fn type_alias_signature_table() {
    let module = parse_clean("type Meters = Int");
    for item in &module.items {
        if let Item::TypeAlias(t) = item {
            assert_eq!(type_alias_signature(t), "type Meters = Int");
        }
    }
    let generic = parse_clean("type Box2<T> = [T]");
    for item in &generic.items {
        if let Item::TypeAlias(t) = item {
            assert_eq!(type_alias_signature(t), "type Box2<T> = [T]");
        }
    }
}

/// Doc comment extraction table: (source, item index, expected description).
#[test]
fn doc_comment_above_table() {
    let cases: Vec<(&str, usize, Option<&str>)> = vec![
        ("-- doc\nfn f()\nend", 0, Some("doc")),
        ("fn f()\nend", 0, None),
        ("-- a\n-- b\nfn f()\nend", 0, Some("a b")),
        ("-- doc\n\nfn f()\nend", 0, None),
        ("-- far\n\n-- near\nfn f()\nend", 0, Some("near")),
        ("  -- indented\nfn f()\nend", 0, Some("indented")),
        ("--[[ block ]]\nfn f()\nend", 0, None),
        ("-- doc\n--[[ block ]]\nfn f()\nend", 0, None),
        ("--[[ block ]]\n-- doc\nfn f()\nend", 0, Some("doc")),
        ("--\n-- doc\nfn f()\nend", 0, Some("doc")),
        ("-- doc\n--\nfn f()\nend", 0, Some("doc")),
        ("-- doc-- dashes\nfn f()\nend", 0, Some("doc-- dashes")),
        ("-- doc with  spaces \nfn f()\nend", 0, Some("doc with  spaces")),
        ("fn g()\nend\n-- doc\nfn f()\nend", 1, Some("doc")),
        ("fn g()\nend\nfn h()\nend\n-- doc\nfn f()\nend", 2, Some("doc")),
        ("-- header\n\n-- doc\nfn f()\nend", 0, Some("doc")),
        ("-- module header\n\nfn f()\nend", 0, None),
        ("-- doc\r\nfn f()\r\nend\r\n", 0, Some("doc")),
        ("-- one\n-- two\n-- three\nfn f()\nend", 0, Some("one two three")),
        ("-- doc\nPoint = struct\n    x: Int\nend", 0, Some("doc")),
        ("-- doc\nColor = enum\n    Red\nend", 0, Some("doc")),
        ("-- doc\nShape = interface\n    area(self)\nend", 0, Some("doc")),
        ("-- doc\nconst K: Int = 1", 0, Some("doc")),
        ("-- doc\nPlayer.hit = fn(self)\nend", 0, Some("doc")),
        ("-- first\nfn g()\nend\n-- second\nfn f()\nend", 1, Some("second")),
        ("-- first\nfn g()\nend\n-- second\nfn f()\nend", 0, Some("first")),
    ];
    for (src, index, expected) in cases {
        let module = parse_clean(src);
        let start = item_span(&module.items[index]);
        let actual = doc_comment_above(src, start.start as usize);
        assert_eq!(
            actual.as_deref(),
            expected,
            "doc comment for item {index} of {src:?}"
        );
    }
}

/// A struct fixture renders a complete module page with sections and fields.
#[test]
fn generate_module_md_struct_page() {
    let src = "-- A 2D point.\nPoint = struct\n    x: Int\n    y: Int\nend\n";
    let module = parse_clean(src);
    let md = generate_module_md("lib", "src/lib.axol", src, &module);
    let expected = "# lib\n\n> Source: `src/lib.axol`\n\n## Structs\n\n### `struct Point`\n\nA 2D point.\n\n- `x`: Int\n- `y`: Int\n";
    assert_eq!(md, expected);
}

/// A function fixture renders header, signature, and description.
#[test]
fn generate_module_md_fn_page() {
    let src = "-- Computes fib.\n-- Recursive.\nfn fib(n: Int) -> Int\n    return n\nend\n";
    let module = parse_clean(src);
    let md = generate_module_md("fib", "src/fib.axol", src, &module);
    let expected =
        "# fib\n\n> Source: `src/fib.axol`\n\n## Functions\n\n### `fn fib(n: Int) -> Int`\n\nComputes fib. Recursive.\n";
    assert_eq!(md, expected);
}

/// A mixed fixture renders every section in order.
#[test]
fn generate_module_md_mixed_page() {
    let src = "-- A library.\n\n-- A 2D point.\nPoint = struct\n    x: Int\nend\n\n-- Shapes.\nShape = enum\n    Circle\n    Rect(w: Int, h: Int)\nend\n\n-- Drawable things.\nDrawable = interface\n    draw(self) -> Bool\nend\n\n-- Max tries.\nconst MAX: Int = 3\n\n-- Scale.\nfn scale(k: Int) -> Int\n    return k\nend\n\nPoint.show = fn(self)\nend\n";
    let module = parse_clean(src);
    let md = generate_module_md("lib", "src/lib.axol", src, &module);
    let expected = "# lib\n\n> Source: `src/lib.axol`\n\n## Functions\n\n### `fn scale(k: Int) -> Int`\n\nScale.\n\n## Structs\n\n### `struct Point`\n\nA 2D point.\n\n- `x`: Int\n\n## Enums\n\n### `enum Shape`\n\nShapes.\n\n- `Circle`\n- `Rect(w: Int, h: Int)`\n\n## Interfaces\n\n### `interface Drawable`\n\nDrawable things.\n\n- `draw(self) -> Bool`\n\n## Constants\n\n### `const MAX: Int = 3`\n\nMax tries.\n\n## Methods\n\n### `Point.show(self)`\n";
    assert_eq!(md, expected);
}

/// An empty module renders only the header.
#[test]
fn generate_module_md_empty_page() {
    let src = "-- Just a header comment.\n";
    let module = parse_clean(src);
    let md = generate_module_md("empty", "src/empty.axol", src, &module);
    assert_eq!(md, "# empty\n\n> Source: `src/empty.axol`\n");
}

/// Untyped params and extern fns render in signatures.
#[test]
fn generate_module_md_untyped_params() {
    let src = "fn bare(x)\nend\n";
    let module = parse_clean(src);
    let md = generate_module_md("m", "src/m.axol", src, &module);
    assert_eq!(md, "# m\n\n> Source: `src/m.axol`\n\n## Functions\n\n### `fn bare(x)`\n");
}

/// The markdown builder strips trailing blanks and ends with a newline.
#[test]
fn md_builder_finish_normalizes_the_tail() {
    let mut b = MdBuilder::new();
    b.line("# t");
    b.blank();
    b.section("S");
    b.item("fn x()", None, Vec::new());
    let out = b.finish();
    assert!(out.ends_with("### `fn x()`\n"));
    assert!(!out.ends_with("\n\n"));
    assert!(out.starts_with("# t\n"));
    assert!(out.contains("## S\n"));
}

/// The markdown builder places descriptions before detail lines.
#[test]
fn md_builder_item_layout() {
    let mut b = MdBuilder::new();
    b.item("struct P", Some("A point.".to_string()), vec!["- `x`: Int".to_string()]);
    let out = b.finish();
    assert_eq!(out, "### `struct P`\n\nA point.\n\n- `x`: Int\n");
}

/// Module file names use the stem unless it collides.
#[test]
fn module_file_name_avoids_collisions() {
    let root = Path::new("/proj");
    let a = root.join("src/main.axol");
    let b = root.join("lib/main.axol");
    let unique: HashMap<String, usize> = HashMap::from([("main".to_string(), 1)]);
    let colliding: HashMap<String, usize> = HashMap::from([("main".to_string(), 2)]);
    assert_eq!(module_file_name(&a, root, &unique), "main");
    assert_eq!(module_file_name(&a, root, &colliding), "src-main");
    assert_eq!(module_file_name(&b, root, &colliding), "lib-main");
    let lone: HashMap<String, usize> = HashMap::new();
    assert_eq!(module_file_name(&b, root, &lone), "main");
}

/// The doc CLI generates markdown files for a scaffolded project.
#[test]
fn cli_doc_generates_files_for_a_new_project() {
    let dir = temp_dir("new-project");
    let project = dir.join("proj");
    run_bucket_in(&dir, &["new", "proj"]);
    let (out, err, code) = run_bucket_in(&project, &["doc"]);
    assert_eq!(code, 0, "stderr: {err}");
    assert!(err.is_empty());
    assert!(out.contains("Ambystoma: doc/main.md <- src/main.axol"));
    assert!(out.contains("Ambystoma: doc/lib.md <- src/lib.axol"));
    assert!(out.contains("Ambystoma: wrote 2 doc file(s) under doc/"));
    let main_md = fs::read_to_string(project.join("doc/main.md")).unwrap();
    assert_eq!(main_md, "# main\n\n> Source: `src/main.axol`\n\n## Functions\n\n### `fn main()`\n");
    let lib_md = fs::read_to_string(project.join("doc/lib.md")).unwrap();
    assert_eq!(lib_md, "# lib\n\n> Source: `src/lib.axol`\n");
    let _ = fs::remove_dir_all(&dir);
}

/// The doc CLI documents a full fixture and reports each generated file.
#[test]
fn cli_doc_generates_full_fixture() {
    let dir = temp_dir("full");
    fs::write(
        dir.join("Bucket.jsonc"),
        "{ \"name\": \"d\", \"version\": \"0.1.0\", \"dependencies\": {} }",
    )
    .unwrap();
    fs::create_dir_all(dir.join("src")).unwrap();
    fs::write(
        dir.join("src/game.axol"),
        "-- A player.\nPlayer = struct\n    hp: Int\nend\n\n-- Attacks.\nfn attack(p: & var Player, n: Int)\n    p.hp = p.hp - n\nend\n",
    )
    .unwrap();
    let (out, err, code) = run_bucket_in(&dir, &["doc"]);
    assert_eq!(code, 0, "stderr: {err}");
    assert!(err.is_empty());
    assert!(out.contains("Ambystoma: doc/game.md <- src/game.axol"));
    let game_md = fs::read_to_string(dir.join("doc/game.md")).unwrap();
    assert!(game_md.contains("### `struct Player`"));
    assert!(game_md.contains("A player."));
    assert!(game_md.contains("- `hp`: Int"));
    assert!(game_md.contains("### `fn attack(p: &mut Player, n: Int)`"));
    assert!(game_md.contains("Attacks."));
    let _ = fs::remove_dir_all(&dir);
}

/// The doc CLI warns about parse errors but still documents parsed items.
#[test]
fn cli_doc_warns_on_parse_errors() {
    let dir = temp_dir("broken");
    fs::write(
        dir.join("Bucket.jsonc"),
        "{ \"name\": \"b\", \"version\": \"0.1.0\", \"dependencies\": {} }",
    )
    .unwrap();
    fs::create_dir_all(dir.join("src")).unwrap();
    fs::write(dir.join("src/broken.axol"), "fn f()\n    let x =\nend\n").unwrap();
    let (out, err, code) = run_bucket_in(&dir, &["doc"]);
    assert_eq!(code, 0);
    assert!(err.contains("src/broken.axol has parse errors"));
    assert!(out.contains("Ambystoma: doc/broken.md <- src/broken.axol"));
    assert!(dir.join("doc/broken.md").is_file());
    let _ = fs::remove_dir_all(&dir);
}

/// The doc CLI exits one when the project has no sources.
#[test]
fn cli_doc_fails_without_sources() {
    let dir = temp_dir("empty");
    fs::write(
        dir.join("Bucket.jsonc"),
        "{ \"name\": \"e\", \"version\": \"0.1.0\", \"dependencies\": {} }",
    )
    .unwrap();
    let (_out, err, code) = run_bucket_in(&dir, &["doc"]);
    assert_eq!(code, 1);
    assert!(err.contains("Ambystoma: no .axol sources found"));
    let _ = fs::remove_dir_all(&dir);
}

/// The doc CLI disambiguates same-stem files with slugged names.
#[test]
fn cli_doc_slug_same_stem_files() {
    let dir = temp_dir("stems");
    fs::create_dir_all(dir.join("a")).unwrap();
    fs::create_dir_all(dir.join("b")).unwrap();
    fs::write(dir.join("a/thing.axol"), "fn a()\nend\n").unwrap();
    fs::write(dir.join("b/thing.axol"), "fn b()\nend\n").unwrap();
    let (out, _err, code) = run_bucket_in(&dir, &["doc"]);
    assert_eq!(code, 0);
    assert!(out.contains("doc/a-thing.md <- a/thing.axol"));
    assert!(out.contains("doc/b-thing.md <- b/thing.axol"));
    assert!(dir.join("doc/a-thing.md").is_file());
    assert!(dir.join("doc/b-thing.md").is_file());
    let a_md = fs::read_to_string(dir.join("doc/a-thing.md")).unwrap();
    assert!(a_md.contains("### `fn a()`"));
    let b_md = fs::read_to_string(dir.join("doc/b-thing.md")).unwrap();
    assert!(b_md.contains("### `fn b()`"));
    let _ = fs::remove_dir_all(&dir);
}
