// Owner: PascalElixir / axolrs (GitHub org)
// File: Auto-generated tests for boolean logic.

use axolc_core::interpret;

#[test]
fn and_0() {
    let (out, diags) = interpret("fn main() print(true and true) end", 0);
    assert!(!diags.has_errors(), "and_0: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn and_1() {
    let (out, diags) = interpret("fn main() print(false and false) end", 0);
    assert!(!diags.has_errors(), "and_1: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn and_2() {
    let (out, diags) = interpret("fn main() print(true and false) end", 0);
    assert!(!diags.has_errors(), "and_2: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn and_3() {
    let (out, diags) = interpret("fn main() print(false and true) end", 0);
    assert!(!diags.has_errors(), "and_3: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn and_4() {
    let (out, diags) = interpret("fn main() print(true and false) end", 0);
    assert!(!diags.has_errors(), "and_4: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn and_5() {
    let (out, diags) = interpret("fn main() print(false and false) end", 0);
    assert!(!diags.has_errors(), "and_5: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn and_6() {
    let (out, diags) = interpret("fn main() print(true and true) end", 0);
    assert!(!diags.has_errors(), "and_6: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn and_7() {
    let (out, diags) = interpret("fn main() print(false and false) end", 0);
    assert!(!diags.has_errors(), "and_7: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn and_8() {
    let (out, diags) = interpret("fn main() print(true and false) end", 0);
    assert!(!diags.has_errors(), "and_8: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn and_9() {
    let (out, diags) = interpret("fn main() print(false and true) end", 0);
    assert!(!diags.has_errors(), "and_9: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn and_10() {
    let (out, diags) = interpret("fn main() print(true and false) end", 0);
    assert!(!diags.has_errors(), "and_10: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn and_11() {
    let (out, diags) = interpret("fn main() print(false and false) end", 0);
    assert!(!diags.has_errors(), "and_11: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn and_12() {
    let (out, diags) = interpret("fn main() print(true and true) end", 0);
    assert!(!diags.has_errors(), "and_12: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn and_13() {
    let (out, diags) = interpret("fn main() print(false and false) end", 0);
    assert!(!diags.has_errors(), "and_13: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn and_14() {
    let (out, diags) = interpret("fn main() print(true and false) end", 0);
    assert!(!diags.has_errors(), "and_14: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn and_15() {
    let (out, diags) = interpret("fn main() print(false and true) end", 0);
    assert!(!diags.has_errors(), "and_15: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn and_16() {
    let (out, diags) = interpret("fn main() print(true and false) end", 0);
    assert!(!diags.has_errors(), "and_16: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn and_17() {
    let (out, diags) = interpret("fn main() print(false and false) end", 0);
    assert!(!diags.has_errors(), "and_17: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn and_18() {
    let (out, diags) = interpret("fn main() print(true and true) end", 0);
    assert!(!diags.has_errors(), "and_18: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn and_19() {
    let (out, diags) = interpret("fn main() print(false and false) end", 0);
    assert!(!diags.has_errors(), "and_19: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn and_20() {
    let (out, diags) = interpret("fn main() print(true and false) end", 0);
    assert!(!diags.has_errors(), "and_20: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn and_21() {
    let (out, diags) = interpret("fn main() print(false and true) end", 0);
    assert!(!diags.has_errors(), "and_21: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn and_22() {
    let (out, diags) = interpret("fn main() print(true and false) end", 0);
    assert!(!diags.has_errors(), "and_22: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn and_23() {
    let (out, diags) = interpret("fn main() print(false and false) end", 0);
    assert!(!diags.has_errors(), "and_23: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn and_24() {
    let (out, diags) = interpret("fn main() print(true and true) end", 0);
    assert!(!diags.has_errors(), "and_24: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn and_25() {
    let (out, diags) = interpret("fn main() print(false and false) end", 0);
    assert!(!diags.has_errors(), "and_25: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn and_26() {
    let (out, diags) = interpret("fn main() print(true and false) end", 0);
    assert!(!diags.has_errors(), "and_26: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn and_27() {
    let (out, diags) = interpret("fn main() print(false and true) end", 0);
    assert!(!diags.has_errors(), "and_27: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn and_28() {
    let (out, diags) = interpret("fn main() print(true and false) end", 0);
    assert!(!diags.has_errors(), "and_28: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn and_29() {
    let (out, diags) = interpret("fn main() print(false and false) end", 0);
    assert!(!diags.has_errors(), "and_29: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn and_30() {
    let (out, diags) = interpret("fn main() print(true and true) end", 0);
    assert!(!diags.has_errors(), "and_30: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn and_31() {
    let (out, diags) = interpret("fn main() print(false and false) end", 0);
    assert!(!diags.has_errors(), "and_31: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn and_32() {
    let (out, diags) = interpret("fn main() print(true and false) end", 0);
    assert!(!diags.has_errors(), "and_32: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn and_33() {
    let (out, diags) = interpret("fn main() print(false and true) end", 0);
    assert!(!diags.has_errors(), "and_33: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn and_34() {
    let (out, diags) = interpret("fn main() print(true and false) end", 0);
    assert!(!diags.has_errors(), "and_34: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn and_35() {
    let (out, diags) = interpret("fn main() print(false and false) end", 0);
    assert!(!diags.has_errors(), "and_35: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn and_36() {
    let (out, diags) = interpret("fn main() print(true and true) end", 0);
    assert!(!diags.has_errors(), "and_36: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn and_37() {
    let (out, diags) = interpret("fn main() print(false and false) end", 0);
    assert!(!diags.has_errors(), "and_37: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn and_38() {
    let (out, diags) = interpret("fn main() print(true and false) end", 0);
    assert!(!diags.has_errors(), "and_38: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn and_39() {
    let (out, diags) = interpret("fn main() print(false and true) end", 0);
    assert!(!diags.has_errors(), "and_39: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn and_40() {
    let (out, diags) = interpret("fn main() print(true and false) end", 0);
    assert!(!diags.has_errors(), "and_40: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn and_41() {
    let (out, diags) = interpret("fn main() print(false and false) end", 0);
    assert!(!diags.has_errors(), "and_41: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn and_42() {
    let (out, diags) = interpret("fn main() print(true and true) end", 0);
    assert!(!diags.has_errors(), "and_42: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn and_43() {
    let (out, diags) = interpret("fn main() print(false and false) end", 0);
    assert!(!diags.has_errors(), "and_43: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn and_44() {
    let (out, diags) = interpret("fn main() print(true and false) end", 0);
    assert!(!diags.has_errors(), "and_44: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn and_45() {
    let (out, diags) = interpret("fn main() print(false and true) end", 0);
    assert!(!diags.has_errors(), "and_45: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn and_46() {
    let (out, diags) = interpret("fn main() print(true and false) end", 0);
    assert!(!diags.has_errors(), "and_46: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn and_47() {
    let (out, diags) = interpret("fn main() print(false and false) end", 0);
    assert!(!diags.has_errors(), "and_47: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn and_48() {
    let (out, diags) = interpret("fn main() print(true and true) end", 0);
    assert!(!diags.has_errors(), "and_48: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn and_49() {
    let (out, diags) = interpret("fn main() print(false and false) end", 0);
    assert!(!diags.has_errors(), "and_49: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn or_0() {
    let (out, diags) = interpret("fn main() print(true or true) end", 0);
    assert!(!diags.has_errors(), "or_0: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn or_1() {
    let (out, diags) = interpret("fn main() print(false or false) end", 0);
    assert!(!diags.has_errors(), "or_1: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn or_2() {
    let (out, diags) = interpret("fn main() print(true or false) end", 0);
    assert!(!diags.has_errors(), "or_2: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn or_3() {
    let (out, diags) = interpret("fn main() print(false or true) end", 0);
    assert!(!diags.has_errors(), "or_3: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn or_4() {
    let (out, diags) = interpret("fn main() print(true or false) end", 0);
    assert!(!diags.has_errors(), "or_4: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn or_5() {
    let (out, diags) = interpret("fn main() print(false or false) end", 0);
    assert!(!diags.has_errors(), "or_5: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn or_6() {
    let (out, diags) = interpret("fn main() print(true or true) end", 0);
    assert!(!diags.has_errors(), "or_6: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn or_7() {
    let (out, diags) = interpret("fn main() print(false or false) end", 0);
    assert!(!diags.has_errors(), "or_7: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn or_8() {
    let (out, diags) = interpret("fn main() print(true or false) end", 0);
    assert!(!diags.has_errors(), "or_8: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn or_9() {
    let (out, diags) = interpret("fn main() print(false or true) end", 0);
    assert!(!diags.has_errors(), "or_9: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn or_10() {
    let (out, diags) = interpret("fn main() print(true or false) end", 0);
    assert!(!diags.has_errors(), "or_10: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn or_11() {
    let (out, diags) = interpret("fn main() print(false or false) end", 0);
    assert!(!diags.has_errors(), "or_11: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn or_12() {
    let (out, diags) = interpret("fn main() print(true or true) end", 0);
    assert!(!diags.has_errors(), "or_12: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn or_13() {
    let (out, diags) = interpret("fn main() print(false or false) end", 0);
    assert!(!diags.has_errors(), "or_13: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn or_14() {
    let (out, diags) = interpret("fn main() print(true or false) end", 0);
    assert!(!diags.has_errors(), "or_14: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn or_15() {
    let (out, diags) = interpret("fn main() print(false or true) end", 0);
    assert!(!diags.has_errors(), "or_15: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn or_16() {
    let (out, diags) = interpret("fn main() print(true or false) end", 0);
    assert!(!diags.has_errors(), "or_16: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn or_17() {
    let (out, diags) = interpret("fn main() print(false or false) end", 0);
    assert!(!diags.has_errors(), "or_17: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn or_18() {
    let (out, diags) = interpret("fn main() print(true or true) end", 0);
    assert!(!diags.has_errors(), "or_18: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn or_19() {
    let (out, diags) = interpret("fn main() print(false or false) end", 0);
    assert!(!diags.has_errors(), "or_19: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn or_20() {
    let (out, diags) = interpret("fn main() print(true or false) end", 0);
    assert!(!diags.has_errors(), "or_20: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn or_21() {
    let (out, diags) = interpret("fn main() print(false or true) end", 0);
    assert!(!diags.has_errors(), "or_21: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn or_22() {
    let (out, diags) = interpret("fn main() print(true or false) end", 0);
    assert!(!diags.has_errors(), "or_22: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn or_23() {
    let (out, diags) = interpret("fn main() print(false or false) end", 0);
    assert!(!diags.has_errors(), "or_23: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn or_24() {
    let (out, diags) = interpret("fn main() print(true or true) end", 0);
    assert!(!diags.has_errors(), "or_24: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn or_25() {
    let (out, diags) = interpret("fn main() print(false or false) end", 0);
    assert!(!diags.has_errors(), "or_25: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn or_26() {
    let (out, diags) = interpret("fn main() print(true or false) end", 0);
    assert!(!diags.has_errors(), "or_26: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn or_27() {
    let (out, diags) = interpret("fn main() print(false or true) end", 0);
    assert!(!diags.has_errors(), "or_27: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn or_28() {
    let (out, diags) = interpret("fn main() print(true or false) end", 0);
    assert!(!diags.has_errors(), "or_28: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn or_29() {
    let (out, diags) = interpret("fn main() print(false or false) end", 0);
    assert!(!diags.has_errors(), "or_29: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn or_30() {
    let (out, diags) = interpret("fn main() print(true or true) end", 0);
    assert!(!diags.has_errors(), "or_30: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn or_31() {
    let (out, diags) = interpret("fn main() print(false or false) end", 0);
    assert!(!diags.has_errors(), "or_31: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn or_32() {
    let (out, diags) = interpret("fn main() print(true or false) end", 0);
    assert!(!diags.has_errors(), "or_32: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn or_33() {
    let (out, diags) = interpret("fn main() print(false or true) end", 0);
    assert!(!diags.has_errors(), "or_33: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn or_34() {
    let (out, diags) = interpret("fn main() print(true or false) end", 0);
    assert!(!diags.has_errors(), "or_34: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn or_35() {
    let (out, diags) = interpret("fn main() print(false or false) end", 0);
    assert!(!diags.has_errors(), "or_35: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn or_36() {
    let (out, diags) = interpret("fn main() print(true or true) end", 0);
    assert!(!diags.has_errors(), "or_36: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn or_37() {
    let (out, diags) = interpret("fn main() print(false or false) end", 0);
    assert!(!diags.has_errors(), "or_37: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn or_38() {
    let (out, diags) = interpret("fn main() print(true or false) end", 0);
    assert!(!diags.has_errors(), "or_38: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn or_39() {
    let (out, diags) = interpret("fn main() print(false or true) end", 0);
    assert!(!diags.has_errors(), "or_39: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn or_40() {
    let (out, diags) = interpret("fn main() print(true or false) end", 0);
    assert!(!diags.has_errors(), "or_40: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn or_41() {
    let (out, diags) = interpret("fn main() print(false or false) end", 0);
    assert!(!diags.has_errors(), "or_41: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn or_42() {
    let (out, diags) = interpret("fn main() print(true or true) end", 0);
    assert!(!diags.has_errors(), "or_42: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn or_43() {
    let (out, diags) = interpret("fn main() print(false or false) end", 0);
    assert!(!diags.has_errors(), "or_43: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn or_44() {
    let (out, diags) = interpret("fn main() print(true or false) end", 0);
    assert!(!diags.has_errors(), "or_44: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn or_45() {
    let (out, diags) = interpret("fn main() print(false or true) end", 0);
    assert!(!diags.has_errors(), "or_45: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn or_46() {
    let (out, diags) = interpret("fn main() print(true or false) end", 0);
    assert!(!diags.has_errors(), "or_46: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn or_47() {
    let (out, diags) = interpret("fn main() print(false or false) end", 0);
    assert!(!diags.has_errors(), "or_47: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn or_48() {
    let (out, diags) = interpret("fn main() print(true or true) end", 0);
    assert!(!diags.has_errors(), "or_48: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn or_49() {
    let (out, diags) = interpret("fn main() print(false or false) end", 0);
    assert!(!diags.has_errors(), "or_49: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn not_0() {
    let (out, diags) = interpret("fn main() print(not true) end", 0);
    assert!(!diags.has_errors(), "not_0: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn not_1() {
    let (out, diags) = interpret("fn main() print(not false) end", 0);
    assert!(!diags.has_errors(), "not_1: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn not_2() {
    let (out, diags) = interpret("fn main() print(not true) end", 0);
    assert!(!diags.has_errors(), "not_2: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn not_3() {
    let (out, diags) = interpret("fn main() print(not false) end", 0);
    assert!(!diags.has_errors(), "not_3: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn not_4() {
    let (out, diags) = interpret("fn main() print(not true) end", 0);
    assert!(!diags.has_errors(), "not_4: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn not_5() {
    let (out, diags) = interpret("fn main() print(not false) end", 0);
    assert!(!diags.has_errors(), "not_5: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn not_6() {
    let (out, diags) = interpret("fn main() print(not true) end", 0);
    assert!(!diags.has_errors(), "not_6: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn not_7() {
    let (out, diags) = interpret("fn main() print(not false) end", 0);
    assert!(!diags.has_errors(), "not_7: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn not_8() {
    let (out, diags) = interpret("fn main() print(not true) end", 0);
    assert!(!diags.has_errors(), "not_8: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn not_9() {
    let (out, diags) = interpret("fn main() print(not false) end", 0);
    assert!(!diags.has_errors(), "not_9: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn not_10() {
    let (out, diags) = interpret("fn main() print(not true) end", 0);
    assert!(!diags.has_errors(), "not_10: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn not_11() {
    let (out, diags) = interpret("fn main() print(not false) end", 0);
    assert!(!diags.has_errors(), "not_11: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn not_12() {
    let (out, diags) = interpret("fn main() print(not true) end", 0);
    assert!(!diags.has_errors(), "not_12: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn not_13() {
    let (out, diags) = interpret("fn main() print(not false) end", 0);
    assert!(!diags.has_errors(), "not_13: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn not_14() {
    let (out, diags) = interpret("fn main() print(not true) end", 0);
    assert!(!diags.has_errors(), "not_14: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn not_15() {
    let (out, diags) = interpret("fn main() print(not false) end", 0);
    assert!(!diags.has_errors(), "not_15: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn not_16() {
    let (out, diags) = interpret("fn main() print(not true) end", 0);
    assert!(!diags.has_errors(), "not_16: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn not_17() {
    let (out, diags) = interpret("fn main() print(not false) end", 0);
    assert!(!diags.has_errors(), "not_17: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn not_18() {
    let (out, diags) = interpret("fn main() print(not true) end", 0);
    assert!(!diags.has_errors(), "not_18: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn not_19() {
    let (out, diags) = interpret("fn main() print(not false) end", 0);
    assert!(!diags.has_errors(), "not_19: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn not_20() {
    let (out, diags) = interpret("fn main() print(not true) end", 0);
    assert!(!diags.has_errors(), "not_20: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn not_21() {
    let (out, diags) = interpret("fn main() print(not false) end", 0);
    assert!(!diags.has_errors(), "not_21: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn not_22() {
    let (out, diags) = interpret("fn main() print(not true) end", 0);
    assert!(!diags.has_errors(), "not_22: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn not_23() {
    let (out, diags) = interpret("fn main() print(not false) end", 0);
    assert!(!diags.has_errors(), "not_23: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn not_24() {
    let (out, diags) = interpret("fn main() print(not true) end", 0);
    assert!(!diags.has_errors(), "not_24: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn not_25() {
    let (out, diags) = interpret("fn main() print(not false) end", 0);
    assert!(!diags.has_errors(), "not_25: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn not_26() {
    let (out, diags) = interpret("fn main() print(not true) end", 0);
    assert!(!diags.has_errors(), "not_26: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn not_27() {
    let (out, diags) = interpret("fn main() print(not false) end", 0);
    assert!(!diags.has_errors(), "not_27: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn not_28() {
    let (out, diags) = interpret("fn main() print(not true) end", 0);
    assert!(!diags.has_errors(), "not_28: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn not_29() {
    let (out, diags) = interpret("fn main() print(not false) end", 0);
    assert!(!diags.has_errors(), "not_29: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn not_30() {
    let (out, diags) = interpret("fn main() print(not true) end", 0);
    assert!(!diags.has_errors(), "not_30: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn not_31() {
    let (out, diags) = interpret("fn main() print(not false) end", 0);
    assert!(!diags.has_errors(), "not_31: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn not_32() {
    let (out, diags) = interpret("fn main() print(not true) end", 0);
    assert!(!diags.has_errors(), "not_32: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn not_33() {
    let (out, diags) = interpret("fn main() print(not false) end", 0);
    assert!(!diags.has_errors(), "not_33: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn not_34() {
    let (out, diags) = interpret("fn main() print(not true) end", 0);
    assert!(!diags.has_errors(), "not_34: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn not_35() {
    let (out, diags) = interpret("fn main() print(not false) end", 0);
    assert!(!diags.has_errors(), "not_35: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn not_36() {
    let (out, diags) = interpret("fn main() print(not true) end", 0);
    assert!(!diags.has_errors(), "not_36: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn not_37() {
    let (out, diags) = interpret("fn main() print(not false) end", 0);
    assert!(!diags.has_errors(), "not_37: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn not_38() {
    let (out, diags) = interpret("fn main() print(not true) end", 0);
    assert!(!diags.has_errors(), "not_38: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn not_39() {
    let (out, diags) = interpret("fn main() print(not false) end", 0);
    assert!(!diags.has_errors(), "not_39: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn not_40() {
    let (out, diags) = interpret("fn main() print(not true) end", 0);
    assert!(!diags.has_errors(), "not_40: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn not_41() {
    let (out, diags) = interpret("fn main() print(not false) end", 0);
    assert!(!diags.has_errors(), "not_41: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn not_42() {
    let (out, diags) = interpret("fn main() print(not true) end", 0);
    assert!(!diags.has_errors(), "not_42: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn not_43() {
    let (out, diags) = interpret("fn main() print(not false) end", 0);
    assert!(!diags.has_errors(), "not_43: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn not_44() {
    let (out, diags) = interpret("fn main() print(not true) end", 0);
    assert!(!diags.has_errors(), "not_44: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn not_45() {
    let (out, diags) = interpret("fn main() print(not false) end", 0);
    assert!(!diags.has_errors(), "not_45: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn not_46() {
    let (out, diags) = interpret("fn main() print(not true) end", 0);
    assert!(!diags.has_errors(), "not_46: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn not_47() {
    let (out, diags) = interpret("fn main() print(not false) end", 0);
    assert!(!diags.has_errors(), "not_47: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn not_48() {
    let (out, diags) = interpret("fn main() print(not true) end", 0);
    assert!(!diags.has_errors(), "not_48: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn not_49() {
    let (out, diags) = interpret("fn main() print(not false) end", 0);
    assert!(!diags.has_errors(), "not_49: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

