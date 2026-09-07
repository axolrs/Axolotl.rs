// Owner: PascalElixir / axolrs (GitHub org)
// File: Auto-generated tests for multi-statement functions.

use axolc_core::interpret;

#[test]
fn multi_stmt_0() {
    let (out, diags) = interpret("fn main() let a = 0 let b = a + 1 let c = b + 1 print(a, b, c) end", 0);
    assert!(!diags.has_errors(), "multi_stmt_0: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"0 1 2
");
}

#[test]
fn multi_stmt_1() {
    let (out, diags) = interpret("fn main() let a = 1 let b = a + 1 let c = b + 1 print(a, b, c) end", 0);
    assert!(!diags.has_errors(), "multi_stmt_1: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"1 2 3
");
}

#[test]
fn multi_stmt_2() {
    let (out, diags) = interpret("fn main() let a = 2 let b = a + 1 let c = b + 1 print(a, b, c) end", 0);
    assert!(!diags.has_errors(), "multi_stmt_2: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"2 3 4
");
}

#[test]
fn multi_stmt_3() {
    let (out, diags) = interpret("fn main() let a = 3 let b = a + 1 let c = b + 1 print(a, b, c) end", 0);
    assert!(!diags.has_errors(), "multi_stmt_3: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"3 4 5
");
}

#[test]
fn multi_stmt_4() {
    let (out, diags) = interpret("fn main() let a = 4 let b = a + 1 let c = b + 1 print(a, b, c) end", 0);
    assert!(!diags.has_errors(), "multi_stmt_4: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"4 5 6
");
}

#[test]
fn multi_stmt_5() {
    let (out, diags) = interpret("fn main() let a = 5 let b = a + 1 let c = b + 1 print(a, b, c) end", 0);
    assert!(!diags.has_errors(), "multi_stmt_5: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"5 6 7
");
}

#[test]
fn multi_stmt_6() {
    let (out, diags) = interpret("fn main() let a = 6 let b = a + 1 let c = b + 1 print(a, b, c) end", 0);
    assert!(!diags.has_errors(), "multi_stmt_6: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"6 7 8
");
}

#[test]
fn multi_stmt_7() {
    let (out, diags) = interpret("fn main() let a = 7 let b = a + 1 let c = b + 1 print(a, b, c) end", 0);
    assert!(!diags.has_errors(), "multi_stmt_7: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"7 8 9
");
}

#[test]
fn multi_stmt_8() {
    let (out, diags) = interpret("fn main() let a = 8 let b = a + 1 let c = b + 1 print(a, b, c) end", 0);
    assert!(!diags.has_errors(), "multi_stmt_8: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"8 9 10
");
}

#[test]
fn multi_stmt_9() {
    let (out, diags) = interpret("fn main() let a = 9 let b = a + 1 let c = b + 1 print(a, b, c) end", 0);
    assert!(!diags.has_errors(), "multi_stmt_9: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"9 10 11
");
}

#[test]
fn multi_stmt_10() {
    let (out, diags) = interpret("fn main() let a = 10 let b = a + 1 let c = b + 1 print(a, b, c) end", 0);
    assert!(!diags.has_errors(), "multi_stmt_10: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"10 11 12
");
}

#[test]
fn multi_stmt_11() {
    let (out, diags) = interpret("fn main() let a = 11 let b = a + 1 let c = b + 1 print(a, b, c) end", 0);
    assert!(!diags.has_errors(), "multi_stmt_11: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"11 12 13
");
}

#[test]
fn multi_stmt_12() {
    let (out, diags) = interpret("fn main() let a = 12 let b = a + 1 let c = b + 1 print(a, b, c) end", 0);
    assert!(!diags.has_errors(), "multi_stmt_12: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"12 13 14
");
}

#[test]
fn multi_stmt_13() {
    let (out, diags) = interpret("fn main() let a = 13 let b = a + 1 let c = b + 1 print(a, b, c) end", 0);
    assert!(!diags.has_errors(), "multi_stmt_13: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"13 14 15
");
}

#[test]
fn multi_stmt_14() {
    let (out, diags) = interpret("fn main() let a = 14 let b = a + 1 let c = b + 1 print(a, b, c) end", 0);
    assert!(!diags.has_errors(), "multi_stmt_14: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"14 15 16
");
}

#[test]
fn multi_stmt_15() {
    let (out, diags) = interpret("fn main() let a = 15 let b = a + 1 let c = b + 1 print(a, b, c) end", 0);
    assert!(!diags.has_errors(), "multi_stmt_15: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"15 16 17
");
}

#[test]
fn multi_stmt_16() {
    let (out, diags) = interpret("fn main() let a = 16 let b = a + 1 let c = b + 1 print(a, b, c) end", 0);
    assert!(!diags.has_errors(), "multi_stmt_16: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"16 17 18
");
}

#[test]
fn multi_stmt_17() {
    let (out, diags) = interpret("fn main() let a = 17 let b = a + 1 let c = b + 1 print(a, b, c) end", 0);
    assert!(!diags.has_errors(), "multi_stmt_17: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"17 18 19
");
}

#[test]
fn multi_stmt_18() {
    let (out, diags) = interpret("fn main() let a = 18 let b = a + 1 let c = b + 1 print(a, b, c) end", 0);
    assert!(!diags.has_errors(), "multi_stmt_18: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"18 19 20
");
}

#[test]
fn multi_stmt_19() {
    let (out, diags) = interpret("fn main() let a = 19 let b = a + 1 let c = b + 1 print(a, b, c) end", 0);
    assert!(!diags.has_errors(), "multi_stmt_19: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"19 20 21
");
}

#[test]
fn multi_stmt_20() {
    let (out, diags) = interpret("fn main() let a = 20 let b = a + 1 let c = b + 1 print(a, b, c) end", 0);
    assert!(!diags.has_errors(), "multi_stmt_20: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"20 21 22
");
}

#[test]
fn multi_stmt_21() {
    let (out, diags) = interpret("fn main() let a = 21 let b = a + 1 let c = b + 1 print(a, b, c) end", 0);
    assert!(!diags.has_errors(), "multi_stmt_21: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"21 22 23
");
}

#[test]
fn multi_stmt_22() {
    let (out, diags) = interpret("fn main() let a = 22 let b = a + 1 let c = b + 1 print(a, b, c) end", 0);
    assert!(!diags.has_errors(), "multi_stmt_22: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"22 23 24
");
}

#[test]
fn multi_stmt_23() {
    let (out, diags) = interpret("fn main() let a = 23 let b = a + 1 let c = b + 1 print(a, b, c) end", 0);
    assert!(!diags.has_errors(), "multi_stmt_23: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"23 24 25
");
}

#[test]
fn multi_stmt_24() {
    let (out, diags) = interpret("fn main() let a = 24 let b = a + 1 let c = b + 1 print(a, b, c) end", 0);
    assert!(!diags.has_errors(), "multi_stmt_24: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"24 25 26
");
}

#[test]
fn multi_stmt_25() {
    let (out, diags) = interpret("fn main() let a = 25 let b = a + 1 let c = b + 1 print(a, b, c) end", 0);
    assert!(!diags.has_errors(), "multi_stmt_25: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"25 26 27
");
}

#[test]
fn multi_stmt_26() {
    let (out, diags) = interpret("fn main() let a = 26 let b = a + 1 let c = b + 1 print(a, b, c) end", 0);
    assert!(!diags.has_errors(), "multi_stmt_26: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"26 27 28
");
}

#[test]
fn multi_stmt_27() {
    let (out, diags) = interpret("fn main() let a = 27 let b = a + 1 let c = b + 1 print(a, b, c) end", 0);
    assert!(!diags.has_errors(), "multi_stmt_27: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"27 28 29
");
}

#[test]
fn multi_stmt_28() {
    let (out, diags) = interpret("fn main() let a = 28 let b = a + 1 let c = b + 1 print(a, b, c) end", 0);
    assert!(!diags.has_errors(), "multi_stmt_28: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"28 29 30
");
}

#[test]
fn multi_stmt_29() {
    let (out, diags) = interpret("fn main() let a = 29 let b = a + 1 let c = b + 1 print(a, b, c) end", 0);
    assert!(!diags.has_errors(), "multi_stmt_29: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"29 30 31
");
}

#[test]
fn multi_stmt_30() {
    let (out, diags) = interpret("fn main() let a = 30 let b = a + 1 let c = b + 1 print(a, b, c) end", 0);
    assert!(!diags.has_errors(), "multi_stmt_30: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"30 31 32
");
}

#[test]
fn multi_stmt_31() {
    let (out, diags) = interpret("fn main() let a = 31 let b = a + 1 let c = b + 1 print(a, b, c) end", 0);
    assert!(!diags.has_errors(), "multi_stmt_31: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"31 32 33
");
}

#[test]
fn multi_stmt_32() {
    let (out, diags) = interpret("fn main() let a = 32 let b = a + 1 let c = b + 1 print(a, b, c) end", 0);
    assert!(!diags.has_errors(), "multi_stmt_32: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"32 33 34
");
}

#[test]
fn multi_stmt_33() {
    let (out, diags) = interpret("fn main() let a = 33 let b = a + 1 let c = b + 1 print(a, b, c) end", 0);
    assert!(!diags.has_errors(), "multi_stmt_33: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"33 34 35
");
}

#[test]
fn multi_stmt_34() {
    let (out, diags) = interpret("fn main() let a = 34 let b = a + 1 let c = b + 1 print(a, b, c) end", 0);
    assert!(!diags.has_errors(), "multi_stmt_34: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"34 35 36
");
}

#[test]
fn multi_stmt_35() {
    let (out, diags) = interpret("fn main() let a = 35 let b = a + 1 let c = b + 1 print(a, b, c) end", 0);
    assert!(!diags.has_errors(), "multi_stmt_35: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"35 36 37
");
}

#[test]
fn multi_stmt_36() {
    let (out, diags) = interpret("fn main() let a = 36 let b = a + 1 let c = b + 1 print(a, b, c) end", 0);
    assert!(!diags.has_errors(), "multi_stmt_36: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"36 37 38
");
}

#[test]
fn multi_stmt_37() {
    let (out, diags) = interpret("fn main() let a = 37 let b = a + 1 let c = b + 1 print(a, b, c) end", 0);
    assert!(!diags.has_errors(), "multi_stmt_37: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"37 38 39
");
}

#[test]
fn multi_stmt_38() {
    let (out, diags) = interpret("fn main() let a = 38 let b = a + 1 let c = b + 1 print(a, b, c) end", 0);
    assert!(!diags.has_errors(), "multi_stmt_38: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"38 39 40
");
}

#[test]
fn multi_stmt_39() {
    let (out, diags) = interpret("fn main() let a = 39 let b = a + 1 let c = b + 1 print(a, b, c) end", 0);
    assert!(!diags.has_errors(), "multi_stmt_39: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"39 40 41
");
}

#[test]
fn multi_stmt_40() {
    let (out, diags) = interpret("fn main() let a = 40 let b = a + 1 let c = b + 1 print(a, b, c) end", 0);
    assert!(!diags.has_errors(), "multi_stmt_40: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"40 41 42
");
}

#[test]
fn multi_stmt_41() {
    let (out, diags) = interpret("fn main() let a = 41 let b = a + 1 let c = b + 1 print(a, b, c) end", 0);
    assert!(!diags.has_errors(), "multi_stmt_41: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"41 42 43
");
}

#[test]
fn multi_stmt_42() {
    let (out, diags) = interpret("fn main() let a = 42 let b = a + 1 let c = b + 1 print(a, b, c) end", 0);
    assert!(!diags.has_errors(), "multi_stmt_42: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"42 43 44
");
}

#[test]
fn multi_stmt_43() {
    let (out, diags) = interpret("fn main() let a = 43 let b = a + 1 let c = b + 1 print(a, b, c) end", 0);
    assert!(!diags.has_errors(), "multi_stmt_43: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"43 44 45
");
}

#[test]
fn multi_stmt_44() {
    let (out, diags) = interpret("fn main() let a = 44 let b = a + 1 let c = b + 1 print(a, b, c) end", 0);
    assert!(!diags.has_errors(), "multi_stmt_44: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"44 45 46
");
}

#[test]
fn multi_stmt_45() {
    let (out, diags) = interpret("fn main() let a = 45 let b = a + 1 let c = b + 1 print(a, b, c) end", 0);
    assert!(!diags.has_errors(), "multi_stmt_45: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"45 46 47
");
}

#[test]
fn multi_stmt_46() {
    let (out, diags) = interpret("fn main() let a = 46 let b = a + 1 let c = b + 1 print(a, b, c) end", 0);
    assert!(!diags.has_errors(), "multi_stmt_46: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"46 47 48
");
}

#[test]
fn multi_stmt_47() {
    let (out, diags) = interpret("fn main() let a = 47 let b = a + 1 let c = b + 1 print(a, b, c) end", 0);
    assert!(!diags.has_errors(), "multi_stmt_47: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"47 48 49
");
}

#[test]
fn multi_stmt_48() {
    let (out, diags) = interpret("fn main() let a = 48 let b = a + 1 let c = b + 1 print(a, b, c) end", 0);
    assert!(!diags.has_errors(), "multi_stmt_48: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"48 49 50
");
}

#[test]
fn multi_stmt_49() {
    let (out, diags) = interpret("fn main() let a = 49 let b = a + 1 let c = b + 1 print(a, b, c) end", 0);
    assert!(!diags.has_errors(), "multi_stmt_49: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"49 50 51
");
}

