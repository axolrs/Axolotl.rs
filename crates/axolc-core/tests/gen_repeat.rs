// Owner: PascalElixir / axolrs (GitHub org)
// File: Auto-generated tests for repeat/until semantics (body runs at least once, correct exit), nested repeat, break inside repeat, and repeat codegen.

use axolc_core::interpret;
use axolc_core::compile_to_rust;

#[test]
fn repeat_once_1() {
    let (out, diags) = interpret(r#"fn main() do
  var c = 1
  repeat
    c = c + 1
  until true
  print(c)
end"#, 0);
    assert!(!diags.has_errors(), "repeat_once_1: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"2
"#);
}

#[test]
fn repeat_once_2() {
    let (out, diags) = interpret(r#"fn main() do
  var c = 2
  repeat
    c = c + 1
  until true
  print(c)
end"#, 0);
    assert!(!diags.has_errors(), "repeat_once_2: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"3
"#);
}

#[test]
fn repeat_once_3() {
    let (out, diags) = interpret(r#"fn main() do
  var c = 3
  repeat
    c = c + 1
  until true
  print(c)
end"#, 0);
    assert!(!diags.has_errors(), "repeat_once_3: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"4
"#);
}

#[test]
fn repeat_once_4() {
    let (out, diags) = interpret(r#"fn main() do
  var c = 4
  repeat
    c = c + 1
  until true
  print(c)
end"#, 0);
    assert!(!diags.has_errors(), "repeat_once_4: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"5
"#);
}

#[test]
fn repeat_once_5() {
    let (out, diags) = interpret(r#"fn main() do
  var c = 5
  repeat
    c = c + 1
  until true
  print(c)
end"#, 0);
    assert!(!diags.has_errors(), "repeat_once_5: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"6
"#);
}

#[test]
fn repeat_once_6() {
    let (out, diags) = interpret(r#"fn main() do
  var c = 6
  repeat
    c = c + 1
  until true
  print(c)
end"#, 0);
    assert!(!diags.has_errors(), "repeat_once_6: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"7
"#);
}

#[test]
fn repeat_once_7() {
    let (out, diags) = interpret(r#"fn main() do
  var c = 7
  repeat
    c = c + 1
  until true
  print(c)
end"#, 0);
    assert!(!diags.has_errors(), "repeat_once_7: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"8
"#);
}

#[test]
fn repeat_once_8() {
    let (out, diags) = interpret(r#"fn main() do
  var c = 8
  repeat
    c = c + 1
  until true
  print(c)
end"#, 0);
    assert!(!diags.has_errors(), "repeat_once_8: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"9
"#);
}

#[test]
fn repeat_once_9() {
    let (out, diags) = interpret(r#"fn main() do
  var c = 9
  repeat
    c = c + 1
  until true
  print(c)
end"#, 0);
    assert!(!diags.has_errors(), "repeat_once_9: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"10
"#);
}

#[test]
fn repeat_once_10() {
    let (out, diags) = interpret(r#"fn main() do
  var c = 10
  repeat
    c = c + 1
  until true
  print(c)
end"#, 0);
    assert!(!diags.has_errors(), "repeat_once_10: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"11
"#);
}

#[test]
fn repeat_once_11() {
    let (out, diags) = interpret(r#"fn main() do
  var c = 11
  repeat
    c = c + 1
  until true
  print(c)
end"#, 0);
    assert!(!diags.has_errors(), "repeat_once_11: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"12
"#);
}

#[test]
fn repeat_once_12() {
    let (out, diags) = interpret(r#"fn main() do
  var c = 12
  repeat
    c = c + 1
  until true
  print(c)
end"#, 0);
    assert!(!diags.has_errors(), "repeat_once_12: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"13
"#);
}

#[test]
fn repeat_once_13() {
    let (out, diags) = interpret(r#"fn main() do
  var c = 13
  repeat
    c = c + 1
  until true
  print(c)
end"#, 0);
    assert!(!diags.has_errors(), "repeat_once_13: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"14
"#);
}

#[test]
fn repeat_once_14() {
    let (out, diags) = interpret(r#"fn main() do
  var c = 14
  repeat
    c = c + 1
  until true
  print(c)
end"#, 0);
    assert!(!diags.has_errors(), "repeat_once_14: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"15
"#);
}

#[test]
fn repeat_once_15() {
    let (out, diags) = interpret(r#"fn main() do
  var c = 15
  repeat
    c = c + 1
  until true
  print(c)
end"#, 0);
    assert!(!diags.has_errors(), "repeat_once_15: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"16
"#);
}

#[test]
fn repeat_once_16() {
    let (out, diags) = interpret(r#"fn main() do
  var c = 16
  repeat
    c = c + 1
  until true
  print(c)
end"#, 0);
    assert!(!diags.has_errors(), "repeat_once_16: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"17
"#);
}

#[test]
fn repeat_once_17() {
    let (out, diags) = interpret(r#"fn main() do
  var c = 17
  repeat
    c = c + 1
  until true
  print(c)
end"#, 0);
    assert!(!diags.has_errors(), "repeat_once_17: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"18
"#);
}

#[test]
fn repeat_once_18() {
    let (out, diags) = interpret(r#"fn main() do
  var c = 18
  repeat
    c = c + 1
  until true
  print(c)
end"#, 0);
    assert!(!diags.has_errors(), "repeat_once_18: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"19
"#);
}

#[test]
fn repeat_once_19() {
    let (out, diags) = interpret(r#"fn main() do
  var c = 19
  repeat
    c = c + 1
  until true
  print(c)
end"#, 0);
    assert!(!diags.has_errors(), "repeat_once_19: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"20
"#);
}

#[test]
fn repeat_once_20() {
    let (out, diags) = interpret(r#"fn main() do
  var c = 20
  repeat
    c = c + 1
  until true
  print(c)
end"#, 0);
    assert!(!diags.has_errors(), "repeat_once_20: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"21
"#);
}

#[test]
fn repeat_once_21() {
    let (out, diags) = interpret(r#"fn main() do
  var c = 21
  repeat
    c = c + 1
  until true
  print(c)
end"#, 0);
    assert!(!diags.has_errors(), "repeat_once_21: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"22
"#);
}

#[test]
fn repeat_once_22() {
    let (out, diags) = interpret(r#"fn main() do
  var c = 22
  repeat
    c = c + 1
  until true
  print(c)
end"#, 0);
    assert!(!diags.has_errors(), "repeat_once_22: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"23
"#);
}

#[test]
fn repeat_once_23() {
    let (out, diags) = interpret(r#"fn main() do
  var c = 23
  repeat
    c = c + 1
  until true
  print(c)
end"#, 0);
    assert!(!diags.has_errors(), "repeat_once_23: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"24
"#);
}

#[test]
fn repeat_once_24() {
    let (out, diags) = interpret(r#"fn main() do
  var c = 24
  repeat
    c = c + 1
  until true
  print(c)
end"#, 0);
    assert!(!diags.has_errors(), "repeat_once_24: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"25
"#);
}

#[test]
fn repeat_once_25() {
    let (out, diags) = interpret(r#"fn main() do
  var c = 25
  repeat
    c = c + 1
  until true
  print(c)
end"#, 0);
    assert!(!diags.has_errors(), "repeat_once_25: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"26
"#);
}

#[test]
fn repeat_count_1() {
    let (out, diags) = interpret(r#"fn main() do
  var c = 0
  repeat
    c = c + 1
  until c >= 3
  print(c)
end"#, 0);
    assert!(!diags.has_errors(), "repeat_count_1: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"3
"#);
}

#[test]
fn repeat_count_2() {
    let (out, diags) = interpret(r#"fn main() do
  var c = 0
  repeat
    c = c + 1
  until c >= 4
  print(c)
end"#, 0);
    assert!(!diags.has_errors(), "repeat_count_2: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"4
"#);
}

#[test]
fn repeat_count_3() {
    let (out, diags) = interpret(r#"fn main() do
  var c = 0
  repeat
    c = c + 1
  until c >= 5
  print(c)
end"#, 0);
    assert!(!diags.has_errors(), "repeat_count_3: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"5
"#);
}

#[test]
fn repeat_count_4() {
    let (out, diags) = interpret(r#"fn main() do
  var c = 0
  repeat
    c = c + 1
  until c >= 6
  print(c)
end"#, 0);
    assert!(!diags.has_errors(), "repeat_count_4: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"6
"#);
}

#[test]
fn repeat_count_5() {
    let (out, diags) = interpret(r#"fn main() do
  var c = 0
  repeat
    c = c + 1
  until c >= 7
  print(c)
end"#, 0);
    assert!(!diags.has_errors(), "repeat_count_5: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"7
"#);
}

#[test]
fn repeat_count_6() {
    let (out, diags) = interpret(r#"fn main() do
  var c = 0
  repeat
    c = c + 1
  until c >= 8
  print(c)
end"#, 0);
    assert!(!diags.has_errors(), "repeat_count_6: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"8
"#);
}

#[test]
fn repeat_count_7() {
    let (out, diags) = interpret(r#"fn main() do
  var c = 0
  repeat
    c = c + 1
  until c >= 9
  print(c)
end"#, 0);
    assert!(!diags.has_errors(), "repeat_count_7: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"9
"#);
}

#[test]
fn repeat_count_8() {
    let (out, diags) = interpret(r#"fn main() do
  var c = 0
  repeat
    c = c + 1
  until c >= 10
  print(c)
end"#, 0);
    assert!(!diags.has_errors(), "repeat_count_8: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"10
"#);
}

#[test]
fn repeat_count_9() {
    let (out, diags) = interpret(r#"fn main() do
  var c = 0
  repeat
    c = c + 1
  until c >= 11
  print(c)
end"#, 0);
    assert!(!diags.has_errors(), "repeat_count_9: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"11
"#);
}

#[test]
fn repeat_count_10() {
    let (out, diags) = interpret(r#"fn main() do
  var c = 0
  repeat
    c = c + 1
  until c >= 12
  print(c)
end"#, 0);
    assert!(!diags.has_errors(), "repeat_count_10: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"12
"#);
}

#[test]
fn repeat_count_11() {
    let (out, diags) = interpret(r#"fn main() do
  var c = 0
  repeat
    c = c + 1
  until c >= 13
  print(c)
end"#, 0);
    assert!(!diags.has_errors(), "repeat_count_11: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"13
"#);
}

#[test]
fn repeat_count_12() {
    let (out, diags) = interpret(r#"fn main() do
  var c = 0
  repeat
    c = c + 1
  until c >= 14
  print(c)
end"#, 0);
    assert!(!diags.has_errors(), "repeat_count_12: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"14
"#);
}

#[test]
fn repeat_count_13() {
    let (out, diags) = interpret(r#"fn main() do
  var c = 0
  repeat
    c = c + 1
  until c >= 15
  print(c)
end"#, 0);
    assert!(!diags.has_errors(), "repeat_count_13: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"15
"#);
}

#[test]
fn repeat_count_14() {
    let (out, diags) = interpret(r#"fn main() do
  var c = 0
  repeat
    c = c + 1
  until c >= 16
  print(c)
end"#, 0);
    assert!(!diags.has_errors(), "repeat_count_14: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"16
"#);
}

#[test]
fn repeat_count_15() {
    let (out, diags) = interpret(r#"fn main() do
  var c = 0
  repeat
    c = c + 1
  until c >= 17
  print(c)
end"#, 0);
    assert!(!diags.has_errors(), "repeat_count_15: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"17
"#);
}

#[test]
fn repeat_count_16() {
    let (out, diags) = interpret(r#"fn main() do
  var c = 0
  repeat
    c = c + 1
  until c >= 18
  print(c)
end"#, 0);
    assert!(!diags.has_errors(), "repeat_count_16: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"18
"#);
}

#[test]
fn repeat_count_17() {
    let (out, diags) = interpret(r#"fn main() do
  var c = 0
  repeat
    c = c + 1
  until c >= 19
  print(c)
end"#, 0);
    assert!(!diags.has_errors(), "repeat_count_17: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"19
"#);
}

#[test]
fn repeat_count_18() {
    let (out, diags) = interpret(r#"fn main() do
  var c = 0
  repeat
    c = c + 1
  until c >= 20
  print(c)
end"#, 0);
    assert!(!diags.has_errors(), "repeat_count_18: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"20
"#);
}

#[test]
fn repeat_count_19() {
    let (out, diags) = interpret(r#"fn main() do
  var c = 0
  repeat
    c = c + 1
  until c >= 21
  print(c)
end"#, 0);
    assert!(!diags.has_errors(), "repeat_count_19: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"21
"#);
}

#[test]
fn repeat_count_20() {
    let (out, diags) = interpret(r#"fn main() do
  var c = 0
  repeat
    c = c + 1
  until c >= 22
  print(c)
end"#, 0);
    assert!(!diags.has_errors(), "repeat_count_20: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"22
"#);
}

#[test]
fn repeat_count_21() {
    let (out, diags) = interpret(r#"fn main() do
  var c = 0
  repeat
    c = c + 1
  until c >= 23
  print(c)
end"#, 0);
    assert!(!diags.has_errors(), "repeat_count_21: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"23
"#);
}

#[test]
fn repeat_count_22() {
    let (out, diags) = interpret(r#"fn main() do
  var c = 0
  repeat
    c = c + 1
  until c >= 24
  print(c)
end"#, 0);
    assert!(!diags.has_errors(), "repeat_count_22: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"24
"#);
}

#[test]
fn repeat_count_23() {
    let (out, diags) = interpret(r#"fn main() do
  var c = 0
  repeat
    c = c + 1
  until c >= 25
  print(c)
end"#, 0);
    assert!(!diags.has_errors(), "repeat_count_23: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"25
"#);
}

#[test]
fn repeat_count_24() {
    let (out, diags) = interpret(r#"fn main() do
  var c = 0
  repeat
    c = c + 1
  until c >= 26
  print(c)
end"#, 0);
    assert!(!diags.has_errors(), "repeat_count_24: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"26
"#);
}

#[test]
fn repeat_count_25() {
    let (out, diags) = interpret(r#"fn main() do
  var c = 0
  repeat
    c = c + 1
  until c >= 27
  print(c)
end"#, 0);
    assert!(!diags.has_errors(), "repeat_count_25: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"27
"#);
}

#[test]
fn repeat_count_26() {
    let (out, diags) = interpret(r#"fn main() do
  var c = 0
  repeat
    c = c + 1
  until c >= 28
  print(c)
end"#, 0);
    assert!(!diags.has_errors(), "repeat_count_26: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"28
"#);
}

#[test]
fn repeat_count_27() {
    let (out, diags) = interpret(r#"fn main() do
  var c = 0
  repeat
    c = c + 1
  until c >= 29
  print(c)
end"#, 0);
    assert!(!diags.has_errors(), "repeat_count_27: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"29
"#);
}

#[test]
fn repeat_count_28() {
    let (out, diags) = interpret(r#"fn main() do
  var c = 0
  repeat
    c = c + 1
  until c >= 30
  print(c)
end"#, 0);
    assert!(!diags.has_errors(), "repeat_count_28: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"30
"#);
}

#[test]
fn repeat_count_29() {
    let (out, diags) = interpret(r#"fn main() do
  var c = 0
  repeat
    c = c + 1
  until c >= 31
  print(c)
end"#, 0);
    assert!(!diags.has_errors(), "repeat_count_29: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"31
"#);
}

#[test]
fn repeat_count_30() {
    let (out, diags) = interpret(r#"fn main() do
  var c = 0
  repeat
    c = c + 1
  until c >= 32
  print(c)
end"#, 0);
    assert!(!diags.has_errors(), "repeat_count_30: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"32
"#);
}

#[test]
fn repeat_accumulate_1() {
    let (out, diags) = interpret(r#"fn main() do
  var c = 0
  var s = 0
  repeat
    c = c + 1
    s = s + c
  until c >= 4
  print(c)
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "repeat_accumulate_1: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"4
10
"#);
}

#[test]
fn repeat_accumulate_2() {
    let (out, diags) = interpret(r#"fn main() do
  var c = 0
  var s = 0
  repeat
    c = c + 1
    s = s + c
  until c >= 5
  print(c)
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "repeat_accumulate_2: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"5
15
"#);
}

#[test]
fn repeat_accumulate_3() {
    let (out, diags) = interpret(r#"fn main() do
  var c = 0
  var s = 0
  repeat
    c = c + 1
    s = s + c
  until c >= 6
  print(c)
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "repeat_accumulate_3: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"6
21
"#);
}

#[test]
fn repeat_accumulate_4() {
    let (out, diags) = interpret(r#"fn main() do
  var c = 0
  var s = 0
  repeat
    c = c + 1
    s = s + c
  until c >= 7
  print(c)
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "repeat_accumulate_4: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"7
28
"#);
}

#[test]
fn repeat_accumulate_5() {
    let (out, diags) = interpret(r#"fn main() do
  var c = 0
  var s = 0
  repeat
    c = c + 1
    s = s + c
  until c >= 8
  print(c)
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "repeat_accumulate_5: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"8
36
"#);
}

#[test]
fn repeat_accumulate_6() {
    let (out, diags) = interpret(r#"fn main() do
  var c = 0
  var s = 0
  repeat
    c = c + 1
    s = s + c
  until c >= 3
  print(c)
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "repeat_accumulate_6: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"3
6
"#);
}

#[test]
fn repeat_accumulate_7() {
    let (out, diags) = interpret(r#"fn main() do
  var c = 0
  var s = 0
  repeat
    c = c + 1
    s = s + c
  until c >= 4
  print(c)
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "repeat_accumulate_7: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"4
10
"#);
}

#[test]
fn repeat_accumulate_8() {
    let (out, diags) = interpret(r#"fn main() do
  var c = 0
  var s = 0
  repeat
    c = c + 1
    s = s + c
  until c >= 5
  print(c)
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "repeat_accumulate_8: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"5
15
"#);
}

#[test]
fn repeat_accumulate_9() {
    let (out, diags) = interpret(r#"fn main() do
  var c = 0
  var s = 0
  repeat
    c = c + 1
    s = s + c
  until c >= 6
  print(c)
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "repeat_accumulate_9: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"6
21
"#);
}

#[test]
fn repeat_accumulate_10() {
    let (out, diags) = interpret(r#"fn main() do
  var c = 0
  var s = 0
  repeat
    c = c + 1
    s = s + c
  until c >= 7
  print(c)
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "repeat_accumulate_10: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"7
28
"#);
}

#[test]
fn repeat_accumulate_11() {
    let (out, diags) = interpret(r#"fn main() do
  var c = 0
  var s = 0
  repeat
    c = c + 1
    s = s + c
  until c >= 8
  print(c)
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "repeat_accumulate_11: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"8
36
"#);
}

#[test]
fn repeat_accumulate_12() {
    let (out, diags) = interpret(r#"fn main() do
  var c = 0
  var s = 0
  repeat
    c = c + 1
    s = s + c
  until c >= 3
  print(c)
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "repeat_accumulate_12: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"3
6
"#);
}

#[test]
fn repeat_accumulate_13() {
    let (out, diags) = interpret(r#"fn main() do
  var c = 0
  var s = 0
  repeat
    c = c + 1
    s = s + c
  until c >= 4
  print(c)
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "repeat_accumulate_13: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"4
10
"#);
}

#[test]
fn repeat_accumulate_14() {
    let (out, diags) = interpret(r#"fn main() do
  var c = 0
  var s = 0
  repeat
    c = c + 1
    s = s + c
  until c >= 5
  print(c)
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "repeat_accumulate_14: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"5
15
"#);
}

#[test]
fn repeat_accumulate_15() {
    let (out, diags) = interpret(r#"fn main() do
  var c = 0
  var s = 0
  repeat
    c = c + 1
    s = s + c
  until c >= 6
  print(c)
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "repeat_accumulate_15: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"6
21
"#);
}

#[test]
fn repeat_accumulate_16() {
    let (out, diags) = interpret(r#"fn main() do
  var c = 0
  var s = 0
  repeat
    c = c + 1
    s = s + c
  until c >= 7
  print(c)
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "repeat_accumulate_16: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"7
28
"#);
}

#[test]
fn repeat_accumulate_17() {
    let (out, diags) = interpret(r#"fn main() do
  var c = 0
  var s = 0
  repeat
    c = c + 1
    s = s + c
  until c >= 8
  print(c)
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "repeat_accumulate_17: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"8
36
"#);
}

#[test]
fn repeat_accumulate_18() {
    let (out, diags) = interpret(r#"fn main() do
  var c = 0
  var s = 0
  repeat
    c = c + 1
    s = s + c
  until c >= 3
  print(c)
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "repeat_accumulate_18: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"3
6
"#);
}

#[test]
fn repeat_accumulate_19() {
    let (out, diags) = interpret(r#"fn main() do
  var c = 0
  var s = 0
  repeat
    c = c + 1
    s = s + c
  until c >= 4
  print(c)
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "repeat_accumulate_19: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"4
10
"#);
}

#[test]
fn repeat_accumulate_20() {
    let (out, diags) = interpret(r#"fn main() do
  var c = 0
  var s = 0
  repeat
    c = c + 1
    s = s + c
  until c >= 5
  print(c)
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "repeat_accumulate_20: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"5
15
"#);
}

#[test]
fn repeat_nested_1() {
    let (out, diags) = interpret(r#"fn main() do
  var i = 0
  var hits = 0
  repeat
    var j = 0
    repeat
      j = j + 1
      hits = hits + 1
    until j >= 4
    i = i + 1
  until i >= 3
  print(i)
  print(j)
  print(hits)
end"#, 0);
    assert!(!diags.has_errors(), "repeat_nested_1: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"3
4
12
"#);
}

#[test]
fn repeat_nested_2() {
    let (out, diags) = interpret(r#"fn main() do
  var i = 0
  var hits = 0
  repeat
    var j = 0
    repeat
      j = j + 1
      hits = hits + 1
    until j >= 5
    i = i + 1
  until i >= 4
  print(i)
  print(j)
  print(hits)
end"#, 0);
    assert!(!diags.has_errors(), "repeat_nested_2: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"4
5
20
"#);
}

#[test]
fn repeat_nested_3() {
    let (out, diags) = interpret(r#"fn main() do
  var i = 0
  var hits = 0
  repeat
    var j = 0
    repeat
      j = j + 1
      hits = hits + 1
    until j >= 6
    i = i + 1
  until i >= 2
  print(i)
  print(j)
  print(hits)
end"#, 0);
    assert!(!diags.has_errors(), "repeat_nested_3: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"2
6
12
"#);
}

#[test]
fn repeat_nested_4() {
    let (out, diags) = interpret(r#"fn main() do
  var i = 0
  var hits = 0
  repeat
    var j = 0
    repeat
      j = j + 1
      hits = hits + 1
    until j >= 3
    i = i + 1
  until i >= 3
  print(i)
  print(j)
  print(hits)
end"#, 0);
    assert!(!diags.has_errors(), "repeat_nested_4: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"3
3
9
"#);
}

#[test]
fn repeat_nested_5() {
    let (out, diags) = interpret(r#"fn main() do
  var i = 0
  var hits = 0
  repeat
    var j = 0
    repeat
      j = j + 1
      hits = hits + 1
    until j >= 4
    i = i + 1
  until i >= 4
  print(i)
  print(j)
  print(hits)
end"#, 0);
    assert!(!diags.has_errors(), "repeat_nested_5: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"4
4
16
"#);
}

#[test]
fn repeat_nested_6() {
    let (out, diags) = interpret(r#"fn main() do
  var i = 0
  var hits = 0
  repeat
    var j = 0
    repeat
      j = j + 1
      hits = hits + 1
    until j >= 5
    i = i + 1
  until i >= 2
  print(i)
  print(j)
  print(hits)
end"#, 0);
    assert!(!diags.has_errors(), "repeat_nested_6: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"2
5
10
"#);
}

#[test]
fn repeat_nested_7() {
    let (out, diags) = interpret(r#"fn main() do
  var i = 0
  var hits = 0
  repeat
    var j = 0
    repeat
      j = j + 1
      hits = hits + 1
    until j >= 6
    i = i + 1
  until i >= 3
  print(i)
  print(j)
  print(hits)
end"#, 0);
    assert!(!diags.has_errors(), "repeat_nested_7: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"3
6
18
"#);
}

#[test]
fn repeat_nested_8() {
    let (out, diags) = interpret(r#"fn main() do
  var i = 0
  var hits = 0
  repeat
    var j = 0
    repeat
      j = j + 1
      hits = hits + 1
    until j >= 3
    i = i + 1
  until i >= 4
  print(i)
  print(j)
  print(hits)
end"#, 0);
    assert!(!diags.has_errors(), "repeat_nested_8: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"4
3
12
"#);
}

#[test]
fn repeat_nested_9() {
    let (out, diags) = interpret(r#"fn main() do
  var i = 0
  var hits = 0
  repeat
    var j = 0
    repeat
      j = j + 1
      hits = hits + 1
    until j >= 4
    i = i + 1
  until i >= 2
  print(i)
  print(j)
  print(hits)
end"#, 0);
    assert!(!diags.has_errors(), "repeat_nested_9: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"2
4
8
"#);
}

#[test]
fn repeat_nested_10() {
    let (out, diags) = interpret(r#"fn main() do
  var i = 0
  var hits = 0
  repeat
    var j = 0
    repeat
      j = j + 1
      hits = hits + 1
    until j >= 5
    i = i + 1
  until i >= 3
  print(i)
  print(j)
  print(hits)
end"#, 0);
    assert!(!diags.has_errors(), "repeat_nested_10: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"3
5
15
"#);
}

#[test]
fn repeat_nested_11() {
    let (out, diags) = interpret(r#"fn main() do
  var i = 0
  var hits = 0
  repeat
    var j = 0
    repeat
      j = j + 1
      hits = hits + 1
    until j >= 6
    i = i + 1
  until i >= 4
  print(i)
  print(j)
  print(hits)
end"#, 0);
    assert!(!diags.has_errors(), "repeat_nested_11: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"4
6
24
"#);
}

#[test]
fn repeat_nested_12() {
    let (out, diags) = interpret(r#"fn main() do
  var i = 0
  var hits = 0
  repeat
    var j = 0
    repeat
      j = j + 1
      hits = hits + 1
    until j >= 3
    i = i + 1
  until i >= 2
  print(i)
  print(j)
  print(hits)
end"#, 0);
    assert!(!diags.has_errors(), "repeat_nested_12: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"2
3
6
"#);
}

#[test]
fn repeat_nested_13() {
    let (out, diags) = interpret(r#"fn main() do
  var i = 0
  var hits = 0
  repeat
    var j = 0
    repeat
      j = j + 1
      hits = hits + 1
    until j >= 4
    i = i + 1
  until i >= 3
  print(i)
  print(j)
  print(hits)
end"#, 0);
    assert!(!diags.has_errors(), "repeat_nested_13: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"3
4
12
"#);
}

#[test]
fn repeat_nested_14() {
    let (out, diags) = interpret(r#"fn main() do
  var i = 0
  var hits = 0
  repeat
    var j = 0
    repeat
      j = j + 1
      hits = hits + 1
    until j >= 5
    i = i + 1
  until i >= 4
  print(i)
  print(j)
  print(hits)
end"#, 0);
    assert!(!diags.has_errors(), "repeat_nested_14: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"4
5
20
"#);
}

#[test]
fn repeat_nested_15() {
    let (out, diags) = interpret(r#"fn main() do
  var i = 0
  var hits = 0
  repeat
    var j = 0
    repeat
      j = j + 1
      hits = hits + 1
    until j >= 6
    i = i + 1
  until i >= 2
  print(i)
  print(j)
  print(hits)
end"#, 0);
    assert!(!diags.has_errors(), "repeat_nested_15: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"2
6
12
"#);
}

#[test]
fn repeat_nested_16() {
    let (out, diags) = interpret(r#"fn main() do
  var i = 0
  var hits = 0
  repeat
    var j = 0
    repeat
      j = j + 1
      hits = hits + 1
    until j >= 3
    i = i + 1
  until i >= 3
  print(i)
  print(j)
  print(hits)
end"#, 0);
    assert!(!diags.has_errors(), "repeat_nested_16: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"3
3
9
"#);
}

#[test]
fn repeat_nested_17() {
    let (out, diags) = interpret(r#"fn main() do
  var i = 0
  var hits = 0
  repeat
    var j = 0
    repeat
      j = j + 1
      hits = hits + 1
    until j >= 4
    i = i + 1
  until i >= 4
  print(i)
  print(j)
  print(hits)
end"#, 0);
    assert!(!diags.has_errors(), "repeat_nested_17: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"4
4
16
"#);
}

#[test]
fn repeat_nested_18() {
    let (out, diags) = interpret(r#"fn main() do
  var i = 0
  var hits = 0
  repeat
    var j = 0
    repeat
      j = j + 1
      hits = hits + 1
    until j >= 5
    i = i + 1
  until i >= 2
  print(i)
  print(j)
  print(hits)
end"#, 0);
    assert!(!diags.has_errors(), "repeat_nested_18: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"2
5
10
"#);
}

#[test]
fn repeat_nested_19() {
    let (out, diags) = interpret(r#"fn main() do
  var i = 0
  var hits = 0
  repeat
    var j = 0
    repeat
      j = j + 1
      hits = hits + 1
    until j >= 6
    i = i + 1
  until i >= 3
  print(i)
  print(j)
  print(hits)
end"#, 0);
    assert!(!diags.has_errors(), "repeat_nested_19: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"3
6
18
"#);
}

#[test]
fn repeat_nested_20() {
    let (out, diags) = interpret(r#"fn main() do
  var i = 0
  var hits = 0
  repeat
    var j = 0
    repeat
      j = j + 1
      hits = hits + 1
    until j >= 3
    i = i + 1
  until i >= 4
  print(i)
  print(j)
  print(hits)
end"#, 0);
    assert!(!diags.has_errors(), "repeat_nested_20: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"4
3
12
"#);
}

#[test]
fn repeat_break_1() {
    let (out, diags) = interpret(r#"fn main() do
  var c = 0
  repeat
    c = c + 1
    if c == 3 then break end
  until false
  print(c)
end"#, 0);
    assert!(!diags.has_errors(), "repeat_break_1: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"3
"#);
}

#[test]
fn repeat_break_2() {
    let (out, diags) = interpret(r#"fn main() do
  var c = 0
  repeat
    c = c + 1
    if c == 4 then break end
  until false
  print(c)
end"#, 0);
    assert!(!diags.has_errors(), "repeat_break_2: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"4
"#);
}

#[test]
fn repeat_break_3() {
    let (out, diags) = interpret(r#"fn main() do
  var c = 0
  repeat
    c = c + 1
    if c == 5 then break end
  until false
  print(c)
end"#, 0);
    assert!(!diags.has_errors(), "repeat_break_3: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"5
"#);
}

#[test]
fn repeat_break_4() {
    let (out, diags) = interpret(r#"fn main() do
  var c = 0
  repeat
    c = c + 1
    if c == 2 then break end
  until false
  print(c)
end"#, 0);
    assert!(!diags.has_errors(), "repeat_break_4: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"2
"#);
}

#[test]
fn repeat_break_5() {
    let (out, diags) = interpret(r#"fn main() do
  var c = 0
  repeat
    c = c + 1
    if c == 3 then break end
  until false
  print(c)
end"#, 0);
    assert!(!diags.has_errors(), "repeat_break_5: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"3
"#);
}

#[test]
fn repeat_break_6() {
    let (out, diags) = interpret(r#"fn main() do
  var c = 0
  repeat
    c = c + 1
    if c == 4 then break end
  until false
  print(c)
end"#, 0);
    assert!(!diags.has_errors(), "repeat_break_6: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"4
"#);
}

#[test]
fn repeat_break_7() {
    let (out, diags) = interpret(r#"fn main() do
  var c = 0
  repeat
    c = c + 1
    if c == 5 then break end
  until false
  print(c)
end"#, 0);
    assert!(!diags.has_errors(), "repeat_break_7: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"5
"#);
}

#[test]
fn repeat_break_8() {
    let (out, diags) = interpret(r#"fn main() do
  var c = 0
  repeat
    c = c + 1
    if c == 2 then break end
  until false
  print(c)
end"#, 0);
    assert!(!diags.has_errors(), "repeat_break_8: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"2
"#);
}

#[test]
fn repeat_break_9() {
    let (out, diags) = interpret(r#"fn main() do
  var c = 0
  repeat
    c = c + 1
    if c == 3 then break end
  until false
  print(c)
end"#, 0);
    assert!(!diags.has_errors(), "repeat_break_9: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"3
"#);
}

#[test]
fn repeat_break_10() {
    let (out, diags) = interpret(r#"fn main() do
  var c = 0
  repeat
    c = c + 1
    if c == 4 then break end
  until false
  print(c)
end"#, 0);
    assert!(!diags.has_errors(), "repeat_break_10: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"4
"#);
}

#[test]
fn repeat_break_11() {
    let (out, diags) = interpret(r#"fn main() do
  var c = 0
  repeat
    c = c + 1
    if c == 5 then break end
  until false
  print(c)
end"#, 0);
    assert!(!diags.has_errors(), "repeat_break_11: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"5
"#);
}

#[test]
fn repeat_break_12() {
    let (out, diags) = interpret(r#"fn main() do
  var c = 0
  repeat
    c = c + 1
    if c == 2 then break end
  until false
  print(c)
end"#, 0);
    assert!(!diags.has_errors(), "repeat_break_12: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"2
"#);
}

#[test]
fn repeat_break_13() {
    let (out, diags) = interpret(r#"fn main() do
  var c = 0
  repeat
    c = c + 1
    if c == 3 then break end
  until false
  print(c)
end"#, 0);
    assert!(!diags.has_errors(), "repeat_break_13: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"3
"#);
}

#[test]
fn repeat_break_14() {
    let (out, diags) = interpret(r#"fn main() do
  var c = 0
  repeat
    c = c + 1
    if c == 4 then break end
  until false
  print(c)
end"#, 0);
    assert!(!diags.has_errors(), "repeat_break_14: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"4
"#);
}

#[test]
fn repeat_break_15() {
    let (out, diags) = interpret(r#"fn main() do
  var c = 0
  repeat
    c = c + 1
    if c == 5 then break end
  until false
  print(c)
end"#, 0);
    assert!(!diags.has_errors(), "repeat_break_15: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"5
"#);
}

#[test]
fn repeat_arith_cond_1() {
    let (out, diags) = interpret(r#"fn main() do
  var c = 0
  repeat
    c = c + 2
  until c * c >= 100
  print(c)
end"#, 0);
    assert!(!diags.has_errors(), "repeat_arith_cond_1: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"10
"#);
}

#[test]
fn repeat_arith_cond_2() {
    let (out, diags) = interpret(r#"fn main() do
  var c = 0
  repeat
    c = c + 4
  until c * c >= 100
  print(c)
end"#, 0);
    assert!(!diags.has_errors(), "repeat_arith_cond_2: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"12
"#);
}

#[test]
fn repeat_arith_cond_3() {
    let (out, diags) = interpret(r#"fn main() do
  var c = 0
  repeat
    c = c + 6
  until c * c >= 100
  print(c)
end"#, 0);
    assert!(!diags.has_errors(), "repeat_arith_cond_3: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"12
"#);
}

#[test]
fn repeat_arith_cond_4() {
    let (out, diags) = interpret(r#"fn main() do
  var c = 0
  repeat
    c = c + 8
  until c * c >= 100
  print(c)
end"#, 0);
    assert!(!diags.has_errors(), "repeat_arith_cond_4: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"16
"#);
}

#[test]
fn repeat_arith_cond_5() {
    let (out, diags) = interpret(r#"fn main() do
  var c = 0
  repeat
    c = c + 10
  until c * c >= 100
  print(c)
end"#, 0);
    assert!(!diags.has_errors(), "repeat_arith_cond_5: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"10
"#);
}

#[test]
fn repeat_arith_cond_6() {
    let (out, diags) = interpret(r#"fn main() do
  var c = 0
  repeat
    c = c + 12
  until c * c >= 100
  print(c)
end"#, 0);
    assert!(!diags.has_errors(), "repeat_arith_cond_6: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"12
"#);
}

#[test]
fn repeat_arith_cond_7() {
    let (out, diags) = interpret(r#"fn main() do
  var c = 0
  repeat
    c = c + 14
  until c * c >= 100
  print(c)
end"#, 0);
    assert!(!diags.has_errors(), "repeat_arith_cond_7: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"14
"#);
}

#[test]
fn repeat_arith_cond_8() {
    let (out, diags) = interpret(r#"fn main() do
  var c = 0
  repeat
    c = c + 16
  until c * c >= 100
  print(c)
end"#, 0);
    assert!(!diags.has_errors(), "repeat_arith_cond_8: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"16
"#);
}

#[test]
fn repeat_arith_cond_9() {
    let (out, diags) = interpret(r#"fn main() do
  var c = 0
  repeat
    c = c + 18
  until c * c >= 100
  print(c)
end"#, 0);
    assert!(!diags.has_errors(), "repeat_arith_cond_9: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"18
"#);
}

#[test]
fn repeat_arith_cond_10() {
    let (out, diags) = interpret(r#"fn main() do
  var c = 0
  repeat
    c = c + 20
  until c * c >= 100
  print(c)
end"#, 0);
    assert!(!diags.has_errors(), "repeat_arith_cond_10: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"20
"#);
}

#[test]
fn repeat_arith_cond_11() {
    let (out, diags) = interpret(r#"fn main() do
  var c = 0
  repeat
    c = c + 22
  until c * c >= 100
  print(c)
end"#, 0);
    assert!(!diags.has_errors(), "repeat_arith_cond_11: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"22
"#);
}

#[test]
fn repeat_arith_cond_12() {
    let (out, diags) = interpret(r#"fn main() do
  var c = 0
  repeat
    c = c + 24
  until c * c >= 100
  print(c)
end"#, 0);
    assert!(!diags.has_errors(), "repeat_arith_cond_12: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"24
"#);
}

#[test]
fn repeat_arith_cond_13() {
    let (out, diags) = interpret(r#"fn main() do
  var c = 0
  repeat
    c = c + 26
  until c * c >= 100
  print(c)
end"#, 0);
    assert!(!diags.has_errors(), "repeat_arith_cond_13: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"26
"#);
}

#[test]
fn repeat_arith_cond_14() {
    let (out, diags) = interpret(r#"fn main() do
  var c = 0
  repeat
    c = c + 28
  until c * c >= 100
  print(c)
end"#, 0);
    assert!(!diags.has_errors(), "repeat_arith_cond_14: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"28
"#);
}

#[test]
fn repeat_arith_cond_15() {
    let (out, diags) = interpret(r#"fn main() do
  var c = 0
  repeat
    c = c + 30
  until c * c >= 100
  print(c)
end"#, 0);
    assert!(!diags.has_errors(), "repeat_arith_cond_15: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"30
"#);
}

#[test]
fn repeat_countdown_1() {
    let (out, diags) = interpret(r#"fn main() do
  var n = 6
  var steps = 0
  repeat
    n = n - 1
    steps = steps + 1
  until n == 0
  print(n)
  print(steps)
end"#, 0);
    assert!(!diags.has_errors(), "repeat_countdown_1: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"0
6
"#);
}

#[test]
fn repeat_countdown_2() {
    let (out, diags) = interpret(r#"fn main() do
  var n = 7
  var steps = 0
  repeat
    n = n - 1
    steps = steps + 1
  until n == 0
  print(n)
  print(steps)
end"#, 0);
    assert!(!diags.has_errors(), "repeat_countdown_2: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"0
7
"#);
}

#[test]
fn repeat_countdown_3() {
    let (out, diags) = interpret(r#"fn main() do
  var n = 8
  var steps = 0
  repeat
    n = n - 1
    steps = steps + 1
  until n == 0
  print(n)
  print(steps)
end"#, 0);
    assert!(!diags.has_errors(), "repeat_countdown_3: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"0
8
"#);
}

#[test]
fn repeat_countdown_4() {
    let (out, diags) = interpret(r#"fn main() do
  var n = 9
  var steps = 0
  repeat
    n = n - 1
    steps = steps + 1
  until n == 0
  print(n)
  print(steps)
end"#, 0);
    assert!(!diags.has_errors(), "repeat_countdown_4: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"0
9
"#);
}

#[test]
fn repeat_countdown_5() {
    let (out, diags) = interpret(r#"fn main() do
  var n = 10
  var steps = 0
  repeat
    n = n - 1
    steps = steps + 1
  until n == 0
  print(n)
  print(steps)
end"#, 0);
    assert!(!diags.has_errors(), "repeat_countdown_5: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"0
10
"#);
}

#[test]
fn repeat_countdown_6() {
    let (out, diags) = interpret(r#"fn main() do
  var n = 11
  var steps = 0
  repeat
    n = n - 1
    steps = steps + 1
  until n == 0
  print(n)
  print(steps)
end"#, 0);
    assert!(!diags.has_errors(), "repeat_countdown_6: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"0
11
"#);
}

#[test]
fn repeat_countdown_7() {
    let (out, diags) = interpret(r#"fn main() do
  var n = 12
  var steps = 0
  repeat
    n = n - 1
    steps = steps + 1
  until n == 0
  print(n)
  print(steps)
end"#, 0);
    assert!(!diags.has_errors(), "repeat_countdown_7: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"0
12
"#);
}

#[test]
fn repeat_countdown_8() {
    let (out, diags) = interpret(r#"fn main() do
  var n = 13
  var steps = 0
  repeat
    n = n - 1
    steps = steps + 1
  until n == 0
  print(n)
  print(steps)
end"#, 0);
    assert!(!diags.has_errors(), "repeat_countdown_8: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"0
13
"#);
}

#[test]
fn repeat_countdown_9() {
    let (out, diags) = interpret(r#"fn main() do
  var n = 14
  var steps = 0
  repeat
    n = n - 1
    steps = steps + 1
  until n == 0
  print(n)
  print(steps)
end"#, 0);
    assert!(!diags.has_errors(), "repeat_countdown_9: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"0
14
"#);
}

#[test]
fn repeat_countdown_10() {
    let (out, diags) = interpret(r#"fn main() do
  var n = 15
  var steps = 0
  repeat
    n = n - 1
    steps = steps + 1
  until n == 0
  print(n)
  print(steps)
end"#, 0);
    assert!(!diags.has_errors(), "repeat_countdown_10: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"0
15
"#);
}

#[test]
fn repeat_countdown_11() {
    let (out, diags) = interpret(r#"fn main() do
  var n = 16
  var steps = 0
  repeat
    n = n - 1
    steps = steps + 1
  until n == 0
  print(n)
  print(steps)
end"#, 0);
    assert!(!diags.has_errors(), "repeat_countdown_11: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"0
16
"#);
}

#[test]
fn repeat_countdown_12() {
    let (out, diags) = interpret(r#"fn main() do
  var n = 17
  var steps = 0
  repeat
    n = n - 1
    steps = steps + 1
  until n == 0
  print(n)
  print(steps)
end"#, 0);
    assert!(!diags.has_errors(), "repeat_countdown_12: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"0
17
"#);
}

#[test]
fn repeat_countdown_13() {
    let (out, diags) = interpret(r#"fn main() do
  var n = 18
  var steps = 0
  repeat
    n = n - 1
    steps = steps + 1
  until n == 0
  print(n)
  print(steps)
end"#, 0);
    assert!(!diags.has_errors(), "repeat_countdown_13: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"0
18
"#);
}

#[test]
fn repeat_countdown_14() {
    let (out, diags) = interpret(r#"fn main() do
  var n = 19
  var steps = 0
  repeat
    n = n - 1
    steps = steps + 1
  until n == 0
  print(n)
  print(steps)
end"#, 0);
    assert!(!diags.has_errors(), "repeat_countdown_14: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"0
19
"#);
}

#[test]
fn repeat_countdown_15() {
    let (out, diags) = interpret(r#"fn main() do
  var n = 20
  var steps = 0
  repeat
    n = n - 1
    steps = steps + 1
  until n == 0
  print(n)
  print(steps)
end"#, 0);
    assert!(!diags.has_errors(), "repeat_countdown_15: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"0
20
"#);
}

#[test]
fn repeat_codegen_1() {
    let (rust, diags) = compile_to_rust(r#"fn main() do
  var c = 0
  repeat
    c = c + 1
  until c >= 4
  print(c)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("loop {"));
    assert!(rust.contains("if c >= 4 {"));
    assert!(rust.contains("break;"));
    assert!(!rust.contains("while"));
}

#[test]
fn repeat_codegen_2() {
    let (rust, diags) = compile_to_rust(r#"fn main() do
  var c = 0
  repeat
    c = c + 1
  until c >= 5
  print(c)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("loop {"));
    assert!(rust.contains("if c >= 5 {"));
    assert!(rust.contains("break;"));
    assert!(!rust.contains("while"));
}

#[test]
fn repeat_codegen_3() {
    let (rust, diags) = compile_to_rust(r#"fn main() do
  var c = 0
  repeat
    c = c + 1
  until c >= 6
  print(c)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("loop {"));
    assert!(rust.contains("if c >= 6 {"));
    assert!(rust.contains("break;"));
    assert!(!rust.contains("while"));
}

#[test]
fn repeat_codegen_4() {
    let (rust, diags) = compile_to_rust(r#"fn main() do
  var c = 0
  repeat
    c = c + 1
  until c >= 7
  print(c)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("loop {"));
    assert!(rust.contains("if c >= 7 {"));
    assert!(rust.contains("break;"));
    assert!(!rust.contains("while"));
}

#[test]
fn repeat_codegen_5() {
    let (rust, diags) = compile_to_rust(r#"fn main() do
  var c = 0
  repeat
    c = c + 1
  until c >= 8
  print(c)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("loop {"));
    assert!(rust.contains("if c >= 8 {"));
    assert!(rust.contains("break;"));
    assert!(!rust.contains("while"));
}

#[test]
fn repeat_codegen_6() {
    let (rust, diags) = compile_to_rust(r#"fn main() do
  var c = 0
  repeat
    c = c + 1
  until c >= 9
  print(c)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("loop {"));
    assert!(rust.contains("if c >= 9 {"));
    assert!(rust.contains("break;"));
    assert!(!rust.contains("while"));
}

#[test]
fn repeat_codegen_7() {
    let (rust, diags) = compile_to_rust(r#"fn main() do
  var c = 0
  repeat
    c = c + 1
  until c >= 10
  print(c)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("loop {"));
    assert!(rust.contains("if c >= 10 {"));
    assert!(rust.contains("break;"));
    assert!(!rust.contains("while"));
}

#[test]
fn repeat_codegen_8() {
    let (rust, diags) = compile_to_rust(r#"fn main() do
  var c = 0
  repeat
    c = c + 1
  until c >= 11
  print(c)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("loop {"));
    assert!(rust.contains("if c >= 11 {"));
    assert!(rust.contains("break;"));
    assert!(!rust.contains("while"));
}

#[test]
fn repeat_codegen_9() {
    let (rust, diags) = compile_to_rust(r#"fn main() do
  var c = 0
  repeat
    c = c + 1
  until c >= 12
  print(c)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("loop {"));
    assert!(rust.contains("if c >= 12 {"));
    assert!(rust.contains("break;"));
    assert!(!rust.contains("while"));
}

#[test]
fn repeat_codegen_10() {
    let (rust, diags) = compile_to_rust(r#"fn main() do
  var c = 0
  repeat
    c = c + 1
  until c >= 13
  print(c)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("loop {"));
    assert!(rust.contains("if c >= 13 {"));
    assert!(rust.contains("break;"));
    assert!(!rust.contains("while"));
}

#[test]
fn repeat_codegen_11() {
    let (rust, diags) = compile_to_rust(r#"fn main() do
  var c = 0
  repeat
    c = c + 1
  until c >= 14
  print(c)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("loop {"));
    assert!(rust.contains("if c >= 14 {"));
    assert!(rust.contains("break;"));
    assert!(!rust.contains("while"));
}

#[test]
fn repeat_codegen_12() {
    let (rust, diags) = compile_to_rust(r#"fn main() do
  var c = 0
  repeat
    c = c + 1
  until c >= 15
  print(c)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("loop {"));
    assert!(rust.contains("if c >= 15 {"));
    assert!(rust.contains("break;"));
    assert!(!rust.contains("while"));
}

#[test]
fn repeat_codegen_13() {
    let (rust, diags) = compile_to_rust(r#"fn main() do
  var c = 0
  repeat
    c = c + 1
  until c >= 16
  print(c)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("loop {"));
    assert!(rust.contains("if c >= 16 {"));
    assert!(rust.contains("break;"));
    assert!(!rust.contains("while"));
}

#[test]
fn repeat_codegen_14() {
    let (rust, diags) = compile_to_rust(r#"fn main() do
  var c = 0
  repeat
    c = c + 1
  until c >= 17
  print(c)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("loop {"));
    assert!(rust.contains("if c >= 17 {"));
    assert!(rust.contains("break;"));
    assert!(!rust.contains("while"));
}

#[test]
fn repeat_codegen_15() {
    let (rust, diags) = compile_to_rust(r#"fn main() do
  var c = 0
  repeat
    c = c + 1
  until c >= 18
  print(c)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("loop {"));
    assert!(rust.contains("if c >= 18 {"));
    assert!(rust.contains("break;"));
    assert!(!rust.contains("while"));
}
