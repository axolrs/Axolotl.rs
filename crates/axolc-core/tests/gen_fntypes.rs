// Owner: PascalElixir / axolrs (GitHub org)
// File: Function-type, closure-escape, and interpolation codegen tests.

use axolc_testkit::{emit_rust, interpret_output, parses_clean};

#[test]
fn fn_type_parses_in_return_position() {
    assert!(parses_clean("fn make() -> fn() -> Int return fn() -> Int return 1 end end"));
}

#[test]
fn fn_type_parses_with_params() {
    assert!(parses_clean("fn make() -> fn(Int, Int) -> Int return fn(a: Int, b: Int) -> Int return a + b end end"));
}

#[test]
fn fn_type_parses_in_let_annotation() {
    assert!(parses_clean("fn main() let f: fn(Int) -> Int = fn(x) x end end"));
}

#[test]
fn fn_type_parses_in_param_position() {
    assert!(parses_clean("fn apply(f: fn(Int) -> Int, x: Int) -> Int return f(x) end"));
}

#[test]
fn fn_type_parses_without_arrow() {
    assert!(parses_clean("fn apply(f: fn(Int)) return 1 end"));
}

#[test]
fn counter_interprets() {
    let out = interpret_output(
        "fn make_counter() -> fn() -> Int
            var count = 0
            return fn() -> Int
                count = count + 1
                return count
            end
        end
        fn main()
            let next = make_counter()
            print(next())
            print(next())
            print(next())
        end",
    );
    assert_eq!(out, "1\n2\n3\n");
}

#[test]
fn fn_type_emits_boxed_fnmut() {
    let rust = emit_rust("fn make() -> fn() -> Int return fn() -> Int return 1 end end");
    assert!(rust.contains("Box<dyn FnMut() -> i64>"), "no boxed fn type in: {}", rust);
}

#[test]
fn fn_type_params_emit_boxed() {
    let rust = emit_rust("fn apply(f: fn(Int) -> Int) -> Int return f(1) end");
    assert!(rust.contains("Box<dyn FnMut(i64) -> i64>"), "no boxed param type in: {}", rust);
}

#[test]
fn fn_type_param_is_mut() {
    let rust = emit_rust("fn apply(f: fn(Int) -> Int) -> Int return f(1) end");
    assert!(rust.contains("mut f:"), "param not mut in: {}", rust);
}

#[test]
fn capturing_closure_emits_move() {
    let rust = emit_rust(
        "fn make() -> fn() -> Int
            var count = 0
            return fn() -> Int return count end
        end",
    );
    assert!(rust.contains("move |"), "no move closure in: {}", rust);
}

#[test]
fn noncapturing_closure_no_move() {
    let rust = emit_rust("fn main() let f = fn(x) x * 2 end print(f(3)) end");
    assert!(!rust.contains("move |"), "unexpected move in: {}", rust);
}

#[test]
fn closure_let_from_fn_call_is_mut() {
    let rust = emit_rust(
        "fn make() -> fn() -> Int return fn() -> Int return 1 end end
        fn main() let next = make() print(next()) end",
    );
    assert!(rust.contains("let mut next"), "next not mut in: {}", rust);
}

#[test]
fn closure_emits_boxed() {
    let rust = emit_rust("fn main() let f = fn(x) x * 2 end print(f(3)) end");
    assert!(rust.contains("Box::new("), "closure not boxed in: {}", rust);
}

#[test]
fn interp_string_interpolates_interpreter() {
    let out = interpret_output("fn main() let x = 5 print(\"x is ${x}\") end");
    assert_eq!(out, "x is 5\n");
}

#[test]
fn interp_string_emits_format() {
    let rust = emit_rust("fn main() let x = 5 print(\"x is ${x}\") end");
    assert!(rust.contains("format!(\"x is {}\", x)"), "no format! in: {}", rust);
}

#[test]
fn interp_string_two_exprs() {
    let out = interpret_output("fn main() let a = 1 let b = 2 print(\"${a}+${b}\") end");
    assert_eq!(out, "1+2\n");
}

#[test]
fn interp_string_nested_braces() {
    let out = interpret_output(
        "fn main()
            let p = Point { x = 3, y = 4 }
            print(\"${p.x}\")
        end
        Point = struct
            x: Int
            y: Int
        end",
    );
    assert_eq!(out, "3\n");
}

#[test]
fn print_array_uses_debug_format() {
    let rust = emit_rust("fn main() let xs = [1, 2, 3] print(xs) end");
    assert!(rust.contains("{:?}"), "no debug format in: {}", rust);
}

#[test]
fn print_map_uses_debug_format() {
    let rust = emit_rust("fn main() let m = {\"a\" = 1} print(m) end");
    assert!(rust.contains("{:?}"), "no debug format in: {}", rust);
}

#[test]
fn print_int_still_display_format() {
    let rust = emit_rust("fn main() let n = 5 print(n) end");
    assert!(rust.contains("println!(\"{}\", n)"), "unexpected format in: {}", rust);
}

#[test]
fn print_mixed_args_pick_format_per_arg() {
    let rust = emit_rust("fn main() let xs = [1] print(\"sum:\", xs) end");
    assert!(rust.contains("\"{} {:?}\""), "mixed formats wrong in: {}", rust);
}

#[test]
fn print_array_interprets() {
    let out = interpret_output("fn main() let xs = [1, 2, 3] print(xs) end");
    assert_eq!(out, "[1, 2, 3]\n");
}

#[test]
fn array_var_propagates_debug_format() {
    let rust = emit_rust("fn main() var xs = [1] xs.push(2) print(xs) end");
    assert!(rust.contains("{:?}"), "no debug format in: {}", rust);
}

#[test]
fn let_from_array_var_keeps_debug_format() {
    let rust = emit_rust("fn main() var xs = [1] let ys = xs print(ys) end");
    assert!(rust.contains("{:?}"), "no debug format in: {}", rust);
}
