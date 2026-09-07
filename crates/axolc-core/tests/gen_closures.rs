// Owner: PascalElixir / axolrs (GitHub org)
// File: Auto-generated tests for closures.

use axolc_core::interpret;

#[test]
fn closure_simple_0() {
    let (out, diags) = interpret("fn main() let f = fn(x) x * 2 end print(f(0)) end", 0);
    assert!(!diags.has_errors(), "closure_simple_0: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"0
");
}

#[test]
fn closure_simple_1() {
    let (out, diags) = interpret("fn main() let f = fn(x) x * 2 end print(f(1)) end", 0);
    assert!(!diags.has_errors(), "closure_simple_1: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"2
");
}

#[test]
fn closure_simple_2() {
    let (out, diags) = interpret("fn main() let f = fn(x) x * 2 end print(f(2)) end", 0);
    assert!(!diags.has_errors(), "closure_simple_2: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"4
");
}

#[test]
fn closure_simple_3() {
    let (out, diags) = interpret("fn main() let f = fn(x) x * 2 end print(f(3)) end", 0);
    assert!(!diags.has_errors(), "closure_simple_3: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"6
");
}

#[test]
fn closure_simple_4() {
    let (out, diags) = interpret("fn main() let f = fn(x) x * 2 end print(f(4)) end", 0);
    assert!(!diags.has_errors(), "closure_simple_4: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"8
");
}

#[test]
fn closure_simple_5() {
    let (out, diags) = interpret("fn main() let f = fn(x) x * 2 end print(f(5)) end", 0);
    assert!(!diags.has_errors(), "closure_simple_5: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"10
");
}

#[test]
fn closure_simple_6() {
    let (out, diags) = interpret("fn main() let f = fn(x) x * 2 end print(f(6)) end", 0);
    assert!(!diags.has_errors(), "closure_simple_6: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"12
");
}

#[test]
fn closure_simple_7() {
    let (out, diags) = interpret("fn main() let f = fn(x) x * 2 end print(f(7)) end", 0);
    assert!(!diags.has_errors(), "closure_simple_7: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"14
");
}

#[test]
fn closure_simple_8() {
    let (out, diags) = interpret("fn main() let f = fn(x) x * 2 end print(f(8)) end", 0);
    assert!(!diags.has_errors(), "closure_simple_8: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"16
");
}

#[test]
fn closure_simple_9() {
    let (out, diags) = interpret("fn main() let f = fn(x) x * 2 end print(f(9)) end", 0);
    assert!(!diags.has_errors(), "closure_simple_9: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"18
");
}

#[test]
fn closure_simple_10() {
    let (out, diags) = interpret("fn main() let f = fn(x) x * 2 end print(f(10)) end", 0);
    assert!(!diags.has_errors(), "closure_simple_10: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"20
");
}

#[test]
fn closure_simple_11() {
    let (out, diags) = interpret("fn main() let f = fn(x) x * 2 end print(f(11)) end", 0);
    assert!(!diags.has_errors(), "closure_simple_11: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"22
");
}

#[test]
fn closure_simple_12() {
    let (out, diags) = interpret("fn main() let f = fn(x) x * 2 end print(f(12)) end", 0);
    assert!(!diags.has_errors(), "closure_simple_12: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"24
");
}

#[test]
fn closure_simple_13() {
    let (out, diags) = interpret("fn main() let f = fn(x) x * 2 end print(f(13)) end", 0);
    assert!(!diags.has_errors(), "closure_simple_13: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"26
");
}

#[test]
fn closure_simple_14() {
    let (out, diags) = interpret("fn main() let f = fn(x) x * 2 end print(f(14)) end", 0);
    assert!(!diags.has_errors(), "closure_simple_14: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"28
");
}

#[test]
fn closure_simple_15() {
    let (out, diags) = interpret("fn main() let f = fn(x) x * 2 end print(f(15)) end", 0);
    assert!(!diags.has_errors(), "closure_simple_15: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"30
");
}

#[test]
fn closure_simple_16() {
    let (out, diags) = interpret("fn main() let f = fn(x) x * 2 end print(f(16)) end", 0);
    assert!(!diags.has_errors(), "closure_simple_16: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"32
");
}

#[test]
fn closure_simple_17() {
    let (out, diags) = interpret("fn main() let f = fn(x) x * 2 end print(f(17)) end", 0);
    assert!(!diags.has_errors(), "closure_simple_17: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"34
");
}

#[test]
fn closure_simple_18() {
    let (out, diags) = interpret("fn main() let f = fn(x) x * 2 end print(f(18)) end", 0);
    assert!(!diags.has_errors(), "closure_simple_18: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"36
");
}

#[test]
fn closure_simple_19() {
    let (out, diags) = interpret("fn main() let f = fn(x) x * 2 end print(f(19)) end", 0);
    assert!(!diags.has_errors(), "closure_simple_19: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"38
");
}

#[test]
fn closure_simple_20() {
    let (out, diags) = interpret("fn main() let f = fn(x) x * 2 end print(f(20)) end", 0);
    assert!(!diags.has_errors(), "closure_simple_20: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"40
");
}

#[test]
fn closure_simple_21() {
    let (out, diags) = interpret("fn main() let f = fn(x) x * 2 end print(f(21)) end", 0);
    assert!(!diags.has_errors(), "closure_simple_21: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"42
");
}

#[test]
fn closure_simple_22() {
    let (out, diags) = interpret("fn main() let f = fn(x) x * 2 end print(f(22)) end", 0);
    assert!(!diags.has_errors(), "closure_simple_22: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"44
");
}

#[test]
fn closure_simple_23() {
    let (out, diags) = interpret("fn main() let f = fn(x) x * 2 end print(f(23)) end", 0);
    assert!(!diags.has_errors(), "closure_simple_23: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"46
");
}

#[test]
fn closure_simple_24() {
    let (out, diags) = interpret("fn main() let f = fn(x) x * 2 end print(f(24)) end", 0);
    assert!(!diags.has_errors(), "closure_simple_24: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"48
");
}

#[test]
fn closure_simple_25() {
    let (out, diags) = interpret("fn main() let f = fn(x) x * 2 end print(f(25)) end", 0);
    assert!(!diags.has_errors(), "closure_simple_25: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"50
");
}

#[test]
fn closure_simple_26() {
    let (out, diags) = interpret("fn main() let f = fn(x) x * 2 end print(f(26)) end", 0);
    assert!(!diags.has_errors(), "closure_simple_26: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"52
");
}

#[test]
fn closure_simple_27() {
    let (out, diags) = interpret("fn main() let f = fn(x) x * 2 end print(f(27)) end", 0);
    assert!(!diags.has_errors(), "closure_simple_27: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"54
");
}

#[test]
fn closure_simple_28() {
    let (out, diags) = interpret("fn main() let f = fn(x) x * 2 end print(f(28)) end", 0);
    assert!(!diags.has_errors(), "closure_simple_28: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"56
");
}

#[test]
fn closure_simple_29() {
    let (out, diags) = interpret("fn main() let f = fn(x) x * 2 end print(f(29)) end", 0);
    assert!(!diags.has_errors(), "closure_simple_29: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"58
");
}

#[test]
fn closure_simple_30() {
    let (out, diags) = interpret("fn main() let f = fn(x) x * 2 end print(f(30)) end", 0);
    assert!(!diags.has_errors(), "closure_simple_30: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"60
");
}

#[test]
fn closure_simple_31() {
    let (out, diags) = interpret("fn main() let f = fn(x) x * 2 end print(f(31)) end", 0);
    assert!(!diags.has_errors(), "closure_simple_31: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"62
");
}

#[test]
fn closure_simple_32() {
    let (out, diags) = interpret("fn main() let f = fn(x) x * 2 end print(f(32)) end", 0);
    assert!(!diags.has_errors(), "closure_simple_32: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"64
");
}

#[test]
fn closure_simple_33() {
    let (out, diags) = interpret("fn main() let f = fn(x) x * 2 end print(f(33)) end", 0);
    assert!(!diags.has_errors(), "closure_simple_33: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"66
");
}

#[test]
fn closure_simple_34() {
    let (out, diags) = interpret("fn main() let f = fn(x) x * 2 end print(f(34)) end", 0);
    assert!(!diags.has_errors(), "closure_simple_34: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"68
");
}

#[test]
fn closure_simple_35() {
    let (out, diags) = interpret("fn main() let f = fn(x) x * 2 end print(f(35)) end", 0);
    assert!(!diags.has_errors(), "closure_simple_35: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"70
");
}

#[test]
fn closure_simple_36() {
    let (out, diags) = interpret("fn main() let f = fn(x) x * 2 end print(f(36)) end", 0);
    assert!(!diags.has_errors(), "closure_simple_36: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"72
");
}

#[test]
fn closure_simple_37() {
    let (out, diags) = interpret("fn main() let f = fn(x) x * 2 end print(f(37)) end", 0);
    assert!(!diags.has_errors(), "closure_simple_37: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"74
");
}

#[test]
fn closure_simple_38() {
    let (out, diags) = interpret("fn main() let f = fn(x) x * 2 end print(f(38)) end", 0);
    assert!(!diags.has_errors(), "closure_simple_38: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"76
");
}

#[test]
fn closure_simple_39() {
    let (out, diags) = interpret("fn main() let f = fn(x) x * 2 end print(f(39)) end", 0);
    assert!(!diags.has_errors(), "closure_simple_39: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"78
");
}

#[test]
fn closure_simple_40() {
    let (out, diags) = interpret("fn main() let f = fn(x) x * 2 end print(f(40)) end", 0);
    assert!(!diags.has_errors(), "closure_simple_40: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"80
");
}

#[test]
fn closure_simple_41() {
    let (out, diags) = interpret("fn main() let f = fn(x) x * 2 end print(f(41)) end", 0);
    assert!(!diags.has_errors(), "closure_simple_41: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"82
");
}

#[test]
fn closure_simple_42() {
    let (out, diags) = interpret("fn main() let f = fn(x) x * 2 end print(f(42)) end", 0);
    assert!(!diags.has_errors(), "closure_simple_42: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"84
");
}

#[test]
fn closure_simple_43() {
    let (out, diags) = interpret("fn main() let f = fn(x) x * 2 end print(f(43)) end", 0);
    assert!(!diags.has_errors(), "closure_simple_43: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"86
");
}

#[test]
fn closure_simple_44() {
    let (out, diags) = interpret("fn main() let f = fn(x) x * 2 end print(f(44)) end", 0);
    assert!(!diags.has_errors(), "closure_simple_44: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"88
");
}

#[test]
fn closure_simple_45() {
    let (out, diags) = interpret("fn main() let f = fn(x) x * 2 end print(f(45)) end", 0);
    assert!(!diags.has_errors(), "closure_simple_45: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"90
");
}

#[test]
fn closure_simple_46() {
    let (out, diags) = interpret("fn main() let f = fn(x) x * 2 end print(f(46)) end", 0);
    assert!(!diags.has_errors(), "closure_simple_46: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"92
");
}

#[test]
fn closure_simple_47() {
    let (out, diags) = interpret("fn main() let f = fn(x) x * 2 end print(f(47)) end", 0);
    assert!(!diags.has_errors(), "closure_simple_47: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"94
");
}

#[test]
fn closure_simple_48() {
    let (out, diags) = interpret("fn main() let f = fn(x) x * 2 end print(f(48)) end", 0);
    assert!(!diags.has_errors(), "closure_simple_48: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"96
");
}

#[test]
fn closure_simple_49() {
    let (out, diags) = interpret("fn main() let f = fn(x) x * 2 end print(f(49)) end", 0);
    assert!(!diags.has_errors(), "closure_simple_49: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"98
");
}

#[test]
fn closure_capture_0() {
    let (out, diags) = interpret("fn main() let base = 0 let add = fn(x) x + base end print(add(5)) end", 0);
    assert!(!diags.has_errors(), "closure_capture_0: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"5
");
}

#[test]
fn closure_capture_1() {
    let (out, diags) = interpret("fn main() let base = 1 let add = fn(x) x + base end print(add(6)) end", 0);
    assert!(!diags.has_errors(), "closure_capture_1: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"7
");
}

#[test]
fn closure_capture_2() {
    let (out, diags) = interpret("fn main() let base = 2 let add = fn(x) x + base end print(add(7)) end", 0);
    assert!(!diags.has_errors(), "closure_capture_2: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"9
");
}

#[test]
fn closure_capture_3() {
    let (out, diags) = interpret("fn main() let base = 3 let add = fn(x) x + base end print(add(8)) end", 0);
    assert!(!diags.has_errors(), "closure_capture_3: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"11
");
}

#[test]
fn closure_capture_4() {
    let (out, diags) = interpret("fn main() let base = 4 let add = fn(x) x + base end print(add(9)) end", 0);
    assert!(!diags.has_errors(), "closure_capture_4: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"13
");
}

#[test]
fn closure_capture_5() {
    let (out, diags) = interpret("fn main() let base = 5 let add = fn(x) x + base end print(add(10)) end", 0);
    assert!(!diags.has_errors(), "closure_capture_5: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"15
");
}

#[test]
fn closure_capture_6() {
    let (out, diags) = interpret("fn main() let base = 6 let add = fn(x) x + base end print(add(11)) end", 0);
    assert!(!diags.has_errors(), "closure_capture_6: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"17
");
}

#[test]
fn closure_capture_7() {
    let (out, diags) = interpret("fn main() let base = 7 let add = fn(x) x + base end print(add(12)) end", 0);
    assert!(!diags.has_errors(), "closure_capture_7: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"19
");
}

#[test]
fn closure_capture_8() {
    let (out, diags) = interpret("fn main() let base = 8 let add = fn(x) x + base end print(add(13)) end", 0);
    assert!(!diags.has_errors(), "closure_capture_8: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"21
");
}

#[test]
fn closure_capture_9() {
    let (out, diags) = interpret("fn main() let base = 9 let add = fn(x) x + base end print(add(14)) end", 0);
    assert!(!diags.has_errors(), "closure_capture_9: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"23
");
}

#[test]
fn closure_capture_10() {
    let (out, diags) = interpret("fn main() let base = 10 let add = fn(x) x + base end print(add(15)) end", 0);
    assert!(!diags.has_errors(), "closure_capture_10: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"25
");
}

#[test]
fn closure_capture_11() {
    let (out, diags) = interpret("fn main() let base = 11 let add = fn(x) x + base end print(add(16)) end", 0);
    assert!(!diags.has_errors(), "closure_capture_11: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"27
");
}

#[test]
fn closure_capture_12() {
    let (out, diags) = interpret("fn main() let base = 12 let add = fn(x) x + base end print(add(17)) end", 0);
    assert!(!diags.has_errors(), "closure_capture_12: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"29
");
}

#[test]
fn closure_capture_13() {
    let (out, diags) = interpret("fn main() let base = 13 let add = fn(x) x + base end print(add(18)) end", 0);
    assert!(!diags.has_errors(), "closure_capture_13: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"31
");
}

#[test]
fn closure_capture_14() {
    let (out, diags) = interpret("fn main() let base = 14 let add = fn(x) x + base end print(add(19)) end", 0);
    assert!(!diags.has_errors(), "closure_capture_14: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"33
");
}

#[test]
fn closure_capture_15() {
    let (out, diags) = interpret("fn main() let base = 15 let add = fn(x) x + base end print(add(20)) end", 0);
    assert!(!diags.has_errors(), "closure_capture_15: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"35
");
}

#[test]
fn closure_capture_16() {
    let (out, diags) = interpret("fn main() let base = 16 let add = fn(x) x + base end print(add(21)) end", 0);
    assert!(!diags.has_errors(), "closure_capture_16: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"37
");
}

#[test]
fn closure_capture_17() {
    let (out, diags) = interpret("fn main() let base = 17 let add = fn(x) x + base end print(add(22)) end", 0);
    assert!(!diags.has_errors(), "closure_capture_17: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"39
");
}

#[test]
fn closure_capture_18() {
    let (out, diags) = interpret("fn main() let base = 18 let add = fn(x) x + base end print(add(23)) end", 0);
    assert!(!diags.has_errors(), "closure_capture_18: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"41
");
}

#[test]
fn closure_capture_19() {
    let (out, diags) = interpret("fn main() let base = 19 let add = fn(x) x + base end print(add(24)) end", 0);
    assert!(!diags.has_errors(), "closure_capture_19: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"43
");
}

#[test]
fn closure_capture_20() {
    let (out, diags) = interpret("fn main() let base = 20 let add = fn(x) x + base end print(add(25)) end", 0);
    assert!(!diags.has_errors(), "closure_capture_20: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"45
");
}

#[test]
fn closure_capture_21() {
    let (out, diags) = interpret("fn main() let base = 21 let add = fn(x) x + base end print(add(26)) end", 0);
    assert!(!diags.has_errors(), "closure_capture_21: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"47
");
}

#[test]
fn closure_capture_22() {
    let (out, diags) = interpret("fn main() let base = 22 let add = fn(x) x + base end print(add(27)) end", 0);
    assert!(!diags.has_errors(), "closure_capture_22: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"49
");
}

#[test]
fn closure_capture_23() {
    let (out, diags) = interpret("fn main() let base = 23 let add = fn(x) x + base end print(add(28)) end", 0);
    assert!(!diags.has_errors(), "closure_capture_23: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"51
");
}

#[test]
fn closure_capture_24() {
    let (out, diags) = interpret("fn main() let base = 24 let add = fn(x) x + base end print(add(29)) end", 0);
    assert!(!diags.has_errors(), "closure_capture_24: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"53
");
}

#[test]
fn closure_capture_25() {
    let (out, diags) = interpret("fn main() let base = 25 let add = fn(x) x + base end print(add(30)) end", 0);
    assert!(!diags.has_errors(), "closure_capture_25: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"55
");
}

#[test]
fn closure_capture_26() {
    let (out, diags) = interpret("fn main() let base = 26 let add = fn(x) x + base end print(add(31)) end", 0);
    assert!(!diags.has_errors(), "closure_capture_26: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"57
");
}

#[test]
fn closure_capture_27() {
    let (out, diags) = interpret("fn main() let base = 27 let add = fn(x) x + base end print(add(32)) end", 0);
    assert!(!diags.has_errors(), "closure_capture_27: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"59
");
}

#[test]
fn closure_capture_28() {
    let (out, diags) = interpret("fn main() let base = 28 let add = fn(x) x + base end print(add(33)) end", 0);
    assert!(!diags.has_errors(), "closure_capture_28: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"61
");
}

#[test]
fn closure_capture_29() {
    let (out, diags) = interpret("fn main() let base = 29 let add = fn(x) x + base end print(add(34)) end", 0);
    assert!(!diags.has_errors(), "closure_capture_29: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r"63
");
}

