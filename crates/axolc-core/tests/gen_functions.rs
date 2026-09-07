// Owner: PascalElixir / axolrs (GitHub org)
// File: Auto-generated tests for function definitions.

use axolc_core::interpret;

#[test]
fn fn_add_0() {
    let (out, diags) = interpret("fn add(a, b) return a + b end fn main() print(add(0, 10)) end", 0);
    assert!(!diags.has_errors(), "fn_add_0: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"10
");
}

#[test]
fn fn_add_1() {
    let (out, diags) = interpret("fn add(a, b) return a + b end fn main() print(add(1, 11)) end", 0);
    assert!(!diags.has_errors(), "fn_add_1: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"12
");
}

#[test]
fn fn_add_2() {
    let (out, diags) = interpret("fn add(a, b) return a + b end fn main() print(add(2, 12)) end", 0);
    assert!(!diags.has_errors(), "fn_add_2: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"14
");
}

#[test]
fn fn_add_3() {
    let (out, diags) = interpret("fn add(a, b) return a + b end fn main() print(add(3, 13)) end", 0);
    assert!(!diags.has_errors(), "fn_add_3: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"16
");
}

#[test]
fn fn_add_4() {
    let (out, diags) = interpret("fn add(a, b) return a + b end fn main() print(add(4, 14)) end", 0);
    assert!(!diags.has_errors(), "fn_add_4: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"18
");
}

#[test]
fn fn_add_5() {
    let (out, diags) = interpret("fn add(a, b) return a + b end fn main() print(add(5, 15)) end", 0);
    assert!(!diags.has_errors(), "fn_add_5: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"20
");
}

#[test]
fn fn_add_6() {
    let (out, diags) = interpret("fn add(a, b) return a + b end fn main() print(add(6, 16)) end", 0);
    assert!(!diags.has_errors(), "fn_add_6: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"22
");
}

#[test]
fn fn_add_7() {
    let (out, diags) = interpret("fn add(a, b) return a + b end fn main() print(add(7, 17)) end", 0);
    assert!(!diags.has_errors(), "fn_add_7: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"24
");
}

#[test]
fn fn_add_8() {
    let (out, diags) = interpret("fn add(a, b) return a + b end fn main() print(add(8, 18)) end", 0);
    assert!(!diags.has_errors(), "fn_add_8: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"26
");
}

#[test]
fn fn_add_9() {
    let (out, diags) = interpret("fn add(a, b) return a + b end fn main() print(add(9, 19)) end", 0);
    assert!(!diags.has_errors(), "fn_add_9: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"28
");
}

#[test]
fn fn_add_10() {
    let (out, diags) = interpret("fn add(a, b) return a + b end fn main() print(add(10, 20)) end", 0);
    assert!(!diags.has_errors(), "fn_add_10: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"30
");
}

#[test]
fn fn_add_11() {
    let (out, diags) = interpret("fn add(a, b) return a + b end fn main() print(add(11, 21)) end", 0);
    assert!(!diags.has_errors(), "fn_add_11: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"32
");
}

#[test]
fn fn_add_12() {
    let (out, diags) = interpret("fn add(a, b) return a + b end fn main() print(add(12, 22)) end", 0);
    assert!(!diags.has_errors(), "fn_add_12: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"34
");
}

#[test]
fn fn_add_13() {
    let (out, diags) = interpret("fn add(a, b) return a + b end fn main() print(add(13, 23)) end", 0);
    assert!(!diags.has_errors(), "fn_add_13: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"36
");
}

#[test]
fn fn_add_14() {
    let (out, diags) = interpret("fn add(a, b) return a + b end fn main() print(add(14, 24)) end", 0);
    assert!(!diags.has_errors(), "fn_add_14: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"38
");
}

#[test]
fn fn_add_15() {
    let (out, diags) = interpret("fn add(a, b) return a + b end fn main() print(add(15, 25)) end", 0);
    assert!(!diags.has_errors(), "fn_add_15: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"40
");
}

#[test]
fn fn_add_16() {
    let (out, diags) = interpret("fn add(a, b) return a + b end fn main() print(add(16, 26)) end", 0);
    assert!(!diags.has_errors(), "fn_add_16: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"42
");
}

#[test]
fn fn_add_17() {
    let (out, diags) = interpret("fn add(a, b) return a + b end fn main() print(add(17, 27)) end", 0);
    assert!(!diags.has_errors(), "fn_add_17: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"44
");
}

#[test]
fn fn_add_18() {
    let (out, diags) = interpret("fn add(a, b) return a + b end fn main() print(add(18, 28)) end", 0);
    assert!(!diags.has_errors(), "fn_add_18: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"46
");
}

#[test]
fn fn_add_19() {
    let (out, diags) = interpret("fn add(a, b) return a + b end fn main() print(add(19, 29)) end", 0);
    assert!(!diags.has_errors(), "fn_add_19: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"48
");
}

#[test]
fn fn_add_20() {
    let (out, diags) = interpret("fn add(a, b) return a + b end fn main() print(add(20, 30)) end", 0);
    assert!(!diags.has_errors(), "fn_add_20: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"50
");
}

#[test]
fn fn_add_21() {
    let (out, diags) = interpret("fn add(a, b) return a + b end fn main() print(add(21, 31)) end", 0);
    assert!(!diags.has_errors(), "fn_add_21: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"52
");
}

#[test]
fn fn_add_22() {
    let (out, diags) = interpret("fn add(a, b) return a + b end fn main() print(add(22, 32)) end", 0);
    assert!(!diags.has_errors(), "fn_add_22: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"54
");
}

#[test]
fn fn_add_23() {
    let (out, diags) = interpret("fn add(a, b) return a + b end fn main() print(add(23, 33)) end", 0);
    assert!(!diags.has_errors(), "fn_add_23: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"56
");
}

#[test]
fn fn_add_24() {
    let (out, diags) = interpret("fn add(a, b) return a + b end fn main() print(add(24, 34)) end", 0);
    assert!(!diags.has_errors(), "fn_add_24: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"58
");
}

#[test]
fn fn_add_25() {
    let (out, diags) = interpret("fn add(a, b) return a + b end fn main() print(add(25, 35)) end", 0);
    assert!(!diags.has_errors(), "fn_add_25: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"60
");
}

#[test]
fn fn_add_26() {
    let (out, diags) = interpret("fn add(a, b) return a + b end fn main() print(add(26, 36)) end", 0);
    assert!(!diags.has_errors(), "fn_add_26: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"62
");
}

#[test]
fn fn_add_27() {
    let (out, diags) = interpret("fn add(a, b) return a + b end fn main() print(add(27, 37)) end", 0);
    assert!(!diags.has_errors(), "fn_add_27: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"64
");
}

#[test]
fn fn_add_28() {
    let (out, diags) = interpret("fn add(a, b) return a + b end fn main() print(add(28, 38)) end", 0);
    assert!(!diags.has_errors(), "fn_add_28: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"66
");
}

#[test]
fn fn_add_29() {
    let (out, diags) = interpret("fn add(a, b) return a + b end fn main() print(add(29, 39)) end", 0);
    assert!(!diags.has_errors(), "fn_add_29: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"68
");
}

#[test]
fn fn_add_30() {
    let (out, diags) = interpret("fn add(a, b) return a + b end fn main() print(add(30, 40)) end", 0);
    assert!(!diags.has_errors(), "fn_add_30: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"70
");
}

#[test]
fn fn_add_31() {
    let (out, diags) = interpret("fn add(a, b) return a + b end fn main() print(add(31, 41)) end", 0);
    assert!(!diags.has_errors(), "fn_add_31: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"72
");
}

#[test]
fn fn_add_32() {
    let (out, diags) = interpret("fn add(a, b) return a + b end fn main() print(add(32, 42)) end", 0);
    assert!(!diags.has_errors(), "fn_add_32: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"74
");
}

#[test]
fn fn_add_33() {
    let (out, diags) = interpret("fn add(a, b) return a + b end fn main() print(add(33, 43)) end", 0);
    assert!(!diags.has_errors(), "fn_add_33: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"76
");
}

#[test]
fn fn_add_34() {
    let (out, diags) = interpret("fn add(a, b) return a + b end fn main() print(add(34, 44)) end", 0);
    assert!(!diags.has_errors(), "fn_add_34: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"78
");
}

#[test]
fn fn_add_35() {
    let (out, diags) = interpret("fn add(a, b) return a + b end fn main() print(add(35, 45)) end", 0);
    assert!(!diags.has_errors(), "fn_add_35: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"80
");
}

#[test]
fn fn_add_36() {
    let (out, diags) = interpret("fn add(a, b) return a + b end fn main() print(add(36, 46)) end", 0);
    assert!(!diags.has_errors(), "fn_add_36: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"82
");
}

#[test]
fn fn_add_37() {
    let (out, diags) = interpret("fn add(a, b) return a + b end fn main() print(add(37, 47)) end", 0);
    assert!(!diags.has_errors(), "fn_add_37: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"84
");
}

#[test]
fn fn_add_38() {
    let (out, diags) = interpret("fn add(a, b) return a + b end fn main() print(add(38, 48)) end", 0);
    assert!(!diags.has_errors(), "fn_add_38: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"86
");
}

#[test]
fn fn_add_39() {
    let (out, diags) = interpret("fn add(a, b) return a + b end fn main() print(add(39, 49)) end", 0);
    assert!(!diags.has_errors(), "fn_add_39: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"88
");
}

#[test]
fn fn_add_40() {
    let (out, diags) = interpret("fn add(a, b) return a + b end fn main() print(add(40, 50)) end", 0);
    assert!(!diags.has_errors(), "fn_add_40: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"90
");
}

#[test]
fn fn_add_41() {
    let (out, diags) = interpret("fn add(a, b) return a + b end fn main() print(add(41, 51)) end", 0);
    assert!(!diags.has_errors(), "fn_add_41: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"92
");
}

#[test]
fn fn_add_42() {
    let (out, diags) = interpret("fn add(a, b) return a + b end fn main() print(add(42, 52)) end", 0);
    assert!(!diags.has_errors(), "fn_add_42: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"94
");
}

#[test]
fn fn_add_43() {
    let (out, diags) = interpret("fn add(a, b) return a + b end fn main() print(add(43, 53)) end", 0);
    assert!(!diags.has_errors(), "fn_add_43: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"96
");
}

#[test]
fn fn_add_44() {
    let (out, diags) = interpret("fn add(a, b) return a + b end fn main() print(add(44, 54)) end", 0);
    assert!(!diags.has_errors(), "fn_add_44: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"98
");
}

#[test]
fn fn_add_45() {
    let (out, diags) = interpret("fn add(a, b) return a + b end fn main() print(add(45, 55)) end", 0);
    assert!(!diags.has_errors(), "fn_add_45: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"100
");
}

#[test]
fn fn_add_46() {
    let (out, diags) = interpret("fn add(a, b) return a + b end fn main() print(add(46, 56)) end", 0);
    assert!(!diags.has_errors(), "fn_add_46: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"102
");
}

#[test]
fn fn_add_47() {
    let (out, diags) = interpret("fn add(a, b) return a + b end fn main() print(add(47, 57)) end", 0);
    assert!(!diags.has_errors(), "fn_add_47: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"104
");
}

#[test]
fn fn_add_48() {
    let (out, diags) = interpret("fn add(a, b) return a + b end fn main() print(add(48, 58)) end", 0);
    assert!(!diags.has_errors(), "fn_add_48: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"106
");
}

#[test]
fn fn_add_49() {
    let (out, diags) = interpret("fn add(a, b) return a + b end fn main() print(add(49, 59)) end", 0);
    assert!(!diags.has_errors(), "fn_add_49: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"108
");
}

#[test]
fn fn_add_50() {
    let (out, diags) = interpret("fn add(a, b) return a + b end fn main() print(add(50, 60)) end", 0);
    assert!(!diags.has_errors(), "fn_add_50: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"110
");
}

#[test]
fn fn_add_51() {
    let (out, diags) = interpret("fn add(a, b) return a + b end fn main() print(add(51, 61)) end", 0);
    assert!(!diags.has_errors(), "fn_add_51: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"112
");
}

#[test]
fn fn_add_52() {
    let (out, diags) = interpret("fn add(a, b) return a + b end fn main() print(add(52, 62)) end", 0);
    assert!(!diags.has_errors(), "fn_add_52: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"114
");
}

#[test]
fn fn_add_53() {
    let (out, diags) = interpret("fn add(a, b) return a + b end fn main() print(add(53, 63)) end", 0);
    assert!(!diags.has_errors(), "fn_add_53: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"116
");
}

#[test]
fn fn_add_54() {
    let (out, diags) = interpret("fn add(a, b) return a + b end fn main() print(add(54, 64)) end", 0);
    assert!(!diags.has_errors(), "fn_add_54: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"118
");
}

#[test]
fn fn_add_55() {
    let (out, diags) = interpret("fn add(a, b) return a + b end fn main() print(add(55, 65)) end", 0);
    assert!(!diags.has_errors(), "fn_add_55: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"120
");
}

#[test]
fn fn_add_56() {
    let (out, diags) = interpret("fn add(a, b) return a + b end fn main() print(add(56, 66)) end", 0);
    assert!(!diags.has_errors(), "fn_add_56: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"122
");
}

#[test]
fn fn_add_57() {
    let (out, diags) = interpret("fn add(a, b) return a + b end fn main() print(add(57, 67)) end", 0);
    assert!(!diags.has_errors(), "fn_add_57: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"124
");
}

#[test]
fn fn_add_58() {
    let (out, diags) = interpret("fn add(a, b) return a + b end fn main() print(add(58, 68)) end", 0);
    assert!(!diags.has_errors(), "fn_add_58: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"126
");
}

#[test]
fn fn_add_59() {
    let (out, diags) = interpret("fn add(a, b) return a + b end fn main() print(add(59, 69)) end", 0);
    assert!(!diags.has_errors(), "fn_add_59: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"128
");
}

#[test]
fn fn_add_60() {
    let (out, diags) = interpret("fn add(a, b) return a + b end fn main() print(add(60, 70)) end", 0);
    assert!(!diags.has_errors(), "fn_add_60: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"130
");
}

#[test]
fn fn_add_61() {
    let (out, diags) = interpret("fn add(a, b) return a + b end fn main() print(add(61, 71)) end", 0);
    assert!(!diags.has_errors(), "fn_add_61: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"132
");
}

#[test]
fn fn_add_62() {
    let (out, diags) = interpret("fn add(a, b) return a + b end fn main() print(add(62, 72)) end", 0);
    assert!(!diags.has_errors(), "fn_add_62: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"134
");
}

#[test]
fn fn_add_63() {
    let (out, diags) = interpret("fn add(a, b) return a + b end fn main() print(add(63, 73)) end", 0);
    assert!(!diags.has_errors(), "fn_add_63: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"136
");
}

#[test]
fn fn_add_64() {
    let (out, diags) = interpret("fn add(a, b) return a + b end fn main() print(add(64, 74)) end", 0);
    assert!(!diags.has_errors(), "fn_add_64: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"138
");
}

#[test]
fn fn_add_65() {
    let (out, diags) = interpret("fn add(a, b) return a + b end fn main() print(add(65, 75)) end", 0);
    assert!(!diags.has_errors(), "fn_add_65: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"140
");
}

#[test]
fn fn_add_66() {
    let (out, diags) = interpret("fn add(a, b) return a + b end fn main() print(add(66, 76)) end", 0);
    assert!(!diags.has_errors(), "fn_add_66: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"142
");
}

#[test]
fn fn_add_67() {
    let (out, diags) = interpret("fn add(a, b) return a + b end fn main() print(add(67, 77)) end", 0);
    assert!(!diags.has_errors(), "fn_add_67: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"144
");
}

#[test]
fn fn_add_68() {
    let (out, diags) = interpret("fn add(a, b) return a + b end fn main() print(add(68, 78)) end", 0);
    assert!(!diags.has_errors(), "fn_add_68: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"146
");
}

#[test]
fn fn_add_69() {
    let (out, diags) = interpret("fn add(a, b) return a + b end fn main() print(add(69, 79)) end", 0);
    assert!(!diags.has_errors(), "fn_add_69: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"148
");
}

#[test]
fn fn_add_70() {
    let (out, diags) = interpret("fn add(a, b) return a + b end fn main() print(add(70, 80)) end", 0);
    assert!(!diags.has_errors(), "fn_add_70: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"150
");
}

#[test]
fn fn_add_71() {
    let (out, diags) = interpret("fn add(a, b) return a + b end fn main() print(add(71, 81)) end", 0);
    assert!(!diags.has_errors(), "fn_add_71: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"152
");
}

#[test]
fn fn_add_72() {
    let (out, diags) = interpret("fn add(a, b) return a + b end fn main() print(add(72, 82)) end", 0);
    assert!(!diags.has_errors(), "fn_add_72: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"154
");
}

#[test]
fn fn_add_73() {
    let (out, diags) = interpret("fn add(a, b) return a + b end fn main() print(add(73, 83)) end", 0);
    assert!(!diags.has_errors(), "fn_add_73: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"156
");
}

#[test]
fn fn_add_74() {
    let (out, diags) = interpret("fn add(a, b) return a + b end fn main() print(add(74, 84)) end", 0);
    assert!(!diags.has_errors(), "fn_add_74: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"158
");
}

#[test]
fn fn_add_75() {
    let (out, diags) = interpret("fn add(a, b) return a + b end fn main() print(add(75, 85)) end", 0);
    assert!(!diags.has_errors(), "fn_add_75: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"160
");
}

#[test]
fn fn_add_76() {
    let (out, diags) = interpret("fn add(a, b) return a + b end fn main() print(add(76, 86)) end", 0);
    assert!(!diags.has_errors(), "fn_add_76: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"162
");
}

#[test]
fn fn_add_77() {
    let (out, diags) = interpret("fn add(a, b) return a + b end fn main() print(add(77, 87)) end", 0);
    assert!(!diags.has_errors(), "fn_add_77: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"164
");
}

#[test]
fn fn_add_78() {
    let (out, diags) = interpret("fn add(a, b) return a + b end fn main() print(add(78, 88)) end", 0);
    assert!(!diags.has_errors(), "fn_add_78: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"166
");
}

#[test]
fn fn_add_79() {
    let (out, diags) = interpret("fn add(a, b) return a + b end fn main() print(add(79, 89)) end", 0);
    assert!(!diags.has_errors(), "fn_add_79: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"168
");
}

#[test]
fn fn_add_80() {
    let (out, diags) = interpret("fn add(a, b) return a + b end fn main() print(add(80, 90)) end", 0);
    assert!(!diags.has_errors(), "fn_add_80: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"170
");
}

#[test]
fn fn_add_81() {
    let (out, diags) = interpret("fn add(a, b) return a + b end fn main() print(add(81, 91)) end", 0);
    assert!(!diags.has_errors(), "fn_add_81: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"172
");
}

#[test]
fn fn_add_82() {
    let (out, diags) = interpret("fn add(a, b) return a + b end fn main() print(add(82, 92)) end", 0);
    assert!(!diags.has_errors(), "fn_add_82: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"174
");
}

#[test]
fn fn_add_83() {
    let (out, diags) = interpret("fn add(a, b) return a + b end fn main() print(add(83, 93)) end", 0);
    assert!(!diags.has_errors(), "fn_add_83: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"176
");
}

#[test]
fn fn_add_84() {
    let (out, diags) = interpret("fn add(a, b) return a + b end fn main() print(add(84, 94)) end", 0);
    assert!(!diags.has_errors(), "fn_add_84: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"178
");
}

#[test]
fn fn_add_85() {
    let (out, diags) = interpret("fn add(a, b) return a + b end fn main() print(add(85, 95)) end", 0);
    assert!(!diags.has_errors(), "fn_add_85: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"180
");
}

#[test]
fn fn_add_86() {
    let (out, diags) = interpret("fn add(a, b) return a + b end fn main() print(add(86, 96)) end", 0);
    assert!(!diags.has_errors(), "fn_add_86: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"182
");
}

#[test]
fn fn_add_87() {
    let (out, diags) = interpret("fn add(a, b) return a + b end fn main() print(add(87, 97)) end", 0);
    assert!(!diags.has_errors(), "fn_add_87: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"184
");
}

#[test]
fn fn_add_88() {
    let (out, diags) = interpret("fn add(a, b) return a + b end fn main() print(add(88, 98)) end", 0);
    assert!(!diags.has_errors(), "fn_add_88: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"186
");
}

#[test]
fn fn_add_89() {
    let (out, diags) = interpret("fn add(a, b) return a + b end fn main() print(add(89, 99)) end", 0);
    assert!(!diags.has_errors(), "fn_add_89: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"188
");
}

#[test]
fn fn_add_90() {
    let (out, diags) = interpret("fn add(a, b) return a + b end fn main() print(add(90, 100)) end", 0);
    assert!(!diags.has_errors(), "fn_add_90: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"190
");
}

#[test]
fn fn_add_91() {
    let (out, diags) = interpret("fn add(a, b) return a + b end fn main() print(add(91, 101)) end", 0);
    assert!(!diags.has_errors(), "fn_add_91: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"192
");
}

#[test]
fn fn_add_92() {
    let (out, diags) = interpret("fn add(a, b) return a + b end fn main() print(add(92, 102)) end", 0);
    assert!(!diags.has_errors(), "fn_add_92: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"194
");
}

#[test]
fn fn_add_93() {
    let (out, diags) = interpret("fn add(a, b) return a + b end fn main() print(add(93, 103)) end", 0);
    assert!(!diags.has_errors(), "fn_add_93: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"196
");
}

#[test]
fn fn_add_94() {
    let (out, diags) = interpret("fn add(a, b) return a + b end fn main() print(add(94, 104)) end", 0);
    assert!(!diags.has_errors(), "fn_add_94: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"198
");
}

#[test]
fn fn_add_95() {
    let (out, diags) = interpret("fn add(a, b) return a + b end fn main() print(add(95, 105)) end", 0);
    assert!(!diags.has_errors(), "fn_add_95: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"200
");
}

#[test]
fn fn_add_96() {
    let (out, diags) = interpret("fn add(a, b) return a + b end fn main() print(add(96, 106)) end", 0);
    assert!(!diags.has_errors(), "fn_add_96: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"202
");
}

#[test]
fn fn_add_97() {
    let (out, diags) = interpret("fn add(a, b) return a + b end fn main() print(add(97, 107)) end", 0);
    assert!(!diags.has_errors(), "fn_add_97: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"204
");
}

#[test]
fn fn_add_98() {
    let (out, diags) = interpret("fn add(a, b) return a + b end fn main() print(add(98, 108)) end", 0);
    assert!(!diags.has_errors(), "fn_add_98: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"206
");
}

#[test]
fn fn_add_99() {
    let (out, diags) = interpret("fn add(a, b) return a + b end fn main() print(add(99, 109)) end", 0);
    assert!(!diags.has_errors(), "fn_add_99: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"208
");
}

#[test]
fn fn_mul_0() {
    let (out, diags) = interpret("fn mul(a, b) return a * b end fn main() print(mul(0, 2)) end", 0);
    assert!(!diags.has_errors(), "fn_mul_0: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"0
");
}

#[test]
fn fn_mul_1() {
    let (out, diags) = interpret("fn mul(a, b) return a * b end fn main() print(mul(1, 3)) end", 0);
    assert!(!diags.has_errors(), "fn_mul_1: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"3
");
}

#[test]
fn fn_mul_2() {
    let (out, diags) = interpret("fn mul(a, b) return a * b end fn main() print(mul(2, 4)) end", 0);
    assert!(!diags.has_errors(), "fn_mul_2: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"8
");
}

#[test]
fn fn_mul_3() {
    let (out, diags) = interpret("fn mul(a, b) return a * b end fn main() print(mul(3, 5)) end", 0);
    assert!(!diags.has_errors(), "fn_mul_3: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"15
");
}

#[test]
fn fn_mul_4() {
    let (out, diags) = interpret("fn mul(a, b) return a * b end fn main() print(mul(4, 6)) end", 0);
    assert!(!diags.has_errors(), "fn_mul_4: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"24
");
}

#[test]
fn fn_mul_5() {
    let (out, diags) = interpret("fn mul(a, b) return a * b end fn main() print(mul(5, 7)) end", 0);
    assert!(!diags.has_errors(), "fn_mul_5: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"35
");
}

#[test]
fn fn_mul_6() {
    let (out, diags) = interpret("fn mul(a, b) return a * b end fn main() print(mul(6, 8)) end", 0);
    assert!(!diags.has_errors(), "fn_mul_6: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"48
");
}

#[test]
fn fn_mul_7() {
    let (out, diags) = interpret("fn mul(a, b) return a * b end fn main() print(mul(7, 9)) end", 0);
    assert!(!diags.has_errors(), "fn_mul_7: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"63
");
}

#[test]
fn fn_mul_8() {
    let (out, diags) = interpret("fn mul(a, b) return a * b end fn main() print(mul(8, 10)) end", 0);
    assert!(!diags.has_errors(), "fn_mul_8: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"80
");
}

#[test]
fn fn_mul_9() {
    let (out, diags) = interpret("fn mul(a, b) return a * b end fn main() print(mul(9, 11)) end", 0);
    assert!(!diags.has_errors(), "fn_mul_9: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"99
");
}

#[test]
fn fn_mul_10() {
    let (out, diags) = interpret("fn mul(a, b) return a * b end fn main() print(mul(10, 12)) end", 0);
    assert!(!diags.has_errors(), "fn_mul_10: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"120
");
}

#[test]
fn fn_mul_11() {
    let (out, diags) = interpret("fn mul(a, b) return a * b end fn main() print(mul(11, 13)) end", 0);
    assert!(!diags.has_errors(), "fn_mul_11: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"143
");
}

#[test]
fn fn_mul_12() {
    let (out, diags) = interpret("fn mul(a, b) return a * b end fn main() print(mul(12, 14)) end", 0);
    assert!(!diags.has_errors(), "fn_mul_12: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"168
");
}

#[test]
fn fn_mul_13() {
    let (out, diags) = interpret("fn mul(a, b) return a * b end fn main() print(mul(13, 15)) end", 0);
    assert!(!diags.has_errors(), "fn_mul_13: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"195
");
}

#[test]
fn fn_mul_14() {
    let (out, diags) = interpret("fn mul(a, b) return a * b end fn main() print(mul(14, 16)) end", 0);
    assert!(!diags.has_errors(), "fn_mul_14: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"224
");
}

#[test]
fn fn_mul_15() {
    let (out, diags) = interpret("fn mul(a, b) return a * b end fn main() print(mul(15, 17)) end", 0);
    assert!(!diags.has_errors(), "fn_mul_15: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"255
");
}

#[test]
fn fn_mul_16() {
    let (out, diags) = interpret("fn mul(a, b) return a * b end fn main() print(mul(16, 18)) end", 0);
    assert!(!diags.has_errors(), "fn_mul_16: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"288
");
}

#[test]
fn fn_mul_17() {
    let (out, diags) = interpret("fn mul(a, b) return a * b end fn main() print(mul(17, 19)) end", 0);
    assert!(!diags.has_errors(), "fn_mul_17: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"323
");
}

#[test]
fn fn_mul_18() {
    let (out, diags) = interpret("fn mul(a, b) return a * b end fn main() print(mul(18, 20)) end", 0);
    assert!(!diags.has_errors(), "fn_mul_18: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"360
");
}

#[test]
fn fn_mul_19() {
    let (out, diags) = interpret("fn mul(a, b) return a * b end fn main() print(mul(19, 21)) end", 0);
    assert!(!diags.has_errors(), "fn_mul_19: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"399
");
}

#[test]
fn fn_mul_20() {
    let (out, diags) = interpret("fn mul(a, b) return a * b end fn main() print(mul(20, 22)) end", 0);
    assert!(!diags.has_errors(), "fn_mul_20: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"440
");
}

#[test]
fn fn_mul_21() {
    let (out, diags) = interpret("fn mul(a, b) return a * b end fn main() print(mul(21, 23)) end", 0);
    assert!(!diags.has_errors(), "fn_mul_21: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"483
");
}

#[test]
fn fn_mul_22() {
    let (out, diags) = interpret("fn mul(a, b) return a * b end fn main() print(mul(22, 24)) end", 0);
    assert!(!diags.has_errors(), "fn_mul_22: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"528
");
}

#[test]
fn fn_mul_23() {
    let (out, diags) = interpret("fn mul(a, b) return a * b end fn main() print(mul(23, 25)) end", 0);
    assert!(!diags.has_errors(), "fn_mul_23: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"575
");
}

#[test]
fn fn_mul_24() {
    let (out, diags) = interpret("fn mul(a, b) return a * b end fn main() print(mul(24, 26)) end", 0);
    assert!(!diags.has_errors(), "fn_mul_24: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"624
");
}

#[test]
fn fn_mul_25() {
    let (out, diags) = interpret("fn mul(a, b) return a * b end fn main() print(mul(25, 27)) end", 0);
    assert!(!diags.has_errors(), "fn_mul_25: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"675
");
}

#[test]
fn fn_mul_26() {
    let (out, diags) = interpret("fn mul(a, b) return a * b end fn main() print(mul(26, 28)) end", 0);
    assert!(!diags.has_errors(), "fn_mul_26: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"728
");
}

#[test]
fn fn_mul_27() {
    let (out, diags) = interpret("fn mul(a, b) return a * b end fn main() print(mul(27, 29)) end", 0);
    assert!(!diags.has_errors(), "fn_mul_27: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"783
");
}

#[test]
fn fn_mul_28() {
    let (out, diags) = interpret("fn mul(a, b) return a * b end fn main() print(mul(28, 30)) end", 0);
    assert!(!diags.has_errors(), "fn_mul_28: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"840
");
}

#[test]
fn fn_mul_29() {
    let (out, diags) = interpret("fn mul(a, b) return a * b end fn main() print(mul(29, 31)) end", 0);
    assert!(!diags.has_errors(), "fn_mul_29: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"899
");
}

#[test]
fn fn_mul_30() {
    let (out, diags) = interpret("fn mul(a, b) return a * b end fn main() print(mul(30, 32)) end", 0);
    assert!(!diags.has_errors(), "fn_mul_30: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"960
");
}

#[test]
fn fn_mul_31() {
    let (out, diags) = interpret("fn mul(a, b) return a * b end fn main() print(mul(31, 33)) end", 0);
    assert!(!diags.has_errors(), "fn_mul_31: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"1023
");
}

#[test]
fn fn_mul_32() {
    let (out, diags) = interpret("fn mul(a, b) return a * b end fn main() print(mul(32, 34)) end", 0);
    assert!(!diags.has_errors(), "fn_mul_32: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"1088
");
}

#[test]
fn fn_mul_33() {
    let (out, diags) = interpret("fn mul(a, b) return a * b end fn main() print(mul(33, 35)) end", 0);
    assert!(!diags.has_errors(), "fn_mul_33: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"1155
");
}

#[test]
fn fn_mul_34() {
    let (out, diags) = interpret("fn mul(a, b) return a * b end fn main() print(mul(34, 36)) end", 0);
    assert!(!diags.has_errors(), "fn_mul_34: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"1224
");
}

#[test]
fn fn_mul_35() {
    let (out, diags) = interpret("fn mul(a, b) return a * b end fn main() print(mul(35, 37)) end", 0);
    assert!(!diags.has_errors(), "fn_mul_35: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"1295
");
}

#[test]
fn fn_mul_36() {
    let (out, diags) = interpret("fn mul(a, b) return a * b end fn main() print(mul(36, 38)) end", 0);
    assert!(!diags.has_errors(), "fn_mul_36: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"1368
");
}

#[test]
fn fn_mul_37() {
    let (out, diags) = interpret("fn mul(a, b) return a * b end fn main() print(mul(37, 39)) end", 0);
    assert!(!diags.has_errors(), "fn_mul_37: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"1443
");
}

#[test]
fn fn_mul_38() {
    let (out, diags) = interpret("fn mul(a, b) return a * b end fn main() print(mul(38, 40)) end", 0);
    assert!(!diags.has_errors(), "fn_mul_38: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"1520
");
}

#[test]
fn fn_mul_39() {
    let (out, diags) = interpret("fn mul(a, b) return a * b end fn main() print(mul(39, 41)) end", 0);
    assert!(!diags.has_errors(), "fn_mul_39: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"1599
");
}

#[test]
fn fn_mul_40() {
    let (out, diags) = interpret("fn mul(a, b) return a * b end fn main() print(mul(40, 42)) end", 0);
    assert!(!diags.has_errors(), "fn_mul_40: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"1680
");
}

#[test]
fn fn_mul_41() {
    let (out, diags) = interpret("fn mul(a, b) return a * b end fn main() print(mul(41, 43)) end", 0);
    assert!(!diags.has_errors(), "fn_mul_41: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"1763
");
}

#[test]
fn fn_mul_42() {
    let (out, diags) = interpret("fn mul(a, b) return a * b end fn main() print(mul(42, 44)) end", 0);
    assert!(!diags.has_errors(), "fn_mul_42: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"1848
");
}

#[test]
fn fn_mul_43() {
    let (out, diags) = interpret("fn mul(a, b) return a * b end fn main() print(mul(43, 45)) end", 0);
    assert!(!diags.has_errors(), "fn_mul_43: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"1935
");
}

#[test]
fn fn_mul_44() {
    let (out, diags) = interpret("fn mul(a, b) return a * b end fn main() print(mul(44, 46)) end", 0);
    assert!(!diags.has_errors(), "fn_mul_44: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"2024
");
}

#[test]
fn fn_mul_45() {
    let (out, diags) = interpret("fn mul(a, b) return a * b end fn main() print(mul(45, 47)) end", 0);
    assert!(!diags.has_errors(), "fn_mul_45: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"2115
");
}

#[test]
fn fn_mul_46() {
    let (out, diags) = interpret("fn mul(a, b) return a * b end fn main() print(mul(46, 48)) end", 0);
    assert!(!diags.has_errors(), "fn_mul_46: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"2208
");
}

#[test]
fn fn_mul_47() {
    let (out, diags) = interpret("fn mul(a, b) return a * b end fn main() print(mul(47, 49)) end", 0);
    assert!(!diags.has_errors(), "fn_mul_47: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"2303
");
}

#[test]
fn fn_mul_48() {
    let (out, diags) = interpret("fn mul(a, b) return a * b end fn main() print(mul(48, 50)) end", 0);
    assert!(!diags.has_errors(), "fn_mul_48: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"2400
");
}

#[test]
fn fn_mul_49() {
    let (out, diags) = interpret("fn mul(a, b) return a * b end fn main() print(mul(49, 51)) end", 0);
    assert!(!diags.has_errors(), "fn_mul_49: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"2499
");
}

#[test]
fn fn_fact_0() {
    let (out, diags) = interpret("fn fact(n) if n <= 1 then return 1 end return n * fact(n - 1) end fn main() print(fact(3)) end", 0);
    assert!(!diags.has_errors(), "fn_fact_0: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"6
");
}

#[test]
fn fn_fact_1() {
    let (out, diags) = interpret("fn fact(n) if n <= 1 then return 1 end return n * fact(n - 1) end fn main() print(fact(4)) end", 0);
    assert!(!diags.has_errors(), "fn_fact_1: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"24
");
}

#[test]
fn fn_fact_2() {
    let (out, diags) = interpret("fn fact(n) if n <= 1 then return 1 end return n * fact(n - 1) end fn main() print(fact(5)) end", 0);
    assert!(!diags.has_errors(), "fn_fact_2: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"120
");
}

#[test]
fn fn_fact_3() {
    let (out, diags) = interpret("fn fact(n) if n <= 1 then return 1 end return n * fact(n - 1) end fn main() print(fact(6)) end", 0);
    assert!(!diags.has_errors(), "fn_fact_3: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"720
");
}

#[test]
fn fn_fact_4() {
    let (out, diags) = interpret("fn fact(n) if n <= 1 then return 1 end return n * fact(n - 1) end fn main() print(fact(7)) end", 0);
    assert!(!diags.has_errors(), "fn_fact_4: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"5040
");
}

#[test]
fn fn_fact_5() {
    let (out, diags) = interpret("fn fact(n) if n <= 1 then return 1 end return n * fact(n - 1) end fn main() print(fact(8)) end", 0);
    assert!(!diags.has_errors(), "fn_fact_5: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"40320
");
}

#[test]
fn fn_fact_6() {
    let (out, diags) = interpret("fn fact(n) if n <= 1 then return 1 end return n * fact(n - 1) end fn main() print(fact(9)) end", 0);
    assert!(!diags.has_errors(), "fn_fact_6: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"362880
");
}

#[test]
fn fn_fact_7() {
    let (out, diags) = interpret("fn fact(n) if n <= 1 then return 1 end return n * fact(n - 1) end fn main() print(fact(10)) end", 0);
    assert!(!diags.has_errors(), "fn_fact_7: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"3628800
");
}

#[test]
fn fn_fact_8() {
    let (out, diags) = interpret("fn fact(n) if n <= 1 then return 1 end return n * fact(n - 1) end fn main() print(fact(11)) end", 0);
    assert!(!diags.has_errors(), "fn_fact_8: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"39916800
");
}

#[test]
fn fn_fact_9() {
    let (out, diags) = interpret("fn fact(n) if n <= 1 then return 1 end return n * fact(n - 1) end fn main() print(fact(12)) end", 0);
    assert!(!diags.has_errors(), "fn_fact_9: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"479001600
");
}

#[test]
fn fn_fact_10() {
    let (out, diags) = interpret("fn fact(n) if n <= 1 then return 1 end return n * fact(n - 1) end fn main() print(fact(13)) end", 0);
    assert!(!diags.has_errors(), "fn_fact_10: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"6227020800
");
}

#[test]
fn fn_fact_11() {
    let (out, diags) = interpret("fn fact(n) if n <= 1 then return 1 end return n * fact(n - 1) end fn main() print(fact(14)) end", 0);
    assert!(!diags.has_errors(), "fn_fact_11: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"87178291200
");
}

#[test]
fn fn_fact_12() {
    let (out, diags) = interpret("fn fact(n) if n <= 1 then return 1 end return n * fact(n - 1) end fn main() print(fact(15)) end", 0);
    assert!(!diags.has_errors(), "fn_fact_12: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"1307674368000
");
}

#[test]
fn fn_fact_13() {
    let (out, diags) = interpret("fn fact(n) if n <= 1 then return 1 end return n * fact(n - 1) end fn main() print(fact(16)) end", 0);
    assert!(!diags.has_errors(), "fn_fact_13: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"20922789888000
");
}

#[test]
fn fn_fact_14() {
    let (out, diags) = interpret("fn fact(n) if n <= 1 then return 1 end return n * fact(n - 1) end fn main() print(fact(17)) end", 0);
    assert!(!diags.has_errors(), "fn_fact_14: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"355687428096000
");
}

#[test]
fn fn_fact_15() {
    let (out, diags) = interpret("fn fact(n) if n <= 1 then return 1 end return n * fact(n - 1) end fn main() print(fact(18)) end", 0);
    assert!(!diags.has_errors(), "fn_fact_15: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"6402373705728000
");
}

#[test]
fn fn_fact_16() {
    let (out, diags) = interpret("fn fact(n) if n <= 1 then return 1 end return n * fact(n - 1) end fn main() print(fact(19)) end", 0);
    assert!(!diags.has_errors(), "fn_fact_16: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"121645100408832000
");
}

#[test]
fn fn_fact_17() {
    let (out, diags) = interpret("fn fact(n) if n <= 1 then return 1 end return n * fact(n - 1) end fn main() print(fact(20)) end", 0);
    assert!(!diags.has_errors(), "fn_fact_17: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"2432902008176640000
");
}

#[test]
fn fn_fact_18() {
    let (out, diags) = interpret("fn fact(n) if n <= 1 then return 1 end return n * fact(n - 1) end fn main() print(fact(20)) end", 0);
    assert!(!diags.has_errors(), "fn_fact_18: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"2432902008176640000
");
}

#[test]
fn fn_fact_19() {
    let (out, diags) = interpret("fn fact(n) if n <= 1 then return 1 end return n * fact(n - 1) end fn main() print(fact(20)) end", 0);
    assert!(!diags.has_errors(), "fn_fact_19: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"2432902008176640000
");
}

#[test]
fn fn_sum_to_0() {
    let (out, diags) = interpret("fn sum(n) if n == 0 then return 0 end return n + sum(n - 1) end fn main() print(sum(1)) end", 0);
    assert!(!diags.has_errors(), "fn_sum_to_0: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"1
");
}

#[test]
fn fn_sum_to_1() {
    let (out, diags) = interpret("fn sum(n) if n == 0 then return 0 end return n + sum(n - 1) end fn main() print(sum(2)) end", 0);
    assert!(!diags.has_errors(), "fn_sum_to_1: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"3
");
}

#[test]
fn fn_sum_to_2() {
    let (out, diags) = interpret("fn sum(n) if n == 0 then return 0 end return n + sum(n - 1) end fn main() print(sum(3)) end", 0);
    assert!(!diags.has_errors(), "fn_sum_to_2: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"6
");
}

#[test]
fn fn_sum_to_3() {
    let (out, diags) = interpret("fn sum(n) if n == 0 then return 0 end return n + sum(n - 1) end fn main() print(sum(4)) end", 0);
    assert!(!diags.has_errors(), "fn_sum_to_3: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"10
");
}

#[test]
fn fn_sum_to_4() {
    let (out, diags) = interpret("fn sum(n) if n == 0 then return 0 end return n + sum(n - 1) end fn main() print(sum(5)) end", 0);
    assert!(!diags.has_errors(), "fn_sum_to_4: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"15
");
}

#[test]
fn fn_sum_to_5() {
    let (out, diags) = interpret("fn sum(n) if n == 0 then return 0 end return n + sum(n - 1) end fn main() print(sum(6)) end", 0);
    assert!(!diags.has_errors(), "fn_sum_to_5: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"21
");
}

#[test]
fn fn_sum_to_6() {
    let (out, diags) = interpret("fn sum(n) if n == 0 then return 0 end return n + sum(n - 1) end fn main() print(sum(7)) end", 0);
    assert!(!diags.has_errors(), "fn_sum_to_6: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"28
");
}

#[test]
fn fn_sum_to_7() {
    let (out, diags) = interpret("fn sum(n) if n == 0 then return 0 end return n + sum(n - 1) end fn main() print(sum(8)) end", 0);
    assert!(!diags.has_errors(), "fn_sum_to_7: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"36
");
}

#[test]
fn fn_sum_to_8() {
    let (out, diags) = interpret("fn sum(n) if n == 0 then return 0 end return n + sum(n - 1) end fn main() print(sum(9)) end", 0);
    assert!(!diags.has_errors(), "fn_sum_to_8: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"45
");
}

#[test]
fn fn_sum_to_9() {
    let (out, diags) = interpret("fn sum(n) if n == 0 then return 0 end return n + sum(n - 1) end fn main() print(sum(10)) end", 0);
    assert!(!diags.has_errors(), "fn_sum_to_9: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"55
");
}

#[test]
fn fn_sum_to_10() {
    let (out, diags) = interpret("fn sum(n) if n == 0 then return 0 end return n + sum(n - 1) end fn main() print(sum(11)) end", 0);
    assert!(!diags.has_errors(), "fn_sum_to_10: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"66
");
}

#[test]
fn fn_sum_to_11() {
    let (out, diags) = interpret("fn sum(n) if n == 0 then return 0 end return n + sum(n - 1) end fn main() print(sum(12)) end", 0);
    assert!(!diags.has_errors(), "fn_sum_to_11: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"78
");
}

#[test]
fn fn_sum_to_12() {
    let (out, diags) = interpret("fn sum(n) if n == 0 then return 0 end return n + sum(n - 1) end fn main() print(sum(13)) end", 0);
    assert!(!diags.has_errors(), "fn_sum_to_12: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"91
");
}

#[test]
fn fn_sum_to_13() {
    let (out, diags) = interpret("fn sum(n) if n == 0 then return 0 end return n + sum(n - 1) end fn main() print(sum(14)) end", 0);
    assert!(!diags.has_errors(), "fn_sum_to_13: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"105
");
}

#[test]
fn fn_sum_to_14() {
    let (out, diags) = interpret("fn sum(n) if n == 0 then return 0 end return n + sum(n - 1) end fn main() print(sum(15)) end", 0);
    assert!(!diags.has_errors(), "fn_sum_to_14: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"120
");
}

#[test]
fn fn_sum_to_15() {
    let (out, diags) = interpret("fn sum(n) if n == 0 then return 0 end return n + sum(n - 1) end fn main() print(sum(16)) end", 0);
    assert!(!diags.has_errors(), "fn_sum_to_15: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"136
");
}

#[test]
fn fn_sum_to_16() {
    let (out, diags) = interpret("fn sum(n) if n == 0 then return 0 end return n + sum(n - 1) end fn main() print(sum(17)) end", 0);
    assert!(!diags.has_errors(), "fn_sum_to_16: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"153
");
}

#[test]
fn fn_sum_to_17() {
    let (out, diags) = interpret("fn sum(n) if n == 0 then return 0 end return n + sum(n - 1) end fn main() print(sum(18)) end", 0);
    assert!(!diags.has_errors(), "fn_sum_to_17: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"171
");
}

#[test]
fn fn_sum_to_18() {
    let (out, diags) = interpret("fn sum(n) if n == 0 then return 0 end return n + sum(n - 1) end fn main() print(sum(19)) end", 0);
    assert!(!diags.has_errors(), "fn_sum_to_18: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"190
");
}

#[test]
fn fn_sum_to_19() {
    let (out, diags) = interpret("fn sum(n) if n == 0 then return 0 end return n + sum(n - 1) end fn main() print(sum(20)) end", 0);
    assert!(!diags.has_errors(), "fn_sum_to_19: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"210
");
}

#[test]
fn fn_sum_to_20() {
    let (out, diags) = interpret("fn sum(n) if n == 0 then return 0 end return n + sum(n - 1) end fn main() print(sum(21)) end", 0);
    assert!(!diags.has_errors(), "fn_sum_to_20: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"231
");
}

#[test]
fn fn_sum_to_21() {
    let (out, diags) = interpret("fn sum(n) if n == 0 then return 0 end return n + sum(n - 1) end fn main() print(sum(22)) end", 0);
    assert!(!diags.has_errors(), "fn_sum_to_21: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"253
");
}

#[test]
fn fn_sum_to_22() {
    let (out, diags) = interpret("fn sum(n) if n == 0 then return 0 end return n + sum(n - 1) end fn main() print(sum(23)) end", 0);
    assert!(!diags.has_errors(), "fn_sum_to_22: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"276
");
}

#[test]
fn fn_sum_to_23() {
    let (out, diags) = interpret("fn sum(n) if n == 0 then return 0 end return n + sum(n - 1) end fn main() print(sum(24)) end", 0);
    assert!(!diags.has_errors(), "fn_sum_to_23: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"300
");
}

#[test]
fn fn_sum_to_24() {
    let (out, diags) = interpret("fn sum(n) if n == 0 then return 0 end return n + sum(n - 1) end fn main() print(sum(25)) end", 0);
    assert!(!diags.has_errors(), "fn_sum_to_24: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"325
");
}

#[test]
fn fn_sum_to_25() {
    let (out, diags) = interpret("fn sum(n) if n == 0 then return 0 end return n + sum(n - 1) end fn main() print(sum(26)) end", 0);
    assert!(!diags.has_errors(), "fn_sum_to_25: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"351
");
}

#[test]
fn fn_sum_to_26() {
    let (out, diags) = interpret("fn sum(n) if n == 0 then return 0 end return n + sum(n - 1) end fn main() print(sum(27)) end", 0);
    assert!(!diags.has_errors(), "fn_sum_to_26: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"378
");
}

#[test]
fn fn_sum_to_27() {
    let (out, diags) = interpret("fn sum(n) if n == 0 then return 0 end return n + sum(n - 1) end fn main() print(sum(28)) end", 0);
    assert!(!diags.has_errors(), "fn_sum_to_27: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"406
");
}

#[test]
fn fn_sum_to_28() {
    let (out, diags) = interpret("fn sum(n) if n == 0 then return 0 end return n + sum(n - 1) end fn main() print(sum(29)) end", 0);
    assert!(!diags.has_errors(), "fn_sum_to_28: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"435
");
}

#[test]
fn fn_sum_to_29() {
    let (out, diags) = interpret("fn sum(n) if n == 0 then return 0 end return n + sum(n - 1) end fn main() print(sum(30)) end", 0);
    assert!(!diags.has_errors(), "fn_sum_to_29: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"465
");
}

#[test]
fn fn_id_0() {
    let (out, diags) = interpret("fn id(x) return x end fn main() print(id(0)) end", 0);
    assert!(!diags.has_errors(), "fn_id_0: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"0
");
}

#[test]
fn fn_id_1() {
    let (out, diags) = interpret("fn id(x) return x end fn main() print(id(1)) end", 0);
    assert!(!diags.has_errors(), "fn_id_1: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"1
");
}

#[test]
fn fn_id_2() {
    let (out, diags) = interpret("fn id(x) return x end fn main() print(id(2)) end", 0);
    assert!(!diags.has_errors(), "fn_id_2: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"2
");
}

#[test]
fn fn_id_3() {
    let (out, diags) = interpret("fn id(x) return x end fn main() print(id(3)) end", 0);
    assert!(!diags.has_errors(), "fn_id_3: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"3
");
}

#[test]
fn fn_id_4() {
    let (out, diags) = interpret("fn id(x) return x end fn main() print(id(4)) end", 0);
    assert!(!diags.has_errors(), "fn_id_4: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"4
");
}

#[test]
fn fn_id_5() {
    let (out, diags) = interpret("fn id(x) return x end fn main() print(id(5)) end", 0);
    assert!(!diags.has_errors(), "fn_id_5: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"5
");
}

#[test]
fn fn_id_6() {
    let (out, diags) = interpret("fn id(x) return x end fn main() print(id(6)) end", 0);
    assert!(!diags.has_errors(), "fn_id_6: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"6
");
}

#[test]
fn fn_id_7() {
    let (out, diags) = interpret("fn id(x) return x end fn main() print(id(7)) end", 0);
    assert!(!diags.has_errors(), "fn_id_7: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"7
");
}

#[test]
fn fn_id_8() {
    let (out, diags) = interpret("fn id(x) return x end fn main() print(id(8)) end", 0);
    assert!(!diags.has_errors(), "fn_id_8: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"8
");
}

#[test]
fn fn_id_9() {
    let (out, diags) = interpret("fn id(x) return x end fn main() print(id(9)) end", 0);
    assert!(!diags.has_errors(), "fn_id_9: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"9
");
}

#[test]
fn fn_id_10() {
    let (out, diags) = interpret("fn id(x) return x end fn main() print(id(10)) end", 0);
    assert!(!diags.has_errors(), "fn_id_10: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"10
");
}

#[test]
fn fn_id_11() {
    let (out, diags) = interpret("fn id(x) return x end fn main() print(id(11)) end", 0);
    assert!(!diags.has_errors(), "fn_id_11: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"11
");
}

#[test]
fn fn_id_12() {
    let (out, diags) = interpret("fn id(x) return x end fn main() print(id(12)) end", 0);
    assert!(!diags.has_errors(), "fn_id_12: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"12
");
}

#[test]
fn fn_id_13() {
    let (out, diags) = interpret("fn id(x) return x end fn main() print(id(13)) end", 0);
    assert!(!diags.has_errors(), "fn_id_13: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"13
");
}

#[test]
fn fn_id_14() {
    let (out, diags) = interpret("fn id(x) return x end fn main() print(id(14)) end", 0);
    assert!(!diags.has_errors(), "fn_id_14: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"14
");
}

#[test]
fn fn_id_15() {
    let (out, diags) = interpret("fn id(x) return x end fn main() print(id(15)) end", 0);
    assert!(!diags.has_errors(), "fn_id_15: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"15
");
}

#[test]
fn fn_id_16() {
    let (out, diags) = interpret("fn id(x) return x end fn main() print(id(16)) end", 0);
    assert!(!diags.has_errors(), "fn_id_16: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"16
");
}

#[test]
fn fn_id_17() {
    let (out, diags) = interpret("fn id(x) return x end fn main() print(id(17)) end", 0);
    assert!(!diags.has_errors(), "fn_id_17: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"17
");
}

#[test]
fn fn_id_18() {
    let (out, diags) = interpret("fn id(x) return x end fn main() print(id(18)) end", 0);
    assert!(!diags.has_errors(), "fn_id_18: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"18
");
}

#[test]
fn fn_id_19() {
    let (out, diags) = interpret("fn id(x) return x end fn main() print(id(19)) end", 0);
    assert!(!diags.has_errors(), "fn_id_19: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"19
");
}

#[test]
fn fn_id_20() {
    let (out, diags) = interpret("fn id(x) return x end fn main() print(id(20)) end", 0);
    assert!(!diags.has_errors(), "fn_id_20: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"20
");
}

#[test]
fn fn_id_21() {
    let (out, diags) = interpret("fn id(x) return x end fn main() print(id(21)) end", 0);
    assert!(!diags.has_errors(), "fn_id_21: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"21
");
}

#[test]
fn fn_id_22() {
    let (out, diags) = interpret("fn id(x) return x end fn main() print(id(22)) end", 0);
    assert!(!diags.has_errors(), "fn_id_22: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"22
");
}

#[test]
fn fn_id_23() {
    let (out, diags) = interpret("fn id(x) return x end fn main() print(id(23)) end", 0);
    assert!(!diags.has_errors(), "fn_id_23: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"23
");
}

#[test]
fn fn_id_24() {
    let (out, diags) = interpret("fn id(x) return x end fn main() print(id(24)) end", 0);
    assert!(!diags.has_errors(), "fn_id_24: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"24
");
}

#[test]
fn fn_id_25() {
    let (out, diags) = interpret("fn id(x) return x end fn main() print(id(25)) end", 0);
    assert!(!diags.has_errors(), "fn_id_25: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"25
");
}

#[test]
fn fn_id_26() {
    let (out, diags) = interpret("fn id(x) return x end fn main() print(id(26)) end", 0);
    assert!(!diags.has_errors(), "fn_id_26: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"26
");
}

#[test]
fn fn_id_27() {
    let (out, diags) = interpret("fn id(x) return x end fn main() print(id(27)) end", 0);
    assert!(!diags.has_errors(), "fn_id_27: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"27
");
}

#[test]
fn fn_id_28() {
    let (out, diags) = interpret("fn id(x) return x end fn main() print(id(28)) end", 0);
    assert!(!diags.has_errors(), "fn_id_28: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"28
");
}

#[test]
fn fn_id_29() {
    let (out, diags) = interpret("fn id(x) return x end fn main() print(id(29)) end", 0);
    assert!(!diags.has_errors(), "fn_id_29: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"29
");
}

#[test]
fn fn_id_30() {
    let (out, diags) = interpret("fn id(x) return x end fn main() print(id(30)) end", 0);
    assert!(!diags.has_errors(), "fn_id_30: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"30
");
}

#[test]
fn fn_id_31() {
    let (out, diags) = interpret("fn id(x) return x end fn main() print(id(31)) end", 0);
    assert!(!diags.has_errors(), "fn_id_31: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"31
");
}

#[test]
fn fn_id_32() {
    let (out, diags) = interpret("fn id(x) return x end fn main() print(id(32)) end", 0);
    assert!(!diags.has_errors(), "fn_id_32: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"32
");
}

#[test]
fn fn_id_33() {
    let (out, diags) = interpret("fn id(x) return x end fn main() print(id(33)) end", 0);
    assert!(!diags.has_errors(), "fn_id_33: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"33
");
}

#[test]
fn fn_id_34() {
    let (out, diags) = interpret("fn id(x) return x end fn main() print(id(34)) end", 0);
    assert!(!diags.has_errors(), "fn_id_34: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"34
");
}

#[test]
fn fn_id_35() {
    let (out, diags) = interpret("fn id(x) return x end fn main() print(id(35)) end", 0);
    assert!(!diags.has_errors(), "fn_id_35: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"35
");
}

#[test]
fn fn_id_36() {
    let (out, diags) = interpret("fn id(x) return x end fn main() print(id(36)) end", 0);
    assert!(!diags.has_errors(), "fn_id_36: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"36
");
}

#[test]
fn fn_id_37() {
    let (out, diags) = interpret("fn id(x) return x end fn main() print(id(37)) end", 0);
    assert!(!diags.has_errors(), "fn_id_37: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"37
");
}

#[test]
fn fn_id_38() {
    let (out, diags) = interpret("fn id(x) return x end fn main() print(id(38)) end", 0);
    assert!(!diags.has_errors(), "fn_id_38: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"38
");
}

#[test]
fn fn_id_39() {
    let (out, diags) = interpret("fn id(x) return x end fn main() print(id(39)) end", 0);
    assert!(!diags.has_errors(), "fn_id_39: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"39
");
}

#[test]
fn fn_id_40() {
    let (out, diags) = interpret("fn id(x) return x end fn main() print(id(40)) end", 0);
    assert!(!diags.has_errors(), "fn_id_40: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"40
");
}

#[test]
fn fn_id_41() {
    let (out, diags) = interpret("fn id(x) return x end fn main() print(id(41)) end", 0);
    assert!(!diags.has_errors(), "fn_id_41: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"41
");
}

#[test]
fn fn_id_42() {
    let (out, diags) = interpret("fn id(x) return x end fn main() print(id(42)) end", 0);
    assert!(!diags.has_errors(), "fn_id_42: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"42
");
}

#[test]
fn fn_id_43() {
    let (out, diags) = interpret("fn id(x) return x end fn main() print(id(43)) end", 0);
    assert!(!diags.has_errors(), "fn_id_43: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"43
");
}

#[test]
fn fn_id_44() {
    let (out, diags) = interpret("fn id(x) return x end fn main() print(id(44)) end", 0);
    assert!(!diags.has_errors(), "fn_id_44: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"44
");
}

#[test]
fn fn_id_45() {
    let (out, diags) = interpret("fn id(x) return x end fn main() print(id(45)) end", 0);
    assert!(!diags.has_errors(), "fn_id_45: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"45
");
}

#[test]
fn fn_id_46() {
    let (out, diags) = interpret("fn id(x) return x end fn main() print(id(46)) end", 0);
    assert!(!diags.has_errors(), "fn_id_46: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"46
");
}

#[test]
fn fn_id_47() {
    let (out, diags) = interpret("fn id(x) return x end fn main() print(id(47)) end", 0);
    assert!(!diags.has_errors(), "fn_id_47: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"47
");
}

#[test]
fn fn_id_48() {
    let (out, diags) = interpret("fn id(x) return x end fn main() print(id(48)) end", 0);
    assert!(!diags.has_errors(), "fn_id_48: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"48
");
}

#[test]
fn fn_id_49() {
    let (out, diags) = interpret("fn id(x) return x end fn main() print(id(49)) end", 0);
    assert!(!diags.has_errors(), "fn_id_49: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"49
");
}

#[test]
fn fn_double_0() {
    let (out, diags) = interpret("fn double(x) return x * 2 end fn main() print(double(0)) end", 0);
    assert!(!diags.has_errors(), "fn_double_0: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"0
");
}

#[test]
fn fn_double_1() {
    let (out, diags) = interpret("fn double(x) return x * 2 end fn main() print(double(1)) end", 0);
    assert!(!diags.has_errors(), "fn_double_1: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"2
");
}

#[test]
fn fn_double_2() {
    let (out, diags) = interpret("fn double(x) return x * 2 end fn main() print(double(2)) end", 0);
    assert!(!diags.has_errors(), "fn_double_2: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"4
");
}

#[test]
fn fn_double_3() {
    let (out, diags) = interpret("fn double(x) return x * 2 end fn main() print(double(3)) end", 0);
    assert!(!diags.has_errors(), "fn_double_3: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"6
");
}

#[test]
fn fn_double_4() {
    let (out, diags) = interpret("fn double(x) return x * 2 end fn main() print(double(4)) end", 0);
    assert!(!diags.has_errors(), "fn_double_4: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"8
");
}

#[test]
fn fn_double_5() {
    let (out, diags) = interpret("fn double(x) return x * 2 end fn main() print(double(5)) end", 0);
    assert!(!diags.has_errors(), "fn_double_5: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"10
");
}

#[test]
fn fn_double_6() {
    let (out, diags) = interpret("fn double(x) return x * 2 end fn main() print(double(6)) end", 0);
    assert!(!diags.has_errors(), "fn_double_6: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"12
");
}

#[test]
fn fn_double_7() {
    let (out, diags) = interpret("fn double(x) return x * 2 end fn main() print(double(7)) end", 0);
    assert!(!diags.has_errors(), "fn_double_7: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"14
");
}

#[test]
fn fn_double_8() {
    let (out, diags) = interpret("fn double(x) return x * 2 end fn main() print(double(8)) end", 0);
    assert!(!diags.has_errors(), "fn_double_8: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"16
");
}

#[test]
fn fn_double_9() {
    let (out, diags) = interpret("fn double(x) return x * 2 end fn main() print(double(9)) end", 0);
    assert!(!diags.has_errors(), "fn_double_9: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"18
");
}

#[test]
fn fn_double_10() {
    let (out, diags) = interpret("fn double(x) return x * 2 end fn main() print(double(10)) end", 0);
    assert!(!diags.has_errors(), "fn_double_10: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"20
");
}

#[test]
fn fn_double_11() {
    let (out, diags) = interpret("fn double(x) return x * 2 end fn main() print(double(11)) end", 0);
    assert!(!diags.has_errors(), "fn_double_11: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"22
");
}

#[test]
fn fn_double_12() {
    let (out, diags) = interpret("fn double(x) return x * 2 end fn main() print(double(12)) end", 0);
    assert!(!diags.has_errors(), "fn_double_12: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"24
");
}

#[test]
fn fn_double_13() {
    let (out, diags) = interpret("fn double(x) return x * 2 end fn main() print(double(13)) end", 0);
    assert!(!diags.has_errors(), "fn_double_13: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"26
");
}

#[test]
fn fn_double_14() {
    let (out, diags) = interpret("fn double(x) return x * 2 end fn main() print(double(14)) end", 0);
    assert!(!diags.has_errors(), "fn_double_14: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"28
");
}

#[test]
fn fn_double_15() {
    let (out, diags) = interpret("fn double(x) return x * 2 end fn main() print(double(15)) end", 0);
    assert!(!diags.has_errors(), "fn_double_15: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"30
");
}

#[test]
fn fn_double_16() {
    let (out, diags) = interpret("fn double(x) return x * 2 end fn main() print(double(16)) end", 0);
    assert!(!diags.has_errors(), "fn_double_16: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"32
");
}

#[test]
fn fn_double_17() {
    let (out, diags) = interpret("fn double(x) return x * 2 end fn main() print(double(17)) end", 0);
    assert!(!diags.has_errors(), "fn_double_17: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"34
");
}

#[test]
fn fn_double_18() {
    let (out, diags) = interpret("fn double(x) return x * 2 end fn main() print(double(18)) end", 0);
    assert!(!diags.has_errors(), "fn_double_18: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"36
");
}

#[test]
fn fn_double_19() {
    let (out, diags) = interpret("fn double(x) return x * 2 end fn main() print(double(19)) end", 0);
    assert!(!diags.has_errors(), "fn_double_19: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"38
");
}

#[test]
fn fn_double_20() {
    let (out, diags) = interpret("fn double(x) return x * 2 end fn main() print(double(20)) end", 0);
    assert!(!diags.has_errors(), "fn_double_20: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"40
");
}

#[test]
fn fn_double_21() {
    let (out, diags) = interpret("fn double(x) return x * 2 end fn main() print(double(21)) end", 0);
    assert!(!diags.has_errors(), "fn_double_21: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"42
");
}

#[test]
fn fn_double_22() {
    let (out, diags) = interpret("fn double(x) return x * 2 end fn main() print(double(22)) end", 0);
    assert!(!diags.has_errors(), "fn_double_22: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"44
");
}

#[test]
fn fn_double_23() {
    let (out, diags) = interpret("fn double(x) return x * 2 end fn main() print(double(23)) end", 0);
    assert!(!diags.has_errors(), "fn_double_23: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"46
");
}

#[test]
fn fn_double_24() {
    let (out, diags) = interpret("fn double(x) return x * 2 end fn main() print(double(24)) end", 0);
    assert!(!diags.has_errors(), "fn_double_24: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"48
");
}

#[test]
fn fn_double_25() {
    let (out, diags) = interpret("fn double(x) return x * 2 end fn main() print(double(25)) end", 0);
    assert!(!diags.has_errors(), "fn_double_25: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"50
");
}

#[test]
fn fn_double_26() {
    let (out, diags) = interpret("fn double(x) return x * 2 end fn main() print(double(26)) end", 0);
    assert!(!diags.has_errors(), "fn_double_26: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"52
");
}

#[test]
fn fn_double_27() {
    let (out, diags) = interpret("fn double(x) return x * 2 end fn main() print(double(27)) end", 0);
    assert!(!diags.has_errors(), "fn_double_27: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"54
");
}

#[test]
fn fn_double_28() {
    let (out, diags) = interpret("fn double(x) return x * 2 end fn main() print(double(28)) end", 0);
    assert!(!diags.has_errors(), "fn_double_28: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"56
");
}

#[test]
fn fn_double_29() {
    let (out, diags) = interpret("fn double(x) return x * 2 end fn main() print(double(29)) end", 0);
    assert!(!diags.has_errors(), "fn_double_29: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"58
");
}

#[test]
fn fn_double_30() {
    let (out, diags) = interpret("fn double(x) return x * 2 end fn main() print(double(30)) end", 0);
    assert!(!diags.has_errors(), "fn_double_30: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"60
");
}

#[test]
fn fn_double_31() {
    let (out, diags) = interpret("fn double(x) return x * 2 end fn main() print(double(31)) end", 0);
    assert!(!diags.has_errors(), "fn_double_31: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"62
");
}

#[test]
fn fn_double_32() {
    let (out, diags) = interpret("fn double(x) return x * 2 end fn main() print(double(32)) end", 0);
    assert!(!diags.has_errors(), "fn_double_32: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"64
");
}

#[test]
fn fn_double_33() {
    let (out, diags) = interpret("fn double(x) return x * 2 end fn main() print(double(33)) end", 0);
    assert!(!diags.has_errors(), "fn_double_33: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"66
");
}

#[test]
fn fn_double_34() {
    let (out, diags) = interpret("fn double(x) return x * 2 end fn main() print(double(34)) end", 0);
    assert!(!diags.has_errors(), "fn_double_34: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"68
");
}

#[test]
fn fn_double_35() {
    let (out, diags) = interpret("fn double(x) return x * 2 end fn main() print(double(35)) end", 0);
    assert!(!diags.has_errors(), "fn_double_35: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"70
");
}

#[test]
fn fn_double_36() {
    let (out, diags) = interpret("fn double(x) return x * 2 end fn main() print(double(36)) end", 0);
    assert!(!diags.has_errors(), "fn_double_36: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"72
");
}

#[test]
fn fn_double_37() {
    let (out, diags) = interpret("fn double(x) return x * 2 end fn main() print(double(37)) end", 0);
    assert!(!diags.has_errors(), "fn_double_37: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"74
");
}

#[test]
fn fn_double_38() {
    let (out, diags) = interpret("fn double(x) return x * 2 end fn main() print(double(38)) end", 0);
    assert!(!diags.has_errors(), "fn_double_38: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"76
");
}

#[test]
fn fn_double_39() {
    let (out, diags) = interpret("fn double(x) return x * 2 end fn main() print(double(39)) end", 0);
    assert!(!diags.has_errors(), "fn_double_39: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"78
");
}

#[test]
fn fn_double_40() {
    let (out, diags) = interpret("fn double(x) return x * 2 end fn main() print(double(40)) end", 0);
    assert!(!diags.has_errors(), "fn_double_40: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"80
");
}

#[test]
fn fn_double_41() {
    let (out, diags) = interpret("fn double(x) return x * 2 end fn main() print(double(41)) end", 0);
    assert!(!diags.has_errors(), "fn_double_41: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"82
");
}

#[test]
fn fn_double_42() {
    let (out, diags) = interpret("fn double(x) return x * 2 end fn main() print(double(42)) end", 0);
    assert!(!diags.has_errors(), "fn_double_42: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"84
");
}

#[test]
fn fn_double_43() {
    let (out, diags) = interpret("fn double(x) return x * 2 end fn main() print(double(43)) end", 0);
    assert!(!diags.has_errors(), "fn_double_43: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"86
");
}

#[test]
fn fn_double_44() {
    let (out, diags) = interpret("fn double(x) return x * 2 end fn main() print(double(44)) end", 0);
    assert!(!diags.has_errors(), "fn_double_44: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"88
");
}

#[test]
fn fn_double_45() {
    let (out, diags) = interpret("fn double(x) return x * 2 end fn main() print(double(45)) end", 0);
    assert!(!diags.has_errors(), "fn_double_45: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"90
");
}

#[test]
fn fn_double_46() {
    let (out, diags) = interpret("fn double(x) return x * 2 end fn main() print(double(46)) end", 0);
    assert!(!diags.has_errors(), "fn_double_46: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"92
");
}

#[test]
fn fn_double_47() {
    let (out, diags) = interpret("fn double(x) return x * 2 end fn main() print(double(47)) end", 0);
    assert!(!diags.has_errors(), "fn_double_47: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"94
");
}

#[test]
fn fn_double_48() {
    let (out, diags) = interpret("fn double(x) return x * 2 end fn main() print(double(48)) end", 0);
    assert!(!diags.has_errors(), "fn_double_48: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"96
");
}

#[test]
fn fn_double_49() {
    let (out, diags) = interpret("fn double(x) return x * 2 end fn main() print(double(49)) end", 0);
    assert!(!diags.has_errors(), "fn_double_49: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"98
");
}

#[test]
fn fn_square_0() {
    let (out, diags) = interpret("fn square(x) return x * x end fn main() print(square(0)) end", 0);
    assert!(!diags.has_errors(), "fn_square_0: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"0
");
}

#[test]
fn fn_square_1() {
    let (out, diags) = interpret("fn square(x) return x * x end fn main() print(square(1)) end", 0);
    assert!(!diags.has_errors(), "fn_square_1: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"1
");
}

#[test]
fn fn_square_2() {
    let (out, diags) = interpret("fn square(x) return x * x end fn main() print(square(2)) end", 0);
    assert!(!diags.has_errors(), "fn_square_2: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"4
");
}

#[test]
fn fn_square_3() {
    let (out, diags) = interpret("fn square(x) return x * x end fn main() print(square(3)) end", 0);
    assert!(!diags.has_errors(), "fn_square_3: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"9
");
}

#[test]
fn fn_square_4() {
    let (out, diags) = interpret("fn square(x) return x * x end fn main() print(square(4)) end", 0);
    assert!(!diags.has_errors(), "fn_square_4: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"16
");
}

#[test]
fn fn_square_5() {
    let (out, diags) = interpret("fn square(x) return x * x end fn main() print(square(5)) end", 0);
    assert!(!diags.has_errors(), "fn_square_5: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"25
");
}

#[test]
fn fn_square_6() {
    let (out, diags) = interpret("fn square(x) return x * x end fn main() print(square(6)) end", 0);
    assert!(!diags.has_errors(), "fn_square_6: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"36
");
}

#[test]
fn fn_square_7() {
    let (out, diags) = interpret("fn square(x) return x * x end fn main() print(square(7)) end", 0);
    assert!(!diags.has_errors(), "fn_square_7: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"49
");
}

#[test]
fn fn_square_8() {
    let (out, diags) = interpret("fn square(x) return x * x end fn main() print(square(8)) end", 0);
    assert!(!diags.has_errors(), "fn_square_8: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"64
");
}

#[test]
fn fn_square_9() {
    let (out, diags) = interpret("fn square(x) return x * x end fn main() print(square(9)) end", 0);
    assert!(!diags.has_errors(), "fn_square_9: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"81
");
}

#[test]
fn fn_square_10() {
    let (out, diags) = interpret("fn square(x) return x * x end fn main() print(square(10)) end", 0);
    assert!(!diags.has_errors(), "fn_square_10: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"100
");
}

#[test]
fn fn_square_11() {
    let (out, diags) = interpret("fn square(x) return x * x end fn main() print(square(11)) end", 0);
    assert!(!diags.has_errors(), "fn_square_11: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"121
");
}

#[test]
fn fn_square_12() {
    let (out, diags) = interpret("fn square(x) return x * x end fn main() print(square(12)) end", 0);
    assert!(!diags.has_errors(), "fn_square_12: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"144
");
}

#[test]
fn fn_square_13() {
    let (out, diags) = interpret("fn square(x) return x * x end fn main() print(square(13)) end", 0);
    assert!(!diags.has_errors(), "fn_square_13: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"169
");
}

#[test]
fn fn_square_14() {
    let (out, diags) = interpret("fn square(x) return x * x end fn main() print(square(14)) end", 0);
    assert!(!diags.has_errors(), "fn_square_14: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"196
");
}

#[test]
fn fn_square_15() {
    let (out, diags) = interpret("fn square(x) return x * x end fn main() print(square(15)) end", 0);
    assert!(!diags.has_errors(), "fn_square_15: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"225
");
}

#[test]
fn fn_square_16() {
    let (out, diags) = interpret("fn square(x) return x * x end fn main() print(square(16)) end", 0);
    assert!(!diags.has_errors(), "fn_square_16: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"256
");
}

#[test]
fn fn_square_17() {
    let (out, diags) = interpret("fn square(x) return x * x end fn main() print(square(17)) end", 0);
    assert!(!diags.has_errors(), "fn_square_17: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"289
");
}

#[test]
fn fn_square_18() {
    let (out, diags) = interpret("fn square(x) return x * x end fn main() print(square(18)) end", 0);
    assert!(!diags.has_errors(), "fn_square_18: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"324
");
}

#[test]
fn fn_square_19() {
    let (out, diags) = interpret("fn square(x) return x * x end fn main() print(square(19)) end", 0);
    assert!(!diags.has_errors(), "fn_square_19: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"361
");
}

#[test]
fn fn_square_20() {
    let (out, diags) = interpret("fn square(x) return x * x end fn main() print(square(20)) end", 0);
    assert!(!diags.has_errors(), "fn_square_20: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"400
");
}

#[test]
fn fn_square_21() {
    let (out, diags) = interpret("fn square(x) return x * x end fn main() print(square(21)) end", 0);
    assert!(!diags.has_errors(), "fn_square_21: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"441
");
}

#[test]
fn fn_square_22() {
    let (out, diags) = interpret("fn square(x) return x * x end fn main() print(square(22)) end", 0);
    assert!(!diags.has_errors(), "fn_square_22: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"484
");
}

#[test]
fn fn_square_23() {
    let (out, diags) = interpret("fn square(x) return x * x end fn main() print(square(23)) end", 0);
    assert!(!diags.has_errors(), "fn_square_23: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"529
");
}

#[test]
fn fn_square_24() {
    let (out, diags) = interpret("fn square(x) return x * x end fn main() print(square(24)) end", 0);
    assert!(!diags.has_errors(), "fn_square_24: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"576
");
}

#[test]
fn fn_square_25() {
    let (out, diags) = interpret("fn square(x) return x * x end fn main() print(square(25)) end", 0);
    assert!(!diags.has_errors(), "fn_square_25: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"625
");
}

#[test]
fn fn_square_26() {
    let (out, diags) = interpret("fn square(x) return x * x end fn main() print(square(26)) end", 0);
    assert!(!diags.has_errors(), "fn_square_26: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"676
");
}

#[test]
fn fn_square_27() {
    let (out, diags) = interpret("fn square(x) return x * x end fn main() print(square(27)) end", 0);
    assert!(!diags.has_errors(), "fn_square_27: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"729
");
}

#[test]
fn fn_square_28() {
    let (out, diags) = interpret("fn square(x) return x * x end fn main() print(square(28)) end", 0);
    assert!(!diags.has_errors(), "fn_square_28: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"784
");
}

#[test]
fn fn_square_29() {
    let (out, diags) = interpret("fn square(x) return x * x end fn main() print(square(29)) end", 0);
    assert!(!diags.has_errors(), "fn_square_29: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"841
");
}

#[test]
fn fn_square_30() {
    let (out, diags) = interpret("fn square(x) return x * x end fn main() print(square(30)) end", 0);
    assert!(!diags.has_errors(), "fn_square_30: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"900
");
}

#[test]
fn fn_square_31() {
    let (out, diags) = interpret("fn square(x) return x * x end fn main() print(square(31)) end", 0);
    assert!(!diags.has_errors(), "fn_square_31: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"961
");
}

#[test]
fn fn_square_32() {
    let (out, diags) = interpret("fn square(x) return x * x end fn main() print(square(32)) end", 0);
    assert!(!diags.has_errors(), "fn_square_32: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"1024
");
}

#[test]
fn fn_square_33() {
    let (out, diags) = interpret("fn square(x) return x * x end fn main() print(square(33)) end", 0);
    assert!(!diags.has_errors(), "fn_square_33: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"1089
");
}

#[test]
fn fn_square_34() {
    let (out, diags) = interpret("fn square(x) return x * x end fn main() print(square(34)) end", 0);
    assert!(!diags.has_errors(), "fn_square_34: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"1156
");
}

#[test]
fn fn_square_35() {
    let (out, diags) = interpret("fn square(x) return x * x end fn main() print(square(35)) end", 0);
    assert!(!diags.has_errors(), "fn_square_35: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"1225
");
}

#[test]
fn fn_square_36() {
    let (out, diags) = interpret("fn square(x) return x * x end fn main() print(square(36)) end", 0);
    assert!(!diags.has_errors(), "fn_square_36: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"1296
");
}

#[test]
fn fn_square_37() {
    let (out, diags) = interpret("fn square(x) return x * x end fn main() print(square(37)) end", 0);
    assert!(!diags.has_errors(), "fn_square_37: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"1369
");
}

#[test]
fn fn_square_38() {
    let (out, diags) = interpret("fn square(x) return x * x end fn main() print(square(38)) end", 0);
    assert!(!diags.has_errors(), "fn_square_38: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"1444
");
}

#[test]
fn fn_square_39() {
    let (out, diags) = interpret("fn square(x) return x * x end fn main() print(square(39)) end", 0);
    assert!(!diags.has_errors(), "fn_square_39: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"1521
");
}

#[test]
fn fn_square_40() {
    let (out, diags) = interpret("fn square(x) return x * x end fn main() print(square(40)) end", 0);
    assert!(!diags.has_errors(), "fn_square_40: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"1600
");
}

#[test]
fn fn_square_41() {
    let (out, diags) = interpret("fn square(x) return x * x end fn main() print(square(41)) end", 0);
    assert!(!diags.has_errors(), "fn_square_41: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"1681
");
}

#[test]
fn fn_square_42() {
    let (out, diags) = interpret("fn square(x) return x * x end fn main() print(square(42)) end", 0);
    assert!(!diags.has_errors(), "fn_square_42: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"1764
");
}

#[test]
fn fn_square_43() {
    let (out, diags) = interpret("fn square(x) return x * x end fn main() print(square(43)) end", 0);
    assert!(!diags.has_errors(), "fn_square_43: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"1849
");
}

#[test]
fn fn_square_44() {
    let (out, diags) = interpret("fn square(x) return x * x end fn main() print(square(44)) end", 0);
    assert!(!diags.has_errors(), "fn_square_44: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"1936
");
}

#[test]
fn fn_square_45() {
    let (out, diags) = interpret("fn square(x) return x * x end fn main() print(square(45)) end", 0);
    assert!(!diags.has_errors(), "fn_square_45: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"2025
");
}

#[test]
fn fn_square_46() {
    let (out, diags) = interpret("fn square(x) return x * x end fn main() print(square(46)) end", 0);
    assert!(!diags.has_errors(), "fn_square_46: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"2116
");
}

#[test]
fn fn_square_47() {
    let (out, diags) = interpret("fn square(x) return x * x end fn main() print(square(47)) end", 0);
    assert!(!diags.has_errors(), "fn_square_47: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"2209
");
}

#[test]
fn fn_square_48() {
    let (out, diags) = interpret("fn square(x) return x * x end fn main() print(square(48)) end", 0);
    assert!(!diags.has_errors(), "fn_square_48: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"2304
");
}

#[test]
fn fn_square_49() {
    let (out, diags) = interpret("fn square(x) return x * x end fn main() print(square(49)) end", 0);
    assert!(!diags.has_errors(), "fn_square_49: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"2401
");
}

#[test]
fn fn_neg_0() {
    let (out, diags) = interpret("fn neg(x) return -x end fn main() print(neg(0)) end", 0);
    assert!(!diags.has_errors(), "fn_neg_0: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"0
");
}

#[test]
fn fn_neg_1() {
    let (out, diags) = interpret("fn neg(x) return -x end fn main() print(neg(1)) end", 0);
    assert!(!diags.has_errors(), "fn_neg_1: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"-1
");
}

#[test]
fn fn_neg_2() {
    let (out, diags) = interpret("fn neg(x) return -x end fn main() print(neg(2)) end", 0);
    assert!(!diags.has_errors(), "fn_neg_2: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"-2
");
}

#[test]
fn fn_neg_3() {
    let (out, diags) = interpret("fn neg(x) return -x end fn main() print(neg(3)) end", 0);
    assert!(!diags.has_errors(), "fn_neg_3: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"-3
");
}

#[test]
fn fn_neg_4() {
    let (out, diags) = interpret("fn neg(x) return -x end fn main() print(neg(4)) end", 0);
    assert!(!diags.has_errors(), "fn_neg_4: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"-4
");
}

#[test]
fn fn_neg_5() {
    let (out, diags) = interpret("fn neg(x) return -x end fn main() print(neg(5)) end", 0);
    assert!(!diags.has_errors(), "fn_neg_5: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"-5
");
}

#[test]
fn fn_neg_6() {
    let (out, diags) = interpret("fn neg(x) return -x end fn main() print(neg(6)) end", 0);
    assert!(!diags.has_errors(), "fn_neg_6: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"-6
");
}

#[test]
fn fn_neg_7() {
    let (out, diags) = interpret("fn neg(x) return -x end fn main() print(neg(7)) end", 0);
    assert!(!diags.has_errors(), "fn_neg_7: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"-7
");
}

#[test]
fn fn_neg_8() {
    let (out, diags) = interpret("fn neg(x) return -x end fn main() print(neg(8)) end", 0);
    assert!(!diags.has_errors(), "fn_neg_8: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"-8
");
}

#[test]
fn fn_neg_9() {
    let (out, diags) = interpret("fn neg(x) return -x end fn main() print(neg(9)) end", 0);
    assert!(!diags.has_errors(), "fn_neg_9: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"-9
");
}

#[test]
fn fn_neg_10() {
    let (out, diags) = interpret("fn neg(x) return -x end fn main() print(neg(10)) end", 0);
    assert!(!diags.has_errors(), "fn_neg_10: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"-10
");
}

#[test]
fn fn_neg_11() {
    let (out, diags) = interpret("fn neg(x) return -x end fn main() print(neg(11)) end", 0);
    assert!(!diags.has_errors(), "fn_neg_11: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"-11
");
}

#[test]
fn fn_neg_12() {
    let (out, diags) = interpret("fn neg(x) return -x end fn main() print(neg(12)) end", 0);
    assert!(!diags.has_errors(), "fn_neg_12: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"-12
");
}

#[test]
fn fn_neg_13() {
    let (out, diags) = interpret("fn neg(x) return -x end fn main() print(neg(13)) end", 0);
    assert!(!diags.has_errors(), "fn_neg_13: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"-13
");
}

#[test]
fn fn_neg_14() {
    let (out, diags) = interpret("fn neg(x) return -x end fn main() print(neg(14)) end", 0);
    assert!(!diags.has_errors(), "fn_neg_14: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"-14
");
}

#[test]
fn fn_neg_15() {
    let (out, diags) = interpret("fn neg(x) return -x end fn main() print(neg(15)) end", 0);
    assert!(!diags.has_errors(), "fn_neg_15: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"-15
");
}

#[test]
fn fn_neg_16() {
    let (out, diags) = interpret("fn neg(x) return -x end fn main() print(neg(16)) end", 0);
    assert!(!diags.has_errors(), "fn_neg_16: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"-16
");
}

#[test]
fn fn_neg_17() {
    let (out, diags) = interpret("fn neg(x) return -x end fn main() print(neg(17)) end", 0);
    assert!(!diags.has_errors(), "fn_neg_17: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"-17
");
}

#[test]
fn fn_neg_18() {
    let (out, diags) = interpret("fn neg(x) return -x end fn main() print(neg(18)) end", 0);
    assert!(!diags.has_errors(), "fn_neg_18: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"-18
");
}

#[test]
fn fn_neg_19() {
    let (out, diags) = interpret("fn neg(x) return -x end fn main() print(neg(19)) end", 0);
    assert!(!diags.has_errors(), "fn_neg_19: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"-19
");
}

#[test]
fn fn_neg_20() {
    let (out, diags) = interpret("fn neg(x) return -x end fn main() print(neg(20)) end", 0);
    assert!(!diags.has_errors(), "fn_neg_20: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"-20
");
}

#[test]
fn fn_neg_21() {
    let (out, diags) = interpret("fn neg(x) return -x end fn main() print(neg(21)) end", 0);
    assert!(!diags.has_errors(), "fn_neg_21: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"-21
");
}

#[test]
fn fn_neg_22() {
    let (out, diags) = interpret("fn neg(x) return -x end fn main() print(neg(22)) end", 0);
    assert!(!diags.has_errors(), "fn_neg_22: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"-22
");
}

#[test]
fn fn_neg_23() {
    let (out, diags) = interpret("fn neg(x) return -x end fn main() print(neg(23)) end", 0);
    assert!(!diags.has_errors(), "fn_neg_23: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"-23
");
}

#[test]
fn fn_neg_24() {
    let (out, diags) = interpret("fn neg(x) return -x end fn main() print(neg(24)) end", 0);
    assert!(!diags.has_errors(), "fn_neg_24: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"-24
");
}

#[test]
fn fn_neg_25() {
    let (out, diags) = interpret("fn neg(x) return -x end fn main() print(neg(25)) end", 0);
    assert!(!diags.has_errors(), "fn_neg_25: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"-25
");
}

#[test]
fn fn_neg_26() {
    let (out, diags) = interpret("fn neg(x) return -x end fn main() print(neg(26)) end", 0);
    assert!(!diags.has_errors(), "fn_neg_26: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"-26
");
}

#[test]
fn fn_neg_27() {
    let (out, diags) = interpret("fn neg(x) return -x end fn main() print(neg(27)) end", 0);
    assert!(!diags.has_errors(), "fn_neg_27: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"-27
");
}

#[test]
fn fn_neg_28() {
    let (out, diags) = interpret("fn neg(x) return -x end fn main() print(neg(28)) end", 0);
    assert!(!diags.has_errors(), "fn_neg_28: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"-28
");
}

#[test]
fn fn_neg_29() {
    let (out, diags) = interpret("fn neg(x) return -x end fn main() print(neg(29)) end", 0);
    assert!(!diags.has_errors(), "fn_neg_29: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"-29
");
}

