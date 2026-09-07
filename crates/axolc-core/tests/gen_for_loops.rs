// Owner: PascalElixir / axolrs (GitHub org)
// File: Auto-generated tests for for loops.

use axolc_core::interpret;

#[test]
fn for_sum_0() {
    let (out, diags) = interpret("fn main() var s = 0 for k in [0,1,2,3,4] do s = s + k end print(s) end", 0);
    assert!(!diags.has_errors(), "for_sum_0: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"10
");
}

#[test]
fn for_sum_1() {
    let (out, diags) = interpret("fn main() var s = 0 for k in [0,1,2,3,4,5] do s = s + k end print(s) end", 0);
    assert!(!diags.has_errors(), "for_sum_1: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"15
");
}

#[test]
fn for_sum_2() {
    let (out, diags) = interpret("fn main() var s = 0 for k in [0,1,2,3,4,5,6] do s = s + k end print(s) end", 0);
    assert!(!diags.has_errors(), "for_sum_2: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"21
");
}

#[test]
fn for_sum_3() {
    let (out, diags) = interpret("fn main() var s = 0 for k in [0,1,2,3,4,5,6,7] do s = s + k end print(s) end", 0);
    assert!(!diags.has_errors(), "for_sum_3: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"28
");
}

#[test]
fn for_sum_4() {
    let (out, diags) = interpret("fn main() var s = 0 for k in [0,1,2,3,4,5,6,7,8] do s = s + k end print(s) end", 0);
    assert!(!diags.has_errors(), "for_sum_4: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"36
");
}

#[test]
fn for_sum_5() {
    let (out, diags) = interpret("fn main() var s = 0 for k in [0,1,2,3,4,5,6,7,8,9] do s = s + k end print(s) end", 0);
    assert!(!diags.has_errors(), "for_sum_5: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"45
");
}

#[test]
fn for_sum_6() {
    let (out, diags) = interpret("fn main() var s = 0 for k in [0,1,2,3,4,5,6,7,8,9,10] do s = s + k end print(s) end", 0);
    assert!(!diags.has_errors(), "for_sum_6: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"55
");
}

#[test]
fn for_sum_7() {
    let (out, diags) = interpret("fn main() var s = 0 for k in [0,1,2,3,4,5,6,7,8,9,10,11] do s = s + k end print(s) end", 0);
    assert!(!diags.has_errors(), "for_sum_7: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"66
");
}

#[test]
fn for_sum_8() {
    let (out, diags) = interpret("fn main() var s = 0 for k in [0,1,2,3,4,5,6,7,8,9,10,11,12] do s = s + k end print(s) end", 0);
    assert!(!diags.has_errors(), "for_sum_8: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"78
");
}

#[test]
fn for_sum_9() {
    let (out, diags) = interpret("fn main() var s = 0 for k in [0,1,2,3,4,5,6,7,8,9,10,11,12,13] do s = s + k end print(s) end", 0);
    assert!(!diags.has_errors(), "for_sum_9: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"91
");
}

#[test]
fn for_sum_10() {
    let (out, diags) = interpret("fn main() var s = 0 for k in [0,1,2,3,4,5,6,7,8,9,10,11,12,13,14] do s = s + k end print(s) end", 0);
    assert!(!diags.has_errors(), "for_sum_10: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"105
");
}

#[test]
fn for_sum_11() {
    let (out, diags) = interpret("fn main() var s = 0 for k in [0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15] do s = s + k end print(s) end", 0);
    assert!(!diags.has_errors(), "for_sum_11: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"120
");
}

#[test]
fn for_sum_12() {
    let (out, diags) = interpret("fn main() var s = 0 for k in [0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16] do s = s + k end print(s) end", 0);
    assert!(!diags.has_errors(), "for_sum_12: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"136
");
}

#[test]
fn for_sum_13() {
    let (out, diags) = interpret("fn main() var s = 0 for k in [0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17] do s = s + k end print(s) end", 0);
    assert!(!diags.has_errors(), "for_sum_13: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"153
");
}

#[test]
fn for_sum_14() {
    let (out, diags) = interpret("fn main() var s = 0 for k in [0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18] do s = s + k end print(s) end", 0);
    assert!(!diags.has_errors(), "for_sum_14: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"171
");
}

#[test]
fn for_sum_15() {
    let (out, diags) = interpret("fn main() var s = 0 for k in [0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19] do s = s + k end print(s) end", 0);
    assert!(!diags.has_errors(), "for_sum_15: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"190
");
}

#[test]
fn for_sum_16() {
    let (out, diags) = interpret("fn main() var s = 0 for k in [0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20] do s = s + k end print(s) end", 0);
    assert!(!diags.has_errors(), "for_sum_16: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"210
");
}

#[test]
fn for_sum_17() {
    let (out, diags) = interpret("fn main() var s = 0 for k in [0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21] do s = s + k end print(s) end", 0);
    assert!(!diags.has_errors(), "for_sum_17: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"231
");
}

#[test]
fn for_sum_18() {
    let (out, diags) = interpret("fn main() var s = 0 for k in [0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22] do s = s + k end print(s) end", 0);
    assert!(!diags.has_errors(), "for_sum_18: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"253
");
}

#[test]
fn for_sum_19() {
    let (out, diags) = interpret("fn main() var s = 0 for k in [0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23] do s = s + k end print(s) end", 0);
    assert!(!diags.has_errors(), "for_sum_19: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"276
");
}

#[test]
fn for_sum_20() {
    let (out, diags) = interpret("fn main() var s = 0 for k in [0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24] do s = s + k end print(s) end", 0);
    assert!(!diags.has_errors(), "for_sum_20: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"300
");
}

#[test]
fn for_sum_21() {
    let (out, diags) = interpret("fn main() var s = 0 for k in [0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25] do s = s + k end print(s) end", 0);
    assert!(!diags.has_errors(), "for_sum_21: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"325
");
}

#[test]
fn for_sum_22() {
    let (out, diags) = interpret("fn main() var s = 0 for k in [0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26] do s = s + k end print(s) end", 0);
    assert!(!diags.has_errors(), "for_sum_22: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"351
");
}

#[test]
fn for_sum_23() {
    let (out, diags) = interpret("fn main() var s = 0 for k in [0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27] do s = s + k end print(s) end", 0);
    assert!(!diags.has_errors(), "for_sum_23: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"378
");
}

#[test]
fn for_sum_24() {
    let (out, diags) = interpret("fn main() var s = 0 for k in [0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28] do s = s + k end print(s) end", 0);
    assert!(!diags.has_errors(), "for_sum_24: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"406
");
}

#[test]
fn for_sum_25() {
    let (out, diags) = interpret("fn main() var s = 0 for k in [0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29] do s = s + k end print(s) end", 0);
    assert!(!diags.has_errors(), "for_sum_25: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"435
");
}

#[test]
fn for_sum_26() {
    let (out, diags) = interpret("fn main() var s = 0 for k in [0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30] do s = s + k end print(s) end", 0);
    assert!(!diags.has_errors(), "for_sum_26: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"465
");
}

#[test]
fn for_sum_27() {
    let (out, diags) = interpret("fn main() var s = 0 for k in [0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31] do s = s + k end print(s) end", 0);
    assert!(!diags.has_errors(), "for_sum_27: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"496
");
}

#[test]
fn for_sum_28() {
    let (out, diags) = interpret("fn main() var s = 0 for k in [0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32] do s = s + k end print(s) end", 0);
    assert!(!diags.has_errors(), "for_sum_28: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"528
");
}

#[test]
fn for_sum_29() {
    let (out, diags) = interpret("fn main() var s = 0 for k in [0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33] do s = s + k end print(s) end", 0);
    assert!(!diags.has_errors(), "for_sum_29: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"561
");
}

#[test]
fn for_sum_30() {
    let (out, diags) = interpret("fn main() var s = 0 for k in [0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34] do s = s + k end print(s) end", 0);
    assert!(!diags.has_errors(), "for_sum_30: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"595
");
}

#[test]
fn for_sum_31() {
    let (out, diags) = interpret("fn main() var s = 0 for k in [0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35] do s = s + k end print(s) end", 0);
    assert!(!diags.has_errors(), "for_sum_31: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"630
");
}

#[test]
fn for_sum_32() {
    let (out, diags) = interpret("fn main() var s = 0 for k in [0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36] do s = s + k end print(s) end", 0);
    assert!(!diags.has_errors(), "for_sum_32: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"666
");
}

#[test]
fn for_sum_33() {
    let (out, diags) = interpret("fn main() var s = 0 for k in [0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37] do s = s + k end print(s) end", 0);
    assert!(!diags.has_errors(), "for_sum_33: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"703
");
}

#[test]
fn for_sum_34() {
    let (out, diags) = interpret("fn main() var s = 0 for k in [0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38] do s = s + k end print(s) end", 0);
    assert!(!diags.has_errors(), "for_sum_34: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"741
");
}

#[test]
fn for_sum_35() {
    let (out, diags) = interpret("fn main() var s = 0 for k in [0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39] do s = s + k end print(s) end", 0);
    assert!(!diags.has_errors(), "for_sum_35: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"780
");
}

#[test]
fn for_sum_36() {
    let (out, diags) = interpret("fn main() var s = 0 for k in [0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40] do s = s + k end print(s) end", 0);
    assert!(!diags.has_errors(), "for_sum_36: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"820
");
}

#[test]
fn for_sum_37() {
    let (out, diags) = interpret("fn main() var s = 0 for k in [0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41] do s = s + k end print(s) end", 0);
    assert!(!diags.has_errors(), "for_sum_37: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"861
");
}

#[test]
fn for_sum_38() {
    let (out, diags) = interpret("fn main() var s = 0 for k in [0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42] do s = s + k end print(s) end", 0);
    assert!(!diags.has_errors(), "for_sum_38: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"903
");
}

#[test]
fn for_sum_39() {
    let (out, diags) = interpret("fn main() var s = 0 for k in [0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43] do s = s + k end print(s) end", 0);
    assert!(!diags.has_errors(), "for_sum_39: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"946
");
}

#[test]
fn for_sum_40() {
    let (out, diags) = interpret("fn main() var s = 0 for k in [0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44] do s = s + k end print(s) end", 0);
    assert!(!diags.has_errors(), "for_sum_40: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"990
");
}

#[test]
fn for_sum_41() {
    let (out, diags) = interpret("fn main() var s = 0 for k in [0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45] do s = s + k end print(s) end", 0);
    assert!(!diags.has_errors(), "for_sum_41: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"1035
");
}

#[test]
fn for_sum_42() {
    let (out, diags) = interpret("fn main() var s = 0 for k in [0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46] do s = s + k end print(s) end", 0);
    assert!(!diags.has_errors(), "for_sum_42: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"1081
");
}

#[test]
fn for_sum_43() {
    let (out, diags) = interpret("fn main() var s = 0 for k in [0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47] do s = s + k end print(s) end", 0);
    assert!(!diags.has_errors(), "for_sum_43: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"1128
");
}

#[test]
fn for_sum_44() {
    let (out, diags) = interpret("fn main() var s = 0 for k in [0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48] do s = s + k end print(s) end", 0);
    assert!(!diags.has_errors(), "for_sum_44: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"1176
");
}

#[test]
fn for_sum_45() {
    let (out, diags) = interpret("fn main() var s = 0 for k in [0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49] do s = s + k end print(s) end", 0);
    assert!(!diags.has_errors(), "for_sum_45: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"1225
");
}

#[test]
fn for_sum_46() {
    let (out, diags) = interpret("fn main() var s = 0 for k in [0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50] do s = s + k end print(s) end", 0);
    assert!(!diags.has_errors(), "for_sum_46: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"1275
");
}

#[test]
fn for_sum_47() {
    let (out, diags) = interpret("fn main() var s = 0 for k in [0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51] do s = s + k end print(s) end", 0);
    assert!(!diags.has_errors(), "for_sum_47: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"1326
");
}

#[test]
fn for_sum_48() {
    let (out, diags) = interpret("fn main() var s = 0 for k in [0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51,52] do s = s + k end print(s) end", 0);
    assert!(!diags.has_errors(), "for_sum_48: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"1378
");
}

#[test]
fn for_sum_49() {
    let (out, diags) = interpret("fn main() var s = 0 for k in [0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51,52,53] do s = s + k end print(s) end", 0);
    assert!(!diags.has_errors(), "for_sum_49: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"1431
");
}

