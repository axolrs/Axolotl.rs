// Owner: PascalElixir / axolrs (GitHub org)
// File: crates/bucket/tests/fmt_tests.rs - Shed formatter tests: canonical output, idempotence, token preservation, comment preservation, and semantic preservation.

use axolc_core::lexer::token::TokenKind;
use bucket::fmt::{format_source, line_col_of, scan_pieces, Piece};

/// Format a source string, panicking with context on failure.
fn fmt(src: &str) -> String {
    match format_source(src) {
        Ok(out) => out,
        Err(e) => panic!("fmt failed at byte {}: {}", e.span.start, e.message),
    }
}

/// Report whether formatting a source string fails.
fn fmt_fails(src: &str) -> bool {
    format_source(src).is_err()
}

/// Extract the (kind, text) sequence of real tokens from a source string.
fn tok_seq(src: &str) -> Vec<(TokenKind, String)> {
    let (toks, diags) = axolc_core::tokenize(src, 0);
    assert!(!diags.has_errors(), "tokenize failed on: {src}");
    toks
        .into_iter()
        .filter(|t| t.kind != TokenKind::Eof)
        .map(|t| (t.kind, t.text))
        .collect()
}

/// Extract the (classifier, text) sequence of tokens plus comments from a source string.
fn piece_seq(src: &str) -> Vec<(String, String)> {
    let pieces = scan_pieces(src, 0).expect("scan pieces");
    pieces
        .into_iter()
        .map(|p| match p {
            Piece::Tok(t) => (format!("tok:{:?}", t.kind), t.text),
            Piece::LineComment { text, .. } => ("line".to_string(), text),
            Piece::BlockComment { text, .. } => ("block".to_string(), text),
        })
        .collect()
}

/// Table of (messy input, canonical output) pairs covering the whole grammar.
fn canon_table() -> Vec<(&'static str, &'static str)> {
    vec![
        ("fn main()\nprint(1)\nend", "fn main()\n    print(1)\nend\n"),
        ("fn   main ( )\n    print( 1 )\nend", "fn main()\n    print(1)\nend\n"),
        ("fn main()\n\tprint(1)\n\tend", "fn main()\n    print(1)\nend\n"),
        ("fn main()\r\n    print(1)\r\nend\r\n", "fn main()\n    print(1)\nend\n"),
        ("  fn main()\n     print(1)\n end  ", "fn main()\n    print(1)\nend\n"),
        ("fn main()\nprint(1)\nprint(2)\nend", "fn main()\n    print(1)\n    print(2)\nend\n"),
        (
            "fn main()\n\n\n    print(1)\n\n\n    print(2)\n\nend\n\n\n",
            "fn main()\n    print(1)\n    print(2)\nend\n",
        ),
        (
            "fn f(  x:Int,y:Int  )->Int\n    return x+y\nend",
            "fn f(x: Int, y: Int) -> Int\n    return x + y\nend\n",
        ),
        (
            "fn f(x: Int, y: Int = 3, ...rest) -> Int\n    return x\nend",
            "fn f(x: Int, y: Int = 3, ...rest) -> Int\n    return x\nend\n",
        ),
        (
            "pub fn f()\n    print(1)\nend",
            "pub fn f()\n    print(1)\nend\n",
        ),
        (
            "async fn f()\n    print(1)\nend",
            "async fn f()\n    print(1)\nend\n",
        ),
        (
            "pub async fn f()\n    print(1)\nend",
            "pub async fn f()\n    print(1)\nend\n",
        ),
        (
            "fn f()\nend",
            "fn f()\nend\n",
        ),
        (
            "fn main()\n    let x=1\n    var y   =  2\n    print(x+y)\nend",
            "fn main()\n    let x = 1\n    var y = 2\n    print(x + y)\nend\n",
        ),
        (
            "fn main()\n    let x : Int = 1\n    let ys : [Int] = [ 1,2,3 ]\n    print(x)\nend",
            "fn main()\n    let x: Int = 1\n    let ys: [Int] = [1, 2, 3]\n    print(x)\nend\n",
        ),
        (
            "fn main()\n    let t = ( 1,2 )\n    let m = { \"a\"=1, \"b\"=2 }\n    print(t)\nend",
            "fn main()\n    let t = (1, 2)\n    let m = { \"a\" = 1, \"b\" = 2 }\n    print(t)\nend\n",
        ),
        (
            "fn main()\n    let a=1+2*3-4/5%6\n    print(a)\nend",
            "fn main()\n    let a = 1 + 2 * 3 - 4 / 5 % 6\n    print(a)\nend\n",
        ),
        (
            "fn main()\n    let b=1<2\n    let c=1<=2\n    let d=1>2\n    let e=1>=2\n    let f=1==2\n    let g=1!=2\n    print(b)\nend",
            "fn main()\n    let b = 1 < 2\n    let c = 1 <= 2\n    let d = 1 > 2\n    let e = 1 >= 2\n    let f = 1 == 2\n    let g = 1 != 2\n    print(b)\nend\n",
        ),
        (
            "fn main()\n    let b=true and false or true\n    let c=not b\n    print(c)\nend",
            "fn main()\n    let b = true and false or true\n    let c = not b\n    print(c)\nend\n",
        ),
        (
            "fn main()\n    let a = 1&2|3^4\n    let b = a<<1\n    let c = a>>1\n    print(b)\nend",
            "fn main()\n    let a = 1 & 2 | 3 ^ 4\n    let b = a << 1\n    let c = a >> 1\n    print(b)\nend\n",
        ),
        (
            "fn main()\n    let n = -5\n    let z = - -5\n    let w = !true\n    let q = ~1\n    print(n)\nend",
            "fn main()\n    let n = -5\n    let z = - -5\n    let w = !true\n    let q = ~1\n    print(n)\nend\n",
        ),
        (
            "fn main()\n    let r=1..5\n    let s=1..=5\n    let t=..5\n    print(r)\nend",
            "fn main()\n    let r = 1..5\n    let s = 1..=5\n    let t = ..5\n    print(r)\nend\n",
        ),
        (
            "fn main()\n    x=x+1\n    y-=2\n    z*=3\n    w/=4\n    v%=5\n    print(x)\nend",
            "fn main()\n    x = x + 1\n    y -= 2\n    z *= 3\n    w /= 4\n    v %= 5\n    print(x)\nend\n",
        ),
        (
            "fn main()\n    if true then\n        print(1)\n    end\nend",
            "fn main()\n    if true then\n        print(1)\n    end\nend\n",
        ),
        (
            "fn main()\n    if true then print(1) elseif false then print(2) else print(3) end\nend",
            "fn main()\n    if true then\n        print(1)\n    elseif false then\n        print(2)\n    else\n        print(3)\n    end\nend\n",
        ),
        (
            "fn main()\n    while true do\n        break\n    end\nend",
            "fn main()\n    while true do\n        break\n    end\nend\n",
        ),
        (
            "fn main()\n    for i in 1..10 do\n        print(i)\n    end\nend",
            "fn main()\n    for i in 1..10 do\n        print(i)\n    end\nend\n",
        ),
        (
            "fn main()\n    repeat\n        print(1)\n    until false\nend",
            "fn main()\n    repeat\n        print(1)\n    until false\nend\n",
        ),
        (
            "fn main()\n    loop\n        break\n    end\nend",
            "fn main()\n    loop\n        break\n    end\nend\n",
        ),
        (
            "fn main()\n    match x\n        1 => \"one\"\n        2 => \"two\"\n        _ => \"many\"\n    end\nend",
            "fn main()\n    match x\n        1 => \"one\"\n        2 => \"two\"\n        _ => \"many\"\n    end\nend\n",
        ),
        (
            "fn main()\n    match x\n        1|2 => \"low\"\n        n if n > 9 => \"big\"\n        _ => \"mid\"\n    end\nend",
            "fn main()\n    match x\n        1 | 2 => \"low\"\n        n if n > 9 => \"big\"\n        _ => \"mid\"\n    end\nend\n",
        ),
        (
            "fn main()\n    match p\n        Point { x = 1 } => \"origin\"\n        Some(y) => y\n        _ => 0\n    end\nend",
            "fn main()\n    match p\n        Point { x = 1 } => \"origin\"\n        Some(y) => y\n        _ => 0\n    end\nend\n",
        ),
        (
            "fn main()\n    match v\n        1 => do\n            print(1)\n        end\n        _ => do\n            print(0)\n        end\n    end\nend",
            "fn main()\n    match v\n        1 => do\n            print(1)\n        end\n        _ => do\n            print(0)\n        end\n    end\nend\n",
        ),
        (
            "fn make() -> fn() -> Int\n    var count = 0\n    return fn() -> Int\n        count = count + 1\n        return count\n    end\nend",
            "fn make() -> fn() -> Int\n    var count = 0\n    return fn() -> Int\n        count = count + 1\n        return count\n    end\nend\n",
        ),
        (
            "fn main()\n    let f = fn(a: Int) -> Int\n        return a * 2\n    end\n    print(f(1))\nend",
            "fn main()\n    let f = fn(a: Int) -> Int\n        return a * 2\n    end\n    print(f(1))\nend\n",
        ),
        (
            "P = struct\n    x : Int\n    y : Int\nend",
            "P = struct\n    x: Int\n    y: Int\nend\n",
        ),
        (
            "P=struct\n    x:Int,\n    y:Int,\nend",
            "P = struct\n    x: Int,\n    y: Int,\nend\n",
        ),
        (
            "pub P = struct\n    x: Int = 0\nend",
            "pub P = struct\n    x: Int = 0\nend\n",
        ),
        (
            "Boxed = struct\n    inner: Boxed?\nend",
            "Boxed = struct\n    inner: Boxed?\nend\n",
        ),
        (
            "Pair = struct\n    a: Int\n    b: (Int, String)\nend",
            "Pair = struct\n    a: Int\n    b: (Int, String)\nend\n",
        ),
        (
            "Holder = struct\n    cb: fn(Int) -> Int\nend",
            "Holder = struct\n    cb: fn(Int) -> Int\nend\n",
        ),
        (
            "C = enum\n    Red\n    Green\n    Blue\nend",
            "C = enum\n    Red\n    Green\n    Blue\nend\n",
        ),
        (
            "C = enum\n    Red,\n    Green,\nend",
            "C = enum\n    Red,\n    Green,\nend\n",
        ),
        (
            "Shape = enum\n    Circle(radius: Float)\n    Rect(w: Float, h: Float)\nend",
            "Shape = enum\n    Circle(radius: Float)\n    Rect(w: Float, h: Float)\nend\n",
        ),
        (
            "Shape = interface\n    area(self) -> Float\n    name(self) -> String\nend",
            "Shape = interface\n    area(self) -> Float\n    name(self) -> String\nend\n",
        ),
        (
            "P.damage = fn(self, amount: Int)\n    self.health = self.health - amount\nend",
            "P.damage = fn(self, amount: Int)\n    self.health = self.health - amount\nend\n",
        ),
        (
            "P.create = fn(x: Int)\n    return x\nend",
            "P.create = fn(x: Int)\n    return x\nend\n",
        ),
        (
            "fn main()\n    let p = P { x = 1, y = 2 }\n    print(p.x)\nend",
            "fn main()\n    let p = P { x = 1, y = 2 }\n    print(p.x)\nend\n",
        ),
        (
            "fn main()\n    let p = P { x=1, y=2, }\n    print(p.y)\nend",
            "fn main()\n    let p = P { x = 1, y = 2, }\n    print(p.y)\nend\n",
        ),
        (
            "fn main()\n    let e = P { }\n    print(e)\nend",
            "fn main()\n    let e = P {}\n    print(e)\nend\n",
        ),
        (
            "fn main()\n    let m = {}\n    print(m)\nend",
            "fn main()\n    let m = {}\n    print(m)\nend\n",
        ),
        (
            "fn main()\n    p:damage(30)\n    p:alive()\n    a[0] = 5\n    print(a[1])\nend",
            "fn main()\n    p:damage(30)\n    p:alive()\n    a[0] = 5\n    print(a[1])\nend\n",
        ),
        (
            "fn main()\n    let s = \"a${x}b\"\n    let t = \"${y}z\"\n    let u = \"plain\"\n    print(s)\nend",
            "fn main()\n    let s = \"a${x}b\"\n    let t = \"${y}z\"\n    let u = \"plain\"\n    print(s)\nend\n",
        ),
        (
            "fn main()\n    let s = \"a${ f(1) }b${g(2, 3)}c\"\n    print(s)\nend",
            "fn main()\n    let s = \"a${f(1)}b${g(2, 3)}c\"\n    print(s)\nend\n",
        ),
        (
            "fn main()\n    spawn print(1)\nend",
            "fn main()\n    spawn print(1)\nend\n",
        ),
        (
            "fn main()\n    unsafe\n        print(1)\n    end\nend",
            "fn main()\n    unsafe\n        print(1)\n    end\nend\n",
        ),
        (
            "use std::io\nuse std::fmt",
            "use std::io\n\nuse std::fmt\n",
        ),
        (
            "const MAX: Int = 42\nconst MIN = 0",
            "const MAX: Int = 42\n\nconst MIN = 0\n",
        ),
        (
            "type Pair = (Int, Int)\ntype List = [Int]",
            "type Pair = (Int, Int)\n\ntype List = [Int]\n",
        ),
        (
            "extern \"C\"\nextern",
            "extern \"C\"\n\nextern\n",
        ),
        (
            "fn main()\n    print(1);\nend",
            "fn main()\n    print(1);\nend\n",
        ),
        (
            "fn main()\n    return 1\nend\n\nfn g()\n    return 2\nend",
            "fn main()\n    return 1\nend\n\nfn g()\n    return 2\nend\n",
        ),
        (
            "fn main()\n    return\nend",
            "fn main()\n    return\nend\n",
        ),
        (
            "fn main()\n    if a then\n        if b then\n            if c then\n                print(1)\n            end\n        end\n    end\nend",
            "fn main()\n    if a then\n        if b then\n            if c then\n                print(1)\n            end\n        end\n    end\nend\n",
        ),
        (
            "fn main()\n    while a do\n        while b do\n            break\n        end\n    end\nend",
            "fn main()\n    while a do\n        while b do\n            break\n        end\n    end\nend\n",
        ),
        (
            "fn main()\n    let o = p?.name\n    let q = x as Int\n    let r = y is Int\n    print(o)\nend",
            "fn main()\n    let o = p?.name\n    let q = x as Int\n    let r = y is Int\n    print(o)\nend\n",
        ),
        (
            "fn main()\n    let v = a |> f\n    let w = g <| b\n    print(v)\nend",
            "fn main()\n    let v = a |> f\n    let w = g <| b\n    print(v)\nend\n",
        ),
        (
            "fn main()\n    print(\"hello, ${name}!\")\nend",
            "fn main()\n    print(\"hello, ${name}!\")\nend\n",
        ),
        (
            "fn main()\n    let x = 1\n    x = 2\nend",
            "fn main()\n    let x = 1\n    x = 2\nend\n",
        ),
        (
            "fn main()\n    let b = self.health > 0\n    print(b)\nend",
            "fn main()\n    let b = self.health > 0\n    print(b)\nend\n",
        ),
        (
            "fn main()\n    print(f(x = 1, y = 2))\nend",
            "fn main()\n    print(f(x = 1, y = 2))\nend\n",
        ),
        (
            "fn main()\n    print(f(x = 1, ...rest))\nend",
            "fn main()\n    print(f(x = 1, ...rest))\nend\n",
        ),
        (
            "fn main()\n    let c = 1.5\n    let d = 2.25e10\n    let e = 'a'\n    print(c)\nend",
            "fn main()\n    let c = 1.5\n    let d = 2.25e10\n    let e = 'a'\n    print(c)\nend\n",
        ),
        (
            "fn main()\n    let s = \"tab\\there\"\n    print(s)\nend",
            "fn main()\n    let s = \"tab\\there\"\n    print(s)\nend\n",
        ),
        (
            "fn main()\n    let a = 1\n    let b = a\n    print(\"${a}${b}\")\nend",
            "fn main()\n    let a = 1\n    let b = a\n    print(\"${a}${b}\")\nend\n",
        ),
        (
            "fn main()\n    if x then\n        return 1\n    end\n    return 0\nend",
            "fn main()\n    if x then\n        return 1\n    end\n    return 0\nend\n",
        ),
        (
            "fn main()\n    let t = Self.value\n    print(t)\nend",
            "fn main()\n    let t = Self.value\n    print(t)\nend\n",
        ),
        (
            "fn main()\n    let m: Map<Int, String> = {}\n    print(m)\nend",
            "fn main()\n    let m: Map<Int, String> = {}\n    print(m)\nend\n",
        ),
        (
            "fn main()\n    let r: &Int = ref\n    let s: & var Int = mref\n    print(r)\nend",
            "fn main()\n    let r: &Int = ref\n    let s: & var Int = mref\n    print(r)\nend\n",
        ),
        (
            "fn main()\n    let a = borrow x\n    let b = move y\n    print(a)\nend",
            "fn main()\n    let a = borrow x\n    let b = move y\n    print(a)\nend\n",
        ),
        (
            "fn main()\n    let n = nil\n    print(n)\nend",
            "fn main()\n    let n = nil\n    print(n)\nend\n",
        ),
        (
            "fn main()\n    let p = std::path::join(a, b)\n    print(p)\nend",
            "fn main()\n    let p = std::path::join(a, b)\n    print(p)\nend\n",
        ),
        (
            "fn main()\n    print(1)\nend\n-- trailing file comment",
            "fn main()\n    print(1)\nend\n-- trailing file comment\n",
        ),
        (
            "-- leading\nfn main()\n    print(1)\nend",
            "-- leading\nfn main()\n    print(1)\nend\n",
        ),
        (
            "fn main()\n    let x = 1  -- trailing comment\n    -- leading comment\n    print(x)\nend",
            "fn main()\n    let x = 1  -- trailing comment\n    -- leading comment\n    print(x)\nend\n",
        ),
        (
            "--[[ block comment ]]\nfn main()\n    print(1)\nend",
            "--[[ block comment ]]\nfn main()\n    print(1)\nend\n",
        ),
        (
            "fn main()\n    let x = 1  --[[ inline block ]]\n    print(x)\nend",
            "fn main()\n    let x = 1  --[[ inline block ]]\n    print(x)\nend\n",
        ),
        (
            "-- one\n-- two\nfn main()\n    print(1)\nend",
            "-- one\n-- two\nfn main()\n    print(1)\nend\n",
        ),
        (
            "fn main()\n    -- comment inside body\n    print(1)\n    -- another\n    print(2)\nend",
            "fn main()\n    -- comment inside body\n    print(1)\n    -- another\n    print(2)\nend\n",
        ),
        (
            "fn main()\n    let x = 1  -- keeps\n    -- belongs to print\n    print(x)\nend\n\n-- before g\nfn g()\nend",
            "fn main()\n    let x = 1  -- keeps\n    -- belongs to print\n    print(x)\nend\n\n-- before g\nfn g()\nend\n",
        ),
        (
            "P = struct\n    -- field comment\n    x: Int\nend",
            "P = struct\n    -- field comment\n    x: Int\nend\n",
        ),
        (
            "fn main()\n    while a do\n        -- loop comment\n        break\n    end\nend",
            "fn main()\n    while a do\n        -- loop comment\n        break\n    end\nend\n",
        ),
        (
            "fn main()\n    match x\n        -- arm comment\n        1 => 2\n    end\nend",
            "fn main()\n    match x\n        -- arm comment\n        1 => 2\n    end\nend\n",
        ),
        (
            "fn main()\n    if a then\n        -- then comment\n        print(1)\n    else\n        -- else comment\n        print(2)\n    end\nend",
            "fn main()\n    if a then\n        -- then comment\n        print(1)\n    else\n        -- else comment\n        print(2)\n    end\nend\n",
        ),
        (
            "fn main()\n    repeat\n        -- body comment\n        print(1)\n    until x\nend",
            "fn main()\n    repeat\n        -- body comment\n        print(1)\n    until x\nend\n",
        ),
        (
            "@inline\nfn main()\n    print(1)\nend",
            "@inline\nfn main()\n    print(1)\nend\n",
        ),
        (
            "fn main()\n    let s = `raw string`\n    print(s)\nend",
            "fn main()\n    let s = `raw string`\n    print(s)\nend\n",
        ),
        (
            "fn main()\n    let p = P { x = 1; y = 2 }\n    print(p)\nend",
            "fn main()\n    let p = P { x = 1; y = 2 }\n    print(p)\nend\n",
        ),
    ]
}

/// Every table case formats its messy input to exactly the canonical output.
#[test]
fn canonical_table_formats_exact() {
    for (input, expected) in canon_table() {
        assert_eq!(fmt(input), expected, "input: {input:?}");
    }
}

/// Formatting the canonical output again produces the identical string.
#[test]
fn canonical_table_is_idempotent() {
    for (input, expected) in canon_table() {
        assert_eq!(fmt(expected), expected, "already canonical: {expected:?}");
        assert_eq!(fmt(&fmt(input)), fmt(input), "double format of: {input:?}");
    }
}

/// The token sequence of the formatted output matches the input token sequence.
#[test]
fn canonical_table_preserves_tokens() {
    for (input, expected) in canon_table() {
        assert_eq!(tok_seq(expected), tok_seq(input), "tokens differ for: {input:?}");
    }
}

/// The token-plus-comment sequence survives formatting unchanged.
#[test]
fn canonical_table_preserves_pieces_with_comments() {
    for (input, expected) in canon_table() {
        assert_eq!(piece_seq(expected), piece_seq(input), "pieces differ for: {input:?}");
    }
}

/// No formatted line ends with whitespace and the file ends with one newline.
#[test]
fn canonical_table_has_no_trailing_whitespace() {
    for (input, _) in canon_table() {
        let out = fmt(input);
        for (i, line) in out.lines().enumerate() {
            assert_eq!(line, line.trim_end(), "trailing whitespace on line {} of {input:?}", i + 1);
        }
        assert!(out.is_empty() || out.ends_with('\n'), "missing final newline for {input:?}");
        assert!(
            !out.ends_with("\n\n"),
            "more than one final newline for {input:?}"
        );
        assert_eq!(out.matches('\n').count(), out.lines().count(), "newline count mismatch for {input:?}");
    }
}

/// The empty source string formats to the empty string.
#[test]
fn empty_source_formats_to_empty() {
    assert_eq!(fmt(""), "");
    assert_eq!(fmt("\n\n\n"), "");
    assert_eq!(fmt("   \t \n"), "");
}

/// A comment-only source keeps its comments.
#[test]
fn comment_only_source_keeps_comments() {
    assert_eq!(fmt("-- just a comment\n"), "-- just a comment\n");
    assert_eq!(fmt("--[[ block ]]\n"), "--[[ block ]]\n");
    assert_eq!(fmt("-- a\n\n\n-- b\n"), "-- a\n\n-- b\n");
    assert_eq!(piece_seq(&fmt("-- a\n\n\n-- b\n")).len(), 2);
}

/// Unterminated strings and unknown characters are hard formatting failures.
#[test]
fn lexer_errors_fail_formatting() {
    assert!(fmt_fails("fn main()\n    let s = \"unterminated\nend"));
    assert!(fmt_fails("fn main()\n    let q = \"${x\nend"));
    assert!(fmt_fails("fn main()\n    print(1)\nend\n\u{1}"));
    assert!(!fmt_fails("fn main()\n    print(\"ok\")\nend"));
}

/// Bodies are indented one level per construct depth.
#[test]
fn indentation_is_four_spaces_per_level() {
    let src = "fn a()\n    if x then\n        while y do\n            for i in 1..2 do\n                if z then\n                    print(1)\n                end\n            end\n        end\n    end\nend";
    let out = fmt(src);
    let lines: Vec<&str> = out.lines().collect();
    assert_eq!(lines[0], "fn a()");
    assert_eq!(lines[1], "    if x then");
    assert_eq!(lines[2], "        while y do");
    assert_eq!(lines[3], "            for i in 1..2 do");
    assert_eq!(lines[4], "                if z then");
    assert_eq!(lines[5], "                    print(1)");
    assert_eq!(lines[6], "                end");
    assert_eq!(lines[7], "            end");
    assert_eq!(lines[8], "        end");
    assert_eq!(lines[9], "    end");
    assert_eq!(lines[10], "end");
    for (i, line) in lines.iter().enumerate() {
        let indent = line.len() - line.trim_start().len();
        assert_eq!(indent % 4, 0, "line {} not a multiple of 4 spaces: {line:?}", i + 1);
    }
}

/// Top-level items sit at column zero separated by exactly one blank line.
#[test]
fn top_level_items_are_column_zero_with_blank_lines() {
    let src = "fn a()\n    print(1)\nend\nfn b()\n    print(2)\nend\nfn c()\n    print(3)\nend";
    let out = fmt(src);
    let lines: Vec<&str> = out.lines().collect();
    assert_eq!(lines.len(), 11);
    assert!(lines[0].starts_with("fn a()"));
    assert_eq!(lines[3], "");
    assert!(lines[4].starts_with("fn b()"));
    assert_eq!(lines[7], "");
    assert!(lines[8].starts_with("fn c()"));
    for line in lines.iter().filter(|l| l.starts_with("fn") || *(*l) == "end") {
        assert!(!line.starts_with(' '), "top-level line indented: {line:?}");
    }
}

/// Extra blank lines collapse to a single blank line between items.
#[test]
fn blank_line_runs_collapse() {
    let src = "fn a()\nend\n\n\n\n\nfn b()\nend";
    let out = fmt(src);
    let lines: Vec<&str> = out.lines().collect();
    assert_eq!(lines, vec!["fn a()", "end", "", "fn b()", "end"]);
}

/// Statement lines inside one body are never separated by blank lines.
#[test]
fn body_statements_have_no_blank_lines_between() {
    let src = "fn main()\n    print(1)\n\n\n    print(2)\nend";
    let out = fmt(src);
    let lines: Vec<&str> = out.lines().collect();
    assert_eq!(lines, vec!["fn main()", "    print(1)", "    print(2)", "end"]);
}

/// Struct fields render one per line with the trailing comma preserved from input.
#[test]
fn struct_fields_one_per_line() {
    let src = "P = struct\n    x: Int, y: Int,\n    z: Int\nend";
    let out = fmt(src);
    let lines: Vec<&str> = out.lines().collect();
    assert_eq!(lines[0], "P = struct");
    assert_eq!(lines[1], "    x: Int,");
    assert_eq!(lines[2], "    y: Int,");
    assert_eq!(lines[3], "    z: Int");
    assert_eq!(lines[4], "end");
}

/// Enum variants render one per line.
#[test]
fn enum_variants_one_per_line() {
    let src = "E = enum\n    A, B,\n    C\nend";
    let out = fmt(src);
    let lines: Vec<&str> = out.lines().collect();
    assert_eq!(lines[0], "E = enum");
    assert_eq!(lines[1], "    A,");
    assert_eq!(lines[2], "    B,");
    assert_eq!(lines[3], "    C");
    assert_eq!(lines[4], "end");
}

/// A well-known program round-trips to itself byte-for-byte.
#[test]
fn already_canonical_sources_are_fixed_points() {
    let fixed: Vec<&str> = vec![
        "fn main()\n    print(\"hi\")\nend\n",
        "fn fib(n: Int) -> Int\n    if n < 2 then\n        return n\n    end\n    return fib(n - 1) + fib(n - 2)\nend\n",
        "fn main()\n    let name = \"Axolotl\"\n    print(\"hello, ${name}!\")\nend\n",
        "fn main()\n    var i = 0\n    while i < 10 do\n        print(i)\n        i = i + 1\n    end\nend\n",
        "P = struct\n    health: Int\n    name: String\nend\n\nP.damage = fn(self, amount: Int)\n    self.health = self.health - amount\nend\n",
        "fn main()\n    for i in 1..=20 do\n        print(i)\n    end\nend\n",
        "fn main()\n    match x\n        1 => 2\n        _ => 3\n    end\nend\n",
        "use std::io\n\nfn main()\n    print(1)\nend\n",
    ];
    for src in fixed {
        assert_eq!(fmt(src), src, "not a fixed point: {src:?}");
    }
}

/// Interpolated strings keep their chunks and inner expressions verbatim.
#[test]
fn interpolation_reconstruction_is_exact() {
    let cases: Vec<(&str, &str)> = vec![
        ("\"a\"", "\"a\""),
        ("\"a${x}b\"", "\"a${x}b\""),
        ("\"${x}\"", "\"${x}\""),
        ("\"${ x + 1 }\"", "\"${x + 1}\""),
        ("\"a${f(1)}b${g(2)}c\"", "\"a${f(1)}b${g(2)}c\""),
        ("\"${\"nested\"}\"", "\"${\"nested\"}\""),
    ];
    for (src, expected) in cases {
        let wrapped = format!("fn main()\n    let s = {src}\n    print(s)\nend");
        let out = fmt(&wrapped);
        assert!(out.contains(&format!("let s = {expected}\n")), "in {out:?}");
        assert_eq!(tok_seq(&out), tok_seq(&wrapped));
    }
}

/// Operator spacing follows a + b style for every binary operator.
#[test]
fn binary_operators_are_spaced() {
    let ops: Vec<(&str, &str)> = vec![
        ("1+2", "1 + 2"),
        ("1-2", "1 - 2"),
        ("1*2", "1 * 2"),
        ("1/2", "1 / 2"),
        ("1%2", "1 % 2"),
        ("1==2", "1 == 2"),
        ("1!=2", "1 != 2"),
        ("1<2", "1 < 2"),
        ("1<=2", "1 <= 2"),
        ("1>2", "1 > 2"),
        ("1>=2", "1 >= 2"),
        ("1..2", "1..2"),
        ("1..=2", "1..=2"),
        ("a and b", "a and b"),
        ("a or b", "a or b"),
        ("a&b", "a & b"),
        ("a|b", "a | b"),
        ("a^b", "a ^ b"),
        ("a<<b", "a << b"),
        ("a>>b", "a >> b"),
    ];
    for (src, expected) in ops {
        let wrapped = format!("fn main()\n    let r = {src}\n    print(r)\nend");
        let out = fmt(&wrapped);
        assert!(out.contains(&format!("let r = {expected}\n")), "for {src:?} in {out:?}");
    }
}

/// Call and index brackets stay tight against their operands.
#[test]
fn brackets_stay_tight() {
    let cases: Vec<(&str, &str)> = vec![
        ("print( 1 , 2 )", "print(1, 2)"),
        ("f(  )", "f()"),
        ("a[ 0 ]", "a[0]"),
        ("[ 1 , 2 ]", "[1, 2]"),
        ("( 1 , 2 )", "(1, 2)"),
        ("f( )", "f()"),
    ];
    for (src, expected) in cases {
        let wrapped = format!("fn main()\n    let r = {src}\n    print(1)\nend");
        let out = fmt(&wrapped);
        assert!(out.contains(expected), "for {src:?} in {out:?}");
    }
}

/// line_col_of maps byte offsets to one-based line and char columns.
#[test]
fn line_col_of_computes_positions() {
    let src = "fn main()\n    let x = 1\nend\n";
    assert_eq!(line_col_of(src, 0), (1, 1));
    assert_eq!(line_col_of(src, 1), (1, 2));
    assert_eq!(line_col_of(src, 9), (1, 10));
    assert_eq!(line_col_of(src, 10), (2, 1));
    assert_eq!(line_col_of(src, 14), (2, 5));
    assert_eq!(line_col_of(src, 24), (3, 1));
    assert_eq!(line_col_of(src, 27), (3, 4));
    assert_eq!(line_col_of(src, 1000), (4, 1));
}

/// line_col_of counts columns in characters, not bytes.
#[test]
fn line_col_of_handles_multibyte() {
    let src = "fn main()\n    print(\"héllo\")\nend\n";
    let offset = src.find("é").unwrap();
    assert_eq!(line_col_of(src, offset), (2, 13));
    assert_eq!(line_col_of(src, offset + 1), (2, 13));
    assert_eq!(line_col_of(src, offset + 2), (2, 14));
}

/// The scan_pieces stream positions are non-decreasing.
#[test]
fn scan_pieces_positions_are_ordered() {
    let src = "-- top\nfn main()\n    -- inner\n    print(1) -- trail\nend\n--[[ tail ]]\n";
    let pieces = scan_pieces(src, 0).unwrap();
    let mut last = 0;
    for p in &pieces {
        assert!(p.start() >= last, "piece order violated at {}", p.start());
        last = p.start();
    }
    assert!(pieces.len() >= 10);
    let comments = pieces.iter().filter(|p| p.kind().is_none()).count();
    assert_eq!(comments, 4);
}

/// Examples: every repository example is an idempotent format target.
#[test]
fn examples_are_idempotent_under_fmt() {
    let files = example_files();
    assert_eq!(files.len(), 6);
    for (name, src) in &files {
        let once = fmt(src);
        let twice = fmt(&once);
        assert_eq!(once, twice, "fmt not idempotent on {name}");
    }
}

/// Examples: formatting preserves the token stream of every example.
#[test]
fn examples_preserve_tokens() {
    for (name, src) in example_files() {
        let out = fmt(&src);
        assert_eq!(tok_seq(&out), tok_seq(&src), "token drift on {name}");
    }
}

/// Examples: formatting preserves tokens and comments together.
#[test]
fn examples_preserve_pieces() {
    for (name, src) in example_files() {
        let out = fmt(&src);
        assert_eq!(piece_seq(&out), piece_seq(&src), "piece drift on {name}");
    }
}

/// Examples: interpreting the formatted source matches the original output.
#[test]
fn examples_preserve_interpretation() {
    for (name, src) in example_files() {
        let out = fmt(&src);
        let (before, d1) = axolc_core::interpret(&src, 0);
        let (after, d2) = axolc_core::interpret(&out, 0);
        assert!(!d1.has_errors(), "interpreter errors on original {name}");
        assert!(!d2.has_errors(), "interpreter errors on formatted {name}");
        assert_eq!(before, after, "interpretation changed for {name}");
    }
}

/// Examples: the formatted shape keeps headers, item order, and indentation.
#[test]
fn examples_formatted_shape_is_canonical() {
    let fib = include_str!("../../../examples/fib.axol");
    let out = fmt(fib);
    let lines: Vec<&str> = out.lines().collect();
    assert_eq!(lines[0], "-- Owner: PascalElixir / axolrs (GitHub org)");
    assert_eq!(lines[2], "");
    assert!(lines[3].starts_with("fn fib(n: Int) -> Int"));
    assert_eq!(lines[4], "    if n < 2 then");
    assert_eq!(lines[5], "        return n");
    assert_eq!(lines[6], "    end");
    assert_eq!(lines[7], "    return fib(n - 1) + fib(n - 2)");
    assert_eq!(lines[8], "end");
    assert_eq!(lines[9], "");
    assert!(lines[10].starts_with("fn main()"));
    assert_eq!(lines[14], "        i = i + 1");
    assert_eq!(lines[15], "    end");
    assert_eq!(lines[16], "end");

    let structs = include_str!("../../../examples/structs.axol");
    let out2 = fmt(structs);
    let lines2: Vec<&str> = out2.lines().collect();
    assert_eq!(lines2[2], "");
    assert!(lines2[3].starts_with("Player = struct"));
    assert_eq!(lines2[4], "    health: Int");
    assert_eq!(lines2[5], "    name: String");
    assert_eq!(lines2[6], "end");
    assert_eq!(lines2[7], "");
    assert!(lines2[8].starts_with("Player.damage = fn(self, amount: Int)"));
    assert_eq!(lines2[9], "    self.health = self.health - amount");
    assert_eq!(lines2[10], "end");
    assert_eq!(lines2[14], "end");
    assert_eq!(lines2[18], "    p:damage(30)");
    assert_eq!(lines2[22], "end");

    let hello = include_str!("../../../examples/hello.axol");
    let out3 = fmt(hello);
    let lines3: Vec<&str> = out3.lines().collect();
    assert_eq!(lines3[2], "");
    assert_eq!(lines3[3], "fn main()");
    assert_eq!(lines3[4], "    let name = \"Axolotl\"");
    assert_eq!(lines3[5], "    print(\"hello, ${name}!\")");
    assert_eq!(lines3[6], "end");
}

/// Examples: the header comments survive formatting untouched.
#[test]
fn examples_keep_owner_headers() {
    for (name, src) in example_files() {
        let out = fmt(&src);
        let original_first = src.lines().next().unwrap_or("");
        let formatted_first = out.lines().next().unwrap_or("");
        assert_eq!(original_first, formatted_first, "header changed on {name}");
        let original_second = src.lines().nth(1).unwrap_or("");
        let formatted_second = out.lines().nth(1).unwrap_or("");
        assert_eq!(original_second, formatted_second, "second header line changed on {name}");
    }
}

/// Interpretation is preserved for a corpus of runnable programs.
#[test]
fn interpretation_is_preserved_on_corpus() {
    let corpus: Vec<&str> = vec![
        "fn main()\n    print(1)\nend",
        "fn main()\n    let x = 2\n    print(x * 3)\nend",
        "fn main()\n    print(\"a\")\n    print(\"b\")\nend",
        "fn f(n: Int) -> Int\n    if n < 2 then\n        return n\n    end\n    return f(n - 1) + f(n - 2)\nend\n\nfn main()\n    print(f(6))\nend",
        "fn main()\n    for i in 1..5 do\n        print(i)\n    end\nend",
        "fn main()\n    var i = 0\n    while i < 3 do\n        print(i)\n        i = i + 1\n    end\nend",
        "fn main()\n    match 2\n        1 => print(\"one\")\n        2 => print(\"two\")\n        _ => print(\"many\")\n    end\nend",
        "fn main()\n    let a = [1, 2, 3]\n    print(a[1])\nend",
        "fn main()\n    let name = \"x\"\n    print(\"v=${name}\")\nend",
        "fn main()\n    let t = (1, \"two\")\n    print(t)\nend",
        "fn main()\n    var s = 0\n    repeat\n        s = s + 2\n    until s > 4\n    print(s)\nend",
        "fn main()\n    print(1 + 2 * 3 - 4 / 2)\nend",
        "fn main()\n    print(1 < 2)\n    print(2 == 2)\n    print(not false)\nend",
        "P = struct\n    x: Int\nend\n\nfn main()\n    let p = P { x = 9 }\n    print(p.x)\nend",
        "fn main()\n    let f = fn(a: Int) -> Int\n        return a + 1\n    end\n    print(f(4))\nend",
        "fn make() -> fn() -> Int\n    var count = 0\n    return fn() -> Int\n        count = count + 1\n        return count\n    end\nend\n\nfn main()\n    let next = make()\n    print(next())\n    print(next())\nend",
        "fn main()\n    print(\"${1 + 2}\")\nend",
        "fn main()\n    let m = { \"k\" = 7 }\n    print(m)\nend",
    ];
    for src in corpus {
        let out = fmt(src);
        let (before, d1) = axolc_core::interpret(src, 0);
        let (after, d2) = axolc_core::interpret(&out, 0);
        assert!(!d1.has_errors(), "interpreter errors on corpus {src:?}");
        assert!(!d2.has_errors(), "interpreter errors on formatted corpus {src:?}");
        assert_eq!(before, after, "semantics drifted for {src:?}");
    }
}

/// Gather the repository examples with their file names.
fn example_files() -> Vec<(String, String)> {
    let names = ["hello", "fib", "fizzbuzz", "primes", "counter", "structs"];
    names
        .iter()
        .map(|n| {
            let path = format!("{}/../../examples/{}.axol", env!("CARGO_MANIFEST_DIR"), n);
            (n.to_string(), std::fs::read_to_string(path).expect("read example"))
        })
        .collect()
}

/// A blank line between a comment block and the following item is preserved.
#[test]
fn blank_line_between_comment_and_item_is_preserved() {
    let src = "-- Owner: PascalElixir / axolrs (GitHub org)\n-- File: header comment\n\nfn main()\n    print(1)\nend\n";
    let out = fmt(src);
    assert_eq!(
        out,
        "-- Owner: PascalElixir / axolrs (GitHub org)\n-- File: header comment\n\nfn main()\n    print(1)\nend\n"
    );
    assert_eq!(fmt(&out), out, "preservation is idempotent");
}

/// Multiple blank lines between a comment block and an item collapse to exactly one.
#[test]
fn multiple_blank_lines_after_comment_collapse_to_one() {
    let src = "-- header\n\n\n\nfn main()\n    print(1)\nend\n";
    let out = fmt(src);
    assert_eq!(out, "-- header\n\nfn main()\n    print(1)\nend\n");
    assert_eq!(fmt(&out), out);
}

/// A comment immediately above an item stays attached with no blank line.
#[test]
fn attached_comment_stays_attached() {
    let src = "-- doc for main\nfn main()\n    print(1)\nend\n";
    let out = fmt(src);
    assert_eq!(out, "-- doc for main\nfn main()\n    print(1)\nend\n");
}

/// Blank separation is preserved between two stacked comment blocks.
#[test]
fn blank_between_comment_blocks_is_preserved() {
    let src = "-- first block\n\n-- second block\nfn main()\n    print(1)\nend\n";
    let out = fmt(src);
    assert_eq!(out, "-- first block\n\n-- second block\nfn main()\n    print(1)\nend\n");
    assert_eq!(fmt(&out), out);
}

/// The header-detached blank survives the roundtrip on the real structs example.
#[test]
fn structs_example_header_blank_survives_formatting() {
    let path = format!("{}/../../examples/structs.axol", env!("CARGO_MANIFEST_DIR"));
    let src = std::fs::read_to_string(&path).expect("read example");
    let out = fmt(&src);
    assert_eq!(out, src, "structs.axol is already canonical");
}
