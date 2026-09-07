// Owner: PascalElixir / axolrs (GitHub org)
// File: JIT parity tests - the compiled path must produce the interpreter's output.

use axol_hot_runner::jit;

/// Assert the JIT output equals the interpreter output for one source string.
fn assert_parity(src: &str) {
    let jit_out = jit::run(src).expect("jit run");
    let (interp_out, _) = axolc_core::interpret(src, 0);
    assert_eq!(jit_out.trim(), interp_out.trim(), "src: {}", src);
}

#[test]
fn jit_parity_int_arith() {
    assert_parity("fn main() print(1 + 2) end");
}

#[test]
fn jit_parity_int_precedence() {
    assert_parity("fn main() print(1 + 2 * 3) end");
}

#[test]
fn jit_parity_parenthesized() {
    assert_parity("fn main() print((1 + 2) * 3) end");
}

#[test]
fn jit_parity_subtraction() {
    assert_parity("fn main() print(10 - 4 - 3) end");
}

#[test]
fn jit_parity_division() {
    assert_parity("fn main() print(84 / 4) end");
}

#[test]
fn jit_parity_remainder() {
    assert_parity("fn main() print(85 % 4) end");
}

#[test]
fn jit_parity_negative_numbers() {
    assert_parity("fn main() print(0 - 7) end");
}

#[test]
fn jit_parity_comparison_eq() {
    assert_parity("fn main() print(3 == 3) end");
}

#[test]
fn jit_parity_comparison_ne() {
    assert_parity("fn main() print(3 != 4) end");
}

#[test]
fn jit_parity_comparison_lt() {
    assert_parity("fn main() print(2 < 10) end");
}

#[test]
fn jit_parity_comparison_le() {
    assert_parity("fn main() print(10 <= 10) end");
}

#[test]
fn jit_parity_comparison_gt() {
    assert_parity("fn main() print(9 > 12) end");
}

#[test]
fn jit_parity_comparison_ge() {
    assert_parity("fn main() print(12 >= 12) end");
}

#[test]
fn jit_parity_bool_and() {
    assert_parity("fn main() print(true and false) end");
}

#[test]
fn jit_parity_bool_or() {
    assert_parity("fn main() print(true or false) end");
}

#[test]
fn jit_parity_bool_not() {
    assert_parity("fn main() print(not true) end");
}

#[test]
fn jit_parity_if_taken() {
    assert_parity("fn main() if 1 < 2 then print(11) else print(22) end end");
}

#[test]
fn jit_parity_if_else_taken() {
    assert_parity("fn main() if 2 < 1 then print(11) else print(22) end end");
}

#[test]
fn jit_parity_elseif_chain() {
    assert_parity("fn main() if 1 > 2 then print(1) elseif 2 > 3 then print(2) else print(3) end end");
}

#[test]
fn jit_parity_while_loop() {
    assert_parity("fn main() var i = 0 var s = 0 while i < 10 do s = s + i i = i + 1 end print(s) end");
}

#[test]
fn jit_parity_while_nested() {
    assert_parity(
        "fn main() var total = 0 var i = 0 while i < 5 do var j = 0 while j < 4 do total = total + 1 j = j + 1 end i = i + 1 end print(total) end",
    );
}

#[test]
fn jit_parity_fib_recursive() {
    assert_parity(
        "fn fib(n: Int) -> Int\n    if n < 2 then\n        return n\n    end\n    return fib(n - 1) + fib(n - 2)\nend\nfn main() print(fib(15)) end",
    );
}

#[test]
fn jit_parity_multi_arg_calls() {
    assert_parity("fn add(a: Int, b: Int, c: Int) -> Int return a + b + c end fn main() print(add(1, 2, 3)) end");
}

#[test]
fn jit_parity_call_chain() {
    assert_parity(
        "fn double(x: Int) -> Int return x * 2 end\nfn quad(x: Int) -> Int return double(double(x)) end\nfn main() print(quad(8)) end",
    );
}

#[test]
fn jit_parity_mutual_recursion_indirect() {
    assert_parity(
        "fn even(n: Int) -> Bool if n == 0 then return true end return odd(n - 1) end\nfn odd(n: Int) -> Bool if n == 0 then return false end return even(n - 1) end\nfn main() print(even(10)) print(odd(7)) end",
    );
}

#[test]
fn jit_parity_early_return() {
    assert_parity("fn f(x: Int) -> Int if x > 5 then return 100 end return 7 end fn main() print(f(9)) print(f(1)) end");
}

#[test]
fn jit_parity_const_fold() {
    assert_parity("const MAX: Int = 100\nfn main() print(MAX / 4) end");
}

#[test]
fn jit_parity_loop_break() {
    assert_parity("fn main() var i = 0 while true do if i == 5 then break end i = i + 1 end print(i) end");
}

#[test]
fn jit_parity_loop_continue() {
    assert_parity(
        "fn main() var i = 0 var s = 0 while i < 10 do i = i + 1 if i % 2 == 0 then continue end s = s + i end print(s) end",
    );
}

#[test]
fn jit_parity_string_program_falls_back() {
    assert_parity("fn main() print(\"hello \" + \"axolotl\") end");
}

#[test]
fn jit_parity_array_program_falls_back() {
    assert_parity("fn main() let xs = [1, 2, 3] print(len(xs)) end");
}

#[test]
fn jit_parity_struct_program_falls_back() {
    assert_parity(
        "P = struct\n    x: Int\nend\nfn main() let p = P { x = 5 } print(p.x) end",
    );
}

#[test]
fn jit_parity_match_program_falls_back() {
    assert_parity(
        "fn main() let x = 2\nmatch x\n    1 => print(10)\n    2 => print(20)\n    _ => print(0)\nend\nend",
    );
}

#[test]
fn jit_fib_20_exact_value() {
    let out = jit::run(
        "fn fib(n: Int) -> Int\n    if n < 2 then\n        return n\n    end\n    return fib(n - 1) + fib(n - 2)\nend\nfn main() print(fib(20)) end",
    )
    .expect("jit run");
    assert_eq!(out.trim(), "6765");
}

#[test]
fn jit_parse_error_returns_err() {
    let result = jit::run("fn main( print(1) end");
    assert!(result.is_err());
}

#[test]
fn jit_plan_marks_int_fn_as_jit() {
    let plan = jit::plan("fn add(a: Int, b: Int) -> Int return a + b end fn main() print(add(1, 2)) end")
        .expect("plan");
    assert!(plan.main_jit);
    let add = plan.decision("add").expect("add decision");
    assert_eq!(format!("{:?}", add.mode), "Jit");
}

#[test]
fn jit_plan_marks_string_fn_as_interpreted() {
    let plan = jit::plan("fn greet() return \"hi\" end fn main() print(greet()) end").expect("plan");
    assert!(!plan.main_jit);
}

#[test]
fn jit_trace_run_succeeds() {
    let out = jit::run_traced("fn main() print(40 + 2) end", true).expect("traced run");
    assert_eq!(out.trim(), "42");
}

#[test]
fn jit_parity_deep_recursion() {
    let handle = std::thread::Builder::new()
        .stack_size(16 * 1024 * 1024)
        .spawn(|| {
            assert_parity(
                "fn sum(n: Int) -> Int\n    if n == 0 then\n        return 0\n    end\n    return n + sum(n - 1)\nend\nfn main() print(sum(100)) end",
            );
        })
        .expect("spawn thread");
    handle.join().expect("join thread");
}

#[test]
fn jit_parity_mixed_program() {
    assert_parity(
        "fn twice(n: Int) -> Int return n * 2 end\nfn main()\n    var acc = 0\n    var i = 0\n    while i < 20 do\n        acc = acc + twice(i)\n        i = i + 1\n    end\n    print(acc)\nend",
    );
}
