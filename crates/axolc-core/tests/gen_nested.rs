// Owner: PascalElixir / axolrs (GitHub org)
// File: Auto-generated tests for nested function calls.

use axolc_core::interpret;

#[test]
fn nested_call_0() {
    let (out, diags) = interpret("fn double(x) return x * 2 end fn main() print(double(double(0))) end", 0);
    assert!(!diags.has_errors(), "nested_call_0: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"0
");
}

#[test]
fn nested_call_1() {
    let (out, diags) = interpret("fn double(x) return x * 2 end fn main() print(double(double(1))) end", 0);
    assert!(!diags.has_errors(), "nested_call_1: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"4
");
}

#[test]
fn nested_call_2() {
    let (out, diags) = interpret("fn double(x) return x * 2 end fn main() print(double(double(2))) end", 0);
    assert!(!diags.has_errors(), "nested_call_2: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"8
");
}

#[test]
fn nested_call_3() {
    let (out, diags) = interpret("fn double(x) return x * 2 end fn main() print(double(double(3))) end", 0);
    assert!(!diags.has_errors(), "nested_call_3: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"12
");
}

#[test]
fn nested_call_4() {
    let (out, diags) = interpret("fn double(x) return x * 2 end fn main() print(double(double(4))) end", 0);
    assert!(!diags.has_errors(), "nested_call_4: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"16
");
}

#[test]
fn nested_call_5() {
    let (out, diags) = interpret("fn double(x) return x * 2 end fn main() print(double(double(5))) end", 0);
    assert!(!diags.has_errors(), "nested_call_5: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"20
");
}

#[test]
fn nested_call_6() {
    let (out, diags) = interpret("fn double(x) return x * 2 end fn main() print(double(double(6))) end", 0);
    assert!(!diags.has_errors(), "nested_call_6: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"24
");
}

#[test]
fn nested_call_7() {
    let (out, diags) = interpret("fn double(x) return x * 2 end fn main() print(double(double(7))) end", 0);
    assert!(!diags.has_errors(), "nested_call_7: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"28
");
}

#[test]
fn nested_call_8() {
    let (out, diags) = interpret("fn double(x) return x * 2 end fn main() print(double(double(8))) end", 0);
    assert!(!diags.has_errors(), "nested_call_8: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"32
");
}

#[test]
fn nested_call_9() {
    let (out, diags) = interpret("fn double(x) return x * 2 end fn main() print(double(double(9))) end", 0);
    assert!(!diags.has_errors(), "nested_call_9: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"36
");
}

#[test]
fn nested_call_10() {
    let (out, diags) = interpret("fn double(x) return x * 2 end fn main() print(double(double(10))) end", 0);
    assert!(!diags.has_errors(), "nested_call_10: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"40
");
}

#[test]
fn nested_call_11() {
    let (out, diags) = interpret("fn double(x) return x * 2 end fn main() print(double(double(11))) end", 0);
    assert!(!diags.has_errors(), "nested_call_11: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"44
");
}

#[test]
fn nested_call_12() {
    let (out, diags) = interpret("fn double(x) return x * 2 end fn main() print(double(double(12))) end", 0);
    assert!(!diags.has_errors(), "nested_call_12: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"48
");
}

#[test]
fn nested_call_13() {
    let (out, diags) = interpret("fn double(x) return x * 2 end fn main() print(double(double(13))) end", 0);
    assert!(!diags.has_errors(), "nested_call_13: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"52
");
}

#[test]
fn nested_call_14() {
    let (out, diags) = interpret("fn double(x) return x * 2 end fn main() print(double(double(14))) end", 0);
    assert!(!diags.has_errors(), "nested_call_14: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"56
");
}

#[test]
fn nested_call_15() {
    let (out, diags) = interpret("fn double(x) return x * 2 end fn main() print(double(double(15))) end", 0);
    assert!(!diags.has_errors(), "nested_call_15: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"60
");
}

#[test]
fn nested_call_16() {
    let (out, diags) = interpret("fn double(x) return x * 2 end fn main() print(double(double(16))) end", 0);
    assert!(!diags.has_errors(), "nested_call_16: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"64
");
}

#[test]
fn nested_call_17() {
    let (out, diags) = interpret("fn double(x) return x * 2 end fn main() print(double(double(17))) end", 0);
    assert!(!diags.has_errors(), "nested_call_17: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"68
");
}

#[test]
fn nested_call_18() {
    let (out, diags) = interpret("fn double(x) return x * 2 end fn main() print(double(double(18))) end", 0);
    assert!(!diags.has_errors(), "nested_call_18: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"72
");
}

#[test]
fn nested_call_19() {
    let (out, diags) = interpret("fn double(x) return x * 2 end fn main() print(double(double(19))) end", 0);
    assert!(!diags.has_errors(), "nested_call_19: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"76
");
}

#[test]
fn nested_call_20() {
    let (out, diags) = interpret("fn double(x) return x * 2 end fn main() print(double(double(20))) end", 0);
    assert!(!diags.has_errors(), "nested_call_20: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"80
");
}

#[test]
fn nested_call_21() {
    let (out, diags) = interpret("fn double(x) return x * 2 end fn main() print(double(double(21))) end", 0);
    assert!(!diags.has_errors(), "nested_call_21: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"84
");
}

#[test]
fn nested_call_22() {
    let (out, diags) = interpret("fn double(x) return x * 2 end fn main() print(double(double(22))) end", 0);
    assert!(!diags.has_errors(), "nested_call_22: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"88
");
}

#[test]
fn nested_call_23() {
    let (out, diags) = interpret("fn double(x) return x * 2 end fn main() print(double(double(23))) end", 0);
    assert!(!diags.has_errors(), "nested_call_23: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"92
");
}

#[test]
fn nested_call_24() {
    let (out, diags) = interpret("fn double(x) return x * 2 end fn main() print(double(double(24))) end", 0);
    assert!(!diags.has_errors(), "nested_call_24: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"96
");
}

#[test]
fn nested_call_25() {
    let (out, diags) = interpret("fn double(x) return x * 2 end fn main() print(double(double(25))) end", 0);
    assert!(!diags.has_errors(), "nested_call_25: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"100
");
}

#[test]
fn nested_call_26() {
    let (out, diags) = interpret("fn double(x) return x * 2 end fn main() print(double(double(26))) end", 0);
    assert!(!diags.has_errors(), "nested_call_26: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"104
");
}

#[test]
fn nested_call_27() {
    let (out, diags) = interpret("fn double(x) return x * 2 end fn main() print(double(double(27))) end", 0);
    assert!(!diags.has_errors(), "nested_call_27: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"108
");
}

#[test]
fn nested_call_28() {
    let (out, diags) = interpret("fn double(x) return x * 2 end fn main() print(double(double(28))) end", 0);
    assert!(!diags.has_errors(), "nested_call_28: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"112
");
}

#[test]
fn nested_call_29() {
    let (out, diags) = interpret("fn double(x) return x * 2 end fn main() print(double(double(29))) end", 0);
    assert!(!diags.has_errors(), "nested_call_29: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"116
");
}

#[test]
fn nested_call_30() {
    let (out, diags) = interpret("fn double(x) return x * 2 end fn main() print(double(double(30))) end", 0);
    assert!(!diags.has_errors(), "nested_call_30: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"120
");
}

#[test]
fn nested_call_31() {
    let (out, diags) = interpret("fn double(x) return x * 2 end fn main() print(double(double(31))) end", 0);
    assert!(!diags.has_errors(), "nested_call_31: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"124
");
}

#[test]
fn nested_call_32() {
    let (out, diags) = interpret("fn double(x) return x * 2 end fn main() print(double(double(32))) end", 0);
    assert!(!diags.has_errors(), "nested_call_32: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"128
");
}

#[test]
fn nested_call_33() {
    let (out, diags) = interpret("fn double(x) return x * 2 end fn main() print(double(double(33))) end", 0);
    assert!(!diags.has_errors(), "nested_call_33: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"132
");
}

#[test]
fn nested_call_34() {
    let (out, diags) = interpret("fn double(x) return x * 2 end fn main() print(double(double(34))) end", 0);
    assert!(!diags.has_errors(), "nested_call_34: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"136
");
}

#[test]
fn nested_call_35() {
    let (out, diags) = interpret("fn double(x) return x * 2 end fn main() print(double(double(35))) end", 0);
    assert!(!diags.has_errors(), "nested_call_35: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"140
");
}

#[test]
fn nested_call_36() {
    let (out, diags) = interpret("fn double(x) return x * 2 end fn main() print(double(double(36))) end", 0);
    assert!(!diags.has_errors(), "nested_call_36: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"144
");
}

#[test]
fn nested_call_37() {
    let (out, diags) = interpret("fn double(x) return x * 2 end fn main() print(double(double(37))) end", 0);
    assert!(!diags.has_errors(), "nested_call_37: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"148
");
}

#[test]
fn nested_call_38() {
    let (out, diags) = interpret("fn double(x) return x * 2 end fn main() print(double(double(38))) end", 0);
    assert!(!diags.has_errors(), "nested_call_38: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"152
");
}

#[test]
fn nested_call_39() {
    let (out, diags) = interpret("fn double(x) return x * 2 end fn main() print(double(double(39))) end", 0);
    assert!(!diags.has_errors(), "nested_call_39: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"156
");
}

#[test]
fn nested_call_40() {
    let (out, diags) = interpret("fn double(x) return x * 2 end fn main() print(double(double(40))) end", 0);
    assert!(!diags.has_errors(), "nested_call_40: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"160
");
}

#[test]
fn nested_call_41() {
    let (out, diags) = interpret("fn double(x) return x * 2 end fn main() print(double(double(41))) end", 0);
    assert!(!diags.has_errors(), "nested_call_41: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"164
");
}

#[test]
fn nested_call_42() {
    let (out, diags) = interpret("fn double(x) return x * 2 end fn main() print(double(double(42))) end", 0);
    assert!(!diags.has_errors(), "nested_call_42: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"168
");
}

#[test]
fn nested_call_43() {
    let (out, diags) = interpret("fn double(x) return x * 2 end fn main() print(double(double(43))) end", 0);
    assert!(!diags.has_errors(), "nested_call_43: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"172
");
}

#[test]
fn nested_call_44() {
    let (out, diags) = interpret("fn double(x) return x * 2 end fn main() print(double(double(44))) end", 0);
    assert!(!diags.has_errors(), "nested_call_44: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"176
");
}

#[test]
fn nested_call_45() {
    let (out, diags) = interpret("fn double(x) return x * 2 end fn main() print(double(double(45))) end", 0);
    assert!(!diags.has_errors(), "nested_call_45: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"180
");
}

#[test]
fn nested_call_46() {
    let (out, diags) = interpret("fn double(x) return x * 2 end fn main() print(double(double(46))) end", 0);
    assert!(!diags.has_errors(), "nested_call_46: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"184
");
}

#[test]
fn nested_call_47() {
    let (out, diags) = interpret("fn double(x) return x * 2 end fn main() print(double(double(47))) end", 0);
    assert!(!diags.has_errors(), "nested_call_47: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"188
");
}

#[test]
fn nested_call_48() {
    let (out, diags) = interpret("fn double(x) return x * 2 end fn main() print(double(double(48))) end", 0);
    assert!(!diags.has_errors(), "nested_call_48: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"192
");
}

#[test]
fn nested_call_49() {
    let (out, diags) = interpret("fn double(x) return x * 2 end fn main() print(double(double(49))) end", 0);
    assert!(!diags.has_errors(), "nested_call_49: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"196
");
}

#[test]
fn triple_nested_0() {
    let (out, diags) = interpret("fn inc(x) return x + 1 end fn main() print(inc(inc(inc(0)))) end", 0);
    assert!(!diags.has_errors(), "triple_nested_0: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"3
");
}

#[test]
fn triple_nested_1() {
    let (out, diags) = interpret("fn inc(x) return x + 1 end fn main() print(inc(inc(inc(1)))) end", 0);
    assert!(!diags.has_errors(), "triple_nested_1: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"4
");
}

#[test]
fn triple_nested_2() {
    let (out, diags) = interpret("fn inc(x) return x + 1 end fn main() print(inc(inc(inc(2)))) end", 0);
    assert!(!diags.has_errors(), "triple_nested_2: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"5
");
}

#[test]
fn triple_nested_3() {
    let (out, diags) = interpret("fn inc(x) return x + 1 end fn main() print(inc(inc(inc(3)))) end", 0);
    assert!(!diags.has_errors(), "triple_nested_3: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"6
");
}

#[test]
fn triple_nested_4() {
    let (out, diags) = interpret("fn inc(x) return x + 1 end fn main() print(inc(inc(inc(4)))) end", 0);
    assert!(!diags.has_errors(), "triple_nested_4: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"7
");
}

#[test]
fn triple_nested_5() {
    let (out, diags) = interpret("fn inc(x) return x + 1 end fn main() print(inc(inc(inc(5)))) end", 0);
    assert!(!diags.has_errors(), "triple_nested_5: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"8
");
}

#[test]
fn triple_nested_6() {
    let (out, diags) = interpret("fn inc(x) return x + 1 end fn main() print(inc(inc(inc(6)))) end", 0);
    assert!(!diags.has_errors(), "triple_nested_6: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"9
");
}

#[test]
fn triple_nested_7() {
    let (out, diags) = interpret("fn inc(x) return x + 1 end fn main() print(inc(inc(inc(7)))) end", 0);
    assert!(!diags.has_errors(), "triple_nested_7: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"10
");
}

#[test]
fn triple_nested_8() {
    let (out, diags) = interpret("fn inc(x) return x + 1 end fn main() print(inc(inc(inc(8)))) end", 0);
    assert!(!diags.has_errors(), "triple_nested_8: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"11
");
}

#[test]
fn triple_nested_9() {
    let (out, diags) = interpret("fn inc(x) return x + 1 end fn main() print(inc(inc(inc(9)))) end", 0);
    assert!(!diags.has_errors(), "triple_nested_9: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"12
");
}

#[test]
fn triple_nested_10() {
    let (out, diags) = interpret("fn inc(x) return x + 1 end fn main() print(inc(inc(inc(10)))) end", 0);
    assert!(!diags.has_errors(), "triple_nested_10: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"13
");
}

#[test]
fn triple_nested_11() {
    let (out, diags) = interpret("fn inc(x) return x + 1 end fn main() print(inc(inc(inc(11)))) end", 0);
    assert!(!diags.has_errors(), "triple_nested_11: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"14
");
}

#[test]
fn triple_nested_12() {
    let (out, diags) = interpret("fn inc(x) return x + 1 end fn main() print(inc(inc(inc(12)))) end", 0);
    assert!(!diags.has_errors(), "triple_nested_12: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"15
");
}

#[test]
fn triple_nested_13() {
    let (out, diags) = interpret("fn inc(x) return x + 1 end fn main() print(inc(inc(inc(13)))) end", 0);
    assert!(!diags.has_errors(), "triple_nested_13: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"16
");
}

#[test]
fn triple_nested_14() {
    let (out, diags) = interpret("fn inc(x) return x + 1 end fn main() print(inc(inc(inc(14)))) end", 0);
    assert!(!diags.has_errors(), "triple_nested_14: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"17
");
}

#[test]
fn triple_nested_15() {
    let (out, diags) = interpret("fn inc(x) return x + 1 end fn main() print(inc(inc(inc(15)))) end", 0);
    assert!(!diags.has_errors(), "triple_nested_15: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"18
");
}

#[test]
fn triple_nested_16() {
    let (out, diags) = interpret("fn inc(x) return x + 1 end fn main() print(inc(inc(inc(16)))) end", 0);
    assert!(!diags.has_errors(), "triple_nested_16: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"19
");
}

#[test]
fn triple_nested_17() {
    let (out, diags) = interpret("fn inc(x) return x + 1 end fn main() print(inc(inc(inc(17)))) end", 0);
    assert!(!diags.has_errors(), "triple_nested_17: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"20
");
}

#[test]
fn triple_nested_18() {
    let (out, diags) = interpret("fn inc(x) return x + 1 end fn main() print(inc(inc(inc(18)))) end", 0);
    assert!(!diags.has_errors(), "triple_nested_18: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"21
");
}

#[test]
fn triple_nested_19() {
    let (out, diags) = interpret("fn inc(x) return x + 1 end fn main() print(inc(inc(inc(19)))) end", 0);
    assert!(!diags.has_errors(), "triple_nested_19: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"22
");
}

#[test]
fn triple_nested_20() {
    let (out, diags) = interpret("fn inc(x) return x + 1 end fn main() print(inc(inc(inc(20)))) end", 0);
    assert!(!diags.has_errors(), "triple_nested_20: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"23
");
}

#[test]
fn triple_nested_21() {
    let (out, diags) = interpret("fn inc(x) return x + 1 end fn main() print(inc(inc(inc(21)))) end", 0);
    assert!(!diags.has_errors(), "triple_nested_21: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"24
");
}

#[test]
fn triple_nested_22() {
    let (out, diags) = interpret("fn inc(x) return x + 1 end fn main() print(inc(inc(inc(22)))) end", 0);
    assert!(!diags.has_errors(), "triple_nested_22: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"25
");
}

#[test]
fn triple_nested_23() {
    let (out, diags) = interpret("fn inc(x) return x + 1 end fn main() print(inc(inc(inc(23)))) end", 0);
    assert!(!diags.has_errors(), "triple_nested_23: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"26
");
}

#[test]
fn triple_nested_24() {
    let (out, diags) = interpret("fn inc(x) return x + 1 end fn main() print(inc(inc(inc(24)))) end", 0);
    assert!(!diags.has_errors(), "triple_nested_24: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"27
");
}

#[test]
fn triple_nested_25() {
    let (out, diags) = interpret("fn inc(x) return x + 1 end fn main() print(inc(inc(inc(25)))) end", 0);
    assert!(!diags.has_errors(), "triple_nested_25: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"28
");
}

#[test]
fn triple_nested_26() {
    let (out, diags) = interpret("fn inc(x) return x + 1 end fn main() print(inc(inc(inc(26)))) end", 0);
    assert!(!diags.has_errors(), "triple_nested_26: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"29
");
}

#[test]
fn triple_nested_27() {
    let (out, diags) = interpret("fn inc(x) return x + 1 end fn main() print(inc(inc(inc(27)))) end", 0);
    assert!(!diags.has_errors(), "triple_nested_27: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"30
");
}

#[test]
fn triple_nested_28() {
    let (out, diags) = interpret("fn inc(x) return x + 1 end fn main() print(inc(inc(inc(28)))) end", 0);
    assert!(!diags.has_errors(), "triple_nested_28: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"31
");
}

#[test]
fn triple_nested_29() {
    let (out, diags) = interpret("fn inc(x) return x + 1 end fn main() print(inc(inc(inc(29)))) end", 0);
    assert!(!diags.has_errors(), "triple_nested_29: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"32
");
}

