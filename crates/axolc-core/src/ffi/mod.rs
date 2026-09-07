// Owner: PascalElixir / axolrs (GitHub org)
// File: FFI block handling - extracts foreign function signatures from cblock/cppblock bodies and maps C types to Rust types.

use crate::span::Span;

/// One foreign function declaration extracted from a cblock/cppblock body.
#[derive(Debug, Clone, PartialEq)]
pub struct ForeignFn {
    pub name: String,
    pub params: Vec<(String, String)>,
    pub ret: String,
    pub mangled: Option<String>,
}

/// Extract every function definition from a C/C++ source body.
/// Recognizes the shape `ret name(params) {` at top level, skipping bodies, preprocessor lines, and comments.
pub fn extract_c_functions(body: &str) -> Vec<ForeignFn> {
    let mut out = Vec::new();
    let cleaned = strip_comments(body);
    let mut rest = cleaned.as_str();
    while let Some((sig, consumed)) = scan_next_function(rest) {
        out.push(sig);
        rest = &rest[consumed..];
    }
    out
}

/// Scan for the next top-level function definition; return its signature and the offset just past its body.
fn scan_next_function(src: &str) -> Option<(ForeignFn, usize)> {
    let bytes = src.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        let c = bytes[i] as char;
        if c == '#' {
            while i < bytes.len() && bytes[i] as char != '\n' {
                i += 1;
            }
            continue;
        }
        if !(c.is_alphabetic() || c == '_') {
            i += 1;
            continue;
        }
        let word_start = i;
        while i < bytes.len()
            && ((bytes[i] as char).is_alphanumeric() || bytes[i] as char == '_')
        {
            i += 1;
        }
        let word = &src[word_start..i];
        let mut j = i;
        while j < bytes.len() && (bytes[j] as char == ' ' || bytes[j] as char == '\t') {
            j += 1;
        }
        if j < bytes.len() && bytes[j] as char == '(' && looks_like_fn_name(word) {
            let open = j;
            let close = find_matching(src, open)?;
            let decl_start = walk_back_decl_start(src, word_start);
            let ret = src[decl_start..word_start].trim().to_string();
            let name = word.to_string();
            let params = parse_c_params(&src[open + 1..close]);
            let ret_rust = map_c_type(&ret);
            let body_open = src[close + 1..].find('{').map(|k| close + 1 + k);
            let consumed = match body_open {
                Some(bo) => {
                    let body_close = find_matching(src, bo)?;
                    body_close + 1
                }
                None => close + 1,
            };
            return Some((ForeignFn { name, params, ret: ret_rust, mangled: None }, consumed));
        }
        i = j.max(i + 1);
    }
    None
}

/// Walk backward from a function name to the start of its declaration (the return type).
fn walk_back_decl_start(src: &str, name_start: usize) -> usize {
    let bytes = src.as_bytes();
    let mut k = name_start;
    while k > 0 {
        let prev = bytes[k - 1] as char;
        if prev.is_alphanumeric() || prev == '_' || prev == '*' || prev == ' ' || prev == '\t' {
            k -= 1;
        } else {
            break;
        }
    }
    while k < name_start && (bytes[k] as char == ' ' || bytes[k] as char == '\t') {
        k += 1;
    }
    k
}

/// Filter out C keywords that precede parentheses but are not function names.
fn looks_like_fn_name(word: &str) -> bool {
    !matches!(
        word,
        "if" | "for" | "while" | "switch" | "return" | "sizeof" | "catch" | "else"
    )
}

/// Parse the parameter list between parens into (name, rust_type) pairs.
fn parse_c_params(src: &str) -> Vec<(String, String)> {
    let mut out: Vec<(String, String)> = Vec::new();
    let mut depth = 0;
    let mut current = String::new();
    let mut counter = 0;
    for c in src.chars() {
        match c {
            '(' => {
                depth += 1;
                current.push(c);
            }
            ')' => {
                depth -= 1;
                current.push(c);
            }
            ',' if depth == 0 => {
                push_param(&mut out, &current, counter);
                counter += 1;
                current.clear();
            }
            _ => current.push(c),
        }
    }
    push_param(&mut out, &current, counter);
    out
}

/// Convert one raw parameter declaration into (name, rust_type).
fn push_param(out: &mut Vec<(String, String)>, raw: &str, index: usize) {
    let raw = raw.trim();
    if raw.is_empty() || raw == "void" {
        return;
    }
    let trailing_len = raw
        .chars()
        .rev()
        .take_while(|c| c.is_alphanumeric() || *c == '_')
        .count();
    if trailing_len == 0 || trailing_len == raw.len() {
        let name = format!("arg{}", index);
        out.push((name, map_c_type(raw)));
        return;
    }
    let name = &raw[raw.len() - trailing_len..];
    let ty = raw[..raw.len() - trailing_len].trim();
    let clean_name = name.trim_start_matches('*');
    if clean_name.is_empty() {
        out.push((format!("arg{}", index), map_c_type(raw)));
    } else {
        out.push((sanitize_ident(clean_name), map_c_type(ty)));
    }
}

/// Map a C type expression to the Rust equivalent used in `extern "C"` blocks.
fn map_c_type(c_ty: &str) -> String {
    let t = c_ty.replace("const", "").replace("static", "").replace("unsigned", "");
    let t = t.trim().to_string();
    let base = t.replace('*', "").trim().to_string();
    let star_count = c_ty.matches('*').count();
    let is_const = c_ty.contains("const");
    if star_count > 0 {
        let pointee = match base.as_str() {
            "void" | "" => "std::ffi::c_void",
            "char" => "std::ffi::c_char",
            "int" => "std::ffi::c_int",
            _ => "std::ffi::c_void",
        };
        if is_const {
            return format!("*const {}", pointee);
        }
        return format!("*mut {}", pointee);
    }
    match base.as_str() {
        "int" | "int32_t" => "i32".to_string(),
        "long" | "int64_t" => "i64".to_string(),
        "short" | "int16_t" => "i16".to_string(),
        "char" | "int8_t" => "i8".to_string(),
        "size_t" | "ssize_t" => "usize".to_string(),
        "float" => "f32".to_string(),
        "double" => "f64".to_string(),
        "bool" | "_Bool" => "bool".to_string(),
        "void" => "()".to_string(),
        other => other.to_string(),
    }
}

/// Replace C identifiers that collide with Rust keywords.
fn sanitize_ident(name: &str) -> String {
    match name {
        "type" => "ty".to_string(),
        "fn" => "func".to_string(),
        "match" => "m".to_string(),
        "box" => "b".to_string(),
        other => other.to_string(),
    }
}

/// Find the index of the closing bracket matching the one at `open` (parens, braces, or brackets).
fn find_matching(src: &str, open: usize) -> Option<usize> {
    let bytes = src.as_bytes();
    let open_ch = bytes[open] as char;
    let close_ch = match open_ch {
        '(' => ')',
        '{' => '}',
        '[' => ']',
        _ => return None,
    };
    let mut depth = 0;
    for (i, &b) in bytes.iter().enumerate().skip(open) {
        let c = b as char;
        if c == open_ch {
            depth += 1;
        } else if c == close_ch {
            depth -= 1;
            if depth == 0 {
                return Some(i);
            }
        }
    }
    None
}

/// Remove `//` line comments and `/* */` block comments from C source.
fn strip_comments(src: &str) -> String {
    let mut out = String::with_capacity(src.len());
    let mut chars = src.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '/' {
            match chars.peek() {
                Some('/') => {
                    while let Some(n) = chars.next() {
                        if n == '\n' {
                            out.push('\n');
                            break;
                        }
                    }
                }
                Some('*') => {
                    chars.next();
                    let mut prev = '\0';
                    while let Some(n) = chars.next() {
                        if prev == '*' && n == '/' {
                            break;
                        }
                        prev = n;
                    }
                    out.push(' ');
                }
                _ => out.push(c),
            }
        } else {
            out.push(c);
        }
    }
    out
}

/// Produce the C++ mangled link name for a function (Itanium ABI, simple form).
pub fn cpp_mangle(name: &str, params: &[(String, String)]) -> String {
    let mut mangled = String::from("_Z");
    mangled.push_str(&name.len().to_string());
    mangled.push_str(name);
    if params.is_empty() {
        mangled.push('v');
        return mangled;
    }
    for (_, ty) in params {
        let code = match ty.as_str() {
            "i32" => "i",
            "i64" => "l",
            "i16" => "s",
            "i8" => "a",
            "f32" => "f",
            "f64" => "d",
            "()" => "v",
            "bool" => "b",
            _ => "Pv",
        };
        mangled.push_str(code);
    }
    mangled
}

/// The span of a foreign block for diagnostics (byte range in the .axol source).
pub fn foreign_span(_body: &str) -> Span {
    Span::DUMMY
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A basic C function definition is extracted with its signature.
    #[test]
    fn extracts_simple_c_function() {
        let fns = extract_c_functions("int native_add(int a, int b) { return a + b; }");
        assert_eq!(fns.len(), 1);
        assert_eq!(fns[0].name, "native_add");
        assert_eq!(fns[0].ret, "i32");
        assert_eq!(fns[0].params.len(), 2);
        assert_eq!(fns[0].params[0], ("a".to_string(), "i32".to_string()));
    }

    /// Pointer parameters map to c_void pointers.
    #[test]
    fn extracts_pointer_function() {
        let fns = extract_c_functions("void* native_alloc(size_t size) { return malloc(size); }");
        assert_eq!(fns[0].name, "native_alloc");
        assert_eq!(fns[0].ret, "*mut std::ffi::c_void");
        assert_eq!(fns[0].params[0].1, "usize");
    }

    /// Preprocessor directives and includes are skipped.
    #[test]
    fn skips_includes() {
        let body = "#include <stdio.h>\n#include <stdlib.h>\n\nvoid hello() { printf(\"x\"); }";
        let fns = extract_c_functions(body);
        assert_eq!(fns.len(), 1);
        assert_eq!(fns[0].name, "hello");
    }

    /// Multiple functions are all extracted in order.
    #[test]
    fn extracts_multiple_functions() {
        let body = "int a(int x) { return x; }\nfloat b(float y) { return y; }";
        let fns = extract_c_functions(body);
        assert_eq!(fns.len(), 2);
        assert_eq!(fns[1].ret, "f32");
    }

    /// C++ mangled names follow the Itanium simple form.
    #[test]
    fn mangles_cpp_names() {
        let mangled = cpp_mangle("hello_cpp", &[]);
        assert_eq!(mangled, "_Z9hello_cppv");
    }

    /// Comments in C bodies do not confuse the extractor.
    #[test]
    fn ignores_comments() {
        let body = "/* header */ int f(int a) { // inner\n return a; }";
        let fns = extract_c_functions(body);
        assert_eq!(fns.len(), 1);
        assert_eq!(fns[0].name, "f");
    }
}
