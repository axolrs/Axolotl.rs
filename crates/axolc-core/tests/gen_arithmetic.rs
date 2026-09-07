// Owner: PascalElixir / axolrs (GitHub org)
// File: Auto-generated tests for arithmetic operations.

use axolc_core::interpret;

#[test]
fn add_0() {
    let (out, diags) = interpret("fn main() print(0 + 7) end", 0);
    assert!(!diags.has_errors(), "add_0: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"7
");
}

#[test]
fn add_1() {
    let (out, diags) = interpret("fn main() print(3 + 8) end", 0);
    assert!(!diags.has_errors(), "add_1: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"11
");
}

#[test]
fn add_2() {
    let (out, diags) = interpret("fn main() print(6 + 9) end", 0);
    assert!(!diags.has_errors(), "add_2: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"15
");
}

#[test]
fn add_3() {
    let (out, diags) = interpret("fn main() print(9 + 10) end", 0);
    assert!(!diags.has_errors(), "add_3: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"19
");
}

#[test]
fn add_4() {
    let (out, diags) = interpret("fn main() print(12 + 11) end", 0);
    assert!(!diags.has_errors(), "add_4: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"23
");
}

#[test]
fn add_5() {
    let (out, diags) = interpret("fn main() print(15 + 12) end", 0);
    assert!(!diags.has_errors(), "add_5: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"27
");
}

#[test]
fn add_6() {
    let (out, diags) = interpret("fn main() print(18 + 13) end", 0);
    assert!(!diags.has_errors(), "add_6: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"31
");
}

#[test]
fn add_7() {
    let (out, diags) = interpret("fn main() print(21 + 14) end", 0);
    assert!(!diags.has_errors(), "add_7: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"35
");
}

#[test]
fn add_8() {
    let (out, diags) = interpret("fn main() print(24 + 15) end", 0);
    assert!(!diags.has_errors(), "add_8: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"39
");
}

#[test]
fn add_9() {
    let (out, diags) = interpret("fn main() print(27 + 16) end", 0);
    assert!(!diags.has_errors(), "add_9: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"43
");
}

#[test]
fn add_10() {
    let (out, diags) = interpret("fn main() print(30 + 17) end", 0);
    assert!(!diags.has_errors(), "add_10: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"47
");
}

#[test]
fn add_11() {
    let (out, diags) = interpret("fn main() print(33 + 18) end", 0);
    assert!(!diags.has_errors(), "add_11: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"51
");
}

#[test]
fn add_12() {
    let (out, diags) = interpret("fn main() print(36 + 19) end", 0);
    assert!(!diags.has_errors(), "add_12: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"55
");
}

#[test]
fn add_13() {
    let (out, diags) = interpret("fn main() print(39 + 20) end", 0);
    assert!(!diags.has_errors(), "add_13: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"59
");
}

#[test]
fn add_14() {
    let (out, diags) = interpret("fn main() print(42 + 21) end", 0);
    assert!(!diags.has_errors(), "add_14: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"63
");
}

#[test]
fn add_15() {
    let (out, diags) = interpret("fn main() print(45 + 22) end", 0);
    assert!(!diags.has_errors(), "add_15: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"67
");
}

#[test]
fn add_16() {
    let (out, diags) = interpret("fn main() print(48 + 23) end", 0);
    assert!(!diags.has_errors(), "add_16: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"71
");
}

#[test]
fn add_17() {
    let (out, diags) = interpret("fn main() print(51 + 24) end", 0);
    assert!(!diags.has_errors(), "add_17: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"75
");
}

#[test]
fn add_18() {
    let (out, diags) = interpret("fn main() print(54 + 25) end", 0);
    assert!(!diags.has_errors(), "add_18: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"79
");
}

#[test]
fn add_19() {
    let (out, diags) = interpret("fn main() print(57 + 26) end", 0);
    assert!(!diags.has_errors(), "add_19: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"83
");
}

#[test]
fn add_20() {
    let (out, diags) = interpret("fn main() print(60 + 27) end", 0);
    assert!(!diags.has_errors(), "add_20: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"87
");
}

#[test]
fn add_21() {
    let (out, diags) = interpret("fn main() print(63 + 28) end", 0);
    assert!(!diags.has_errors(), "add_21: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"91
");
}

#[test]
fn add_22() {
    let (out, diags) = interpret("fn main() print(66 + 29) end", 0);
    assert!(!diags.has_errors(), "add_22: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"95
");
}

#[test]
fn add_23() {
    let (out, diags) = interpret("fn main() print(69 + 30) end", 0);
    assert!(!diags.has_errors(), "add_23: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"99
");
}

#[test]
fn add_24() {
    let (out, diags) = interpret("fn main() print(72 + 31) end", 0);
    assert!(!diags.has_errors(), "add_24: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"103
");
}

#[test]
fn add_25() {
    let (out, diags) = interpret("fn main() print(75 + 32) end", 0);
    assert!(!diags.has_errors(), "add_25: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"107
");
}

#[test]
fn add_26() {
    let (out, diags) = interpret("fn main() print(78 + 33) end", 0);
    assert!(!diags.has_errors(), "add_26: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"111
");
}

#[test]
fn add_27() {
    let (out, diags) = interpret("fn main() print(81 + 34) end", 0);
    assert!(!diags.has_errors(), "add_27: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"115
");
}

#[test]
fn add_28() {
    let (out, diags) = interpret("fn main() print(84 + 35) end", 0);
    assert!(!diags.has_errors(), "add_28: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"119
");
}

#[test]
fn add_29() {
    let (out, diags) = interpret("fn main() print(87 + 36) end", 0);
    assert!(!diags.has_errors(), "add_29: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"123
");
}

#[test]
fn add_30() {
    let (out, diags) = interpret("fn main() print(90 + 37) end", 0);
    assert!(!diags.has_errors(), "add_30: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"127
");
}

#[test]
fn add_31() {
    let (out, diags) = interpret("fn main() print(93 + 38) end", 0);
    assert!(!diags.has_errors(), "add_31: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"131
");
}

#[test]
fn add_32() {
    let (out, diags) = interpret("fn main() print(96 + 39) end", 0);
    assert!(!diags.has_errors(), "add_32: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"135
");
}

#[test]
fn add_33() {
    let (out, diags) = interpret("fn main() print(99 + 40) end", 0);
    assert!(!diags.has_errors(), "add_33: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"139
");
}

#[test]
fn add_34() {
    let (out, diags) = interpret("fn main() print(102 + 41) end", 0);
    assert!(!diags.has_errors(), "add_34: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"143
");
}

#[test]
fn add_35() {
    let (out, diags) = interpret("fn main() print(105 + 42) end", 0);
    assert!(!diags.has_errors(), "add_35: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"147
");
}

#[test]
fn add_36() {
    let (out, diags) = interpret("fn main() print(108 + 43) end", 0);
    assert!(!diags.has_errors(), "add_36: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"151
");
}

#[test]
fn add_37() {
    let (out, diags) = interpret("fn main() print(111 + 44) end", 0);
    assert!(!diags.has_errors(), "add_37: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"155
");
}

#[test]
fn add_38() {
    let (out, diags) = interpret("fn main() print(114 + 45) end", 0);
    assert!(!diags.has_errors(), "add_38: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"159
");
}

#[test]
fn add_39() {
    let (out, diags) = interpret("fn main() print(117 + 46) end", 0);
    assert!(!diags.has_errors(), "add_39: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"163
");
}

#[test]
fn add_40() {
    let (out, diags) = interpret("fn main() print(120 + 47) end", 0);
    assert!(!diags.has_errors(), "add_40: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"167
");
}

#[test]
fn add_41() {
    let (out, diags) = interpret("fn main() print(123 + 48) end", 0);
    assert!(!diags.has_errors(), "add_41: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"171
");
}

#[test]
fn add_42() {
    let (out, diags) = interpret("fn main() print(126 + 49) end", 0);
    assert!(!diags.has_errors(), "add_42: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"175
");
}

#[test]
fn add_43() {
    let (out, diags) = interpret("fn main() print(129 + 50) end", 0);
    assert!(!diags.has_errors(), "add_43: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"179
");
}

#[test]
fn add_44() {
    let (out, diags) = interpret("fn main() print(132 + 51) end", 0);
    assert!(!diags.has_errors(), "add_44: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"183
");
}

#[test]
fn add_45() {
    let (out, diags) = interpret("fn main() print(135 + 52) end", 0);
    assert!(!diags.has_errors(), "add_45: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"187
");
}

#[test]
fn add_46() {
    let (out, diags) = interpret("fn main() print(138 + 53) end", 0);
    assert!(!diags.has_errors(), "add_46: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"191
");
}

#[test]
fn add_47() {
    let (out, diags) = interpret("fn main() print(141 + 54) end", 0);
    assert!(!diags.has_errors(), "add_47: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"195
");
}

#[test]
fn add_48() {
    let (out, diags) = interpret("fn main() print(144 + 55) end", 0);
    assert!(!diags.has_errors(), "add_48: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"199
");
}

#[test]
fn add_49() {
    let (out, diags) = interpret("fn main() print(147 + 56) end", 0);
    assert!(!diags.has_errors(), "add_49: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"203
");
}

#[test]
fn add_50() {
    let (out, diags) = interpret("fn main() print(150 + 57) end", 0);
    assert!(!diags.has_errors(), "add_50: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"207
");
}

#[test]
fn add_51() {
    let (out, diags) = interpret("fn main() print(153 + 58) end", 0);
    assert!(!diags.has_errors(), "add_51: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"211
");
}

#[test]
fn add_52() {
    let (out, diags) = interpret("fn main() print(156 + 59) end", 0);
    assert!(!diags.has_errors(), "add_52: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"215
");
}

#[test]
fn add_53() {
    let (out, diags) = interpret("fn main() print(159 + 60) end", 0);
    assert!(!diags.has_errors(), "add_53: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"219
");
}

#[test]
fn add_54() {
    let (out, diags) = interpret("fn main() print(162 + 61) end", 0);
    assert!(!diags.has_errors(), "add_54: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"223
");
}

#[test]
fn add_55() {
    let (out, diags) = interpret("fn main() print(165 + 62) end", 0);
    assert!(!diags.has_errors(), "add_55: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"227
");
}

#[test]
fn add_56() {
    let (out, diags) = interpret("fn main() print(168 + 63) end", 0);
    assert!(!diags.has_errors(), "add_56: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"231
");
}

#[test]
fn add_57() {
    let (out, diags) = interpret("fn main() print(171 + 64) end", 0);
    assert!(!diags.has_errors(), "add_57: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"235
");
}

#[test]
fn add_58() {
    let (out, diags) = interpret("fn main() print(174 + 65) end", 0);
    assert!(!diags.has_errors(), "add_58: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"239
");
}

#[test]
fn add_59() {
    let (out, diags) = interpret("fn main() print(177 + 66) end", 0);
    assert!(!diags.has_errors(), "add_59: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"243
");
}

#[test]
fn add_60() {
    let (out, diags) = interpret("fn main() print(180 + 67) end", 0);
    assert!(!diags.has_errors(), "add_60: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"247
");
}

#[test]
fn add_61() {
    let (out, diags) = interpret("fn main() print(183 + 68) end", 0);
    assert!(!diags.has_errors(), "add_61: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"251
");
}

#[test]
fn add_62() {
    let (out, diags) = interpret("fn main() print(186 + 69) end", 0);
    assert!(!diags.has_errors(), "add_62: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"255
");
}

#[test]
fn add_63() {
    let (out, diags) = interpret("fn main() print(189 + 70) end", 0);
    assert!(!diags.has_errors(), "add_63: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"259
");
}

#[test]
fn add_64() {
    let (out, diags) = interpret("fn main() print(192 + 71) end", 0);
    assert!(!diags.has_errors(), "add_64: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"263
");
}

#[test]
fn add_65() {
    let (out, diags) = interpret("fn main() print(195 + 72) end", 0);
    assert!(!diags.has_errors(), "add_65: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"267
");
}

#[test]
fn add_66() {
    let (out, diags) = interpret("fn main() print(198 + 73) end", 0);
    assert!(!diags.has_errors(), "add_66: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"271
");
}

#[test]
fn add_67() {
    let (out, diags) = interpret("fn main() print(201 + 74) end", 0);
    assert!(!diags.has_errors(), "add_67: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"275
");
}

#[test]
fn add_68() {
    let (out, diags) = interpret("fn main() print(204 + 75) end", 0);
    assert!(!diags.has_errors(), "add_68: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"279
");
}

#[test]
fn add_69() {
    let (out, diags) = interpret("fn main() print(207 + 76) end", 0);
    assert!(!diags.has_errors(), "add_69: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"283
");
}

#[test]
fn add_70() {
    let (out, diags) = interpret("fn main() print(210 + 77) end", 0);
    assert!(!diags.has_errors(), "add_70: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"287
");
}

#[test]
fn add_71() {
    let (out, diags) = interpret("fn main() print(213 + 78) end", 0);
    assert!(!diags.has_errors(), "add_71: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"291
");
}

#[test]
fn add_72() {
    let (out, diags) = interpret("fn main() print(216 + 79) end", 0);
    assert!(!diags.has_errors(), "add_72: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"295
");
}

#[test]
fn add_73() {
    let (out, diags) = interpret("fn main() print(219 + 80) end", 0);
    assert!(!diags.has_errors(), "add_73: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"299
");
}

#[test]
fn add_74() {
    let (out, diags) = interpret("fn main() print(222 + 81) end", 0);
    assert!(!diags.has_errors(), "add_74: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"303
");
}

#[test]
fn add_75() {
    let (out, diags) = interpret("fn main() print(225 + 82) end", 0);
    assert!(!diags.has_errors(), "add_75: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"307
");
}

#[test]
fn add_76() {
    let (out, diags) = interpret("fn main() print(228 + 83) end", 0);
    assert!(!diags.has_errors(), "add_76: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"311
");
}

#[test]
fn add_77() {
    let (out, diags) = interpret("fn main() print(231 + 84) end", 0);
    assert!(!diags.has_errors(), "add_77: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"315
");
}

#[test]
fn add_78() {
    let (out, diags) = interpret("fn main() print(234 + 85) end", 0);
    assert!(!diags.has_errors(), "add_78: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"319
");
}

#[test]
fn add_79() {
    let (out, diags) = interpret("fn main() print(237 + 86) end", 0);
    assert!(!diags.has_errors(), "add_79: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"323
");
}

#[test]
fn add_80() {
    let (out, diags) = interpret("fn main() print(240 + 87) end", 0);
    assert!(!diags.has_errors(), "add_80: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"327
");
}

#[test]
fn add_81() {
    let (out, diags) = interpret("fn main() print(243 + 88) end", 0);
    assert!(!diags.has_errors(), "add_81: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"331
");
}

#[test]
fn add_82() {
    let (out, diags) = interpret("fn main() print(246 + 89) end", 0);
    assert!(!diags.has_errors(), "add_82: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"335
");
}

#[test]
fn add_83() {
    let (out, diags) = interpret("fn main() print(249 + 90) end", 0);
    assert!(!diags.has_errors(), "add_83: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"339
");
}

#[test]
fn add_84() {
    let (out, diags) = interpret("fn main() print(252 + 91) end", 0);
    assert!(!diags.has_errors(), "add_84: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"343
");
}

#[test]
fn add_85() {
    let (out, diags) = interpret("fn main() print(255 + 92) end", 0);
    assert!(!diags.has_errors(), "add_85: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"347
");
}

#[test]
fn add_86() {
    let (out, diags) = interpret("fn main() print(258 + 93) end", 0);
    assert!(!diags.has_errors(), "add_86: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"351
");
}

#[test]
fn add_87() {
    let (out, diags) = interpret("fn main() print(261 + 94) end", 0);
    assert!(!diags.has_errors(), "add_87: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"355
");
}

#[test]
fn add_88() {
    let (out, diags) = interpret("fn main() print(264 + 95) end", 0);
    assert!(!diags.has_errors(), "add_88: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"359
");
}

#[test]
fn add_89() {
    let (out, diags) = interpret("fn main() print(267 + 96) end", 0);
    assert!(!diags.has_errors(), "add_89: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"363
");
}

#[test]
fn add_90() {
    let (out, diags) = interpret("fn main() print(270 + 97) end", 0);
    assert!(!diags.has_errors(), "add_90: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"367
");
}

#[test]
fn add_91() {
    let (out, diags) = interpret("fn main() print(273 + 98) end", 0);
    assert!(!diags.has_errors(), "add_91: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"371
");
}

#[test]
fn add_92() {
    let (out, diags) = interpret("fn main() print(276 + 99) end", 0);
    assert!(!diags.has_errors(), "add_92: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"375
");
}

#[test]
fn add_93() {
    let (out, diags) = interpret("fn main() print(279 + 100) end", 0);
    assert!(!diags.has_errors(), "add_93: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"379
");
}

#[test]
fn add_94() {
    let (out, diags) = interpret("fn main() print(282 + 101) end", 0);
    assert!(!diags.has_errors(), "add_94: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"383
");
}

#[test]
fn add_95() {
    let (out, diags) = interpret("fn main() print(285 + 102) end", 0);
    assert!(!diags.has_errors(), "add_95: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"387
");
}

#[test]
fn add_96() {
    let (out, diags) = interpret("fn main() print(288 + 103) end", 0);
    assert!(!diags.has_errors(), "add_96: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"391
");
}

#[test]
fn add_97() {
    let (out, diags) = interpret("fn main() print(291 + 104) end", 0);
    assert!(!diags.has_errors(), "add_97: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"395
");
}

#[test]
fn add_98() {
    let (out, diags) = interpret("fn main() print(294 + 105) end", 0);
    assert!(!diags.has_errors(), "add_98: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"399
");
}

#[test]
fn add_99() {
    let (out, diags) = interpret("fn main() print(297 + 106) end", 0);
    assert!(!diags.has_errors(), "add_99: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"403
");
}

#[test]
fn sub_0() {
    let (out, diags) = interpret("fn main() print(100 - 1) end", 0);
    assert!(!diags.has_errors(), "sub_0: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"99
");
}

#[test]
fn sub_1() {
    let (out, diags) = interpret("fn main() print(110 - 2) end", 0);
    assert!(!diags.has_errors(), "sub_1: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"108
");
}

#[test]
fn sub_2() {
    let (out, diags) = interpret("fn main() print(120 - 3) end", 0);
    assert!(!diags.has_errors(), "sub_2: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"117
");
}

#[test]
fn sub_3() {
    let (out, diags) = interpret("fn main() print(130 - 4) end", 0);
    assert!(!diags.has_errors(), "sub_3: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"126
");
}

#[test]
fn sub_4() {
    let (out, diags) = interpret("fn main() print(140 - 5) end", 0);
    assert!(!diags.has_errors(), "sub_4: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"135
");
}

#[test]
fn sub_5() {
    let (out, diags) = interpret("fn main() print(150 - 6) end", 0);
    assert!(!diags.has_errors(), "sub_5: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"144
");
}

#[test]
fn sub_6() {
    let (out, diags) = interpret("fn main() print(160 - 7) end", 0);
    assert!(!diags.has_errors(), "sub_6: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"153
");
}

#[test]
fn sub_7() {
    let (out, diags) = interpret("fn main() print(170 - 8) end", 0);
    assert!(!diags.has_errors(), "sub_7: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"162
");
}

#[test]
fn sub_8() {
    let (out, diags) = interpret("fn main() print(180 - 9) end", 0);
    assert!(!diags.has_errors(), "sub_8: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"171
");
}

#[test]
fn sub_9() {
    let (out, diags) = interpret("fn main() print(190 - 10) end", 0);
    assert!(!diags.has_errors(), "sub_9: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"180
");
}

#[test]
fn sub_10() {
    let (out, diags) = interpret("fn main() print(200 - 11) end", 0);
    assert!(!diags.has_errors(), "sub_10: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"189
");
}

#[test]
fn sub_11() {
    let (out, diags) = interpret("fn main() print(210 - 12) end", 0);
    assert!(!diags.has_errors(), "sub_11: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"198
");
}

#[test]
fn sub_12() {
    let (out, diags) = interpret("fn main() print(220 - 13) end", 0);
    assert!(!diags.has_errors(), "sub_12: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"207
");
}

#[test]
fn sub_13() {
    let (out, diags) = interpret("fn main() print(230 - 14) end", 0);
    assert!(!diags.has_errors(), "sub_13: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"216
");
}

#[test]
fn sub_14() {
    let (out, diags) = interpret("fn main() print(240 - 15) end", 0);
    assert!(!diags.has_errors(), "sub_14: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"225
");
}

#[test]
fn sub_15() {
    let (out, diags) = interpret("fn main() print(250 - 16) end", 0);
    assert!(!diags.has_errors(), "sub_15: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"234
");
}

#[test]
fn sub_16() {
    let (out, diags) = interpret("fn main() print(260 - 17) end", 0);
    assert!(!diags.has_errors(), "sub_16: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"243
");
}

#[test]
fn sub_17() {
    let (out, diags) = interpret("fn main() print(270 - 18) end", 0);
    assert!(!diags.has_errors(), "sub_17: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"252
");
}

#[test]
fn sub_18() {
    let (out, diags) = interpret("fn main() print(280 - 19) end", 0);
    assert!(!diags.has_errors(), "sub_18: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"261
");
}

#[test]
fn sub_19() {
    let (out, diags) = interpret("fn main() print(290 - 20) end", 0);
    assert!(!diags.has_errors(), "sub_19: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"270
");
}

#[test]
fn sub_20() {
    let (out, diags) = interpret("fn main() print(300 - 21) end", 0);
    assert!(!diags.has_errors(), "sub_20: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"279
");
}

#[test]
fn sub_21() {
    let (out, diags) = interpret("fn main() print(310 - 22) end", 0);
    assert!(!diags.has_errors(), "sub_21: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"288
");
}

#[test]
fn sub_22() {
    let (out, diags) = interpret("fn main() print(320 - 23) end", 0);
    assert!(!diags.has_errors(), "sub_22: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"297
");
}

#[test]
fn sub_23() {
    let (out, diags) = interpret("fn main() print(330 - 24) end", 0);
    assert!(!diags.has_errors(), "sub_23: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"306
");
}

#[test]
fn sub_24() {
    let (out, diags) = interpret("fn main() print(340 - 25) end", 0);
    assert!(!diags.has_errors(), "sub_24: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"315
");
}

#[test]
fn sub_25() {
    let (out, diags) = interpret("fn main() print(350 - 26) end", 0);
    assert!(!diags.has_errors(), "sub_25: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"324
");
}

#[test]
fn sub_26() {
    let (out, diags) = interpret("fn main() print(360 - 27) end", 0);
    assert!(!diags.has_errors(), "sub_26: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"333
");
}

#[test]
fn sub_27() {
    let (out, diags) = interpret("fn main() print(370 - 28) end", 0);
    assert!(!diags.has_errors(), "sub_27: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"342
");
}

#[test]
fn sub_28() {
    let (out, diags) = interpret("fn main() print(380 - 29) end", 0);
    assert!(!diags.has_errors(), "sub_28: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"351
");
}

#[test]
fn sub_29() {
    let (out, diags) = interpret("fn main() print(390 - 30) end", 0);
    assert!(!diags.has_errors(), "sub_29: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"360
");
}

#[test]
fn sub_30() {
    let (out, diags) = interpret("fn main() print(400 - 31) end", 0);
    assert!(!diags.has_errors(), "sub_30: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"369
");
}

#[test]
fn sub_31() {
    let (out, diags) = interpret("fn main() print(410 - 32) end", 0);
    assert!(!diags.has_errors(), "sub_31: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"378
");
}

#[test]
fn sub_32() {
    let (out, diags) = interpret("fn main() print(420 - 33) end", 0);
    assert!(!diags.has_errors(), "sub_32: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"387
");
}

#[test]
fn sub_33() {
    let (out, diags) = interpret("fn main() print(430 - 34) end", 0);
    assert!(!diags.has_errors(), "sub_33: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"396
");
}

#[test]
fn sub_34() {
    let (out, diags) = interpret("fn main() print(440 - 35) end", 0);
    assert!(!diags.has_errors(), "sub_34: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"405
");
}

#[test]
fn sub_35() {
    let (out, diags) = interpret("fn main() print(450 - 36) end", 0);
    assert!(!diags.has_errors(), "sub_35: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"414
");
}

#[test]
fn sub_36() {
    let (out, diags) = interpret("fn main() print(460 - 37) end", 0);
    assert!(!diags.has_errors(), "sub_36: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"423
");
}

#[test]
fn sub_37() {
    let (out, diags) = interpret("fn main() print(470 - 38) end", 0);
    assert!(!diags.has_errors(), "sub_37: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"432
");
}

#[test]
fn sub_38() {
    let (out, diags) = interpret("fn main() print(480 - 39) end", 0);
    assert!(!diags.has_errors(), "sub_38: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"441
");
}

#[test]
fn sub_39() {
    let (out, diags) = interpret("fn main() print(490 - 40) end", 0);
    assert!(!diags.has_errors(), "sub_39: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"450
");
}

#[test]
fn sub_40() {
    let (out, diags) = interpret("fn main() print(500 - 41) end", 0);
    assert!(!diags.has_errors(), "sub_40: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"459
");
}

#[test]
fn sub_41() {
    let (out, diags) = interpret("fn main() print(510 - 42) end", 0);
    assert!(!diags.has_errors(), "sub_41: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"468
");
}

#[test]
fn sub_42() {
    let (out, diags) = interpret("fn main() print(520 - 43) end", 0);
    assert!(!diags.has_errors(), "sub_42: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"477
");
}

#[test]
fn sub_43() {
    let (out, diags) = interpret("fn main() print(530 - 44) end", 0);
    assert!(!diags.has_errors(), "sub_43: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"486
");
}

#[test]
fn sub_44() {
    let (out, diags) = interpret("fn main() print(540 - 45) end", 0);
    assert!(!diags.has_errors(), "sub_44: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"495
");
}

#[test]
fn sub_45() {
    let (out, diags) = interpret("fn main() print(550 - 46) end", 0);
    assert!(!diags.has_errors(), "sub_45: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"504
");
}

#[test]
fn sub_46() {
    let (out, diags) = interpret("fn main() print(560 - 47) end", 0);
    assert!(!diags.has_errors(), "sub_46: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"513
");
}

#[test]
fn sub_47() {
    let (out, diags) = interpret("fn main() print(570 - 48) end", 0);
    assert!(!diags.has_errors(), "sub_47: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"522
");
}

#[test]
fn sub_48() {
    let (out, diags) = interpret("fn main() print(580 - 49) end", 0);
    assert!(!diags.has_errors(), "sub_48: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"531
");
}

#[test]
fn sub_49() {
    let (out, diags) = interpret("fn main() print(590 - 50) end", 0);
    assert!(!diags.has_errors(), "sub_49: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"540
");
}

#[test]
fn sub_50() {
    let (out, diags) = interpret("fn main() print(600 - 51) end", 0);
    assert!(!diags.has_errors(), "sub_50: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"549
");
}

#[test]
fn sub_51() {
    let (out, diags) = interpret("fn main() print(610 - 52) end", 0);
    assert!(!diags.has_errors(), "sub_51: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"558
");
}

#[test]
fn sub_52() {
    let (out, diags) = interpret("fn main() print(620 - 53) end", 0);
    assert!(!diags.has_errors(), "sub_52: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"567
");
}

#[test]
fn sub_53() {
    let (out, diags) = interpret("fn main() print(630 - 54) end", 0);
    assert!(!diags.has_errors(), "sub_53: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"576
");
}

#[test]
fn sub_54() {
    let (out, diags) = interpret("fn main() print(640 - 55) end", 0);
    assert!(!diags.has_errors(), "sub_54: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"585
");
}

#[test]
fn sub_55() {
    let (out, diags) = interpret("fn main() print(650 - 56) end", 0);
    assert!(!diags.has_errors(), "sub_55: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"594
");
}

#[test]
fn sub_56() {
    let (out, diags) = interpret("fn main() print(660 - 57) end", 0);
    assert!(!diags.has_errors(), "sub_56: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"603
");
}

#[test]
fn sub_57() {
    let (out, diags) = interpret("fn main() print(670 - 58) end", 0);
    assert!(!diags.has_errors(), "sub_57: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"612
");
}

#[test]
fn sub_58() {
    let (out, diags) = interpret("fn main() print(680 - 59) end", 0);
    assert!(!diags.has_errors(), "sub_58: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"621
");
}

#[test]
fn sub_59() {
    let (out, diags) = interpret("fn main() print(690 - 60) end", 0);
    assert!(!diags.has_errors(), "sub_59: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"630
");
}

#[test]
fn sub_60() {
    let (out, diags) = interpret("fn main() print(700 - 61) end", 0);
    assert!(!diags.has_errors(), "sub_60: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"639
");
}

#[test]
fn sub_61() {
    let (out, diags) = interpret("fn main() print(710 - 62) end", 0);
    assert!(!diags.has_errors(), "sub_61: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"648
");
}

#[test]
fn sub_62() {
    let (out, diags) = interpret("fn main() print(720 - 63) end", 0);
    assert!(!diags.has_errors(), "sub_62: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"657
");
}

#[test]
fn sub_63() {
    let (out, diags) = interpret("fn main() print(730 - 64) end", 0);
    assert!(!diags.has_errors(), "sub_63: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"666
");
}

#[test]
fn sub_64() {
    let (out, diags) = interpret("fn main() print(740 - 65) end", 0);
    assert!(!diags.has_errors(), "sub_64: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"675
");
}

#[test]
fn sub_65() {
    let (out, diags) = interpret("fn main() print(750 - 66) end", 0);
    assert!(!diags.has_errors(), "sub_65: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"684
");
}

#[test]
fn sub_66() {
    let (out, diags) = interpret("fn main() print(760 - 67) end", 0);
    assert!(!diags.has_errors(), "sub_66: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"693
");
}

#[test]
fn sub_67() {
    let (out, diags) = interpret("fn main() print(770 - 68) end", 0);
    assert!(!diags.has_errors(), "sub_67: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"702
");
}

#[test]
fn sub_68() {
    let (out, diags) = interpret("fn main() print(780 - 69) end", 0);
    assert!(!diags.has_errors(), "sub_68: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"711
");
}

#[test]
fn sub_69() {
    let (out, diags) = interpret("fn main() print(790 - 70) end", 0);
    assert!(!diags.has_errors(), "sub_69: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"720
");
}

#[test]
fn sub_70() {
    let (out, diags) = interpret("fn main() print(800 - 71) end", 0);
    assert!(!diags.has_errors(), "sub_70: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"729
");
}

#[test]
fn sub_71() {
    let (out, diags) = interpret("fn main() print(810 - 72) end", 0);
    assert!(!diags.has_errors(), "sub_71: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"738
");
}

#[test]
fn sub_72() {
    let (out, diags) = interpret("fn main() print(820 - 73) end", 0);
    assert!(!diags.has_errors(), "sub_72: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"747
");
}

#[test]
fn sub_73() {
    let (out, diags) = interpret("fn main() print(830 - 74) end", 0);
    assert!(!diags.has_errors(), "sub_73: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"756
");
}

#[test]
fn sub_74() {
    let (out, diags) = interpret("fn main() print(840 - 75) end", 0);
    assert!(!diags.has_errors(), "sub_74: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"765
");
}

#[test]
fn sub_75() {
    let (out, diags) = interpret("fn main() print(850 - 76) end", 0);
    assert!(!diags.has_errors(), "sub_75: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"774
");
}

#[test]
fn sub_76() {
    let (out, diags) = interpret("fn main() print(860 - 77) end", 0);
    assert!(!diags.has_errors(), "sub_76: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"783
");
}

#[test]
fn sub_77() {
    let (out, diags) = interpret("fn main() print(870 - 78) end", 0);
    assert!(!diags.has_errors(), "sub_77: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"792
");
}

#[test]
fn sub_78() {
    let (out, diags) = interpret("fn main() print(880 - 79) end", 0);
    assert!(!diags.has_errors(), "sub_78: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"801
");
}

#[test]
fn sub_79() {
    let (out, diags) = interpret("fn main() print(890 - 80) end", 0);
    assert!(!diags.has_errors(), "sub_79: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"810
");
}

#[test]
fn sub_80() {
    let (out, diags) = interpret("fn main() print(900 - 81) end", 0);
    assert!(!diags.has_errors(), "sub_80: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"819
");
}

#[test]
fn sub_81() {
    let (out, diags) = interpret("fn main() print(910 - 82) end", 0);
    assert!(!diags.has_errors(), "sub_81: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"828
");
}

#[test]
fn sub_82() {
    let (out, diags) = interpret("fn main() print(920 - 83) end", 0);
    assert!(!diags.has_errors(), "sub_82: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"837
");
}

#[test]
fn sub_83() {
    let (out, diags) = interpret("fn main() print(930 - 84) end", 0);
    assert!(!diags.has_errors(), "sub_83: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"846
");
}

#[test]
fn sub_84() {
    let (out, diags) = interpret("fn main() print(940 - 85) end", 0);
    assert!(!diags.has_errors(), "sub_84: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"855
");
}

#[test]
fn sub_85() {
    let (out, diags) = interpret("fn main() print(950 - 86) end", 0);
    assert!(!diags.has_errors(), "sub_85: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"864
");
}

#[test]
fn sub_86() {
    let (out, diags) = interpret("fn main() print(960 - 87) end", 0);
    assert!(!diags.has_errors(), "sub_86: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"873
");
}

#[test]
fn sub_87() {
    let (out, diags) = interpret("fn main() print(970 - 88) end", 0);
    assert!(!diags.has_errors(), "sub_87: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"882
");
}

#[test]
fn sub_88() {
    let (out, diags) = interpret("fn main() print(980 - 89) end", 0);
    assert!(!diags.has_errors(), "sub_88: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"891
");
}

#[test]
fn sub_89() {
    let (out, diags) = interpret("fn main() print(990 - 90) end", 0);
    assert!(!diags.has_errors(), "sub_89: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"900
");
}

#[test]
fn sub_90() {
    let (out, diags) = interpret("fn main() print(1000 - 91) end", 0);
    assert!(!diags.has_errors(), "sub_90: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"909
");
}

#[test]
fn sub_91() {
    let (out, diags) = interpret("fn main() print(1010 - 92) end", 0);
    assert!(!diags.has_errors(), "sub_91: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"918
");
}

#[test]
fn sub_92() {
    let (out, diags) = interpret("fn main() print(1020 - 93) end", 0);
    assert!(!diags.has_errors(), "sub_92: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"927
");
}

#[test]
fn sub_93() {
    let (out, diags) = interpret("fn main() print(1030 - 94) end", 0);
    assert!(!diags.has_errors(), "sub_93: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"936
");
}

#[test]
fn sub_94() {
    let (out, diags) = interpret("fn main() print(1040 - 95) end", 0);
    assert!(!diags.has_errors(), "sub_94: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"945
");
}

#[test]
fn sub_95() {
    let (out, diags) = interpret("fn main() print(1050 - 96) end", 0);
    assert!(!diags.has_errors(), "sub_95: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"954
");
}

#[test]
fn sub_96() {
    let (out, diags) = interpret("fn main() print(1060 - 97) end", 0);
    assert!(!diags.has_errors(), "sub_96: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"963
");
}

#[test]
fn sub_97() {
    let (out, diags) = interpret("fn main() print(1070 - 98) end", 0);
    assert!(!diags.has_errors(), "sub_97: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"972
");
}

#[test]
fn sub_98() {
    let (out, diags) = interpret("fn main() print(1080 - 99) end", 0);
    assert!(!diags.has_errors(), "sub_98: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"981
");
}

#[test]
fn sub_99() {
    let (out, diags) = interpret("fn main() print(1090 - 100) end", 0);
    assert!(!diags.has_errors(), "sub_99: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"990
");
}

#[test]
fn mul_0() {
    let (out, diags) = interpret("fn main() print(1 * 2) end", 0);
    assert!(!diags.has_errors(), "mul_0: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"2
");
}

#[test]
fn mul_1() {
    let (out, diags) = interpret("fn main() print(2 * 3) end", 0);
    assert!(!diags.has_errors(), "mul_1: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"6
");
}

#[test]
fn mul_2() {
    let (out, diags) = interpret("fn main() print(3 * 4) end", 0);
    assert!(!diags.has_errors(), "mul_2: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"12
");
}

#[test]
fn mul_3() {
    let (out, diags) = interpret("fn main() print(4 * 5) end", 0);
    assert!(!diags.has_errors(), "mul_3: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"20
");
}

#[test]
fn mul_4() {
    let (out, diags) = interpret("fn main() print(5 * 6) end", 0);
    assert!(!diags.has_errors(), "mul_4: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"30
");
}

#[test]
fn mul_5() {
    let (out, diags) = interpret("fn main() print(6 * 7) end", 0);
    assert!(!diags.has_errors(), "mul_5: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"42
");
}

#[test]
fn mul_6() {
    let (out, diags) = interpret("fn main() print(7 * 8) end", 0);
    assert!(!diags.has_errors(), "mul_6: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"56
");
}

#[test]
fn mul_7() {
    let (out, diags) = interpret("fn main() print(8 * 9) end", 0);
    assert!(!diags.has_errors(), "mul_7: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"72
");
}

#[test]
fn mul_8() {
    let (out, diags) = interpret("fn main() print(9 * 10) end", 0);
    assert!(!diags.has_errors(), "mul_8: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"90
");
}

#[test]
fn mul_9() {
    let (out, diags) = interpret("fn main() print(10 * 11) end", 0);
    assert!(!diags.has_errors(), "mul_9: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"110
");
}

#[test]
fn mul_10() {
    let (out, diags) = interpret("fn main() print(11 * 12) end", 0);
    assert!(!diags.has_errors(), "mul_10: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"132
");
}

#[test]
fn mul_11() {
    let (out, diags) = interpret("fn main() print(12 * 13) end", 0);
    assert!(!diags.has_errors(), "mul_11: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"156
");
}

#[test]
fn mul_12() {
    let (out, diags) = interpret("fn main() print(13 * 14) end", 0);
    assert!(!diags.has_errors(), "mul_12: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"182
");
}

#[test]
fn mul_13() {
    let (out, diags) = interpret("fn main() print(14 * 15) end", 0);
    assert!(!diags.has_errors(), "mul_13: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"210
");
}

#[test]
fn mul_14() {
    let (out, diags) = interpret("fn main() print(15 * 16) end", 0);
    assert!(!diags.has_errors(), "mul_14: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"240
");
}

#[test]
fn mul_15() {
    let (out, diags) = interpret("fn main() print(16 * 17) end", 0);
    assert!(!diags.has_errors(), "mul_15: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"272
");
}

#[test]
fn mul_16() {
    let (out, diags) = interpret("fn main() print(17 * 18) end", 0);
    assert!(!diags.has_errors(), "mul_16: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"306
");
}

#[test]
fn mul_17() {
    let (out, diags) = interpret("fn main() print(18 * 19) end", 0);
    assert!(!diags.has_errors(), "mul_17: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"342
");
}

#[test]
fn mul_18() {
    let (out, diags) = interpret("fn main() print(19 * 20) end", 0);
    assert!(!diags.has_errors(), "mul_18: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"380
");
}

#[test]
fn mul_19() {
    let (out, diags) = interpret("fn main() print(20 * 21) end", 0);
    assert!(!diags.has_errors(), "mul_19: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"420
");
}

#[test]
fn mul_20() {
    let (out, diags) = interpret("fn main() print(21 * 22) end", 0);
    assert!(!diags.has_errors(), "mul_20: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"462
");
}

#[test]
fn mul_21() {
    let (out, diags) = interpret("fn main() print(22 * 23) end", 0);
    assert!(!diags.has_errors(), "mul_21: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"506
");
}

#[test]
fn mul_22() {
    let (out, diags) = interpret("fn main() print(23 * 24) end", 0);
    assert!(!diags.has_errors(), "mul_22: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"552
");
}

#[test]
fn mul_23() {
    let (out, diags) = interpret("fn main() print(24 * 25) end", 0);
    assert!(!diags.has_errors(), "mul_23: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"600
");
}

#[test]
fn mul_24() {
    let (out, diags) = interpret("fn main() print(25 * 26) end", 0);
    assert!(!diags.has_errors(), "mul_24: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"650
");
}

#[test]
fn mul_25() {
    let (out, diags) = interpret("fn main() print(26 * 27) end", 0);
    assert!(!diags.has_errors(), "mul_25: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"702
");
}

#[test]
fn mul_26() {
    let (out, diags) = interpret("fn main() print(27 * 28) end", 0);
    assert!(!diags.has_errors(), "mul_26: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"756
");
}

#[test]
fn mul_27() {
    let (out, diags) = interpret("fn main() print(28 * 29) end", 0);
    assert!(!diags.has_errors(), "mul_27: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"812
");
}

#[test]
fn mul_28() {
    let (out, diags) = interpret("fn main() print(29 * 30) end", 0);
    assert!(!diags.has_errors(), "mul_28: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"870
");
}

#[test]
fn mul_29() {
    let (out, diags) = interpret("fn main() print(30 * 31) end", 0);
    assert!(!diags.has_errors(), "mul_29: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"930
");
}

#[test]
fn mul_30() {
    let (out, diags) = interpret("fn main() print(31 * 32) end", 0);
    assert!(!diags.has_errors(), "mul_30: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"992
");
}

#[test]
fn mul_31() {
    let (out, diags) = interpret("fn main() print(32 * 33) end", 0);
    assert!(!diags.has_errors(), "mul_31: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"1056
");
}

#[test]
fn mul_32() {
    let (out, diags) = interpret("fn main() print(33 * 34) end", 0);
    assert!(!diags.has_errors(), "mul_32: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"1122
");
}

#[test]
fn mul_33() {
    let (out, diags) = interpret("fn main() print(34 * 35) end", 0);
    assert!(!diags.has_errors(), "mul_33: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"1190
");
}

#[test]
fn mul_34() {
    let (out, diags) = interpret("fn main() print(35 * 36) end", 0);
    assert!(!diags.has_errors(), "mul_34: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"1260
");
}

#[test]
fn mul_35() {
    let (out, diags) = interpret("fn main() print(36 * 37) end", 0);
    assert!(!diags.has_errors(), "mul_35: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"1332
");
}

#[test]
fn mul_36() {
    let (out, diags) = interpret("fn main() print(37 * 38) end", 0);
    assert!(!diags.has_errors(), "mul_36: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"1406
");
}

#[test]
fn mul_37() {
    let (out, diags) = interpret("fn main() print(38 * 39) end", 0);
    assert!(!diags.has_errors(), "mul_37: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"1482
");
}

#[test]
fn mul_38() {
    let (out, diags) = interpret("fn main() print(39 * 40) end", 0);
    assert!(!diags.has_errors(), "mul_38: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"1560
");
}

#[test]
fn mul_39() {
    let (out, diags) = interpret("fn main() print(40 * 41) end", 0);
    assert!(!diags.has_errors(), "mul_39: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"1640
");
}

#[test]
fn mul_40() {
    let (out, diags) = interpret("fn main() print(41 * 42) end", 0);
    assert!(!diags.has_errors(), "mul_40: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"1722
");
}

#[test]
fn mul_41() {
    let (out, diags) = interpret("fn main() print(42 * 43) end", 0);
    assert!(!diags.has_errors(), "mul_41: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"1806
");
}

#[test]
fn mul_42() {
    let (out, diags) = interpret("fn main() print(43 * 44) end", 0);
    assert!(!diags.has_errors(), "mul_42: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"1892
");
}

#[test]
fn mul_43() {
    let (out, diags) = interpret("fn main() print(44 * 45) end", 0);
    assert!(!diags.has_errors(), "mul_43: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"1980
");
}

#[test]
fn mul_44() {
    let (out, diags) = interpret("fn main() print(45 * 46) end", 0);
    assert!(!diags.has_errors(), "mul_44: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"2070
");
}

#[test]
fn mul_45() {
    let (out, diags) = interpret("fn main() print(46 * 47) end", 0);
    assert!(!diags.has_errors(), "mul_45: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"2162
");
}

#[test]
fn mul_46() {
    let (out, diags) = interpret("fn main() print(47 * 48) end", 0);
    assert!(!diags.has_errors(), "mul_46: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"2256
");
}

#[test]
fn mul_47() {
    let (out, diags) = interpret("fn main() print(48 * 49) end", 0);
    assert!(!diags.has_errors(), "mul_47: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"2352
");
}

#[test]
fn mul_48() {
    let (out, diags) = interpret("fn main() print(49 * 50) end", 0);
    assert!(!diags.has_errors(), "mul_48: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"2450
");
}

#[test]
fn mul_49() {
    let (out, diags) = interpret("fn main() print(50 * 51) end", 0);
    assert!(!diags.has_errors(), "mul_49: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"2550
");
}

#[test]
fn mul_50() {
    let (out, diags) = interpret("fn main() print(51 * 52) end", 0);
    assert!(!diags.has_errors(), "mul_50: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"2652
");
}

#[test]
fn mul_51() {
    let (out, diags) = interpret("fn main() print(52 * 53) end", 0);
    assert!(!diags.has_errors(), "mul_51: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"2756
");
}

#[test]
fn mul_52() {
    let (out, diags) = interpret("fn main() print(53 * 54) end", 0);
    assert!(!diags.has_errors(), "mul_52: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"2862
");
}

#[test]
fn mul_53() {
    let (out, diags) = interpret("fn main() print(54 * 55) end", 0);
    assert!(!diags.has_errors(), "mul_53: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"2970
");
}

#[test]
fn mul_54() {
    let (out, diags) = interpret("fn main() print(55 * 56) end", 0);
    assert!(!diags.has_errors(), "mul_54: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"3080
");
}

#[test]
fn mul_55() {
    let (out, diags) = interpret("fn main() print(56 * 57) end", 0);
    assert!(!diags.has_errors(), "mul_55: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"3192
");
}

#[test]
fn mul_56() {
    let (out, diags) = interpret("fn main() print(57 * 58) end", 0);
    assert!(!diags.has_errors(), "mul_56: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"3306
");
}

#[test]
fn mul_57() {
    let (out, diags) = interpret("fn main() print(58 * 59) end", 0);
    assert!(!diags.has_errors(), "mul_57: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"3422
");
}

#[test]
fn mul_58() {
    let (out, diags) = interpret("fn main() print(59 * 60) end", 0);
    assert!(!diags.has_errors(), "mul_58: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"3540
");
}

#[test]
fn mul_59() {
    let (out, diags) = interpret("fn main() print(60 * 61) end", 0);
    assert!(!diags.has_errors(), "mul_59: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"3660
");
}

#[test]
fn mul_60() {
    let (out, diags) = interpret("fn main() print(61 * 62) end", 0);
    assert!(!diags.has_errors(), "mul_60: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"3782
");
}

#[test]
fn mul_61() {
    let (out, diags) = interpret("fn main() print(62 * 63) end", 0);
    assert!(!diags.has_errors(), "mul_61: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"3906
");
}

#[test]
fn mul_62() {
    let (out, diags) = interpret("fn main() print(63 * 64) end", 0);
    assert!(!diags.has_errors(), "mul_62: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"4032
");
}

#[test]
fn mul_63() {
    let (out, diags) = interpret("fn main() print(64 * 65) end", 0);
    assert!(!diags.has_errors(), "mul_63: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"4160
");
}

#[test]
fn mul_64() {
    let (out, diags) = interpret("fn main() print(65 * 66) end", 0);
    assert!(!diags.has_errors(), "mul_64: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"4290
");
}

#[test]
fn mul_65() {
    let (out, diags) = interpret("fn main() print(66 * 67) end", 0);
    assert!(!diags.has_errors(), "mul_65: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"4422
");
}

#[test]
fn mul_66() {
    let (out, diags) = interpret("fn main() print(67 * 68) end", 0);
    assert!(!diags.has_errors(), "mul_66: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"4556
");
}

#[test]
fn mul_67() {
    let (out, diags) = interpret("fn main() print(68 * 69) end", 0);
    assert!(!diags.has_errors(), "mul_67: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"4692
");
}

#[test]
fn mul_68() {
    let (out, diags) = interpret("fn main() print(69 * 70) end", 0);
    assert!(!diags.has_errors(), "mul_68: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"4830
");
}

#[test]
fn mul_69() {
    let (out, diags) = interpret("fn main() print(70 * 71) end", 0);
    assert!(!diags.has_errors(), "mul_69: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"4970
");
}

#[test]
fn mul_70() {
    let (out, diags) = interpret("fn main() print(71 * 72) end", 0);
    assert!(!diags.has_errors(), "mul_70: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"5112
");
}

#[test]
fn mul_71() {
    let (out, diags) = interpret("fn main() print(72 * 73) end", 0);
    assert!(!diags.has_errors(), "mul_71: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"5256
");
}

#[test]
fn mul_72() {
    let (out, diags) = interpret("fn main() print(73 * 74) end", 0);
    assert!(!diags.has_errors(), "mul_72: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"5402
");
}

#[test]
fn mul_73() {
    let (out, diags) = interpret("fn main() print(74 * 75) end", 0);
    assert!(!diags.has_errors(), "mul_73: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"5550
");
}

#[test]
fn mul_74() {
    let (out, diags) = interpret("fn main() print(75 * 76) end", 0);
    assert!(!diags.has_errors(), "mul_74: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"5700
");
}

#[test]
fn mul_75() {
    let (out, diags) = interpret("fn main() print(76 * 77) end", 0);
    assert!(!diags.has_errors(), "mul_75: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"5852
");
}

#[test]
fn mul_76() {
    let (out, diags) = interpret("fn main() print(77 * 78) end", 0);
    assert!(!diags.has_errors(), "mul_76: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"6006
");
}

#[test]
fn mul_77() {
    let (out, diags) = interpret("fn main() print(78 * 79) end", 0);
    assert!(!diags.has_errors(), "mul_77: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"6162
");
}

#[test]
fn mul_78() {
    let (out, diags) = interpret("fn main() print(79 * 80) end", 0);
    assert!(!diags.has_errors(), "mul_78: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"6320
");
}

#[test]
fn mul_79() {
    let (out, diags) = interpret("fn main() print(80 * 81) end", 0);
    assert!(!diags.has_errors(), "mul_79: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"6480
");
}

#[test]
fn mul_80() {
    let (out, diags) = interpret("fn main() print(81 * 82) end", 0);
    assert!(!diags.has_errors(), "mul_80: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"6642
");
}

#[test]
fn mul_81() {
    let (out, diags) = interpret("fn main() print(82 * 83) end", 0);
    assert!(!diags.has_errors(), "mul_81: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"6806
");
}

#[test]
fn mul_82() {
    let (out, diags) = interpret("fn main() print(83 * 84) end", 0);
    assert!(!diags.has_errors(), "mul_82: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"6972
");
}

#[test]
fn mul_83() {
    let (out, diags) = interpret("fn main() print(84 * 85) end", 0);
    assert!(!diags.has_errors(), "mul_83: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"7140
");
}

#[test]
fn mul_84() {
    let (out, diags) = interpret("fn main() print(85 * 86) end", 0);
    assert!(!diags.has_errors(), "mul_84: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"7310
");
}

#[test]
fn mul_85() {
    let (out, diags) = interpret("fn main() print(86 * 87) end", 0);
    assert!(!diags.has_errors(), "mul_85: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"7482
");
}

#[test]
fn mul_86() {
    let (out, diags) = interpret("fn main() print(87 * 88) end", 0);
    assert!(!diags.has_errors(), "mul_86: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"7656
");
}

#[test]
fn mul_87() {
    let (out, diags) = interpret("fn main() print(88 * 89) end", 0);
    assert!(!diags.has_errors(), "mul_87: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"7832
");
}

#[test]
fn mul_88() {
    let (out, diags) = interpret("fn main() print(89 * 90) end", 0);
    assert!(!diags.has_errors(), "mul_88: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"8010
");
}

#[test]
fn mul_89() {
    let (out, diags) = interpret("fn main() print(90 * 91) end", 0);
    assert!(!diags.has_errors(), "mul_89: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"8190
");
}

#[test]
fn mul_90() {
    let (out, diags) = interpret("fn main() print(91 * 92) end", 0);
    assert!(!diags.has_errors(), "mul_90: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"8372
");
}

#[test]
fn mul_91() {
    let (out, diags) = interpret("fn main() print(92 * 93) end", 0);
    assert!(!diags.has_errors(), "mul_91: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"8556
");
}

#[test]
fn mul_92() {
    let (out, diags) = interpret("fn main() print(93 * 94) end", 0);
    assert!(!diags.has_errors(), "mul_92: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"8742
");
}

#[test]
fn mul_93() {
    let (out, diags) = interpret("fn main() print(94 * 95) end", 0);
    assert!(!diags.has_errors(), "mul_93: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"8930
");
}

#[test]
fn mul_94() {
    let (out, diags) = interpret("fn main() print(95 * 96) end", 0);
    assert!(!diags.has_errors(), "mul_94: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"9120
");
}

#[test]
fn mul_95() {
    let (out, diags) = interpret("fn main() print(96 * 97) end", 0);
    assert!(!diags.has_errors(), "mul_95: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"9312
");
}

#[test]
fn mul_96() {
    let (out, diags) = interpret("fn main() print(97 * 98) end", 0);
    assert!(!diags.has_errors(), "mul_96: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"9506
");
}

#[test]
fn mul_97() {
    let (out, diags) = interpret("fn main() print(98 * 99) end", 0);
    assert!(!diags.has_errors(), "mul_97: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"9702
");
}

#[test]
fn mul_98() {
    let (out, diags) = interpret("fn main() print(99 * 100) end", 0);
    assert!(!diags.has_errors(), "mul_98: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"9900
");
}

#[test]
fn mul_99() {
    let (out, diags) = interpret("fn main() print(100 * 101) end", 0);
    assert!(!diags.has_errors(), "mul_99: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"10100
");
}

#[test]
fn div_0() {
    let (out, diags) = interpret("fn main() print(35 / 1) end", 0);
    assert!(!diags.has_errors(), "div_0: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"35
");
}

#[test]
fn div_1() {
    let (out, diags) = interpret("fn main() print(42 / 2) end", 0);
    assert!(!diags.has_errors(), "div_1: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"21
");
}

#[test]
fn div_3() {
    let (out, diags) = interpret("fn main() print(56 / 4) end", 0);
    assert!(!diags.has_errors(), "div_3: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"14
");
}

#[test]
fn div_6() {
    let (out, diags) = interpret("fn main() print(77 / 7) end", 0);
    assert!(!diags.has_errors(), "div_6: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"11
");
}

#[test]
fn div_13() {
    let (out, diags) = interpret("fn main() print(126 / 14) end", 0);
    assert!(!diags.has_errors(), "div_13: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"9
");
}

#[test]
fn div_27() {
    let (out, diags) = interpret("fn main() print(224 / 28) end", 0);
    assert!(!diags.has_errors(), "div_27: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"8
");
}

#[test]
fn mod_0() {
    let (out, diags) = interpret("fn main() print(7 % 5) end", 0);
    assert!(!diags.has_errors(), "mod_0: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"2
");
}

#[test]
fn mod_1() {
    let (out, diags) = interpret("fn main() print(20 % 6) end", 0);
    assert!(!diags.has_errors(), "mod_1: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"2
");
}

#[test]
fn mod_2() {
    let (out, diags) = interpret("fn main() print(33 % 7) end", 0);
    assert!(!diags.has_errors(), "mod_2: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"5
");
}

#[test]
fn mod_3() {
    let (out, diags) = interpret("fn main() print(46 % 8) end", 0);
    assert!(!diags.has_errors(), "mod_3: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"6
");
}

#[test]
fn mod_4() {
    let (out, diags) = interpret("fn main() print(59 % 9) end", 0);
    assert!(!diags.has_errors(), "mod_4: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"5
");
}

#[test]
fn mod_5() {
    let (out, diags) = interpret("fn main() print(72 % 10) end", 0);
    assert!(!diags.has_errors(), "mod_5: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"2
");
}

#[test]
fn mod_6() {
    let (out, diags) = interpret("fn main() print(85 % 11) end", 0);
    assert!(!diags.has_errors(), "mod_6: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"8
");
}

#[test]
fn mod_7() {
    let (out, diags) = interpret("fn main() print(98 % 12) end", 0);
    assert!(!diags.has_errors(), "mod_7: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"2
");
}

#[test]
fn mod_8() {
    let (out, diags) = interpret("fn main() print(111 % 13) end", 0);
    assert!(!diags.has_errors(), "mod_8: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"7
");
}

#[test]
fn mod_9() {
    let (out, diags) = interpret("fn main() print(124 % 14) end", 0);
    assert!(!diags.has_errors(), "mod_9: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"12
");
}

#[test]
fn mod_10() {
    let (out, diags) = interpret("fn main() print(137 % 15) end", 0);
    assert!(!diags.has_errors(), "mod_10: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"2
");
}

#[test]
fn mod_11() {
    let (out, diags) = interpret("fn main() print(150 % 16) end", 0);
    assert!(!diags.has_errors(), "mod_11: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"6
");
}

#[test]
fn mod_12() {
    let (out, diags) = interpret("fn main() print(163 % 17) end", 0);
    assert!(!diags.has_errors(), "mod_12: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"10
");
}

#[test]
fn mod_13() {
    let (out, diags) = interpret("fn main() print(176 % 18) end", 0);
    assert!(!diags.has_errors(), "mod_13: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"14
");
}

#[test]
fn mod_14() {
    let (out, diags) = interpret("fn main() print(189 % 19) end", 0);
    assert!(!diags.has_errors(), "mod_14: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"18
");
}

#[test]
fn mod_15() {
    let (out, diags) = interpret("fn main() print(202 % 20) end", 0);
    assert!(!diags.has_errors(), "mod_15: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"2
");
}

#[test]
fn mod_16() {
    let (out, diags) = interpret("fn main() print(215 % 21) end", 0);
    assert!(!diags.has_errors(), "mod_16: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"5
");
}

#[test]
fn mod_17() {
    let (out, diags) = interpret("fn main() print(228 % 22) end", 0);
    assert!(!diags.has_errors(), "mod_17: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"8
");
}

#[test]
fn mod_18() {
    let (out, diags) = interpret("fn main() print(241 % 23) end", 0);
    assert!(!diags.has_errors(), "mod_18: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"11
");
}

#[test]
fn mod_19() {
    let (out, diags) = interpret("fn main() print(254 % 24) end", 0);
    assert!(!diags.has_errors(), "mod_19: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"14
");
}

#[test]
fn mod_20() {
    let (out, diags) = interpret("fn main() print(267 % 25) end", 0);
    assert!(!diags.has_errors(), "mod_20: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"17
");
}

#[test]
fn mod_21() {
    let (out, diags) = interpret("fn main() print(280 % 26) end", 0);
    assert!(!diags.has_errors(), "mod_21: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"20
");
}

#[test]
fn mod_22() {
    let (out, diags) = interpret("fn main() print(293 % 27) end", 0);
    assert!(!diags.has_errors(), "mod_22: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"23
");
}

#[test]
fn mod_23() {
    let (out, diags) = interpret("fn main() print(306 % 28) end", 0);
    assert!(!diags.has_errors(), "mod_23: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"26
");
}

#[test]
fn mod_24() {
    let (out, diags) = interpret("fn main() print(319 % 29) end", 0);
    assert!(!diags.has_errors(), "mod_24: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"0
");
}

#[test]
fn mod_25() {
    let (out, diags) = interpret("fn main() print(332 % 30) end", 0);
    assert!(!diags.has_errors(), "mod_25: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"2
");
}

#[test]
fn mod_26() {
    let (out, diags) = interpret("fn main() print(345 % 31) end", 0);
    assert!(!diags.has_errors(), "mod_26: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"4
");
}

#[test]
fn mod_27() {
    let (out, diags) = interpret("fn main() print(358 % 32) end", 0);
    assert!(!diags.has_errors(), "mod_27: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"6
");
}

#[test]
fn mod_28() {
    let (out, diags) = interpret("fn main() print(371 % 33) end", 0);
    assert!(!diags.has_errors(), "mod_28: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"8
");
}

#[test]
fn mod_29() {
    let (out, diags) = interpret("fn main() print(384 % 34) end", 0);
    assert!(!diags.has_errors(), "mod_29: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"10
");
}

#[test]
fn mod_30() {
    let (out, diags) = interpret("fn main() print(397 % 35) end", 0);
    assert!(!diags.has_errors(), "mod_30: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"12
");
}

#[test]
fn mod_31() {
    let (out, diags) = interpret("fn main() print(410 % 36) end", 0);
    assert!(!diags.has_errors(), "mod_31: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"14
");
}

#[test]
fn mod_32() {
    let (out, diags) = interpret("fn main() print(423 % 37) end", 0);
    assert!(!diags.has_errors(), "mod_32: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"16
");
}

#[test]
fn mod_33() {
    let (out, diags) = interpret("fn main() print(436 % 38) end", 0);
    assert!(!diags.has_errors(), "mod_33: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"18
");
}

#[test]
fn mod_34() {
    let (out, diags) = interpret("fn main() print(449 % 39) end", 0);
    assert!(!diags.has_errors(), "mod_34: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"20
");
}

#[test]
fn mod_35() {
    let (out, diags) = interpret("fn main() print(462 % 40) end", 0);
    assert!(!diags.has_errors(), "mod_35: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"22
");
}

#[test]
fn mod_36() {
    let (out, diags) = interpret("fn main() print(475 % 41) end", 0);
    assert!(!diags.has_errors(), "mod_36: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"24
");
}

#[test]
fn mod_37() {
    let (out, diags) = interpret("fn main() print(488 % 42) end", 0);
    assert!(!diags.has_errors(), "mod_37: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"26
");
}

#[test]
fn mod_38() {
    let (out, diags) = interpret("fn main() print(501 % 43) end", 0);
    assert!(!diags.has_errors(), "mod_38: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"28
");
}

#[test]
fn mod_39() {
    let (out, diags) = interpret("fn main() print(514 % 44) end", 0);
    assert!(!diags.has_errors(), "mod_39: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"30
");
}

#[test]
fn mod_40() {
    let (out, diags) = interpret("fn main() print(527 % 45) end", 0);
    assert!(!diags.has_errors(), "mod_40: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"32
");
}

#[test]
fn mod_41() {
    let (out, diags) = interpret("fn main() print(540 % 46) end", 0);
    assert!(!diags.has_errors(), "mod_41: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"34
");
}

#[test]
fn mod_42() {
    let (out, diags) = interpret("fn main() print(553 % 47) end", 0);
    assert!(!diags.has_errors(), "mod_42: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"36
");
}

#[test]
fn mod_43() {
    let (out, diags) = interpret("fn main() print(566 % 48) end", 0);
    assert!(!diags.has_errors(), "mod_43: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"38
");
}

#[test]
fn mod_44() {
    let (out, diags) = interpret("fn main() print(579 % 49) end", 0);
    assert!(!diags.has_errors(), "mod_44: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"40
");
}

#[test]
fn mod_45() {
    let (out, diags) = interpret("fn main() print(592 % 50) end", 0);
    assert!(!diags.has_errors(), "mod_45: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"42
");
}

#[test]
fn mod_46() {
    let (out, diags) = interpret("fn main() print(605 % 51) end", 0);
    assert!(!diags.has_errors(), "mod_46: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"44
");
}

#[test]
fn mod_47() {
    let (out, diags) = interpret("fn main() print(618 % 52) end", 0);
    assert!(!diags.has_errors(), "mod_47: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"46
");
}

#[test]
fn mod_48() {
    let (out, diags) = interpret("fn main() print(631 % 53) end", 0);
    assert!(!diags.has_errors(), "mod_48: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"48
");
}

#[test]
fn mod_49() {
    let (out, diags) = interpret("fn main() print(644 % 54) end", 0);
    assert!(!diags.has_errors(), "mod_49: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"50
");
}

