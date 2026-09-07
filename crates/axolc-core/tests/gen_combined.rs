// Owner: PascalElixir / axolrs (GitHub org)
// File: Auto-generated tests for combined arithmetic.

use axolc_core::interpret;

#[test]
fn combined_precedence_0() {
    let (out, diags) = interpret("fn main() print(0 + 0 * 2) end", 0);
    assert!(!diags.has_errors(), "combined_precedence_0: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"0
");
}

#[test]
fn combined_precedence_1() {
    let (out, diags) = interpret("fn main() print(1 + 1 * 2) end", 0);
    assert!(!diags.has_errors(), "combined_precedence_1: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"3
");
}

#[test]
fn combined_precedence_2() {
    let (out, diags) = interpret("fn main() print(2 + 2 * 2) end", 0);
    assert!(!diags.has_errors(), "combined_precedence_2: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"6
");
}

#[test]
fn combined_precedence_3() {
    let (out, diags) = interpret("fn main() print(3 + 3 * 2) end", 0);
    assert!(!diags.has_errors(), "combined_precedence_3: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"9
");
}

#[test]
fn combined_precedence_4() {
    let (out, diags) = interpret("fn main() print(4 + 4 * 2) end", 0);
    assert!(!diags.has_errors(), "combined_precedence_4: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"12
");
}

#[test]
fn combined_precedence_5() {
    let (out, diags) = interpret("fn main() print(5 + 5 * 2) end", 0);
    assert!(!diags.has_errors(), "combined_precedence_5: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"15
");
}

#[test]
fn combined_precedence_6() {
    let (out, diags) = interpret("fn main() print(6 + 6 * 2) end", 0);
    assert!(!diags.has_errors(), "combined_precedence_6: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"18
");
}

#[test]
fn combined_precedence_7() {
    let (out, diags) = interpret("fn main() print(7 + 7 * 2) end", 0);
    assert!(!diags.has_errors(), "combined_precedence_7: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"21
");
}

#[test]
fn combined_precedence_8() {
    let (out, diags) = interpret("fn main() print(8 + 8 * 2) end", 0);
    assert!(!diags.has_errors(), "combined_precedence_8: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"24
");
}

#[test]
fn combined_precedence_9() {
    let (out, diags) = interpret("fn main() print(9 + 9 * 2) end", 0);
    assert!(!diags.has_errors(), "combined_precedence_9: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"27
");
}

#[test]
fn combined_precedence_10() {
    let (out, diags) = interpret("fn main() print(10 + 10 * 2) end", 0);
    assert!(!diags.has_errors(), "combined_precedence_10: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"30
");
}

#[test]
fn combined_precedence_11() {
    let (out, diags) = interpret("fn main() print(11 + 11 * 2) end", 0);
    assert!(!diags.has_errors(), "combined_precedence_11: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"33
");
}

#[test]
fn combined_precedence_12() {
    let (out, diags) = interpret("fn main() print(12 + 12 * 2) end", 0);
    assert!(!diags.has_errors(), "combined_precedence_12: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"36
");
}

#[test]
fn combined_precedence_13() {
    let (out, diags) = interpret("fn main() print(13 + 13 * 2) end", 0);
    assert!(!diags.has_errors(), "combined_precedence_13: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"39
");
}

#[test]
fn combined_precedence_14() {
    let (out, diags) = interpret("fn main() print(14 + 14 * 2) end", 0);
    assert!(!diags.has_errors(), "combined_precedence_14: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"42
");
}

#[test]
fn combined_precedence_15() {
    let (out, diags) = interpret("fn main() print(15 + 15 * 2) end", 0);
    assert!(!diags.has_errors(), "combined_precedence_15: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"45
");
}

#[test]
fn combined_precedence_16() {
    let (out, diags) = interpret("fn main() print(16 + 16 * 2) end", 0);
    assert!(!diags.has_errors(), "combined_precedence_16: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"48
");
}

#[test]
fn combined_precedence_17() {
    let (out, diags) = interpret("fn main() print(17 + 17 * 2) end", 0);
    assert!(!diags.has_errors(), "combined_precedence_17: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"51
");
}

#[test]
fn combined_precedence_18() {
    let (out, diags) = interpret("fn main() print(18 + 18 * 2) end", 0);
    assert!(!diags.has_errors(), "combined_precedence_18: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"54
");
}

#[test]
fn combined_precedence_19() {
    let (out, diags) = interpret("fn main() print(19 + 19 * 2) end", 0);
    assert!(!diags.has_errors(), "combined_precedence_19: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"57
");
}

#[test]
fn combined_precedence_20() {
    let (out, diags) = interpret("fn main() print(20 + 20 * 2) end", 0);
    assert!(!diags.has_errors(), "combined_precedence_20: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"60
");
}

#[test]
fn combined_precedence_21() {
    let (out, diags) = interpret("fn main() print(21 + 21 * 2) end", 0);
    assert!(!diags.has_errors(), "combined_precedence_21: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"63
");
}

#[test]
fn combined_precedence_22() {
    let (out, diags) = interpret("fn main() print(22 + 22 * 2) end", 0);
    assert!(!diags.has_errors(), "combined_precedence_22: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"66
");
}

#[test]
fn combined_precedence_23() {
    let (out, diags) = interpret("fn main() print(23 + 23 * 2) end", 0);
    assert!(!diags.has_errors(), "combined_precedence_23: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"69
");
}

#[test]
fn combined_precedence_24() {
    let (out, diags) = interpret("fn main() print(24 + 24 * 2) end", 0);
    assert!(!diags.has_errors(), "combined_precedence_24: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"72
");
}

#[test]
fn combined_precedence_25() {
    let (out, diags) = interpret("fn main() print(25 + 25 * 2) end", 0);
    assert!(!diags.has_errors(), "combined_precedence_25: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"75
");
}

#[test]
fn combined_precedence_26() {
    let (out, diags) = interpret("fn main() print(26 + 26 * 2) end", 0);
    assert!(!diags.has_errors(), "combined_precedence_26: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"78
");
}

#[test]
fn combined_precedence_27() {
    let (out, diags) = interpret("fn main() print(27 + 27 * 2) end", 0);
    assert!(!diags.has_errors(), "combined_precedence_27: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"81
");
}

#[test]
fn combined_precedence_28() {
    let (out, diags) = interpret("fn main() print(28 + 28 * 2) end", 0);
    assert!(!diags.has_errors(), "combined_precedence_28: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"84
");
}

#[test]
fn combined_precedence_29() {
    let (out, diags) = interpret("fn main() print(29 + 29 * 2) end", 0);
    assert!(!diags.has_errors(), "combined_precedence_29: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"87
");
}

#[test]
fn combined_precedence_30() {
    let (out, diags) = interpret("fn main() print(30 + 30 * 2) end", 0);
    assert!(!diags.has_errors(), "combined_precedence_30: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"90
");
}

#[test]
fn combined_precedence_31() {
    let (out, diags) = interpret("fn main() print(31 + 31 * 2) end", 0);
    assert!(!diags.has_errors(), "combined_precedence_31: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"93
");
}

#[test]
fn combined_precedence_32() {
    let (out, diags) = interpret("fn main() print(32 + 32 * 2) end", 0);
    assert!(!diags.has_errors(), "combined_precedence_32: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"96
");
}

#[test]
fn combined_precedence_33() {
    let (out, diags) = interpret("fn main() print(33 + 33 * 2) end", 0);
    assert!(!diags.has_errors(), "combined_precedence_33: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"99
");
}

#[test]
fn combined_precedence_34() {
    let (out, diags) = interpret("fn main() print(34 + 34 * 2) end", 0);
    assert!(!diags.has_errors(), "combined_precedence_34: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"102
");
}

#[test]
fn combined_precedence_35() {
    let (out, diags) = interpret("fn main() print(35 + 35 * 2) end", 0);
    assert!(!diags.has_errors(), "combined_precedence_35: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"105
");
}

#[test]
fn combined_precedence_36() {
    let (out, diags) = interpret("fn main() print(36 + 36 * 2) end", 0);
    assert!(!diags.has_errors(), "combined_precedence_36: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"108
");
}

#[test]
fn combined_precedence_37() {
    let (out, diags) = interpret("fn main() print(37 + 37 * 2) end", 0);
    assert!(!diags.has_errors(), "combined_precedence_37: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"111
");
}

#[test]
fn combined_precedence_38() {
    let (out, diags) = interpret("fn main() print(38 + 38 * 2) end", 0);
    assert!(!diags.has_errors(), "combined_precedence_38: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"114
");
}

#[test]
fn combined_precedence_39() {
    let (out, diags) = interpret("fn main() print(39 + 39 * 2) end", 0);
    assert!(!diags.has_errors(), "combined_precedence_39: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"117
");
}

#[test]
fn combined_precedence_40() {
    let (out, diags) = interpret("fn main() print(40 + 40 * 2) end", 0);
    assert!(!diags.has_errors(), "combined_precedence_40: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"120
");
}

#[test]
fn combined_precedence_41() {
    let (out, diags) = interpret("fn main() print(41 + 41 * 2) end", 0);
    assert!(!diags.has_errors(), "combined_precedence_41: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"123
");
}

#[test]
fn combined_precedence_42() {
    let (out, diags) = interpret("fn main() print(42 + 42 * 2) end", 0);
    assert!(!diags.has_errors(), "combined_precedence_42: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"126
");
}

#[test]
fn combined_precedence_43() {
    let (out, diags) = interpret("fn main() print(43 + 43 * 2) end", 0);
    assert!(!diags.has_errors(), "combined_precedence_43: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"129
");
}

#[test]
fn combined_precedence_44() {
    let (out, diags) = interpret("fn main() print(44 + 44 * 2) end", 0);
    assert!(!diags.has_errors(), "combined_precedence_44: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"132
");
}

#[test]
fn combined_precedence_45() {
    let (out, diags) = interpret("fn main() print(45 + 45 * 2) end", 0);
    assert!(!diags.has_errors(), "combined_precedence_45: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"135
");
}

#[test]
fn combined_precedence_46() {
    let (out, diags) = interpret("fn main() print(46 + 46 * 2) end", 0);
    assert!(!diags.has_errors(), "combined_precedence_46: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"138
");
}

#[test]
fn combined_precedence_47() {
    let (out, diags) = interpret("fn main() print(47 + 47 * 2) end", 0);
    assert!(!diags.has_errors(), "combined_precedence_47: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"141
");
}

#[test]
fn combined_precedence_48() {
    let (out, diags) = interpret("fn main() print(48 + 48 * 2) end", 0);
    assert!(!diags.has_errors(), "combined_precedence_48: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"144
");
}

#[test]
fn combined_precedence_49() {
    let (out, diags) = interpret("fn main() print(49 + 49 * 2) end", 0);
    assert!(!diags.has_errors(), "combined_precedence_49: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"147
");
}

#[test]
fn combined_parens_0() {
    let (out, diags) = interpret("fn main() print((0 + 0) * 2) end", 0);
    assert!(!diags.has_errors(), "combined_parens_0: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"0
");
}

#[test]
fn combined_parens_1() {
    let (out, diags) = interpret("fn main() print((1 + 1) * 2) end", 0);
    assert!(!diags.has_errors(), "combined_parens_1: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"4
");
}

#[test]
fn combined_parens_2() {
    let (out, diags) = interpret("fn main() print((2 + 2) * 2) end", 0);
    assert!(!diags.has_errors(), "combined_parens_2: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"8
");
}

#[test]
fn combined_parens_3() {
    let (out, diags) = interpret("fn main() print((3 + 3) * 2) end", 0);
    assert!(!diags.has_errors(), "combined_parens_3: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"12
");
}

#[test]
fn combined_parens_4() {
    let (out, diags) = interpret("fn main() print((4 + 4) * 2) end", 0);
    assert!(!diags.has_errors(), "combined_parens_4: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"16
");
}

#[test]
fn combined_parens_5() {
    let (out, diags) = interpret("fn main() print((5 + 5) * 2) end", 0);
    assert!(!diags.has_errors(), "combined_parens_5: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"20
");
}

#[test]
fn combined_parens_6() {
    let (out, diags) = interpret("fn main() print((6 + 6) * 2) end", 0);
    assert!(!diags.has_errors(), "combined_parens_6: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"24
");
}

#[test]
fn combined_parens_7() {
    let (out, diags) = interpret("fn main() print((7 + 7) * 2) end", 0);
    assert!(!diags.has_errors(), "combined_parens_7: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"28
");
}

#[test]
fn combined_parens_8() {
    let (out, diags) = interpret("fn main() print((8 + 8) * 2) end", 0);
    assert!(!diags.has_errors(), "combined_parens_8: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"32
");
}

#[test]
fn combined_parens_9() {
    let (out, diags) = interpret("fn main() print((9 + 9) * 2) end", 0);
    assert!(!diags.has_errors(), "combined_parens_9: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"36
");
}

#[test]
fn combined_parens_10() {
    let (out, diags) = interpret("fn main() print((10 + 10) * 2) end", 0);
    assert!(!diags.has_errors(), "combined_parens_10: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"40
");
}

#[test]
fn combined_parens_11() {
    let (out, diags) = interpret("fn main() print((11 + 11) * 2) end", 0);
    assert!(!diags.has_errors(), "combined_parens_11: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"44
");
}

#[test]
fn combined_parens_12() {
    let (out, diags) = interpret("fn main() print((12 + 12) * 2) end", 0);
    assert!(!diags.has_errors(), "combined_parens_12: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"48
");
}

#[test]
fn combined_parens_13() {
    let (out, diags) = interpret("fn main() print((13 + 13) * 2) end", 0);
    assert!(!diags.has_errors(), "combined_parens_13: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"52
");
}

#[test]
fn combined_parens_14() {
    let (out, diags) = interpret("fn main() print((14 + 14) * 2) end", 0);
    assert!(!diags.has_errors(), "combined_parens_14: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"56
");
}

#[test]
fn combined_parens_15() {
    let (out, diags) = interpret("fn main() print((15 + 15) * 2) end", 0);
    assert!(!diags.has_errors(), "combined_parens_15: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"60
");
}

#[test]
fn combined_parens_16() {
    let (out, diags) = interpret("fn main() print((16 + 16) * 2) end", 0);
    assert!(!diags.has_errors(), "combined_parens_16: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"64
");
}

#[test]
fn combined_parens_17() {
    let (out, diags) = interpret("fn main() print((17 + 17) * 2) end", 0);
    assert!(!diags.has_errors(), "combined_parens_17: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"68
");
}

#[test]
fn combined_parens_18() {
    let (out, diags) = interpret("fn main() print((18 + 18) * 2) end", 0);
    assert!(!diags.has_errors(), "combined_parens_18: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"72
");
}

#[test]
fn combined_parens_19() {
    let (out, diags) = interpret("fn main() print((19 + 19) * 2) end", 0);
    assert!(!diags.has_errors(), "combined_parens_19: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"76
");
}

#[test]
fn combined_parens_20() {
    let (out, diags) = interpret("fn main() print((20 + 20) * 2) end", 0);
    assert!(!diags.has_errors(), "combined_parens_20: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"80
");
}

#[test]
fn combined_parens_21() {
    let (out, diags) = interpret("fn main() print((21 + 21) * 2) end", 0);
    assert!(!diags.has_errors(), "combined_parens_21: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"84
");
}

#[test]
fn combined_parens_22() {
    let (out, diags) = interpret("fn main() print((22 + 22) * 2) end", 0);
    assert!(!diags.has_errors(), "combined_parens_22: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"88
");
}

#[test]
fn combined_parens_23() {
    let (out, diags) = interpret("fn main() print((23 + 23) * 2) end", 0);
    assert!(!diags.has_errors(), "combined_parens_23: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"92
");
}

#[test]
fn combined_parens_24() {
    let (out, diags) = interpret("fn main() print((24 + 24) * 2) end", 0);
    assert!(!diags.has_errors(), "combined_parens_24: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"96
");
}

#[test]
fn combined_parens_25() {
    let (out, diags) = interpret("fn main() print((25 + 25) * 2) end", 0);
    assert!(!diags.has_errors(), "combined_parens_25: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"100
");
}

#[test]
fn combined_parens_26() {
    let (out, diags) = interpret("fn main() print((26 + 26) * 2) end", 0);
    assert!(!diags.has_errors(), "combined_parens_26: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"104
");
}

#[test]
fn combined_parens_27() {
    let (out, diags) = interpret("fn main() print((27 + 27) * 2) end", 0);
    assert!(!diags.has_errors(), "combined_parens_27: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"108
");
}

#[test]
fn combined_parens_28() {
    let (out, diags) = interpret("fn main() print((28 + 28) * 2) end", 0);
    assert!(!diags.has_errors(), "combined_parens_28: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"112
");
}

#[test]
fn combined_parens_29() {
    let (out, diags) = interpret("fn main() print((29 + 29) * 2) end", 0);
    assert!(!diags.has_errors(), "combined_parens_29: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"116
");
}

#[test]
fn combined_parens_30() {
    let (out, diags) = interpret("fn main() print((30 + 30) * 2) end", 0);
    assert!(!diags.has_errors(), "combined_parens_30: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"120
");
}

#[test]
fn combined_parens_31() {
    let (out, diags) = interpret("fn main() print((31 + 31) * 2) end", 0);
    assert!(!diags.has_errors(), "combined_parens_31: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"124
");
}

#[test]
fn combined_parens_32() {
    let (out, diags) = interpret("fn main() print((32 + 32) * 2) end", 0);
    assert!(!diags.has_errors(), "combined_parens_32: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"128
");
}

#[test]
fn combined_parens_33() {
    let (out, diags) = interpret("fn main() print((33 + 33) * 2) end", 0);
    assert!(!diags.has_errors(), "combined_parens_33: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"132
");
}

#[test]
fn combined_parens_34() {
    let (out, diags) = interpret("fn main() print((34 + 34) * 2) end", 0);
    assert!(!diags.has_errors(), "combined_parens_34: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"136
");
}

#[test]
fn combined_parens_35() {
    let (out, diags) = interpret("fn main() print((35 + 35) * 2) end", 0);
    assert!(!diags.has_errors(), "combined_parens_35: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"140
");
}

#[test]
fn combined_parens_36() {
    let (out, diags) = interpret("fn main() print((36 + 36) * 2) end", 0);
    assert!(!diags.has_errors(), "combined_parens_36: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"144
");
}

#[test]
fn combined_parens_37() {
    let (out, diags) = interpret("fn main() print((37 + 37) * 2) end", 0);
    assert!(!diags.has_errors(), "combined_parens_37: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"148
");
}

#[test]
fn combined_parens_38() {
    let (out, diags) = interpret("fn main() print((38 + 38) * 2) end", 0);
    assert!(!diags.has_errors(), "combined_parens_38: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"152
");
}

#[test]
fn combined_parens_39() {
    let (out, diags) = interpret("fn main() print((39 + 39) * 2) end", 0);
    assert!(!diags.has_errors(), "combined_parens_39: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"156
");
}

#[test]
fn combined_parens_40() {
    let (out, diags) = interpret("fn main() print((40 + 40) * 2) end", 0);
    assert!(!diags.has_errors(), "combined_parens_40: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"160
");
}

#[test]
fn combined_parens_41() {
    let (out, diags) = interpret("fn main() print((41 + 41) * 2) end", 0);
    assert!(!diags.has_errors(), "combined_parens_41: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"164
");
}

#[test]
fn combined_parens_42() {
    let (out, diags) = interpret("fn main() print((42 + 42) * 2) end", 0);
    assert!(!diags.has_errors(), "combined_parens_42: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"168
");
}

#[test]
fn combined_parens_43() {
    let (out, diags) = interpret("fn main() print((43 + 43) * 2) end", 0);
    assert!(!diags.has_errors(), "combined_parens_43: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"172
");
}

#[test]
fn combined_parens_44() {
    let (out, diags) = interpret("fn main() print((44 + 44) * 2) end", 0);
    assert!(!diags.has_errors(), "combined_parens_44: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"176
");
}

#[test]
fn combined_parens_45() {
    let (out, diags) = interpret("fn main() print((45 + 45) * 2) end", 0);
    assert!(!diags.has_errors(), "combined_parens_45: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"180
");
}

#[test]
fn combined_parens_46() {
    let (out, diags) = interpret("fn main() print((46 + 46) * 2) end", 0);
    assert!(!diags.has_errors(), "combined_parens_46: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"184
");
}

#[test]
fn combined_parens_47() {
    let (out, diags) = interpret("fn main() print((47 + 47) * 2) end", 0);
    assert!(!diags.has_errors(), "combined_parens_47: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"188
");
}

#[test]
fn combined_parens_48() {
    let (out, diags) = interpret("fn main() print((48 + 48) * 2) end", 0);
    assert!(!diags.has_errors(), "combined_parens_48: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"192
");
}

#[test]
fn combined_parens_49() {
    let (out, diags) = interpret("fn main() print((49 + 49) * 2) end", 0);
    assert!(!diags.has_errors(), "combined_parens_49: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"196
");
}

#[test]
fn combined_mixed_0() {
    let (out, diags) = interpret("fn main() print(0 * 2 + 0 * 3) end", 0);
    assert!(!diags.has_errors(), "combined_mixed_0: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"0
");
}

#[test]
fn combined_mixed_1() {
    let (out, diags) = interpret("fn main() print(1 * 2 + 1 * 3) end", 0);
    assert!(!diags.has_errors(), "combined_mixed_1: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"5
");
}

#[test]
fn combined_mixed_2() {
    let (out, diags) = interpret("fn main() print(2 * 2 + 2 * 3) end", 0);
    assert!(!diags.has_errors(), "combined_mixed_2: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"10
");
}

#[test]
fn combined_mixed_3() {
    let (out, diags) = interpret("fn main() print(3 * 2 + 3 * 3) end", 0);
    assert!(!diags.has_errors(), "combined_mixed_3: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"15
");
}

#[test]
fn combined_mixed_4() {
    let (out, diags) = interpret("fn main() print(4 * 2 + 4 * 3) end", 0);
    assert!(!diags.has_errors(), "combined_mixed_4: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"20
");
}

#[test]
fn combined_mixed_5() {
    let (out, diags) = interpret("fn main() print(5 * 2 + 5 * 3) end", 0);
    assert!(!diags.has_errors(), "combined_mixed_5: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"25
");
}

#[test]
fn combined_mixed_6() {
    let (out, diags) = interpret("fn main() print(6 * 2 + 6 * 3) end", 0);
    assert!(!diags.has_errors(), "combined_mixed_6: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"30
");
}

#[test]
fn combined_mixed_7() {
    let (out, diags) = interpret("fn main() print(7 * 2 + 7 * 3) end", 0);
    assert!(!diags.has_errors(), "combined_mixed_7: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"35
");
}

#[test]
fn combined_mixed_8() {
    let (out, diags) = interpret("fn main() print(8 * 2 + 8 * 3) end", 0);
    assert!(!diags.has_errors(), "combined_mixed_8: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"40
");
}

#[test]
fn combined_mixed_9() {
    let (out, diags) = interpret("fn main() print(9 * 2 + 9 * 3) end", 0);
    assert!(!diags.has_errors(), "combined_mixed_9: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"45
");
}

#[test]
fn combined_mixed_10() {
    let (out, diags) = interpret("fn main() print(10 * 2 + 10 * 3) end", 0);
    assert!(!diags.has_errors(), "combined_mixed_10: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"50
");
}

#[test]
fn combined_mixed_11() {
    let (out, diags) = interpret("fn main() print(11 * 2 + 11 * 3) end", 0);
    assert!(!diags.has_errors(), "combined_mixed_11: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"55
");
}

#[test]
fn combined_mixed_12() {
    let (out, diags) = interpret("fn main() print(12 * 2 + 12 * 3) end", 0);
    assert!(!diags.has_errors(), "combined_mixed_12: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"60
");
}

#[test]
fn combined_mixed_13() {
    let (out, diags) = interpret("fn main() print(13 * 2 + 13 * 3) end", 0);
    assert!(!diags.has_errors(), "combined_mixed_13: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"65
");
}

#[test]
fn combined_mixed_14() {
    let (out, diags) = interpret("fn main() print(14 * 2 + 14 * 3) end", 0);
    assert!(!diags.has_errors(), "combined_mixed_14: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"70
");
}

#[test]
fn combined_mixed_15() {
    let (out, diags) = interpret("fn main() print(15 * 2 + 15 * 3) end", 0);
    assert!(!diags.has_errors(), "combined_mixed_15: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"75
");
}

#[test]
fn combined_mixed_16() {
    let (out, diags) = interpret("fn main() print(16 * 2 + 16 * 3) end", 0);
    assert!(!diags.has_errors(), "combined_mixed_16: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"80
");
}

#[test]
fn combined_mixed_17() {
    let (out, diags) = interpret("fn main() print(17 * 2 + 17 * 3) end", 0);
    assert!(!diags.has_errors(), "combined_mixed_17: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"85
");
}

#[test]
fn combined_mixed_18() {
    let (out, diags) = interpret("fn main() print(18 * 2 + 18 * 3) end", 0);
    assert!(!diags.has_errors(), "combined_mixed_18: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"90
");
}

#[test]
fn combined_mixed_19() {
    let (out, diags) = interpret("fn main() print(19 * 2 + 19 * 3) end", 0);
    assert!(!diags.has_errors(), "combined_mixed_19: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"95
");
}

#[test]
fn combined_mixed_20() {
    let (out, diags) = interpret("fn main() print(20 * 2 + 20 * 3) end", 0);
    assert!(!diags.has_errors(), "combined_mixed_20: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"100
");
}

#[test]
fn combined_mixed_21() {
    let (out, diags) = interpret("fn main() print(21 * 2 + 21 * 3) end", 0);
    assert!(!diags.has_errors(), "combined_mixed_21: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"105
");
}

#[test]
fn combined_mixed_22() {
    let (out, diags) = interpret("fn main() print(22 * 2 + 22 * 3) end", 0);
    assert!(!diags.has_errors(), "combined_mixed_22: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"110
");
}

#[test]
fn combined_mixed_23() {
    let (out, diags) = interpret("fn main() print(23 * 2 + 23 * 3) end", 0);
    assert!(!diags.has_errors(), "combined_mixed_23: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"115
");
}

#[test]
fn combined_mixed_24() {
    let (out, diags) = interpret("fn main() print(24 * 2 + 24 * 3) end", 0);
    assert!(!diags.has_errors(), "combined_mixed_24: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"120
");
}

#[test]
fn combined_mixed_25() {
    let (out, diags) = interpret("fn main() print(25 * 2 + 25 * 3) end", 0);
    assert!(!diags.has_errors(), "combined_mixed_25: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"125
");
}

#[test]
fn combined_mixed_26() {
    let (out, diags) = interpret("fn main() print(26 * 2 + 26 * 3) end", 0);
    assert!(!diags.has_errors(), "combined_mixed_26: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"130
");
}

#[test]
fn combined_mixed_27() {
    let (out, diags) = interpret("fn main() print(27 * 2 + 27 * 3) end", 0);
    assert!(!diags.has_errors(), "combined_mixed_27: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"135
");
}

#[test]
fn combined_mixed_28() {
    let (out, diags) = interpret("fn main() print(28 * 2 + 28 * 3) end", 0);
    assert!(!diags.has_errors(), "combined_mixed_28: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"140
");
}

#[test]
fn combined_mixed_29() {
    let (out, diags) = interpret("fn main() print(29 * 2 + 29 * 3) end", 0);
    assert!(!diags.has_errors(), "combined_mixed_29: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"145
");
}

#[test]
fn combined_mixed_30() {
    let (out, diags) = interpret("fn main() print(30 * 2 + 30 * 3) end", 0);
    assert!(!diags.has_errors(), "combined_mixed_30: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"150
");
}

#[test]
fn combined_mixed_31() {
    let (out, diags) = interpret("fn main() print(31 * 2 + 31 * 3) end", 0);
    assert!(!diags.has_errors(), "combined_mixed_31: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"155
");
}

#[test]
fn combined_mixed_32() {
    let (out, diags) = interpret("fn main() print(32 * 2 + 32 * 3) end", 0);
    assert!(!diags.has_errors(), "combined_mixed_32: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"160
");
}

#[test]
fn combined_mixed_33() {
    let (out, diags) = interpret("fn main() print(33 * 2 + 33 * 3) end", 0);
    assert!(!diags.has_errors(), "combined_mixed_33: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"165
");
}

#[test]
fn combined_mixed_34() {
    let (out, diags) = interpret("fn main() print(34 * 2 + 34 * 3) end", 0);
    assert!(!diags.has_errors(), "combined_mixed_34: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"170
");
}

#[test]
fn combined_mixed_35() {
    let (out, diags) = interpret("fn main() print(35 * 2 + 35 * 3) end", 0);
    assert!(!diags.has_errors(), "combined_mixed_35: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"175
");
}

#[test]
fn combined_mixed_36() {
    let (out, diags) = interpret("fn main() print(36 * 2 + 36 * 3) end", 0);
    assert!(!diags.has_errors(), "combined_mixed_36: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"180
");
}

#[test]
fn combined_mixed_37() {
    let (out, diags) = interpret("fn main() print(37 * 2 + 37 * 3) end", 0);
    assert!(!diags.has_errors(), "combined_mixed_37: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"185
");
}

#[test]
fn combined_mixed_38() {
    let (out, diags) = interpret("fn main() print(38 * 2 + 38 * 3) end", 0);
    assert!(!diags.has_errors(), "combined_mixed_38: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"190
");
}

#[test]
fn combined_mixed_39() {
    let (out, diags) = interpret("fn main() print(39 * 2 + 39 * 3) end", 0);
    assert!(!diags.has_errors(), "combined_mixed_39: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"195
");
}

#[test]
fn combined_mixed_40() {
    let (out, diags) = interpret("fn main() print(40 * 2 + 40 * 3) end", 0);
    assert!(!diags.has_errors(), "combined_mixed_40: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"200
");
}

#[test]
fn combined_mixed_41() {
    let (out, diags) = interpret("fn main() print(41 * 2 + 41 * 3) end", 0);
    assert!(!diags.has_errors(), "combined_mixed_41: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"205
");
}

#[test]
fn combined_mixed_42() {
    let (out, diags) = interpret("fn main() print(42 * 2 + 42 * 3) end", 0);
    assert!(!diags.has_errors(), "combined_mixed_42: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"210
");
}

#[test]
fn combined_mixed_43() {
    let (out, diags) = interpret("fn main() print(43 * 2 + 43 * 3) end", 0);
    assert!(!diags.has_errors(), "combined_mixed_43: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"215
");
}

#[test]
fn combined_mixed_44() {
    let (out, diags) = interpret("fn main() print(44 * 2 + 44 * 3) end", 0);
    assert!(!diags.has_errors(), "combined_mixed_44: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"220
");
}

#[test]
fn combined_mixed_45() {
    let (out, diags) = interpret("fn main() print(45 * 2 + 45 * 3) end", 0);
    assert!(!diags.has_errors(), "combined_mixed_45: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"225
");
}

#[test]
fn combined_mixed_46() {
    let (out, diags) = interpret("fn main() print(46 * 2 + 46 * 3) end", 0);
    assert!(!diags.has_errors(), "combined_mixed_46: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"230
");
}

#[test]
fn combined_mixed_47() {
    let (out, diags) = interpret("fn main() print(47 * 2 + 47 * 3) end", 0);
    assert!(!diags.has_errors(), "combined_mixed_47: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"235
");
}

#[test]
fn combined_mixed_48() {
    let (out, diags) = interpret("fn main() print(48 * 2 + 48 * 3) end", 0);
    assert!(!diags.has_errors(), "combined_mixed_48: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"240
");
}

#[test]
fn combined_mixed_49() {
    let (out, diags) = interpret("fn main() print(49 * 2 + 49 * 3) end", 0);
    assert!(!diags.has_errors(), "combined_mixed_49: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"245
");
}

