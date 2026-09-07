// Owner: PascalElixir / axolrs (GitHub org)
// File: Auto-generated tests for if/else control flow.

use axolc_core::interpret;

#[test]
fn if_else_0() {
    let (out, diags) = interpret(r#"fn main() if true then print("yes") else print("other") end end"#, 0);
    assert!(!diags.has_errors(), "if_else_0: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"yes
");
}

#[test]
fn if_else_1() {
    let (out, diags) = interpret(r#"fn main() if false then print("yes") else print("other") end end"#, 0);
    assert!(!diags.has_errors(), "if_else_1: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"other
");
}

#[test]
fn if_else_2() {
    let (out, diags) = interpret(r#"fn main() if true then print("yes") else print("other") end end"#, 0);
    assert!(!diags.has_errors(), "if_else_2: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"yes
");
}

#[test]
fn if_else_3() {
    let (out, diags) = interpret(r#"fn main() if false then print("yes") else print("other") end end"#, 0);
    assert!(!diags.has_errors(), "if_else_3: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"other
");
}

#[test]
fn if_else_4() {
    let (out, diags) = interpret(r#"fn main() if true then print("yes") else print("other") end end"#, 0);
    assert!(!diags.has_errors(), "if_else_4: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"yes
");
}

#[test]
fn if_else_5() {
    let (out, diags) = interpret(r#"fn main() if false then print("yes") else print("other") end end"#, 0);
    assert!(!diags.has_errors(), "if_else_5: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"other
");
}

#[test]
fn if_else_6() {
    let (out, diags) = interpret(r#"fn main() if true then print("yes") else print("other") end end"#, 0);
    assert!(!diags.has_errors(), "if_else_6: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"yes
");
}

#[test]
fn if_else_7() {
    let (out, diags) = interpret(r#"fn main() if false then print("yes") else print("other") end end"#, 0);
    assert!(!diags.has_errors(), "if_else_7: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"other
");
}

#[test]
fn if_else_8() {
    let (out, diags) = interpret(r#"fn main() if true then print("yes") else print("other") end end"#, 0);
    assert!(!diags.has_errors(), "if_else_8: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"yes
");
}

#[test]
fn if_else_9() {
    let (out, diags) = interpret(r#"fn main() if false then print("yes") else print("other") end end"#, 0);
    assert!(!diags.has_errors(), "if_else_9: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"other
");
}

#[test]
fn if_else_10() {
    let (out, diags) = interpret(r#"fn main() if true then print("yes") else print("other") end end"#, 0);
    assert!(!diags.has_errors(), "if_else_10: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"yes
");
}

#[test]
fn if_else_11() {
    let (out, diags) = interpret(r#"fn main() if false then print("yes") else print("other") end end"#, 0);
    assert!(!diags.has_errors(), "if_else_11: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"other
");
}

#[test]
fn if_else_12() {
    let (out, diags) = interpret(r#"fn main() if true then print("yes") else print("other") end end"#, 0);
    assert!(!diags.has_errors(), "if_else_12: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"yes
");
}

#[test]
fn if_else_13() {
    let (out, diags) = interpret(r#"fn main() if false then print("yes") else print("other") end end"#, 0);
    assert!(!diags.has_errors(), "if_else_13: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"other
");
}

#[test]
fn if_else_14() {
    let (out, diags) = interpret(r#"fn main() if true then print("yes") else print("other") end end"#, 0);
    assert!(!diags.has_errors(), "if_else_14: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"yes
");
}

#[test]
fn if_else_15() {
    let (out, diags) = interpret(r#"fn main() if false then print("yes") else print("other") end end"#, 0);
    assert!(!diags.has_errors(), "if_else_15: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"other
");
}

#[test]
fn if_else_16() {
    let (out, diags) = interpret(r#"fn main() if true then print("yes") else print("other") end end"#, 0);
    assert!(!diags.has_errors(), "if_else_16: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"yes
");
}

#[test]
fn if_else_17() {
    let (out, diags) = interpret(r#"fn main() if false then print("yes") else print("other") end end"#, 0);
    assert!(!diags.has_errors(), "if_else_17: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"other
");
}

#[test]
fn if_else_18() {
    let (out, diags) = interpret(r#"fn main() if true then print("yes") else print("other") end end"#, 0);
    assert!(!diags.has_errors(), "if_else_18: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"yes
");
}

#[test]
fn if_else_19() {
    let (out, diags) = interpret(r#"fn main() if false then print("yes") else print("other") end end"#, 0);
    assert!(!diags.has_errors(), "if_else_19: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"other
");
}

#[test]
fn if_else_20() {
    let (out, diags) = interpret(r#"fn main() if true then print("yes") else print("other") end end"#, 0);
    assert!(!diags.has_errors(), "if_else_20: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"yes
");
}

#[test]
fn if_else_21() {
    let (out, diags) = interpret(r#"fn main() if false then print("yes") else print("other") end end"#, 0);
    assert!(!diags.has_errors(), "if_else_21: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"other
");
}

#[test]
fn if_else_22() {
    let (out, diags) = interpret(r#"fn main() if true then print("yes") else print("other") end end"#, 0);
    assert!(!diags.has_errors(), "if_else_22: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"yes
");
}

#[test]
fn if_else_23() {
    let (out, diags) = interpret(r#"fn main() if false then print("yes") else print("other") end end"#, 0);
    assert!(!diags.has_errors(), "if_else_23: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"other
");
}

#[test]
fn if_else_24() {
    let (out, diags) = interpret(r#"fn main() if true then print("yes") else print("other") end end"#, 0);
    assert!(!diags.has_errors(), "if_else_24: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"yes
");
}

#[test]
fn if_else_25() {
    let (out, diags) = interpret(r#"fn main() if false then print("yes") else print("other") end end"#, 0);
    assert!(!diags.has_errors(), "if_else_25: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"other
");
}

#[test]
fn if_else_26() {
    let (out, diags) = interpret(r#"fn main() if true then print("yes") else print("other") end end"#, 0);
    assert!(!diags.has_errors(), "if_else_26: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"yes
");
}

#[test]
fn if_else_27() {
    let (out, diags) = interpret(r#"fn main() if false then print("yes") else print("other") end end"#, 0);
    assert!(!diags.has_errors(), "if_else_27: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"other
");
}

#[test]
fn if_else_28() {
    let (out, diags) = interpret(r#"fn main() if true then print("yes") else print("other") end end"#, 0);
    assert!(!diags.has_errors(), "if_else_28: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"yes
");
}

#[test]
fn if_else_29() {
    let (out, diags) = interpret(r#"fn main() if false then print("yes") else print("other") end end"#, 0);
    assert!(!diags.has_errors(), "if_else_29: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"other
");
}

#[test]
fn if_else_30() {
    let (out, diags) = interpret(r#"fn main() if true then print("yes") else print("other") end end"#, 0);
    assert!(!diags.has_errors(), "if_else_30: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"yes
");
}

#[test]
fn if_else_31() {
    let (out, diags) = interpret(r#"fn main() if false then print("yes") else print("other") end end"#, 0);
    assert!(!diags.has_errors(), "if_else_31: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"other
");
}

#[test]
fn if_else_32() {
    let (out, diags) = interpret(r#"fn main() if true then print("yes") else print("other") end end"#, 0);
    assert!(!diags.has_errors(), "if_else_32: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"yes
");
}

#[test]
fn if_else_33() {
    let (out, diags) = interpret(r#"fn main() if false then print("yes") else print("other") end end"#, 0);
    assert!(!diags.has_errors(), "if_else_33: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"other
");
}

#[test]
fn if_else_34() {
    let (out, diags) = interpret(r#"fn main() if true then print("yes") else print("other") end end"#, 0);
    assert!(!diags.has_errors(), "if_else_34: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"yes
");
}

#[test]
fn if_else_35() {
    let (out, diags) = interpret(r#"fn main() if false then print("yes") else print("other") end end"#, 0);
    assert!(!diags.has_errors(), "if_else_35: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"other
");
}

#[test]
fn if_else_36() {
    let (out, diags) = interpret(r#"fn main() if true then print("yes") else print("other") end end"#, 0);
    assert!(!diags.has_errors(), "if_else_36: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"yes
");
}

#[test]
fn if_else_37() {
    let (out, diags) = interpret(r#"fn main() if false then print("yes") else print("other") end end"#, 0);
    assert!(!diags.has_errors(), "if_else_37: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"other
");
}

#[test]
fn if_else_38() {
    let (out, diags) = interpret(r#"fn main() if true then print("yes") else print("other") end end"#, 0);
    assert!(!diags.has_errors(), "if_else_38: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"yes
");
}

#[test]
fn if_else_39() {
    let (out, diags) = interpret(r#"fn main() if false then print("yes") else print("other") end end"#, 0);
    assert!(!diags.has_errors(), "if_else_39: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"other
");
}

#[test]
fn if_else_40() {
    let (out, diags) = interpret(r#"fn main() if true then print("yes") else print("other") end end"#, 0);
    assert!(!diags.has_errors(), "if_else_40: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"yes
");
}

#[test]
fn if_else_41() {
    let (out, diags) = interpret(r#"fn main() if false then print("yes") else print("other") end end"#, 0);
    assert!(!diags.has_errors(), "if_else_41: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"other
");
}

#[test]
fn if_else_42() {
    let (out, diags) = interpret(r#"fn main() if true then print("yes") else print("other") end end"#, 0);
    assert!(!diags.has_errors(), "if_else_42: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"yes
");
}

#[test]
fn if_else_43() {
    let (out, diags) = interpret(r#"fn main() if false then print("yes") else print("other") end end"#, 0);
    assert!(!diags.has_errors(), "if_else_43: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"other
");
}

#[test]
fn if_else_44() {
    let (out, diags) = interpret(r#"fn main() if true then print("yes") else print("other") end end"#, 0);
    assert!(!diags.has_errors(), "if_else_44: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"yes
");
}

#[test]
fn if_else_45() {
    let (out, diags) = interpret(r#"fn main() if false then print("yes") else print("other") end end"#, 0);
    assert!(!diags.has_errors(), "if_else_45: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"other
");
}

#[test]
fn if_else_46() {
    let (out, diags) = interpret(r#"fn main() if true then print("yes") else print("other") end end"#, 0);
    assert!(!diags.has_errors(), "if_else_46: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"yes
");
}

#[test]
fn if_else_47() {
    let (out, diags) = interpret(r#"fn main() if false then print("yes") else print("other") end end"#, 0);
    assert!(!diags.has_errors(), "if_else_47: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"other
");
}

#[test]
fn if_else_48() {
    let (out, diags) = interpret(r#"fn main() if true then print("yes") else print("other") end end"#, 0);
    assert!(!diags.has_errors(), "if_else_48: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"yes
");
}

#[test]
fn if_else_49() {
    let (out, diags) = interpret(r#"fn main() if false then print("yes") else print("other") end end"#, 0);
    assert!(!diags.has_errors(), "if_else_49: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"other
");
}

#[test]
fn if_else_50() {
    let (out, diags) = interpret(r#"fn main() if true then print("yes") else print("other") end end"#, 0);
    assert!(!diags.has_errors(), "if_else_50: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"yes
");
}

#[test]
fn if_else_51() {
    let (out, diags) = interpret(r#"fn main() if false then print("yes") else print("other") end end"#, 0);
    assert!(!diags.has_errors(), "if_else_51: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"other
");
}

#[test]
fn if_else_52() {
    let (out, diags) = interpret(r#"fn main() if true then print("yes") else print("other") end end"#, 0);
    assert!(!diags.has_errors(), "if_else_52: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"yes
");
}

#[test]
fn if_else_53() {
    let (out, diags) = interpret(r#"fn main() if false then print("yes") else print("other") end end"#, 0);
    assert!(!diags.has_errors(), "if_else_53: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"other
");
}

#[test]
fn if_else_54() {
    let (out, diags) = interpret(r#"fn main() if true then print("yes") else print("other") end end"#, 0);
    assert!(!diags.has_errors(), "if_else_54: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"yes
");
}

#[test]
fn if_else_55() {
    let (out, diags) = interpret(r#"fn main() if false then print("yes") else print("other") end end"#, 0);
    assert!(!diags.has_errors(), "if_else_55: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"other
");
}

#[test]
fn if_else_56() {
    let (out, diags) = interpret(r#"fn main() if true then print("yes") else print("other") end end"#, 0);
    assert!(!diags.has_errors(), "if_else_56: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"yes
");
}

#[test]
fn if_else_57() {
    let (out, diags) = interpret(r#"fn main() if false then print("yes") else print("other") end end"#, 0);
    assert!(!diags.has_errors(), "if_else_57: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"other
");
}

#[test]
fn if_else_58() {
    let (out, diags) = interpret(r#"fn main() if true then print("yes") else print("other") end end"#, 0);
    assert!(!diags.has_errors(), "if_else_58: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"yes
");
}

#[test]
fn if_else_59() {
    let (out, diags) = interpret(r#"fn main() if false then print("yes") else print("other") end end"#, 0);
    assert!(!diags.has_errors(), "if_else_59: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"other
");
}

#[test]
fn if_else_60() {
    let (out, diags) = interpret(r#"fn main() if true then print("yes") else print("other") end end"#, 0);
    assert!(!diags.has_errors(), "if_else_60: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"yes
");
}

#[test]
fn if_else_61() {
    let (out, diags) = interpret(r#"fn main() if false then print("yes") else print("other") end end"#, 0);
    assert!(!diags.has_errors(), "if_else_61: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"other
");
}

#[test]
fn if_else_62() {
    let (out, diags) = interpret(r#"fn main() if true then print("yes") else print("other") end end"#, 0);
    assert!(!diags.has_errors(), "if_else_62: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"yes
");
}

#[test]
fn if_else_63() {
    let (out, diags) = interpret(r#"fn main() if false then print("yes") else print("other") end end"#, 0);
    assert!(!diags.has_errors(), "if_else_63: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"other
");
}

#[test]
fn if_else_64() {
    let (out, diags) = interpret(r#"fn main() if true then print("yes") else print("other") end end"#, 0);
    assert!(!diags.has_errors(), "if_else_64: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"yes
");
}

#[test]
fn if_else_65() {
    let (out, diags) = interpret(r#"fn main() if false then print("yes") else print("other") end end"#, 0);
    assert!(!diags.has_errors(), "if_else_65: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"other
");
}

#[test]
fn if_else_66() {
    let (out, diags) = interpret(r#"fn main() if true then print("yes") else print("other") end end"#, 0);
    assert!(!diags.has_errors(), "if_else_66: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"yes
");
}

#[test]
fn if_else_67() {
    let (out, diags) = interpret(r#"fn main() if false then print("yes") else print("other") end end"#, 0);
    assert!(!diags.has_errors(), "if_else_67: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"other
");
}

#[test]
fn if_else_68() {
    let (out, diags) = interpret(r#"fn main() if true then print("yes") else print("other") end end"#, 0);
    assert!(!diags.has_errors(), "if_else_68: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"yes
");
}

#[test]
fn if_else_69() {
    let (out, diags) = interpret(r#"fn main() if false then print("yes") else print("other") end end"#, 0);
    assert!(!diags.has_errors(), "if_else_69: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"other
");
}

#[test]
fn if_else_70() {
    let (out, diags) = interpret(r#"fn main() if true then print("yes") else print("other") end end"#, 0);
    assert!(!diags.has_errors(), "if_else_70: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"yes
");
}

#[test]
fn if_else_71() {
    let (out, diags) = interpret(r#"fn main() if false then print("yes") else print("other") end end"#, 0);
    assert!(!diags.has_errors(), "if_else_71: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"other
");
}

#[test]
fn if_else_72() {
    let (out, diags) = interpret(r#"fn main() if true then print("yes") else print("other") end end"#, 0);
    assert!(!diags.has_errors(), "if_else_72: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"yes
");
}

#[test]
fn if_else_73() {
    let (out, diags) = interpret(r#"fn main() if false then print("yes") else print("other") end end"#, 0);
    assert!(!diags.has_errors(), "if_else_73: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"other
");
}

#[test]
fn if_else_74() {
    let (out, diags) = interpret(r#"fn main() if true then print("yes") else print("other") end end"#, 0);
    assert!(!diags.has_errors(), "if_else_74: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"yes
");
}

#[test]
fn if_else_75() {
    let (out, diags) = interpret(r#"fn main() if false then print("yes") else print("other") end end"#, 0);
    assert!(!diags.has_errors(), "if_else_75: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"other
");
}

#[test]
fn if_else_76() {
    let (out, diags) = interpret(r#"fn main() if true then print("yes") else print("other") end end"#, 0);
    assert!(!diags.has_errors(), "if_else_76: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"yes
");
}

#[test]
fn if_else_77() {
    let (out, diags) = interpret(r#"fn main() if false then print("yes") else print("other") end end"#, 0);
    assert!(!diags.has_errors(), "if_else_77: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"other
");
}

#[test]
fn if_else_78() {
    let (out, diags) = interpret(r#"fn main() if true then print("yes") else print("other") end end"#, 0);
    assert!(!diags.has_errors(), "if_else_78: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"yes
");
}

#[test]
fn if_else_79() {
    let (out, diags) = interpret(r#"fn main() if false then print("yes") else print("other") end end"#, 0);
    assert!(!diags.has_errors(), "if_else_79: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"other
");
}

#[test]
fn if_else_80() {
    let (out, diags) = interpret(r#"fn main() if true then print("yes") else print("other") end end"#, 0);
    assert!(!diags.has_errors(), "if_else_80: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"yes
");
}

#[test]
fn if_else_81() {
    let (out, diags) = interpret(r#"fn main() if false then print("yes") else print("other") end end"#, 0);
    assert!(!diags.has_errors(), "if_else_81: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"other
");
}

#[test]
fn if_else_82() {
    let (out, diags) = interpret(r#"fn main() if true then print("yes") else print("other") end end"#, 0);
    assert!(!diags.has_errors(), "if_else_82: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"yes
");
}

#[test]
fn if_else_83() {
    let (out, diags) = interpret(r#"fn main() if false then print("yes") else print("other") end end"#, 0);
    assert!(!diags.has_errors(), "if_else_83: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"other
");
}

#[test]
fn if_else_84() {
    let (out, diags) = interpret(r#"fn main() if true then print("yes") else print("other") end end"#, 0);
    assert!(!diags.has_errors(), "if_else_84: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"yes
");
}

#[test]
fn if_else_85() {
    let (out, diags) = interpret(r#"fn main() if false then print("yes") else print("other") end end"#, 0);
    assert!(!diags.has_errors(), "if_else_85: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"other
");
}

#[test]
fn if_else_86() {
    let (out, diags) = interpret(r#"fn main() if true then print("yes") else print("other") end end"#, 0);
    assert!(!diags.has_errors(), "if_else_86: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"yes
");
}

#[test]
fn if_else_87() {
    let (out, diags) = interpret(r#"fn main() if false then print("yes") else print("other") end end"#, 0);
    assert!(!diags.has_errors(), "if_else_87: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"other
");
}

#[test]
fn if_else_88() {
    let (out, diags) = interpret(r#"fn main() if true then print("yes") else print("other") end end"#, 0);
    assert!(!diags.has_errors(), "if_else_88: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"yes
");
}

#[test]
fn if_else_89() {
    let (out, diags) = interpret(r#"fn main() if false then print("yes") else print("other") end end"#, 0);
    assert!(!diags.has_errors(), "if_else_89: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"other
");
}

#[test]
fn if_else_90() {
    let (out, diags) = interpret(r#"fn main() if true then print("yes") else print("other") end end"#, 0);
    assert!(!diags.has_errors(), "if_else_90: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"yes
");
}

#[test]
fn if_else_91() {
    let (out, diags) = interpret(r#"fn main() if false then print("yes") else print("other") end end"#, 0);
    assert!(!diags.has_errors(), "if_else_91: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"other
");
}

#[test]
fn if_else_92() {
    let (out, diags) = interpret(r#"fn main() if true then print("yes") else print("other") end end"#, 0);
    assert!(!diags.has_errors(), "if_else_92: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"yes
");
}

#[test]
fn if_else_93() {
    let (out, diags) = interpret(r#"fn main() if false then print("yes") else print("other") end end"#, 0);
    assert!(!diags.has_errors(), "if_else_93: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"other
");
}

#[test]
fn if_else_94() {
    let (out, diags) = interpret(r#"fn main() if true then print("yes") else print("other") end end"#, 0);
    assert!(!diags.has_errors(), "if_else_94: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"yes
");
}

#[test]
fn if_else_95() {
    let (out, diags) = interpret(r#"fn main() if false then print("yes") else print("other") end end"#, 0);
    assert!(!diags.has_errors(), "if_else_95: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"other
");
}

#[test]
fn if_else_96() {
    let (out, diags) = interpret(r#"fn main() if true then print("yes") else print("other") end end"#, 0);
    assert!(!diags.has_errors(), "if_else_96: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"yes
");
}

#[test]
fn if_else_97() {
    let (out, diags) = interpret(r#"fn main() if false then print("yes") else print("other") end end"#, 0);
    assert!(!diags.has_errors(), "if_else_97: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"other
");
}

#[test]
fn if_else_98() {
    let (out, diags) = interpret(r#"fn main() if true then print("yes") else print("other") end end"#, 0);
    assert!(!diags.has_errors(), "if_else_98: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"yes
");
}

#[test]
fn if_else_99() {
    let (out, diags) = interpret(r#"fn main() if false then print("yes") else print("other") end end"#, 0);
    assert!(!diags.has_errors(), "if_else_99: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"other
");
}

#[test]
fn if_compare_0() {
    let (out, diags) = interpret(r#"fn main() if 10 > 15 then print("high") else print("low") end end"#, 0);
    assert!(!diags.has_errors(), "if_compare_0: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"low
");
}

#[test]
fn if_compare_1() {
    let (out, diags) = interpret(r#"fn main() if 11 > 15 then print("high") else print("low") end end"#, 0);
    assert!(!diags.has_errors(), "if_compare_1: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"low
");
}

#[test]
fn if_compare_2() {
    let (out, diags) = interpret(r#"fn main() if 12 > 15 then print("high") else print("low") end end"#, 0);
    assert!(!diags.has_errors(), "if_compare_2: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"low
");
}

#[test]
fn if_compare_3() {
    let (out, diags) = interpret(r#"fn main() if 13 > 15 then print("high") else print("low") end end"#, 0);
    assert!(!diags.has_errors(), "if_compare_3: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"low
");
}

#[test]
fn if_compare_4() {
    let (out, diags) = interpret(r#"fn main() if 14 > 15 then print("high") else print("low") end end"#, 0);
    assert!(!diags.has_errors(), "if_compare_4: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"low
");
}

#[test]
fn if_compare_5() {
    let (out, diags) = interpret(r#"fn main() if 15 > 15 then print("high") else print("low") end end"#, 0);
    assert!(!diags.has_errors(), "if_compare_5: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"low
");
}

#[test]
fn if_compare_6() {
    let (out, diags) = interpret(r#"fn main() if 16 > 15 then print("high") else print("low") end end"#, 0);
    assert!(!diags.has_errors(), "if_compare_6: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"high
");
}

#[test]
fn if_compare_7() {
    let (out, diags) = interpret(r#"fn main() if 17 > 15 then print("high") else print("low") end end"#, 0);
    assert!(!diags.has_errors(), "if_compare_7: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"high
");
}

#[test]
fn if_compare_8() {
    let (out, diags) = interpret(r#"fn main() if 18 > 15 then print("high") else print("low") end end"#, 0);
    assert!(!diags.has_errors(), "if_compare_8: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"high
");
}

#[test]
fn if_compare_9() {
    let (out, diags) = interpret(r#"fn main() if 19 > 15 then print("high") else print("low") end end"#, 0);
    assert!(!diags.has_errors(), "if_compare_9: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"high
");
}

#[test]
fn if_compare_10() {
    let (out, diags) = interpret(r#"fn main() if 20 > 15 then print("high") else print("low") end end"#, 0);
    assert!(!diags.has_errors(), "if_compare_10: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"high
");
}

#[test]
fn if_compare_11() {
    let (out, diags) = interpret(r#"fn main() if 21 > 15 then print("high") else print("low") end end"#, 0);
    assert!(!diags.has_errors(), "if_compare_11: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"high
");
}

#[test]
fn if_compare_12() {
    let (out, diags) = interpret(r#"fn main() if 22 > 15 then print("high") else print("low") end end"#, 0);
    assert!(!diags.has_errors(), "if_compare_12: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"high
");
}

#[test]
fn if_compare_13() {
    let (out, diags) = interpret(r#"fn main() if 23 > 15 then print("high") else print("low") end end"#, 0);
    assert!(!diags.has_errors(), "if_compare_13: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"high
");
}

#[test]
fn if_compare_14() {
    let (out, diags) = interpret(r#"fn main() if 24 > 15 then print("high") else print("low") end end"#, 0);
    assert!(!diags.has_errors(), "if_compare_14: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"high
");
}

#[test]
fn if_compare_15() {
    let (out, diags) = interpret(r#"fn main() if 25 > 15 then print("high") else print("low") end end"#, 0);
    assert!(!diags.has_errors(), "if_compare_15: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"high
");
}

#[test]
fn if_compare_16() {
    let (out, diags) = interpret(r#"fn main() if 26 > 15 then print("high") else print("low") end end"#, 0);
    assert!(!diags.has_errors(), "if_compare_16: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"high
");
}

#[test]
fn if_compare_17() {
    let (out, diags) = interpret(r#"fn main() if 27 > 15 then print("high") else print("low") end end"#, 0);
    assert!(!diags.has_errors(), "if_compare_17: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"high
");
}

#[test]
fn if_compare_18() {
    let (out, diags) = interpret(r#"fn main() if 28 > 15 then print("high") else print("low") end end"#, 0);
    assert!(!diags.has_errors(), "if_compare_18: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"high
");
}

#[test]
fn if_compare_19() {
    let (out, diags) = interpret(r#"fn main() if 29 > 15 then print("high") else print("low") end end"#, 0);
    assert!(!diags.has_errors(), "if_compare_19: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"high
");
}

#[test]
fn if_compare_20() {
    let (out, diags) = interpret(r#"fn main() if 30 > 15 then print("high") else print("low") end end"#, 0);
    assert!(!diags.has_errors(), "if_compare_20: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"high
");
}

#[test]
fn if_compare_21() {
    let (out, diags) = interpret(r#"fn main() if 31 > 15 then print("high") else print("low") end end"#, 0);
    assert!(!diags.has_errors(), "if_compare_21: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"high
");
}

#[test]
fn if_compare_22() {
    let (out, diags) = interpret(r#"fn main() if 32 > 15 then print("high") else print("low") end end"#, 0);
    assert!(!diags.has_errors(), "if_compare_22: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"high
");
}

#[test]
fn if_compare_23() {
    let (out, diags) = interpret(r#"fn main() if 33 > 15 then print("high") else print("low") end end"#, 0);
    assert!(!diags.has_errors(), "if_compare_23: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"high
");
}

#[test]
fn if_compare_24() {
    let (out, diags) = interpret(r#"fn main() if 34 > 15 then print("high") else print("low") end end"#, 0);
    assert!(!diags.has_errors(), "if_compare_24: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"high
");
}

#[test]
fn if_compare_25() {
    let (out, diags) = interpret(r#"fn main() if 35 > 15 then print("high") else print("low") end end"#, 0);
    assert!(!diags.has_errors(), "if_compare_25: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"high
");
}

#[test]
fn if_compare_26() {
    let (out, diags) = interpret(r#"fn main() if 36 > 15 then print("high") else print("low") end end"#, 0);
    assert!(!diags.has_errors(), "if_compare_26: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"high
");
}

#[test]
fn if_compare_27() {
    let (out, diags) = interpret(r#"fn main() if 37 > 15 then print("high") else print("low") end end"#, 0);
    assert!(!diags.has_errors(), "if_compare_27: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"high
");
}

#[test]
fn if_compare_28() {
    let (out, diags) = interpret(r#"fn main() if 38 > 15 then print("high") else print("low") end end"#, 0);
    assert!(!diags.has_errors(), "if_compare_28: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"high
");
}

#[test]
fn if_compare_29() {
    let (out, diags) = interpret(r#"fn main() if 39 > 15 then print("high") else print("low") end end"#, 0);
    assert!(!diags.has_errors(), "if_compare_29: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"high
");
}

#[test]
fn if_compare_30() {
    let (out, diags) = interpret(r#"fn main() if 40 > 15 then print("high") else print("low") end end"#, 0);
    assert!(!diags.has_errors(), "if_compare_30: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"high
");
}

#[test]
fn if_compare_31() {
    let (out, diags) = interpret(r#"fn main() if 41 > 15 then print("high") else print("low") end end"#, 0);
    assert!(!diags.has_errors(), "if_compare_31: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"high
");
}

#[test]
fn if_compare_32() {
    let (out, diags) = interpret(r#"fn main() if 42 > 15 then print("high") else print("low") end end"#, 0);
    assert!(!diags.has_errors(), "if_compare_32: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"high
");
}

#[test]
fn if_compare_33() {
    let (out, diags) = interpret(r#"fn main() if 43 > 15 then print("high") else print("low") end end"#, 0);
    assert!(!diags.has_errors(), "if_compare_33: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"high
");
}

#[test]
fn if_compare_34() {
    let (out, diags) = interpret(r#"fn main() if 44 > 15 then print("high") else print("low") end end"#, 0);
    assert!(!diags.has_errors(), "if_compare_34: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"high
");
}

#[test]
fn if_compare_35() {
    let (out, diags) = interpret(r#"fn main() if 45 > 15 then print("high") else print("low") end end"#, 0);
    assert!(!diags.has_errors(), "if_compare_35: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"high
");
}

#[test]
fn if_compare_36() {
    let (out, diags) = interpret(r#"fn main() if 46 > 15 then print("high") else print("low") end end"#, 0);
    assert!(!diags.has_errors(), "if_compare_36: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"high
");
}

#[test]
fn if_compare_37() {
    let (out, diags) = interpret(r#"fn main() if 47 > 15 then print("high") else print("low") end end"#, 0);
    assert!(!diags.has_errors(), "if_compare_37: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"high
");
}

#[test]
fn if_compare_38() {
    let (out, diags) = interpret(r#"fn main() if 48 > 15 then print("high") else print("low") end end"#, 0);
    assert!(!diags.has_errors(), "if_compare_38: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"high
");
}

#[test]
fn if_compare_39() {
    let (out, diags) = interpret(r#"fn main() if 49 > 15 then print("high") else print("low") end end"#, 0);
    assert!(!diags.has_errors(), "if_compare_39: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"high
");
}

#[test]
fn if_compare_40() {
    let (out, diags) = interpret(r#"fn main() if 50 > 15 then print("high") else print("low") end end"#, 0);
    assert!(!diags.has_errors(), "if_compare_40: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"high
");
}

#[test]
fn if_compare_41() {
    let (out, diags) = interpret(r#"fn main() if 51 > 15 then print("high") else print("low") end end"#, 0);
    assert!(!diags.has_errors(), "if_compare_41: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"high
");
}

#[test]
fn if_compare_42() {
    let (out, diags) = interpret(r#"fn main() if 52 > 15 then print("high") else print("low") end end"#, 0);
    assert!(!diags.has_errors(), "if_compare_42: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"high
");
}

#[test]
fn if_compare_43() {
    let (out, diags) = interpret(r#"fn main() if 53 > 15 then print("high") else print("low") end end"#, 0);
    assert!(!diags.has_errors(), "if_compare_43: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"high
");
}

#[test]
fn if_compare_44() {
    let (out, diags) = interpret(r#"fn main() if 54 > 15 then print("high") else print("low") end end"#, 0);
    assert!(!diags.has_errors(), "if_compare_44: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"high
");
}

#[test]
fn if_compare_45() {
    let (out, diags) = interpret(r#"fn main() if 55 > 15 then print("high") else print("low") end end"#, 0);
    assert!(!diags.has_errors(), "if_compare_45: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"high
");
}

#[test]
fn if_compare_46() {
    let (out, diags) = interpret(r#"fn main() if 56 > 15 then print("high") else print("low") end end"#, 0);
    assert!(!diags.has_errors(), "if_compare_46: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"high
");
}

#[test]
fn if_compare_47() {
    let (out, diags) = interpret(r#"fn main() if 57 > 15 then print("high") else print("low") end end"#, 0);
    assert!(!diags.has_errors(), "if_compare_47: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"high
");
}

#[test]
fn if_compare_48() {
    let (out, diags) = interpret(r#"fn main() if 58 > 15 then print("high") else print("low") end end"#, 0);
    assert!(!diags.has_errors(), "if_compare_48: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"high
");
}

#[test]
fn if_compare_49() {
    let (out, diags) = interpret(r#"fn main() if 59 > 15 then print("high") else print("low") end end"#, 0);
    assert!(!diags.has_errors(), "if_compare_49: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"high
");
}

#[test]
fn if_elseif_0() {
    let (out, diags) = interpret(r#"fn main() if 0 < 25 then print("neg") elseif 0 < 50 then print("mid") else print("high") end end"#, 0);
    assert!(!diags.has_errors(), "if_elseif_0: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"neg
");
}

#[test]
fn if_elseif_1() {
    let (out, diags) = interpret(r#"fn main() if 1 < 25 then print("neg") elseif 1 < 50 then print("mid") else print("high") end end"#, 0);
    assert!(!diags.has_errors(), "if_elseif_1: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"neg
");
}

#[test]
fn if_elseif_2() {
    let (out, diags) = interpret(r#"fn main() if 2 < 25 then print("neg") elseif 2 < 50 then print("mid") else print("high") end end"#, 0);
    assert!(!diags.has_errors(), "if_elseif_2: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"neg
");
}

#[test]
fn if_elseif_3() {
    let (out, diags) = interpret(r#"fn main() if 3 < 25 then print("neg") elseif 3 < 50 then print("mid") else print("high") end end"#, 0);
    assert!(!diags.has_errors(), "if_elseif_3: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"neg
");
}

#[test]
fn if_elseif_4() {
    let (out, diags) = interpret(r#"fn main() if 4 < 25 then print("neg") elseif 4 < 50 then print("mid") else print("high") end end"#, 0);
    assert!(!diags.has_errors(), "if_elseif_4: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"neg
");
}

#[test]
fn if_elseif_5() {
    let (out, diags) = interpret(r#"fn main() if 5 < 25 then print("neg") elseif 5 < 50 then print("mid") else print("high") end end"#, 0);
    assert!(!diags.has_errors(), "if_elseif_5: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"neg
");
}

#[test]
fn if_elseif_6() {
    let (out, diags) = interpret(r#"fn main() if 6 < 25 then print("neg") elseif 6 < 50 then print("mid") else print("high") end end"#, 0);
    assert!(!diags.has_errors(), "if_elseif_6: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"neg
");
}

#[test]
fn if_elseif_7() {
    let (out, diags) = interpret(r#"fn main() if 7 < 25 then print("neg") elseif 7 < 50 then print("mid") else print("high") end end"#, 0);
    assert!(!diags.has_errors(), "if_elseif_7: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"neg
");
}

#[test]
fn if_elseif_8() {
    let (out, diags) = interpret(r#"fn main() if 8 < 25 then print("neg") elseif 8 < 50 then print("mid") else print("high") end end"#, 0);
    assert!(!diags.has_errors(), "if_elseif_8: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"neg
");
}

#[test]
fn if_elseif_9() {
    let (out, diags) = interpret(r#"fn main() if 9 < 25 then print("neg") elseif 9 < 50 then print("mid") else print("high") end end"#, 0);
    assert!(!diags.has_errors(), "if_elseif_9: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"neg
");
}

#[test]
fn if_elseif_10() {
    let (out, diags) = interpret(r#"fn main() if 10 < 25 then print("neg") elseif 10 < 50 then print("mid") else print("high") end end"#, 0);
    assert!(!diags.has_errors(), "if_elseif_10: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"neg
");
}

#[test]
fn if_elseif_11() {
    let (out, diags) = interpret(r#"fn main() if 11 < 25 then print("neg") elseif 11 < 50 then print("mid") else print("high") end end"#, 0);
    assert!(!diags.has_errors(), "if_elseif_11: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"neg
");
}

#[test]
fn if_elseif_12() {
    let (out, diags) = interpret(r#"fn main() if 12 < 25 then print("neg") elseif 12 < 50 then print("mid") else print("high") end end"#, 0);
    assert!(!diags.has_errors(), "if_elseif_12: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"neg
");
}

#[test]
fn if_elseif_13() {
    let (out, diags) = interpret(r#"fn main() if 13 < 25 then print("neg") elseif 13 < 50 then print("mid") else print("high") end end"#, 0);
    assert!(!diags.has_errors(), "if_elseif_13: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"neg
");
}

#[test]
fn if_elseif_14() {
    let (out, diags) = interpret(r#"fn main() if 14 < 25 then print("neg") elseif 14 < 50 then print("mid") else print("high") end end"#, 0);
    assert!(!diags.has_errors(), "if_elseif_14: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"neg
");
}

#[test]
fn if_elseif_15() {
    let (out, diags) = interpret(r#"fn main() if 15 < 25 then print("neg") elseif 15 < 50 then print("mid") else print("high") end end"#, 0);
    assert!(!diags.has_errors(), "if_elseif_15: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"neg
");
}

#[test]
fn if_elseif_16() {
    let (out, diags) = interpret(r#"fn main() if 16 < 25 then print("neg") elseif 16 < 50 then print("mid") else print("high") end end"#, 0);
    assert!(!diags.has_errors(), "if_elseif_16: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"neg
");
}

#[test]
fn if_elseif_17() {
    let (out, diags) = interpret(r#"fn main() if 17 < 25 then print("neg") elseif 17 < 50 then print("mid") else print("high") end end"#, 0);
    assert!(!diags.has_errors(), "if_elseif_17: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"neg
");
}

#[test]
fn if_elseif_18() {
    let (out, diags) = interpret(r#"fn main() if 18 < 25 then print("neg") elseif 18 < 50 then print("mid") else print("high") end end"#, 0);
    assert!(!diags.has_errors(), "if_elseif_18: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"neg
");
}

#[test]
fn if_elseif_19() {
    let (out, diags) = interpret(r#"fn main() if 19 < 25 then print("neg") elseif 19 < 50 then print("mid") else print("high") end end"#, 0);
    assert!(!diags.has_errors(), "if_elseif_19: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"neg
");
}

#[test]
fn if_elseif_20() {
    let (out, diags) = interpret(r#"fn main() if 20 < 25 then print("neg") elseif 20 < 50 then print("mid") else print("high") end end"#, 0);
    assert!(!diags.has_errors(), "if_elseif_20: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"neg
");
}

#[test]
fn if_elseif_21() {
    let (out, diags) = interpret(r#"fn main() if 21 < 25 then print("neg") elseif 21 < 50 then print("mid") else print("high") end end"#, 0);
    assert!(!diags.has_errors(), "if_elseif_21: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"neg
");
}

#[test]
fn if_elseif_22() {
    let (out, diags) = interpret(r#"fn main() if 22 < 25 then print("neg") elseif 22 < 50 then print("mid") else print("high") end end"#, 0);
    assert!(!diags.has_errors(), "if_elseif_22: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"neg
");
}

#[test]
fn if_elseif_23() {
    let (out, diags) = interpret(r#"fn main() if 23 < 25 then print("neg") elseif 23 < 50 then print("mid") else print("high") end end"#, 0);
    assert!(!diags.has_errors(), "if_elseif_23: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"neg
");
}

#[test]
fn if_elseif_24() {
    let (out, diags) = interpret(r#"fn main() if 24 < 25 then print("neg") elseif 24 < 50 then print("mid") else print("high") end end"#, 0);
    assert!(!diags.has_errors(), "if_elseif_24: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"neg
");
}

#[test]
fn if_elseif_25() {
    let (out, diags) = interpret(r#"fn main() if 25 < 25 then print("neg") elseif 25 < 50 then print("mid") else print("high") end end"#, 0);
    assert!(!diags.has_errors(), "if_elseif_25: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"mid
");
}

#[test]
fn if_elseif_26() {
    let (out, diags) = interpret(r#"fn main() if 26 < 25 then print("neg") elseif 26 < 50 then print("mid") else print("high") end end"#, 0);
    assert!(!diags.has_errors(), "if_elseif_26: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"mid
");
}

#[test]
fn if_elseif_27() {
    let (out, diags) = interpret(r#"fn main() if 27 < 25 then print("neg") elseif 27 < 50 then print("mid") else print("high") end end"#, 0);
    assert!(!diags.has_errors(), "if_elseif_27: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"mid
");
}

#[test]
fn if_elseif_28() {
    let (out, diags) = interpret(r#"fn main() if 28 < 25 then print("neg") elseif 28 < 50 then print("mid") else print("high") end end"#, 0);
    assert!(!diags.has_errors(), "if_elseif_28: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"mid
");
}

#[test]
fn if_elseif_29() {
    let (out, diags) = interpret(r#"fn main() if 29 < 25 then print("neg") elseif 29 < 50 then print("mid") else print("high") end end"#, 0);
    assert!(!diags.has_errors(), "if_elseif_29: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"mid
");
}

#[test]
fn if_elseif_30() {
    let (out, diags) = interpret(r#"fn main() if 30 < 25 then print("neg") elseif 30 < 50 then print("mid") else print("high") end end"#, 0);
    assert!(!diags.has_errors(), "if_elseif_30: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"mid
");
}

#[test]
fn if_elseif_31() {
    let (out, diags) = interpret(r#"fn main() if 31 < 25 then print("neg") elseif 31 < 50 then print("mid") else print("high") end end"#, 0);
    assert!(!diags.has_errors(), "if_elseif_31: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"mid
");
}

#[test]
fn if_elseif_32() {
    let (out, diags) = interpret(r#"fn main() if 32 < 25 then print("neg") elseif 32 < 50 then print("mid") else print("high") end end"#, 0);
    assert!(!diags.has_errors(), "if_elseif_32: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"mid
");
}

#[test]
fn if_elseif_33() {
    let (out, diags) = interpret(r#"fn main() if 33 < 25 then print("neg") elseif 33 < 50 then print("mid") else print("high") end end"#, 0);
    assert!(!diags.has_errors(), "if_elseif_33: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"mid
");
}

#[test]
fn if_elseif_34() {
    let (out, diags) = interpret(r#"fn main() if 34 < 25 then print("neg") elseif 34 < 50 then print("mid") else print("high") end end"#, 0);
    assert!(!diags.has_errors(), "if_elseif_34: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"mid
");
}

#[test]
fn if_elseif_35() {
    let (out, diags) = interpret(r#"fn main() if 35 < 25 then print("neg") elseif 35 < 50 then print("mid") else print("high") end end"#, 0);
    assert!(!diags.has_errors(), "if_elseif_35: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"mid
");
}

#[test]
fn if_elseif_36() {
    let (out, diags) = interpret(r#"fn main() if 36 < 25 then print("neg") elseif 36 < 50 then print("mid") else print("high") end end"#, 0);
    assert!(!diags.has_errors(), "if_elseif_36: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"mid
");
}

#[test]
fn if_elseif_37() {
    let (out, diags) = interpret(r#"fn main() if 37 < 25 then print("neg") elseif 37 < 50 then print("mid") else print("high") end end"#, 0);
    assert!(!diags.has_errors(), "if_elseif_37: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"mid
");
}

#[test]
fn if_elseif_38() {
    let (out, diags) = interpret(r#"fn main() if 38 < 25 then print("neg") elseif 38 < 50 then print("mid") else print("high") end end"#, 0);
    assert!(!diags.has_errors(), "if_elseif_38: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"mid
");
}

#[test]
fn if_elseif_39() {
    let (out, diags) = interpret(r#"fn main() if 39 < 25 then print("neg") elseif 39 < 50 then print("mid") else print("high") end end"#, 0);
    assert!(!diags.has_errors(), "if_elseif_39: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"mid
");
}

#[test]
fn if_elseif_40() {
    let (out, diags) = interpret(r#"fn main() if 40 < 25 then print("neg") elseif 40 < 50 then print("mid") else print("high") end end"#, 0);
    assert!(!diags.has_errors(), "if_elseif_40: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"mid
");
}

#[test]
fn if_elseif_41() {
    let (out, diags) = interpret(r#"fn main() if 41 < 25 then print("neg") elseif 41 < 50 then print("mid") else print("high") end end"#, 0);
    assert!(!diags.has_errors(), "if_elseif_41: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"mid
");
}

#[test]
fn if_elseif_42() {
    let (out, diags) = interpret(r#"fn main() if 42 < 25 then print("neg") elseif 42 < 50 then print("mid") else print("high") end end"#, 0);
    assert!(!diags.has_errors(), "if_elseif_42: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"mid
");
}

#[test]
fn if_elseif_43() {
    let (out, diags) = interpret(r#"fn main() if 43 < 25 then print("neg") elseif 43 < 50 then print("mid") else print("high") end end"#, 0);
    assert!(!diags.has_errors(), "if_elseif_43: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"mid
");
}

#[test]
fn if_elseif_44() {
    let (out, diags) = interpret(r#"fn main() if 44 < 25 then print("neg") elseif 44 < 50 then print("mid") else print("high") end end"#, 0);
    assert!(!diags.has_errors(), "if_elseif_44: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"mid
");
}

#[test]
fn if_elseif_45() {
    let (out, diags) = interpret(r#"fn main() if 45 < 25 then print("neg") elseif 45 < 50 then print("mid") else print("high") end end"#, 0);
    assert!(!diags.has_errors(), "if_elseif_45: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"mid
");
}

#[test]
fn if_elseif_46() {
    let (out, diags) = interpret(r#"fn main() if 46 < 25 then print("neg") elseif 46 < 50 then print("mid") else print("high") end end"#, 0);
    assert!(!diags.has_errors(), "if_elseif_46: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"mid
");
}

#[test]
fn if_elseif_47() {
    let (out, diags) = interpret(r#"fn main() if 47 < 25 then print("neg") elseif 47 < 50 then print("mid") else print("high") end end"#, 0);
    assert!(!diags.has_errors(), "if_elseif_47: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"mid
");
}

#[test]
fn if_elseif_48() {
    let (out, diags) = interpret(r#"fn main() if 48 < 25 then print("neg") elseif 48 < 50 then print("mid") else print("high") end end"#, 0);
    assert!(!diags.has_errors(), "if_elseif_48: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"mid
");
}

#[test]
fn if_elseif_49() {
    let (out, diags) = interpret(r#"fn main() if 49 < 25 then print("neg") elseif 49 < 50 then print("mid") else print("high") end end"#, 0);
    assert!(!diags.has_errors(), "if_elseif_49: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"mid
");
}

