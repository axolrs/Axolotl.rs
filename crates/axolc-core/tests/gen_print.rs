// Owner: PascalElixir / axolrs (GitHub org)
// File: Auto-generated tests for print with multiple args.

use axolc_core::interpret;

#[test]
fn print_two_0() {
    let (out, diags) = interpret("fn main() print(0, 1) end", 0);
    assert!(!diags.has_errors(), "print_two_0: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"0 1
");
}

#[test]
fn print_two_1() {
    let (out, diags) = interpret("fn main() print(1, 2) end", 0);
    assert!(!diags.has_errors(), "print_two_1: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"1 2
");
}

#[test]
fn print_two_2() {
    let (out, diags) = interpret("fn main() print(2, 3) end", 0);
    assert!(!diags.has_errors(), "print_two_2: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"2 3
");
}

#[test]
fn print_two_3() {
    let (out, diags) = interpret("fn main() print(3, 4) end", 0);
    assert!(!diags.has_errors(), "print_two_3: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"3 4
");
}

#[test]
fn print_two_4() {
    let (out, diags) = interpret("fn main() print(4, 5) end", 0);
    assert!(!diags.has_errors(), "print_two_4: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"4 5
");
}

#[test]
fn print_two_5() {
    let (out, diags) = interpret("fn main() print(5, 6) end", 0);
    assert!(!diags.has_errors(), "print_two_5: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"5 6
");
}

#[test]
fn print_two_6() {
    let (out, diags) = interpret("fn main() print(6, 7) end", 0);
    assert!(!diags.has_errors(), "print_two_6: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"6 7
");
}

#[test]
fn print_two_7() {
    let (out, diags) = interpret("fn main() print(7, 8) end", 0);
    assert!(!diags.has_errors(), "print_two_7: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"7 8
");
}

#[test]
fn print_two_8() {
    let (out, diags) = interpret("fn main() print(8, 9) end", 0);
    assert!(!diags.has_errors(), "print_two_8: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"8 9
");
}

#[test]
fn print_two_9() {
    let (out, diags) = interpret("fn main() print(9, 10) end", 0);
    assert!(!diags.has_errors(), "print_two_9: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"9 10
");
}

#[test]
fn print_two_10() {
    let (out, diags) = interpret("fn main() print(10, 11) end", 0);
    assert!(!diags.has_errors(), "print_two_10: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"10 11
");
}

#[test]
fn print_two_11() {
    let (out, diags) = interpret("fn main() print(11, 12) end", 0);
    assert!(!diags.has_errors(), "print_two_11: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"11 12
");
}

#[test]
fn print_two_12() {
    let (out, diags) = interpret("fn main() print(12, 13) end", 0);
    assert!(!diags.has_errors(), "print_two_12: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"12 13
");
}

#[test]
fn print_two_13() {
    let (out, diags) = interpret("fn main() print(13, 14) end", 0);
    assert!(!diags.has_errors(), "print_two_13: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"13 14
");
}

#[test]
fn print_two_14() {
    let (out, diags) = interpret("fn main() print(14, 15) end", 0);
    assert!(!diags.has_errors(), "print_two_14: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"14 15
");
}

#[test]
fn print_two_15() {
    let (out, diags) = interpret("fn main() print(15, 16) end", 0);
    assert!(!diags.has_errors(), "print_two_15: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"15 16
");
}

#[test]
fn print_two_16() {
    let (out, diags) = interpret("fn main() print(16, 17) end", 0);
    assert!(!diags.has_errors(), "print_two_16: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"16 17
");
}

#[test]
fn print_two_17() {
    let (out, diags) = interpret("fn main() print(17, 18) end", 0);
    assert!(!diags.has_errors(), "print_two_17: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"17 18
");
}

#[test]
fn print_two_18() {
    let (out, diags) = interpret("fn main() print(18, 19) end", 0);
    assert!(!diags.has_errors(), "print_two_18: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"18 19
");
}

#[test]
fn print_two_19() {
    let (out, diags) = interpret("fn main() print(19, 20) end", 0);
    assert!(!diags.has_errors(), "print_two_19: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"19 20
");
}

#[test]
fn print_two_20() {
    let (out, diags) = interpret("fn main() print(20, 21) end", 0);
    assert!(!diags.has_errors(), "print_two_20: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"20 21
");
}

#[test]
fn print_two_21() {
    let (out, diags) = interpret("fn main() print(21, 22) end", 0);
    assert!(!diags.has_errors(), "print_two_21: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"21 22
");
}

#[test]
fn print_two_22() {
    let (out, diags) = interpret("fn main() print(22, 23) end", 0);
    assert!(!diags.has_errors(), "print_two_22: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"22 23
");
}

#[test]
fn print_two_23() {
    let (out, diags) = interpret("fn main() print(23, 24) end", 0);
    assert!(!diags.has_errors(), "print_two_23: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"23 24
");
}

#[test]
fn print_two_24() {
    let (out, diags) = interpret("fn main() print(24, 25) end", 0);
    assert!(!diags.has_errors(), "print_two_24: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"24 25
");
}

#[test]
fn print_two_25() {
    let (out, diags) = interpret("fn main() print(25, 26) end", 0);
    assert!(!diags.has_errors(), "print_two_25: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"25 26
");
}

#[test]
fn print_two_26() {
    let (out, diags) = interpret("fn main() print(26, 27) end", 0);
    assert!(!diags.has_errors(), "print_two_26: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"26 27
");
}

#[test]
fn print_two_27() {
    let (out, diags) = interpret("fn main() print(27, 28) end", 0);
    assert!(!diags.has_errors(), "print_two_27: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"27 28
");
}

#[test]
fn print_two_28() {
    let (out, diags) = interpret("fn main() print(28, 29) end", 0);
    assert!(!diags.has_errors(), "print_two_28: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"28 29
");
}

#[test]
fn print_two_29() {
    let (out, diags) = interpret("fn main() print(29, 30) end", 0);
    assert!(!diags.has_errors(), "print_two_29: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"29 30
");
}

#[test]
fn print_two_30() {
    let (out, diags) = interpret("fn main() print(30, 31) end", 0);
    assert!(!diags.has_errors(), "print_two_30: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"30 31
");
}

#[test]
fn print_two_31() {
    let (out, diags) = interpret("fn main() print(31, 32) end", 0);
    assert!(!diags.has_errors(), "print_two_31: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"31 32
");
}

#[test]
fn print_two_32() {
    let (out, diags) = interpret("fn main() print(32, 33) end", 0);
    assert!(!diags.has_errors(), "print_two_32: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"32 33
");
}

#[test]
fn print_two_33() {
    let (out, diags) = interpret("fn main() print(33, 34) end", 0);
    assert!(!diags.has_errors(), "print_two_33: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"33 34
");
}

#[test]
fn print_two_34() {
    let (out, diags) = interpret("fn main() print(34, 35) end", 0);
    assert!(!diags.has_errors(), "print_two_34: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"34 35
");
}

#[test]
fn print_two_35() {
    let (out, diags) = interpret("fn main() print(35, 36) end", 0);
    assert!(!diags.has_errors(), "print_two_35: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"35 36
");
}

#[test]
fn print_two_36() {
    let (out, diags) = interpret("fn main() print(36, 37) end", 0);
    assert!(!diags.has_errors(), "print_two_36: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"36 37
");
}

#[test]
fn print_two_37() {
    let (out, diags) = interpret("fn main() print(37, 38) end", 0);
    assert!(!diags.has_errors(), "print_two_37: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"37 38
");
}

#[test]
fn print_two_38() {
    let (out, diags) = interpret("fn main() print(38, 39) end", 0);
    assert!(!diags.has_errors(), "print_two_38: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"38 39
");
}

#[test]
fn print_two_39() {
    let (out, diags) = interpret("fn main() print(39, 40) end", 0);
    assert!(!diags.has_errors(), "print_two_39: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"39 40
");
}

#[test]
fn print_two_40() {
    let (out, diags) = interpret("fn main() print(40, 41) end", 0);
    assert!(!diags.has_errors(), "print_two_40: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"40 41
");
}

#[test]
fn print_two_41() {
    let (out, diags) = interpret("fn main() print(41, 42) end", 0);
    assert!(!diags.has_errors(), "print_two_41: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"41 42
");
}

#[test]
fn print_two_42() {
    let (out, diags) = interpret("fn main() print(42, 43) end", 0);
    assert!(!diags.has_errors(), "print_two_42: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"42 43
");
}

#[test]
fn print_two_43() {
    let (out, diags) = interpret("fn main() print(43, 44) end", 0);
    assert!(!diags.has_errors(), "print_two_43: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"43 44
");
}

#[test]
fn print_two_44() {
    let (out, diags) = interpret("fn main() print(44, 45) end", 0);
    assert!(!diags.has_errors(), "print_two_44: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"44 45
");
}

#[test]
fn print_two_45() {
    let (out, diags) = interpret("fn main() print(45, 46) end", 0);
    assert!(!diags.has_errors(), "print_two_45: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"45 46
");
}

#[test]
fn print_two_46() {
    let (out, diags) = interpret("fn main() print(46, 47) end", 0);
    assert!(!diags.has_errors(), "print_two_46: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"46 47
");
}

#[test]
fn print_two_47() {
    let (out, diags) = interpret("fn main() print(47, 48) end", 0);
    assert!(!diags.has_errors(), "print_two_47: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"47 48
");
}

#[test]
fn print_two_48() {
    let (out, diags) = interpret("fn main() print(48, 49) end", 0);
    assert!(!diags.has_errors(), "print_two_48: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"48 49
");
}

#[test]
fn print_two_49() {
    let (out, diags) = interpret("fn main() print(49, 50) end", 0);
    assert!(!diags.has_errors(), "print_two_49: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"49 50
");
}

#[test]
fn print_three_0() {
    let (out, diags) = interpret("fn main() print(0, 1, 2) end", 0);
    assert!(!diags.has_errors(), "print_three_0: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"0 1 2
");
}

#[test]
fn print_three_1() {
    let (out, diags) = interpret("fn main() print(1, 2, 3) end", 0);
    assert!(!diags.has_errors(), "print_three_1: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"1 2 3
");
}

#[test]
fn print_three_2() {
    let (out, diags) = interpret("fn main() print(2, 3, 4) end", 0);
    assert!(!diags.has_errors(), "print_three_2: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"2 3 4
");
}

#[test]
fn print_three_3() {
    let (out, diags) = interpret("fn main() print(3, 4, 5) end", 0);
    assert!(!diags.has_errors(), "print_three_3: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"3 4 5
");
}

#[test]
fn print_three_4() {
    let (out, diags) = interpret("fn main() print(4, 5, 6) end", 0);
    assert!(!diags.has_errors(), "print_three_4: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"4 5 6
");
}

#[test]
fn print_three_5() {
    let (out, diags) = interpret("fn main() print(5, 6, 7) end", 0);
    assert!(!diags.has_errors(), "print_three_5: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"5 6 7
");
}

#[test]
fn print_three_6() {
    let (out, diags) = interpret("fn main() print(6, 7, 8) end", 0);
    assert!(!diags.has_errors(), "print_three_6: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"6 7 8
");
}

#[test]
fn print_three_7() {
    let (out, diags) = interpret("fn main() print(7, 8, 9) end", 0);
    assert!(!diags.has_errors(), "print_three_7: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"7 8 9
");
}

#[test]
fn print_three_8() {
    let (out, diags) = interpret("fn main() print(8, 9, 10) end", 0);
    assert!(!diags.has_errors(), "print_three_8: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"8 9 10
");
}

#[test]
fn print_three_9() {
    let (out, diags) = interpret("fn main() print(9, 10, 11) end", 0);
    assert!(!diags.has_errors(), "print_three_9: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"9 10 11
");
}

#[test]
fn print_three_10() {
    let (out, diags) = interpret("fn main() print(10, 11, 12) end", 0);
    assert!(!diags.has_errors(), "print_three_10: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"10 11 12
");
}

#[test]
fn print_three_11() {
    let (out, diags) = interpret("fn main() print(11, 12, 13) end", 0);
    assert!(!diags.has_errors(), "print_three_11: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"11 12 13
");
}

#[test]
fn print_three_12() {
    let (out, diags) = interpret("fn main() print(12, 13, 14) end", 0);
    assert!(!diags.has_errors(), "print_three_12: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"12 13 14
");
}

#[test]
fn print_three_13() {
    let (out, diags) = interpret("fn main() print(13, 14, 15) end", 0);
    assert!(!diags.has_errors(), "print_three_13: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"13 14 15
");
}

#[test]
fn print_three_14() {
    let (out, diags) = interpret("fn main() print(14, 15, 16) end", 0);
    assert!(!diags.has_errors(), "print_three_14: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"14 15 16
");
}

#[test]
fn print_three_15() {
    let (out, diags) = interpret("fn main() print(15, 16, 17) end", 0);
    assert!(!diags.has_errors(), "print_three_15: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"15 16 17
");
}

#[test]
fn print_three_16() {
    let (out, diags) = interpret("fn main() print(16, 17, 18) end", 0);
    assert!(!diags.has_errors(), "print_three_16: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"16 17 18
");
}

#[test]
fn print_three_17() {
    let (out, diags) = interpret("fn main() print(17, 18, 19) end", 0);
    assert!(!diags.has_errors(), "print_three_17: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"17 18 19
");
}

#[test]
fn print_three_18() {
    let (out, diags) = interpret("fn main() print(18, 19, 20) end", 0);
    assert!(!diags.has_errors(), "print_three_18: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"18 19 20
");
}

#[test]
fn print_three_19() {
    let (out, diags) = interpret("fn main() print(19, 20, 21) end", 0);
    assert!(!diags.has_errors(), "print_three_19: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"19 20 21
");
}

#[test]
fn print_three_20() {
    let (out, diags) = interpret("fn main() print(20, 21, 22) end", 0);
    assert!(!diags.has_errors(), "print_three_20: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"20 21 22
");
}

#[test]
fn print_three_21() {
    let (out, diags) = interpret("fn main() print(21, 22, 23) end", 0);
    assert!(!diags.has_errors(), "print_three_21: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"21 22 23
");
}

#[test]
fn print_three_22() {
    let (out, diags) = interpret("fn main() print(22, 23, 24) end", 0);
    assert!(!diags.has_errors(), "print_three_22: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"22 23 24
");
}

#[test]
fn print_three_23() {
    let (out, diags) = interpret("fn main() print(23, 24, 25) end", 0);
    assert!(!diags.has_errors(), "print_three_23: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"23 24 25
");
}

#[test]
fn print_three_24() {
    let (out, diags) = interpret("fn main() print(24, 25, 26) end", 0);
    assert!(!diags.has_errors(), "print_three_24: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"24 25 26
");
}

#[test]
fn print_three_25() {
    let (out, diags) = interpret("fn main() print(25, 26, 27) end", 0);
    assert!(!diags.has_errors(), "print_three_25: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"25 26 27
");
}

#[test]
fn print_three_26() {
    let (out, diags) = interpret("fn main() print(26, 27, 28) end", 0);
    assert!(!diags.has_errors(), "print_three_26: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"26 27 28
");
}

#[test]
fn print_three_27() {
    let (out, diags) = interpret("fn main() print(27, 28, 29) end", 0);
    assert!(!diags.has_errors(), "print_three_27: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"27 28 29
");
}

#[test]
fn print_three_28() {
    let (out, diags) = interpret("fn main() print(28, 29, 30) end", 0);
    assert!(!diags.has_errors(), "print_three_28: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"28 29 30
");
}

#[test]
fn print_three_29() {
    let (out, diags) = interpret("fn main() print(29, 30, 31) end", 0);
    assert!(!diags.has_errors(), "print_three_29: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"29 30 31
");
}

