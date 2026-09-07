// Owner: PascalElixir / axolrs (GitHub org)
// File: Auto-generated tests for comparison operators.

use axolc_core::interpret;

#[test]
fn lt_0() {
    let (out, diags) = interpret("fn main() print(0 < 5) end", 0);
    assert!(!diags.has_errors(), "lt_0: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn lt_1() {
    let (out, diags) = interpret("fn main() print(1 < 6) end", 0);
    assert!(!diags.has_errors(), "lt_1: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn lt_2() {
    let (out, diags) = interpret("fn main() print(2 < 7) end", 0);
    assert!(!diags.has_errors(), "lt_2: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn lt_3() {
    let (out, diags) = interpret("fn main() print(3 < 8) end", 0);
    assert!(!diags.has_errors(), "lt_3: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn lt_4() {
    let (out, diags) = interpret("fn main() print(4 < 9) end", 0);
    assert!(!diags.has_errors(), "lt_4: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn lt_5() {
    let (out, diags) = interpret("fn main() print(5 < 10) end", 0);
    assert!(!diags.has_errors(), "lt_5: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn lt_6() {
    let (out, diags) = interpret("fn main() print(6 < 11) end", 0);
    assert!(!diags.has_errors(), "lt_6: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn lt_7() {
    let (out, diags) = interpret("fn main() print(7 < 12) end", 0);
    assert!(!diags.has_errors(), "lt_7: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn lt_8() {
    let (out, diags) = interpret("fn main() print(8 < 13) end", 0);
    assert!(!diags.has_errors(), "lt_8: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn lt_9() {
    let (out, diags) = interpret("fn main() print(9 < 14) end", 0);
    assert!(!diags.has_errors(), "lt_9: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn lt_10() {
    let (out, diags) = interpret("fn main() print(10 < 15) end", 0);
    assert!(!diags.has_errors(), "lt_10: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn lt_11() {
    let (out, diags) = interpret("fn main() print(11 < 16) end", 0);
    assert!(!diags.has_errors(), "lt_11: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn lt_12() {
    let (out, diags) = interpret("fn main() print(12 < 17) end", 0);
    assert!(!diags.has_errors(), "lt_12: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn lt_13() {
    let (out, diags) = interpret("fn main() print(13 < 18) end", 0);
    assert!(!diags.has_errors(), "lt_13: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn lt_14() {
    let (out, diags) = interpret("fn main() print(14 < 19) end", 0);
    assert!(!diags.has_errors(), "lt_14: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn lt_15() {
    let (out, diags) = interpret("fn main() print(15 < 20) end", 0);
    assert!(!diags.has_errors(), "lt_15: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn lt_16() {
    let (out, diags) = interpret("fn main() print(16 < 21) end", 0);
    assert!(!diags.has_errors(), "lt_16: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn lt_17() {
    let (out, diags) = interpret("fn main() print(17 < 22) end", 0);
    assert!(!diags.has_errors(), "lt_17: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn lt_18() {
    let (out, diags) = interpret("fn main() print(18 < 23) end", 0);
    assert!(!diags.has_errors(), "lt_18: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn lt_19() {
    let (out, diags) = interpret("fn main() print(19 < 24) end", 0);
    assert!(!diags.has_errors(), "lt_19: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn lt_20() {
    let (out, diags) = interpret("fn main() print(20 < 25) end", 0);
    assert!(!diags.has_errors(), "lt_20: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn lt_21() {
    let (out, diags) = interpret("fn main() print(21 < 26) end", 0);
    assert!(!diags.has_errors(), "lt_21: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn lt_22() {
    let (out, diags) = interpret("fn main() print(22 < 27) end", 0);
    assert!(!diags.has_errors(), "lt_22: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn lt_23() {
    let (out, diags) = interpret("fn main() print(23 < 28) end", 0);
    assert!(!diags.has_errors(), "lt_23: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn lt_24() {
    let (out, diags) = interpret("fn main() print(24 < 29) end", 0);
    assert!(!diags.has_errors(), "lt_24: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn lt_25() {
    let (out, diags) = interpret("fn main() print(25 < 30) end", 0);
    assert!(!diags.has_errors(), "lt_25: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn lt_26() {
    let (out, diags) = interpret("fn main() print(26 < 31) end", 0);
    assert!(!diags.has_errors(), "lt_26: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn lt_27() {
    let (out, diags) = interpret("fn main() print(27 < 32) end", 0);
    assert!(!diags.has_errors(), "lt_27: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn lt_28() {
    let (out, diags) = interpret("fn main() print(28 < 33) end", 0);
    assert!(!diags.has_errors(), "lt_28: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn lt_29() {
    let (out, diags) = interpret("fn main() print(29 < 34) end", 0);
    assert!(!diags.has_errors(), "lt_29: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn lt_30() {
    let (out, diags) = interpret("fn main() print(30 < 35) end", 0);
    assert!(!diags.has_errors(), "lt_30: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn lt_31() {
    let (out, diags) = interpret("fn main() print(31 < 36) end", 0);
    assert!(!diags.has_errors(), "lt_31: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn lt_32() {
    let (out, diags) = interpret("fn main() print(32 < 37) end", 0);
    assert!(!diags.has_errors(), "lt_32: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn lt_33() {
    let (out, diags) = interpret("fn main() print(33 < 38) end", 0);
    assert!(!diags.has_errors(), "lt_33: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn lt_34() {
    let (out, diags) = interpret("fn main() print(34 < 39) end", 0);
    assert!(!diags.has_errors(), "lt_34: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn lt_35() {
    let (out, diags) = interpret("fn main() print(35 < 40) end", 0);
    assert!(!diags.has_errors(), "lt_35: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn lt_36() {
    let (out, diags) = interpret("fn main() print(36 < 41) end", 0);
    assert!(!diags.has_errors(), "lt_36: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn lt_37() {
    let (out, diags) = interpret("fn main() print(37 < 42) end", 0);
    assert!(!diags.has_errors(), "lt_37: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn lt_38() {
    let (out, diags) = interpret("fn main() print(38 < 43) end", 0);
    assert!(!diags.has_errors(), "lt_38: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn lt_39() {
    let (out, diags) = interpret("fn main() print(39 < 44) end", 0);
    assert!(!diags.has_errors(), "lt_39: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn lt_40() {
    let (out, diags) = interpret("fn main() print(40 < 45) end", 0);
    assert!(!diags.has_errors(), "lt_40: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn lt_41() {
    let (out, diags) = interpret("fn main() print(41 < 46) end", 0);
    assert!(!diags.has_errors(), "lt_41: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn lt_42() {
    let (out, diags) = interpret("fn main() print(42 < 47) end", 0);
    assert!(!diags.has_errors(), "lt_42: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn lt_43() {
    let (out, diags) = interpret("fn main() print(43 < 48) end", 0);
    assert!(!diags.has_errors(), "lt_43: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn lt_44() {
    let (out, diags) = interpret("fn main() print(44 < 49) end", 0);
    assert!(!diags.has_errors(), "lt_44: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn lt_45() {
    let (out, diags) = interpret("fn main() print(45 < 50) end", 0);
    assert!(!diags.has_errors(), "lt_45: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn lt_46() {
    let (out, diags) = interpret("fn main() print(46 < 51) end", 0);
    assert!(!diags.has_errors(), "lt_46: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn lt_47() {
    let (out, diags) = interpret("fn main() print(47 < 52) end", 0);
    assert!(!diags.has_errors(), "lt_47: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn lt_48() {
    let (out, diags) = interpret("fn main() print(48 < 53) end", 0);
    assert!(!diags.has_errors(), "lt_48: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn lt_49() {
    let (out, diags) = interpret("fn main() print(49 < 54) end", 0);
    assert!(!diags.has_errors(), "lt_49: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn lt_50() {
    let (out, diags) = interpret("fn main() print(50 < 55) end", 0);
    assert!(!diags.has_errors(), "lt_50: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn lt_51() {
    let (out, diags) = interpret("fn main() print(51 < 56) end", 0);
    assert!(!diags.has_errors(), "lt_51: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn lt_52() {
    let (out, diags) = interpret("fn main() print(52 < 57) end", 0);
    assert!(!diags.has_errors(), "lt_52: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn lt_53() {
    let (out, diags) = interpret("fn main() print(53 < 58) end", 0);
    assert!(!diags.has_errors(), "lt_53: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn lt_54() {
    let (out, diags) = interpret("fn main() print(54 < 59) end", 0);
    assert!(!diags.has_errors(), "lt_54: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn lt_55() {
    let (out, diags) = interpret("fn main() print(55 < 60) end", 0);
    assert!(!diags.has_errors(), "lt_55: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn lt_56() {
    let (out, diags) = interpret("fn main() print(56 < 61) end", 0);
    assert!(!diags.has_errors(), "lt_56: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn lt_57() {
    let (out, diags) = interpret("fn main() print(57 < 62) end", 0);
    assert!(!diags.has_errors(), "lt_57: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn lt_58() {
    let (out, diags) = interpret("fn main() print(58 < 63) end", 0);
    assert!(!diags.has_errors(), "lt_58: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn lt_59() {
    let (out, diags) = interpret("fn main() print(59 < 64) end", 0);
    assert!(!diags.has_errors(), "lt_59: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn lt_60() {
    let (out, diags) = interpret("fn main() print(60 < 65) end", 0);
    assert!(!diags.has_errors(), "lt_60: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn lt_61() {
    let (out, diags) = interpret("fn main() print(61 < 66) end", 0);
    assert!(!diags.has_errors(), "lt_61: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn lt_62() {
    let (out, diags) = interpret("fn main() print(62 < 67) end", 0);
    assert!(!diags.has_errors(), "lt_62: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn lt_63() {
    let (out, diags) = interpret("fn main() print(63 < 68) end", 0);
    assert!(!diags.has_errors(), "lt_63: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn lt_64() {
    let (out, diags) = interpret("fn main() print(64 < 69) end", 0);
    assert!(!diags.has_errors(), "lt_64: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn lt_65() {
    let (out, diags) = interpret("fn main() print(65 < 70) end", 0);
    assert!(!diags.has_errors(), "lt_65: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn lt_66() {
    let (out, diags) = interpret("fn main() print(66 < 71) end", 0);
    assert!(!diags.has_errors(), "lt_66: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn lt_67() {
    let (out, diags) = interpret("fn main() print(67 < 72) end", 0);
    assert!(!diags.has_errors(), "lt_67: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn lt_68() {
    let (out, diags) = interpret("fn main() print(68 < 73) end", 0);
    assert!(!diags.has_errors(), "lt_68: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn lt_69() {
    let (out, diags) = interpret("fn main() print(69 < 74) end", 0);
    assert!(!diags.has_errors(), "lt_69: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn lt_70() {
    let (out, diags) = interpret("fn main() print(70 < 75) end", 0);
    assert!(!diags.has_errors(), "lt_70: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn lt_71() {
    let (out, diags) = interpret("fn main() print(71 < 76) end", 0);
    assert!(!diags.has_errors(), "lt_71: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn lt_72() {
    let (out, diags) = interpret("fn main() print(72 < 77) end", 0);
    assert!(!diags.has_errors(), "lt_72: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn lt_73() {
    let (out, diags) = interpret("fn main() print(73 < 78) end", 0);
    assert!(!diags.has_errors(), "lt_73: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn lt_74() {
    let (out, diags) = interpret("fn main() print(74 < 79) end", 0);
    assert!(!diags.has_errors(), "lt_74: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn lt_75() {
    let (out, diags) = interpret("fn main() print(75 < 80) end", 0);
    assert!(!diags.has_errors(), "lt_75: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn lt_76() {
    let (out, diags) = interpret("fn main() print(76 < 81) end", 0);
    assert!(!diags.has_errors(), "lt_76: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn lt_77() {
    let (out, diags) = interpret("fn main() print(77 < 82) end", 0);
    assert!(!diags.has_errors(), "lt_77: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn lt_78() {
    let (out, diags) = interpret("fn main() print(78 < 83) end", 0);
    assert!(!diags.has_errors(), "lt_78: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn lt_79() {
    let (out, diags) = interpret("fn main() print(79 < 84) end", 0);
    assert!(!diags.has_errors(), "lt_79: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn lt_80() {
    let (out, diags) = interpret("fn main() print(80 < 85) end", 0);
    assert!(!diags.has_errors(), "lt_80: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn lt_81() {
    let (out, diags) = interpret("fn main() print(81 < 86) end", 0);
    assert!(!diags.has_errors(), "lt_81: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn lt_82() {
    let (out, diags) = interpret("fn main() print(82 < 87) end", 0);
    assert!(!diags.has_errors(), "lt_82: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn lt_83() {
    let (out, diags) = interpret("fn main() print(83 < 88) end", 0);
    assert!(!diags.has_errors(), "lt_83: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn lt_84() {
    let (out, diags) = interpret("fn main() print(84 < 89) end", 0);
    assert!(!diags.has_errors(), "lt_84: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn lt_85() {
    let (out, diags) = interpret("fn main() print(85 < 90) end", 0);
    assert!(!diags.has_errors(), "lt_85: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn lt_86() {
    let (out, diags) = interpret("fn main() print(86 < 91) end", 0);
    assert!(!diags.has_errors(), "lt_86: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn lt_87() {
    let (out, diags) = interpret("fn main() print(87 < 92) end", 0);
    assert!(!diags.has_errors(), "lt_87: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn lt_88() {
    let (out, diags) = interpret("fn main() print(88 < 93) end", 0);
    assert!(!diags.has_errors(), "lt_88: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn lt_89() {
    let (out, diags) = interpret("fn main() print(89 < 94) end", 0);
    assert!(!diags.has_errors(), "lt_89: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn lt_90() {
    let (out, diags) = interpret("fn main() print(90 < 95) end", 0);
    assert!(!diags.has_errors(), "lt_90: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn lt_91() {
    let (out, diags) = interpret("fn main() print(91 < 96) end", 0);
    assert!(!diags.has_errors(), "lt_91: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn lt_92() {
    let (out, diags) = interpret("fn main() print(92 < 97) end", 0);
    assert!(!diags.has_errors(), "lt_92: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn lt_93() {
    let (out, diags) = interpret("fn main() print(93 < 98) end", 0);
    assert!(!diags.has_errors(), "lt_93: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn lt_94() {
    let (out, diags) = interpret("fn main() print(94 < 99) end", 0);
    assert!(!diags.has_errors(), "lt_94: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn lt_95() {
    let (out, diags) = interpret("fn main() print(95 < 100) end", 0);
    assert!(!diags.has_errors(), "lt_95: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn lt_96() {
    let (out, diags) = interpret("fn main() print(96 < 101) end", 0);
    assert!(!diags.has_errors(), "lt_96: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn lt_97() {
    let (out, diags) = interpret("fn main() print(97 < 102) end", 0);
    assert!(!diags.has_errors(), "lt_97: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn lt_98() {
    let (out, diags) = interpret("fn main() print(98 < 103) end", 0);
    assert!(!diags.has_errors(), "lt_98: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn lt_99() {
    let (out, diags) = interpret("fn main() print(99 < 104) end", 0);
    assert!(!diags.has_errors(), "lt_99: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn gt_0() {
    let (out, diags) = interpret("fn main() print(10 > 0) end", 0);
    assert!(!diags.has_errors(), "gt_0: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn gt_1() {
    let (out, diags) = interpret("fn main() print(11 > 1) end", 0);
    assert!(!diags.has_errors(), "gt_1: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn gt_2() {
    let (out, diags) = interpret("fn main() print(12 > 2) end", 0);
    assert!(!diags.has_errors(), "gt_2: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn gt_3() {
    let (out, diags) = interpret("fn main() print(13 > 3) end", 0);
    assert!(!diags.has_errors(), "gt_3: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn gt_4() {
    let (out, diags) = interpret("fn main() print(14 > 4) end", 0);
    assert!(!diags.has_errors(), "gt_4: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn gt_5() {
    let (out, diags) = interpret("fn main() print(15 > 5) end", 0);
    assert!(!diags.has_errors(), "gt_5: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn gt_6() {
    let (out, diags) = interpret("fn main() print(16 > 6) end", 0);
    assert!(!diags.has_errors(), "gt_6: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn gt_7() {
    let (out, diags) = interpret("fn main() print(17 > 7) end", 0);
    assert!(!diags.has_errors(), "gt_7: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn gt_8() {
    let (out, diags) = interpret("fn main() print(18 > 8) end", 0);
    assert!(!diags.has_errors(), "gt_8: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn gt_9() {
    let (out, diags) = interpret("fn main() print(19 > 9) end", 0);
    assert!(!diags.has_errors(), "gt_9: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn gt_10() {
    let (out, diags) = interpret("fn main() print(20 > 10) end", 0);
    assert!(!diags.has_errors(), "gt_10: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn gt_11() {
    let (out, diags) = interpret("fn main() print(21 > 11) end", 0);
    assert!(!diags.has_errors(), "gt_11: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn gt_12() {
    let (out, diags) = interpret("fn main() print(22 > 12) end", 0);
    assert!(!diags.has_errors(), "gt_12: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn gt_13() {
    let (out, diags) = interpret("fn main() print(23 > 13) end", 0);
    assert!(!diags.has_errors(), "gt_13: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn gt_14() {
    let (out, diags) = interpret("fn main() print(24 > 14) end", 0);
    assert!(!diags.has_errors(), "gt_14: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn gt_15() {
    let (out, diags) = interpret("fn main() print(25 > 15) end", 0);
    assert!(!diags.has_errors(), "gt_15: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn gt_16() {
    let (out, diags) = interpret("fn main() print(26 > 16) end", 0);
    assert!(!diags.has_errors(), "gt_16: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn gt_17() {
    let (out, diags) = interpret("fn main() print(27 > 17) end", 0);
    assert!(!diags.has_errors(), "gt_17: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn gt_18() {
    let (out, diags) = interpret("fn main() print(28 > 18) end", 0);
    assert!(!diags.has_errors(), "gt_18: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn gt_19() {
    let (out, diags) = interpret("fn main() print(29 > 19) end", 0);
    assert!(!diags.has_errors(), "gt_19: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn gt_20() {
    let (out, diags) = interpret("fn main() print(30 > 20) end", 0);
    assert!(!diags.has_errors(), "gt_20: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn gt_21() {
    let (out, diags) = interpret("fn main() print(31 > 21) end", 0);
    assert!(!diags.has_errors(), "gt_21: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn gt_22() {
    let (out, diags) = interpret("fn main() print(32 > 22) end", 0);
    assert!(!diags.has_errors(), "gt_22: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn gt_23() {
    let (out, diags) = interpret("fn main() print(33 > 23) end", 0);
    assert!(!diags.has_errors(), "gt_23: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn gt_24() {
    let (out, diags) = interpret("fn main() print(34 > 24) end", 0);
    assert!(!diags.has_errors(), "gt_24: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn gt_25() {
    let (out, diags) = interpret("fn main() print(35 > 25) end", 0);
    assert!(!diags.has_errors(), "gt_25: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn gt_26() {
    let (out, diags) = interpret("fn main() print(36 > 26) end", 0);
    assert!(!diags.has_errors(), "gt_26: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn gt_27() {
    let (out, diags) = interpret("fn main() print(37 > 27) end", 0);
    assert!(!diags.has_errors(), "gt_27: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn gt_28() {
    let (out, diags) = interpret("fn main() print(38 > 28) end", 0);
    assert!(!diags.has_errors(), "gt_28: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn gt_29() {
    let (out, diags) = interpret("fn main() print(39 > 29) end", 0);
    assert!(!diags.has_errors(), "gt_29: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn gt_30() {
    let (out, diags) = interpret("fn main() print(40 > 30) end", 0);
    assert!(!diags.has_errors(), "gt_30: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn gt_31() {
    let (out, diags) = interpret("fn main() print(41 > 31) end", 0);
    assert!(!diags.has_errors(), "gt_31: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn gt_32() {
    let (out, diags) = interpret("fn main() print(42 > 32) end", 0);
    assert!(!diags.has_errors(), "gt_32: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn gt_33() {
    let (out, diags) = interpret("fn main() print(43 > 33) end", 0);
    assert!(!diags.has_errors(), "gt_33: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn gt_34() {
    let (out, diags) = interpret("fn main() print(44 > 34) end", 0);
    assert!(!diags.has_errors(), "gt_34: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn gt_35() {
    let (out, diags) = interpret("fn main() print(45 > 35) end", 0);
    assert!(!diags.has_errors(), "gt_35: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn gt_36() {
    let (out, diags) = interpret("fn main() print(46 > 36) end", 0);
    assert!(!diags.has_errors(), "gt_36: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn gt_37() {
    let (out, diags) = interpret("fn main() print(47 > 37) end", 0);
    assert!(!diags.has_errors(), "gt_37: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn gt_38() {
    let (out, diags) = interpret("fn main() print(48 > 38) end", 0);
    assert!(!diags.has_errors(), "gt_38: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn gt_39() {
    let (out, diags) = interpret("fn main() print(49 > 39) end", 0);
    assert!(!diags.has_errors(), "gt_39: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn gt_40() {
    let (out, diags) = interpret("fn main() print(50 > 40) end", 0);
    assert!(!diags.has_errors(), "gt_40: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn gt_41() {
    let (out, diags) = interpret("fn main() print(51 > 41) end", 0);
    assert!(!diags.has_errors(), "gt_41: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn gt_42() {
    let (out, diags) = interpret("fn main() print(52 > 42) end", 0);
    assert!(!diags.has_errors(), "gt_42: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn gt_43() {
    let (out, diags) = interpret("fn main() print(53 > 43) end", 0);
    assert!(!diags.has_errors(), "gt_43: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn gt_44() {
    let (out, diags) = interpret("fn main() print(54 > 44) end", 0);
    assert!(!diags.has_errors(), "gt_44: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn gt_45() {
    let (out, diags) = interpret("fn main() print(55 > 45) end", 0);
    assert!(!diags.has_errors(), "gt_45: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn gt_46() {
    let (out, diags) = interpret("fn main() print(56 > 46) end", 0);
    assert!(!diags.has_errors(), "gt_46: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn gt_47() {
    let (out, diags) = interpret("fn main() print(57 > 47) end", 0);
    assert!(!diags.has_errors(), "gt_47: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn gt_48() {
    let (out, diags) = interpret("fn main() print(58 > 48) end", 0);
    assert!(!diags.has_errors(), "gt_48: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn gt_49() {
    let (out, diags) = interpret("fn main() print(59 > 49) end", 0);
    assert!(!diags.has_errors(), "gt_49: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn gt_50() {
    let (out, diags) = interpret("fn main() print(60 > 50) end", 0);
    assert!(!diags.has_errors(), "gt_50: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn gt_51() {
    let (out, diags) = interpret("fn main() print(61 > 51) end", 0);
    assert!(!diags.has_errors(), "gt_51: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn gt_52() {
    let (out, diags) = interpret("fn main() print(62 > 52) end", 0);
    assert!(!diags.has_errors(), "gt_52: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn gt_53() {
    let (out, diags) = interpret("fn main() print(63 > 53) end", 0);
    assert!(!diags.has_errors(), "gt_53: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn gt_54() {
    let (out, diags) = interpret("fn main() print(64 > 54) end", 0);
    assert!(!diags.has_errors(), "gt_54: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn gt_55() {
    let (out, diags) = interpret("fn main() print(65 > 55) end", 0);
    assert!(!diags.has_errors(), "gt_55: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn gt_56() {
    let (out, diags) = interpret("fn main() print(66 > 56) end", 0);
    assert!(!diags.has_errors(), "gt_56: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn gt_57() {
    let (out, diags) = interpret("fn main() print(67 > 57) end", 0);
    assert!(!diags.has_errors(), "gt_57: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn gt_58() {
    let (out, diags) = interpret("fn main() print(68 > 58) end", 0);
    assert!(!diags.has_errors(), "gt_58: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn gt_59() {
    let (out, diags) = interpret("fn main() print(69 > 59) end", 0);
    assert!(!diags.has_errors(), "gt_59: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn gt_60() {
    let (out, diags) = interpret("fn main() print(70 > 60) end", 0);
    assert!(!diags.has_errors(), "gt_60: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn gt_61() {
    let (out, diags) = interpret("fn main() print(71 > 61) end", 0);
    assert!(!diags.has_errors(), "gt_61: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn gt_62() {
    let (out, diags) = interpret("fn main() print(72 > 62) end", 0);
    assert!(!diags.has_errors(), "gt_62: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn gt_63() {
    let (out, diags) = interpret("fn main() print(73 > 63) end", 0);
    assert!(!diags.has_errors(), "gt_63: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn gt_64() {
    let (out, diags) = interpret("fn main() print(74 > 64) end", 0);
    assert!(!diags.has_errors(), "gt_64: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn gt_65() {
    let (out, diags) = interpret("fn main() print(75 > 65) end", 0);
    assert!(!diags.has_errors(), "gt_65: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn gt_66() {
    let (out, diags) = interpret("fn main() print(76 > 66) end", 0);
    assert!(!diags.has_errors(), "gt_66: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn gt_67() {
    let (out, diags) = interpret("fn main() print(77 > 67) end", 0);
    assert!(!diags.has_errors(), "gt_67: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn gt_68() {
    let (out, diags) = interpret("fn main() print(78 > 68) end", 0);
    assert!(!diags.has_errors(), "gt_68: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn gt_69() {
    let (out, diags) = interpret("fn main() print(79 > 69) end", 0);
    assert!(!diags.has_errors(), "gt_69: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn gt_70() {
    let (out, diags) = interpret("fn main() print(80 > 70) end", 0);
    assert!(!diags.has_errors(), "gt_70: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn gt_71() {
    let (out, diags) = interpret("fn main() print(81 > 71) end", 0);
    assert!(!diags.has_errors(), "gt_71: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn gt_72() {
    let (out, diags) = interpret("fn main() print(82 > 72) end", 0);
    assert!(!diags.has_errors(), "gt_72: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn gt_73() {
    let (out, diags) = interpret("fn main() print(83 > 73) end", 0);
    assert!(!diags.has_errors(), "gt_73: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn gt_74() {
    let (out, diags) = interpret("fn main() print(84 > 74) end", 0);
    assert!(!diags.has_errors(), "gt_74: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn gt_75() {
    let (out, diags) = interpret("fn main() print(85 > 75) end", 0);
    assert!(!diags.has_errors(), "gt_75: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn gt_76() {
    let (out, diags) = interpret("fn main() print(86 > 76) end", 0);
    assert!(!diags.has_errors(), "gt_76: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn gt_77() {
    let (out, diags) = interpret("fn main() print(87 > 77) end", 0);
    assert!(!diags.has_errors(), "gt_77: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn gt_78() {
    let (out, diags) = interpret("fn main() print(88 > 78) end", 0);
    assert!(!diags.has_errors(), "gt_78: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn gt_79() {
    let (out, diags) = interpret("fn main() print(89 > 79) end", 0);
    assert!(!diags.has_errors(), "gt_79: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn gt_80() {
    let (out, diags) = interpret("fn main() print(90 > 80) end", 0);
    assert!(!diags.has_errors(), "gt_80: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn gt_81() {
    let (out, diags) = interpret("fn main() print(91 > 81) end", 0);
    assert!(!diags.has_errors(), "gt_81: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn gt_82() {
    let (out, diags) = interpret("fn main() print(92 > 82) end", 0);
    assert!(!diags.has_errors(), "gt_82: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn gt_83() {
    let (out, diags) = interpret("fn main() print(93 > 83) end", 0);
    assert!(!diags.has_errors(), "gt_83: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn gt_84() {
    let (out, diags) = interpret("fn main() print(94 > 84) end", 0);
    assert!(!diags.has_errors(), "gt_84: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn gt_85() {
    let (out, diags) = interpret("fn main() print(95 > 85) end", 0);
    assert!(!diags.has_errors(), "gt_85: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn gt_86() {
    let (out, diags) = interpret("fn main() print(96 > 86) end", 0);
    assert!(!diags.has_errors(), "gt_86: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn gt_87() {
    let (out, diags) = interpret("fn main() print(97 > 87) end", 0);
    assert!(!diags.has_errors(), "gt_87: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn gt_88() {
    let (out, diags) = interpret("fn main() print(98 > 88) end", 0);
    assert!(!diags.has_errors(), "gt_88: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn gt_89() {
    let (out, diags) = interpret("fn main() print(99 > 89) end", 0);
    assert!(!diags.has_errors(), "gt_89: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn gt_90() {
    let (out, diags) = interpret("fn main() print(100 > 90) end", 0);
    assert!(!diags.has_errors(), "gt_90: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn gt_91() {
    let (out, diags) = interpret("fn main() print(101 > 91) end", 0);
    assert!(!diags.has_errors(), "gt_91: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn gt_92() {
    let (out, diags) = interpret("fn main() print(102 > 92) end", 0);
    assert!(!diags.has_errors(), "gt_92: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn gt_93() {
    let (out, diags) = interpret("fn main() print(103 > 93) end", 0);
    assert!(!diags.has_errors(), "gt_93: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn gt_94() {
    let (out, diags) = interpret("fn main() print(104 > 94) end", 0);
    assert!(!diags.has_errors(), "gt_94: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn gt_95() {
    let (out, diags) = interpret("fn main() print(105 > 95) end", 0);
    assert!(!diags.has_errors(), "gt_95: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn gt_96() {
    let (out, diags) = interpret("fn main() print(106 > 96) end", 0);
    assert!(!diags.has_errors(), "gt_96: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn gt_97() {
    let (out, diags) = interpret("fn main() print(107 > 97) end", 0);
    assert!(!diags.has_errors(), "gt_97: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn gt_98() {
    let (out, diags) = interpret("fn main() print(108 > 98) end", 0);
    assert!(!diags.has_errors(), "gt_98: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn gt_99() {
    let (out, diags) = interpret("fn main() print(109 > 99) end", 0);
    assert!(!diags.has_errors(), "gt_99: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn eq_same_0() {
    let (out, diags) = interpret("fn main() print(0 == 0) end", 0);
    assert!(!diags.has_errors(), "eq_same_0: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn eq_same_1() {
    let (out, diags) = interpret("fn main() print(1 == 1) end", 0);
    assert!(!diags.has_errors(), "eq_same_1: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn eq_same_2() {
    let (out, diags) = interpret("fn main() print(2 == 2) end", 0);
    assert!(!diags.has_errors(), "eq_same_2: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn eq_same_3() {
    let (out, diags) = interpret("fn main() print(3 == 3) end", 0);
    assert!(!diags.has_errors(), "eq_same_3: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn eq_same_4() {
    let (out, diags) = interpret("fn main() print(4 == 4) end", 0);
    assert!(!diags.has_errors(), "eq_same_4: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn eq_same_5() {
    let (out, diags) = interpret("fn main() print(5 == 5) end", 0);
    assert!(!diags.has_errors(), "eq_same_5: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn eq_same_6() {
    let (out, diags) = interpret("fn main() print(6 == 6) end", 0);
    assert!(!diags.has_errors(), "eq_same_6: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn eq_same_7() {
    let (out, diags) = interpret("fn main() print(7 == 7) end", 0);
    assert!(!diags.has_errors(), "eq_same_7: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn eq_same_8() {
    let (out, diags) = interpret("fn main() print(8 == 8) end", 0);
    assert!(!diags.has_errors(), "eq_same_8: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn eq_same_9() {
    let (out, diags) = interpret("fn main() print(9 == 9) end", 0);
    assert!(!diags.has_errors(), "eq_same_9: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn eq_same_10() {
    let (out, diags) = interpret("fn main() print(10 == 10) end", 0);
    assert!(!diags.has_errors(), "eq_same_10: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn eq_same_11() {
    let (out, diags) = interpret("fn main() print(11 == 11) end", 0);
    assert!(!diags.has_errors(), "eq_same_11: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn eq_same_12() {
    let (out, diags) = interpret("fn main() print(12 == 12) end", 0);
    assert!(!diags.has_errors(), "eq_same_12: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn eq_same_13() {
    let (out, diags) = interpret("fn main() print(13 == 13) end", 0);
    assert!(!diags.has_errors(), "eq_same_13: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn eq_same_14() {
    let (out, diags) = interpret("fn main() print(14 == 14) end", 0);
    assert!(!diags.has_errors(), "eq_same_14: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn eq_same_15() {
    let (out, diags) = interpret("fn main() print(15 == 15) end", 0);
    assert!(!diags.has_errors(), "eq_same_15: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn eq_same_16() {
    let (out, diags) = interpret("fn main() print(16 == 16) end", 0);
    assert!(!diags.has_errors(), "eq_same_16: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn eq_same_17() {
    let (out, diags) = interpret("fn main() print(17 == 17) end", 0);
    assert!(!diags.has_errors(), "eq_same_17: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn eq_same_18() {
    let (out, diags) = interpret("fn main() print(18 == 18) end", 0);
    assert!(!diags.has_errors(), "eq_same_18: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn eq_same_19() {
    let (out, diags) = interpret("fn main() print(19 == 19) end", 0);
    assert!(!diags.has_errors(), "eq_same_19: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn eq_same_20() {
    let (out, diags) = interpret("fn main() print(20 == 20) end", 0);
    assert!(!diags.has_errors(), "eq_same_20: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn eq_same_21() {
    let (out, diags) = interpret("fn main() print(21 == 21) end", 0);
    assert!(!diags.has_errors(), "eq_same_21: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn eq_same_22() {
    let (out, diags) = interpret("fn main() print(22 == 22) end", 0);
    assert!(!diags.has_errors(), "eq_same_22: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn eq_same_23() {
    let (out, diags) = interpret("fn main() print(23 == 23) end", 0);
    assert!(!diags.has_errors(), "eq_same_23: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn eq_same_24() {
    let (out, diags) = interpret("fn main() print(24 == 24) end", 0);
    assert!(!diags.has_errors(), "eq_same_24: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn eq_same_25() {
    let (out, diags) = interpret("fn main() print(25 == 25) end", 0);
    assert!(!diags.has_errors(), "eq_same_25: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn eq_same_26() {
    let (out, diags) = interpret("fn main() print(26 == 26) end", 0);
    assert!(!diags.has_errors(), "eq_same_26: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn eq_same_27() {
    let (out, diags) = interpret("fn main() print(27 == 27) end", 0);
    assert!(!diags.has_errors(), "eq_same_27: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn eq_same_28() {
    let (out, diags) = interpret("fn main() print(28 == 28) end", 0);
    assert!(!diags.has_errors(), "eq_same_28: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn eq_same_29() {
    let (out, diags) = interpret("fn main() print(29 == 29) end", 0);
    assert!(!diags.has_errors(), "eq_same_29: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn eq_same_30() {
    let (out, diags) = interpret("fn main() print(30 == 30) end", 0);
    assert!(!diags.has_errors(), "eq_same_30: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn eq_same_31() {
    let (out, diags) = interpret("fn main() print(31 == 31) end", 0);
    assert!(!diags.has_errors(), "eq_same_31: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn eq_same_32() {
    let (out, diags) = interpret("fn main() print(32 == 32) end", 0);
    assert!(!diags.has_errors(), "eq_same_32: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn eq_same_33() {
    let (out, diags) = interpret("fn main() print(33 == 33) end", 0);
    assert!(!diags.has_errors(), "eq_same_33: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn eq_same_34() {
    let (out, diags) = interpret("fn main() print(34 == 34) end", 0);
    assert!(!diags.has_errors(), "eq_same_34: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn eq_same_35() {
    let (out, diags) = interpret("fn main() print(35 == 35) end", 0);
    assert!(!diags.has_errors(), "eq_same_35: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn eq_same_36() {
    let (out, diags) = interpret("fn main() print(36 == 36) end", 0);
    assert!(!diags.has_errors(), "eq_same_36: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn eq_same_37() {
    let (out, diags) = interpret("fn main() print(37 == 37) end", 0);
    assert!(!diags.has_errors(), "eq_same_37: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn eq_same_38() {
    let (out, diags) = interpret("fn main() print(38 == 38) end", 0);
    assert!(!diags.has_errors(), "eq_same_38: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn eq_same_39() {
    let (out, diags) = interpret("fn main() print(39 == 39) end", 0);
    assert!(!diags.has_errors(), "eq_same_39: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn eq_same_40() {
    let (out, diags) = interpret("fn main() print(40 == 40) end", 0);
    assert!(!diags.has_errors(), "eq_same_40: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn eq_same_41() {
    let (out, diags) = interpret("fn main() print(41 == 41) end", 0);
    assert!(!diags.has_errors(), "eq_same_41: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn eq_same_42() {
    let (out, diags) = interpret("fn main() print(42 == 42) end", 0);
    assert!(!diags.has_errors(), "eq_same_42: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn eq_same_43() {
    let (out, diags) = interpret("fn main() print(43 == 43) end", 0);
    assert!(!diags.has_errors(), "eq_same_43: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn eq_same_44() {
    let (out, diags) = interpret("fn main() print(44 == 44) end", 0);
    assert!(!diags.has_errors(), "eq_same_44: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn eq_same_45() {
    let (out, diags) = interpret("fn main() print(45 == 45) end", 0);
    assert!(!diags.has_errors(), "eq_same_45: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn eq_same_46() {
    let (out, diags) = interpret("fn main() print(46 == 46) end", 0);
    assert!(!diags.has_errors(), "eq_same_46: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn eq_same_47() {
    let (out, diags) = interpret("fn main() print(47 == 47) end", 0);
    assert!(!diags.has_errors(), "eq_same_47: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn eq_same_48() {
    let (out, diags) = interpret("fn main() print(48 == 48) end", 0);
    assert!(!diags.has_errors(), "eq_same_48: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn eq_same_49() {
    let (out, diags) = interpret("fn main() print(49 == 49) end", 0);
    assert!(!diags.has_errors(), "eq_same_49: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn eq_same_50() {
    let (out, diags) = interpret("fn main() print(50 == 50) end", 0);
    assert!(!diags.has_errors(), "eq_same_50: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn eq_same_51() {
    let (out, diags) = interpret("fn main() print(51 == 51) end", 0);
    assert!(!diags.has_errors(), "eq_same_51: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn eq_same_52() {
    let (out, diags) = interpret("fn main() print(52 == 52) end", 0);
    assert!(!diags.has_errors(), "eq_same_52: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn eq_same_53() {
    let (out, diags) = interpret("fn main() print(53 == 53) end", 0);
    assert!(!diags.has_errors(), "eq_same_53: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn eq_same_54() {
    let (out, diags) = interpret("fn main() print(54 == 54) end", 0);
    assert!(!diags.has_errors(), "eq_same_54: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn eq_same_55() {
    let (out, diags) = interpret("fn main() print(55 == 55) end", 0);
    assert!(!diags.has_errors(), "eq_same_55: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn eq_same_56() {
    let (out, diags) = interpret("fn main() print(56 == 56) end", 0);
    assert!(!diags.has_errors(), "eq_same_56: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn eq_same_57() {
    let (out, diags) = interpret("fn main() print(57 == 57) end", 0);
    assert!(!diags.has_errors(), "eq_same_57: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn eq_same_58() {
    let (out, diags) = interpret("fn main() print(58 == 58) end", 0);
    assert!(!diags.has_errors(), "eq_same_58: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn eq_same_59() {
    let (out, diags) = interpret("fn main() print(59 == 59) end", 0);
    assert!(!diags.has_errors(), "eq_same_59: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn eq_same_60() {
    let (out, diags) = interpret("fn main() print(60 == 60) end", 0);
    assert!(!diags.has_errors(), "eq_same_60: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn eq_same_61() {
    let (out, diags) = interpret("fn main() print(61 == 61) end", 0);
    assert!(!diags.has_errors(), "eq_same_61: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn eq_same_62() {
    let (out, diags) = interpret("fn main() print(62 == 62) end", 0);
    assert!(!diags.has_errors(), "eq_same_62: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn eq_same_63() {
    let (out, diags) = interpret("fn main() print(63 == 63) end", 0);
    assert!(!diags.has_errors(), "eq_same_63: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn eq_same_64() {
    let (out, diags) = interpret("fn main() print(64 == 64) end", 0);
    assert!(!diags.has_errors(), "eq_same_64: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn eq_same_65() {
    let (out, diags) = interpret("fn main() print(65 == 65) end", 0);
    assert!(!diags.has_errors(), "eq_same_65: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn eq_same_66() {
    let (out, diags) = interpret("fn main() print(66 == 66) end", 0);
    assert!(!diags.has_errors(), "eq_same_66: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn eq_same_67() {
    let (out, diags) = interpret("fn main() print(67 == 67) end", 0);
    assert!(!diags.has_errors(), "eq_same_67: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn eq_same_68() {
    let (out, diags) = interpret("fn main() print(68 == 68) end", 0);
    assert!(!diags.has_errors(), "eq_same_68: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn eq_same_69() {
    let (out, diags) = interpret("fn main() print(69 == 69) end", 0);
    assert!(!diags.has_errors(), "eq_same_69: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn eq_same_70() {
    let (out, diags) = interpret("fn main() print(70 == 70) end", 0);
    assert!(!diags.has_errors(), "eq_same_70: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn eq_same_71() {
    let (out, diags) = interpret("fn main() print(71 == 71) end", 0);
    assert!(!diags.has_errors(), "eq_same_71: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn eq_same_72() {
    let (out, diags) = interpret("fn main() print(72 == 72) end", 0);
    assert!(!diags.has_errors(), "eq_same_72: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn eq_same_73() {
    let (out, diags) = interpret("fn main() print(73 == 73) end", 0);
    assert!(!diags.has_errors(), "eq_same_73: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn eq_same_74() {
    let (out, diags) = interpret("fn main() print(74 == 74) end", 0);
    assert!(!diags.has_errors(), "eq_same_74: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn eq_same_75() {
    let (out, diags) = interpret("fn main() print(75 == 75) end", 0);
    assert!(!diags.has_errors(), "eq_same_75: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn eq_same_76() {
    let (out, diags) = interpret("fn main() print(76 == 76) end", 0);
    assert!(!diags.has_errors(), "eq_same_76: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn eq_same_77() {
    let (out, diags) = interpret("fn main() print(77 == 77) end", 0);
    assert!(!diags.has_errors(), "eq_same_77: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn eq_same_78() {
    let (out, diags) = interpret("fn main() print(78 == 78) end", 0);
    assert!(!diags.has_errors(), "eq_same_78: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn eq_same_79() {
    let (out, diags) = interpret("fn main() print(79 == 79) end", 0);
    assert!(!diags.has_errors(), "eq_same_79: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn eq_same_80() {
    let (out, diags) = interpret("fn main() print(80 == 80) end", 0);
    assert!(!diags.has_errors(), "eq_same_80: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn eq_same_81() {
    let (out, diags) = interpret("fn main() print(81 == 81) end", 0);
    assert!(!diags.has_errors(), "eq_same_81: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn eq_same_82() {
    let (out, diags) = interpret("fn main() print(82 == 82) end", 0);
    assert!(!diags.has_errors(), "eq_same_82: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn eq_same_83() {
    let (out, diags) = interpret("fn main() print(83 == 83) end", 0);
    assert!(!diags.has_errors(), "eq_same_83: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn eq_same_84() {
    let (out, diags) = interpret("fn main() print(84 == 84) end", 0);
    assert!(!diags.has_errors(), "eq_same_84: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn eq_same_85() {
    let (out, diags) = interpret("fn main() print(85 == 85) end", 0);
    assert!(!diags.has_errors(), "eq_same_85: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn eq_same_86() {
    let (out, diags) = interpret("fn main() print(86 == 86) end", 0);
    assert!(!diags.has_errors(), "eq_same_86: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn eq_same_87() {
    let (out, diags) = interpret("fn main() print(87 == 87) end", 0);
    assert!(!diags.has_errors(), "eq_same_87: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn eq_same_88() {
    let (out, diags) = interpret("fn main() print(88 == 88) end", 0);
    assert!(!diags.has_errors(), "eq_same_88: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn eq_same_89() {
    let (out, diags) = interpret("fn main() print(89 == 89) end", 0);
    assert!(!diags.has_errors(), "eq_same_89: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn eq_same_90() {
    let (out, diags) = interpret("fn main() print(90 == 90) end", 0);
    assert!(!diags.has_errors(), "eq_same_90: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn eq_same_91() {
    let (out, diags) = interpret("fn main() print(91 == 91) end", 0);
    assert!(!diags.has_errors(), "eq_same_91: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn eq_same_92() {
    let (out, diags) = interpret("fn main() print(92 == 92) end", 0);
    assert!(!diags.has_errors(), "eq_same_92: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn eq_same_93() {
    let (out, diags) = interpret("fn main() print(93 == 93) end", 0);
    assert!(!diags.has_errors(), "eq_same_93: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn eq_same_94() {
    let (out, diags) = interpret("fn main() print(94 == 94) end", 0);
    assert!(!diags.has_errors(), "eq_same_94: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn eq_same_95() {
    let (out, diags) = interpret("fn main() print(95 == 95) end", 0);
    assert!(!diags.has_errors(), "eq_same_95: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn eq_same_96() {
    let (out, diags) = interpret("fn main() print(96 == 96) end", 0);
    assert!(!diags.has_errors(), "eq_same_96: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn eq_same_97() {
    let (out, diags) = interpret("fn main() print(97 == 97) end", 0);
    assert!(!diags.has_errors(), "eq_same_97: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn eq_same_98() {
    let (out, diags) = interpret("fn main() print(98 == 98) end", 0);
    assert!(!diags.has_errors(), "eq_same_98: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn eq_same_99() {
    let (out, diags) = interpret("fn main() print(99 == 99) end", 0);
    assert!(!diags.has_errors(), "eq_same_99: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn eq_diff_0() {
    let (out, diags) = interpret("fn main() print(0 == 1) end", 0);
    assert!(!diags.has_errors(), "eq_diff_0: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn eq_diff_1() {
    let (out, diags) = interpret("fn main() print(1 == 2) end", 0);
    assert!(!diags.has_errors(), "eq_diff_1: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn eq_diff_2() {
    let (out, diags) = interpret("fn main() print(2 == 3) end", 0);
    assert!(!diags.has_errors(), "eq_diff_2: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn eq_diff_3() {
    let (out, diags) = interpret("fn main() print(3 == 4) end", 0);
    assert!(!diags.has_errors(), "eq_diff_3: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn eq_diff_4() {
    let (out, diags) = interpret("fn main() print(4 == 5) end", 0);
    assert!(!diags.has_errors(), "eq_diff_4: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn eq_diff_5() {
    let (out, diags) = interpret("fn main() print(5 == 6) end", 0);
    assert!(!diags.has_errors(), "eq_diff_5: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn eq_diff_6() {
    let (out, diags) = interpret("fn main() print(6 == 7) end", 0);
    assert!(!diags.has_errors(), "eq_diff_6: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn eq_diff_7() {
    let (out, diags) = interpret("fn main() print(7 == 8) end", 0);
    assert!(!diags.has_errors(), "eq_diff_7: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn eq_diff_8() {
    let (out, diags) = interpret("fn main() print(8 == 9) end", 0);
    assert!(!diags.has_errors(), "eq_diff_8: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn eq_diff_9() {
    let (out, diags) = interpret("fn main() print(9 == 10) end", 0);
    assert!(!diags.has_errors(), "eq_diff_9: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn eq_diff_10() {
    let (out, diags) = interpret("fn main() print(10 == 11) end", 0);
    assert!(!diags.has_errors(), "eq_diff_10: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn eq_diff_11() {
    let (out, diags) = interpret("fn main() print(11 == 12) end", 0);
    assert!(!diags.has_errors(), "eq_diff_11: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn eq_diff_12() {
    let (out, diags) = interpret("fn main() print(12 == 13) end", 0);
    assert!(!diags.has_errors(), "eq_diff_12: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn eq_diff_13() {
    let (out, diags) = interpret("fn main() print(13 == 14) end", 0);
    assert!(!diags.has_errors(), "eq_diff_13: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn eq_diff_14() {
    let (out, diags) = interpret("fn main() print(14 == 15) end", 0);
    assert!(!diags.has_errors(), "eq_diff_14: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn eq_diff_15() {
    let (out, diags) = interpret("fn main() print(15 == 16) end", 0);
    assert!(!diags.has_errors(), "eq_diff_15: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn eq_diff_16() {
    let (out, diags) = interpret("fn main() print(16 == 17) end", 0);
    assert!(!diags.has_errors(), "eq_diff_16: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn eq_diff_17() {
    let (out, diags) = interpret("fn main() print(17 == 18) end", 0);
    assert!(!diags.has_errors(), "eq_diff_17: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn eq_diff_18() {
    let (out, diags) = interpret("fn main() print(18 == 19) end", 0);
    assert!(!diags.has_errors(), "eq_diff_18: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn eq_diff_19() {
    let (out, diags) = interpret("fn main() print(19 == 20) end", 0);
    assert!(!diags.has_errors(), "eq_diff_19: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn eq_diff_20() {
    let (out, diags) = interpret("fn main() print(20 == 21) end", 0);
    assert!(!diags.has_errors(), "eq_diff_20: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn eq_diff_21() {
    let (out, diags) = interpret("fn main() print(21 == 22) end", 0);
    assert!(!diags.has_errors(), "eq_diff_21: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn eq_diff_22() {
    let (out, diags) = interpret("fn main() print(22 == 23) end", 0);
    assert!(!diags.has_errors(), "eq_diff_22: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn eq_diff_23() {
    let (out, diags) = interpret("fn main() print(23 == 24) end", 0);
    assert!(!diags.has_errors(), "eq_diff_23: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn eq_diff_24() {
    let (out, diags) = interpret("fn main() print(24 == 25) end", 0);
    assert!(!diags.has_errors(), "eq_diff_24: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn eq_diff_25() {
    let (out, diags) = interpret("fn main() print(25 == 26) end", 0);
    assert!(!diags.has_errors(), "eq_diff_25: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn eq_diff_26() {
    let (out, diags) = interpret("fn main() print(26 == 27) end", 0);
    assert!(!diags.has_errors(), "eq_diff_26: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn eq_diff_27() {
    let (out, diags) = interpret("fn main() print(27 == 28) end", 0);
    assert!(!diags.has_errors(), "eq_diff_27: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn eq_diff_28() {
    let (out, diags) = interpret("fn main() print(28 == 29) end", 0);
    assert!(!diags.has_errors(), "eq_diff_28: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn eq_diff_29() {
    let (out, diags) = interpret("fn main() print(29 == 30) end", 0);
    assert!(!diags.has_errors(), "eq_diff_29: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn eq_diff_30() {
    let (out, diags) = interpret("fn main() print(30 == 31) end", 0);
    assert!(!diags.has_errors(), "eq_diff_30: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn eq_diff_31() {
    let (out, diags) = interpret("fn main() print(31 == 32) end", 0);
    assert!(!diags.has_errors(), "eq_diff_31: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn eq_diff_32() {
    let (out, diags) = interpret("fn main() print(32 == 33) end", 0);
    assert!(!diags.has_errors(), "eq_diff_32: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn eq_diff_33() {
    let (out, diags) = interpret("fn main() print(33 == 34) end", 0);
    assert!(!diags.has_errors(), "eq_diff_33: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn eq_diff_34() {
    let (out, diags) = interpret("fn main() print(34 == 35) end", 0);
    assert!(!diags.has_errors(), "eq_diff_34: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn eq_diff_35() {
    let (out, diags) = interpret("fn main() print(35 == 36) end", 0);
    assert!(!diags.has_errors(), "eq_diff_35: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn eq_diff_36() {
    let (out, diags) = interpret("fn main() print(36 == 37) end", 0);
    assert!(!diags.has_errors(), "eq_diff_36: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn eq_diff_37() {
    let (out, diags) = interpret("fn main() print(37 == 38) end", 0);
    assert!(!diags.has_errors(), "eq_diff_37: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn eq_diff_38() {
    let (out, diags) = interpret("fn main() print(38 == 39) end", 0);
    assert!(!diags.has_errors(), "eq_diff_38: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn eq_diff_39() {
    let (out, diags) = interpret("fn main() print(39 == 40) end", 0);
    assert!(!diags.has_errors(), "eq_diff_39: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn eq_diff_40() {
    let (out, diags) = interpret("fn main() print(40 == 41) end", 0);
    assert!(!diags.has_errors(), "eq_diff_40: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn eq_diff_41() {
    let (out, diags) = interpret("fn main() print(41 == 42) end", 0);
    assert!(!diags.has_errors(), "eq_diff_41: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn eq_diff_42() {
    let (out, diags) = interpret("fn main() print(42 == 43) end", 0);
    assert!(!diags.has_errors(), "eq_diff_42: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn eq_diff_43() {
    let (out, diags) = interpret("fn main() print(43 == 44) end", 0);
    assert!(!diags.has_errors(), "eq_diff_43: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn eq_diff_44() {
    let (out, diags) = interpret("fn main() print(44 == 45) end", 0);
    assert!(!diags.has_errors(), "eq_diff_44: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn eq_diff_45() {
    let (out, diags) = interpret("fn main() print(45 == 46) end", 0);
    assert!(!diags.has_errors(), "eq_diff_45: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn eq_diff_46() {
    let (out, diags) = interpret("fn main() print(46 == 47) end", 0);
    assert!(!diags.has_errors(), "eq_diff_46: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn eq_diff_47() {
    let (out, diags) = interpret("fn main() print(47 == 48) end", 0);
    assert!(!diags.has_errors(), "eq_diff_47: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn eq_diff_48() {
    let (out, diags) = interpret("fn main() print(48 == 49) end", 0);
    assert!(!diags.has_errors(), "eq_diff_48: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn eq_diff_49() {
    let (out, diags) = interpret("fn main() print(49 == 50) end", 0);
    assert!(!diags.has_errors(), "eq_diff_49: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn eq_diff_50() {
    let (out, diags) = interpret("fn main() print(50 == 51) end", 0);
    assert!(!diags.has_errors(), "eq_diff_50: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn eq_diff_51() {
    let (out, diags) = interpret("fn main() print(51 == 52) end", 0);
    assert!(!diags.has_errors(), "eq_diff_51: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn eq_diff_52() {
    let (out, diags) = interpret("fn main() print(52 == 53) end", 0);
    assert!(!diags.has_errors(), "eq_diff_52: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn eq_diff_53() {
    let (out, diags) = interpret("fn main() print(53 == 54) end", 0);
    assert!(!diags.has_errors(), "eq_diff_53: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn eq_diff_54() {
    let (out, diags) = interpret("fn main() print(54 == 55) end", 0);
    assert!(!diags.has_errors(), "eq_diff_54: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn eq_diff_55() {
    let (out, diags) = interpret("fn main() print(55 == 56) end", 0);
    assert!(!diags.has_errors(), "eq_diff_55: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn eq_diff_56() {
    let (out, diags) = interpret("fn main() print(56 == 57) end", 0);
    assert!(!diags.has_errors(), "eq_diff_56: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn eq_diff_57() {
    let (out, diags) = interpret("fn main() print(57 == 58) end", 0);
    assert!(!diags.has_errors(), "eq_diff_57: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn eq_diff_58() {
    let (out, diags) = interpret("fn main() print(58 == 59) end", 0);
    assert!(!diags.has_errors(), "eq_diff_58: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn eq_diff_59() {
    let (out, diags) = interpret("fn main() print(59 == 60) end", 0);
    assert!(!diags.has_errors(), "eq_diff_59: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn eq_diff_60() {
    let (out, diags) = interpret("fn main() print(60 == 61) end", 0);
    assert!(!diags.has_errors(), "eq_diff_60: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn eq_diff_61() {
    let (out, diags) = interpret("fn main() print(61 == 62) end", 0);
    assert!(!diags.has_errors(), "eq_diff_61: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn eq_diff_62() {
    let (out, diags) = interpret("fn main() print(62 == 63) end", 0);
    assert!(!diags.has_errors(), "eq_diff_62: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn eq_diff_63() {
    let (out, diags) = interpret("fn main() print(63 == 64) end", 0);
    assert!(!diags.has_errors(), "eq_diff_63: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn eq_diff_64() {
    let (out, diags) = interpret("fn main() print(64 == 65) end", 0);
    assert!(!diags.has_errors(), "eq_diff_64: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn eq_diff_65() {
    let (out, diags) = interpret("fn main() print(65 == 66) end", 0);
    assert!(!diags.has_errors(), "eq_diff_65: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn eq_diff_66() {
    let (out, diags) = interpret("fn main() print(66 == 67) end", 0);
    assert!(!diags.has_errors(), "eq_diff_66: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn eq_diff_67() {
    let (out, diags) = interpret("fn main() print(67 == 68) end", 0);
    assert!(!diags.has_errors(), "eq_diff_67: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn eq_diff_68() {
    let (out, diags) = interpret("fn main() print(68 == 69) end", 0);
    assert!(!diags.has_errors(), "eq_diff_68: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn eq_diff_69() {
    let (out, diags) = interpret("fn main() print(69 == 70) end", 0);
    assert!(!diags.has_errors(), "eq_diff_69: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn eq_diff_70() {
    let (out, diags) = interpret("fn main() print(70 == 71) end", 0);
    assert!(!diags.has_errors(), "eq_diff_70: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn eq_diff_71() {
    let (out, diags) = interpret("fn main() print(71 == 72) end", 0);
    assert!(!diags.has_errors(), "eq_diff_71: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn eq_diff_72() {
    let (out, diags) = interpret("fn main() print(72 == 73) end", 0);
    assert!(!diags.has_errors(), "eq_diff_72: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn eq_diff_73() {
    let (out, diags) = interpret("fn main() print(73 == 74) end", 0);
    assert!(!diags.has_errors(), "eq_diff_73: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn eq_diff_74() {
    let (out, diags) = interpret("fn main() print(74 == 75) end", 0);
    assert!(!diags.has_errors(), "eq_diff_74: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn eq_diff_75() {
    let (out, diags) = interpret("fn main() print(75 == 76) end", 0);
    assert!(!diags.has_errors(), "eq_diff_75: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn eq_diff_76() {
    let (out, diags) = interpret("fn main() print(76 == 77) end", 0);
    assert!(!diags.has_errors(), "eq_diff_76: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn eq_diff_77() {
    let (out, diags) = interpret("fn main() print(77 == 78) end", 0);
    assert!(!diags.has_errors(), "eq_diff_77: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn eq_diff_78() {
    let (out, diags) = interpret("fn main() print(78 == 79) end", 0);
    assert!(!diags.has_errors(), "eq_diff_78: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn eq_diff_79() {
    let (out, diags) = interpret("fn main() print(79 == 80) end", 0);
    assert!(!diags.has_errors(), "eq_diff_79: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn eq_diff_80() {
    let (out, diags) = interpret("fn main() print(80 == 81) end", 0);
    assert!(!diags.has_errors(), "eq_diff_80: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn eq_diff_81() {
    let (out, diags) = interpret("fn main() print(81 == 82) end", 0);
    assert!(!diags.has_errors(), "eq_diff_81: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn eq_diff_82() {
    let (out, diags) = interpret("fn main() print(82 == 83) end", 0);
    assert!(!diags.has_errors(), "eq_diff_82: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn eq_diff_83() {
    let (out, diags) = interpret("fn main() print(83 == 84) end", 0);
    assert!(!diags.has_errors(), "eq_diff_83: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn eq_diff_84() {
    let (out, diags) = interpret("fn main() print(84 == 85) end", 0);
    assert!(!diags.has_errors(), "eq_diff_84: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn eq_diff_85() {
    let (out, diags) = interpret("fn main() print(85 == 86) end", 0);
    assert!(!diags.has_errors(), "eq_diff_85: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn eq_diff_86() {
    let (out, diags) = interpret("fn main() print(86 == 87) end", 0);
    assert!(!diags.has_errors(), "eq_diff_86: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn eq_diff_87() {
    let (out, diags) = interpret("fn main() print(87 == 88) end", 0);
    assert!(!diags.has_errors(), "eq_diff_87: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn eq_diff_88() {
    let (out, diags) = interpret("fn main() print(88 == 89) end", 0);
    assert!(!diags.has_errors(), "eq_diff_88: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn eq_diff_89() {
    let (out, diags) = interpret("fn main() print(89 == 90) end", 0);
    assert!(!diags.has_errors(), "eq_diff_89: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn eq_diff_90() {
    let (out, diags) = interpret("fn main() print(90 == 91) end", 0);
    assert!(!diags.has_errors(), "eq_diff_90: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn eq_diff_91() {
    let (out, diags) = interpret("fn main() print(91 == 92) end", 0);
    assert!(!diags.has_errors(), "eq_diff_91: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn eq_diff_92() {
    let (out, diags) = interpret("fn main() print(92 == 93) end", 0);
    assert!(!diags.has_errors(), "eq_diff_92: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn eq_diff_93() {
    let (out, diags) = interpret("fn main() print(93 == 94) end", 0);
    assert!(!diags.has_errors(), "eq_diff_93: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn eq_diff_94() {
    let (out, diags) = interpret("fn main() print(94 == 95) end", 0);
    assert!(!diags.has_errors(), "eq_diff_94: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn eq_diff_95() {
    let (out, diags) = interpret("fn main() print(95 == 96) end", 0);
    assert!(!diags.has_errors(), "eq_diff_95: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn eq_diff_96() {
    let (out, diags) = interpret("fn main() print(96 == 97) end", 0);
    assert!(!diags.has_errors(), "eq_diff_96: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn eq_diff_97() {
    let (out, diags) = interpret("fn main() print(97 == 98) end", 0);
    assert!(!diags.has_errors(), "eq_diff_97: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn eq_diff_98() {
    let (out, diags) = interpret("fn main() print(98 == 99) end", 0);
    assert!(!diags.has_errors(), "eq_diff_98: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn eq_diff_99() {
    let (out, diags) = interpret("fn main() print(99 == 100) end", 0);
    assert!(!diags.has_errors(), "eq_diff_99: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"false
");
}

#[test]
fn ne_0() {
    let (out, diags) = interpret("fn main() print(0 ~= 1) end", 0);
    assert!(!diags.has_errors(), "ne_0: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ne_1() {
    let (out, diags) = interpret("fn main() print(1 ~= 2) end", 0);
    assert!(!diags.has_errors(), "ne_1: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ne_2() {
    let (out, diags) = interpret("fn main() print(2 ~= 3) end", 0);
    assert!(!diags.has_errors(), "ne_2: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ne_3() {
    let (out, diags) = interpret("fn main() print(3 ~= 4) end", 0);
    assert!(!diags.has_errors(), "ne_3: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ne_4() {
    let (out, diags) = interpret("fn main() print(4 ~= 5) end", 0);
    assert!(!diags.has_errors(), "ne_4: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ne_5() {
    let (out, diags) = interpret("fn main() print(5 ~= 6) end", 0);
    assert!(!diags.has_errors(), "ne_5: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ne_6() {
    let (out, diags) = interpret("fn main() print(6 ~= 7) end", 0);
    assert!(!diags.has_errors(), "ne_6: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ne_7() {
    let (out, diags) = interpret("fn main() print(7 ~= 8) end", 0);
    assert!(!diags.has_errors(), "ne_7: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ne_8() {
    let (out, diags) = interpret("fn main() print(8 ~= 9) end", 0);
    assert!(!diags.has_errors(), "ne_8: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ne_9() {
    let (out, diags) = interpret("fn main() print(9 ~= 10) end", 0);
    assert!(!diags.has_errors(), "ne_9: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ne_10() {
    let (out, diags) = interpret("fn main() print(10 ~= 11) end", 0);
    assert!(!diags.has_errors(), "ne_10: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ne_11() {
    let (out, diags) = interpret("fn main() print(11 ~= 12) end", 0);
    assert!(!diags.has_errors(), "ne_11: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ne_12() {
    let (out, diags) = interpret("fn main() print(12 ~= 13) end", 0);
    assert!(!diags.has_errors(), "ne_12: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ne_13() {
    let (out, diags) = interpret("fn main() print(13 ~= 14) end", 0);
    assert!(!diags.has_errors(), "ne_13: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ne_14() {
    let (out, diags) = interpret("fn main() print(14 ~= 15) end", 0);
    assert!(!diags.has_errors(), "ne_14: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ne_15() {
    let (out, diags) = interpret("fn main() print(15 ~= 16) end", 0);
    assert!(!diags.has_errors(), "ne_15: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ne_16() {
    let (out, diags) = interpret("fn main() print(16 ~= 17) end", 0);
    assert!(!diags.has_errors(), "ne_16: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ne_17() {
    let (out, diags) = interpret("fn main() print(17 ~= 18) end", 0);
    assert!(!diags.has_errors(), "ne_17: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ne_18() {
    let (out, diags) = interpret("fn main() print(18 ~= 19) end", 0);
    assert!(!diags.has_errors(), "ne_18: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ne_19() {
    let (out, diags) = interpret("fn main() print(19 ~= 20) end", 0);
    assert!(!diags.has_errors(), "ne_19: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ne_20() {
    let (out, diags) = interpret("fn main() print(20 ~= 21) end", 0);
    assert!(!diags.has_errors(), "ne_20: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ne_21() {
    let (out, diags) = interpret("fn main() print(21 ~= 22) end", 0);
    assert!(!diags.has_errors(), "ne_21: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ne_22() {
    let (out, diags) = interpret("fn main() print(22 ~= 23) end", 0);
    assert!(!diags.has_errors(), "ne_22: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ne_23() {
    let (out, diags) = interpret("fn main() print(23 ~= 24) end", 0);
    assert!(!diags.has_errors(), "ne_23: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ne_24() {
    let (out, diags) = interpret("fn main() print(24 ~= 25) end", 0);
    assert!(!diags.has_errors(), "ne_24: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ne_25() {
    let (out, diags) = interpret("fn main() print(25 ~= 26) end", 0);
    assert!(!diags.has_errors(), "ne_25: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ne_26() {
    let (out, diags) = interpret("fn main() print(26 ~= 27) end", 0);
    assert!(!diags.has_errors(), "ne_26: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ne_27() {
    let (out, diags) = interpret("fn main() print(27 ~= 28) end", 0);
    assert!(!diags.has_errors(), "ne_27: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ne_28() {
    let (out, diags) = interpret("fn main() print(28 ~= 29) end", 0);
    assert!(!diags.has_errors(), "ne_28: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ne_29() {
    let (out, diags) = interpret("fn main() print(29 ~= 30) end", 0);
    assert!(!diags.has_errors(), "ne_29: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ne_30() {
    let (out, diags) = interpret("fn main() print(30 ~= 31) end", 0);
    assert!(!diags.has_errors(), "ne_30: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ne_31() {
    let (out, diags) = interpret("fn main() print(31 ~= 32) end", 0);
    assert!(!diags.has_errors(), "ne_31: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ne_32() {
    let (out, diags) = interpret("fn main() print(32 ~= 33) end", 0);
    assert!(!diags.has_errors(), "ne_32: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ne_33() {
    let (out, diags) = interpret("fn main() print(33 ~= 34) end", 0);
    assert!(!diags.has_errors(), "ne_33: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ne_34() {
    let (out, diags) = interpret("fn main() print(34 ~= 35) end", 0);
    assert!(!diags.has_errors(), "ne_34: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ne_35() {
    let (out, diags) = interpret("fn main() print(35 ~= 36) end", 0);
    assert!(!diags.has_errors(), "ne_35: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ne_36() {
    let (out, diags) = interpret("fn main() print(36 ~= 37) end", 0);
    assert!(!diags.has_errors(), "ne_36: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ne_37() {
    let (out, diags) = interpret("fn main() print(37 ~= 38) end", 0);
    assert!(!diags.has_errors(), "ne_37: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ne_38() {
    let (out, diags) = interpret("fn main() print(38 ~= 39) end", 0);
    assert!(!diags.has_errors(), "ne_38: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ne_39() {
    let (out, diags) = interpret("fn main() print(39 ~= 40) end", 0);
    assert!(!diags.has_errors(), "ne_39: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ne_40() {
    let (out, diags) = interpret("fn main() print(40 ~= 41) end", 0);
    assert!(!diags.has_errors(), "ne_40: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ne_41() {
    let (out, diags) = interpret("fn main() print(41 ~= 42) end", 0);
    assert!(!diags.has_errors(), "ne_41: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ne_42() {
    let (out, diags) = interpret("fn main() print(42 ~= 43) end", 0);
    assert!(!diags.has_errors(), "ne_42: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ne_43() {
    let (out, diags) = interpret("fn main() print(43 ~= 44) end", 0);
    assert!(!diags.has_errors(), "ne_43: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ne_44() {
    let (out, diags) = interpret("fn main() print(44 ~= 45) end", 0);
    assert!(!diags.has_errors(), "ne_44: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ne_45() {
    let (out, diags) = interpret("fn main() print(45 ~= 46) end", 0);
    assert!(!diags.has_errors(), "ne_45: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ne_46() {
    let (out, diags) = interpret("fn main() print(46 ~= 47) end", 0);
    assert!(!diags.has_errors(), "ne_46: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ne_47() {
    let (out, diags) = interpret("fn main() print(47 ~= 48) end", 0);
    assert!(!diags.has_errors(), "ne_47: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ne_48() {
    let (out, diags) = interpret("fn main() print(48 ~= 49) end", 0);
    assert!(!diags.has_errors(), "ne_48: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ne_49() {
    let (out, diags) = interpret("fn main() print(49 ~= 50) end", 0);
    assert!(!diags.has_errors(), "ne_49: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ne_50() {
    let (out, diags) = interpret("fn main() print(50 ~= 51) end", 0);
    assert!(!diags.has_errors(), "ne_50: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ne_51() {
    let (out, diags) = interpret("fn main() print(51 ~= 52) end", 0);
    assert!(!diags.has_errors(), "ne_51: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ne_52() {
    let (out, diags) = interpret("fn main() print(52 ~= 53) end", 0);
    assert!(!diags.has_errors(), "ne_52: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ne_53() {
    let (out, diags) = interpret("fn main() print(53 ~= 54) end", 0);
    assert!(!diags.has_errors(), "ne_53: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ne_54() {
    let (out, diags) = interpret("fn main() print(54 ~= 55) end", 0);
    assert!(!diags.has_errors(), "ne_54: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ne_55() {
    let (out, diags) = interpret("fn main() print(55 ~= 56) end", 0);
    assert!(!diags.has_errors(), "ne_55: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ne_56() {
    let (out, diags) = interpret("fn main() print(56 ~= 57) end", 0);
    assert!(!diags.has_errors(), "ne_56: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ne_57() {
    let (out, diags) = interpret("fn main() print(57 ~= 58) end", 0);
    assert!(!diags.has_errors(), "ne_57: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ne_58() {
    let (out, diags) = interpret("fn main() print(58 ~= 59) end", 0);
    assert!(!diags.has_errors(), "ne_58: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ne_59() {
    let (out, diags) = interpret("fn main() print(59 ~= 60) end", 0);
    assert!(!diags.has_errors(), "ne_59: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ne_60() {
    let (out, diags) = interpret("fn main() print(60 ~= 61) end", 0);
    assert!(!diags.has_errors(), "ne_60: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ne_61() {
    let (out, diags) = interpret("fn main() print(61 ~= 62) end", 0);
    assert!(!diags.has_errors(), "ne_61: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ne_62() {
    let (out, diags) = interpret("fn main() print(62 ~= 63) end", 0);
    assert!(!diags.has_errors(), "ne_62: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ne_63() {
    let (out, diags) = interpret("fn main() print(63 ~= 64) end", 0);
    assert!(!diags.has_errors(), "ne_63: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ne_64() {
    let (out, diags) = interpret("fn main() print(64 ~= 65) end", 0);
    assert!(!diags.has_errors(), "ne_64: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ne_65() {
    let (out, diags) = interpret("fn main() print(65 ~= 66) end", 0);
    assert!(!diags.has_errors(), "ne_65: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ne_66() {
    let (out, diags) = interpret("fn main() print(66 ~= 67) end", 0);
    assert!(!diags.has_errors(), "ne_66: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ne_67() {
    let (out, diags) = interpret("fn main() print(67 ~= 68) end", 0);
    assert!(!diags.has_errors(), "ne_67: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ne_68() {
    let (out, diags) = interpret("fn main() print(68 ~= 69) end", 0);
    assert!(!diags.has_errors(), "ne_68: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ne_69() {
    let (out, diags) = interpret("fn main() print(69 ~= 70) end", 0);
    assert!(!diags.has_errors(), "ne_69: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ne_70() {
    let (out, diags) = interpret("fn main() print(70 ~= 71) end", 0);
    assert!(!diags.has_errors(), "ne_70: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ne_71() {
    let (out, diags) = interpret("fn main() print(71 ~= 72) end", 0);
    assert!(!diags.has_errors(), "ne_71: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ne_72() {
    let (out, diags) = interpret("fn main() print(72 ~= 73) end", 0);
    assert!(!diags.has_errors(), "ne_72: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ne_73() {
    let (out, diags) = interpret("fn main() print(73 ~= 74) end", 0);
    assert!(!diags.has_errors(), "ne_73: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ne_74() {
    let (out, diags) = interpret("fn main() print(74 ~= 75) end", 0);
    assert!(!diags.has_errors(), "ne_74: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ne_75() {
    let (out, diags) = interpret("fn main() print(75 ~= 76) end", 0);
    assert!(!diags.has_errors(), "ne_75: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ne_76() {
    let (out, diags) = interpret("fn main() print(76 ~= 77) end", 0);
    assert!(!diags.has_errors(), "ne_76: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ne_77() {
    let (out, diags) = interpret("fn main() print(77 ~= 78) end", 0);
    assert!(!diags.has_errors(), "ne_77: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ne_78() {
    let (out, diags) = interpret("fn main() print(78 ~= 79) end", 0);
    assert!(!diags.has_errors(), "ne_78: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ne_79() {
    let (out, diags) = interpret("fn main() print(79 ~= 80) end", 0);
    assert!(!diags.has_errors(), "ne_79: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ne_80() {
    let (out, diags) = interpret("fn main() print(80 ~= 81) end", 0);
    assert!(!diags.has_errors(), "ne_80: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ne_81() {
    let (out, diags) = interpret("fn main() print(81 ~= 82) end", 0);
    assert!(!diags.has_errors(), "ne_81: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ne_82() {
    let (out, diags) = interpret("fn main() print(82 ~= 83) end", 0);
    assert!(!diags.has_errors(), "ne_82: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ne_83() {
    let (out, diags) = interpret("fn main() print(83 ~= 84) end", 0);
    assert!(!diags.has_errors(), "ne_83: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ne_84() {
    let (out, diags) = interpret("fn main() print(84 ~= 85) end", 0);
    assert!(!diags.has_errors(), "ne_84: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ne_85() {
    let (out, diags) = interpret("fn main() print(85 ~= 86) end", 0);
    assert!(!diags.has_errors(), "ne_85: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ne_86() {
    let (out, diags) = interpret("fn main() print(86 ~= 87) end", 0);
    assert!(!diags.has_errors(), "ne_86: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ne_87() {
    let (out, diags) = interpret("fn main() print(87 ~= 88) end", 0);
    assert!(!diags.has_errors(), "ne_87: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ne_88() {
    let (out, diags) = interpret("fn main() print(88 ~= 89) end", 0);
    assert!(!diags.has_errors(), "ne_88: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ne_89() {
    let (out, diags) = interpret("fn main() print(89 ~= 90) end", 0);
    assert!(!diags.has_errors(), "ne_89: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ne_90() {
    let (out, diags) = interpret("fn main() print(90 ~= 91) end", 0);
    assert!(!diags.has_errors(), "ne_90: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ne_91() {
    let (out, diags) = interpret("fn main() print(91 ~= 92) end", 0);
    assert!(!diags.has_errors(), "ne_91: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ne_92() {
    let (out, diags) = interpret("fn main() print(92 ~= 93) end", 0);
    assert!(!diags.has_errors(), "ne_92: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ne_93() {
    let (out, diags) = interpret("fn main() print(93 ~= 94) end", 0);
    assert!(!diags.has_errors(), "ne_93: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ne_94() {
    let (out, diags) = interpret("fn main() print(94 ~= 95) end", 0);
    assert!(!diags.has_errors(), "ne_94: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ne_95() {
    let (out, diags) = interpret("fn main() print(95 ~= 96) end", 0);
    assert!(!diags.has_errors(), "ne_95: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ne_96() {
    let (out, diags) = interpret("fn main() print(96 ~= 97) end", 0);
    assert!(!diags.has_errors(), "ne_96: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ne_97() {
    let (out, diags) = interpret("fn main() print(97 ~= 98) end", 0);
    assert!(!diags.has_errors(), "ne_97: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ne_98() {
    let (out, diags) = interpret("fn main() print(98 ~= 99) end", 0);
    assert!(!diags.has_errors(), "ne_98: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ne_99() {
    let (out, diags) = interpret("fn main() print(99 ~= 100) end", 0);
    assert!(!diags.has_errors(), "ne_99: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn le_same_0() {
    let (out, diags) = interpret("fn main() print(0 <= 0) end", 0);
    assert!(!diags.has_errors(), "le_same_0: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn le_same_1() {
    let (out, diags) = interpret("fn main() print(1 <= 1) end", 0);
    assert!(!diags.has_errors(), "le_same_1: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn le_same_2() {
    let (out, diags) = interpret("fn main() print(2 <= 2) end", 0);
    assert!(!diags.has_errors(), "le_same_2: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn le_same_3() {
    let (out, diags) = interpret("fn main() print(3 <= 3) end", 0);
    assert!(!diags.has_errors(), "le_same_3: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn le_same_4() {
    let (out, diags) = interpret("fn main() print(4 <= 4) end", 0);
    assert!(!diags.has_errors(), "le_same_4: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn le_same_5() {
    let (out, diags) = interpret("fn main() print(5 <= 5) end", 0);
    assert!(!diags.has_errors(), "le_same_5: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn le_same_6() {
    let (out, diags) = interpret("fn main() print(6 <= 6) end", 0);
    assert!(!diags.has_errors(), "le_same_6: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn le_same_7() {
    let (out, diags) = interpret("fn main() print(7 <= 7) end", 0);
    assert!(!diags.has_errors(), "le_same_7: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn le_same_8() {
    let (out, diags) = interpret("fn main() print(8 <= 8) end", 0);
    assert!(!diags.has_errors(), "le_same_8: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn le_same_9() {
    let (out, diags) = interpret("fn main() print(9 <= 9) end", 0);
    assert!(!diags.has_errors(), "le_same_9: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn le_same_10() {
    let (out, diags) = interpret("fn main() print(10 <= 10) end", 0);
    assert!(!diags.has_errors(), "le_same_10: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn le_same_11() {
    let (out, diags) = interpret("fn main() print(11 <= 11) end", 0);
    assert!(!diags.has_errors(), "le_same_11: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn le_same_12() {
    let (out, diags) = interpret("fn main() print(12 <= 12) end", 0);
    assert!(!diags.has_errors(), "le_same_12: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn le_same_13() {
    let (out, diags) = interpret("fn main() print(13 <= 13) end", 0);
    assert!(!diags.has_errors(), "le_same_13: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn le_same_14() {
    let (out, diags) = interpret("fn main() print(14 <= 14) end", 0);
    assert!(!diags.has_errors(), "le_same_14: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn le_same_15() {
    let (out, diags) = interpret("fn main() print(15 <= 15) end", 0);
    assert!(!diags.has_errors(), "le_same_15: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn le_same_16() {
    let (out, diags) = interpret("fn main() print(16 <= 16) end", 0);
    assert!(!diags.has_errors(), "le_same_16: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn le_same_17() {
    let (out, diags) = interpret("fn main() print(17 <= 17) end", 0);
    assert!(!diags.has_errors(), "le_same_17: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn le_same_18() {
    let (out, diags) = interpret("fn main() print(18 <= 18) end", 0);
    assert!(!diags.has_errors(), "le_same_18: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn le_same_19() {
    let (out, diags) = interpret("fn main() print(19 <= 19) end", 0);
    assert!(!diags.has_errors(), "le_same_19: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn le_same_20() {
    let (out, diags) = interpret("fn main() print(20 <= 20) end", 0);
    assert!(!diags.has_errors(), "le_same_20: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn le_same_21() {
    let (out, diags) = interpret("fn main() print(21 <= 21) end", 0);
    assert!(!diags.has_errors(), "le_same_21: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn le_same_22() {
    let (out, diags) = interpret("fn main() print(22 <= 22) end", 0);
    assert!(!diags.has_errors(), "le_same_22: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn le_same_23() {
    let (out, diags) = interpret("fn main() print(23 <= 23) end", 0);
    assert!(!diags.has_errors(), "le_same_23: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn le_same_24() {
    let (out, diags) = interpret("fn main() print(24 <= 24) end", 0);
    assert!(!diags.has_errors(), "le_same_24: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn le_same_25() {
    let (out, diags) = interpret("fn main() print(25 <= 25) end", 0);
    assert!(!diags.has_errors(), "le_same_25: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn le_same_26() {
    let (out, diags) = interpret("fn main() print(26 <= 26) end", 0);
    assert!(!diags.has_errors(), "le_same_26: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn le_same_27() {
    let (out, diags) = interpret("fn main() print(27 <= 27) end", 0);
    assert!(!diags.has_errors(), "le_same_27: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn le_same_28() {
    let (out, diags) = interpret("fn main() print(28 <= 28) end", 0);
    assert!(!diags.has_errors(), "le_same_28: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn le_same_29() {
    let (out, diags) = interpret("fn main() print(29 <= 29) end", 0);
    assert!(!diags.has_errors(), "le_same_29: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn le_same_30() {
    let (out, diags) = interpret("fn main() print(30 <= 30) end", 0);
    assert!(!diags.has_errors(), "le_same_30: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn le_same_31() {
    let (out, diags) = interpret("fn main() print(31 <= 31) end", 0);
    assert!(!diags.has_errors(), "le_same_31: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn le_same_32() {
    let (out, diags) = interpret("fn main() print(32 <= 32) end", 0);
    assert!(!diags.has_errors(), "le_same_32: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn le_same_33() {
    let (out, diags) = interpret("fn main() print(33 <= 33) end", 0);
    assert!(!diags.has_errors(), "le_same_33: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn le_same_34() {
    let (out, diags) = interpret("fn main() print(34 <= 34) end", 0);
    assert!(!diags.has_errors(), "le_same_34: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn le_same_35() {
    let (out, diags) = interpret("fn main() print(35 <= 35) end", 0);
    assert!(!diags.has_errors(), "le_same_35: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn le_same_36() {
    let (out, diags) = interpret("fn main() print(36 <= 36) end", 0);
    assert!(!diags.has_errors(), "le_same_36: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn le_same_37() {
    let (out, diags) = interpret("fn main() print(37 <= 37) end", 0);
    assert!(!diags.has_errors(), "le_same_37: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn le_same_38() {
    let (out, diags) = interpret("fn main() print(38 <= 38) end", 0);
    assert!(!diags.has_errors(), "le_same_38: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn le_same_39() {
    let (out, diags) = interpret("fn main() print(39 <= 39) end", 0);
    assert!(!diags.has_errors(), "le_same_39: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn le_same_40() {
    let (out, diags) = interpret("fn main() print(40 <= 40) end", 0);
    assert!(!diags.has_errors(), "le_same_40: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn le_same_41() {
    let (out, diags) = interpret("fn main() print(41 <= 41) end", 0);
    assert!(!diags.has_errors(), "le_same_41: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn le_same_42() {
    let (out, diags) = interpret("fn main() print(42 <= 42) end", 0);
    assert!(!diags.has_errors(), "le_same_42: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn le_same_43() {
    let (out, diags) = interpret("fn main() print(43 <= 43) end", 0);
    assert!(!diags.has_errors(), "le_same_43: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn le_same_44() {
    let (out, diags) = interpret("fn main() print(44 <= 44) end", 0);
    assert!(!diags.has_errors(), "le_same_44: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn le_same_45() {
    let (out, diags) = interpret("fn main() print(45 <= 45) end", 0);
    assert!(!diags.has_errors(), "le_same_45: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn le_same_46() {
    let (out, diags) = interpret("fn main() print(46 <= 46) end", 0);
    assert!(!diags.has_errors(), "le_same_46: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn le_same_47() {
    let (out, diags) = interpret("fn main() print(47 <= 47) end", 0);
    assert!(!diags.has_errors(), "le_same_47: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn le_same_48() {
    let (out, diags) = interpret("fn main() print(48 <= 48) end", 0);
    assert!(!diags.has_errors(), "le_same_48: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn le_same_49() {
    let (out, diags) = interpret("fn main() print(49 <= 49) end", 0);
    assert!(!diags.has_errors(), "le_same_49: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn le_same_50() {
    let (out, diags) = interpret("fn main() print(50 <= 50) end", 0);
    assert!(!diags.has_errors(), "le_same_50: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn le_same_51() {
    let (out, diags) = interpret("fn main() print(51 <= 51) end", 0);
    assert!(!diags.has_errors(), "le_same_51: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn le_same_52() {
    let (out, diags) = interpret("fn main() print(52 <= 52) end", 0);
    assert!(!diags.has_errors(), "le_same_52: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn le_same_53() {
    let (out, diags) = interpret("fn main() print(53 <= 53) end", 0);
    assert!(!diags.has_errors(), "le_same_53: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn le_same_54() {
    let (out, diags) = interpret("fn main() print(54 <= 54) end", 0);
    assert!(!diags.has_errors(), "le_same_54: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn le_same_55() {
    let (out, diags) = interpret("fn main() print(55 <= 55) end", 0);
    assert!(!diags.has_errors(), "le_same_55: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn le_same_56() {
    let (out, diags) = interpret("fn main() print(56 <= 56) end", 0);
    assert!(!diags.has_errors(), "le_same_56: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn le_same_57() {
    let (out, diags) = interpret("fn main() print(57 <= 57) end", 0);
    assert!(!diags.has_errors(), "le_same_57: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn le_same_58() {
    let (out, diags) = interpret("fn main() print(58 <= 58) end", 0);
    assert!(!diags.has_errors(), "le_same_58: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn le_same_59() {
    let (out, diags) = interpret("fn main() print(59 <= 59) end", 0);
    assert!(!diags.has_errors(), "le_same_59: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn le_same_60() {
    let (out, diags) = interpret("fn main() print(60 <= 60) end", 0);
    assert!(!diags.has_errors(), "le_same_60: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn le_same_61() {
    let (out, diags) = interpret("fn main() print(61 <= 61) end", 0);
    assert!(!diags.has_errors(), "le_same_61: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn le_same_62() {
    let (out, diags) = interpret("fn main() print(62 <= 62) end", 0);
    assert!(!diags.has_errors(), "le_same_62: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn le_same_63() {
    let (out, diags) = interpret("fn main() print(63 <= 63) end", 0);
    assert!(!diags.has_errors(), "le_same_63: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn le_same_64() {
    let (out, diags) = interpret("fn main() print(64 <= 64) end", 0);
    assert!(!diags.has_errors(), "le_same_64: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn le_same_65() {
    let (out, diags) = interpret("fn main() print(65 <= 65) end", 0);
    assert!(!diags.has_errors(), "le_same_65: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn le_same_66() {
    let (out, diags) = interpret("fn main() print(66 <= 66) end", 0);
    assert!(!diags.has_errors(), "le_same_66: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn le_same_67() {
    let (out, diags) = interpret("fn main() print(67 <= 67) end", 0);
    assert!(!diags.has_errors(), "le_same_67: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn le_same_68() {
    let (out, diags) = interpret("fn main() print(68 <= 68) end", 0);
    assert!(!diags.has_errors(), "le_same_68: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn le_same_69() {
    let (out, diags) = interpret("fn main() print(69 <= 69) end", 0);
    assert!(!diags.has_errors(), "le_same_69: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn le_same_70() {
    let (out, diags) = interpret("fn main() print(70 <= 70) end", 0);
    assert!(!diags.has_errors(), "le_same_70: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn le_same_71() {
    let (out, diags) = interpret("fn main() print(71 <= 71) end", 0);
    assert!(!diags.has_errors(), "le_same_71: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn le_same_72() {
    let (out, diags) = interpret("fn main() print(72 <= 72) end", 0);
    assert!(!diags.has_errors(), "le_same_72: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn le_same_73() {
    let (out, diags) = interpret("fn main() print(73 <= 73) end", 0);
    assert!(!diags.has_errors(), "le_same_73: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn le_same_74() {
    let (out, diags) = interpret("fn main() print(74 <= 74) end", 0);
    assert!(!diags.has_errors(), "le_same_74: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn le_same_75() {
    let (out, diags) = interpret("fn main() print(75 <= 75) end", 0);
    assert!(!diags.has_errors(), "le_same_75: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn le_same_76() {
    let (out, diags) = interpret("fn main() print(76 <= 76) end", 0);
    assert!(!diags.has_errors(), "le_same_76: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn le_same_77() {
    let (out, diags) = interpret("fn main() print(77 <= 77) end", 0);
    assert!(!diags.has_errors(), "le_same_77: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn le_same_78() {
    let (out, diags) = interpret("fn main() print(78 <= 78) end", 0);
    assert!(!diags.has_errors(), "le_same_78: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn le_same_79() {
    let (out, diags) = interpret("fn main() print(79 <= 79) end", 0);
    assert!(!diags.has_errors(), "le_same_79: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn le_same_80() {
    let (out, diags) = interpret("fn main() print(80 <= 80) end", 0);
    assert!(!diags.has_errors(), "le_same_80: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn le_same_81() {
    let (out, diags) = interpret("fn main() print(81 <= 81) end", 0);
    assert!(!diags.has_errors(), "le_same_81: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn le_same_82() {
    let (out, diags) = interpret("fn main() print(82 <= 82) end", 0);
    assert!(!diags.has_errors(), "le_same_82: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn le_same_83() {
    let (out, diags) = interpret("fn main() print(83 <= 83) end", 0);
    assert!(!diags.has_errors(), "le_same_83: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn le_same_84() {
    let (out, diags) = interpret("fn main() print(84 <= 84) end", 0);
    assert!(!diags.has_errors(), "le_same_84: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn le_same_85() {
    let (out, diags) = interpret("fn main() print(85 <= 85) end", 0);
    assert!(!diags.has_errors(), "le_same_85: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn le_same_86() {
    let (out, diags) = interpret("fn main() print(86 <= 86) end", 0);
    assert!(!diags.has_errors(), "le_same_86: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn le_same_87() {
    let (out, diags) = interpret("fn main() print(87 <= 87) end", 0);
    assert!(!diags.has_errors(), "le_same_87: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn le_same_88() {
    let (out, diags) = interpret("fn main() print(88 <= 88) end", 0);
    assert!(!diags.has_errors(), "le_same_88: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn le_same_89() {
    let (out, diags) = interpret("fn main() print(89 <= 89) end", 0);
    assert!(!diags.has_errors(), "le_same_89: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn le_same_90() {
    let (out, diags) = interpret("fn main() print(90 <= 90) end", 0);
    assert!(!diags.has_errors(), "le_same_90: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn le_same_91() {
    let (out, diags) = interpret("fn main() print(91 <= 91) end", 0);
    assert!(!diags.has_errors(), "le_same_91: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn le_same_92() {
    let (out, diags) = interpret("fn main() print(92 <= 92) end", 0);
    assert!(!diags.has_errors(), "le_same_92: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn le_same_93() {
    let (out, diags) = interpret("fn main() print(93 <= 93) end", 0);
    assert!(!diags.has_errors(), "le_same_93: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn le_same_94() {
    let (out, diags) = interpret("fn main() print(94 <= 94) end", 0);
    assert!(!diags.has_errors(), "le_same_94: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn le_same_95() {
    let (out, diags) = interpret("fn main() print(95 <= 95) end", 0);
    assert!(!diags.has_errors(), "le_same_95: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn le_same_96() {
    let (out, diags) = interpret("fn main() print(96 <= 96) end", 0);
    assert!(!diags.has_errors(), "le_same_96: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn le_same_97() {
    let (out, diags) = interpret("fn main() print(97 <= 97) end", 0);
    assert!(!diags.has_errors(), "le_same_97: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn le_same_98() {
    let (out, diags) = interpret("fn main() print(98 <= 98) end", 0);
    assert!(!diags.has_errors(), "le_same_98: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn le_same_99() {
    let (out, diags) = interpret("fn main() print(99 <= 99) end", 0);
    assert!(!diags.has_errors(), "le_same_99: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ge_same_0() {
    let (out, diags) = interpret("fn main() print(0 >= 0) end", 0);
    assert!(!diags.has_errors(), "ge_same_0: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ge_same_1() {
    let (out, diags) = interpret("fn main() print(1 >= 1) end", 0);
    assert!(!diags.has_errors(), "ge_same_1: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ge_same_2() {
    let (out, diags) = interpret("fn main() print(2 >= 2) end", 0);
    assert!(!diags.has_errors(), "ge_same_2: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ge_same_3() {
    let (out, diags) = interpret("fn main() print(3 >= 3) end", 0);
    assert!(!diags.has_errors(), "ge_same_3: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ge_same_4() {
    let (out, diags) = interpret("fn main() print(4 >= 4) end", 0);
    assert!(!diags.has_errors(), "ge_same_4: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ge_same_5() {
    let (out, diags) = interpret("fn main() print(5 >= 5) end", 0);
    assert!(!diags.has_errors(), "ge_same_5: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ge_same_6() {
    let (out, diags) = interpret("fn main() print(6 >= 6) end", 0);
    assert!(!diags.has_errors(), "ge_same_6: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ge_same_7() {
    let (out, diags) = interpret("fn main() print(7 >= 7) end", 0);
    assert!(!diags.has_errors(), "ge_same_7: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ge_same_8() {
    let (out, diags) = interpret("fn main() print(8 >= 8) end", 0);
    assert!(!diags.has_errors(), "ge_same_8: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ge_same_9() {
    let (out, diags) = interpret("fn main() print(9 >= 9) end", 0);
    assert!(!diags.has_errors(), "ge_same_9: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ge_same_10() {
    let (out, diags) = interpret("fn main() print(10 >= 10) end", 0);
    assert!(!diags.has_errors(), "ge_same_10: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ge_same_11() {
    let (out, diags) = interpret("fn main() print(11 >= 11) end", 0);
    assert!(!diags.has_errors(), "ge_same_11: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ge_same_12() {
    let (out, diags) = interpret("fn main() print(12 >= 12) end", 0);
    assert!(!diags.has_errors(), "ge_same_12: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ge_same_13() {
    let (out, diags) = interpret("fn main() print(13 >= 13) end", 0);
    assert!(!diags.has_errors(), "ge_same_13: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ge_same_14() {
    let (out, diags) = interpret("fn main() print(14 >= 14) end", 0);
    assert!(!diags.has_errors(), "ge_same_14: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ge_same_15() {
    let (out, diags) = interpret("fn main() print(15 >= 15) end", 0);
    assert!(!diags.has_errors(), "ge_same_15: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ge_same_16() {
    let (out, diags) = interpret("fn main() print(16 >= 16) end", 0);
    assert!(!diags.has_errors(), "ge_same_16: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ge_same_17() {
    let (out, diags) = interpret("fn main() print(17 >= 17) end", 0);
    assert!(!diags.has_errors(), "ge_same_17: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ge_same_18() {
    let (out, diags) = interpret("fn main() print(18 >= 18) end", 0);
    assert!(!diags.has_errors(), "ge_same_18: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ge_same_19() {
    let (out, diags) = interpret("fn main() print(19 >= 19) end", 0);
    assert!(!diags.has_errors(), "ge_same_19: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ge_same_20() {
    let (out, diags) = interpret("fn main() print(20 >= 20) end", 0);
    assert!(!diags.has_errors(), "ge_same_20: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ge_same_21() {
    let (out, diags) = interpret("fn main() print(21 >= 21) end", 0);
    assert!(!diags.has_errors(), "ge_same_21: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ge_same_22() {
    let (out, diags) = interpret("fn main() print(22 >= 22) end", 0);
    assert!(!diags.has_errors(), "ge_same_22: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ge_same_23() {
    let (out, diags) = interpret("fn main() print(23 >= 23) end", 0);
    assert!(!diags.has_errors(), "ge_same_23: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ge_same_24() {
    let (out, diags) = interpret("fn main() print(24 >= 24) end", 0);
    assert!(!diags.has_errors(), "ge_same_24: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ge_same_25() {
    let (out, diags) = interpret("fn main() print(25 >= 25) end", 0);
    assert!(!diags.has_errors(), "ge_same_25: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ge_same_26() {
    let (out, diags) = interpret("fn main() print(26 >= 26) end", 0);
    assert!(!diags.has_errors(), "ge_same_26: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ge_same_27() {
    let (out, diags) = interpret("fn main() print(27 >= 27) end", 0);
    assert!(!diags.has_errors(), "ge_same_27: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ge_same_28() {
    let (out, diags) = interpret("fn main() print(28 >= 28) end", 0);
    assert!(!diags.has_errors(), "ge_same_28: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ge_same_29() {
    let (out, diags) = interpret("fn main() print(29 >= 29) end", 0);
    assert!(!diags.has_errors(), "ge_same_29: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ge_same_30() {
    let (out, diags) = interpret("fn main() print(30 >= 30) end", 0);
    assert!(!diags.has_errors(), "ge_same_30: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ge_same_31() {
    let (out, diags) = interpret("fn main() print(31 >= 31) end", 0);
    assert!(!diags.has_errors(), "ge_same_31: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ge_same_32() {
    let (out, diags) = interpret("fn main() print(32 >= 32) end", 0);
    assert!(!diags.has_errors(), "ge_same_32: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ge_same_33() {
    let (out, diags) = interpret("fn main() print(33 >= 33) end", 0);
    assert!(!diags.has_errors(), "ge_same_33: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ge_same_34() {
    let (out, diags) = interpret("fn main() print(34 >= 34) end", 0);
    assert!(!diags.has_errors(), "ge_same_34: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ge_same_35() {
    let (out, diags) = interpret("fn main() print(35 >= 35) end", 0);
    assert!(!diags.has_errors(), "ge_same_35: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ge_same_36() {
    let (out, diags) = interpret("fn main() print(36 >= 36) end", 0);
    assert!(!diags.has_errors(), "ge_same_36: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ge_same_37() {
    let (out, diags) = interpret("fn main() print(37 >= 37) end", 0);
    assert!(!diags.has_errors(), "ge_same_37: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ge_same_38() {
    let (out, diags) = interpret("fn main() print(38 >= 38) end", 0);
    assert!(!diags.has_errors(), "ge_same_38: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ge_same_39() {
    let (out, diags) = interpret("fn main() print(39 >= 39) end", 0);
    assert!(!diags.has_errors(), "ge_same_39: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ge_same_40() {
    let (out, diags) = interpret("fn main() print(40 >= 40) end", 0);
    assert!(!diags.has_errors(), "ge_same_40: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ge_same_41() {
    let (out, diags) = interpret("fn main() print(41 >= 41) end", 0);
    assert!(!diags.has_errors(), "ge_same_41: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ge_same_42() {
    let (out, diags) = interpret("fn main() print(42 >= 42) end", 0);
    assert!(!diags.has_errors(), "ge_same_42: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ge_same_43() {
    let (out, diags) = interpret("fn main() print(43 >= 43) end", 0);
    assert!(!diags.has_errors(), "ge_same_43: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ge_same_44() {
    let (out, diags) = interpret("fn main() print(44 >= 44) end", 0);
    assert!(!diags.has_errors(), "ge_same_44: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ge_same_45() {
    let (out, diags) = interpret("fn main() print(45 >= 45) end", 0);
    assert!(!diags.has_errors(), "ge_same_45: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ge_same_46() {
    let (out, diags) = interpret("fn main() print(46 >= 46) end", 0);
    assert!(!diags.has_errors(), "ge_same_46: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ge_same_47() {
    let (out, diags) = interpret("fn main() print(47 >= 47) end", 0);
    assert!(!diags.has_errors(), "ge_same_47: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ge_same_48() {
    let (out, diags) = interpret("fn main() print(48 >= 48) end", 0);
    assert!(!diags.has_errors(), "ge_same_48: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ge_same_49() {
    let (out, diags) = interpret("fn main() print(49 >= 49) end", 0);
    assert!(!diags.has_errors(), "ge_same_49: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ge_same_50() {
    let (out, diags) = interpret("fn main() print(50 >= 50) end", 0);
    assert!(!diags.has_errors(), "ge_same_50: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ge_same_51() {
    let (out, diags) = interpret("fn main() print(51 >= 51) end", 0);
    assert!(!diags.has_errors(), "ge_same_51: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ge_same_52() {
    let (out, diags) = interpret("fn main() print(52 >= 52) end", 0);
    assert!(!diags.has_errors(), "ge_same_52: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ge_same_53() {
    let (out, diags) = interpret("fn main() print(53 >= 53) end", 0);
    assert!(!diags.has_errors(), "ge_same_53: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ge_same_54() {
    let (out, diags) = interpret("fn main() print(54 >= 54) end", 0);
    assert!(!diags.has_errors(), "ge_same_54: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ge_same_55() {
    let (out, diags) = interpret("fn main() print(55 >= 55) end", 0);
    assert!(!diags.has_errors(), "ge_same_55: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ge_same_56() {
    let (out, diags) = interpret("fn main() print(56 >= 56) end", 0);
    assert!(!diags.has_errors(), "ge_same_56: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ge_same_57() {
    let (out, diags) = interpret("fn main() print(57 >= 57) end", 0);
    assert!(!diags.has_errors(), "ge_same_57: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ge_same_58() {
    let (out, diags) = interpret("fn main() print(58 >= 58) end", 0);
    assert!(!diags.has_errors(), "ge_same_58: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ge_same_59() {
    let (out, diags) = interpret("fn main() print(59 >= 59) end", 0);
    assert!(!diags.has_errors(), "ge_same_59: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ge_same_60() {
    let (out, diags) = interpret("fn main() print(60 >= 60) end", 0);
    assert!(!diags.has_errors(), "ge_same_60: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ge_same_61() {
    let (out, diags) = interpret("fn main() print(61 >= 61) end", 0);
    assert!(!diags.has_errors(), "ge_same_61: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ge_same_62() {
    let (out, diags) = interpret("fn main() print(62 >= 62) end", 0);
    assert!(!diags.has_errors(), "ge_same_62: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ge_same_63() {
    let (out, diags) = interpret("fn main() print(63 >= 63) end", 0);
    assert!(!diags.has_errors(), "ge_same_63: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ge_same_64() {
    let (out, diags) = interpret("fn main() print(64 >= 64) end", 0);
    assert!(!diags.has_errors(), "ge_same_64: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ge_same_65() {
    let (out, diags) = interpret("fn main() print(65 >= 65) end", 0);
    assert!(!diags.has_errors(), "ge_same_65: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ge_same_66() {
    let (out, diags) = interpret("fn main() print(66 >= 66) end", 0);
    assert!(!diags.has_errors(), "ge_same_66: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ge_same_67() {
    let (out, diags) = interpret("fn main() print(67 >= 67) end", 0);
    assert!(!diags.has_errors(), "ge_same_67: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ge_same_68() {
    let (out, diags) = interpret("fn main() print(68 >= 68) end", 0);
    assert!(!diags.has_errors(), "ge_same_68: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ge_same_69() {
    let (out, diags) = interpret("fn main() print(69 >= 69) end", 0);
    assert!(!diags.has_errors(), "ge_same_69: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ge_same_70() {
    let (out, diags) = interpret("fn main() print(70 >= 70) end", 0);
    assert!(!diags.has_errors(), "ge_same_70: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ge_same_71() {
    let (out, diags) = interpret("fn main() print(71 >= 71) end", 0);
    assert!(!diags.has_errors(), "ge_same_71: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ge_same_72() {
    let (out, diags) = interpret("fn main() print(72 >= 72) end", 0);
    assert!(!diags.has_errors(), "ge_same_72: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ge_same_73() {
    let (out, diags) = interpret("fn main() print(73 >= 73) end", 0);
    assert!(!diags.has_errors(), "ge_same_73: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ge_same_74() {
    let (out, diags) = interpret("fn main() print(74 >= 74) end", 0);
    assert!(!diags.has_errors(), "ge_same_74: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ge_same_75() {
    let (out, diags) = interpret("fn main() print(75 >= 75) end", 0);
    assert!(!diags.has_errors(), "ge_same_75: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ge_same_76() {
    let (out, diags) = interpret("fn main() print(76 >= 76) end", 0);
    assert!(!diags.has_errors(), "ge_same_76: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ge_same_77() {
    let (out, diags) = interpret("fn main() print(77 >= 77) end", 0);
    assert!(!diags.has_errors(), "ge_same_77: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ge_same_78() {
    let (out, diags) = interpret("fn main() print(78 >= 78) end", 0);
    assert!(!diags.has_errors(), "ge_same_78: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ge_same_79() {
    let (out, diags) = interpret("fn main() print(79 >= 79) end", 0);
    assert!(!diags.has_errors(), "ge_same_79: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ge_same_80() {
    let (out, diags) = interpret("fn main() print(80 >= 80) end", 0);
    assert!(!diags.has_errors(), "ge_same_80: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ge_same_81() {
    let (out, diags) = interpret("fn main() print(81 >= 81) end", 0);
    assert!(!diags.has_errors(), "ge_same_81: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ge_same_82() {
    let (out, diags) = interpret("fn main() print(82 >= 82) end", 0);
    assert!(!diags.has_errors(), "ge_same_82: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ge_same_83() {
    let (out, diags) = interpret("fn main() print(83 >= 83) end", 0);
    assert!(!diags.has_errors(), "ge_same_83: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ge_same_84() {
    let (out, diags) = interpret("fn main() print(84 >= 84) end", 0);
    assert!(!diags.has_errors(), "ge_same_84: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ge_same_85() {
    let (out, diags) = interpret("fn main() print(85 >= 85) end", 0);
    assert!(!diags.has_errors(), "ge_same_85: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ge_same_86() {
    let (out, diags) = interpret("fn main() print(86 >= 86) end", 0);
    assert!(!diags.has_errors(), "ge_same_86: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ge_same_87() {
    let (out, diags) = interpret("fn main() print(87 >= 87) end", 0);
    assert!(!diags.has_errors(), "ge_same_87: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ge_same_88() {
    let (out, diags) = interpret("fn main() print(88 >= 88) end", 0);
    assert!(!diags.has_errors(), "ge_same_88: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ge_same_89() {
    let (out, diags) = interpret("fn main() print(89 >= 89) end", 0);
    assert!(!diags.has_errors(), "ge_same_89: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ge_same_90() {
    let (out, diags) = interpret("fn main() print(90 >= 90) end", 0);
    assert!(!diags.has_errors(), "ge_same_90: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ge_same_91() {
    let (out, diags) = interpret("fn main() print(91 >= 91) end", 0);
    assert!(!diags.has_errors(), "ge_same_91: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ge_same_92() {
    let (out, diags) = interpret("fn main() print(92 >= 92) end", 0);
    assert!(!diags.has_errors(), "ge_same_92: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ge_same_93() {
    let (out, diags) = interpret("fn main() print(93 >= 93) end", 0);
    assert!(!diags.has_errors(), "ge_same_93: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ge_same_94() {
    let (out, diags) = interpret("fn main() print(94 >= 94) end", 0);
    assert!(!diags.has_errors(), "ge_same_94: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ge_same_95() {
    let (out, diags) = interpret("fn main() print(95 >= 95) end", 0);
    assert!(!diags.has_errors(), "ge_same_95: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ge_same_96() {
    let (out, diags) = interpret("fn main() print(96 >= 96) end", 0);
    assert!(!diags.has_errors(), "ge_same_96: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ge_same_97() {
    let (out, diags) = interpret("fn main() print(97 >= 97) end", 0);
    assert!(!diags.has_errors(), "ge_same_97: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ge_same_98() {
    let (out, diags) = interpret("fn main() print(98 >= 98) end", 0);
    assert!(!diags.has_errors(), "ge_same_98: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

#[test]
fn ge_same_99() {
    let (out, diags) = interpret("fn main() print(99 >= 99) end", 0);
    assert!(!diags.has_errors(), "ge_same_99: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"true
");
}

