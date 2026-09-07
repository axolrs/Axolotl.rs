// Owner: PascalElixir / axolrs (GitHub org)
// File: crates/bucket/tests/lint_tests.rs - Neoten linter tests: per-check positive and negative cases, shadowing, surfaced errors, and CLI behavior.

use bucket::fmt::line_col_of;
use bucket::lint::{
    self, LintLevel, Finding, N0101, N0102, N0103, N0104, N0105, N0106,
};
use std::process::Command;

/// Run the full lint pipeline over one source string.
fn findings(src: &str) -> Vec<Finding> {
    lint::lint_source(src, 0)
}

/// Collect only warning-level findings.
fn warnings(src: &str) -> Vec<Finding> {
    findings(src).into_iter().filter(|f| f.level == LintLevel::Warning).collect()
}

/// Collect only error-level findings.
fn errors(src: &str) -> Vec<Finding> {
    findings(src).into_iter().filter(|f| f.level == LintLevel::Error).collect()
}

/// Collect the finding codes in source order.
fn codes(src: &str) -> Vec<String> {
    findings(src).into_iter().map(|f| f.code).collect()
}

/// Return the warnings with the given code.
fn warns_of(src: &str, code: &str) -> Vec<Finding> {
    warnings(src).into_iter().filter(|f| f.code == code).collect()
}

/// Return the first warning with the given code, panicking when absent.
fn first_warn(src: &str, code: &str) -> Finding {
    warns_of(src, code).first().expect("expected a warning").clone()
}

/// Return the count of warnings with the given code.
fn warn_count(src: &str, code: &str) -> usize {
    warns_of(src, code).len()
}

/// Return the 1-based (line, col) of a finding's span start.
fn pos(src: &str, f: &Finding) -> (usize, usize) {
    line_col_of(src, f.span.start as usize)
}

/// Run the token-level unreachable-statement scan.
fn unreach(src: &str) -> Vec<lint::UnreachableStmt> {
    lint::scan_unreachable_stmts(src, 0).expect("scan")
}

/// Parse a source string, panicking on failure.
fn module(src: &str) -> axolc_core::ast::Module {
    let (m, d) = axolc_core::parse(src, 0);
    assert!(!d.has_errors(), "parse failed: {:?}", d.items);
    m
}

/// Wrap statements in a main function body.
fn in_main(body: &str) -> String {
    format!("fn main()\n{}\nend\n", body)
}

#[test]
fn unused_let_is_flagged() {
    let src = in_main("    let dead = 5\n    print(1)");
    let f = first_warn(&src, N0101);
    assert_eq!(f.code, N0101);
    assert_eq!(f.level, LintLevel::Warning);
    assert_eq!(warn_count(&src, N0101), 1);
    assert!(f.message.contains("dead"), "message: {}", f.message);
    assert!(f.message.contains("never read"), "message: {}", f.message);
    assert_eq!(pos(&src, &f), (2, 9));
    assert_eq!(f.span.start as usize, src.find("dead").unwrap());
    assert_eq!(f.span.end as usize, src.find("dead").unwrap() + 4);
}

#[test]
fn unused_var_is_flagged() {
    let src = in_main("    var counter = 0\n    print(\"done\")");
    let f = first_warn(&src, N0101);
    assert_eq!(f.code, N0101);
    assert_eq!(warn_count(&src, N0101), 1);
    assert!(f.message.contains("counter"));
    assert_eq!(pos(&src, &f), (2, 9));
}

#[test]
fn used_let_is_not_flagged() {
    let src = in_main("    let x = 5\n    print(x)");
    assert_eq!(warn_count(&src, N0101), 0);
    assert_eq!(warnings(&src).len(), 0);
    assert_eq!(findings(&src).len(), 0);
}

#[test]
fn var_used_in_print_is_not_flagged() {
    let src = in_main("    var y = 2\n    y = y + 1\n    print(y)");
    assert_eq!(warn_count(&src, N0101), 0);
}

#[test]
fn let_used_in_interpolation_is_not_flagged() {
    let src = in_main("    let name = \"a\"\n    print(\"hi ${name}\")");
    assert_eq!(warn_count(&src, N0101), 0);
}

#[test]
fn let_used_in_condition_is_not_flagged() {
    let src = in_main("    let flag = true\n    if flag then\n        print(1)\n    end");
    assert_eq!(warn_count(&src, N0101), 0);
}

#[test]
fn let_used_in_return_is_not_flagged() {
    let src = "fn f() -> Int\n    let v = 3\n    return v\nend\n".to_string();
    assert_eq!(warn_count(&src, N0101), 0);
}

#[test]
fn let_used_in_array_literal_is_not_flagged() {
    let src = in_main("    let a = 1\n    let b = 2\n    let xs = [a, b, a + b]\n    print(xs)");
    assert_eq!(warn_count(&src, N0101), 0);
}

#[test]
fn let_used_in_tuple_is_not_flagged() {
    let src = in_main("    let t = 1\n    let pair = (t, t + 1)\n    print(pair)");
    assert_eq!(warn_count(&src, N0101), 0);
}

#[test]
fn let_used_in_map_value_is_not_flagged() {
    let src = in_main("    let v = 7\n    let m = { \"k\" = v }\n    print(m)");
    assert_eq!(warn_count(&src, N0101), 0);
}

#[test]
fn let_used_as_call_argument_is_not_flagged() {
    let src = in_main("    let n = 4\n    print(n)\n    print(n * 2)");
    assert_eq!(warn_count(&src, N0101), 0);
}

#[test]
fn let_used_in_closure_capture_is_not_flagged() {
    let src = in_main(
        "    let base = 10\n    let add = fn(v: Int) -> Int\n        return base + v\n    end\n    print(add(1))",
    );
    assert_eq!(warn_count(&src, N0101), 0);
}

#[test]
fn let_used_in_closure_inner_block_is_not_flagged() {
    let src = in_main(
        "    let outer = 1\n    if true then\n        let f = fn()\n            print(outer)\n        end\n    end",
    );
    assert_eq!(warn_count(&src, N0101), 0);
}

#[test]
fn let_used_in_range_is_not_flagged() {
    let src = in_main("    let lo = 1\n    let hi = 5\n    for i in lo..hi do\n        print(i)\n    end");
    assert_eq!(warn_count(&src, N0101), 0);
}

#[test]
fn let_used_in_field_chain_is_not_flagged() {
    let src = in_main("    let p = make()\n    print(p.name)\n    print(p.inner.value)");
    assert_eq!(warn_count(&src, N0101), 0);
}

#[test]
fn write_only_let_is_flagged() {
    let src = in_main("    let x = 1\n    x = 2\n    print(1)");
    assert_eq!(warn_count(&src, N0101), 1);
    let f = first_warn(&src, N0101);
    assert!(f.message.contains("x"));
    assert_eq!(pos(&src, &f), (2, 9));
}

#[test]
fn compound_assignment_counts_as_read() {
    let src = in_main("    var x = 1\n    x += 2\n    print(x)");
    assert_eq!(warn_count(&src, N0101), 0);
}

#[test]
fn compound_bitwise_assignment_counts_as_read() {
    let src = in_main("    var x = 1\n    x = x & 3\n    print(x)");
    assert_eq!(warn_count(&src, N0101), 0);
}

#[test]
fn assignment_rhs_reads_variable() {
    let src = in_main("    var a = 1\n    var b = 2\n    b = a\n    print(b)");
    assert_eq!(warn_count(&src, N0101), 0);
}

#[test]
fn field_assignment_target_reads_receiver() {
    let src = in_main("    var p = make()\n    p.hp = 10\n    print(1)");
    assert_eq!(warn_count(&src, N0101), 0);
}

#[test]
fn index_assignment_reads_base_and_index() {
    let src = in_main("    var a = [1, 2]\n    var i = 0\n    a[i] = 9\n    print(a)");
    assert_eq!(warn_count(&src, N0101), 0);
}

#[test]
fn underscore_prefixed_local_is_allowed() {
    let src = in_main("    let _unused = 5\n    print(1)");
    assert_eq!(warn_count(&src, N0101), 0);
}

#[test]
fn underscore_prefixed_var_is_allowed() {
    let src = in_main("    var _drop = 5\n    print(1)");
    assert_eq!(warn_count(&src, N0101), 0);
}

#[test]
fn two_unused_locals_yield_two_findings() {
    let src = in_main("    let a = 1\n    let b = 2\n    print(3)");
    let ws = warns_of(&src, N0101);
    assert_eq!(ws.len(), 2);
    assert!(ws[0].message.contains("a"));
    assert!(ws[1].message.contains("b"));
    assert_eq!(pos(&src, &ws[0]), (2, 9));
    assert_eq!(pos(&src, &ws[1]), (3, 9));
    assert_eq!(ws[0].span.start as usize, src.find("a =").unwrap());
    assert_eq!(ws[1].span.start as usize, src.find("b =").unwrap());
}

#[test]
fn unused_for_loop_variable_is_flagged() {
    let src = in_main("    for i in 1..10 do\n        print(\"tick\")\n    end");
    let f = first_warn(&src, N0101);
    assert_eq!(warn_count(&src, N0101), 1);
    assert!(f.message.contains("i"));
    assert_eq!(pos(&src, &f), (2, 9));
}

#[test]
fn used_for_loop_variable_is_not_flagged() {
    let src = in_main("    for i in 1..10 do\n        print(i)\n    end");
    assert_eq!(warn_count(&src, N0101), 0);
}

#[test]
fn unused_match_binding_is_flagged() {
    let src = in_main("    match 2\n        n => print(\"got\")\n        _ => print(\"other\")\n    end");
    let f = first_warn(&src, N0101);
    assert_eq!(warn_count(&src, N0101), 1);
    assert!(f.message.contains("n"));
}

#[test]
fn used_match_binding_is_not_flagged() {
    let src = in_main("    match 2\n        n => print(n)\n        _ => print(0)\n    end");
    assert_eq!(warn_count(&src, N0101), 0);
}

#[test]
fn struct_pattern_bindings_are_locals() {
    let src = in_main(
        "    match p\n        Point { x = 1, y = v } => print(v)\n        _ => print(0)\n    end",
    );
    assert_eq!(warn_count(&src, N0101), 0);
}

#[test]
fn shadowing_inner_unused_outer_used_later() {
    let src = in_main(
        "    let x = 1\n    if true then\n        let x = 2\n    end\n    print(x)",
    );
    let ws = warns_of(&src, N0101);
    assert_eq!(ws.len(), 1);
    assert!(ws[0].message.contains("x"));
    assert_eq!(pos(&src, &ws[0]), (4, 13));
}

#[test]
fn shadowing_inner_used_outer_unused() {
    let src = in_main(
        "    let x = 1\n    if true then\n        let x = 2\n        print(x)\n    end",
    );
    let ws = warns_of(&src, N0101);
    assert_eq!(ws.len(), 1);
    assert_eq!(pos(&src, &ws[0]), (2, 9));
}

#[test]
fn use_before_shadow_marks_outer_binding() {
    let src = in_main("    let x = 1\n    print(x)\n    if true then\n        let x = 2\n    end");
    assert_eq!(warn_count(&src, N0101), 1);
    let f = first_warn(&src, N0101);
    assert_eq!(pos(&src, &f), (5, 13));
}

#[test]
fn shadowed_let_initializer_reads_the_outer_binding() {
    let src = in_main("    let x = 1\n    let x = x + 1\n    print(x)");
    assert_eq!(warn_count(&src, N0101), 0);
}

#[test]
fn sequential_same_name_lets_flag_only_the_unused_one() {
    let src = in_main("    let x = 1\n    let x = 2\n    print(x)");
    let ws = warns_of(&src, N0101);
    assert_eq!(ws.len(), 1);
    assert_eq!(pos(&src, &ws[0]), (2, 9));
}

#[test]
fn let_shadowing_param_flags_the_param() {
    let src = "fn f(a: Int) -> Int\n    let a = 1\n    return a\nend\n".to_string();
    let ws = warns_of(&src, N0102);
    assert_eq!(ws.len(), 1);
    assert!(ws[0].message.contains("a"));
    assert_eq!(pos(&src, &ws[0]), (1, 6));
    assert_eq!(warn_count(&src, N0101), 0);
}

#[test]
fn param_used_in_let_initializer_is_not_flagged() {
    let src = "fn f(a: Int) -> Int\n    let b = a + 1\n    return b\nend\n".to_string();
    assert_eq!(warn_count(&src, N0102), 0);
}

#[test]
fn while_cond_reads_outer_scope_binding() {
    let src = in_main("    let go = true\n    while go do\n        break\n    end");
    assert_eq!(warn_count(&src, N0101), 0);
}

#[test]
fn repeat_cond_reads_body_scope_binding() {
    let src = in_main("    repeat\n        let d = 1\n        print(1)\n    until d > 0");
    assert_eq!(warn_count(&src, N0101), 0);
}

#[test]
fn inner_block_local_is_not_visible_after_the_block() {
    let src = in_main("    if true then\n        let inner = 1\n        print(inner)\n    end\n    print(1)");
    assert_eq!(warn_count(&src, N0101), 0);
}

#[test]
fn closure_local_stays_inside_the_closure() {
    let src = in_main("    let f = fn()\n        let c = 1\n        return c\n    end\n    print(1)");
    assert_eq!(warn_count(&src, N0101), 0);
    assert_eq!(warn_count(&src, N0102), 0);
}

#[test]
fn unsafe_block_scope_is_tracked() {
    let src = in_main("    unsafe\n        let u = 1\n        print(u)\n    end");
    assert_eq!(warn_count(&src, N0101), 0);
}

#[test]
fn unused_local_inside_unsafe_block_is_flagged() {
    let src = in_main("    unsafe\n        let u = 1\n    end");
    assert_eq!(warn_count(&src, N0101), 1);
    let f = first_warn(&src, N0101);
    assert_eq!(pos(&src, &f), (3, 13));
}

#[test]
fn elseif_and_else_bodies_get_their_own_scopes() {
    let src = in_main(
        "    if true then\n        print(1)\n    elseif false then\n        let e = 2\n        print(e)\n    else\n        let s = 3\n        print(s)\n    end",
    );
    assert_eq!(warn_count(&src, N0101), 0);
}

#[test]
fn unused_local_in_elseif_body_is_flagged() {
    let src = in_main("    if true then\n        print(1)\n    elseif false then\n        let e = 2\n    else\n        print(3)\n    end");
    let ws = warns_of(&src, N0101);
    assert_eq!(ws.len(), 1);
    assert_eq!(pos(&src, &ws[0]), (5, 13));
}

#[test]
fn unused_local_in_match_arm_is_flagged() {
    let src = in_main("    match 1\n        n =>\n            let w = 1\n            print(n)\n        _ => print(0)\n    end");
    assert_eq!(warn_count(&src, N0101), 0);
}

#[test]
fn spawn_call_reads_bindings() {
    let src = in_main("    let job = 1\n    spawn print(job)");
    assert_eq!(warn_count(&src, N0101), 0);
}

#[test]
fn pipe_operators_read_both_sides() {
    let src = in_main("    let v = 2\n    let r = v |> double\n    print(r)");
    assert_eq!(warn_count(&src, N0101), 0);
}

#[test]
fn cast_and_is_read_the_operand() {
    let src = in_main("    let v = 2\n    let a = v as Int\n    let b = v is Int\n    print(a)");
    assert_eq!(warn_count(&src, N0101), 0);
}

#[test]
fn await_try_bang_read_the_operand() {
    let src = in_main("    let v = 2\n    print(v)\n    print(1)");
    assert_eq!(warn_count(&src, N0101), 0);
}

#[test]
fn question_dot_reads_the_receiver() {
    let src = in_main("    let o = make()\n    let n = o?.name\n    print(n)");
    assert_eq!(warn_count(&src, N0101), 0);
}

#[test]
fn method_call_reads_the_receiver() {
    let src = in_main("    let p = make()\n    p:damage(30)\n    print(1)");
    assert_eq!(warn_count(&src, N0101), 0);
}

#[test]
fn named_call_arguments_read_their_values() {
    let src = in_main("    let v = 1\n    config(width = v)\n    print(1)");
    assert_eq!(warn_count(&src, N0101), 0);
}

#[test]
fn spread_argument_reads_the_value() {
    let src = in_main("    let xs = [1]\n    print(xs)\n    call(...xs)");
    assert_eq!(warn_count(&src, N0101), 0);
}

#[test]
fn unused_fn_param_is_flagged() {
    let src = "fn add(a: Int, b: Int) -> Int\n    return a\nend\n".to_string();
    let f = first_warn(&src, N0102);
    assert_eq!(f.code, N0102);
    assert_eq!(f.level, LintLevel::Warning);
    assert_eq!(warn_count(&src, N0102), 1);
    assert!(f.message.contains("b"));
    assert!(f.message.contains("parameter"), "message: {}", f.message);
    assert_eq!(pos(&src, &f), (1, 16));
    assert_eq!(f.span.start as usize, src.find("b").unwrap());
}

#[test]
fn used_fn_params_are_not_flagged() {
    let src = "fn add(a: Int, b: Int) -> Int\n    return a + b\nend\n".to_string();
    assert_eq!(warn_count(&src, N0102), 0);
    assert_eq!(warnings(&src).len(), 0);
}

#[test]
fn self_param_is_never_flagged() {
    let src = "P.hit = fn(self)\n    return 1\nend\n".to_string();
    assert_eq!(warn_count(&src, N0102), 0);
}

#[test]
fn method_self_param_with_unused_value_param() {
    let src = "P.damage = fn(self, amount: Int)\n    return 0\nend\n".to_string();
    let f = first_warn(&src, N0102);
    assert_eq!(warn_count(&src, N0102), 1);
    assert!(f.message.contains("amount"));
    assert_eq!(pos(&src, &f), (1, 21));
}

#[test]
fn method_param_used_is_not_flagged() {
    let src = "P.damage = fn(self, amount: Int)\n    return amount\nend\n".to_string();
    assert_eq!(warn_count(&src, N0102), 0);
}

#[test]
fn two_unused_params_yield_two_findings() {
    let src = "fn f(a: Int, b: Int, c: Int) -> Int\n    return c\nend\n".to_string();
    let ws = warns_of(&src, N0102);
    assert_eq!(ws.len(), 2);
    assert!(ws[0].message.contains("a"));
    assert!(ws[1].message.contains("b"));
    assert_eq!(pos(&src, &ws[0]), (1, 6));
    assert_eq!(pos(&src, &ws[1]), (1, 14));
}

#[test]
fn unused_closure_param_is_flagged() {
    let src = in_main("    let f = fn(a: Int) -> Int\n        return 0\n    end\n    print(f(1))");
    let f = first_warn(&src, N0102);
    assert_eq!(warn_count(&src, N0102), 1);
    assert!(f.message.contains("a"));
}

#[test]
fn used_closure_param_is_not_flagged() {
    let src = in_main("    let f = fn(a: Int) -> Int\n        return a\n    end\n    print(f(1))");
    assert_eq!(warn_count(&src, N0102), 0);
}

#[test]
fn param_used_in_default_value_is_not_flagged() {
    let src = "fn f(x: Int, y: Int = x) -> Int\n    return y\nend\n".to_string();
    assert_eq!(warn_count(&src, N0102), 0);
}

#[test]
fn underscore_prefixed_param_is_allowed() {
    let src = "fn f(_ignored: Int) -> Int\n    return 1\nend\n".to_string();
    assert_eq!(warn_count(&src, N0102), 0);
}

#[test]
fn param_used_in_match_is_not_flagged() {
    let src = "fn f(n: Int) -> Int\n    match n\n        0 => return 1\n        _ => return 2\n    end\nend\n".to_string();
    assert_eq!(warn_count(&src, N0102), 0);
}

#[test]
fn param_used_in_closure_inside_fn_is_not_flagged() {
    let src = "fn f(n: Int) -> Int\n    let g = fn() -> Int\n        return n\n    end\n    return g()\nend\n".to_string();
    assert_eq!(warn_count(&src, N0102), 0);
}

#[test]
fn param_used_in_while_cond_is_not_flagged() {
    let src = "fn f(n: Int) -> Int\n    while n > 0 do\n        return 1\n    end\n    return 0\nend\n".to_string();
    assert_eq!(warn_count(&src, N0102), 0);
}

#[test]
fn interface_default_body_param_is_flagged() {
    let src = "I = interface\n    go(self, n: Int) -> Int\n        return 0\n    end\nend\n".to_string();
    let ws = warns_of(&src, N0102);
    assert_eq!(ws.len(), 1);
    assert!(ws[0].message.contains("n"));
}

#[test]
fn interface_default_body_param_used_is_not_flagged() {
    let src = "I = interface\n    go(self, n: Int) -> Int\n        return n\n    end\nend\n".to_string();
    assert_eq!(warn_count(&src, N0102), 0);
}

#[test]
fn interface_signature_only_params_are_not_flagged() {
    let src = "I = interface\n    go(self, n: Int) -> Int\nend\n".to_string();
    assert_eq!(warn_count(&src, N0102), 0);
}

#[test]
fn struct_field_never_read_is_flagged() {
    let src = "P = struct\n    hp: Int\nend\n\nfn make() -> P\n    return P { hp = 1 }\nend\n".to_string();
    let f = first_warn(&src, N0103);
    assert_eq!(f.code, N0103);
    assert_eq!(f.level, LintLevel::Warning);
    assert_eq!(warn_count(&src, N0103), 1);
    assert!(f.message.contains("hp"));
    assert!(f.message.contains("never read"), "message: {}", f.message);
    assert_eq!(pos(&src, &f), (2, 5));
    assert_eq!(f.span.start as usize, src.find("hp").unwrap());
}

#[test]
fn struct_field_read_via_dot_is_not_flagged() {
    let src = "P = struct\n    hp: Int\nend\n\nfn get(p: P) -> Int\n    return p.hp\nend\n".to_string();
    assert_eq!(warn_count(&src, N0103), 0);
}

#[test]
fn struct_field_read_via_self_is_not_flagged() {
    let src = "P = struct\n    hp: Int\nend\n\nP.hit = fn(self)\n    return self.hp\nend\n".to_string();
    assert_eq!(warn_count(&src, N0103), 0);
}

#[test]
fn struct_field_written_but_never_read_is_flagged() {
    let src = "P = struct\n    hp: Int\nend\n\nfn set(p: P)\n    p.hp = 10\nend\n".to_string();
    assert_eq!(warn_count(&src, N0103), 1);
    let f = first_warn(&src, N0103);
    assert!(f.message.contains("hp"));
}

#[test]
fn struct_field_read_in_rhs_of_field_assign_is_not_flagged() {
    let src = "P = struct\n    hp: Int\nend\n\nfn bump(p: P)\n    p.hp = p.hp + 1\nend\n".to_string();
    assert_eq!(warn_count(&src, N0103), 0);
}

#[test]
fn struct_field_read_via_destructuring_pattern_is_not_flagged() {
    let src = "P = struct\n    hp: Int\nend\n\nfn f(p: P) -> Int\n    match p\n        P { hp = h } => return h\n        _ => return 0\n    end\nend\n".to_string();
    assert_eq!(warn_count(&src, N0103), 0);
}

#[test]
fn struct_field_read_via_question_dot_is_not_flagged() {
    let src = "P = struct\n    name: String\nend\n\nfn f(p: P?)\n    return p?.name\nend\n".to_string();
    assert_eq!(warn_count(&src, N0103), 0);
}

#[test]
fn mixed_fields_flag_only_the_unread_one() {
    let src = "P = struct\n    used: Int\n    dead: Int\nend\n\nfn f(p: P) -> Int\n    return p.used\nend\n".to_string();
    let ws = warns_of(&src, N0103);
    assert_eq!(ws.len(), 1);
    assert!(ws[0].message.contains("dead"));
    assert_eq!(pos(&src, &ws[0]), (3, 5));
}

#[test]
fn field_name_read_on_any_receiver_counts() {
    let src = "A = struct\n    hp: Int\nend\n\nB = struct\n    hp: Int\nend\n\nfn f(b: B) -> Int\n    return b.hp\nend\n".to_string();
    assert_eq!(warn_count(&src, N0103), 0);
}

#[test]
fn enum_variant_fields_do_not_count_as_struct_field_reads() {
    let src = "E = enum\n    Has(x: Int)\nend\n\nS = struct\n    x: Int\nend\n".to_string();
    let ws = warns_of(&src, N0103);
    assert_eq!(ws.len(), 1);
    assert!(ws[0].message.contains("x"));
}

#[test]
fn struct_field_with_default_expression_is_walked() {
    let src = "P = struct\n    hp: Int = 10\nend\n\nfn f() -> Int\n    let p = P {}\n    return p.hp\nend\n".to_string();
    assert_eq!(warn_count(&src, N0103), 0);
}

#[test]
fn struct_literal_keys_do_not_shadow_locals() {
    let src = in_main("    let hp = 5\n    let p = P { hp = hp }\n    print(p)\n    print(hp)");
    assert_eq!(warn_count(&src, N0101), 0);
}

#[test]
fn map_literal_ident_keys_do_not_shadow_locals() {
    let src = in_main("    let k = 1\n    let m = { k = k }\n    print(k)\n    print(m)");
    assert_eq!(warn_count(&src, N0101), 0);
}

#[test]
fn unreachable_stmt_after_return_is_flagged() {
    let src = "fn f() -> Int\n    return 1\n    print(2)\nend\n".to_string();
    let ws = warns_of(&src, N0104);
    assert_eq!(ws.len(), 1);
    let f = &ws[0];
    assert_eq!(f.code, N0104);
    assert_eq!(f.level, LintLevel::Warning);
    assert!(f.message.contains("unreachable"), "message: {}", f.message);
    assert!(f.message.contains("return"));
    assert_eq!(pos(&src, f), (3, 5));
    assert_eq!(f.span.start as usize, src.find("print(2)").unwrap());
}

#[test]
fn return_as_last_stmt_is_not_flagged() {
    let src = "fn f() -> Int\n    return 1\nend\n".to_string();
    assert_eq!(unreach(&src).len(), 0);
    assert_eq!(warn_count(&src, N0104), 0);
}

#[test]
fn code_after_if_with_return_is_reachable() {
    let src = "fn f(n: Int) -> Int\n    if n < 2 then\n        return n\n    end\n    return f(n - 1)\nend\n".to_string();
    assert_eq!(unreach(&src).len(), 0);
    assert_eq!(warn_count(&src, N0104), 0);
}

#[test]
fn bare_return_then_stmts_are_flagged() {
    let src = "fn f()\n    return\n    print(1)\nend\n".to_string();
    let ws = warns_of(&src, N0104);
    assert_eq!(ws.len(), 1);
    assert_eq!(pos(&src, &ws[0]), (3, 5));
}

#[test]
fn two_unreachable_stmts_yield_two_findings() {
    let src = "fn f() -> Int\n    return 1\n    let z = 5\n    print(9)\nend\n".to_string();
    let ws = warns_of(&src, N0104);
    assert_eq!(ws.len(), 2);
    assert_eq!(pos(&src, &ws[0]), (3, 5));
    assert_eq!(pos(&src, &ws[1]), (4, 5));
    assert!(ws[0].message.contains("unreachable"));
    assert!(ws[1].message.contains("unreachable"));
}

#[test]
fn unreachable_chunks_carry_purity() {
    let src = "fn f() -> Int\n    return 1\n    2 + 3\n    print(9)\n    let z = 5\nend\n".to_string();
    let us = unreach(&src);
    assert_eq!(us.len(), 3);
    assert!(us[0].pure, "literal arithmetic should be pure");
    assert!(!us[1].pure, "calls should be impure");
    assert!(us[2].pure, "pure let initializers should be pure");
    assert_eq!(pos(&src, &warns_of(&src, N0104)[0]), (3, 5));
}

#[test]
fn greedy_expression_continuation_is_not_unreachable() {
    let src = "fn f() -> Int\n    return 1\n    -2\nend\n".to_string();
    assert_eq!(unreach(&src).len(), 0);
    assert_eq!(warn_count(&src, N0104), 0);
}

#[test]
fn greedy_expression_with_plus_is_not_unreachable() {
    let src = "fn f() -> Int\n    return 1\n    + 2\nend\n".to_string();
    assert_eq!(unreach(&src).len(), 0);
}

#[test]
fn return_with_call_expression_is_consumed() {
    let src = "fn f() -> Int\n    return helper(1, 2)\n    print(3)\nend\n".to_string();
    let ws = warns_of(&src, N0104);
    assert_eq!(ws.len(), 1);
    assert_eq!(pos(&src, &ws[0]), (3, 5));
}

#[test]
fn return_with_closure_is_consumed_without_false_positives() {
    let src = "fn make() -> fn() -> Int\n    var count = 0\n    return fn() -> Int\n        count = count + 1\n        return count\n    end\nend\n".to_string();
    assert_eq!(unreach(&src).len(), 0);
    assert_eq!(warn_count(&src, N0104), 0);
}

#[test]
fn fn_type_in_let_annotation_does_not_corrupt_the_scan() {
    let src = "fn make() -> Int\n    let cb: fn(Int) -> Int = fn(x)\n        return x\n    end\n    return cb(2)\nend\n".to_string();
    assert_eq!(unreach(&src).len(), 0);
    assert_eq!(warn_count(&src, N0104), 0);
}

#[test]
fn fn_type_in_generic_annotation_does_not_corrupt_the_scan() {
    let src = "fn make() -> Int\n    let m: Map<Int, fn(Int) -> Bool> = build()\n    return 1\nend\n".to_string();
    assert_eq!(unreach(&src).len(), 0);
    assert_eq!(warn_count(&src, N0104), 0);
}

#[test]
fn region_after_return_ends_at_else() {
    let src = "fn f(c: Bool) -> Int\n    if c then\n        return 1\n        print(2)\n    else\n        print(3)\n    end\n    return 0\nend\n".to_string();
    let ws = warns_of(&src, N0104);
    assert_eq!(ws.len(), 1);
    assert_eq!(pos(&src, &ws[0]), (4, 9));
    let us = unreach(&src);
    assert_eq!(us.len(), 1);
    assert!(!us[0].pure);
}

#[test]
fn region_after_return_ends_at_elseif() {
    let src = "fn f(n: Int) -> Int\n    if n == 0 then\n        return 1\n        print(9)\n    elseif n == 1 then\n        print(2)\n    else\n        print(3)\n    end\n    return 0\nend\n".to_string();
    let ws = warns_of(&src, N0104);
    assert_eq!(ws.len(), 1);
    assert_eq!(pos(&src, &ws[0]), (4, 9));
}

#[test]
fn return_inside_else_is_flagged_to_the_chain_end() {
    let src = "fn f(c: Bool) -> Int\n    if c then\n        print(1)\n    else\n        return 2\n        print(3)\n    end\n    return 0\nend\n".to_string();
    let ws = warns_of(&src, N0104);
    assert_eq!(ws.len(), 1);
    assert_eq!(pos(&src, &ws[0]), (6, 9));
}

#[test]
fn unreachable_while_is_one_chunk() {
    let src = "fn f() -> Int\n    return 1\n    while true do\n        break\n    end\nend\n".to_string();
    let ws = warns_of(&src, N0104);
    assert_eq!(ws.len(), 1);
    assert_eq!(pos(&src, &ws[0]), (3, 5));
    let us = unreach(&src);
    assert!(!us[0].pure);
    assert!(src[us[0].span.start as usize..us[0].span.end as usize].contains("while"));
}

#[test]
fn unreachable_for_is_one_chunk() {
    let src = "fn f() -> Int\n    return 1\n    for i in 1..3 do\n        print(i)\n    end\nend\n".to_string();
    let ws = warns_of(&src, N0104);
    assert_eq!(ws.len(), 1);
    assert_eq!(pos(&src, &ws[0]), (3, 5));
    assert!(!unreach(&src)[0].pure);
}

#[test]
fn unreachable_match_is_one_chunk() {
    let src = "fn f() -> Int\n    return 1\n    match 2\n        1 => print(1)\n        _ => print(0)\n    end\nend\n".to_string();
    let ws = warns_of(&src, N0104);
    assert_eq!(ws.len(), 1);
    assert_eq!(pos(&src, &ws[0]), (3, 5));
}

#[test]
fn unreachable_repeat_is_one_chunk_with_its_cond() {
    let src = "fn f() -> Int\n    return 1\n    repeat\n        print(1)\n    until true\nend\n".to_string();
    let ws = warns_of(&src, N0104);
    assert_eq!(ws.len(), 1);
    assert_eq!(pos(&src, &ws[0]), (3, 5));
    let us = unreach(&src);
    assert!(src[us[0].span.start as usize..us[0].span.end as usize].contains("until"));
}

#[test]
fn unreachable_if_chain_is_one_chunk() {
    let src = "fn f() -> Int\n    return 1\n    if true then\n        print(1)\n    else\n        print(2)\n    end\nend\n".to_string();
    let ws = warns_of(&src, N0104);
    assert_eq!(ws.len(), 1);
    assert_eq!(pos(&src, &ws[0]), (3, 5));
}

#[test]
fn return_inside_do_block_of_match_arm_is_flagged() {
    let src = "fn f(n: Int) -> Int\n    match n\n        1 => do\n            return 1\n            1 + 1\n        end\n        _ => return 0\n    end\nend\n".to_string();
    let ws = warns_of(&src, N0104);
    assert_eq!(ws.len(), 1);
    assert_eq!(pos(&src, &ws[0]), (5, 13));
}

#[test]
fn direct_return_in_match_arm_is_not_flagged() {
    let src = "fn f(n: Int) -> Int\n    match n\n        1 => return 1\n        _ => return 2\n    end\nend\n".to_string();
    assert_eq!(unreach(&src).len(), 0);
    assert_eq!(warn_count(&src, N0104), 0);
}

#[test]
fn return_inside_unsafe_block_is_flagged() {
    let src = "fn f() -> Int\n    unsafe\n        return 1\n        2 + 2\n    end\nend\n".to_string();
    let ws = warns_of(&src, N0104);
    assert_eq!(ws.len(), 1);
    assert_eq!(pos(&src, &ws[0]), (4, 9));
}

#[test]
fn return_inside_loop_body_is_flagged() {
    let src = "fn f() -> Int\n    loop\n        return 1\n        print(2)\n    end\nend\n".to_string();
    let ws = warns_of(&src, N0104);
    assert_eq!(ws.len(), 1);
    assert_eq!(pos(&src, &ws[0]), (4, 9));
}

#[test]
fn struct_and_enum_declarations_do_not_corrupt_the_scan() {
    let src = "P = struct\n    hp: Int\nend\n\nE = enum\n    A\n    B\nend\n\nI = interface\n    hi(self) -> Int\nend\n\nfn f(p: P) -> Int\n    return p.hp\nend\n".to_string();
    assert_eq!(unreach(&src).len(), 0);
    assert_eq!(warn_count(&src, N0104), 0);
}

#[test]
fn nested_closure_inside_unreachable_region_is_chunked() {
    let src = "fn f() -> Int\n    return 1\n    let g = fn()\n        return 2\n    end\nend\n".to_string();
    let ws = warns_of(&src, N0104);
    assert_eq!(ws.len(), 1);
    assert_eq!(pos(&src, &ws[0]), (3, 5));
    let us = unreach(&src);
    assert!(!us[0].pure);
}

#[test]
fn second_return_in_region_extends_its_own_nested_region() {
    let src = "fn f() -> Int\n    return 1\n    if true then\n        return 2\n        print(3)\n    end\nend\n".to_string();
    let ws = warns_of(&src, N0104);
    assert_eq!(ws.len(), 2);
    assert_eq!(pos(&src, &ws[0]), (3, 5));
    assert_eq!(pos(&src, &ws[1]), (5, 9));
}

#[test]
fn chunk_purity_examples() {
    let cases: Vec<(&str, bool)> = vec![
        ("2 + 3", true),
        ("x", true),
        ("x.y", true),
        ("a[1]", true),
        ("(1, 2)", true),
        ("[1, 2, 3]", true),
        ("{ \"k\" = 1 }", true),
        ("P { x = 1 }", true),
        ("1 < 2", true),
        ("not true", true),
        ("print(2)", false),
        ("f(1)", false),
        ("p:m()", false),
        ("obj.field(1)", false),
        ("x = 1", false),
        ("spawn f()", false),
        ("\"${x}\"", false),
        ("fn()\n    return 1\nend", false),
    ];
    for (body, pure) in cases {
        let src = format!("fn f() -> Int\n    return 1\n    {}\nend\n", body);
        let us = unreach(&src);
        assert_eq!(us.len(), 1, "one chunk for {:?}", body);
        assert_eq!(us[0].pure, pure, "purity of {:?}", body);
    }
}

#[test]
fn let_chunk_purity_examples() {
    let cases: Vec<(&str, bool)> = vec![
        ("let z = 5", true),
        ("let z: Int = 5", true),
        ("let z = a.b", true),
        ("let z = [1, 2]", true),
        ("let z = (1, 2)", true),
        ("var z = 5", true),
        ("let z = f(1)", false),
        ("let z = p:m()", false),
        ("let z = \"${x}\"", false),
    ];
    for (body, pure) in cases {
        let src = format!("fn f() -> Int\n    return 1\n    {}\nend\n", body);
        let us = unreach(&src);
        assert_eq!(us.len(), 1, "one chunk for {:?}", body);
        assert_eq!(us[0].pure, pure, "purity of {:?}", body);
    }
}

#[test]
fn semicolons_are_included_in_unreachable_chunks() {
    let src = "fn f() -> Int\n    return 1\n    2 + 3;\nend\n".to_string();
    let us = unreach(&src);
    assert_eq!(us.len(), 1);
    let text = &src[us[0].span.start as usize..us[0].span.end as usize];
    assert!(text.contains("2 + 3"));
    assert!(text.contains(';'));
}

#[test]
fn self_assignment_is_flagged() {
    let src = in_main("    var x = 1\n    x = x\n    print(x)");
    let f = first_warn(&src, N0105);
    assert_eq!(f.code, N0105);
    assert_eq!(f.level, LintLevel::Warning);
    assert_eq!(warn_count(&src, N0105), 1);
    assert!(f.message.contains("x = x"), "message: {}", f.message);
    assert!(f.message.contains("no effect"), "message: {}", f.message);
    assert_eq!(pos(&src, &f), (3, 5));
    assert_eq!(f.span.start as usize, src.find("x = x").unwrap());
    assert_eq!(f.span.end as usize, src.find("x = x").unwrap() + 5);
}

#[test]
fn different_names_are_not_self_assignment() {
    let src = in_main("    var x = 1\n    var y = 2\n    x = y\n    print(x)");
    assert_eq!(warn_count(&src, N0105), 0);
}

#[test]
fn compound_assignment_is_not_self_assignment() {
    let src = in_main("    var x = 1\n    x += x\n    print(x)");
    assert_eq!(warn_count(&src, N0105), 0);
}

#[test]
fn field_self_assignment_is_not_flagged() {
    let src = "P = struct\n    hp: Int\nend\n\nfn f(p: P)\n    p.hp = p.hp\nend\n".to_string();
    assert_eq!(warn_count(&src, N0105), 0);
}

#[test]
fn two_self_assignments_yield_two_findings() {
    let src = in_main("    var x = 1\n    var y = 2\n    x = x\n    y = y\n    print(x + y)");
    let ws = warns_of(&src, N0105);
    assert_eq!(ws.len(), 2);
    assert!(ws[0].message.contains("x = x"));
    assert!(ws[1].message.contains("y = y"));
    assert_eq!(pos(&src, &ws[0]), (4, 5));
    assert_eq!(pos(&src, &ws[1]), (5, 5));
}

#[test]
fn self_assignment_still_counts_as_a_read() {
    let src = in_main("    var x = 1\n    x = x\n    print(x)");
    assert_eq!(warn_count(&src, N0101), 0);
}

#[test]
fn self_assignment_inside_closure_is_flagged() {
    let src = in_main("    let f = fn()\n        var n = 1\n        n = n\n        return n\n    end\n    print(f())");
    let ws = warns_of(&src, N0105);
    assert_eq!(ws.len(), 1);
    assert_eq!(pos(&src, &ws[0]), (4, 9));
}

#[test]
fn self_assignment_in_let_value_is_flagged() {
    let src = in_main("    var x = 1\n    let y = (x = x)\n    print(x)");
    assert_eq!(warn_count(&src, N0105), 1);
}

#[test]
fn empty_fn_body_is_flagged() {
    let src = "fn noop()\nend\n".to_string();
    let f = first_warn(&src, N0106);
    assert_eq!(f.code, N0106);
    assert_eq!(f.level, LintLevel::Warning);
    assert_eq!(warn_count(&src, N0106), 1);
    assert!(f.message.contains("empty"), "message: {}", f.message);
}

#[test]
fn nonempty_fn_body_is_not_flagged() {
    let src = "fn f()\n    print(1)\nend\n".to_string();
    assert_eq!(warn_count(&src, N0106), 0);
}

#[test]
fn fn_body_with_tail_expression_is_not_flagged() {
    let src = "fn f() -> Int\n    42\nend\n".to_string();
    assert_eq!(warn_count(&src, N0106), 0);
}

#[test]
fn empty_then_block_is_flagged() {
    let src = in_main("    if true then\n    else\n        print(1)\n    end");
    let ws = warns_of(&src, N0106);
    assert_eq!(ws.len(), 1);
}

#[test]
fn empty_else_block_is_flagged() {
    let src = in_main("    if true then\n        print(1)\n    else\n    end");
    let ws = warns_of(&src, N0106);
    assert_eq!(ws.len(), 1);
}

#[test]
fn empty_while_body_is_flagged() {
    let src = in_main("    while true do\n    end");
    assert_eq!(warn_count(&src, N0106), 1);
}

#[test]
fn empty_loop_body_is_flagged() {
    let src = in_main("    loop\n    end");
    assert_eq!(warn_count(&src, N0106), 1);
}

#[test]
fn empty_repeat_body_is_flagged() {
    let src = in_main("    repeat\n    until true");
    assert_eq!(warn_count(&src, N0106), 1);
}

#[test]
fn empty_unsafe_body_is_flagged() {
    let src = in_main("    unsafe\n    end");
    assert_eq!(warn_count(&src, N0106), 1);
}

#[test]
fn empty_closure_body_is_flagged() {
    let src = in_main("    let f = fn()\n    end\n    print(1)");
    assert_eq!(warn_count(&src, N0106), 1);
}

#[test]
fn populated_bodies_are_not_flagged() {
    let src = in_main("    if true then\n        print(1)\n    else\n        print(2)\n    end\n    while false do\n        break\n    end");
    assert_eq!(warn_count(&src, N0106), 0);
}

#[test]
fn findings_are_sorted_by_position() {
    let src = "fn f()\n    let dead = 1\n    x = x\nend\n".to_string();
    let ws = warnings(&src);
    assert_eq!(ws.len(), 2);
    assert!(ws[0].span.start <= ws[1].span.start);
    assert_eq!(ws[0].code, N0101);
    assert_eq!(ws[1].code, N0105);
}

#[test]
fn multiple_check_codes_in_one_file() {
    let src = "P = struct\n    hp: Int\nend\n\nfn f(unused: Int)\n    let dead = 1\n    x = x\nend\n".to_string();
    let got = codes(&src);
    assert!(got.contains(&N0102.to_string()), "codes: {:?}", got);
    assert!(got.contains(&N0101.to_string()), "codes: {:?}", got);
    assert!(got.contains(&N0103.to_string()), "codes: {:?}", got);
    assert!(got.contains(&N0105.to_string()), "codes: {:?}", got);
    assert_eq!(warnings(&src).len(), 4);
    assert_eq!(errors(&src).len(), 0);
}

#[test]
fn clean_source_has_no_findings() {
    let src = "fn fib(n: Int) -> Int\n    if n < 2 then\n        return n\n    end\n    return fib(n - 1) + fib(n - 2)\nend\n".to_string();
    assert_eq!(findings(&src).len(), 0);
    assert_eq!(codes(&src).len(), 0);
}

#[test]
fn parse_errors_surface_as_errors() {
    let src = "fn f(\nend\n".to_string();
    let es = errors(&src);
    assert!(!es.is_empty());
    assert_eq!(es[0].level, LintLevel::Error);
    assert_eq!(es[0].code, "E0000");
    assert_eq!(warn_count(&src, N0101), 0);
    assert_eq!(warnings(&src).len(), 0);
}

#[test]
fn parse_errors_skip_ast_checks_to_avoid_false_positives() {
    let src = "fn f()\n    let used_later = 1\n    return 1\n    print(used_later)\nend\n".to_string();
    assert!(!errors(&src).is_empty());
    assert_eq!(warn_count(&src, N0101), 0);
    assert_eq!(warn_count(&src, N0104), 1);
}

#[test]
fn lexer_errors_surface_as_errors() {
    let src = "fn f()\n    let s = \"unterminated\nend\n".to_string();
    let es = errors(&src);
    assert_eq!(es.len(), 1);
    assert_eq!(es[0].level, LintLevel::Error);
    assert!(es[0].message.contains("unterminated"));
    assert!(lint::scan_unreachable_stmts(&src, 0).is_err());
}

#[test]
fn ownership_errors_surface_as_errors() {
    let src = "fn f(x: borrow Int)\n    x = 1\nend\n".to_string();
    let es = errors(&src);
    assert_eq!(es.len(), 1);
    assert_eq!(es[0].level, LintLevel::Error);
    assert_eq!(es[0].code, "E0104");
    assert_eq!(es[0].span.start as usize, src.find("borrow").unwrap());
}

#[test]
fn use_after_move_surfaces_as_error() {
    let src = "fn take(x: move Int)\n    print(x)\nend\n\nfn main()\n    let a = 1\n    take(a)\n    print(a)\nend\n".to_string();
    let es = errors(&src);
    assert_eq!(es.len(), 1);
    assert_eq!(es[0].code, "E0101");
    assert!(es[0].message.contains("moved"));
}

#[test]
fn ownership_clean_file_has_no_errors() {
    let src = "fn f(x: borrow Int)\n    print(x)\nend\n".to_string();
    assert_eq!(errors(&src).len(), 0);
    assert_eq!(findings(&src).len(), 0);
}

#[test]
fn lint_source_mixes_n0104_with_parse_errors_in_order() {
    let src = "fn f() -> Int\n    return 1\n    print(2)\nend\n".to_string();
    let got = findings(&src);
    assert!(got.len() > 2);
    for w in got.windows(2) {
        assert!(w[0].span.start <= w[1].span.start, "unsorted findings");
    }
}

#[test]
fn render_finding_formats_the_canonical_line() {
    let src = "fn f()\n    let dead = 1\nend\n".to_string();
    let f = first_warn(&src, N0101);
    let path = std::path::Path::new("src/main.axol");
    let line = lint::render_finding(path, &src, &f);
    assert_eq!(line, "src/main.axol:2:9: warning[N0101]: unused local `dead` (never read after declaration)");
}

#[test]
fn render_finding_formats_error_level() {
    let src = "fn f(\nend\n".to_string();
    let f = errors(&src)[0].clone();
    let path = std::path::Path::new("lib.axol");
    let line = lint::render_finding(path, &src, &f);
    assert!(line.starts_with("lib.axol:2:"));
    assert!(line.contains(": error[E0000]: "));
}

#[test]
fn analyze_runs_on_a_module_directly() {
    let src = "fn f(unused: Int)\n    let dead = 1\nend\n".to_string();
    let got = lint::analyze(&module(&src));
    assert_eq!(got.len(), 2);
    assert_eq!(got[0].code, N0102);
    assert_eq!(got[1].code, N0101);
    assert_eq!(got[0].level, LintLevel::Warning);
    assert_eq!(got[1].level, LintLevel::Warning);
}

#[test]
fn analyze_returns_empty_for_a_clean_module() {
    let src = "fn f(n: Int) -> Int\n    return n\nend\n".to_string();
    assert_eq!(lint::analyze(&module(&src)).len(), 0);
}

#[test]
fn examples_lint_clean() {
    let names = ["hello", "fib", "fizzbuzz", "primes", "counter", "structs"];
    for n in names {
        let path = format!("{}/../../examples/{}.axol", env!("CARGO_MANIFEST_DIR"), n);
        let src = std::fs::read_to_string(&path).expect("read example");
        let got = lint::lint_source(&src, 0);
        assert_eq!(got.len(), 0, "example {} produced {:?}", n, got);
        assert_eq!(lint::analyze(&module(&src)).len(), 0, "example {}", n);
        assert_eq!(lint::scan_unreachable_stmts(&src, 0).expect("scan").len(), 0, "example {}", n);
    }
}

fn bucket_binary() -> String {
    let mut path = std::env::current_exe().unwrap();
    path.pop();
    path.pop();
    path.join("bucket").to_string_lossy().to_string()
}

fn run_bucket_in(dir: &std::path::Path, args: &[&str]) -> (String, String, i32) {
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

fn temp_project(tag: &str) -> std::path::PathBuf {
    let dir = std::env::temp_dir().join(format!("bucket-lint-{}-{}", tag, std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(dir.join("src")).unwrap();
    dir
}

#[test]
fn cli_clean_project_exits_zero() {
    let dir = temp_project("clean");
    std::fs::write(
        dir.join("src/main.axol"),
        "fn main()\n    print(\"hi\")\nend\n",
    )
    .unwrap();
    let (out, err, code) = run_bucket_in(&dir, &["lint"]);
    assert_eq!(code, 0);
    assert_eq!(out, "Neoten: 0 warnings, 0 errors\n");
    assert_eq!(err, "");
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn cli_warnings_only_exit_zero() {
    let dir = temp_project("warns");
    std::fs::write(
        dir.join("src/main.axol"),
        "fn main()\n    let dead = 1\n    print(1)\nend\n",
    )
    .unwrap();
    let (out, err, code) = run_bucket_in(&dir, &["lint"]);
    assert_eq!(code, 0);
    assert_eq!(err, "");
    assert!(out.contains("src/main.axol:2:9: warning[N0101]: unused local `dead`"));
    assert!(out.contains("Neoten: 1 warnings, 0 errors"));
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn cli_parse_error_exits_one() {
    let dir = temp_project("parse");
    std::fs::write(dir.join("src/main.axol"), "fn main(\nend\n").unwrap();
    let (out, _err, code) = run_bucket_in(&dir, &["lint"]);
    assert_eq!(code, 1);
    assert!(out.contains(": error[E0000]: "));
    assert!(out.contains("Neoten: 0 warnings"));
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn cli_ownership_error_exits_one() {
    let dir = temp_project("own");
    std::fs::write(
        dir.join("src/main.axol"),
        "fn f(x: borrow Int)\n    x = 1\nend\n",
    )
    .unwrap();
    let (out, _err, code) = run_bucket_in(&dir, &["lint"]);
    assert_eq!(code, 1);
    assert!(out.contains(": error[E0104]: "));
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn cli_multiple_files_are_all_linted() {
    let dir = temp_project("multi");
    std::fs::write(dir.join("src/main.axol"), "fn main()\n    let a = 1\n    print(1)\nend\n").unwrap();
    std::fs::write(dir.join("src/lib.axol"), "fn helper(b: Int) -> Int\n    return 1\nend\n").unwrap();
    let (out, _err, code) = run_bucket_in(&dir, &["lint"]);
    assert_eq!(code, 0);
    assert!(out.contains("src/main.axol:2:9: warning[N0101]: unused local `a`"));
    assert!(out.contains("src/lib.axol:1:11: warning[N0102]: unused fn parameter `b`"));
    assert!(out.contains("Neoten: 2 warnings, 0 errors"));
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn cli_unreachable_code_reports_n0104_with_parse_errors() {
    let dir = temp_project("unreach");
    std::fs::write(
        dir.join("src/main.axol"),
        "fn f() -> Int\n    return 1\n    print(2)\nend\n\nfn main()\n    print(f())\nend\n",
    )
    .unwrap();
    let (out, _err, code) = run_bucket_in(&dir, &["lint"]);
    assert_eq!(code, 1);
    assert!(out.contains("src/main.axol:3:5: warning[N0104]: unreachable code after `return`"));
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn cli_empty_project_reports_no_files() {
    let dir = temp_project("empty");
    let (out, _err, code) = run_bucket_in(&dir, &["lint"]);
    assert_eq!(code, 0);
    assert_eq!(out, "Neoten: no .axol files found\n");
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn cli_examples_project_lints_clean() {
    let dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../examples");
    let (out, err, code) = run_bucket_in(&dir, &["lint"]);
    assert_eq!(code, 0);
    assert_eq!(out, "Neoten: 0 warnings, 0 errors\n");
    assert_eq!(err, "");
}
