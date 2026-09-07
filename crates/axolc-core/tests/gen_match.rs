// Owner: PascalElixir / axolrs (GitHub org)
// File: Auto-generated tests for match expressions.

use axolc_core::interpret;

#[test]
fn match_simple_0() {
    let (out, diags) = interpret(r#"fn main() match 0 0 => print("zero") 1 => print("one") _ => print("two") end end"#, 0);
    assert!(!diags.has_errors(), "match_simple_0: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"zero
");
}

#[test]
fn match_simple_1() {
    let (out, diags) = interpret(r#"fn main() match 1 0 => print("zero") 1 => print("one") _ => print("two") end end"#, 0);
    assert!(!diags.has_errors(), "match_simple_1: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"one
");
}

#[test]
fn match_simple_2() {
    let (out, diags) = interpret(r#"fn main() match 2 0 => print("zero") 1 => print("one") _ => print("two") end end"#, 0);
    assert!(!diags.has_errors(), "match_simple_2: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"two
");
}

#[test]
fn match_simple_3() {
    let (out, diags) = interpret(r#"fn main() match 0 0 => print("zero") 1 => print("one") _ => print("two") end end"#, 0);
    assert!(!diags.has_errors(), "match_simple_3: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"zero
");
}

#[test]
fn match_simple_4() {
    let (out, diags) = interpret(r#"fn main() match 1 0 => print("zero") 1 => print("one") _ => print("two") end end"#, 0);
    assert!(!diags.has_errors(), "match_simple_4: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"one
");
}

#[test]
fn match_simple_5() {
    let (out, diags) = interpret(r#"fn main() match 2 0 => print("zero") 1 => print("one") _ => print("two") end end"#, 0);
    assert!(!diags.has_errors(), "match_simple_5: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"two
");
}

#[test]
fn match_simple_6() {
    let (out, diags) = interpret(r#"fn main() match 0 0 => print("zero") 1 => print("one") _ => print("two") end end"#, 0);
    assert!(!diags.has_errors(), "match_simple_6: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"zero
");
}

#[test]
fn match_simple_7() {
    let (out, diags) = interpret(r#"fn main() match 1 0 => print("zero") 1 => print("one") _ => print("two") end end"#, 0);
    assert!(!diags.has_errors(), "match_simple_7: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"one
");
}

#[test]
fn match_simple_8() {
    let (out, diags) = interpret(r#"fn main() match 2 0 => print("zero") 1 => print("one") _ => print("two") end end"#, 0);
    assert!(!diags.has_errors(), "match_simple_8: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"two
");
}

#[test]
fn match_simple_9() {
    let (out, diags) = interpret(r#"fn main() match 0 0 => print("zero") 1 => print("one") _ => print("two") end end"#, 0);
    assert!(!diags.has_errors(), "match_simple_9: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"zero
");
}

#[test]
fn match_simple_10() {
    let (out, diags) = interpret(r#"fn main() match 1 0 => print("zero") 1 => print("one") _ => print("two") end end"#, 0);
    assert!(!diags.has_errors(), "match_simple_10: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"one
");
}

#[test]
fn match_simple_11() {
    let (out, diags) = interpret(r#"fn main() match 2 0 => print("zero") 1 => print("one") _ => print("two") end end"#, 0);
    assert!(!diags.has_errors(), "match_simple_11: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"two
");
}

#[test]
fn match_simple_12() {
    let (out, diags) = interpret(r#"fn main() match 0 0 => print("zero") 1 => print("one") _ => print("two") end end"#, 0);
    assert!(!diags.has_errors(), "match_simple_12: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"zero
");
}

#[test]
fn match_simple_13() {
    let (out, diags) = interpret(r#"fn main() match 1 0 => print("zero") 1 => print("one") _ => print("two") end end"#, 0);
    assert!(!diags.has_errors(), "match_simple_13: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"one
");
}

#[test]
fn match_simple_14() {
    let (out, diags) = interpret(r#"fn main() match 2 0 => print("zero") 1 => print("one") _ => print("two") end end"#, 0);
    assert!(!diags.has_errors(), "match_simple_14: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"two
");
}

#[test]
fn match_simple_15() {
    let (out, diags) = interpret(r#"fn main() match 0 0 => print("zero") 1 => print("one") _ => print("two") end end"#, 0);
    assert!(!diags.has_errors(), "match_simple_15: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"zero
");
}

#[test]
fn match_simple_16() {
    let (out, diags) = interpret(r#"fn main() match 1 0 => print("zero") 1 => print("one") _ => print("two") end end"#, 0);
    assert!(!diags.has_errors(), "match_simple_16: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"one
");
}

#[test]
fn match_simple_17() {
    let (out, diags) = interpret(r#"fn main() match 2 0 => print("zero") 1 => print("one") _ => print("two") end end"#, 0);
    assert!(!diags.has_errors(), "match_simple_17: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"two
");
}

#[test]
fn match_simple_18() {
    let (out, diags) = interpret(r#"fn main() match 0 0 => print("zero") 1 => print("one") _ => print("two") end end"#, 0);
    assert!(!diags.has_errors(), "match_simple_18: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"zero
");
}

#[test]
fn match_simple_19() {
    let (out, diags) = interpret(r#"fn main() match 1 0 => print("zero") 1 => print("one") _ => print("two") end end"#, 0);
    assert!(!diags.has_errors(), "match_simple_19: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"one
");
}

#[test]
fn match_simple_20() {
    let (out, diags) = interpret(r#"fn main() match 2 0 => print("zero") 1 => print("one") _ => print("two") end end"#, 0);
    assert!(!diags.has_errors(), "match_simple_20: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"two
");
}

#[test]
fn match_simple_21() {
    let (out, diags) = interpret(r#"fn main() match 0 0 => print("zero") 1 => print("one") _ => print("two") end end"#, 0);
    assert!(!diags.has_errors(), "match_simple_21: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"zero
");
}

#[test]
fn match_simple_22() {
    let (out, diags) = interpret(r#"fn main() match 1 0 => print("zero") 1 => print("one") _ => print("two") end end"#, 0);
    assert!(!diags.has_errors(), "match_simple_22: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"one
");
}

#[test]
fn match_simple_23() {
    let (out, diags) = interpret(r#"fn main() match 2 0 => print("zero") 1 => print("one") _ => print("two") end end"#, 0);
    assert!(!diags.has_errors(), "match_simple_23: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"two
");
}

#[test]
fn match_simple_24() {
    let (out, diags) = interpret(r#"fn main() match 0 0 => print("zero") 1 => print("one") _ => print("two") end end"#, 0);
    assert!(!diags.has_errors(), "match_simple_24: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"zero
");
}

#[test]
fn match_simple_25() {
    let (out, diags) = interpret(r#"fn main() match 1 0 => print("zero") 1 => print("one") _ => print("two") end end"#, 0);
    assert!(!diags.has_errors(), "match_simple_25: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"one
");
}

#[test]
fn match_simple_26() {
    let (out, diags) = interpret(r#"fn main() match 2 0 => print("zero") 1 => print("one") _ => print("two") end end"#, 0);
    assert!(!diags.has_errors(), "match_simple_26: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"two
");
}

#[test]
fn match_simple_27() {
    let (out, diags) = interpret(r#"fn main() match 0 0 => print("zero") 1 => print("one") _ => print("two") end end"#, 0);
    assert!(!diags.has_errors(), "match_simple_27: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"zero
");
}

#[test]
fn match_simple_28() {
    let (out, diags) = interpret(r#"fn main() match 1 0 => print("zero") 1 => print("one") _ => print("two") end end"#, 0);
    assert!(!diags.has_errors(), "match_simple_28: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"one
");
}

#[test]
fn match_simple_29() {
    let (out, diags) = interpret(r#"fn main() match 2 0 => print("zero") 1 => print("one") _ => print("two") end end"#, 0);
    assert!(!diags.has_errors(), "match_simple_29: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"two
");
}

#[test]
fn match_simple_30() {
    let (out, diags) = interpret(r#"fn main() match 0 0 => print("zero") 1 => print("one") _ => print("two") end end"#, 0);
    assert!(!diags.has_errors(), "match_simple_30: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"zero
");
}

#[test]
fn match_simple_31() {
    let (out, diags) = interpret(r#"fn main() match 1 0 => print("zero") 1 => print("one") _ => print("two") end end"#, 0);
    assert!(!diags.has_errors(), "match_simple_31: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"one
");
}

#[test]
fn match_simple_32() {
    let (out, diags) = interpret(r#"fn main() match 2 0 => print("zero") 1 => print("one") _ => print("two") end end"#, 0);
    assert!(!diags.has_errors(), "match_simple_32: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"two
");
}

#[test]
fn match_simple_33() {
    let (out, diags) = interpret(r#"fn main() match 0 0 => print("zero") 1 => print("one") _ => print("two") end end"#, 0);
    assert!(!diags.has_errors(), "match_simple_33: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"zero
");
}

#[test]
fn match_simple_34() {
    let (out, diags) = interpret(r#"fn main() match 1 0 => print("zero") 1 => print("one") _ => print("two") end end"#, 0);
    assert!(!diags.has_errors(), "match_simple_34: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"one
");
}

#[test]
fn match_simple_35() {
    let (out, diags) = interpret(r#"fn main() match 2 0 => print("zero") 1 => print("one") _ => print("two") end end"#, 0);
    assert!(!diags.has_errors(), "match_simple_35: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"two
");
}

#[test]
fn match_simple_36() {
    let (out, diags) = interpret(r#"fn main() match 0 0 => print("zero") 1 => print("one") _ => print("two") end end"#, 0);
    assert!(!diags.has_errors(), "match_simple_36: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"zero
");
}

#[test]
fn match_simple_37() {
    let (out, diags) = interpret(r#"fn main() match 1 0 => print("zero") 1 => print("one") _ => print("two") end end"#, 0);
    assert!(!diags.has_errors(), "match_simple_37: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"one
");
}

#[test]
fn match_simple_38() {
    let (out, diags) = interpret(r#"fn main() match 2 0 => print("zero") 1 => print("one") _ => print("two") end end"#, 0);
    assert!(!diags.has_errors(), "match_simple_38: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"two
");
}

#[test]
fn match_simple_39() {
    let (out, diags) = interpret(r#"fn main() match 0 0 => print("zero") 1 => print("one") _ => print("two") end end"#, 0);
    assert!(!diags.has_errors(), "match_simple_39: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"zero
");
}

#[test]
fn match_simple_40() {
    let (out, diags) = interpret(r#"fn main() match 1 0 => print("zero") 1 => print("one") _ => print("two") end end"#, 0);
    assert!(!diags.has_errors(), "match_simple_40: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"one
");
}

#[test]
fn match_simple_41() {
    let (out, diags) = interpret(r#"fn main() match 2 0 => print("zero") 1 => print("one") _ => print("two") end end"#, 0);
    assert!(!diags.has_errors(), "match_simple_41: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"two
");
}

#[test]
fn match_simple_42() {
    let (out, diags) = interpret(r#"fn main() match 0 0 => print("zero") 1 => print("one") _ => print("two") end end"#, 0);
    assert!(!diags.has_errors(), "match_simple_42: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"zero
");
}

#[test]
fn match_simple_43() {
    let (out, diags) = interpret(r#"fn main() match 1 0 => print("zero") 1 => print("one") _ => print("two") end end"#, 0);
    assert!(!diags.has_errors(), "match_simple_43: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"one
");
}

#[test]
fn match_simple_44() {
    let (out, diags) = interpret(r#"fn main() match 2 0 => print("zero") 1 => print("one") _ => print("two") end end"#, 0);
    assert!(!diags.has_errors(), "match_simple_44: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"two
");
}

#[test]
fn match_simple_45() {
    let (out, diags) = interpret(r#"fn main() match 0 0 => print("zero") 1 => print("one") _ => print("two") end end"#, 0);
    assert!(!diags.has_errors(), "match_simple_45: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"zero
");
}

#[test]
fn match_simple_46() {
    let (out, diags) = interpret(r#"fn main() match 1 0 => print("zero") 1 => print("one") _ => print("two") end end"#, 0);
    assert!(!diags.has_errors(), "match_simple_46: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"one
");
}

#[test]
fn match_simple_47() {
    let (out, diags) = interpret(r#"fn main() match 2 0 => print("zero") 1 => print("one") _ => print("two") end end"#, 0);
    assert!(!diags.has_errors(), "match_simple_47: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"two
");
}

#[test]
fn match_simple_48() {
    let (out, diags) = interpret(r#"fn main() match 0 0 => print("zero") 1 => print("one") _ => print("two") end end"#, 0);
    assert!(!diags.has_errors(), "match_simple_48: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"zero
");
}

#[test]
fn match_simple_49() {
    let (out, diags) = interpret(r#"fn main() match 1 0 => print("zero") 1 => print("one") _ => print("two") end end"#, 0);
    assert!(!diags.has_errors(), "match_simple_49: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"one
");
}

