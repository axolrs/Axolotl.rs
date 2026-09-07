// Owner: PascalElixir / axolrs (GitHub org)
// File: Auto-generated tests for while loops.

use axolc_core::interpret;

#[test]
fn while_sum_0() {
    let (out, diags) = interpret("fn main() var i = 0 var sum = 0 while i < 1 do sum = sum + i i = i + 1 end print(sum) end", 0);
    assert!(!diags.has_errors(), "while_sum_0: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"0
");
}

#[test]
fn while_sum_1() {
    let (out, diags) = interpret("fn main() var i = 0 var sum = 0 while i < 2 do sum = sum + i i = i + 1 end print(sum) end", 0);
    assert!(!diags.has_errors(), "while_sum_1: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"1
");
}

#[test]
fn while_sum_2() {
    let (out, diags) = interpret("fn main() var i = 0 var sum = 0 while i < 3 do sum = sum + i i = i + 1 end print(sum) end", 0);
    assert!(!diags.has_errors(), "while_sum_2: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"3
");
}

#[test]
fn while_sum_3() {
    let (out, diags) = interpret("fn main() var i = 0 var sum = 0 while i < 4 do sum = sum + i i = i + 1 end print(sum) end", 0);
    assert!(!diags.has_errors(), "while_sum_3: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"6
");
}

#[test]
fn while_sum_4() {
    let (out, diags) = interpret("fn main() var i = 0 var sum = 0 while i < 5 do sum = sum + i i = i + 1 end print(sum) end", 0);
    assert!(!diags.has_errors(), "while_sum_4: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"10
");
}

#[test]
fn while_sum_5() {
    let (out, diags) = interpret("fn main() var i = 0 var sum = 0 while i < 6 do sum = sum + i i = i + 1 end print(sum) end", 0);
    assert!(!diags.has_errors(), "while_sum_5: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"15
");
}

#[test]
fn while_sum_6() {
    let (out, diags) = interpret("fn main() var i = 0 var sum = 0 while i < 7 do sum = sum + i i = i + 1 end print(sum) end", 0);
    assert!(!diags.has_errors(), "while_sum_6: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"21
");
}

#[test]
fn while_sum_7() {
    let (out, diags) = interpret("fn main() var i = 0 var sum = 0 while i < 8 do sum = sum + i i = i + 1 end print(sum) end", 0);
    assert!(!diags.has_errors(), "while_sum_7: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"28
");
}

#[test]
fn while_sum_8() {
    let (out, diags) = interpret("fn main() var i = 0 var sum = 0 while i < 9 do sum = sum + i i = i + 1 end print(sum) end", 0);
    assert!(!diags.has_errors(), "while_sum_8: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"36
");
}

#[test]
fn while_sum_9() {
    let (out, diags) = interpret("fn main() var i = 0 var sum = 0 while i < 10 do sum = sum + i i = i + 1 end print(sum) end", 0);
    assert!(!diags.has_errors(), "while_sum_9: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"45
");
}

#[test]
fn while_sum_10() {
    let (out, diags) = interpret("fn main() var i = 0 var sum = 0 while i < 11 do sum = sum + i i = i + 1 end print(sum) end", 0);
    assert!(!diags.has_errors(), "while_sum_10: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"55
");
}

#[test]
fn while_sum_11() {
    let (out, diags) = interpret("fn main() var i = 0 var sum = 0 while i < 12 do sum = sum + i i = i + 1 end print(sum) end", 0);
    assert!(!diags.has_errors(), "while_sum_11: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"66
");
}

#[test]
fn while_sum_12() {
    let (out, diags) = interpret("fn main() var i = 0 var sum = 0 while i < 13 do sum = sum + i i = i + 1 end print(sum) end", 0);
    assert!(!diags.has_errors(), "while_sum_12: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"78
");
}

#[test]
fn while_sum_13() {
    let (out, diags) = interpret("fn main() var i = 0 var sum = 0 while i < 14 do sum = sum + i i = i + 1 end print(sum) end", 0);
    assert!(!diags.has_errors(), "while_sum_13: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"91
");
}

#[test]
fn while_sum_14() {
    let (out, diags) = interpret("fn main() var i = 0 var sum = 0 while i < 15 do sum = sum + i i = i + 1 end print(sum) end", 0);
    assert!(!diags.has_errors(), "while_sum_14: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"105
");
}

#[test]
fn while_sum_15() {
    let (out, diags) = interpret("fn main() var i = 0 var sum = 0 while i < 16 do sum = sum + i i = i + 1 end print(sum) end", 0);
    assert!(!diags.has_errors(), "while_sum_15: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"120
");
}

#[test]
fn while_sum_16() {
    let (out, diags) = interpret("fn main() var i = 0 var sum = 0 while i < 17 do sum = sum + i i = i + 1 end print(sum) end", 0);
    assert!(!diags.has_errors(), "while_sum_16: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"136
");
}

#[test]
fn while_sum_17() {
    let (out, diags) = interpret("fn main() var i = 0 var sum = 0 while i < 18 do sum = sum + i i = i + 1 end print(sum) end", 0);
    assert!(!diags.has_errors(), "while_sum_17: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"153
");
}

#[test]
fn while_sum_18() {
    let (out, diags) = interpret("fn main() var i = 0 var sum = 0 while i < 19 do sum = sum + i i = i + 1 end print(sum) end", 0);
    assert!(!diags.has_errors(), "while_sum_18: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"171
");
}

#[test]
fn while_sum_19() {
    let (out, diags) = interpret("fn main() var i = 0 var sum = 0 while i < 20 do sum = sum + i i = i + 1 end print(sum) end", 0);
    assert!(!diags.has_errors(), "while_sum_19: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"190
");
}

#[test]
fn while_sum_20() {
    let (out, diags) = interpret("fn main() var i = 0 var sum = 0 while i < 21 do sum = sum + i i = i + 1 end print(sum) end", 0);
    assert!(!diags.has_errors(), "while_sum_20: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"210
");
}

#[test]
fn while_sum_21() {
    let (out, diags) = interpret("fn main() var i = 0 var sum = 0 while i < 22 do sum = sum + i i = i + 1 end print(sum) end", 0);
    assert!(!diags.has_errors(), "while_sum_21: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"231
");
}

#[test]
fn while_sum_22() {
    let (out, diags) = interpret("fn main() var i = 0 var sum = 0 while i < 23 do sum = sum + i i = i + 1 end print(sum) end", 0);
    assert!(!diags.has_errors(), "while_sum_22: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"253
");
}

#[test]
fn while_sum_23() {
    let (out, diags) = interpret("fn main() var i = 0 var sum = 0 while i < 24 do sum = sum + i i = i + 1 end print(sum) end", 0);
    assert!(!diags.has_errors(), "while_sum_23: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"276
");
}

#[test]
fn while_sum_24() {
    let (out, diags) = interpret("fn main() var i = 0 var sum = 0 while i < 25 do sum = sum + i i = i + 1 end print(sum) end", 0);
    assert!(!diags.has_errors(), "while_sum_24: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"300
");
}

#[test]
fn while_sum_25() {
    let (out, diags) = interpret("fn main() var i = 0 var sum = 0 while i < 26 do sum = sum + i i = i + 1 end print(sum) end", 0);
    assert!(!diags.has_errors(), "while_sum_25: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"325
");
}

#[test]
fn while_sum_26() {
    let (out, diags) = interpret("fn main() var i = 0 var sum = 0 while i < 27 do sum = sum + i i = i + 1 end print(sum) end", 0);
    assert!(!diags.has_errors(), "while_sum_26: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"351
");
}

#[test]
fn while_sum_27() {
    let (out, diags) = interpret("fn main() var i = 0 var sum = 0 while i < 28 do sum = sum + i i = i + 1 end print(sum) end", 0);
    assert!(!diags.has_errors(), "while_sum_27: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"378
");
}

#[test]
fn while_sum_28() {
    let (out, diags) = interpret("fn main() var i = 0 var sum = 0 while i < 29 do sum = sum + i i = i + 1 end print(sum) end", 0);
    assert!(!diags.has_errors(), "while_sum_28: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"406
");
}

#[test]
fn while_sum_29() {
    let (out, diags) = interpret("fn main() var i = 0 var sum = 0 while i < 30 do sum = sum + i i = i + 1 end print(sum) end", 0);
    assert!(!diags.has_errors(), "while_sum_29: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"435
");
}

#[test]
fn while_sum_30() {
    let (out, diags) = interpret("fn main() var i = 0 var sum = 0 while i < 31 do sum = sum + i i = i + 1 end print(sum) end", 0);
    assert!(!diags.has_errors(), "while_sum_30: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"465
");
}

#[test]
fn while_sum_31() {
    let (out, diags) = interpret("fn main() var i = 0 var sum = 0 while i < 32 do sum = sum + i i = i + 1 end print(sum) end", 0);
    assert!(!diags.has_errors(), "while_sum_31: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"496
");
}

#[test]
fn while_sum_32() {
    let (out, diags) = interpret("fn main() var i = 0 var sum = 0 while i < 33 do sum = sum + i i = i + 1 end print(sum) end", 0);
    assert!(!diags.has_errors(), "while_sum_32: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"528
");
}

#[test]
fn while_sum_33() {
    let (out, diags) = interpret("fn main() var i = 0 var sum = 0 while i < 34 do sum = sum + i i = i + 1 end print(sum) end", 0);
    assert!(!diags.has_errors(), "while_sum_33: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"561
");
}

#[test]
fn while_sum_34() {
    let (out, diags) = interpret("fn main() var i = 0 var sum = 0 while i < 35 do sum = sum + i i = i + 1 end print(sum) end", 0);
    assert!(!diags.has_errors(), "while_sum_34: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"595
");
}

#[test]
fn while_sum_35() {
    let (out, diags) = interpret("fn main() var i = 0 var sum = 0 while i < 36 do sum = sum + i i = i + 1 end print(sum) end", 0);
    assert!(!diags.has_errors(), "while_sum_35: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"630
");
}

#[test]
fn while_sum_36() {
    let (out, diags) = interpret("fn main() var i = 0 var sum = 0 while i < 37 do sum = sum + i i = i + 1 end print(sum) end", 0);
    assert!(!diags.has_errors(), "while_sum_36: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"666
");
}

#[test]
fn while_sum_37() {
    let (out, diags) = interpret("fn main() var i = 0 var sum = 0 while i < 38 do sum = sum + i i = i + 1 end print(sum) end", 0);
    assert!(!diags.has_errors(), "while_sum_37: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"703
");
}

#[test]
fn while_sum_38() {
    let (out, diags) = interpret("fn main() var i = 0 var sum = 0 while i < 39 do sum = sum + i i = i + 1 end print(sum) end", 0);
    assert!(!diags.has_errors(), "while_sum_38: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"741
");
}

#[test]
fn while_sum_39() {
    let (out, diags) = interpret("fn main() var i = 0 var sum = 0 while i < 40 do sum = sum + i i = i + 1 end print(sum) end", 0);
    assert!(!diags.has_errors(), "while_sum_39: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"780
");
}

#[test]
fn while_sum_40() {
    let (out, diags) = interpret("fn main() var i = 0 var sum = 0 while i < 41 do sum = sum + i i = i + 1 end print(sum) end", 0);
    assert!(!diags.has_errors(), "while_sum_40: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"820
");
}

#[test]
fn while_sum_41() {
    let (out, diags) = interpret("fn main() var i = 0 var sum = 0 while i < 42 do sum = sum + i i = i + 1 end print(sum) end", 0);
    assert!(!diags.has_errors(), "while_sum_41: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"861
");
}

#[test]
fn while_sum_42() {
    let (out, diags) = interpret("fn main() var i = 0 var sum = 0 while i < 43 do sum = sum + i i = i + 1 end print(sum) end", 0);
    assert!(!diags.has_errors(), "while_sum_42: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"903
");
}

#[test]
fn while_sum_43() {
    let (out, diags) = interpret("fn main() var i = 0 var sum = 0 while i < 44 do sum = sum + i i = i + 1 end print(sum) end", 0);
    assert!(!diags.has_errors(), "while_sum_43: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"946
");
}

#[test]
fn while_sum_44() {
    let (out, diags) = interpret("fn main() var i = 0 var sum = 0 while i < 45 do sum = sum + i i = i + 1 end print(sum) end", 0);
    assert!(!diags.has_errors(), "while_sum_44: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"990
");
}

#[test]
fn while_sum_45() {
    let (out, diags) = interpret("fn main() var i = 0 var sum = 0 while i < 46 do sum = sum + i i = i + 1 end print(sum) end", 0);
    assert!(!diags.has_errors(), "while_sum_45: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"1035
");
}

#[test]
fn while_sum_46() {
    let (out, diags) = interpret("fn main() var i = 0 var sum = 0 while i < 47 do sum = sum + i i = i + 1 end print(sum) end", 0);
    assert!(!diags.has_errors(), "while_sum_46: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"1081
");
}

#[test]
fn while_sum_47() {
    let (out, diags) = interpret("fn main() var i = 0 var sum = 0 while i < 48 do sum = sum + i i = i + 1 end print(sum) end", 0);
    assert!(!diags.has_errors(), "while_sum_47: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"1128
");
}

#[test]
fn while_sum_48() {
    let (out, diags) = interpret("fn main() var i = 0 var sum = 0 while i < 49 do sum = sum + i i = i + 1 end print(sum) end", 0);
    assert!(!diags.has_errors(), "while_sum_48: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"1176
");
}

#[test]
fn while_sum_49() {
    let (out, diags) = interpret("fn main() var i = 0 var sum = 0 while i < 50 do sum = sum + i i = i + 1 end print(sum) end", 0);
    assert!(!diags.has_errors(), "while_sum_49: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"1225
");
}

#[test]
fn while_factorial_0() {
    let (out, diags) = interpret("fn main() var i = 1 var f = 1 while i <= 2 do f = f * i i = i + 1 end print(f) end", 0);
    assert!(!diags.has_errors(), "while_factorial_0: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"2
");
}

#[test]
fn while_factorial_1() {
    let (out, diags) = interpret("fn main() var i = 1 var f = 1 while i <= 3 do f = f * i i = i + 1 end print(f) end", 0);
    assert!(!diags.has_errors(), "while_factorial_1: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"6
");
}

#[test]
fn while_factorial_2() {
    let (out, diags) = interpret("fn main() var i = 1 var f = 1 while i <= 4 do f = f * i i = i + 1 end print(f) end", 0);
    assert!(!diags.has_errors(), "while_factorial_2: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"24
");
}

#[test]
fn while_factorial_3() {
    let (out, diags) = interpret("fn main() var i = 1 var f = 1 while i <= 5 do f = f * i i = i + 1 end print(f) end", 0);
    assert!(!diags.has_errors(), "while_factorial_3: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"120
");
}

#[test]
fn while_factorial_4() {
    let (out, diags) = interpret("fn main() var i = 1 var f = 1 while i <= 6 do f = f * i i = i + 1 end print(f) end", 0);
    assert!(!diags.has_errors(), "while_factorial_4: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"720
");
}

#[test]
fn while_factorial_5() {
    let (out, diags) = interpret("fn main() var i = 1 var f = 1 while i <= 7 do f = f * i i = i + 1 end print(f) end", 0);
    assert!(!diags.has_errors(), "while_factorial_5: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"5040
");
}

#[test]
fn while_factorial_6() {
    let (out, diags) = interpret("fn main() var i = 1 var f = 1 while i <= 8 do f = f * i i = i + 1 end print(f) end", 0);
    assert!(!diags.has_errors(), "while_factorial_6: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"40320
");
}

#[test]
fn while_factorial_7() {
    let (out, diags) = interpret("fn main() var i = 1 var f = 1 while i <= 9 do f = f * i i = i + 1 end print(f) end", 0);
    assert!(!diags.has_errors(), "while_factorial_7: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"362880
");
}

#[test]
fn while_factorial_8() {
    let (out, diags) = interpret("fn main() var i = 1 var f = 1 while i <= 10 do f = f * i i = i + 1 end print(f) end", 0);
    assert!(!diags.has_errors(), "while_factorial_8: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"3628800
");
}

#[test]
fn while_factorial_9() {
    let (out, diags) = interpret("fn main() var i = 1 var f = 1 while i <= 11 do f = f * i i = i + 1 end print(f) end", 0);
    assert!(!diags.has_errors(), "while_factorial_9: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"39916800
");
}

#[test]
fn while_factorial_10() {
    let (out, diags) = interpret("fn main() var i = 1 var f = 1 while i <= 12 do f = f * i i = i + 1 end print(f) end", 0);
    assert!(!diags.has_errors(), "while_factorial_10: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"479001600
");
}

#[test]
fn while_factorial_11() {
    let (out, diags) = interpret("fn main() var i = 1 var f = 1 while i <= 13 do f = f * i i = i + 1 end print(f) end", 0);
    assert!(!diags.has_errors(), "while_factorial_11: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"6227020800
");
}

#[test]
fn while_factorial_12() {
    let (out, diags) = interpret("fn main() var i = 1 var f = 1 while i <= 14 do f = f * i i = i + 1 end print(f) end", 0);
    assert!(!diags.has_errors(), "while_factorial_12: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"87178291200
");
}

#[test]
fn while_factorial_13() {
    let (out, diags) = interpret("fn main() var i = 1 var f = 1 while i <= 15 do f = f * i i = i + 1 end print(f) end", 0);
    assert!(!diags.has_errors(), "while_factorial_13: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"1307674368000
");
}

#[test]
fn while_factorial_14() {
    let (out, diags) = interpret("fn main() var i = 1 var f = 1 while i <= 16 do f = f * i i = i + 1 end print(f) end", 0);
    assert!(!diags.has_errors(), "while_factorial_14: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"20922789888000
");
}

#[test]
fn while_factorial_15() {
    let (out, diags) = interpret("fn main() var i = 1 var f = 1 while i <= 17 do f = f * i i = i + 1 end print(f) end", 0);
    assert!(!diags.has_errors(), "while_factorial_15: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"355687428096000
");
}

#[test]
fn while_factorial_16() {
    let (out, diags) = interpret("fn main() var i = 1 var f = 1 while i <= 18 do f = f * i i = i + 1 end print(f) end", 0);
    assert!(!diags.has_errors(), "while_factorial_16: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"6402373705728000
");
}

#[test]
fn while_factorial_17() {
    let (out, diags) = interpret("fn main() var i = 1 var f = 1 while i <= 19 do f = f * i i = i + 1 end print(f) end", 0);
    assert!(!diags.has_errors(), "while_factorial_17: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"121645100408832000
");
}

#[test]
fn while_factorial_18() {
    let (out, diags) = interpret("fn main() var i = 1 var f = 1 while i <= 20 do f = f * i i = i + 1 end print(f) end", 0);
    assert!(!diags.has_errors(), "while_factorial_18: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"2432902008176640000
");
}

#[test]
fn while_factorial_19() {
    let (out, diags) = interpret("fn main() var i = 1 var f = 1 while i <= 20 do f = f * i i = i + 1 end print(f) end", 0);
    assert!(!diags.has_errors(), "while_factorial_19: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"2432902008176640000
");
}

#[test]
fn while_factorial_20() {
    let (out, diags) = interpret("fn main() var i = 1 var f = 1 while i <= 20 do f = f * i i = i + 1 end print(f) end", 0);
    assert!(!diags.has_errors(), "while_factorial_20: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"2432902008176640000
");
}

#[test]
fn while_factorial_21() {
    let (out, diags) = interpret("fn main() var i = 1 var f = 1 while i <= 20 do f = f * i i = i + 1 end print(f) end", 0);
    assert!(!diags.has_errors(), "while_factorial_21: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"2432902008176640000
");
}

#[test]
fn while_factorial_22() {
    let (out, diags) = interpret("fn main() var i = 1 var f = 1 while i <= 20 do f = f * i i = i + 1 end print(f) end", 0);
    assert!(!diags.has_errors(), "while_factorial_22: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"2432902008176640000
");
}

#[test]
fn while_factorial_23() {
    let (out, diags) = interpret("fn main() var i = 1 var f = 1 while i <= 20 do f = f * i i = i + 1 end print(f) end", 0);
    assert!(!diags.has_errors(), "while_factorial_23: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"2432902008176640000
");
}

#[test]
fn while_factorial_24() {
    let (out, diags) = interpret("fn main() var i = 1 var f = 1 while i <= 20 do f = f * i i = i + 1 end print(f) end", 0);
    assert!(!diags.has_errors(), "while_factorial_24: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"2432902008176640000
");
}

#[test]
fn while_factorial_25() {
    let (out, diags) = interpret("fn main() var i = 1 var f = 1 while i <= 20 do f = f * i i = i + 1 end print(f) end", 0);
    assert!(!diags.has_errors(), "while_factorial_25: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"2432902008176640000
");
}

#[test]
fn while_factorial_26() {
    let (out, diags) = interpret("fn main() var i = 1 var f = 1 while i <= 20 do f = f * i i = i + 1 end print(f) end", 0);
    assert!(!diags.has_errors(), "while_factorial_26: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"2432902008176640000
");
}

#[test]
fn while_factorial_27() {
    let (out, diags) = interpret("fn main() var i = 1 var f = 1 while i <= 20 do f = f * i i = i + 1 end print(f) end", 0);
    assert!(!diags.has_errors(), "while_factorial_27: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"2432902008176640000
");
}

#[test]
fn while_factorial_28() {
    let (out, diags) = interpret("fn main() var i = 1 var f = 1 while i <= 20 do f = f * i i = i + 1 end print(f) end", 0);
    assert!(!diags.has_errors(), "while_factorial_28: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"2432902008176640000
");
}

#[test]
fn while_factorial_29() {
    let (out, diags) = interpret("fn main() var i = 1 var f = 1 while i <= 20 do f = f * i i = i + 1 end print(f) end", 0);
    assert!(!diags.has_errors(), "while_factorial_29: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"2432902008176640000
");
}

