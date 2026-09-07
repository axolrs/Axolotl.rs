// Owner: PascalElixir / axolrs (GitHub org)
// File: Auto-generated tests for numeric for loops (the lowered Range form), loop bounds, break/continue, nested loops, HIR range lowering, and range codegen.

use axolc_core::interpret;
use axolc_core::compile_to_rust;
use axolc_core::lower;
use axolc_core::ast::{Expr, Stmt};
use axolc_core::hir::Item;

#[test]
fn range_for_sum_1() {
    let (out, diags) = interpret(r#"fn main() do
  var s = 0
  for i in 4 do
    s = s + i
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "range_for_sum_1: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"6
"#);
}

#[test]
fn range_for_sum_2() {
    let (out, diags) = interpret(r#"fn main() do
  var s = 0
  for i in 5 do
    s = s + i
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "range_for_sum_2: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"10
"#);
}

#[test]
fn range_for_sum_3() {
    let (out, diags) = interpret(r#"fn main() do
  var s = 0
  for i in 6 do
    s = s + i
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "range_for_sum_3: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"15
"#);
}

#[test]
fn range_for_sum_4() {
    let (out, diags) = interpret(r#"fn main() do
  var s = 0
  for i in 7 do
    s = s + i
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "range_for_sum_4: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"21
"#);
}

#[test]
fn range_for_sum_5() {
    let (out, diags) = interpret(r#"fn main() do
  var s = 0
  for i in 8 do
    s = s + i
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "range_for_sum_5: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"28
"#);
}

#[test]
fn range_for_sum_6() {
    let (out, diags) = interpret(r#"fn main() do
  var s = 0
  for i in 9 do
    s = s + i
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "range_for_sum_6: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"36
"#);
}

#[test]
fn range_for_sum_7() {
    let (out, diags) = interpret(r#"fn main() do
  var s = 0
  for i in 10 do
    s = s + i
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "range_for_sum_7: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"45
"#);
}

#[test]
fn range_for_sum_8() {
    let (out, diags) = interpret(r#"fn main() do
  var s = 0
  for i in 11 do
    s = s + i
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "range_for_sum_8: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"55
"#);
}

#[test]
fn range_for_sum_9() {
    let (out, diags) = interpret(r#"fn main() do
  var s = 0
  for i in 12 do
    s = s + i
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "range_for_sum_9: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"66
"#);
}

#[test]
fn range_for_sum_10() {
    let (out, diags) = interpret(r#"fn main() do
  var s = 0
  for i in 13 do
    s = s + i
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "range_for_sum_10: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"78
"#);
}

#[test]
fn range_for_sum_11() {
    let (out, diags) = interpret(r#"fn main() do
  var s = 0
  for i in 14 do
    s = s + i
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "range_for_sum_11: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"91
"#);
}

#[test]
fn range_for_sum_12() {
    let (out, diags) = interpret(r#"fn main() do
  var s = 0
  for i in 15 do
    s = s + i
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "range_for_sum_12: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"105
"#);
}

#[test]
fn range_for_sum_13() {
    let (out, diags) = interpret(r#"fn main() do
  var s = 0
  for i in 16 do
    s = s + i
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "range_for_sum_13: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"120
"#);
}

#[test]
fn range_for_sum_14() {
    let (out, diags) = interpret(r#"fn main() do
  var s = 0
  for i in 17 do
    s = s + i
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "range_for_sum_14: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"136
"#);
}

#[test]
fn range_for_sum_15() {
    let (out, diags) = interpret(r#"fn main() do
  var s = 0
  for i in 18 do
    s = s + i
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "range_for_sum_15: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"153
"#);
}

#[test]
fn range_for_sum_16() {
    let (out, diags) = interpret(r#"fn main() do
  var s = 0
  for i in 19 do
    s = s + i
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "range_for_sum_16: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"171
"#);
}

#[test]
fn range_for_sum_17() {
    let (out, diags) = interpret(r#"fn main() do
  var s = 0
  for i in 20 do
    s = s + i
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "range_for_sum_17: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"190
"#);
}

#[test]
fn range_for_sum_18() {
    let (out, diags) = interpret(r#"fn main() do
  var s = 0
  for i in 21 do
    s = s + i
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "range_for_sum_18: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"210
"#);
}

#[test]
fn range_for_sum_19() {
    let (out, diags) = interpret(r#"fn main() do
  var s = 0
  for i in 22 do
    s = s + i
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "range_for_sum_19: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"231
"#);
}

#[test]
fn range_for_sum_20() {
    let (out, diags) = interpret(r#"fn main() do
  var s = 0
  for i in 23 do
    s = s + i
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "range_for_sum_20: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"253
"#);
}

#[test]
fn range_for_sum_21() {
    let (out, diags) = interpret(r#"fn main() do
  var s = 0
  for i in 24 do
    s = s + i
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "range_for_sum_21: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"276
"#);
}

#[test]
fn range_for_sum_22() {
    let (out, diags) = interpret(r#"fn main() do
  var s = 0
  for i in 25 do
    s = s + i
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "range_for_sum_22: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"300
"#);
}

#[test]
fn range_for_sum_23() {
    let (out, diags) = interpret(r#"fn main() do
  var s = 0
  for i in 26 do
    s = s + i
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "range_for_sum_23: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"325
"#);
}

#[test]
fn range_for_sum_24() {
    let (out, diags) = interpret(r#"fn main() do
  var s = 0
  for i in 27 do
    s = s + i
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "range_for_sum_24: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"351
"#);
}

#[test]
fn range_for_sum_25() {
    let (out, diags) = interpret(r#"fn main() do
  var s = 0
  for i in 28 do
    s = s + i
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "range_for_sum_25: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"378
"#);
}

#[test]
fn range_for_sum_26() {
    let (out, diags) = interpret(r#"fn main() do
  var s = 0
  for i in 29 do
    s = s + i
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "range_for_sum_26: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"406
"#);
}

#[test]
fn range_for_sum_27() {
    let (out, diags) = interpret(r#"fn main() do
  var s = 0
  for i in 30 do
    s = s + i
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "range_for_sum_27: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"435
"#);
}

#[test]
fn range_for_sum_28() {
    let (out, diags) = interpret(r#"fn main() do
  var s = 0
  for i in 31 do
    s = s + i
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "range_for_sum_28: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"465
"#);
}

#[test]
fn range_for_sum_29() {
    let (out, diags) = interpret(r#"fn main() do
  var s = 0
  for i in 32 do
    s = s + i
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "range_for_sum_29: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"496
"#);
}

#[test]
fn range_for_sum_30() {
    let (out, diags) = interpret(r#"fn main() do
  var s = 0
  for i in 33 do
    s = s + i
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "range_for_sum_30: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"528
"#);
}

#[test]
fn range_for_sum_31() {
    let (out, diags) = interpret(r#"fn main() do
  var s = 0
  for i in 34 do
    s = s + i
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "range_for_sum_31: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"561
"#);
}

#[test]
fn range_for_sum_32() {
    let (out, diags) = interpret(r#"fn main() do
  var s = 0
  for i in 35 do
    s = s + i
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "range_for_sum_32: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"595
"#);
}

#[test]
fn range_for_sum_33() {
    let (out, diags) = interpret(r#"fn main() do
  var s = 0
  for i in 36 do
    s = s + i
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "range_for_sum_33: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"630
"#);
}

#[test]
fn range_for_sum_34() {
    let (out, diags) = interpret(r#"fn main() do
  var s = 0
  for i in 37 do
    s = s + i
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "range_for_sum_34: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"666
"#);
}

#[test]
fn range_for_sum_35() {
    let (out, diags) = interpret(r#"fn main() do
  var s = 0
  for i in 38 do
    s = s + i
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "range_for_sum_35: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"703
"#);
}

#[test]
fn range_for_sum_36() {
    let (out, diags) = interpret(r#"fn main() do
  var s = 0
  for i in 39 do
    s = s + i
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "range_for_sum_36: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"741
"#);
}

#[test]
fn range_for_sum_37() {
    let (out, diags) = interpret(r#"fn main() do
  var s = 0
  for i in 40 do
    s = s + i
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "range_for_sum_37: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"780
"#);
}

#[test]
fn range_for_sum_38() {
    let (out, diags) = interpret(r#"fn main() do
  var s = 0
  for i in 41 do
    s = s + i
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "range_for_sum_38: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"820
"#);
}

#[test]
fn range_for_sum_39() {
    let (out, diags) = interpret(r#"fn main() do
  var s = 0
  for i in 42 do
    s = s + i
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "range_for_sum_39: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"861
"#);
}

#[test]
fn range_for_sum_40() {
    let (out, diags) = interpret(r#"fn main() do
  var s = 0
  for i in 43 do
    s = s + i
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "range_for_sum_40: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"903
"#);
}

#[test]
fn range_for_print_1() {
    let (out, diags) = interpret(r#"fn main() do
  for i in 3 do
    print(i)
  end
end"#, 0);
    assert!(!diags.has_errors(), "range_for_print_1: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"0
1
2
"#);
}

#[test]
fn range_for_print_2() {
    let (out, diags) = interpret(r#"fn main() do
  for i in 4 do
    print(i)
  end
end"#, 0);
    assert!(!diags.has_errors(), "range_for_print_2: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"0
1
2
3
"#);
}

#[test]
fn range_for_print_3() {
    let (out, diags) = interpret(r#"fn main() do
  for i in 5 do
    print(i)
  end
end"#, 0);
    assert!(!diags.has_errors(), "range_for_print_3: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"0
1
2
3
4
"#);
}

#[test]
fn range_for_print_4() {
    let (out, diags) = interpret(r#"fn main() do
  for i in 6 do
    print(i)
  end
end"#, 0);
    assert!(!diags.has_errors(), "range_for_print_4: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"0
1
2
3
4
5
"#);
}

#[test]
fn range_for_print_5() {
    let (out, diags) = interpret(r#"fn main() do
  for i in 2 do
    print(i)
  end
end"#, 0);
    assert!(!diags.has_errors(), "range_for_print_5: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"0
1
"#);
}

#[test]
fn range_for_print_6() {
    let (out, diags) = interpret(r#"fn main() do
  for i in 3 do
    print(i)
  end
end"#, 0);
    assert!(!diags.has_errors(), "range_for_print_6: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"0
1
2
"#);
}

#[test]
fn range_for_print_7() {
    let (out, diags) = interpret(r#"fn main() do
  for i in 4 do
    print(i)
  end
end"#, 0);
    assert!(!diags.has_errors(), "range_for_print_7: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"0
1
2
3
"#);
}

#[test]
fn range_for_print_8() {
    let (out, diags) = interpret(r#"fn main() do
  for i in 5 do
    print(i)
  end
end"#, 0);
    assert!(!diags.has_errors(), "range_for_print_8: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"0
1
2
3
4
"#);
}

#[test]
fn range_for_print_9() {
    let (out, diags) = interpret(r#"fn main() do
  for i in 6 do
    print(i)
  end
end"#, 0);
    assert!(!diags.has_errors(), "range_for_print_9: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"0
1
2
3
4
5
"#);
}

#[test]
fn range_for_print_10() {
    let (out, diags) = interpret(r#"fn main() do
  for i in 2 do
    print(i)
  end
end"#, 0);
    assert!(!diags.has_errors(), "range_for_print_10: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"0
1
"#);
}

#[test]
fn range_for_print_11() {
    let (out, diags) = interpret(r#"fn main() do
  for i in 3 do
    print(i)
  end
end"#, 0);
    assert!(!diags.has_errors(), "range_for_print_11: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"0
1
2
"#);
}

#[test]
fn range_for_print_12() {
    let (out, diags) = interpret(r#"fn main() do
  for i in 4 do
    print(i)
  end
end"#, 0);
    assert!(!diags.has_errors(), "range_for_print_12: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"0
1
2
3
"#);
}

#[test]
fn range_for_print_13() {
    let (out, diags) = interpret(r#"fn main() do
  for i in 5 do
    print(i)
  end
end"#, 0);
    assert!(!diags.has_errors(), "range_for_print_13: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"0
1
2
3
4
"#);
}

#[test]
fn range_for_print_14() {
    let (out, diags) = interpret(r#"fn main() do
  for i in 6 do
    print(i)
  end
end"#, 0);
    assert!(!diags.has_errors(), "range_for_print_14: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"0
1
2
3
4
5
"#);
}

#[test]
fn range_for_print_15() {
    let (out, diags) = interpret(r#"fn main() do
  for i in 2 do
    print(i)
  end
end"#, 0);
    assert!(!diags.has_errors(), "range_for_print_15: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"0
1
"#);
}

#[test]
fn range_for_print_16() {
    let (out, diags) = interpret(r#"fn main() do
  for i in 3 do
    print(i)
  end
end"#, 0);
    assert!(!diags.has_errors(), "range_for_print_16: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"0
1
2
"#);
}

#[test]
fn range_for_print_17() {
    let (out, diags) = interpret(r#"fn main() do
  for i in 4 do
    print(i)
  end
end"#, 0);
    assert!(!diags.has_errors(), "range_for_print_17: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"0
1
2
3
"#);
}

#[test]
fn range_for_print_18() {
    let (out, diags) = interpret(r#"fn main() do
  for i in 5 do
    print(i)
  end
end"#, 0);
    assert!(!diags.has_errors(), "range_for_print_18: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"0
1
2
3
4
"#);
}

#[test]
fn range_for_print_19() {
    let (out, diags) = interpret(r#"fn main() do
  for i in 6 do
    print(i)
  end
end"#, 0);
    assert!(!diags.has_errors(), "range_for_print_19: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"0
1
2
3
4
5
"#);
}

#[test]
fn range_for_print_20() {
    let (out, diags) = interpret(r#"fn main() do
  for i in 2 do
    print(i)
  end
end"#, 0);
    assert!(!diags.has_errors(), "range_for_print_20: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"0
1
"#);
}

#[test]
fn range_for_zero_1() {
    let (out, diags) = interpret(r#"fn main() do
  var s = 1
  for i in 0 do
    s = s + 1
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "range_for_zero_1: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"1
"#);
}

#[test]
fn range_for_zero_2() {
    let (out, diags) = interpret(r#"fn main() do
  var s = 2
  for i in 0 do
    s = s + 1
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "range_for_zero_2: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"2
"#);
}

#[test]
fn range_for_zero_3() {
    let (out, diags) = interpret(r#"fn main() do
  var s = 3
  for i in 0 do
    s = s + 1
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "range_for_zero_3: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"3
"#);
}

#[test]
fn range_for_zero_4() {
    let (out, diags) = interpret(r#"fn main() do
  var s = 4
  for i in 0 do
    s = s + 1
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "range_for_zero_4: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"4
"#);
}

#[test]
fn range_for_zero_5() {
    let (out, diags) = interpret(r#"fn main() do
  var s = 5
  for i in 0 do
    s = s + 1
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "range_for_zero_5: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"5
"#);
}

#[test]
fn range_for_zero_6() {
    let (out, diags) = interpret(r#"fn main() do
  var s = 6
  for i in 0 do
    s = s + 1
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "range_for_zero_6: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"6
"#);
}

#[test]
fn range_for_zero_7() {
    let (out, diags) = interpret(r#"fn main() do
  var s = 7
  for i in 0 do
    s = s + 1
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "range_for_zero_7: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"7
"#);
}

#[test]
fn range_for_zero_8() {
    let (out, diags) = interpret(r#"fn main() do
  var s = 8
  for i in 0 do
    s = s + 1
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "range_for_zero_8: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"8
"#);
}

#[test]
fn range_for_zero_9() {
    let (out, diags) = interpret(r#"fn main() do
  var s = 9
  for i in 0 do
    s = s + 1
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "range_for_zero_9: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"9
"#);
}

#[test]
fn range_for_zero_10() {
    let (out, diags) = interpret(r#"fn main() do
  var s = 10
  for i in 0 do
    s = s + 1
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "range_for_zero_10: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"10
"#);
}

#[test]
fn range_for_nested_1() {
    let (out, diags) = interpret(r#"fn main() do
  var s = 0
  for i in 4 do
    for j in 3 do
      s = s + j
    end
    s = s + i
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "range_for_nested_1: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"18
"#);
}

#[test]
fn range_for_nested_2() {
    let (out, diags) = interpret(r#"fn main() do
  var s = 0
  for i in 5 do
    for j in 4 do
      s = s + j
    end
    s = s + i
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "range_for_nested_2: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"40
"#);
}

#[test]
fn range_for_nested_3() {
    let (out, diags) = interpret(r#"fn main() do
  var s = 0
  for i in 6 do
    for j in 2 do
      s = s + j
    end
    s = s + i
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "range_for_nested_3: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"21
"#);
}

#[test]
fn range_for_nested_4() {
    let (out, diags) = interpret(r#"fn main() do
  var s = 0
  for i in 3 do
    for j in 3 do
      s = s + j
    end
    s = s + i
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "range_for_nested_4: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"12
"#);
}

#[test]
fn range_for_nested_5() {
    let (out, diags) = interpret(r#"fn main() do
  var s = 0
  for i in 4 do
    for j in 4 do
      s = s + j
    end
    s = s + i
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "range_for_nested_5: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"30
"#);
}

#[test]
fn range_for_nested_6() {
    let (out, diags) = interpret(r#"fn main() do
  var s = 0
  for i in 5 do
    for j in 2 do
      s = s + j
    end
    s = s + i
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "range_for_nested_6: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"15
"#);
}

#[test]
fn range_for_nested_7() {
    let (out, diags) = interpret(r#"fn main() do
  var s = 0
  for i in 6 do
    for j in 3 do
      s = s + j
    end
    s = s + i
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "range_for_nested_7: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"33
"#);
}

#[test]
fn range_for_nested_8() {
    let (out, diags) = interpret(r#"fn main() do
  var s = 0
  for i in 3 do
    for j in 4 do
      s = s + j
    end
    s = s + i
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "range_for_nested_8: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"21
"#);
}

#[test]
fn range_for_nested_9() {
    let (out, diags) = interpret(r#"fn main() do
  var s = 0
  for i in 4 do
    for j in 2 do
      s = s + j
    end
    s = s + i
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "range_for_nested_9: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"10
"#);
}

#[test]
fn range_for_nested_10() {
    let (out, diags) = interpret(r#"fn main() do
  var s = 0
  for i in 5 do
    for j in 3 do
      s = s + j
    end
    s = s + i
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "range_for_nested_10: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"25
"#);
}

#[test]
fn range_for_nested_11() {
    let (out, diags) = interpret(r#"fn main() do
  var s = 0
  for i in 6 do
    for j in 4 do
      s = s + j
    end
    s = s + i
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "range_for_nested_11: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"51
"#);
}

#[test]
fn range_for_nested_12() {
    let (out, diags) = interpret(r#"fn main() do
  var s = 0
  for i in 3 do
    for j in 2 do
      s = s + j
    end
    s = s + i
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "range_for_nested_12: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"6
"#);
}

#[test]
fn range_for_nested_13() {
    let (out, diags) = interpret(r#"fn main() do
  var s = 0
  for i in 4 do
    for j in 3 do
      s = s + j
    end
    s = s + i
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "range_for_nested_13: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"18
"#);
}

#[test]
fn range_for_nested_14() {
    let (out, diags) = interpret(r#"fn main() do
  var s = 0
  for i in 5 do
    for j in 4 do
      s = s + j
    end
    s = s + i
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "range_for_nested_14: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"40
"#);
}

#[test]
fn range_for_nested_15() {
    let (out, diags) = interpret(r#"fn main() do
  var s = 0
  for i in 6 do
    for j in 2 do
      s = s + j
    end
    s = s + i
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "range_for_nested_15: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"21
"#);
}

#[test]
fn range_for_nested_16() {
    let (out, diags) = interpret(r#"fn main() do
  var s = 0
  for i in 3 do
    for j in 3 do
      s = s + j
    end
    s = s + i
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "range_for_nested_16: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"12
"#);
}

#[test]
fn range_for_nested_17() {
    let (out, diags) = interpret(r#"fn main() do
  var s = 0
  for i in 4 do
    for j in 4 do
      s = s + j
    end
    s = s + i
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "range_for_nested_17: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"30
"#);
}

#[test]
fn range_for_nested_18() {
    let (out, diags) = interpret(r#"fn main() do
  var s = 0
  for i in 5 do
    for j in 2 do
      s = s + j
    end
    s = s + i
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "range_for_nested_18: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"15
"#);
}

#[test]
fn range_for_nested_19() {
    let (out, diags) = interpret(r#"fn main() do
  var s = 0
  for i in 6 do
    for j in 3 do
      s = s + j
    end
    s = s + i
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "range_for_nested_19: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"33
"#);
}

#[test]
fn range_for_nested_20() {
    let (out, diags) = interpret(r#"fn main() do
  var s = 0
  for i in 3 do
    for j in 4 do
      s = s + j
    end
    s = s + i
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "range_for_nested_20: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"21
"#);
}

#[test]
fn range_for_break_1() {
    let (out, diags) = interpret(r#"fn main() do
  var s = 0
  for i in 8 do
    if i == 3 then break end
    s = s + i
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "range_for_break_1: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"3
"#);
}

#[test]
fn range_for_break_2() {
    let (out, diags) = interpret(r#"fn main() do
  var s = 0
  for i in 8 do
    if i == 4 then break end
    s = s + i
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "range_for_break_2: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"6
"#);
}

#[test]
fn range_for_break_3() {
    let (out, diags) = interpret(r#"fn main() do
  var s = 0
  for i in 8 do
    if i == 5 then break end
    s = s + i
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "range_for_break_3: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"10
"#);
}

#[test]
fn range_for_break_4() {
    let (out, diags) = interpret(r#"fn main() do
  var s = 0
  for i in 8 do
    if i == 6 then break end
    s = s + i
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "range_for_break_4: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"15
"#);
}

#[test]
fn range_for_break_5() {
    let (out, diags) = interpret(r#"fn main() do
  var s = 0
  for i in 8 do
    if i == 2 then break end
    s = s + i
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "range_for_break_5: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"1
"#);
}

#[test]
fn range_for_break_6() {
    let (out, diags) = interpret(r#"fn main() do
  var s = 0
  for i in 8 do
    if i == 3 then break end
    s = s + i
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "range_for_break_6: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"3
"#);
}

#[test]
fn range_for_break_7() {
    let (out, diags) = interpret(r#"fn main() do
  var s = 0
  for i in 8 do
    if i == 4 then break end
    s = s + i
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "range_for_break_7: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"6
"#);
}

#[test]
fn range_for_break_8() {
    let (out, diags) = interpret(r#"fn main() do
  var s = 0
  for i in 8 do
    if i == 5 then break end
    s = s + i
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "range_for_break_8: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"10
"#);
}

#[test]
fn range_for_break_9() {
    let (out, diags) = interpret(r#"fn main() do
  var s = 0
  for i in 8 do
    if i == 6 then break end
    s = s + i
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "range_for_break_9: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"15
"#);
}

#[test]
fn range_for_break_10() {
    let (out, diags) = interpret(r#"fn main() do
  var s = 0
  for i in 8 do
    if i == 2 then break end
    s = s + i
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "range_for_break_10: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"1
"#);
}

#[test]
fn range_for_break_11() {
    let (out, diags) = interpret(r#"fn main() do
  var s = 0
  for i in 8 do
    if i == 3 then break end
    s = s + i
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "range_for_break_11: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"3
"#);
}

#[test]
fn range_for_break_12() {
    let (out, diags) = interpret(r#"fn main() do
  var s = 0
  for i in 8 do
    if i == 4 then break end
    s = s + i
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "range_for_break_12: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"6
"#);
}

#[test]
fn range_for_break_13() {
    let (out, diags) = interpret(r#"fn main() do
  var s = 0
  for i in 8 do
    if i == 5 then break end
    s = s + i
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "range_for_break_13: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"10
"#);
}

#[test]
fn range_for_break_14() {
    let (out, diags) = interpret(r#"fn main() do
  var s = 0
  for i in 8 do
    if i == 6 then break end
    s = s + i
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "range_for_break_14: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"15
"#);
}

#[test]
fn range_for_break_15() {
    let (out, diags) = interpret(r#"fn main() do
  var s = 0
  for i in 8 do
    if i == 2 then break end
    s = s + i
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "range_for_break_15: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"1
"#);
}

#[test]
fn range_for_break_16() {
    let (out, diags) = interpret(r#"fn main() do
  var s = 0
  for i in 8 do
    if i == 3 then break end
    s = s + i
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "range_for_break_16: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"3
"#);
}

#[test]
fn range_for_break_17() {
    let (out, diags) = interpret(r#"fn main() do
  var s = 0
  for i in 8 do
    if i == 4 then break end
    s = s + i
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "range_for_break_17: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"6
"#);
}

#[test]
fn range_for_break_18() {
    let (out, diags) = interpret(r#"fn main() do
  var s = 0
  for i in 8 do
    if i == 5 then break end
    s = s + i
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "range_for_break_18: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"10
"#);
}

#[test]
fn range_for_break_19() {
    let (out, diags) = interpret(r#"fn main() do
  var s = 0
  for i in 8 do
    if i == 6 then break end
    s = s + i
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "range_for_break_19: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"15
"#);
}

#[test]
fn range_for_break_20() {
    let (out, diags) = interpret(r#"fn main() do
  var s = 0
  for i in 8 do
    if i == 2 then break end
    s = s + i
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "range_for_break_20: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"1
"#);
}

#[test]
fn range_for_continue_1() {
    let (out, diags) = interpret(r#"fn main() do
  var s = 0
  for i in 7 do
    if i == 3 then continue end
    s = s + i
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "range_for_continue_1: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"18
"#);
}

#[test]
fn range_for_continue_2() {
    let (out, diags) = interpret(r#"fn main() do
  var s = 0
  for i in 7 do
    if i == 4 then continue end
    s = s + i
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "range_for_continue_2: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"17
"#);
}

#[test]
fn range_for_continue_3() {
    let (out, diags) = interpret(r#"fn main() do
  var s = 0
  for i in 7 do
    if i == 5 then continue end
    s = s + i
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "range_for_continue_3: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"16
"#);
}

#[test]
fn range_for_continue_4() {
    let (out, diags) = interpret(r#"fn main() do
  var s = 0
  for i in 7 do
    if i == 2 then continue end
    s = s + i
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "range_for_continue_4: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"19
"#);
}

#[test]
fn range_for_continue_5() {
    let (out, diags) = interpret(r#"fn main() do
  var s = 0
  for i in 7 do
    if i == 3 then continue end
    s = s + i
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "range_for_continue_5: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"18
"#);
}

#[test]
fn range_for_continue_6() {
    let (out, diags) = interpret(r#"fn main() do
  var s = 0
  for i in 7 do
    if i == 4 then continue end
    s = s + i
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "range_for_continue_6: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"17
"#);
}

#[test]
fn range_for_continue_7() {
    let (out, diags) = interpret(r#"fn main() do
  var s = 0
  for i in 7 do
    if i == 5 then continue end
    s = s + i
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "range_for_continue_7: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"16
"#);
}

#[test]
fn range_for_continue_8() {
    let (out, diags) = interpret(r#"fn main() do
  var s = 0
  for i in 7 do
    if i == 2 then continue end
    s = s + i
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "range_for_continue_8: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"19
"#);
}

#[test]
fn range_for_continue_9() {
    let (out, diags) = interpret(r#"fn main() do
  var s = 0
  for i in 7 do
    if i == 3 then continue end
    s = s + i
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "range_for_continue_9: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"18
"#);
}

#[test]
fn range_for_continue_10() {
    let (out, diags) = interpret(r#"fn main() do
  var s = 0
  for i in 7 do
    if i == 4 then continue end
    s = s + i
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "range_for_continue_10: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"17
"#);
}

#[test]
fn range_for_continue_11() {
    let (out, diags) = interpret(r#"fn main() do
  var s = 0
  for i in 7 do
    if i == 5 then continue end
    s = s + i
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "range_for_continue_11: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"16
"#);
}

#[test]
fn range_for_continue_12() {
    let (out, diags) = interpret(r#"fn main() do
  var s = 0
  for i in 7 do
    if i == 2 then continue end
    s = s + i
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "range_for_continue_12: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"19
"#);
}

#[test]
fn range_for_continue_13() {
    let (out, diags) = interpret(r#"fn main() do
  var s = 0
  for i in 7 do
    if i == 3 then continue end
    s = s + i
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "range_for_continue_13: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"18
"#);
}

#[test]
fn range_for_continue_14() {
    let (out, diags) = interpret(r#"fn main() do
  var s = 0
  for i in 7 do
    if i == 4 then continue end
    s = s + i
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "range_for_continue_14: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"17
"#);
}

#[test]
fn range_for_continue_15() {
    let (out, diags) = interpret(r#"fn main() do
  var s = 0
  for i in 7 do
    if i == 5 then continue end
    s = s + i
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "range_for_continue_15: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"16
"#);
}

#[test]
fn range_for_continue_16() {
    let (out, diags) = interpret(r#"fn main() do
  var s = 0
  for i in 7 do
    if i == 2 then continue end
    s = s + i
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "range_for_continue_16: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"19
"#);
}

#[test]
fn range_for_continue_17() {
    let (out, diags) = interpret(r#"fn main() do
  var s = 0
  for i in 7 do
    if i == 3 then continue end
    s = s + i
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "range_for_continue_17: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"18
"#);
}

#[test]
fn range_for_continue_18() {
    let (out, diags) = interpret(r#"fn main() do
  var s = 0
  for i in 7 do
    if i == 4 then continue end
    s = s + i
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "range_for_continue_18: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"17
"#);
}

#[test]
fn range_for_continue_19() {
    let (out, diags) = interpret(r#"fn main() do
  var s = 0
  for i in 7 do
    if i == 5 then continue end
    s = s + i
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "range_for_continue_19: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"16
"#);
}

#[test]
fn range_for_continue_20() {
    let (out, diags) = interpret(r#"fn main() do
  var s = 0
  for i in 7 do
    if i == 2 then continue end
    s = s + i
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "range_for_continue_20: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"19
"#);
}

#[test]
fn range_for_expr_bound_1() {
    let (out, diags) = interpret(r#"fn main() do
  var s = 0
  for i in (2 + 2) do
    s = s + i
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "range_for_expr_bound_1: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"6
"#);
}

#[test]
fn range_for_expr_bound_2() {
    let (out, diags) = interpret(r#"fn main() do
  var s = 0
  for i in (2 + 3) do
    s = s + i
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "range_for_expr_bound_2: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"10
"#);
}

#[test]
fn range_for_expr_bound_3() {
    let (out, diags) = interpret(r#"fn main() do
  var s = 0
  for i in (2 + 1) do
    s = s + i
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "range_for_expr_bound_3: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"3
"#);
}

#[test]
fn range_for_expr_bound_4() {
    let (out, diags) = interpret(r#"fn main() do
  var s = 0
  for i in (2 + 2) do
    s = s + i
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "range_for_expr_bound_4: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"6
"#);
}

#[test]
fn range_for_expr_bound_5() {
    let (out, diags) = interpret(r#"fn main() do
  var s = 0
  for i in (2 + 3) do
    s = s + i
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "range_for_expr_bound_5: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"10
"#);
}

#[test]
fn range_for_expr_bound_6() {
    let (out, diags) = interpret(r#"fn main() do
  var s = 0
  for i in (2 + 1) do
    s = s + i
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "range_for_expr_bound_6: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"3
"#);
}

#[test]
fn range_for_expr_bound_7() {
    let (out, diags) = interpret(r#"fn main() do
  var s = 0
  for i in (2 + 2) do
    s = s + i
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "range_for_expr_bound_7: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"6
"#);
}

#[test]
fn range_for_expr_bound_8() {
    let (out, diags) = interpret(r#"fn main() do
  var s = 0
  for i in (2 + 3) do
    s = s + i
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "range_for_expr_bound_8: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"10
"#);
}

#[test]
fn range_for_expr_bound_9() {
    let (out, diags) = interpret(r#"fn main() do
  var s = 0
  for i in (2 + 1) do
    s = s + i
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "range_for_expr_bound_9: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"3
"#);
}

#[test]
fn range_for_expr_bound_10() {
    let (out, diags) = interpret(r#"fn main() do
  var s = 0
  for i in (2 + 2) do
    s = s + i
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "range_for_expr_bound_10: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"6
"#);
}

#[test]
fn range_lowering_1() {
    let (module, diags) = lower(r#"fn main() do
  for i in 6 do
    print(i)
  end
end"#, 0);
    assert!(diags.items.iter().all(|d| d.severity != axolc_core::diag::Severity::Error));
    let mut ranges = 0;
    for item in &module.items {
        if let Item::Fn(fun) = item {
            for s in &fun.body.stmts {
                if let Stmt::For { iter, .. } = s {
                    if let Expr::Range(lo, hi, inc, _) = iter {
                        assert_eq!(*inc, false);
                        assert!(matches!(lo.as_deref(), Some(Expr::IntLit(t, _)) if t == "0"));
                        assert!(matches!(hi.as_deref(), Some(Expr::IntLit(t, _)) if t == "6"));
                        ranges += 1;
                    }
                }
            }
        }
    }
    assert_eq!(ranges, 1);
}

#[test]
fn range_lowering_2() {
    let (module, diags) = lower(r#"fn main() do
  for i in 7 do
    print(i)
  end
end"#, 0);
    assert!(diags.items.iter().all(|d| d.severity != axolc_core::diag::Severity::Error));
    let mut ranges = 0;
    for item in &module.items {
        if let Item::Fn(fun) = item {
            for s in &fun.body.stmts {
                if let Stmt::For { iter, .. } = s {
                    if let Expr::Range(lo, hi, inc, _) = iter {
                        assert_eq!(*inc, false);
                        assert!(matches!(lo.as_deref(), Some(Expr::IntLit(t, _)) if t == "0"));
                        assert!(matches!(hi.as_deref(), Some(Expr::IntLit(t, _)) if t == "7"));
                        ranges += 1;
                    }
                }
            }
        }
    }
    assert_eq!(ranges, 1);
}

#[test]
fn range_lowering_3() {
    let (module, diags) = lower(r#"fn main() do
  for i in 8 do
    print(i)
  end
end"#, 0);
    assert!(diags.items.iter().all(|d| d.severity != axolc_core::diag::Severity::Error));
    let mut ranges = 0;
    for item in &module.items {
        if let Item::Fn(fun) = item {
            for s in &fun.body.stmts {
                if let Stmt::For { iter, .. } = s {
                    if let Expr::Range(lo, hi, inc, _) = iter {
                        assert_eq!(*inc, false);
                        assert!(matches!(lo.as_deref(), Some(Expr::IntLit(t, _)) if t == "0"));
                        assert!(matches!(hi.as_deref(), Some(Expr::IntLit(t, _)) if t == "8"));
                        ranges += 1;
                    }
                }
            }
        }
    }
    assert_eq!(ranges, 1);
}

#[test]
fn range_lowering_4() {
    let (module, diags) = lower(r#"fn main() do
  for i in 9 do
    print(i)
  end
end"#, 0);
    assert!(diags.items.iter().all(|d| d.severity != axolc_core::diag::Severity::Error));
    let mut ranges = 0;
    for item in &module.items {
        if let Item::Fn(fun) = item {
            for s in &fun.body.stmts {
                if let Stmt::For { iter, .. } = s {
                    if let Expr::Range(lo, hi, inc, _) = iter {
                        assert_eq!(*inc, false);
                        assert!(matches!(lo.as_deref(), Some(Expr::IntLit(t, _)) if t == "0"));
                        assert!(matches!(hi.as_deref(), Some(Expr::IntLit(t, _)) if t == "9"));
                        ranges += 1;
                    }
                }
            }
        }
    }
    assert_eq!(ranges, 1);
}

#[test]
fn range_lowering_5() {
    let (module, diags) = lower(r#"fn main() do
  for i in 10 do
    print(i)
  end
end"#, 0);
    assert!(diags.items.iter().all(|d| d.severity != axolc_core::diag::Severity::Error));
    let mut ranges = 0;
    for item in &module.items {
        if let Item::Fn(fun) = item {
            for s in &fun.body.stmts {
                if let Stmt::For { iter, .. } = s {
                    if let Expr::Range(lo, hi, inc, _) = iter {
                        assert_eq!(*inc, false);
                        assert!(matches!(lo.as_deref(), Some(Expr::IntLit(t, _)) if t == "0"));
                        assert!(matches!(hi.as_deref(), Some(Expr::IntLit(t, _)) if t == "10"));
                        ranges += 1;
                    }
                }
            }
        }
    }
    assert_eq!(ranges, 1);
}

#[test]
fn range_lowering_6() {
    let (module, diags) = lower(r#"fn main() do
  for i in 11 do
    print(i)
  end
end"#, 0);
    assert!(diags.items.iter().all(|d| d.severity != axolc_core::diag::Severity::Error));
    let mut ranges = 0;
    for item in &module.items {
        if let Item::Fn(fun) = item {
            for s in &fun.body.stmts {
                if let Stmt::For { iter, .. } = s {
                    if let Expr::Range(lo, hi, inc, _) = iter {
                        assert_eq!(*inc, false);
                        assert!(matches!(lo.as_deref(), Some(Expr::IntLit(t, _)) if t == "0"));
                        assert!(matches!(hi.as_deref(), Some(Expr::IntLit(t, _)) if t == "11"));
                        ranges += 1;
                    }
                }
            }
        }
    }
    assert_eq!(ranges, 1);
}

#[test]
fn range_lowering_7() {
    let (module, diags) = lower(r#"fn main() do
  for i in 12 do
    print(i)
  end
end"#, 0);
    assert!(diags.items.iter().all(|d| d.severity != axolc_core::diag::Severity::Error));
    let mut ranges = 0;
    for item in &module.items {
        if let Item::Fn(fun) = item {
            for s in &fun.body.stmts {
                if let Stmt::For { iter, .. } = s {
                    if let Expr::Range(lo, hi, inc, _) = iter {
                        assert_eq!(*inc, false);
                        assert!(matches!(lo.as_deref(), Some(Expr::IntLit(t, _)) if t == "0"));
                        assert!(matches!(hi.as_deref(), Some(Expr::IntLit(t, _)) if t == "12"));
                        ranges += 1;
                    }
                }
            }
        }
    }
    assert_eq!(ranges, 1);
}

#[test]
fn range_lowering_8() {
    let (module, diags) = lower(r#"fn main() do
  for i in 13 do
    print(i)
  end
end"#, 0);
    assert!(diags.items.iter().all(|d| d.severity != axolc_core::diag::Severity::Error));
    let mut ranges = 0;
    for item in &module.items {
        if let Item::Fn(fun) = item {
            for s in &fun.body.stmts {
                if let Stmt::For { iter, .. } = s {
                    if let Expr::Range(lo, hi, inc, _) = iter {
                        assert_eq!(*inc, false);
                        assert!(matches!(lo.as_deref(), Some(Expr::IntLit(t, _)) if t == "0"));
                        assert!(matches!(hi.as_deref(), Some(Expr::IntLit(t, _)) if t == "13"));
                        ranges += 1;
                    }
                }
            }
        }
    }
    assert_eq!(ranges, 1);
}

#[test]
fn range_lowering_9() {
    let (module, diags) = lower(r#"fn main() do
  for i in 14 do
    print(i)
  end
end"#, 0);
    assert!(diags.items.iter().all(|d| d.severity != axolc_core::diag::Severity::Error));
    let mut ranges = 0;
    for item in &module.items {
        if let Item::Fn(fun) = item {
            for s in &fun.body.stmts {
                if let Stmt::For { iter, .. } = s {
                    if let Expr::Range(lo, hi, inc, _) = iter {
                        assert_eq!(*inc, false);
                        assert!(matches!(lo.as_deref(), Some(Expr::IntLit(t, _)) if t == "0"));
                        assert!(matches!(hi.as_deref(), Some(Expr::IntLit(t, _)) if t == "14"));
                        ranges += 1;
                    }
                }
            }
        }
    }
    assert_eq!(ranges, 1);
}

#[test]
fn range_lowering_10() {
    let (module, diags) = lower(r#"fn main() do
  for i in 15 do
    print(i)
  end
end"#, 0);
    assert!(diags.items.iter().all(|d| d.severity != axolc_core::diag::Severity::Error));
    let mut ranges = 0;
    for item in &module.items {
        if let Item::Fn(fun) = item {
            for s in &fun.body.stmts {
                if let Stmt::For { iter, .. } = s {
                    if let Expr::Range(lo, hi, inc, _) = iter {
                        assert_eq!(*inc, false);
                        assert!(matches!(lo.as_deref(), Some(Expr::IntLit(t, _)) if t == "0"));
                        assert!(matches!(hi.as_deref(), Some(Expr::IntLit(t, _)) if t == "15"));
                        ranges += 1;
                    }
                }
            }
        }
    }
    assert_eq!(ranges, 1);
}

#[test]
fn range_lowering_11() {
    let (module, diags) = lower(r#"fn main() do
  for i in 16 do
    print(i)
  end
end"#, 0);
    assert!(diags.items.iter().all(|d| d.severity != axolc_core::diag::Severity::Error));
    let mut ranges = 0;
    for item in &module.items {
        if let Item::Fn(fun) = item {
            for s in &fun.body.stmts {
                if let Stmt::For { iter, .. } = s {
                    if let Expr::Range(lo, hi, inc, _) = iter {
                        assert_eq!(*inc, false);
                        assert!(matches!(lo.as_deref(), Some(Expr::IntLit(t, _)) if t == "0"));
                        assert!(matches!(hi.as_deref(), Some(Expr::IntLit(t, _)) if t == "16"));
                        ranges += 1;
                    }
                }
            }
        }
    }
    assert_eq!(ranges, 1);
}

#[test]
fn range_lowering_12() {
    let (module, diags) = lower(r#"fn main() do
  for i in 17 do
    print(i)
  end
end"#, 0);
    assert!(diags.items.iter().all(|d| d.severity != axolc_core::diag::Severity::Error));
    let mut ranges = 0;
    for item in &module.items {
        if let Item::Fn(fun) = item {
            for s in &fun.body.stmts {
                if let Stmt::For { iter, .. } = s {
                    if let Expr::Range(lo, hi, inc, _) = iter {
                        assert_eq!(*inc, false);
                        assert!(matches!(lo.as_deref(), Some(Expr::IntLit(t, _)) if t == "0"));
                        assert!(matches!(hi.as_deref(), Some(Expr::IntLit(t, _)) if t == "17"));
                        ranges += 1;
                    }
                }
            }
        }
    }
    assert_eq!(ranges, 1);
}

#[test]
fn range_lowering_13() {
    let (module, diags) = lower(r#"fn main() do
  for i in 18 do
    print(i)
  end
end"#, 0);
    assert!(diags.items.iter().all(|d| d.severity != axolc_core::diag::Severity::Error));
    let mut ranges = 0;
    for item in &module.items {
        if let Item::Fn(fun) = item {
            for s in &fun.body.stmts {
                if let Stmt::For { iter, .. } = s {
                    if let Expr::Range(lo, hi, inc, _) = iter {
                        assert_eq!(*inc, false);
                        assert!(matches!(lo.as_deref(), Some(Expr::IntLit(t, _)) if t == "0"));
                        assert!(matches!(hi.as_deref(), Some(Expr::IntLit(t, _)) if t == "18"));
                        ranges += 1;
                    }
                }
            }
        }
    }
    assert_eq!(ranges, 1);
}

#[test]
fn range_lowering_14() {
    let (module, diags) = lower(r#"fn main() do
  for i in 19 do
    print(i)
  end
end"#, 0);
    assert!(diags.items.iter().all(|d| d.severity != axolc_core::diag::Severity::Error));
    let mut ranges = 0;
    for item in &module.items {
        if let Item::Fn(fun) = item {
            for s in &fun.body.stmts {
                if let Stmt::For { iter, .. } = s {
                    if let Expr::Range(lo, hi, inc, _) = iter {
                        assert_eq!(*inc, false);
                        assert!(matches!(lo.as_deref(), Some(Expr::IntLit(t, _)) if t == "0"));
                        assert!(matches!(hi.as_deref(), Some(Expr::IntLit(t, _)) if t == "19"));
                        ranges += 1;
                    }
                }
            }
        }
    }
    assert_eq!(ranges, 1);
}

#[test]
fn range_lowering_15() {
    let (module, diags) = lower(r#"fn main() do
  for i in 20 do
    print(i)
  end
end"#, 0);
    assert!(diags.items.iter().all(|d| d.severity != axolc_core::diag::Severity::Error));
    let mut ranges = 0;
    for item in &module.items {
        if let Item::Fn(fun) = item {
            for s in &fun.body.stmts {
                if let Stmt::For { iter, .. } = s {
                    if let Expr::Range(lo, hi, inc, _) = iter {
                        assert_eq!(*inc, false);
                        assert!(matches!(lo.as_deref(), Some(Expr::IntLit(t, _)) if t == "0"));
                        assert!(matches!(hi.as_deref(), Some(Expr::IntLit(t, _)) if t == "20"));
                        ranges += 1;
                    }
                }
            }
        }
    }
    assert_eq!(ranges, 1);
}

#[test]
fn range_lowering_16() {
    let (module, diags) = lower(r#"fn main() do
  for i in 21 do
    print(i)
  end
end"#, 0);
    assert!(diags.items.iter().all(|d| d.severity != axolc_core::diag::Severity::Error));
    let mut ranges = 0;
    for item in &module.items {
        if let Item::Fn(fun) = item {
            for s in &fun.body.stmts {
                if let Stmt::For { iter, .. } = s {
                    if let Expr::Range(lo, hi, inc, _) = iter {
                        assert_eq!(*inc, false);
                        assert!(matches!(lo.as_deref(), Some(Expr::IntLit(t, _)) if t == "0"));
                        assert!(matches!(hi.as_deref(), Some(Expr::IntLit(t, _)) if t == "21"));
                        ranges += 1;
                    }
                }
            }
        }
    }
    assert_eq!(ranges, 1);
}

#[test]
fn range_lowering_17() {
    let (module, diags) = lower(r#"fn main() do
  for i in 22 do
    print(i)
  end
end"#, 0);
    assert!(diags.items.iter().all(|d| d.severity != axolc_core::diag::Severity::Error));
    let mut ranges = 0;
    for item in &module.items {
        if let Item::Fn(fun) = item {
            for s in &fun.body.stmts {
                if let Stmt::For { iter, .. } = s {
                    if let Expr::Range(lo, hi, inc, _) = iter {
                        assert_eq!(*inc, false);
                        assert!(matches!(lo.as_deref(), Some(Expr::IntLit(t, _)) if t == "0"));
                        assert!(matches!(hi.as_deref(), Some(Expr::IntLit(t, _)) if t == "22"));
                        ranges += 1;
                    }
                }
            }
        }
    }
    assert_eq!(ranges, 1);
}

#[test]
fn range_lowering_18() {
    let (module, diags) = lower(r#"fn main() do
  for i in 23 do
    print(i)
  end
end"#, 0);
    assert!(diags.items.iter().all(|d| d.severity != axolc_core::diag::Severity::Error));
    let mut ranges = 0;
    for item in &module.items {
        if let Item::Fn(fun) = item {
            for s in &fun.body.stmts {
                if let Stmt::For { iter, .. } = s {
                    if let Expr::Range(lo, hi, inc, _) = iter {
                        assert_eq!(*inc, false);
                        assert!(matches!(lo.as_deref(), Some(Expr::IntLit(t, _)) if t == "0"));
                        assert!(matches!(hi.as_deref(), Some(Expr::IntLit(t, _)) if t == "23"));
                        ranges += 1;
                    }
                }
            }
        }
    }
    assert_eq!(ranges, 1);
}

#[test]
fn range_lowering_19() {
    let (module, diags) = lower(r#"fn main() do
  for i in 24 do
    print(i)
  end
end"#, 0);
    assert!(diags.items.iter().all(|d| d.severity != axolc_core::diag::Severity::Error));
    let mut ranges = 0;
    for item in &module.items {
        if let Item::Fn(fun) = item {
            for s in &fun.body.stmts {
                if let Stmt::For { iter, .. } = s {
                    if let Expr::Range(lo, hi, inc, _) = iter {
                        assert_eq!(*inc, false);
                        assert!(matches!(lo.as_deref(), Some(Expr::IntLit(t, _)) if t == "0"));
                        assert!(matches!(hi.as_deref(), Some(Expr::IntLit(t, _)) if t == "24"));
                        ranges += 1;
                    }
                }
            }
        }
    }
    assert_eq!(ranges, 1);
}

#[test]
fn range_lowering_20() {
    let (module, diags) = lower(r#"fn main() do
  for i in 25 do
    print(i)
  end
end"#, 0);
    assert!(diags.items.iter().all(|d| d.severity != axolc_core::diag::Severity::Error));
    let mut ranges = 0;
    for item in &module.items {
        if let Item::Fn(fun) = item {
            for s in &fun.body.stmts {
                if let Stmt::For { iter, .. } = s {
                    if let Expr::Range(lo, hi, inc, _) = iter {
                        assert_eq!(*inc, false);
                        assert!(matches!(lo.as_deref(), Some(Expr::IntLit(t, _)) if t == "0"));
                        assert!(matches!(hi.as_deref(), Some(Expr::IntLit(t, _)) if t == "25"));
                        ranges += 1;
                    }
                }
            }
        }
    }
    assert_eq!(ranges, 1);
}

#[test]
fn range_codegen_1() {
    let (rust, diags) = compile_to_rust(r#"fn main() do
  var s = 0
  for i in 5 do
    s = s + i
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("for i in 0..5 {"));
    assert!(rust.contains("println!"));
}

#[test]
fn range_codegen_2() {
    let (rust, diags) = compile_to_rust(r#"fn main() do
  var s = 0
  for i in 6 do
    s = s + i
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("for i in 0..6 {"));
    assert!(rust.contains("println!"));
}

#[test]
fn range_codegen_3() {
    let (rust, diags) = compile_to_rust(r#"fn main() do
  var s = 0
  for i in 7 do
    s = s + i
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("for i in 0..7 {"));
    assert!(rust.contains("println!"));
}

#[test]
fn range_codegen_4() {
    let (rust, diags) = compile_to_rust(r#"fn main() do
  var s = 0
  for i in 8 do
    s = s + i
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("for i in 0..8 {"));
    assert!(rust.contains("println!"));
}

#[test]
fn range_codegen_5() {
    let (rust, diags) = compile_to_rust(r#"fn main() do
  var s = 0
  for i in 9 do
    s = s + i
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("for i in 0..9 {"));
    assert!(rust.contains("println!"));
}

#[test]
fn range_codegen_6() {
    let (rust, diags) = compile_to_rust(r#"fn main() do
  var s = 0
  for i in 10 do
    s = s + i
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("for i in 0..10 {"));
    assert!(rust.contains("println!"));
}

#[test]
fn range_codegen_7() {
    let (rust, diags) = compile_to_rust(r#"fn main() do
  var s = 0
  for i in 11 do
    s = s + i
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("for i in 0..11 {"));
    assert!(rust.contains("println!"));
}

#[test]
fn range_codegen_8() {
    let (rust, diags) = compile_to_rust(r#"fn main() do
  var s = 0
  for i in 12 do
    s = s + i
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("for i in 0..12 {"));
    assert!(rust.contains("println!"));
}

#[test]
fn range_codegen_9() {
    let (rust, diags) = compile_to_rust(r#"fn main() do
  var s = 0
  for i in 13 do
    s = s + i
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("for i in 0..13 {"));
    assert!(rust.contains("println!"));
}

#[test]
fn range_codegen_10() {
    let (rust, diags) = compile_to_rust(r#"fn main() do
  var s = 0
  for i in 14 do
    s = s + i
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("for i in 0..14 {"));
    assert!(rust.contains("println!"));
}

#[test]
fn range_codegen_11() {
    let (rust, diags) = compile_to_rust(r#"fn main() do
  var s = 0
  for i in 15 do
    s = s + i
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("for i in 0..15 {"));
    assert!(rust.contains("println!"));
}

#[test]
fn range_codegen_12() {
    let (rust, diags) = compile_to_rust(r#"fn main() do
  var s = 0
  for i in 16 do
    s = s + i
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("for i in 0..16 {"));
    assert!(rust.contains("println!"));
}

#[test]
fn range_codegen_13() {
    let (rust, diags) = compile_to_rust(r#"fn main() do
  var s = 0
  for i in 17 do
    s = s + i
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("for i in 0..17 {"));
    assert!(rust.contains("println!"));
}

#[test]
fn range_codegen_14() {
    let (rust, diags) = compile_to_rust(r#"fn main() do
  var s = 0
  for i in 18 do
    s = s + i
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("for i in 0..18 {"));
    assert!(rust.contains("println!"));
}

#[test]
fn range_codegen_15() {
    let (rust, diags) = compile_to_rust(r#"fn main() do
  var s = 0
  for i in 19 do
    s = s + i
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("for i in 0..19 {"));
    assert!(rust.contains("println!"));
}

#[test]
fn range_codegen_16() {
    let (rust, diags) = compile_to_rust(r#"fn main() do
  var s = 0
  for i in 20 do
    s = s + i
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("for i in 0..20 {"));
    assert!(rust.contains("println!"));
}

#[test]
fn range_codegen_17() {
    let (rust, diags) = compile_to_rust(r#"fn main() do
  var s = 0
  for i in 21 do
    s = s + i
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("for i in 0..21 {"));
    assert!(rust.contains("println!"));
}

#[test]
fn range_codegen_18() {
    let (rust, diags) = compile_to_rust(r#"fn main() do
  var s = 0
  for i in 22 do
    s = s + i
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("for i in 0..22 {"));
    assert!(rust.contains("println!"));
}

#[test]
fn range_codegen_19() {
    let (rust, diags) = compile_to_rust(r#"fn main() do
  var s = 0
  for i in 23 do
    s = s + i
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("for i in 0..23 {"));
    assert!(rust.contains("println!"));
}

#[test]
fn range_codegen_20() {
    let (rust, diags) = compile_to_rust(r#"fn main() do
  var s = 0
  for i in 24 do
    s = s + i
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("for i in 0..24 {"));
    assert!(rust.contains("println!"));
}

#[test]
fn range_for_array_1() {
    let (out, diags) = interpret(r#"fn main() do
  var s = 0
  for x in [1, 2, 3, 4] do
    s = s + x
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "range_for_array_1: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"10
"#);
}

#[test]
fn range_for_array_2() {
    let (out, diags) = interpret(r#"fn main() do
  var s = 0
  for x in [2, 3, 4, 5] do
    s = s + x
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "range_for_array_2: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"14
"#);
}

#[test]
fn range_for_array_3() {
    let (out, diags) = interpret(r#"fn main() do
  var s = 0
  for x in [3, 4, 5, 6] do
    s = s + x
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "range_for_array_3: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"18
"#);
}

#[test]
fn range_for_array_4() {
    let (out, diags) = interpret(r#"fn main() do
  var s = 0
  for x in [4, 5, 6, 7] do
    s = s + x
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "range_for_array_4: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"22
"#);
}

#[test]
fn range_for_array_5() {
    let (out, diags) = interpret(r#"fn main() do
  var s = 0
  for x in [5, 6, 7, 8] do
    s = s + x
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "range_for_array_5: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"26
"#);
}

#[test]
fn range_for_array_6() {
    let (out, diags) = interpret(r#"fn main() do
  var s = 0
  for x in [6, 7, 8, 9] do
    s = s + x
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "range_for_array_6: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"30
"#);
}

#[test]
fn range_for_array_7() {
    let (out, diags) = interpret(r#"fn main() do
  var s = 0
  for x in [7, 8, 9, 10] do
    s = s + x
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "range_for_array_7: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"34
"#);
}

#[test]
fn range_for_array_8() {
    let (out, diags) = interpret(r#"fn main() do
  var s = 0
  for x in [8, 9, 10, 11] do
    s = s + x
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "range_for_array_8: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"38
"#);
}

#[test]
fn range_for_array_9() {
    let (out, diags) = interpret(r#"fn main() do
  var s = 0
  for x in [9, 10, 11, 12] do
    s = s + x
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "range_for_array_9: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"42
"#);
}

#[test]
fn range_for_array_10() {
    let (out, diags) = interpret(r#"fn main() do
  var s = 0
  for x in [10, 11, 12, 13] do
    s = s + x
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "range_for_array_10: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"46
"#);
}

#[test]
fn range_for_array_11() {
    let (out, diags) = interpret(r#"fn main() do
  var s = 0
  for x in [11, 12, 13, 14] do
    s = s + x
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "range_for_array_11: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"50
"#);
}

#[test]
fn range_for_array_12() {
    let (out, diags) = interpret(r#"fn main() do
  var s = 0
  for x in [12, 13, 14, 15] do
    s = s + x
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "range_for_array_12: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"54
"#);
}

#[test]
fn range_for_array_13() {
    let (out, diags) = interpret(r#"fn main() do
  var s = 0
  for x in [13, 14, 15, 16] do
    s = s + x
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "range_for_array_13: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"58
"#);
}

#[test]
fn range_for_array_14() {
    let (out, diags) = interpret(r#"fn main() do
  var s = 0
  for x in [14, 15, 16, 17] do
    s = s + x
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "range_for_array_14: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"62
"#);
}

#[test]
fn range_for_array_15() {
    let (out, diags) = interpret(r#"fn main() do
  var s = 0
  for x in [15, 16, 17, 18] do
    s = s + x
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "range_for_array_15: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"66
"#);
}

#[test]
fn range_for_array_16() {
    let (out, diags) = interpret(r#"fn main() do
  var s = 0
  for x in [16, 17, 18, 19] do
    s = s + x
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "range_for_array_16: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"70
"#);
}

#[test]
fn range_for_array_17() {
    let (out, diags) = interpret(r#"fn main() do
  var s = 0
  for x in [17, 18, 19, 20] do
    s = s + x
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "range_for_array_17: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"74
"#);
}

#[test]
fn range_for_array_18() {
    let (out, diags) = interpret(r#"fn main() do
  var s = 0
  for x in [18, 19, 20, 21] do
    s = s + x
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "range_for_array_18: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"78
"#);
}

#[test]
fn range_for_array_19() {
    let (out, diags) = interpret(r#"fn main() do
  var s = 0
  for x in [19, 20, 21, 22] do
    s = s + x
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "range_for_array_19: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"82
"#);
}

#[test]
fn range_for_array_20() {
    let (out, diags) = interpret(r#"fn main() do
  var s = 0
  for x in [20, 21, 22, 23] do
    s = s + x
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "range_for_array_20: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"86
"#);
}
