// Owner: PascalElixir / axolrs (GitHub org)
// File: Auto-generated tests for tuples.

use axolc_core::interpret;

#[test]
fn tuple_lit_0() {
    let (out, diags) = interpret("fn main() let t = (0, 1) print(t[0]) end", 0);
    assert!(!diags.has_errors(), "tuple_lit_0: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"0
");
}

#[test]
fn tuple_lit_1() {
    let (out, diags) = interpret("fn main() let t = (1, 2) print(t[0]) end", 0);
    assert!(!diags.has_errors(), "tuple_lit_1: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"1
");
}

#[test]
fn tuple_lit_2() {
    let (out, diags) = interpret("fn main() let t = (2, 3) print(t[0]) end", 0);
    assert!(!diags.has_errors(), "tuple_lit_2: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"2
");
}

#[test]
fn tuple_lit_3() {
    let (out, diags) = interpret("fn main() let t = (3, 4) print(t[0]) end", 0);
    assert!(!diags.has_errors(), "tuple_lit_3: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"3
");
}

#[test]
fn tuple_lit_4() {
    let (out, diags) = interpret("fn main() let t = (4, 5) print(t[0]) end", 0);
    assert!(!diags.has_errors(), "tuple_lit_4: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"4
");
}

#[test]
fn tuple_lit_5() {
    let (out, diags) = interpret("fn main() let t = (5, 6) print(t[0]) end", 0);
    assert!(!diags.has_errors(), "tuple_lit_5: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"5
");
}

#[test]
fn tuple_lit_6() {
    let (out, diags) = interpret("fn main() let t = (6, 7) print(t[0]) end", 0);
    assert!(!diags.has_errors(), "tuple_lit_6: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"6
");
}

#[test]
fn tuple_lit_7() {
    let (out, diags) = interpret("fn main() let t = (7, 8) print(t[0]) end", 0);
    assert!(!diags.has_errors(), "tuple_lit_7: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"7
");
}

#[test]
fn tuple_lit_8() {
    let (out, diags) = interpret("fn main() let t = (8, 9) print(t[0]) end", 0);
    assert!(!diags.has_errors(), "tuple_lit_8: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"8
");
}

#[test]
fn tuple_lit_9() {
    let (out, diags) = interpret("fn main() let t = (9, 10) print(t[0]) end", 0);
    assert!(!diags.has_errors(), "tuple_lit_9: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"9
");
}

#[test]
fn tuple_lit_10() {
    let (out, diags) = interpret("fn main() let t = (10, 11) print(t[0]) end", 0);
    assert!(!diags.has_errors(), "tuple_lit_10: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"10
");
}

#[test]
fn tuple_lit_11() {
    let (out, diags) = interpret("fn main() let t = (11, 12) print(t[0]) end", 0);
    assert!(!diags.has_errors(), "tuple_lit_11: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"11
");
}

#[test]
fn tuple_lit_12() {
    let (out, diags) = interpret("fn main() let t = (12, 13) print(t[0]) end", 0);
    assert!(!diags.has_errors(), "tuple_lit_12: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"12
");
}

#[test]
fn tuple_lit_13() {
    let (out, diags) = interpret("fn main() let t = (13, 14) print(t[0]) end", 0);
    assert!(!diags.has_errors(), "tuple_lit_13: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"13
");
}

#[test]
fn tuple_lit_14() {
    let (out, diags) = interpret("fn main() let t = (14, 15) print(t[0]) end", 0);
    assert!(!diags.has_errors(), "tuple_lit_14: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"14
");
}

#[test]
fn tuple_lit_15() {
    let (out, diags) = interpret("fn main() let t = (15, 16) print(t[0]) end", 0);
    assert!(!diags.has_errors(), "tuple_lit_15: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"15
");
}

#[test]
fn tuple_lit_16() {
    let (out, diags) = interpret("fn main() let t = (16, 17) print(t[0]) end", 0);
    assert!(!diags.has_errors(), "tuple_lit_16: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"16
");
}

#[test]
fn tuple_lit_17() {
    let (out, diags) = interpret("fn main() let t = (17, 18) print(t[0]) end", 0);
    assert!(!diags.has_errors(), "tuple_lit_17: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"17
");
}

#[test]
fn tuple_lit_18() {
    let (out, diags) = interpret("fn main() let t = (18, 19) print(t[0]) end", 0);
    assert!(!diags.has_errors(), "tuple_lit_18: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"18
");
}

#[test]
fn tuple_lit_19() {
    let (out, diags) = interpret("fn main() let t = (19, 20) print(t[0]) end", 0);
    assert!(!diags.has_errors(), "tuple_lit_19: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"19
");
}

#[test]
fn tuple_lit_20() {
    let (out, diags) = interpret("fn main() let t = (20, 21) print(t[0]) end", 0);
    assert!(!diags.has_errors(), "tuple_lit_20: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"20
");
}

#[test]
fn tuple_lit_21() {
    let (out, diags) = interpret("fn main() let t = (21, 22) print(t[0]) end", 0);
    assert!(!diags.has_errors(), "tuple_lit_21: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"21
");
}

#[test]
fn tuple_lit_22() {
    let (out, diags) = interpret("fn main() let t = (22, 23) print(t[0]) end", 0);
    assert!(!diags.has_errors(), "tuple_lit_22: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"22
");
}

#[test]
fn tuple_lit_23() {
    let (out, diags) = interpret("fn main() let t = (23, 24) print(t[0]) end", 0);
    assert!(!diags.has_errors(), "tuple_lit_23: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"23
");
}

#[test]
fn tuple_lit_24() {
    let (out, diags) = interpret("fn main() let t = (24, 25) print(t[0]) end", 0);
    assert!(!diags.has_errors(), "tuple_lit_24: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"24
");
}

#[test]
fn tuple_lit_25() {
    let (out, diags) = interpret("fn main() let t = (25, 26) print(t[0]) end", 0);
    assert!(!diags.has_errors(), "tuple_lit_25: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"25
");
}

#[test]
fn tuple_lit_26() {
    let (out, diags) = interpret("fn main() let t = (26, 27) print(t[0]) end", 0);
    assert!(!diags.has_errors(), "tuple_lit_26: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"26
");
}

#[test]
fn tuple_lit_27() {
    let (out, diags) = interpret("fn main() let t = (27, 28) print(t[0]) end", 0);
    assert!(!diags.has_errors(), "tuple_lit_27: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"27
");
}

#[test]
fn tuple_lit_28() {
    let (out, diags) = interpret("fn main() let t = (28, 29) print(t[0]) end", 0);
    assert!(!diags.has_errors(), "tuple_lit_28: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"28
");
}

#[test]
fn tuple_lit_29() {
    let (out, diags) = interpret("fn main() let t = (29, 30) print(t[0]) end", 0);
    assert!(!diags.has_errors(), "tuple_lit_29: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"29
");
}

#[test]
fn tuple_lit_30() {
    let (out, diags) = interpret("fn main() let t = (30, 31) print(t[0]) end", 0);
    assert!(!diags.has_errors(), "tuple_lit_30: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"30
");
}

#[test]
fn tuple_lit_31() {
    let (out, diags) = interpret("fn main() let t = (31, 32) print(t[0]) end", 0);
    assert!(!diags.has_errors(), "tuple_lit_31: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"31
");
}

#[test]
fn tuple_lit_32() {
    let (out, diags) = interpret("fn main() let t = (32, 33) print(t[0]) end", 0);
    assert!(!diags.has_errors(), "tuple_lit_32: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"32
");
}

#[test]
fn tuple_lit_33() {
    let (out, diags) = interpret("fn main() let t = (33, 34) print(t[0]) end", 0);
    assert!(!diags.has_errors(), "tuple_lit_33: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"33
");
}

#[test]
fn tuple_lit_34() {
    let (out, diags) = interpret("fn main() let t = (34, 35) print(t[0]) end", 0);
    assert!(!diags.has_errors(), "tuple_lit_34: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"34
");
}

#[test]
fn tuple_lit_35() {
    let (out, diags) = interpret("fn main() let t = (35, 36) print(t[0]) end", 0);
    assert!(!diags.has_errors(), "tuple_lit_35: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"35
");
}

#[test]
fn tuple_lit_36() {
    let (out, diags) = interpret("fn main() let t = (36, 37) print(t[0]) end", 0);
    assert!(!diags.has_errors(), "tuple_lit_36: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"36
");
}

#[test]
fn tuple_lit_37() {
    let (out, diags) = interpret("fn main() let t = (37, 38) print(t[0]) end", 0);
    assert!(!diags.has_errors(), "tuple_lit_37: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"37
");
}

#[test]
fn tuple_lit_38() {
    let (out, diags) = interpret("fn main() let t = (38, 39) print(t[0]) end", 0);
    assert!(!diags.has_errors(), "tuple_lit_38: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"38
");
}

#[test]
fn tuple_lit_39() {
    let (out, diags) = interpret("fn main() let t = (39, 40) print(t[0]) end", 0);
    assert!(!diags.has_errors(), "tuple_lit_39: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"39
");
}

#[test]
fn tuple_lit_40() {
    let (out, diags) = interpret("fn main() let t = (40, 41) print(t[0]) end", 0);
    assert!(!diags.has_errors(), "tuple_lit_40: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"40
");
}

#[test]
fn tuple_lit_41() {
    let (out, diags) = interpret("fn main() let t = (41, 42) print(t[0]) end", 0);
    assert!(!diags.has_errors(), "tuple_lit_41: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"41
");
}

#[test]
fn tuple_lit_42() {
    let (out, diags) = interpret("fn main() let t = (42, 43) print(t[0]) end", 0);
    assert!(!diags.has_errors(), "tuple_lit_42: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"42
");
}

#[test]
fn tuple_lit_43() {
    let (out, diags) = interpret("fn main() let t = (43, 44) print(t[0]) end", 0);
    assert!(!diags.has_errors(), "tuple_lit_43: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"43
");
}

#[test]
fn tuple_lit_44() {
    let (out, diags) = interpret("fn main() let t = (44, 45) print(t[0]) end", 0);
    assert!(!diags.has_errors(), "tuple_lit_44: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"44
");
}

#[test]
fn tuple_lit_45() {
    let (out, diags) = interpret("fn main() let t = (45, 46) print(t[0]) end", 0);
    assert!(!diags.has_errors(), "tuple_lit_45: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"45
");
}

#[test]
fn tuple_lit_46() {
    let (out, diags) = interpret("fn main() let t = (46, 47) print(t[0]) end", 0);
    assert!(!diags.has_errors(), "tuple_lit_46: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"46
");
}

#[test]
fn tuple_lit_47() {
    let (out, diags) = interpret("fn main() let t = (47, 48) print(t[0]) end", 0);
    assert!(!diags.has_errors(), "tuple_lit_47: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"47
");
}

#[test]
fn tuple_lit_48() {
    let (out, diags) = interpret("fn main() let t = (48, 49) print(t[0]) end", 0);
    assert!(!diags.has_errors(), "tuple_lit_48: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"48
");
}

#[test]
fn tuple_lit_49() {
    let (out, diags) = interpret("fn main() let t = (49, 50) print(t[0]) end", 0);
    assert!(!diags.has_errors(), "tuple_lit_49: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"49
");
}

#[test]
fn tuple_index1_0() {
    let (out, diags) = interpret("fn main() let t = (0, 1) print(t[1]) end", 0);
    assert!(!diags.has_errors(), "tuple_index1_0: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"1
");
}

#[test]
fn tuple_index1_1() {
    let (out, diags) = interpret("fn main() let t = (1, 2) print(t[1]) end", 0);
    assert!(!diags.has_errors(), "tuple_index1_1: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"2
");
}

#[test]
fn tuple_index1_2() {
    let (out, diags) = interpret("fn main() let t = (2, 3) print(t[1]) end", 0);
    assert!(!diags.has_errors(), "tuple_index1_2: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"3
");
}

#[test]
fn tuple_index1_3() {
    let (out, diags) = interpret("fn main() let t = (3, 4) print(t[1]) end", 0);
    assert!(!diags.has_errors(), "tuple_index1_3: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"4
");
}

#[test]
fn tuple_index1_4() {
    let (out, diags) = interpret("fn main() let t = (4, 5) print(t[1]) end", 0);
    assert!(!diags.has_errors(), "tuple_index1_4: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"5
");
}

#[test]
fn tuple_index1_5() {
    let (out, diags) = interpret("fn main() let t = (5, 6) print(t[1]) end", 0);
    assert!(!diags.has_errors(), "tuple_index1_5: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"6
");
}

#[test]
fn tuple_index1_6() {
    let (out, diags) = interpret("fn main() let t = (6, 7) print(t[1]) end", 0);
    assert!(!diags.has_errors(), "tuple_index1_6: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"7
");
}

#[test]
fn tuple_index1_7() {
    let (out, diags) = interpret("fn main() let t = (7, 8) print(t[1]) end", 0);
    assert!(!diags.has_errors(), "tuple_index1_7: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"8
");
}

#[test]
fn tuple_index1_8() {
    let (out, diags) = interpret("fn main() let t = (8, 9) print(t[1]) end", 0);
    assert!(!diags.has_errors(), "tuple_index1_8: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"9
");
}

#[test]
fn tuple_index1_9() {
    let (out, diags) = interpret("fn main() let t = (9, 10) print(t[1]) end", 0);
    assert!(!diags.has_errors(), "tuple_index1_9: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"10
");
}

#[test]
fn tuple_index1_10() {
    let (out, diags) = interpret("fn main() let t = (10, 11) print(t[1]) end", 0);
    assert!(!diags.has_errors(), "tuple_index1_10: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"11
");
}

#[test]
fn tuple_index1_11() {
    let (out, diags) = interpret("fn main() let t = (11, 12) print(t[1]) end", 0);
    assert!(!diags.has_errors(), "tuple_index1_11: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"12
");
}

#[test]
fn tuple_index1_12() {
    let (out, diags) = interpret("fn main() let t = (12, 13) print(t[1]) end", 0);
    assert!(!diags.has_errors(), "tuple_index1_12: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"13
");
}

#[test]
fn tuple_index1_13() {
    let (out, diags) = interpret("fn main() let t = (13, 14) print(t[1]) end", 0);
    assert!(!diags.has_errors(), "tuple_index1_13: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"14
");
}

#[test]
fn tuple_index1_14() {
    let (out, diags) = interpret("fn main() let t = (14, 15) print(t[1]) end", 0);
    assert!(!diags.has_errors(), "tuple_index1_14: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"15
");
}

#[test]
fn tuple_index1_15() {
    let (out, diags) = interpret("fn main() let t = (15, 16) print(t[1]) end", 0);
    assert!(!diags.has_errors(), "tuple_index1_15: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"16
");
}

#[test]
fn tuple_index1_16() {
    let (out, diags) = interpret("fn main() let t = (16, 17) print(t[1]) end", 0);
    assert!(!diags.has_errors(), "tuple_index1_16: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"17
");
}

#[test]
fn tuple_index1_17() {
    let (out, diags) = interpret("fn main() let t = (17, 18) print(t[1]) end", 0);
    assert!(!diags.has_errors(), "tuple_index1_17: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"18
");
}

#[test]
fn tuple_index1_18() {
    let (out, diags) = interpret("fn main() let t = (18, 19) print(t[1]) end", 0);
    assert!(!diags.has_errors(), "tuple_index1_18: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"19
");
}

#[test]
fn tuple_index1_19() {
    let (out, diags) = interpret("fn main() let t = (19, 20) print(t[1]) end", 0);
    assert!(!diags.has_errors(), "tuple_index1_19: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"20
");
}

#[test]
fn tuple_index1_20() {
    let (out, diags) = interpret("fn main() let t = (20, 21) print(t[1]) end", 0);
    assert!(!diags.has_errors(), "tuple_index1_20: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"21
");
}

#[test]
fn tuple_index1_21() {
    let (out, diags) = interpret("fn main() let t = (21, 22) print(t[1]) end", 0);
    assert!(!diags.has_errors(), "tuple_index1_21: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"22
");
}

#[test]
fn tuple_index1_22() {
    let (out, diags) = interpret("fn main() let t = (22, 23) print(t[1]) end", 0);
    assert!(!diags.has_errors(), "tuple_index1_22: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"23
");
}

#[test]
fn tuple_index1_23() {
    let (out, diags) = interpret("fn main() let t = (23, 24) print(t[1]) end", 0);
    assert!(!diags.has_errors(), "tuple_index1_23: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"24
");
}

#[test]
fn tuple_index1_24() {
    let (out, diags) = interpret("fn main() let t = (24, 25) print(t[1]) end", 0);
    assert!(!diags.has_errors(), "tuple_index1_24: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"25
");
}

#[test]
fn tuple_index1_25() {
    let (out, diags) = interpret("fn main() let t = (25, 26) print(t[1]) end", 0);
    assert!(!diags.has_errors(), "tuple_index1_25: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"26
");
}

#[test]
fn tuple_index1_26() {
    let (out, diags) = interpret("fn main() let t = (26, 27) print(t[1]) end", 0);
    assert!(!diags.has_errors(), "tuple_index1_26: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"27
");
}

#[test]
fn tuple_index1_27() {
    let (out, diags) = interpret("fn main() let t = (27, 28) print(t[1]) end", 0);
    assert!(!diags.has_errors(), "tuple_index1_27: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"28
");
}

#[test]
fn tuple_index1_28() {
    let (out, diags) = interpret("fn main() let t = (28, 29) print(t[1]) end", 0);
    assert!(!diags.has_errors(), "tuple_index1_28: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"29
");
}

#[test]
fn tuple_index1_29() {
    let (out, diags) = interpret("fn main() let t = (29, 30) print(t[1]) end", 0);
    assert!(!diags.has_errors(), "tuple_index1_29: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"30
");
}

#[test]
fn tuple_index1_30() {
    let (out, diags) = interpret("fn main() let t = (30, 31) print(t[1]) end", 0);
    assert!(!diags.has_errors(), "tuple_index1_30: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"31
");
}

#[test]
fn tuple_index1_31() {
    let (out, diags) = interpret("fn main() let t = (31, 32) print(t[1]) end", 0);
    assert!(!diags.has_errors(), "tuple_index1_31: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"32
");
}

#[test]
fn tuple_index1_32() {
    let (out, diags) = interpret("fn main() let t = (32, 33) print(t[1]) end", 0);
    assert!(!diags.has_errors(), "tuple_index1_32: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"33
");
}

#[test]
fn tuple_index1_33() {
    let (out, diags) = interpret("fn main() let t = (33, 34) print(t[1]) end", 0);
    assert!(!diags.has_errors(), "tuple_index1_33: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"34
");
}

#[test]
fn tuple_index1_34() {
    let (out, diags) = interpret("fn main() let t = (34, 35) print(t[1]) end", 0);
    assert!(!diags.has_errors(), "tuple_index1_34: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"35
");
}

#[test]
fn tuple_index1_35() {
    let (out, diags) = interpret("fn main() let t = (35, 36) print(t[1]) end", 0);
    assert!(!diags.has_errors(), "tuple_index1_35: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"36
");
}

#[test]
fn tuple_index1_36() {
    let (out, diags) = interpret("fn main() let t = (36, 37) print(t[1]) end", 0);
    assert!(!diags.has_errors(), "tuple_index1_36: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"37
");
}

#[test]
fn tuple_index1_37() {
    let (out, diags) = interpret("fn main() let t = (37, 38) print(t[1]) end", 0);
    assert!(!diags.has_errors(), "tuple_index1_37: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"38
");
}

#[test]
fn tuple_index1_38() {
    let (out, diags) = interpret("fn main() let t = (38, 39) print(t[1]) end", 0);
    assert!(!diags.has_errors(), "tuple_index1_38: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"39
");
}

#[test]
fn tuple_index1_39() {
    let (out, diags) = interpret("fn main() let t = (39, 40) print(t[1]) end", 0);
    assert!(!diags.has_errors(), "tuple_index1_39: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"40
");
}

#[test]
fn tuple_index1_40() {
    let (out, diags) = interpret("fn main() let t = (40, 41) print(t[1]) end", 0);
    assert!(!diags.has_errors(), "tuple_index1_40: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"41
");
}

#[test]
fn tuple_index1_41() {
    let (out, diags) = interpret("fn main() let t = (41, 42) print(t[1]) end", 0);
    assert!(!diags.has_errors(), "tuple_index1_41: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"42
");
}

#[test]
fn tuple_index1_42() {
    let (out, diags) = interpret("fn main() let t = (42, 43) print(t[1]) end", 0);
    assert!(!diags.has_errors(), "tuple_index1_42: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"43
");
}

#[test]
fn tuple_index1_43() {
    let (out, diags) = interpret("fn main() let t = (43, 44) print(t[1]) end", 0);
    assert!(!diags.has_errors(), "tuple_index1_43: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"44
");
}

#[test]
fn tuple_index1_44() {
    let (out, diags) = interpret("fn main() let t = (44, 45) print(t[1]) end", 0);
    assert!(!diags.has_errors(), "tuple_index1_44: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"45
");
}

#[test]
fn tuple_index1_45() {
    let (out, diags) = interpret("fn main() let t = (45, 46) print(t[1]) end", 0);
    assert!(!diags.has_errors(), "tuple_index1_45: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"46
");
}

#[test]
fn tuple_index1_46() {
    let (out, diags) = interpret("fn main() let t = (46, 47) print(t[1]) end", 0);
    assert!(!diags.has_errors(), "tuple_index1_46: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"47
");
}

#[test]
fn tuple_index1_47() {
    let (out, diags) = interpret("fn main() let t = (47, 48) print(t[1]) end", 0);
    assert!(!diags.has_errors(), "tuple_index1_47: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"48
");
}

#[test]
fn tuple_index1_48() {
    let (out, diags) = interpret("fn main() let t = (48, 49) print(t[1]) end", 0);
    assert!(!diags.has_errors(), "tuple_index1_48: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"49
");
}

#[test]
fn tuple_index1_49() {
    let (out, diags) = interpret("fn main() let t = (49, 50) print(t[1]) end", 0);
    assert!(!diags.has_errors(), "tuple_index1_49: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"50
");
}

