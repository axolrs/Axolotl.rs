// Owner: PascalElixir / axolrs (GitHub org)
// File: Auto-generated tests for method definitions (both syntaxes), colon and dot calls, self mutation with caller write-back, methods returning values, methods with multiple params, and static-method codegen.

use axolc_core::interpret;
use axolc_core::compile_to_rust;

#[test]
fn method_eqfn_colon_1() {
    let (out, diags) = interpret(r#"M = struct
  v: Int = 0
end

M.bump = fn(self, by) do
  self.v = self.v + by
  return self.v
end

fn main() do
  let m = M { v = 1 }
  let r = m:bump(10)
  print(r)
  print(m.v)
end"#, 0);
    assert!(!diags.has_errors(), "method_eqfn_colon_1: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"11
11
"#);
}

#[test]
fn method_eqfn_colon_2() {
    let (out, diags) = interpret(r#"M = struct
  v: Int = 0
end

M.bump = fn(self, by) do
  self.v = self.v + by
  return self.v
end

fn main() do
  let m = M { v = 2 }
  let r = m:bump(20)
  print(r)
  print(m.v)
end"#, 0);
    assert!(!diags.has_errors(), "method_eqfn_colon_2: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"22
22
"#);
}

#[test]
fn method_eqfn_colon_3() {
    let (out, diags) = interpret(r#"M = struct
  v: Int = 0
end

M.bump = fn(self, by) do
  self.v = self.v + by
  return self.v
end

fn main() do
  let m = M { v = 3 }
  let r = m:bump(30)
  print(r)
  print(m.v)
end"#, 0);
    assert!(!diags.has_errors(), "method_eqfn_colon_3: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"33
33
"#);
}

#[test]
fn method_eqfn_colon_4() {
    let (out, diags) = interpret(r#"M = struct
  v: Int = 0
end

M.bump = fn(self, by) do
  self.v = self.v + by
  return self.v
end

fn main() do
  let m = M { v = 4 }
  let r = m:bump(40)
  print(r)
  print(m.v)
end"#, 0);
    assert!(!diags.has_errors(), "method_eqfn_colon_4: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"44
44
"#);
}

#[test]
fn method_eqfn_colon_5() {
    let (out, diags) = interpret(r#"M = struct
  v: Int = 0
end

M.bump = fn(self, by) do
  self.v = self.v + by
  return self.v
end

fn main() do
  let m = M { v = 5 }
  let r = m:bump(50)
  print(r)
  print(m.v)
end"#, 0);
    assert!(!diags.has_errors(), "method_eqfn_colon_5: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"55
55
"#);
}

#[test]
fn method_eqfn_colon_6() {
    let (out, diags) = interpret(r#"M = struct
  v: Int = 0
end

M.bump = fn(self, by) do
  self.v = self.v + by
  return self.v
end

fn main() do
  let m = M { v = 6 }
  let r = m:bump(60)
  print(r)
  print(m.v)
end"#, 0);
    assert!(!diags.has_errors(), "method_eqfn_colon_6: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"66
66
"#);
}

#[test]
fn method_eqfn_colon_7() {
    let (out, diags) = interpret(r#"M = struct
  v: Int = 0
end

M.bump = fn(self, by) do
  self.v = self.v + by
  return self.v
end

fn main() do
  let m = M { v = 7 }
  let r = m:bump(70)
  print(r)
  print(m.v)
end"#, 0);
    assert!(!diags.has_errors(), "method_eqfn_colon_7: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"77
77
"#);
}

#[test]
fn method_eqfn_colon_8() {
    let (out, diags) = interpret(r#"M = struct
  v: Int = 0
end

M.bump = fn(self, by) do
  self.v = self.v + by
  return self.v
end

fn main() do
  let m = M { v = 8 }
  let r = m:bump(80)
  print(r)
  print(m.v)
end"#, 0);
    assert!(!diags.has_errors(), "method_eqfn_colon_8: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"88
88
"#);
}

#[test]
fn method_eqfn_colon_9() {
    let (out, diags) = interpret(r#"M = struct
  v: Int = 0
end

M.bump = fn(self, by) do
  self.v = self.v + by
  return self.v
end

fn main() do
  let m = M { v = 9 }
  let r = m:bump(90)
  print(r)
  print(m.v)
end"#, 0);
    assert!(!diags.has_errors(), "method_eqfn_colon_9: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"99
99
"#);
}

#[test]
fn method_eqfn_colon_10() {
    let (out, diags) = interpret(r#"M = struct
  v: Int = 0
end

M.bump = fn(self, by) do
  self.v = self.v + by
  return self.v
end

fn main() do
  let m = M { v = 10 }
  let r = m:bump(100)
  print(r)
  print(m.v)
end"#, 0);
    assert!(!diags.has_errors(), "method_eqfn_colon_10: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"110
110
"#);
}

#[test]
fn method_eqfn_colon_11() {
    let (out, diags) = interpret(r#"M = struct
  v: Int = 0
end

M.bump = fn(self, by) do
  self.v = self.v + by
  return self.v
end

fn main() do
  let m = M { v = 11 }
  let r = m:bump(110)
  print(r)
  print(m.v)
end"#, 0);
    assert!(!diags.has_errors(), "method_eqfn_colon_11: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"121
121
"#);
}

#[test]
fn method_eqfn_colon_12() {
    let (out, diags) = interpret(r#"M = struct
  v: Int = 0
end

M.bump = fn(self, by) do
  self.v = self.v + by
  return self.v
end

fn main() do
  let m = M { v = 12 }
  let r = m:bump(120)
  print(r)
  print(m.v)
end"#, 0);
    assert!(!diags.has_errors(), "method_eqfn_colon_12: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"132
132
"#);
}

#[test]
fn method_eqfn_colon_13() {
    let (out, diags) = interpret(r#"M = struct
  v: Int = 0
end

M.bump = fn(self, by) do
  self.v = self.v + by
  return self.v
end

fn main() do
  let m = M { v = 13 }
  let r = m:bump(130)
  print(r)
  print(m.v)
end"#, 0);
    assert!(!diags.has_errors(), "method_eqfn_colon_13: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"143
143
"#);
}

#[test]
fn method_eqfn_colon_14() {
    let (out, diags) = interpret(r#"M = struct
  v: Int = 0
end

M.bump = fn(self, by) do
  self.v = self.v + by
  return self.v
end

fn main() do
  let m = M { v = 14 }
  let r = m:bump(140)
  print(r)
  print(m.v)
end"#, 0);
    assert!(!diags.has_errors(), "method_eqfn_colon_14: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"154
154
"#);
}

#[test]
fn method_eqfn_colon_15() {
    let (out, diags) = interpret(r#"M = struct
  v: Int = 0
end

M.bump = fn(self, by) do
  self.v = self.v + by
  return self.v
end

fn main() do
  let m = M { v = 15 }
  let r = m:bump(150)
  print(r)
  print(m.v)
end"#, 0);
    assert!(!diags.has_errors(), "method_eqfn_colon_15: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"165
165
"#);
}

#[test]
fn method_eqfn_colon_16() {
    let (out, diags) = interpret(r#"M = struct
  v: Int = 0
end

M.bump = fn(self, by) do
  self.v = self.v + by
  return self.v
end

fn main() do
  let m = M { v = 16 }
  let r = m:bump(160)
  print(r)
  print(m.v)
end"#, 0);
    assert!(!diags.has_errors(), "method_eqfn_colon_16: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"176
176
"#);
}

#[test]
fn method_eqfn_colon_17() {
    let (out, diags) = interpret(r#"M = struct
  v: Int = 0
end

M.bump = fn(self, by) do
  self.v = self.v + by
  return self.v
end

fn main() do
  let m = M { v = 17 }
  let r = m:bump(170)
  print(r)
  print(m.v)
end"#, 0);
    assert!(!diags.has_errors(), "method_eqfn_colon_17: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"187
187
"#);
}

#[test]
fn method_eqfn_colon_18() {
    let (out, diags) = interpret(r#"M = struct
  v: Int = 0
end

M.bump = fn(self, by) do
  self.v = self.v + by
  return self.v
end

fn main() do
  let m = M { v = 18 }
  let r = m:bump(180)
  print(r)
  print(m.v)
end"#, 0);
    assert!(!diags.has_errors(), "method_eqfn_colon_18: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"198
198
"#);
}

#[test]
fn method_eqfn_colon_19() {
    let (out, diags) = interpret(r#"M = struct
  v: Int = 0
end

M.bump = fn(self, by) do
  self.v = self.v + by
  return self.v
end

fn main() do
  let m = M { v = 19 }
  let r = m:bump(190)
  print(r)
  print(m.v)
end"#, 0);
    assert!(!diags.has_errors(), "method_eqfn_colon_19: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"209
209
"#);
}

#[test]
fn method_eqfn_colon_20() {
    let (out, diags) = interpret(r#"M = struct
  v: Int = 0
end

M.bump = fn(self, by) do
  self.v = self.v + by
  return self.v
end

fn main() do
  let m = M { v = 20 }
  let r = m:bump(200)
  print(r)
  print(m.v)
end"#, 0);
    assert!(!diags.has_errors(), "method_eqfn_colon_20: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"220
220
"#);
}

#[test]
fn method_eqfn_colon_21() {
    let (out, diags) = interpret(r#"M = struct
  v: Int = 0
end

M.bump = fn(self, by) do
  self.v = self.v + by
  return self.v
end

fn main() do
  let m = M { v = 21 }
  let r = m:bump(210)
  print(r)
  print(m.v)
end"#, 0);
    assert!(!diags.has_errors(), "method_eqfn_colon_21: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"231
231
"#);
}

#[test]
fn method_eqfn_colon_22() {
    let (out, diags) = interpret(r#"M = struct
  v: Int = 0
end

M.bump = fn(self, by) do
  self.v = self.v + by
  return self.v
end

fn main() do
  let m = M { v = 22 }
  let r = m:bump(220)
  print(r)
  print(m.v)
end"#, 0);
    assert!(!diags.has_errors(), "method_eqfn_colon_22: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"242
242
"#);
}

#[test]
fn method_eqfn_colon_23() {
    let (out, diags) = interpret(r#"M = struct
  v: Int = 0
end

M.bump = fn(self, by) do
  self.v = self.v + by
  return self.v
end

fn main() do
  let m = M { v = 23 }
  let r = m:bump(230)
  print(r)
  print(m.v)
end"#, 0);
    assert!(!diags.has_errors(), "method_eqfn_colon_23: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"253
253
"#);
}

#[test]
fn method_eqfn_colon_24() {
    let (out, diags) = interpret(r#"M = struct
  v: Int = 0
end

M.bump = fn(self, by) do
  self.v = self.v + by
  return self.v
end

fn main() do
  let m = M { v = 24 }
  let r = m:bump(240)
  print(r)
  print(m.v)
end"#, 0);
    assert!(!diags.has_errors(), "method_eqfn_colon_24: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"264
264
"#);
}

#[test]
fn method_eqfn_colon_25() {
    let (out, diags) = interpret(r#"M = struct
  v: Int = 0
end

M.bump = fn(self, by) do
  self.v = self.v + by
  return self.v
end

fn main() do
  let m = M { v = 25 }
  let r = m:bump(250)
  print(r)
  print(m.v)
end"#, 0);
    assert!(!diags.has_errors(), "method_eqfn_colon_25: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"275
275
"#);
}

#[test]
fn method_eqfn_colon_26() {
    let (out, diags) = interpret(r#"M = struct
  v: Int = 0
end

M.bump = fn(self, by) do
  self.v = self.v + by
  return self.v
end

fn main() do
  let m = M { v = 26 }
  let r = m:bump(260)
  print(r)
  print(m.v)
end"#, 0);
    assert!(!diags.has_errors(), "method_eqfn_colon_26: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"286
286
"#);
}

#[test]
fn method_eqfn_colon_27() {
    let (out, diags) = interpret(r#"M = struct
  v: Int = 0
end

M.bump = fn(self, by) do
  self.v = self.v + by
  return self.v
end

fn main() do
  let m = M { v = 27 }
  let r = m:bump(270)
  print(r)
  print(m.v)
end"#, 0);
    assert!(!diags.has_errors(), "method_eqfn_colon_27: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"297
297
"#);
}

#[test]
fn method_eqfn_colon_28() {
    let (out, diags) = interpret(r#"M = struct
  v: Int = 0
end

M.bump = fn(self, by) do
  self.v = self.v + by
  return self.v
end

fn main() do
  let m = M { v = 28 }
  let r = m:bump(280)
  print(r)
  print(m.v)
end"#, 0);
    assert!(!diags.has_errors(), "method_eqfn_colon_28: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"308
308
"#);
}

#[test]
fn method_eqfn_colon_29() {
    let (out, diags) = interpret(r#"M = struct
  v: Int = 0
end

M.bump = fn(self, by) do
  self.v = self.v + by
  return self.v
end

fn main() do
  let m = M { v = 29 }
  let r = m:bump(290)
  print(r)
  print(m.v)
end"#, 0);
    assert!(!diags.has_errors(), "method_eqfn_colon_29: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"319
319
"#);
}

#[test]
fn method_eqfn_colon_30() {
    let (out, diags) = interpret(r#"M = struct
  v: Int = 0
end

M.bump = fn(self, by) do
  self.v = self.v + by
  return self.v
end

fn main() do
  let m = M { v = 30 }
  let r = m:bump(300)
  print(r)
  print(m.v)
end"#, 0);
    assert!(!diags.has_errors(), "method_eqfn_colon_30: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"330
330
"#);
}

#[test]
fn method_block_dot_1() {
    let (out, diags) = interpret(r#"M = struct
  v: Int = 0
end

M.drop(self, by) do
  self.v = self.v - by
  return self.v
end

fn main() do
  let m = M { v = 2 }
  let r = m.drop(1)
  print(r)
  print(m.v)
end"#, 0);
    assert!(!diags.has_errors(), "method_block_dot_1: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"1
1
"#);
}

#[test]
fn method_block_dot_2() {
    let (out, diags) = interpret(r#"M = struct
  v: Int = 0
end

M.drop(self, by) do
  self.v = self.v - by
  return self.v
end

fn main() do
  let m = M { v = 4 }
  let r = m.drop(2)
  print(r)
  print(m.v)
end"#, 0);
    assert!(!diags.has_errors(), "method_block_dot_2: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"2
2
"#);
}

#[test]
fn method_block_dot_3() {
    let (out, diags) = interpret(r#"M = struct
  v: Int = 0
end

M.drop(self, by) do
  self.v = self.v - by
  return self.v
end

fn main() do
  let m = M { v = 6 }
  let r = m.drop(3)
  print(r)
  print(m.v)
end"#, 0);
    assert!(!diags.has_errors(), "method_block_dot_3: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"3
3
"#);
}

#[test]
fn method_block_dot_4() {
    let (out, diags) = interpret(r#"M = struct
  v: Int = 0
end

M.drop(self, by) do
  self.v = self.v - by
  return self.v
end

fn main() do
  let m = M { v = 8 }
  let r = m.drop(4)
  print(r)
  print(m.v)
end"#, 0);
    assert!(!diags.has_errors(), "method_block_dot_4: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"4
4
"#);
}

#[test]
fn method_block_dot_5() {
    let (out, diags) = interpret(r#"M = struct
  v: Int = 0
end

M.drop(self, by) do
  self.v = self.v - by
  return self.v
end

fn main() do
  let m = M { v = 10 }
  let r = m.drop(5)
  print(r)
  print(m.v)
end"#, 0);
    assert!(!diags.has_errors(), "method_block_dot_5: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"5
5
"#);
}

#[test]
fn method_block_dot_6() {
    let (out, diags) = interpret(r#"M = struct
  v: Int = 0
end

M.drop(self, by) do
  self.v = self.v - by
  return self.v
end

fn main() do
  let m = M { v = 12 }
  let r = m.drop(6)
  print(r)
  print(m.v)
end"#, 0);
    assert!(!diags.has_errors(), "method_block_dot_6: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"6
6
"#);
}

#[test]
fn method_block_dot_7() {
    let (out, diags) = interpret(r#"M = struct
  v: Int = 0
end

M.drop(self, by) do
  self.v = self.v - by
  return self.v
end

fn main() do
  let m = M { v = 14 }
  let r = m.drop(7)
  print(r)
  print(m.v)
end"#, 0);
    assert!(!diags.has_errors(), "method_block_dot_7: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"7
7
"#);
}

#[test]
fn method_block_dot_8() {
    let (out, diags) = interpret(r#"M = struct
  v: Int = 0
end

M.drop(self, by) do
  self.v = self.v - by
  return self.v
end

fn main() do
  let m = M { v = 16 }
  let r = m.drop(8)
  print(r)
  print(m.v)
end"#, 0);
    assert!(!diags.has_errors(), "method_block_dot_8: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"8
8
"#);
}

#[test]
fn method_block_dot_9() {
    let (out, diags) = interpret(r#"M = struct
  v: Int = 0
end

M.drop(self, by) do
  self.v = self.v - by
  return self.v
end

fn main() do
  let m = M { v = 18 }
  let r = m.drop(9)
  print(r)
  print(m.v)
end"#, 0);
    assert!(!diags.has_errors(), "method_block_dot_9: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"9
9
"#);
}

#[test]
fn method_block_dot_10() {
    let (out, diags) = interpret(r#"M = struct
  v: Int = 0
end

M.drop(self, by) do
  self.v = self.v - by
  return self.v
end

fn main() do
  let m = M { v = 20 }
  let r = m.drop(10)
  print(r)
  print(m.v)
end"#, 0);
    assert!(!diags.has_errors(), "method_block_dot_10: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"10
10
"#);
}

#[test]
fn method_block_dot_11() {
    let (out, diags) = interpret(r#"M = struct
  v: Int = 0
end

M.drop(self, by) do
  self.v = self.v - by
  return self.v
end

fn main() do
  let m = M { v = 22 }
  let r = m.drop(11)
  print(r)
  print(m.v)
end"#, 0);
    assert!(!diags.has_errors(), "method_block_dot_11: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"11
11
"#);
}

#[test]
fn method_block_dot_12() {
    let (out, diags) = interpret(r#"M = struct
  v: Int = 0
end

M.drop(self, by) do
  self.v = self.v - by
  return self.v
end

fn main() do
  let m = M { v = 24 }
  let r = m.drop(12)
  print(r)
  print(m.v)
end"#, 0);
    assert!(!diags.has_errors(), "method_block_dot_12: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"12
12
"#);
}

#[test]
fn method_block_dot_13() {
    let (out, diags) = interpret(r#"M = struct
  v: Int = 0
end

M.drop(self, by) do
  self.v = self.v - by
  return self.v
end

fn main() do
  let m = M { v = 26 }
  let r = m.drop(13)
  print(r)
  print(m.v)
end"#, 0);
    assert!(!diags.has_errors(), "method_block_dot_13: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"13
13
"#);
}

#[test]
fn method_block_dot_14() {
    let (out, diags) = interpret(r#"M = struct
  v: Int = 0
end

M.drop(self, by) do
  self.v = self.v - by
  return self.v
end

fn main() do
  let m = M { v = 28 }
  let r = m.drop(14)
  print(r)
  print(m.v)
end"#, 0);
    assert!(!diags.has_errors(), "method_block_dot_14: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"14
14
"#);
}

#[test]
fn method_block_dot_15() {
    let (out, diags) = interpret(r#"M = struct
  v: Int = 0
end

M.drop(self, by) do
  self.v = self.v - by
  return self.v
end

fn main() do
  let m = M { v = 30 }
  let r = m.drop(15)
  print(r)
  print(m.v)
end"#, 0);
    assert!(!diags.has_errors(), "method_block_dot_15: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"15
15
"#);
}

#[test]
fn method_block_dot_16() {
    let (out, diags) = interpret(r#"M = struct
  v: Int = 0
end

M.drop(self, by) do
  self.v = self.v - by
  return self.v
end

fn main() do
  let m = M { v = 32 }
  let r = m.drop(16)
  print(r)
  print(m.v)
end"#, 0);
    assert!(!diags.has_errors(), "method_block_dot_16: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"16
16
"#);
}

#[test]
fn method_block_dot_17() {
    let (out, diags) = interpret(r#"M = struct
  v: Int = 0
end

M.drop(self, by) do
  self.v = self.v - by
  return self.v
end

fn main() do
  let m = M { v = 34 }
  let r = m.drop(17)
  print(r)
  print(m.v)
end"#, 0);
    assert!(!diags.has_errors(), "method_block_dot_17: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"17
17
"#);
}

#[test]
fn method_block_dot_18() {
    let (out, diags) = interpret(r#"M = struct
  v: Int = 0
end

M.drop(self, by) do
  self.v = self.v - by
  return self.v
end

fn main() do
  let m = M { v = 36 }
  let r = m.drop(18)
  print(r)
  print(m.v)
end"#, 0);
    assert!(!diags.has_errors(), "method_block_dot_18: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"18
18
"#);
}

#[test]
fn method_block_dot_19() {
    let (out, diags) = interpret(r#"M = struct
  v: Int = 0
end

M.drop(self, by) do
  self.v = self.v - by
  return self.v
end

fn main() do
  let m = M { v = 38 }
  let r = m.drop(19)
  print(r)
  print(m.v)
end"#, 0);
    assert!(!diags.has_errors(), "method_block_dot_19: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"19
19
"#);
}

#[test]
fn method_block_dot_20() {
    let (out, diags) = interpret(r#"M = struct
  v: Int = 0
end

M.drop(self, by) do
  self.v = self.v - by
  return self.v
end

fn main() do
  let m = M { v = 40 }
  let r = m.drop(20)
  print(r)
  print(m.v)
end"#, 0);
    assert!(!diags.has_errors(), "method_block_dot_20: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"20
20
"#);
}

#[test]
fn method_block_dot_21() {
    let (out, diags) = interpret(r#"M = struct
  v: Int = 0
end

M.drop(self, by) do
  self.v = self.v - by
  return self.v
end

fn main() do
  let m = M { v = 42 }
  let r = m.drop(21)
  print(r)
  print(m.v)
end"#, 0);
    assert!(!diags.has_errors(), "method_block_dot_21: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"21
21
"#);
}

#[test]
fn method_block_dot_22() {
    let (out, diags) = interpret(r#"M = struct
  v: Int = 0
end

M.drop(self, by) do
  self.v = self.v - by
  return self.v
end

fn main() do
  let m = M { v = 44 }
  let r = m.drop(22)
  print(r)
  print(m.v)
end"#, 0);
    assert!(!diags.has_errors(), "method_block_dot_22: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"22
22
"#);
}

#[test]
fn method_block_dot_23() {
    let (out, diags) = interpret(r#"M = struct
  v: Int = 0
end

M.drop(self, by) do
  self.v = self.v - by
  return self.v
end

fn main() do
  let m = M { v = 46 }
  let r = m.drop(23)
  print(r)
  print(m.v)
end"#, 0);
    assert!(!diags.has_errors(), "method_block_dot_23: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"23
23
"#);
}

#[test]
fn method_block_dot_24() {
    let (out, diags) = interpret(r#"M = struct
  v: Int = 0
end

M.drop(self, by) do
  self.v = self.v - by
  return self.v
end

fn main() do
  let m = M { v = 48 }
  let r = m.drop(24)
  print(r)
  print(m.v)
end"#, 0);
    assert!(!diags.has_errors(), "method_block_dot_24: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"24
24
"#);
}

#[test]
fn method_block_dot_25() {
    let (out, diags) = interpret(r#"M = struct
  v: Int = 0
end

M.drop(self, by) do
  self.v = self.v - by
  return self.v
end

fn main() do
  let m = M { v = 50 }
  let r = m.drop(25)
  print(r)
  print(m.v)
end"#, 0);
    assert!(!diags.has_errors(), "method_block_dot_25: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"25
25
"#);
}

#[test]
fn method_block_dot_26() {
    let (out, diags) = interpret(r#"M = struct
  v: Int = 0
end

M.drop(self, by) do
  self.v = self.v - by
  return self.v
end

fn main() do
  let m = M { v = 52 }
  let r = m.drop(26)
  print(r)
  print(m.v)
end"#, 0);
    assert!(!diags.has_errors(), "method_block_dot_26: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"26
26
"#);
}

#[test]
fn method_block_dot_27() {
    let (out, diags) = interpret(r#"M = struct
  v: Int = 0
end

M.drop(self, by) do
  self.v = self.v - by
  return self.v
end

fn main() do
  let m = M { v = 54 }
  let r = m.drop(27)
  print(r)
  print(m.v)
end"#, 0);
    assert!(!diags.has_errors(), "method_block_dot_27: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"27
27
"#);
}

#[test]
fn method_block_dot_28() {
    let (out, diags) = interpret(r#"M = struct
  v: Int = 0
end

M.drop(self, by) do
  self.v = self.v - by
  return self.v
end

fn main() do
  let m = M { v = 56 }
  let r = m.drop(28)
  print(r)
  print(m.v)
end"#, 0);
    assert!(!diags.has_errors(), "method_block_dot_28: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"28
28
"#);
}

#[test]
fn method_block_dot_29() {
    let (out, diags) = interpret(r#"M = struct
  v: Int = 0
end

M.drop(self, by) do
  self.v = self.v - by
  return self.v
end

fn main() do
  let m = M { v = 58 }
  let r = m.drop(29)
  print(r)
  print(m.v)
end"#, 0);
    assert!(!diags.has_errors(), "method_block_dot_29: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"29
29
"#);
}

#[test]
fn method_block_dot_30() {
    let (out, diags) = interpret(r#"M = struct
  v: Int = 0
end

M.drop(self, by) do
  self.v = self.v - by
  return self.v
end

fn main() do
  let m = M { v = 60 }
  let r = m.drop(30)
  print(r)
  print(m.v)
end"#, 0);
    assert!(!diags.has_errors(), "method_block_dot_30: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"30
30
"#);
}

#[test]
fn method_return_multi_1() {
    let (out, diags) = interpret(r#"M = struct
  v: Int = 0
end

M.mix = fn(self, a, b) do
  return self.v * 10 + a * 2 + b
end

fn main() do
  let m = M { v = 1 }
  print(m:mix(3, 4))
  print(m.v)
end"#, 0);
    assert!(!diags.has_errors(), "method_return_multi_1: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"20
1
"#);
}

#[test]
fn method_return_multi_2() {
    let (out, diags) = interpret(r#"M = struct
  v: Int = 0
end

M.mix = fn(self, a, b) do
  return self.v * 10 + a * 2 + b
end

fn main() do
  let m = M { v = 2 }
  print(m:mix(3, 4))
  print(m.v)
end"#, 0);
    assert!(!diags.has_errors(), "method_return_multi_2: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"30
2
"#);
}

#[test]
fn method_return_multi_3() {
    let (out, diags) = interpret(r#"M = struct
  v: Int = 0
end

M.mix = fn(self, a, b) do
  return self.v * 10 + a * 2 + b
end

fn main() do
  let m = M { v = 3 }
  print(m:mix(3, 4))
  print(m.v)
end"#, 0);
    assert!(!diags.has_errors(), "method_return_multi_3: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"40
3
"#);
}

#[test]
fn method_return_multi_4() {
    let (out, diags) = interpret(r#"M = struct
  v: Int = 0
end

M.mix = fn(self, a, b) do
  return self.v * 10 + a * 2 + b
end

fn main() do
  let m = M { v = 4 }
  print(m:mix(3, 4))
  print(m.v)
end"#, 0);
    assert!(!diags.has_errors(), "method_return_multi_4: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"50
4
"#);
}

#[test]
fn method_return_multi_5() {
    let (out, diags) = interpret(r#"M = struct
  v: Int = 0
end

M.mix = fn(self, a, b) do
  return self.v * 10 + a * 2 + b
end

fn main() do
  let m = M { v = 5 }
  print(m:mix(3, 4))
  print(m.v)
end"#, 0);
    assert!(!diags.has_errors(), "method_return_multi_5: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"60
5
"#);
}

#[test]
fn method_return_multi_6() {
    let (out, diags) = interpret(r#"M = struct
  v: Int = 0
end

M.mix = fn(self, a, b) do
  return self.v * 10 + a * 2 + b
end

fn main() do
  let m = M { v = 6 }
  print(m:mix(3, 4))
  print(m.v)
end"#, 0);
    assert!(!diags.has_errors(), "method_return_multi_6: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"70
6
"#);
}

#[test]
fn method_return_multi_7() {
    let (out, diags) = interpret(r#"M = struct
  v: Int = 0
end

M.mix = fn(self, a, b) do
  return self.v * 10 + a * 2 + b
end

fn main() do
  let m = M { v = 7 }
  print(m:mix(3, 4))
  print(m.v)
end"#, 0);
    assert!(!diags.has_errors(), "method_return_multi_7: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"80
7
"#);
}

#[test]
fn method_return_multi_8() {
    let (out, diags) = interpret(r#"M = struct
  v: Int = 0
end

M.mix = fn(self, a, b) do
  return self.v * 10 + a * 2 + b
end

fn main() do
  let m = M { v = 8 }
  print(m:mix(3, 4))
  print(m.v)
end"#, 0);
    assert!(!diags.has_errors(), "method_return_multi_8: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"90
8
"#);
}

#[test]
fn method_return_multi_9() {
    let (out, diags) = interpret(r#"M = struct
  v: Int = 0
end

M.mix = fn(self, a, b) do
  return self.v * 10 + a * 2 + b
end

fn main() do
  let m = M { v = 9 }
  print(m:mix(3, 4))
  print(m.v)
end"#, 0);
    assert!(!diags.has_errors(), "method_return_multi_9: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"100
9
"#);
}

#[test]
fn method_return_multi_10() {
    let (out, diags) = interpret(r#"M = struct
  v: Int = 0
end

M.mix = fn(self, a, b) do
  return self.v * 10 + a * 2 + b
end

fn main() do
  let m = M { v = 10 }
  print(m:mix(3, 4))
  print(m.v)
end"#, 0);
    assert!(!diags.has_errors(), "method_return_multi_10: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"110
10
"#);
}

#[test]
fn method_return_multi_11() {
    let (out, diags) = interpret(r#"M = struct
  v: Int = 0
end

M.mix = fn(self, a, b) do
  return self.v * 10 + a * 2 + b
end

fn main() do
  let m = M { v = 11 }
  print(m:mix(3, 4))
  print(m.v)
end"#, 0);
    assert!(!diags.has_errors(), "method_return_multi_11: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"120
11
"#);
}

#[test]
fn method_return_multi_12() {
    let (out, diags) = interpret(r#"M = struct
  v: Int = 0
end

M.mix = fn(self, a, b) do
  return self.v * 10 + a * 2 + b
end

fn main() do
  let m = M { v = 12 }
  print(m:mix(3, 4))
  print(m.v)
end"#, 0);
    assert!(!diags.has_errors(), "method_return_multi_12: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"130
12
"#);
}

#[test]
fn method_return_multi_13() {
    let (out, diags) = interpret(r#"M = struct
  v: Int = 0
end

M.mix = fn(self, a, b) do
  return self.v * 10 + a * 2 + b
end

fn main() do
  let m = M { v = 13 }
  print(m:mix(3, 4))
  print(m.v)
end"#, 0);
    assert!(!diags.has_errors(), "method_return_multi_13: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"140
13
"#);
}

#[test]
fn method_return_multi_14() {
    let (out, diags) = interpret(r#"M = struct
  v: Int = 0
end

M.mix = fn(self, a, b) do
  return self.v * 10 + a * 2 + b
end

fn main() do
  let m = M { v = 14 }
  print(m:mix(3, 4))
  print(m.v)
end"#, 0);
    assert!(!diags.has_errors(), "method_return_multi_14: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"150
14
"#);
}

#[test]
fn method_return_multi_15() {
    let (out, diags) = interpret(r#"M = struct
  v: Int = 0
end

M.mix = fn(self, a, b) do
  return self.v * 10 + a * 2 + b
end

fn main() do
  let m = M { v = 15 }
  print(m:mix(3, 4))
  print(m.v)
end"#, 0);
    assert!(!diags.has_errors(), "method_return_multi_15: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"160
15
"#);
}

#[test]
fn method_return_multi_16() {
    let (out, diags) = interpret(r#"M = struct
  v: Int = 0
end

M.mix = fn(self, a, b) do
  return self.v * 10 + a * 2 + b
end

fn main() do
  let m = M { v = 16 }
  print(m:mix(3, 4))
  print(m.v)
end"#, 0);
    assert!(!diags.has_errors(), "method_return_multi_16: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"170
16
"#);
}

#[test]
fn method_return_multi_17() {
    let (out, diags) = interpret(r#"M = struct
  v: Int = 0
end

M.mix = fn(self, a, b) do
  return self.v * 10 + a * 2 + b
end

fn main() do
  let m = M { v = 17 }
  print(m:mix(3, 4))
  print(m.v)
end"#, 0);
    assert!(!diags.has_errors(), "method_return_multi_17: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"180
17
"#);
}

#[test]
fn method_return_multi_18() {
    let (out, diags) = interpret(r#"M = struct
  v: Int = 0
end

M.mix = fn(self, a, b) do
  return self.v * 10 + a * 2 + b
end

fn main() do
  let m = M { v = 18 }
  print(m:mix(3, 4))
  print(m.v)
end"#, 0);
    assert!(!diags.has_errors(), "method_return_multi_18: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"190
18
"#);
}

#[test]
fn method_return_multi_19() {
    let (out, diags) = interpret(r#"M = struct
  v: Int = 0
end

M.mix = fn(self, a, b) do
  return self.v * 10 + a * 2 + b
end

fn main() do
  let m = M { v = 19 }
  print(m:mix(3, 4))
  print(m.v)
end"#, 0);
    assert!(!diags.has_errors(), "method_return_multi_19: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"200
19
"#);
}

#[test]
fn method_return_multi_20() {
    let (out, diags) = interpret(r#"M = struct
  v: Int = 0
end

M.mix = fn(self, a, b) do
  return self.v * 10 + a * 2 + b
end

fn main() do
  let m = M { v = 20 }
  print(m:mix(3, 4))
  print(m.v)
end"#, 0);
    assert!(!diags.has_errors(), "method_return_multi_20: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"210
20
"#);
}

#[test]
fn method_return_multi_21() {
    let (out, diags) = interpret(r#"M = struct
  v: Int = 0
end

M.mix = fn(self, a, b) do
  return self.v * 10 + a * 2 + b
end

fn main() do
  let m = M { v = 21 }
  print(m:mix(3, 4))
  print(m.v)
end"#, 0);
    assert!(!diags.has_errors(), "method_return_multi_21: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"220
21
"#);
}

#[test]
fn method_return_multi_22() {
    let (out, diags) = interpret(r#"M = struct
  v: Int = 0
end

M.mix = fn(self, a, b) do
  return self.v * 10 + a * 2 + b
end

fn main() do
  let m = M { v = 22 }
  print(m:mix(3, 4))
  print(m.v)
end"#, 0);
    assert!(!diags.has_errors(), "method_return_multi_22: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"230
22
"#);
}

#[test]
fn method_return_multi_23() {
    let (out, diags) = interpret(r#"M = struct
  v: Int = 0
end

M.mix = fn(self, a, b) do
  return self.v * 10 + a * 2 + b
end

fn main() do
  let m = M { v = 23 }
  print(m:mix(3, 4))
  print(m.v)
end"#, 0);
    assert!(!diags.has_errors(), "method_return_multi_23: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"240
23
"#);
}

#[test]
fn method_return_multi_24() {
    let (out, diags) = interpret(r#"M = struct
  v: Int = 0
end

M.mix = fn(self, a, b) do
  return self.v * 10 + a * 2 + b
end

fn main() do
  let m = M { v = 24 }
  print(m:mix(3, 4))
  print(m.v)
end"#, 0);
    assert!(!diags.has_errors(), "method_return_multi_24: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"250
24
"#);
}

#[test]
fn method_return_multi_25() {
    let (out, diags) = interpret(r#"M = struct
  v: Int = 0
end

M.mix = fn(self, a, b) do
  return self.v * 10 + a * 2 + b
end

fn main() do
  let m = M { v = 25 }
  print(m:mix(3, 4))
  print(m.v)
end"#, 0);
    assert!(!diags.has_errors(), "method_return_multi_25: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"260
25
"#);
}

#[test]
fn method_writeback_loop_1() {
    let (out, diags) = interpret(r#"C = struct
  n: Int = 0
end

C.inc = fn(self) do
  self.n = self.n + 2
  return self.n
end

fn main() do
  let c = C { n = 0 }
  let i = 0
  while i < 4 do
    c:inc()
    i = i + 1
  end
  print(c.n)
end"#, 0);
    assert!(!diags.has_errors(), "method_writeback_loop_1: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"8
"#);
}

#[test]
fn method_writeback_loop_2() {
    let (out, diags) = interpret(r#"C = struct
  n: Int = 0
end

C.inc = fn(self) do
  self.n = self.n + 2
  return self.n
end

fn main() do
  let c = C { n = 0 }
  let i = 0
  while i < 5 do
    c:inc()
    i = i + 1
  end
  print(c.n)
end"#, 0);
    assert!(!diags.has_errors(), "method_writeback_loop_2: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"10
"#);
}

#[test]
fn method_writeback_loop_3() {
    let (out, diags) = interpret(r#"C = struct
  n: Int = 0
end

C.inc = fn(self) do
  self.n = self.n + 2
  return self.n
end

fn main() do
  let c = C { n = 0 }
  let i = 0
  while i < 6 do
    c:inc()
    i = i + 1
  end
  print(c.n)
end"#, 0);
    assert!(!diags.has_errors(), "method_writeback_loop_3: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"12
"#);
}

#[test]
fn method_writeback_loop_4() {
    let (out, diags) = interpret(r#"C = struct
  n: Int = 0
end

C.inc = fn(self) do
  self.n = self.n + 2
  return self.n
end

fn main() do
  let c = C { n = 0 }
  let i = 0
  while i < 7 do
    c:inc()
    i = i + 1
  end
  print(c.n)
end"#, 0);
    assert!(!diags.has_errors(), "method_writeback_loop_4: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"14
"#);
}

#[test]
fn method_writeback_loop_5() {
    let (out, diags) = interpret(r#"C = struct
  n: Int = 0
end

C.inc = fn(self) do
  self.n = self.n + 2
  return self.n
end

fn main() do
  let c = C { n = 0 }
  let i = 0
  while i < 3 do
    c:inc()
    i = i + 1
  end
  print(c.n)
end"#, 0);
    assert!(!diags.has_errors(), "method_writeback_loop_5: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"6
"#);
}

#[test]
fn method_writeback_loop_6() {
    let (out, diags) = interpret(r#"C = struct
  n: Int = 0
end

C.inc = fn(self) do
  self.n = self.n + 2
  return self.n
end

fn main() do
  let c = C { n = 0 }
  let i = 0
  while i < 4 do
    c:inc()
    i = i + 1
  end
  print(c.n)
end"#, 0);
    assert!(!diags.has_errors(), "method_writeback_loop_6: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"8
"#);
}

#[test]
fn method_writeback_loop_7() {
    let (out, diags) = interpret(r#"C = struct
  n: Int = 0
end

C.inc = fn(self) do
  self.n = self.n + 2
  return self.n
end

fn main() do
  let c = C { n = 0 }
  let i = 0
  while i < 5 do
    c:inc()
    i = i + 1
  end
  print(c.n)
end"#, 0);
    assert!(!diags.has_errors(), "method_writeback_loop_7: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"10
"#);
}

#[test]
fn method_writeback_loop_8() {
    let (out, diags) = interpret(r#"C = struct
  n: Int = 0
end

C.inc = fn(self) do
  self.n = self.n + 2
  return self.n
end

fn main() do
  let c = C { n = 0 }
  let i = 0
  while i < 6 do
    c:inc()
    i = i + 1
  end
  print(c.n)
end"#, 0);
    assert!(!diags.has_errors(), "method_writeback_loop_8: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"12
"#);
}

#[test]
fn method_writeback_loop_9() {
    let (out, diags) = interpret(r#"C = struct
  n: Int = 0
end

C.inc = fn(self) do
  self.n = self.n + 2
  return self.n
end

fn main() do
  let c = C { n = 0 }
  let i = 0
  while i < 7 do
    c:inc()
    i = i + 1
  end
  print(c.n)
end"#, 0);
    assert!(!diags.has_errors(), "method_writeback_loop_9: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"14
"#);
}

#[test]
fn method_writeback_loop_10() {
    let (out, diags) = interpret(r#"C = struct
  n: Int = 0
end

C.inc = fn(self) do
  self.n = self.n + 2
  return self.n
end

fn main() do
  let c = C { n = 0 }
  let i = 0
  while i < 3 do
    c:inc()
    i = i + 1
  end
  print(c.n)
end"#, 0);
    assert!(!diags.has_errors(), "method_writeback_loop_10: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"6
"#);
}

#[test]
fn method_writeback_loop_11() {
    let (out, diags) = interpret(r#"C = struct
  n: Int = 0
end

C.inc = fn(self) do
  self.n = self.n + 2
  return self.n
end

fn main() do
  let c = C { n = 0 }
  let i = 0
  while i < 4 do
    c:inc()
    i = i + 1
  end
  print(c.n)
end"#, 0);
    assert!(!diags.has_errors(), "method_writeback_loop_11: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"8
"#);
}

#[test]
fn method_writeback_loop_12() {
    let (out, diags) = interpret(r#"C = struct
  n: Int = 0
end

C.inc = fn(self) do
  self.n = self.n + 2
  return self.n
end

fn main() do
  let c = C { n = 0 }
  let i = 0
  while i < 5 do
    c:inc()
    i = i + 1
  end
  print(c.n)
end"#, 0);
    assert!(!diags.has_errors(), "method_writeback_loop_12: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"10
"#);
}

#[test]
fn method_writeback_loop_13() {
    let (out, diags) = interpret(r#"C = struct
  n: Int = 0
end

C.inc = fn(self) do
  self.n = self.n + 2
  return self.n
end

fn main() do
  let c = C { n = 0 }
  let i = 0
  while i < 6 do
    c:inc()
    i = i + 1
  end
  print(c.n)
end"#, 0);
    assert!(!diags.has_errors(), "method_writeback_loop_13: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"12
"#);
}

#[test]
fn method_writeback_loop_14() {
    let (out, diags) = interpret(r#"C = struct
  n: Int = 0
end

C.inc = fn(self) do
  self.n = self.n + 2
  return self.n
end

fn main() do
  let c = C { n = 0 }
  let i = 0
  while i < 7 do
    c:inc()
    i = i + 1
  end
  print(c.n)
end"#, 0);
    assert!(!diags.has_errors(), "method_writeback_loop_14: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"14
"#);
}

#[test]
fn method_writeback_loop_15() {
    let (out, diags) = interpret(r#"C = struct
  n: Int = 0
end

C.inc = fn(self) do
  self.n = self.n + 2
  return self.n
end

fn main() do
  let c = C { n = 0 }
  let i = 0
  while i < 3 do
    c:inc()
    i = i + 1
  end
  print(c.n)
end"#, 0);
    assert!(!diags.has_errors(), "method_writeback_loop_15: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"6
"#);
}

#[test]
fn method_writeback_loop_16() {
    let (out, diags) = interpret(r#"C = struct
  n: Int = 0
end

C.inc = fn(self) do
  self.n = self.n + 2
  return self.n
end

fn main() do
  let c = C { n = 0 }
  let i = 0
  while i < 4 do
    c:inc()
    i = i + 1
  end
  print(c.n)
end"#, 0);
    assert!(!diags.has_errors(), "method_writeback_loop_16: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"8
"#);
}

#[test]
fn method_writeback_loop_17() {
    let (out, diags) = interpret(r#"C = struct
  n: Int = 0
end

C.inc = fn(self) do
  self.n = self.n + 2
  return self.n
end

fn main() do
  let c = C { n = 0 }
  let i = 0
  while i < 5 do
    c:inc()
    i = i + 1
  end
  print(c.n)
end"#, 0);
    assert!(!diags.has_errors(), "method_writeback_loop_17: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"10
"#);
}

#[test]
fn method_writeback_loop_18() {
    let (out, diags) = interpret(r#"C = struct
  n: Int = 0
end

C.inc = fn(self) do
  self.n = self.n + 2
  return self.n
end

fn main() do
  let c = C { n = 0 }
  let i = 0
  while i < 6 do
    c:inc()
    i = i + 1
  end
  print(c.n)
end"#, 0);
    assert!(!diags.has_errors(), "method_writeback_loop_18: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"12
"#);
}

#[test]
fn method_writeback_loop_19() {
    let (out, diags) = interpret(r#"C = struct
  n: Int = 0
end

C.inc = fn(self) do
  self.n = self.n + 2
  return self.n
end

fn main() do
  let c = C { n = 0 }
  let i = 0
  while i < 7 do
    c:inc()
    i = i + 1
  end
  print(c.n)
end"#, 0);
    assert!(!diags.has_errors(), "method_writeback_loop_19: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"14
"#);
}

#[test]
fn method_writeback_loop_20() {
    let (out, diags) = interpret(r#"C = struct
  n: Int = 0
end

C.inc = fn(self) do
  self.n = self.n + 2
  return self.n
end

fn main() do
  let c = C { n = 0 }
  let i = 0
  while i < 3 do
    c:inc()
    i = i + 1
  end
  print(c.n)
end"#, 0);
    assert!(!diags.has_errors(), "method_writeback_loop_20: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"6
"#);
}

#[test]
fn method_read_fields_1() {
    let (out, diags) = interpret(r#"P = struct
  x: Int = 0
  y: Int = 0
end

P.area = fn(self, k) do
  return (self.x + self.y) * k
end

fn main() do
  let p = P { x = 1, y = 3 }
  print(p:area(2))
  print(p:area(3))
end"#, 0);
    assert!(!diags.has_errors(), "method_read_fields_1: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"8
12
"#);
}

#[test]
fn method_read_fields_2() {
    let (out, diags) = interpret(r#"P = struct
  x: Int = 0
  y: Int = 0
end

P.area = fn(self, k) do
  return (self.x + self.y) * k
end

fn main() do
  let p = P { x = 2, y = 6 }
  print(p:area(2))
  print(p:area(3))
end"#, 0);
    assert!(!diags.has_errors(), "method_read_fields_2: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"16
24
"#);
}

#[test]
fn method_read_fields_3() {
    let (out, diags) = interpret(r#"P = struct
  x: Int = 0
  y: Int = 0
end

P.area = fn(self, k) do
  return (self.x + self.y) * k
end

fn main() do
  let p = P { x = 3, y = 9 }
  print(p:area(2))
  print(p:area(3))
end"#, 0);
    assert!(!diags.has_errors(), "method_read_fields_3: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"24
36
"#);
}

#[test]
fn method_read_fields_4() {
    let (out, diags) = interpret(r#"P = struct
  x: Int = 0
  y: Int = 0
end

P.area = fn(self, k) do
  return (self.x + self.y) * k
end

fn main() do
  let p = P { x = 4, y = 12 }
  print(p:area(2))
  print(p:area(3))
end"#, 0);
    assert!(!diags.has_errors(), "method_read_fields_4: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"32
48
"#);
}

#[test]
fn method_read_fields_5() {
    let (out, diags) = interpret(r#"P = struct
  x: Int = 0
  y: Int = 0
end

P.area = fn(self, k) do
  return (self.x + self.y) * k
end

fn main() do
  let p = P { x = 5, y = 15 }
  print(p:area(2))
  print(p:area(3))
end"#, 0);
    assert!(!diags.has_errors(), "method_read_fields_5: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"40
60
"#);
}

#[test]
fn method_read_fields_6() {
    let (out, diags) = interpret(r#"P = struct
  x: Int = 0
  y: Int = 0
end

P.area = fn(self, k) do
  return (self.x + self.y) * k
end

fn main() do
  let p = P { x = 6, y = 18 }
  print(p:area(2))
  print(p:area(3))
end"#, 0);
    assert!(!diags.has_errors(), "method_read_fields_6: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"48
72
"#);
}

#[test]
fn method_read_fields_7() {
    let (out, diags) = interpret(r#"P = struct
  x: Int = 0
  y: Int = 0
end

P.area = fn(self, k) do
  return (self.x + self.y) * k
end

fn main() do
  let p = P { x = 7, y = 21 }
  print(p:area(2))
  print(p:area(3))
end"#, 0);
    assert!(!diags.has_errors(), "method_read_fields_7: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"56
84
"#);
}

#[test]
fn method_read_fields_8() {
    let (out, diags) = interpret(r#"P = struct
  x: Int = 0
  y: Int = 0
end

P.area = fn(self, k) do
  return (self.x + self.y) * k
end

fn main() do
  let p = P { x = 8, y = 24 }
  print(p:area(2))
  print(p:area(3))
end"#, 0);
    assert!(!diags.has_errors(), "method_read_fields_8: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"64
96
"#);
}

#[test]
fn method_read_fields_9() {
    let (out, diags) = interpret(r#"P = struct
  x: Int = 0
  y: Int = 0
end

P.area = fn(self, k) do
  return (self.x + self.y) * k
end

fn main() do
  let p = P { x = 9, y = 27 }
  print(p:area(2))
  print(p:area(3))
end"#, 0);
    assert!(!diags.has_errors(), "method_read_fields_9: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"72
108
"#);
}

#[test]
fn method_read_fields_10() {
    let (out, diags) = interpret(r#"P = struct
  x: Int = 0
  y: Int = 0
end

P.area = fn(self, k) do
  return (self.x + self.y) * k
end

fn main() do
  let p = P { x = 10, y = 30 }
  print(p:area(2))
  print(p:area(3))
end"#, 0);
    assert!(!diags.has_errors(), "method_read_fields_10: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"80
120
"#);
}

#[test]
fn method_read_fields_11() {
    let (out, diags) = interpret(r#"P = struct
  x: Int = 0
  y: Int = 0
end

P.area = fn(self, k) do
  return (self.x + self.y) * k
end

fn main() do
  let p = P { x = 11, y = 33 }
  print(p:area(2))
  print(p:area(3))
end"#, 0);
    assert!(!diags.has_errors(), "method_read_fields_11: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"88
132
"#);
}

#[test]
fn method_read_fields_12() {
    let (out, diags) = interpret(r#"P = struct
  x: Int = 0
  y: Int = 0
end

P.area = fn(self, k) do
  return (self.x + self.y) * k
end

fn main() do
  let p = P { x = 12, y = 36 }
  print(p:area(2))
  print(p:area(3))
end"#, 0);
    assert!(!diags.has_errors(), "method_read_fields_12: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"96
144
"#);
}

#[test]
fn method_read_fields_13() {
    let (out, diags) = interpret(r#"P = struct
  x: Int = 0
  y: Int = 0
end

P.area = fn(self, k) do
  return (self.x + self.y) * k
end

fn main() do
  let p = P { x = 13, y = 39 }
  print(p:area(2))
  print(p:area(3))
end"#, 0);
    assert!(!diags.has_errors(), "method_read_fields_13: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"104
156
"#);
}

#[test]
fn method_read_fields_14() {
    let (out, diags) = interpret(r#"P = struct
  x: Int = 0
  y: Int = 0
end

P.area = fn(self, k) do
  return (self.x + self.y) * k
end

fn main() do
  let p = P { x = 14, y = 42 }
  print(p:area(2))
  print(p:area(3))
end"#, 0);
    assert!(!diags.has_errors(), "method_read_fields_14: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"112
168
"#);
}

#[test]
fn method_read_fields_15() {
    let (out, diags) = interpret(r#"P = struct
  x: Int = 0
  y: Int = 0
end

P.area = fn(self, k) do
  return (self.x + self.y) * k
end

fn main() do
  let p = P { x = 15, y = 45 }
  print(p:area(2))
  print(p:area(3))
end"#, 0);
    assert!(!diags.has_errors(), "method_read_fields_15: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"120
180
"#);
}

#[test]
fn method_read_fields_16() {
    let (out, diags) = interpret(r#"P = struct
  x: Int = 0
  y: Int = 0
end

P.area = fn(self, k) do
  return (self.x + self.y) * k
end

fn main() do
  let p = P { x = 16, y = 48 }
  print(p:area(2))
  print(p:area(3))
end"#, 0);
    assert!(!diags.has_errors(), "method_read_fields_16: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"128
192
"#);
}

#[test]
fn method_read_fields_17() {
    let (out, diags) = interpret(r#"P = struct
  x: Int = 0
  y: Int = 0
end

P.area = fn(self, k) do
  return (self.x + self.y) * k
end

fn main() do
  let p = P { x = 17, y = 51 }
  print(p:area(2))
  print(p:area(3))
end"#, 0);
    assert!(!diags.has_errors(), "method_read_fields_17: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"136
204
"#);
}

#[test]
fn method_read_fields_18() {
    let (out, diags) = interpret(r#"P = struct
  x: Int = 0
  y: Int = 0
end

P.area = fn(self, k) do
  return (self.x + self.y) * k
end

fn main() do
  let p = P { x = 18, y = 54 }
  print(p:area(2))
  print(p:area(3))
end"#, 0);
    assert!(!diags.has_errors(), "method_read_fields_18: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"144
216
"#);
}

#[test]
fn method_read_fields_19() {
    let (out, diags) = interpret(r#"P = struct
  x: Int = 0
  y: Int = 0
end

P.area = fn(self, k) do
  return (self.x + self.y) * k
end

fn main() do
  let p = P { x = 19, y = 57 }
  print(p:area(2))
  print(p:area(3))
end"#, 0);
    assert!(!diags.has_errors(), "method_read_fields_19: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"152
228
"#);
}

#[test]
fn method_read_fields_20() {
    let (out, diags) = interpret(r#"P = struct
  x: Int = 0
  y: Int = 0
end

P.area = fn(self, k) do
  return (self.x + self.y) * k
end

fn main() do
  let p = P { x = 20, y = 60 }
  print(p:area(2))
  print(p:area(3))
end"#, 0);
    assert!(!diags.has_errors(), "method_read_fields_20: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"160
240
"#);
}

#[test]
fn method_preserves_fields_1() {
    let (out, diags) = interpret(r#"P = struct
  x: Int = 0
  y: Int = 0
end

P.setx = fn(self, v) do
  self.x = v
  return v
end

fn main() do
  let p = P { x = 0, y = 7 }
  p:setx(1)
  print(p.x)
  print(p.y)
end"#, 0);
    assert!(!diags.has_errors(), "method_preserves_fields_1: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"1
7
"#);
}

#[test]
fn method_preserves_fields_2() {
    let (out, diags) = interpret(r#"P = struct
  x: Int = 0
  y: Int = 0
end

P.setx = fn(self, v) do
  self.x = v
  return v
end

fn main() do
  let p = P { x = 0, y = 14 }
  p:setx(2)
  print(p.x)
  print(p.y)
end"#, 0);
    assert!(!diags.has_errors(), "method_preserves_fields_2: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"2
14
"#);
}

#[test]
fn method_preserves_fields_3() {
    let (out, diags) = interpret(r#"P = struct
  x: Int = 0
  y: Int = 0
end

P.setx = fn(self, v) do
  self.x = v
  return v
end

fn main() do
  let p = P { x = 0, y = 21 }
  p:setx(3)
  print(p.x)
  print(p.y)
end"#, 0);
    assert!(!diags.has_errors(), "method_preserves_fields_3: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"3
21
"#);
}

#[test]
fn method_preserves_fields_4() {
    let (out, diags) = interpret(r#"P = struct
  x: Int = 0
  y: Int = 0
end

P.setx = fn(self, v) do
  self.x = v
  return v
end

fn main() do
  let p = P { x = 0, y = 28 }
  p:setx(4)
  print(p.x)
  print(p.y)
end"#, 0);
    assert!(!diags.has_errors(), "method_preserves_fields_4: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"4
28
"#);
}

#[test]
fn method_preserves_fields_5() {
    let (out, diags) = interpret(r#"P = struct
  x: Int = 0
  y: Int = 0
end

P.setx = fn(self, v) do
  self.x = v
  return v
end

fn main() do
  let p = P { x = 0, y = 35 }
  p:setx(5)
  print(p.x)
  print(p.y)
end"#, 0);
    assert!(!diags.has_errors(), "method_preserves_fields_5: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"5
35
"#);
}

#[test]
fn method_preserves_fields_6() {
    let (out, diags) = interpret(r#"P = struct
  x: Int = 0
  y: Int = 0
end

P.setx = fn(self, v) do
  self.x = v
  return v
end

fn main() do
  let p = P { x = 0, y = 42 }
  p:setx(6)
  print(p.x)
  print(p.y)
end"#, 0);
    assert!(!diags.has_errors(), "method_preserves_fields_6: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"6
42
"#);
}

#[test]
fn method_preserves_fields_7() {
    let (out, diags) = interpret(r#"P = struct
  x: Int = 0
  y: Int = 0
end

P.setx = fn(self, v) do
  self.x = v
  return v
end

fn main() do
  let p = P { x = 0, y = 49 }
  p:setx(7)
  print(p.x)
  print(p.y)
end"#, 0);
    assert!(!diags.has_errors(), "method_preserves_fields_7: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"7
49
"#);
}

#[test]
fn method_preserves_fields_8() {
    let (out, diags) = interpret(r#"P = struct
  x: Int = 0
  y: Int = 0
end

P.setx = fn(self, v) do
  self.x = v
  return v
end

fn main() do
  let p = P { x = 0, y = 56 }
  p:setx(8)
  print(p.x)
  print(p.y)
end"#, 0);
    assert!(!diags.has_errors(), "method_preserves_fields_8: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"8
56
"#);
}

#[test]
fn method_preserves_fields_9() {
    let (out, diags) = interpret(r#"P = struct
  x: Int = 0
  y: Int = 0
end

P.setx = fn(self, v) do
  self.x = v
  return v
end

fn main() do
  let p = P { x = 0, y = 63 }
  p:setx(9)
  print(p.x)
  print(p.y)
end"#, 0);
    assert!(!diags.has_errors(), "method_preserves_fields_9: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"9
63
"#);
}

#[test]
fn method_preserves_fields_10() {
    let (out, diags) = interpret(r#"P = struct
  x: Int = 0
  y: Int = 0
end

P.setx = fn(self, v) do
  self.x = v
  return v
end

fn main() do
  let p = P { x = 0, y = 70 }
  p:setx(10)
  print(p.x)
  print(p.y)
end"#, 0);
    assert!(!diags.has_errors(), "method_preserves_fields_10: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"10
70
"#);
}

#[test]
fn method_preserves_fields_11() {
    let (out, diags) = interpret(r#"P = struct
  x: Int = 0
  y: Int = 0
end

P.setx = fn(self, v) do
  self.x = v
  return v
end

fn main() do
  let p = P { x = 0, y = 77 }
  p:setx(11)
  print(p.x)
  print(p.y)
end"#, 0);
    assert!(!diags.has_errors(), "method_preserves_fields_11: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"11
77
"#);
}

#[test]
fn method_preserves_fields_12() {
    let (out, diags) = interpret(r#"P = struct
  x: Int = 0
  y: Int = 0
end

P.setx = fn(self, v) do
  self.x = v
  return v
end

fn main() do
  let p = P { x = 0, y = 84 }
  p:setx(12)
  print(p.x)
  print(p.y)
end"#, 0);
    assert!(!diags.has_errors(), "method_preserves_fields_12: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"12
84
"#);
}

#[test]
fn method_preserves_fields_13() {
    let (out, diags) = interpret(r#"P = struct
  x: Int = 0
  y: Int = 0
end

P.setx = fn(self, v) do
  self.x = v
  return v
end

fn main() do
  let p = P { x = 0, y = 91 }
  p:setx(13)
  print(p.x)
  print(p.y)
end"#, 0);
    assert!(!diags.has_errors(), "method_preserves_fields_13: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"13
91
"#);
}

#[test]
fn method_preserves_fields_14() {
    let (out, diags) = interpret(r#"P = struct
  x: Int = 0
  y: Int = 0
end

P.setx = fn(self, v) do
  self.x = v
  return v
end

fn main() do
  let p = P { x = 0, y = 98 }
  p:setx(14)
  print(p.x)
  print(p.y)
end"#, 0);
    assert!(!diags.has_errors(), "method_preserves_fields_14: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"14
98
"#);
}

#[test]
fn method_preserves_fields_15() {
    let (out, diags) = interpret(r#"P = struct
  x: Int = 0
  y: Int = 0
end

P.setx = fn(self, v) do
  self.x = v
  return v
end

fn main() do
  let p = P { x = 0, y = 105 }
  p:setx(15)
  print(p.x)
  print(p.y)
end"#, 0);
    assert!(!diags.has_errors(), "method_preserves_fields_15: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"15
105
"#);
}

#[test]
fn method_preserves_fields_16() {
    let (out, diags) = interpret(r#"P = struct
  x: Int = 0
  y: Int = 0
end

P.setx = fn(self, v) do
  self.x = v
  return v
end

fn main() do
  let p = P { x = 0, y = 112 }
  p:setx(16)
  print(p.x)
  print(p.y)
end"#, 0);
    assert!(!diags.has_errors(), "method_preserves_fields_16: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"16
112
"#);
}

#[test]
fn method_preserves_fields_17() {
    let (out, diags) = interpret(r#"P = struct
  x: Int = 0
  y: Int = 0
end

P.setx = fn(self, v) do
  self.x = v
  return v
end

fn main() do
  let p = P { x = 0, y = 119 }
  p:setx(17)
  print(p.x)
  print(p.y)
end"#, 0);
    assert!(!diags.has_errors(), "method_preserves_fields_17: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"17
119
"#);
}

#[test]
fn method_preserves_fields_18() {
    let (out, diags) = interpret(r#"P = struct
  x: Int = 0
  y: Int = 0
end

P.setx = fn(self, v) do
  self.x = v
  return v
end

fn main() do
  let p = P { x = 0, y = 126 }
  p:setx(18)
  print(p.x)
  print(p.y)
end"#, 0);
    assert!(!diags.has_errors(), "method_preserves_fields_18: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"18
126
"#);
}

#[test]
fn method_preserves_fields_19() {
    let (out, diags) = interpret(r#"P = struct
  x: Int = 0
  y: Int = 0
end

P.setx = fn(self, v) do
  self.x = v
  return v
end

fn main() do
  let p = P { x = 0, y = 133 }
  p:setx(19)
  print(p.x)
  print(p.y)
end"#, 0);
    assert!(!diags.has_errors(), "method_preserves_fields_19: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"19
133
"#);
}

#[test]
fn method_preserves_fields_20() {
    let (out, diags) = interpret(r#"P = struct
  x: Int = 0
  y: Int = 0
end

P.setx = fn(self, v) do
  self.x = v
  return v
end

fn main() do
  let p = P { x = 0, y = 140 }
  p:setx(20)
  print(p.x)
  print(p.y)
end"#, 0);
    assert!(!diags.has_errors(), "method_preserves_fields_20: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"20
140
"#);
}

#[test]
fn method_colon_dot_equiv_1() {
    let (out, diags) = interpret(r#"D = struct
  v: Int = 0
end

D.double = fn(self) do
  self.v = self.v * 2
  return self.v
end

fn main() do
  let a = D { v = 5 }
  let b = D { v = 5 }
  let r1 = a:double()
  let r2 = b.double()
  print(r1)
  print(r2)
  print(a.v)
  print(b.v)
end"#, 0);
    assert!(!diags.has_errors(), "method_colon_dot_equiv_1: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"10
10
10
10
"#);
}

#[test]
fn method_colon_dot_equiv_2() {
    let (out, diags) = interpret(r#"D = struct
  v: Int = 0
end

D.double = fn(self) do
  self.v = self.v * 2
  return self.v
end

fn main() do
  let a = D { v = 10 }
  let b = D { v = 10 }
  let r1 = a:double()
  let r2 = b.double()
  print(r1)
  print(r2)
  print(a.v)
  print(b.v)
end"#, 0);
    assert!(!diags.has_errors(), "method_colon_dot_equiv_2: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"20
20
20
20
"#);
}

#[test]
fn method_colon_dot_equiv_3() {
    let (out, diags) = interpret(r#"D = struct
  v: Int = 0
end

D.double = fn(self) do
  self.v = self.v * 2
  return self.v
end

fn main() do
  let a = D { v = 15 }
  let b = D { v = 15 }
  let r1 = a:double()
  let r2 = b.double()
  print(r1)
  print(r2)
  print(a.v)
  print(b.v)
end"#, 0);
    assert!(!diags.has_errors(), "method_colon_dot_equiv_3: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"30
30
30
30
"#);
}

#[test]
fn method_colon_dot_equiv_4() {
    let (out, diags) = interpret(r#"D = struct
  v: Int = 0
end

D.double = fn(self) do
  self.v = self.v * 2
  return self.v
end

fn main() do
  let a = D { v = 20 }
  let b = D { v = 20 }
  let r1 = a:double()
  let r2 = b.double()
  print(r1)
  print(r2)
  print(a.v)
  print(b.v)
end"#, 0);
    assert!(!diags.has_errors(), "method_colon_dot_equiv_4: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"40
40
40
40
"#);
}

#[test]
fn method_colon_dot_equiv_5() {
    let (out, diags) = interpret(r#"D = struct
  v: Int = 0
end

D.double = fn(self) do
  self.v = self.v * 2
  return self.v
end

fn main() do
  let a = D { v = 25 }
  let b = D { v = 25 }
  let r1 = a:double()
  let r2 = b.double()
  print(r1)
  print(r2)
  print(a.v)
  print(b.v)
end"#, 0);
    assert!(!diags.has_errors(), "method_colon_dot_equiv_5: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"50
50
50
50
"#);
}

#[test]
fn method_colon_dot_equiv_6() {
    let (out, diags) = interpret(r#"D = struct
  v: Int = 0
end

D.double = fn(self) do
  self.v = self.v * 2
  return self.v
end

fn main() do
  let a = D { v = 30 }
  let b = D { v = 30 }
  let r1 = a:double()
  let r2 = b.double()
  print(r1)
  print(r2)
  print(a.v)
  print(b.v)
end"#, 0);
    assert!(!diags.has_errors(), "method_colon_dot_equiv_6: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"60
60
60
60
"#);
}

#[test]
fn method_colon_dot_equiv_7() {
    let (out, diags) = interpret(r#"D = struct
  v: Int = 0
end

D.double = fn(self) do
  self.v = self.v * 2
  return self.v
end

fn main() do
  let a = D { v = 35 }
  let b = D { v = 35 }
  let r1 = a:double()
  let r2 = b.double()
  print(r1)
  print(r2)
  print(a.v)
  print(b.v)
end"#, 0);
    assert!(!diags.has_errors(), "method_colon_dot_equiv_7: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"70
70
70
70
"#);
}

#[test]
fn method_colon_dot_equiv_8() {
    let (out, diags) = interpret(r#"D = struct
  v: Int = 0
end

D.double = fn(self) do
  self.v = self.v * 2
  return self.v
end

fn main() do
  let a = D { v = 40 }
  let b = D { v = 40 }
  let r1 = a:double()
  let r2 = b.double()
  print(r1)
  print(r2)
  print(a.v)
  print(b.v)
end"#, 0);
    assert!(!diags.has_errors(), "method_colon_dot_equiv_8: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"80
80
80
80
"#);
}

#[test]
fn method_colon_dot_equiv_9() {
    let (out, diags) = interpret(r#"D = struct
  v: Int = 0
end

D.double = fn(self) do
  self.v = self.v * 2
  return self.v
end

fn main() do
  let a = D { v = 45 }
  let b = D { v = 45 }
  let r1 = a:double()
  let r2 = b.double()
  print(r1)
  print(r2)
  print(a.v)
  print(b.v)
end"#, 0);
    assert!(!diags.has_errors(), "method_colon_dot_equiv_9: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"90
90
90
90
"#);
}

#[test]
fn method_colon_dot_equiv_10() {
    let (out, diags) = interpret(r#"D = struct
  v: Int = 0
end

D.double = fn(self) do
  self.v = self.v * 2
  return self.v
end

fn main() do
  let a = D { v = 50 }
  let b = D { v = 50 }
  let r1 = a:double()
  let r2 = b.double()
  print(r1)
  print(r2)
  print(a.v)
  print(b.v)
end"#, 0);
    assert!(!diags.has_errors(), "method_colon_dot_equiv_10: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"100
100
100
100
"#);
}

#[test]
fn method_colon_dot_equiv_11() {
    let (out, diags) = interpret(r#"D = struct
  v: Int = 0
end

D.double = fn(self) do
  self.v = self.v * 2
  return self.v
end

fn main() do
  let a = D { v = 55 }
  let b = D { v = 55 }
  let r1 = a:double()
  let r2 = b.double()
  print(r1)
  print(r2)
  print(a.v)
  print(b.v)
end"#, 0);
    assert!(!diags.has_errors(), "method_colon_dot_equiv_11: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"110
110
110
110
"#);
}

#[test]
fn method_colon_dot_equiv_12() {
    let (out, diags) = interpret(r#"D = struct
  v: Int = 0
end

D.double = fn(self) do
  self.v = self.v * 2
  return self.v
end

fn main() do
  let a = D { v = 60 }
  let b = D { v = 60 }
  let r1 = a:double()
  let r2 = b.double()
  print(r1)
  print(r2)
  print(a.v)
  print(b.v)
end"#, 0);
    assert!(!diags.has_errors(), "method_colon_dot_equiv_12: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"120
120
120
120
"#);
}

#[test]
fn method_colon_dot_equiv_13() {
    let (out, diags) = interpret(r#"D = struct
  v: Int = 0
end

D.double = fn(self) do
  self.v = self.v * 2
  return self.v
end

fn main() do
  let a = D { v = 65 }
  let b = D { v = 65 }
  let r1 = a:double()
  let r2 = b.double()
  print(r1)
  print(r2)
  print(a.v)
  print(b.v)
end"#, 0);
    assert!(!diags.has_errors(), "method_colon_dot_equiv_13: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"130
130
130
130
"#);
}

#[test]
fn method_colon_dot_equiv_14() {
    let (out, diags) = interpret(r#"D = struct
  v: Int = 0
end

D.double = fn(self) do
  self.v = self.v * 2
  return self.v
end

fn main() do
  let a = D { v = 70 }
  let b = D { v = 70 }
  let r1 = a:double()
  let r2 = b.double()
  print(r1)
  print(r2)
  print(a.v)
  print(b.v)
end"#, 0);
    assert!(!diags.has_errors(), "method_colon_dot_equiv_14: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"140
140
140
140
"#);
}

#[test]
fn method_colon_dot_equiv_15() {
    let (out, diags) = interpret(r#"D = struct
  v: Int = 0
end

D.double = fn(self) do
  self.v = self.v * 2
  return self.v
end

fn main() do
  let a = D { v = 75 }
  let b = D { v = 75 }
  let r1 = a:double()
  let r2 = b.double()
  print(r1)
  print(r2)
  print(a.v)
  print(b.v)
end"#, 0);
    assert!(!diags.has_errors(), "method_colon_dot_equiv_15: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"150
150
150
150
"#);
}

#[test]
fn method_colon_dot_equiv_16() {
    let (out, diags) = interpret(r#"D = struct
  v: Int = 0
end

D.double = fn(self) do
  self.v = self.v * 2
  return self.v
end

fn main() do
  let a = D { v = 80 }
  let b = D { v = 80 }
  let r1 = a:double()
  let r2 = b.double()
  print(r1)
  print(r2)
  print(a.v)
  print(b.v)
end"#, 0);
    assert!(!diags.has_errors(), "method_colon_dot_equiv_16: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"160
160
160
160
"#);
}

#[test]
fn method_colon_dot_equiv_17() {
    let (out, diags) = interpret(r#"D = struct
  v: Int = 0
end

D.double = fn(self) do
  self.v = self.v * 2
  return self.v
end

fn main() do
  let a = D { v = 85 }
  let b = D { v = 85 }
  let r1 = a:double()
  let r2 = b.double()
  print(r1)
  print(r2)
  print(a.v)
  print(b.v)
end"#, 0);
    assert!(!diags.has_errors(), "method_colon_dot_equiv_17: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"170
170
170
170
"#);
}

#[test]
fn method_colon_dot_equiv_18() {
    let (out, diags) = interpret(r#"D = struct
  v: Int = 0
end

D.double = fn(self) do
  self.v = self.v * 2
  return self.v
end

fn main() do
  let a = D { v = 90 }
  let b = D { v = 90 }
  let r1 = a:double()
  let r2 = b.double()
  print(r1)
  print(r2)
  print(a.v)
  print(b.v)
end"#, 0);
    assert!(!diags.has_errors(), "method_colon_dot_equiv_18: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"180
180
180
180
"#);
}

#[test]
fn method_colon_dot_equiv_19() {
    let (out, diags) = interpret(r#"D = struct
  v: Int = 0
end

D.double = fn(self) do
  self.v = self.v * 2
  return self.v
end

fn main() do
  let a = D { v = 95 }
  let b = D { v = 95 }
  let r1 = a:double()
  let r2 = b.double()
  print(r1)
  print(r2)
  print(a.v)
  print(b.v)
end"#, 0);
    assert!(!diags.has_errors(), "method_colon_dot_equiv_19: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"190
190
190
190
"#);
}

#[test]
fn method_colon_dot_equiv_20() {
    let (out, diags) = interpret(r#"D = struct
  v: Int = 0
end

D.double = fn(self) do
  self.v = self.v * 2
  return self.v
end

fn main() do
  let a = D { v = 100 }
  let b = D { v = 100 }
  let r1 = a:double()
  let r2 = b.double()
  print(r1)
  print(r2)
  print(a.v)
  print(b.v)
end"#, 0);
    assert!(!diags.has_errors(), "method_colon_dot_equiv_20: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"200
200
200
200
"#);
}

#[test]
fn method_compound_self_1() {
    let (out, diags) = interpret(r#"C = struct
  n: Int = 0
end

C.add = fn(self, d) do
  self.n += d
  return self.n
end

fn main() do
  let c = C { n = 1 }
  c:add(10)
  c:add(5)
  print(c.n)
end"#, 0);
    assert!(!diags.has_errors(), "method_compound_self_1: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"16
"#);
}

#[test]
fn method_compound_self_2() {
    let (out, diags) = interpret(r#"C = struct
  n: Int = 0
end

C.add = fn(self, d) do
  self.n += d
  return self.n
end

fn main() do
  let c = C { n = 2 }
  c:add(10)
  c:add(5)
  print(c.n)
end"#, 0);
    assert!(!diags.has_errors(), "method_compound_self_2: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"17
"#);
}

#[test]
fn method_compound_self_3() {
    let (out, diags) = interpret(r#"C = struct
  n: Int = 0
end

C.add = fn(self, d) do
  self.n += d
  return self.n
end

fn main() do
  let c = C { n = 3 }
  c:add(10)
  c:add(5)
  print(c.n)
end"#, 0);
    assert!(!diags.has_errors(), "method_compound_self_3: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"18
"#);
}

#[test]
fn method_compound_self_4() {
    let (out, diags) = interpret(r#"C = struct
  n: Int = 0
end

C.add = fn(self, d) do
  self.n += d
  return self.n
end

fn main() do
  let c = C { n = 4 }
  c:add(10)
  c:add(5)
  print(c.n)
end"#, 0);
    assert!(!diags.has_errors(), "method_compound_self_4: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"19
"#);
}

#[test]
fn method_compound_self_5() {
    let (out, diags) = interpret(r#"C = struct
  n: Int = 0
end

C.add = fn(self, d) do
  self.n += d
  return self.n
end

fn main() do
  let c = C { n = 5 }
  c:add(10)
  c:add(5)
  print(c.n)
end"#, 0);
    assert!(!diags.has_errors(), "method_compound_self_5: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"20
"#);
}

#[test]
fn method_compound_self_6() {
    let (out, diags) = interpret(r#"C = struct
  n: Int = 0
end

C.add = fn(self, d) do
  self.n += d
  return self.n
end

fn main() do
  let c = C { n = 6 }
  c:add(10)
  c:add(5)
  print(c.n)
end"#, 0);
    assert!(!diags.has_errors(), "method_compound_self_6: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"21
"#);
}

#[test]
fn method_compound_self_7() {
    let (out, diags) = interpret(r#"C = struct
  n: Int = 0
end

C.add = fn(self, d) do
  self.n += d
  return self.n
end

fn main() do
  let c = C { n = 7 }
  c:add(10)
  c:add(5)
  print(c.n)
end"#, 0);
    assert!(!diags.has_errors(), "method_compound_self_7: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"22
"#);
}

#[test]
fn method_compound_self_8() {
    let (out, diags) = interpret(r#"C = struct
  n: Int = 0
end

C.add = fn(self, d) do
  self.n += d
  return self.n
end

fn main() do
  let c = C { n = 8 }
  c:add(10)
  c:add(5)
  print(c.n)
end"#, 0);
    assert!(!diags.has_errors(), "method_compound_self_8: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"23
"#);
}

#[test]
fn method_compound_self_9() {
    let (out, diags) = interpret(r#"C = struct
  n: Int = 0
end

C.add = fn(self, d) do
  self.n += d
  return self.n
end

fn main() do
  let c = C { n = 9 }
  c:add(10)
  c:add(5)
  print(c.n)
end"#, 0);
    assert!(!diags.has_errors(), "method_compound_self_9: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"24
"#);
}

#[test]
fn method_compound_self_10() {
    let (out, diags) = interpret(r#"C = struct
  n: Int = 0
end

C.add = fn(self, d) do
  self.n += d
  return self.n
end

fn main() do
  let c = C { n = 10 }
  c:add(10)
  c:add(5)
  print(c.n)
end"#, 0);
    assert!(!diags.has_errors(), "method_compound_self_10: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"25
"#);
}

#[test]
fn method_compound_self_11() {
    let (out, diags) = interpret(r#"C = struct
  n: Int = 0
end

C.add = fn(self, d) do
  self.n += d
  return self.n
end

fn main() do
  let c = C { n = 11 }
  c:add(10)
  c:add(5)
  print(c.n)
end"#, 0);
    assert!(!diags.has_errors(), "method_compound_self_11: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"26
"#);
}

#[test]
fn method_compound_self_12() {
    let (out, diags) = interpret(r#"C = struct
  n: Int = 0
end

C.add = fn(self, d) do
  self.n += d
  return self.n
end

fn main() do
  let c = C { n = 12 }
  c:add(10)
  c:add(5)
  print(c.n)
end"#, 0);
    assert!(!diags.has_errors(), "method_compound_self_12: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"27
"#);
}

#[test]
fn method_compound_self_13() {
    let (out, diags) = interpret(r#"C = struct
  n: Int = 0
end

C.add = fn(self, d) do
  self.n += d
  return self.n
end

fn main() do
  let c = C { n = 13 }
  c:add(10)
  c:add(5)
  print(c.n)
end"#, 0);
    assert!(!diags.has_errors(), "method_compound_self_13: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"28
"#);
}

#[test]
fn method_compound_self_14() {
    let (out, diags) = interpret(r#"C = struct
  n: Int = 0
end

C.add = fn(self, d) do
  self.n += d
  return self.n
end

fn main() do
  let c = C { n = 14 }
  c:add(10)
  c:add(5)
  print(c.n)
end"#, 0);
    assert!(!diags.has_errors(), "method_compound_self_14: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"29
"#);
}

#[test]
fn method_compound_self_15() {
    let (out, diags) = interpret(r#"C = struct
  n: Int = 0
end

C.add = fn(self, d) do
  self.n += d
  return self.n
end

fn main() do
  let c = C { n = 15 }
  c:add(10)
  c:add(5)
  print(c.n)
end"#, 0);
    assert!(!diags.has_errors(), "method_compound_self_15: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"30
"#);
}

#[test]
fn method_in_for_1() {
    let (out, diags) = interpret(r#"C = struct
  n: Int = 0
end

C.inc = fn(self) do
  self.n = self.n + 1
  return self.n
end

fn main() do
  let c = C { n = 0 }
  for k in 5 do
    c:inc()
  end
  print(c.n)
end"#, 0);
    assert!(!diags.has_errors(), "method_in_for_1: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"5
"#);
}

#[test]
fn method_in_for_2() {
    let (out, diags) = interpret(r#"C = struct
  n: Int = 0
end

C.inc = fn(self) do
  self.n = self.n + 1
  return self.n
end

fn main() do
  let c = C { n = 0 }
  for k in 6 do
    c:inc()
  end
  print(c.n)
end"#, 0);
    assert!(!diags.has_errors(), "method_in_for_2: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"6
"#);
}

#[test]
fn method_in_for_3() {
    let (out, diags) = interpret(r#"C = struct
  n: Int = 0
end

C.inc = fn(self) do
  self.n = self.n + 1
  return self.n
end

fn main() do
  let c = C { n = 0 }
  for k in 7 do
    c:inc()
  end
  print(c.n)
end"#, 0);
    assert!(!diags.has_errors(), "method_in_for_3: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"7
"#);
}

#[test]
fn method_in_for_4() {
    let (out, diags) = interpret(r#"C = struct
  n: Int = 0
end

C.inc = fn(self) do
  self.n = self.n + 1
  return self.n
end

fn main() do
  let c = C { n = 0 }
  for k in 4 do
    c:inc()
  end
  print(c.n)
end"#, 0);
    assert!(!diags.has_errors(), "method_in_for_4: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"4
"#);
}

#[test]
fn method_in_for_5() {
    let (out, diags) = interpret(r#"C = struct
  n: Int = 0
end

C.inc = fn(self) do
  self.n = self.n + 1
  return self.n
end

fn main() do
  let c = C { n = 0 }
  for k in 5 do
    c:inc()
  end
  print(c.n)
end"#, 0);
    assert!(!diags.has_errors(), "method_in_for_5: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"5
"#);
}

#[test]
fn method_in_for_6() {
    let (out, diags) = interpret(r#"C = struct
  n: Int = 0
end

C.inc = fn(self) do
  self.n = self.n + 1
  return self.n
end

fn main() do
  let c = C { n = 0 }
  for k in 6 do
    c:inc()
  end
  print(c.n)
end"#, 0);
    assert!(!diags.has_errors(), "method_in_for_6: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"6
"#);
}

#[test]
fn method_in_for_7() {
    let (out, diags) = interpret(r#"C = struct
  n: Int = 0
end

C.inc = fn(self) do
  self.n = self.n + 1
  return self.n
end

fn main() do
  let c = C { n = 0 }
  for k in 7 do
    c:inc()
  end
  print(c.n)
end"#, 0);
    assert!(!diags.has_errors(), "method_in_for_7: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"7
"#);
}

#[test]
fn method_in_for_8() {
    let (out, diags) = interpret(r#"C = struct
  n: Int = 0
end

C.inc = fn(self) do
  self.n = self.n + 1
  return self.n
end

fn main() do
  let c = C { n = 0 }
  for k in 4 do
    c:inc()
  end
  print(c.n)
end"#, 0);
    assert!(!diags.has_errors(), "method_in_for_8: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"4
"#);
}

#[test]
fn method_in_for_9() {
    let (out, diags) = interpret(r#"C = struct
  n: Int = 0
end

C.inc = fn(self) do
  self.n = self.n + 1
  return self.n
end

fn main() do
  let c = C { n = 0 }
  for k in 5 do
    c:inc()
  end
  print(c.n)
end"#, 0);
    assert!(!diags.has_errors(), "method_in_for_9: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"5
"#);
}

#[test]
fn method_in_for_10() {
    let (out, diags) = interpret(r#"C = struct
  n: Int = 0
end

C.inc = fn(self) do
  self.n = self.n + 1
  return self.n
end

fn main() do
  let c = C { n = 0 }
  for k in 6 do
    c:inc()
  end
  print(c.n)
end"#, 0);
    assert!(!diags.has_errors(), "method_in_for_10: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"6
"#);
}

#[test]
fn method_in_for_11() {
    let (out, diags) = interpret(r#"C = struct
  n: Int = 0
end

C.inc = fn(self) do
  self.n = self.n + 1
  return self.n
end

fn main() do
  let c = C { n = 0 }
  for k in 7 do
    c:inc()
  end
  print(c.n)
end"#, 0);
    assert!(!diags.has_errors(), "method_in_for_11: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"7
"#);
}

#[test]
fn method_in_for_12() {
    let (out, diags) = interpret(r#"C = struct
  n: Int = 0
end

C.inc = fn(self) do
  self.n = self.n + 1
  return self.n
end

fn main() do
  let c = C { n = 0 }
  for k in 4 do
    c:inc()
  end
  print(c.n)
end"#, 0);
    assert!(!diags.has_errors(), "method_in_for_12: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"4
"#);
}

#[test]
fn method_in_for_13() {
    let (out, diags) = interpret(r#"C = struct
  n: Int = 0
end

C.inc = fn(self) do
  self.n = self.n + 1
  return self.n
end

fn main() do
  let c = C { n = 0 }
  for k in 5 do
    c:inc()
  end
  print(c.n)
end"#, 0);
    assert!(!diags.has_errors(), "method_in_for_13: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"5
"#);
}

#[test]
fn method_in_for_14() {
    let (out, diags) = interpret(r#"C = struct
  n: Int = 0
end

C.inc = fn(self) do
  self.n = self.n + 1
  return self.n
end

fn main() do
  let c = C { n = 0 }
  for k in 6 do
    c:inc()
  end
  print(c.n)
end"#, 0);
    assert!(!diags.has_errors(), "method_in_for_14: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"6
"#);
}

#[test]
fn method_in_for_15() {
    let (out, diags) = interpret(r#"C = struct
  n: Int = 0
end

C.inc = fn(self) do
  self.n = self.n + 1
  return self.n
end

fn main() do
  let c = C { n = 0 }
  for k in 7 do
    c:inc()
  end
  print(c.n)
end"#, 0);
    assert!(!diags.has_errors(), "method_in_for_15: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"7
"#);
}

#[test]
fn method_codegen_impl_1() {
    let (rust, diags) = compile_to_rust(r#"P = struct
  x: Int = 0
  y: Int = 0
end

P.scale = fn(self, k) do
  self.x = self.x * k
  return self.x
end

P.getsum = fn(self) do
  return self.x + self.y
end

P.origin = fn(x0) do
  return P { x = x0, y = 0 }
end

fn main() do
  let p = P { x = 1, y = 2 }
  let a = p:scale(2)
  let b = p.getsum()
  let o = P:origin(7)
  print(a)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("impl P {"));
    assert_eq!(rust.matches("impl P {").count(), 1);
    assert!(rust.contains("fn scale(&mut self, k)"));
    assert!(rust.contains("fn getsum(&self)"));
    assert!(rust.contains("P::origin(7)"));
}

#[test]
fn method_codegen_impl_2() {
    let (rust, diags) = compile_to_rust(r#"P = struct
  x: Int = 0
  y: Int = 0
end

P.scale = fn(self, k) do
  self.x = self.x * k
  return self.x
end

P.getsum = fn(self) do
  return self.x + self.y
end

P.origin = fn(x0) do
  return P { x = x0, y = 0 }
end

fn main() do
  let p = P { x = 2, y = 2 }
  let a = p:scale(3)
  let b = p.getsum()
  let o = P:origin(7)
  print(a)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("impl P {"));
    assert_eq!(rust.matches("impl P {").count(), 1);
    assert!(rust.contains("fn scale(&mut self, k)"));
    assert!(rust.contains("fn getsum(&self)"));
    assert!(rust.contains("P::origin(7)"));
}

#[test]
fn method_codegen_impl_3() {
    let (rust, diags) = compile_to_rust(r#"P = struct
  x: Int = 0
  y: Int = 0
end

P.scale = fn(self, k) do
  self.x = self.x * k
  return self.x
end

P.getsum = fn(self) do
  return self.x + self.y
end

P.origin = fn(x0) do
  return P { x = x0, y = 0 }
end

fn main() do
  let p = P { x = 3, y = 2 }
  let a = p:scale(4)
  let b = p.getsum()
  let o = P:origin(7)
  print(a)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("impl P {"));
    assert_eq!(rust.matches("impl P {").count(), 1);
    assert!(rust.contains("fn scale(&mut self, k)"));
    assert!(rust.contains("fn getsum(&self)"));
    assert!(rust.contains("P::origin(7)"));
}

#[test]
fn method_codegen_impl_4() {
    let (rust, diags) = compile_to_rust(r#"P = struct
  x: Int = 0
  y: Int = 0
end

P.scale = fn(self, k) do
  self.x = self.x * k
  return self.x
end

P.getsum = fn(self) do
  return self.x + self.y
end

P.origin = fn(x0) do
  return P { x = x0, y = 0 }
end

fn main() do
  let p = P { x = 4, y = 2 }
  let a = p:scale(5)
  let b = p.getsum()
  let o = P:origin(7)
  print(a)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("impl P {"));
    assert_eq!(rust.matches("impl P {").count(), 1);
    assert!(rust.contains("fn scale(&mut self, k)"));
    assert!(rust.contains("fn getsum(&self)"));
    assert!(rust.contains("P::origin(7)"));
}

#[test]
fn method_codegen_impl_5() {
    let (rust, diags) = compile_to_rust(r#"P = struct
  x: Int = 0
  y: Int = 0
end

P.scale = fn(self, k) do
  self.x = self.x * k
  return self.x
end

P.getsum = fn(self) do
  return self.x + self.y
end

P.origin = fn(x0) do
  return P { x = x0, y = 0 }
end

fn main() do
  let p = P { x = 5, y = 2 }
  let a = p:scale(6)
  let b = p.getsum()
  let o = P:origin(7)
  print(a)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("impl P {"));
    assert_eq!(rust.matches("impl P {").count(), 1);
    assert!(rust.contains("fn scale(&mut self, k)"));
    assert!(rust.contains("fn getsum(&self)"));
    assert!(rust.contains("P::origin(7)"));
}

#[test]
fn method_codegen_impl_6() {
    let (rust, diags) = compile_to_rust(r#"P = struct
  x: Int = 0
  y: Int = 0
end

P.scale = fn(self, k) do
  self.x = self.x * k
  return self.x
end

P.getsum = fn(self) do
  return self.x + self.y
end

P.origin = fn(x0) do
  return P { x = x0, y = 0 }
end

fn main() do
  let p = P { x = 6, y = 2 }
  let a = p:scale(7)
  let b = p.getsum()
  let o = P:origin(7)
  print(a)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("impl P {"));
    assert_eq!(rust.matches("impl P {").count(), 1);
    assert!(rust.contains("fn scale(&mut self, k)"));
    assert!(rust.contains("fn getsum(&self)"));
    assert!(rust.contains("P::origin(7)"));
}

#[test]
fn method_codegen_impl_7() {
    let (rust, diags) = compile_to_rust(r#"P = struct
  x: Int = 0
  y: Int = 0
end

P.scale = fn(self, k) do
  self.x = self.x * k
  return self.x
end

P.getsum = fn(self) do
  return self.x + self.y
end

P.origin = fn(x0) do
  return P { x = x0, y = 0 }
end

fn main() do
  let p = P { x = 7, y = 2 }
  let a = p:scale(8)
  let b = p.getsum()
  let o = P:origin(7)
  print(a)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("impl P {"));
    assert_eq!(rust.matches("impl P {").count(), 1);
    assert!(rust.contains("fn scale(&mut self, k)"));
    assert!(rust.contains("fn getsum(&self)"));
    assert!(rust.contains("P::origin(7)"));
}

#[test]
fn method_codegen_impl_8() {
    let (rust, diags) = compile_to_rust(r#"P = struct
  x: Int = 0
  y: Int = 0
end

P.scale = fn(self, k) do
  self.x = self.x * k
  return self.x
end

P.getsum = fn(self) do
  return self.x + self.y
end

P.origin = fn(x0) do
  return P { x = x0, y = 0 }
end

fn main() do
  let p = P { x = 8, y = 2 }
  let a = p:scale(9)
  let b = p.getsum()
  let o = P:origin(7)
  print(a)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("impl P {"));
    assert_eq!(rust.matches("impl P {").count(), 1);
    assert!(rust.contains("fn scale(&mut self, k)"));
    assert!(rust.contains("fn getsum(&self)"));
    assert!(rust.contains("P::origin(7)"));
}

#[test]
fn method_codegen_impl_9() {
    let (rust, diags) = compile_to_rust(r#"P = struct
  x: Int = 0
  y: Int = 0
end

P.scale = fn(self, k) do
  self.x = self.x * k
  return self.x
end

P.getsum = fn(self) do
  return self.x + self.y
end

P.origin = fn(x0) do
  return P { x = x0, y = 0 }
end

fn main() do
  let p = P { x = 9, y = 2 }
  let a = p:scale(10)
  let b = p.getsum()
  let o = P:origin(7)
  print(a)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("impl P {"));
    assert_eq!(rust.matches("impl P {").count(), 1);
    assert!(rust.contains("fn scale(&mut self, k)"));
    assert!(rust.contains("fn getsum(&self)"));
    assert!(rust.contains("P::origin(7)"));
}

#[test]
fn method_codegen_impl_10() {
    let (rust, diags) = compile_to_rust(r#"P = struct
  x: Int = 0
  y: Int = 0
end

P.scale = fn(self, k) do
  self.x = self.x * k
  return self.x
end

P.getsum = fn(self) do
  return self.x + self.y
end

P.origin = fn(x0) do
  return P { x = x0, y = 0 }
end

fn main() do
  let p = P { x = 10, y = 2 }
  let a = p:scale(11)
  let b = p.getsum()
  let o = P:origin(7)
  print(a)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("impl P {"));
    assert_eq!(rust.matches("impl P {").count(), 1);
    assert!(rust.contains("fn scale(&mut self, k)"));
    assert!(rust.contains("fn getsum(&self)"));
    assert!(rust.contains("P::origin(7)"));
}

#[test]
fn method_codegen_impl_11() {
    let (rust, diags) = compile_to_rust(r#"P = struct
  x: Int = 0
  y: Int = 0
end

P.scale = fn(self, k) do
  self.x = self.x * k
  return self.x
end

P.getsum = fn(self) do
  return self.x + self.y
end

P.origin = fn(x0) do
  return P { x = x0, y = 0 }
end

fn main() do
  let p = P { x = 11, y = 2 }
  let a = p:scale(12)
  let b = p.getsum()
  let o = P:origin(7)
  print(a)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("impl P {"));
    assert_eq!(rust.matches("impl P {").count(), 1);
    assert!(rust.contains("fn scale(&mut self, k)"));
    assert!(rust.contains("fn getsum(&self)"));
    assert!(rust.contains("P::origin(7)"));
}

#[test]
fn method_codegen_impl_12() {
    let (rust, diags) = compile_to_rust(r#"P = struct
  x: Int = 0
  y: Int = 0
end

P.scale = fn(self, k) do
  self.x = self.x * k
  return self.x
end

P.getsum = fn(self) do
  return self.x + self.y
end

P.origin = fn(x0) do
  return P { x = x0, y = 0 }
end

fn main() do
  let p = P { x = 12, y = 2 }
  let a = p:scale(13)
  let b = p.getsum()
  let o = P:origin(7)
  print(a)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("impl P {"));
    assert_eq!(rust.matches("impl P {").count(), 1);
    assert!(rust.contains("fn scale(&mut self, k)"));
    assert!(rust.contains("fn getsum(&self)"));
    assert!(rust.contains("P::origin(7)"));
}

#[test]
fn method_codegen_impl_13() {
    let (rust, diags) = compile_to_rust(r#"P = struct
  x: Int = 0
  y: Int = 0
end

P.scale = fn(self, k) do
  self.x = self.x * k
  return self.x
end

P.getsum = fn(self) do
  return self.x + self.y
end

P.origin = fn(x0) do
  return P { x = x0, y = 0 }
end

fn main() do
  let p = P { x = 13, y = 2 }
  let a = p:scale(14)
  let b = p.getsum()
  let o = P:origin(7)
  print(a)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("impl P {"));
    assert_eq!(rust.matches("impl P {").count(), 1);
    assert!(rust.contains("fn scale(&mut self, k)"));
    assert!(rust.contains("fn getsum(&self)"));
    assert!(rust.contains("P::origin(7)"));
}

#[test]
fn method_codegen_impl_14() {
    let (rust, diags) = compile_to_rust(r#"P = struct
  x: Int = 0
  y: Int = 0
end

P.scale = fn(self, k) do
  self.x = self.x * k
  return self.x
end

P.getsum = fn(self) do
  return self.x + self.y
end

P.origin = fn(x0) do
  return P { x = x0, y = 0 }
end

fn main() do
  let p = P { x = 14, y = 2 }
  let a = p:scale(15)
  let b = p.getsum()
  let o = P:origin(7)
  print(a)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("impl P {"));
    assert_eq!(rust.matches("impl P {").count(), 1);
    assert!(rust.contains("fn scale(&mut self, k)"));
    assert!(rust.contains("fn getsum(&self)"));
    assert!(rust.contains("P::origin(7)"));
}

#[test]
fn method_codegen_impl_15() {
    let (rust, diags) = compile_to_rust(r#"P = struct
  x: Int = 0
  y: Int = 0
end

P.scale = fn(self, k) do
  self.x = self.x * k
  return self.x
end

P.getsum = fn(self) do
  return self.x + self.y
end

P.origin = fn(x0) do
  return P { x = x0, y = 0 }
end

fn main() do
  let p = P { x = 15, y = 2 }
  let a = p:scale(16)
  let b = p.getsum()
  let o = P:origin(7)
  print(a)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("impl P {"));
    assert_eq!(rust.matches("impl P {").count(), 1);
    assert!(rust.contains("fn scale(&mut self, k)"));
    assert!(rust.contains("fn getsum(&self)"));
    assert!(rust.contains("P::origin(7)"));
}

#[test]
fn method_codegen_impl_16() {
    let (rust, diags) = compile_to_rust(r#"P = struct
  x: Int = 0
  y: Int = 0
end

P.scale = fn(self, k) do
  self.x = self.x * k
  return self.x
end

P.getsum = fn(self) do
  return self.x + self.y
end

P.origin = fn(x0) do
  return P { x = x0, y = 0 }
end

fn main() do
  let p = P { x = 16, y = 2 }
  let a = p:scale(17)
  let b = p.getsum()
  let o = P:origin(7)
  print(a)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("impl P {"));
    assert_eq!(rust.matches("impl P {").count(), 1);
    assert!(rust.contains("fn scale(&mut self, k)"));
    assert!(rust.contains("fn getsum(&self)"));
    assert!(rust.contains("P::origin(7)"));
}

#[test]
fn method_codegen_impl_17() {
    let (rust, diags) = compile_to_rust(r#"P = struct
  x: Int = 0
  y: Int = 0
end

P.scale = fn(self, k) do
  self.x = self.x * k
  return self.x
end

P.getsum = fn(self) do
  return self.x + self.y
end

P.origin = fn(x0) do
  return P { x = x0, y = 0 }
end

fn main() do
  let p = P { x = 17, y = 2 }
  let a = p:scale(18)
  let b = p.getsum()
  let o = P:origin(7)
  print(a)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("impl P {"));
    assert_eq!(rust.matches("impl P {").count(), 1);
    assert!(rust.contains("fn scale(&mut self, k)"));
    assert!(rust.contains("fn getsum(&self)"));
    assert!(rust.contains("P::origin(7)"));
}

#[test]
fn method_codegen_impl_18() {
    let (rust, diags) = compile_to_rust(r#"P = struct
  x: Int = 0
  y: Int = 0
end

P.scale = fn(self, k) do
  self.x = self.x * k
  return self.x
end

P.getsum = fn(self) do
  return self.x + self.y
end

P.origin = fn(x0) do
  return P { x = x0, y = 0 }
end

fn main() do
  let p = P { x = 18, y = 2 }
  let a = p:scale(19)
  let b = p.getsum()
  let o = P:origin(7)
  print(a)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("impl P {"));
    assert_eq!(rust.matches("impl P {").count(), 1);
    assert!(rust.contains("fn scale(&mut self, k)"));
    assert!(rust.contains("fn getsum(&self)"));
    assert!(rust.contains("P::origin(7)"));
}

#[test]
fn method_codegen_impl_19() {
    let (rust, diags) = compile_to_rust(r#"P = struct
  x: Int = 0
  y: Int = 0
end

P.scale = fn(self, k) do
  self.x = self.x * k
  return self.x
end

P.getsum = fn(self) do
  return self.x + self.y
end

P.origin = fn(x0) do
  return P { x = x0, y = 0 }
end

fn main() do
  let p = P { x = 19, y = 2 }
  let a = p:scale(20)
  let b = p.getsum()
  let o = P:origin(7)
  print(a)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("impl P {"));
    assert_eq!(rust.matches("impl P {").count(), 1);
    assert!(rust.contains("fn scale(&mut self, k)"));
    assert!(rust.contains("fn getsum(&self)"));
    assert!(rust.contains("P::origin(7)"));
}

#[test]
fn method_codegen_impl_20() {
    let (rust, diags) = compile_to_rust(r#"P = struct
  x: Int = 0
  y: Int = 0
end

P.scale = fn(self, k) do
  self.x = self.x * k
  return self.x
end

P.getsum = fn(self) do
  return self.x + self.y
end

P.origin = fn(x0) do
  return P { x = x0, y = 0 }
end

fn main() do
  let p = P { x = 20, y = 2 }
  let a = p:scale(21)
  let b = p.getsum()
  let o = P:origin(7)
  print(a)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("impl P {"));
    assert_eq!(rust.matches("impl P {").count(), 1);
    assert!(rust.contains("fn scale(&mut self, k)"));
    assert!(rust.contains("fn getsum(&self)"));
    assert!(rust.contains("P::origin(7)"));
}

#[test]
fn method_codegen_grouped_1() {
    let (rust, diags) = compile_to_rust(r#"T1 = struct
  v: Int = 0
end

T1.a = fn(self) do
  return self.v
end

T1.b = fn(self) do
  self.v = self.v + 1
  return self.v
end

fn main() do
  let t = T1 { v = 1 }
  print(t:b())
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("impl T1 {"));
    assert_eq!(rust.matches("impl T1 {").count(), 1);
    assert!(rust.contains("fn a(&self)"));
    assert!(rust.contains("fn b(&mut self)"));
}

#[test]
fn method_codegen_grouped_2() {
    let (rust, diags) = compile_to_rust(r#"T2 = struct
  v: Int = 0
end

T2.a = fn(self) do
  return self.v
end

T2.b = fn(self) do
  self.v = self.v + 1
  return self.v
end

fn main() do
  let t = T2 { v = 1 }
  print(t:b())
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("impl T2 {"));
    assert_eq!(rust.matches("impl T2 {").count(), 1);
    assert!(rust.contains("fn a(&self)"));
    assert!(rust.contains("fn b(&mut self)"));
}

#[test]
fn method_codegen_grouped_3() {
    let (rust, diags) = compile_to_rust(r#"T3 = struct
  v: Int = 0
end

T3.a = fn(self) do
  return self.v
end

T3.b = fn(self) do
  self.v = self.v + 1
  return self.v
end

fn main() do
  let t = T3 { v = 1 }
  print(t:b())
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("impl T3 {"));
    assert_eq!(rust.matches("impl T3 {").count(), 1);
    assert!(rust.contains("fn a(&self)"));
    assert!(rust.contains("fn b(&mut self)"));
}

#[test]
fn method_codegen_grouped_4() {
    let (rust, diags) = compile_to_rust(r#"T4 = struct
  v: Int = 0
end

T4.a = fn(self) do
  return self.v
end

T4.b = fn(self) do
  self.v = self.v + 1
  return self.v
end

fn main() do
  let t = T4 { v = 1 }
  print(t:b())
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("impl T4 {"));
    assert_eq!(rust.matches("impl T4 {").count(), 1);
    assert!(rust.contains("fn a(&self)"));
    assert!(rust.contains("fn b(&mut self)"));
}

#[test]
fn method_codegen_grouped_5() {
    let (rust, diags) = compile_to_rust(r#"T5 = struct
  v: Int = 0
end

T5.a = fn(self) do
  return self.v
end

T5.b = fn(self) do
  self.v = self.v + 1
  return self.v
end

fn main() do
  let t = T5 { v = 1 }
  print(t:b())
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("impl T5 {"));
    assert_eq!(rust.matches("impl T5 {").count(), 1);
    assert!(rust.contains("fn a(&self)"));
    assert!(rust.contains("fn b(&mut self)"));
}

#[test]
fn method_codegen_grouped_6() {
    let (rust, diags) = compile_to_rust(r#"T6 = struct
  v: Int = 0
end

T6.a = fn(self) do
  return self.v
end

T6.b = fn(self) do
  self.v = self.v + 1
  return self.v
end

fn main() do
  let t = T6 { v = 1 }
  print(t:b())
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("impl T6 {"));
    assert_eq!(rust.matches("impl T6 {").count(), 1);
    assert!(rust.contains("fn a(&self)"));
    assert!(rust.contains("fn b(&mut self)"));
}

#[test]
fn method_codegen_grouped_7() {
    let (rust, diags) = compile_to_rust(r#"T7 = struct
  v: Int = 0
end

T7.a = fn(self) do
  return self.v
end

T7.b = fn(self) do
  self.v = self.v + 1
  return self.v
end

fn main() do
  let t = T7 { v = 1 }
  print(t:b())
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("impl T7 {"));
    assert_eq!(rust.matches("impl T7 {").count(), 1);
    assert!(rust.contains("fn a(&self)"));
    assert!(rust.contains("fn b(&mut self)"));
}

#[test]
fn method_codegen_grouped_8() {
    let (rust, diags) = compile_to_rust(r#"T8 = struct
  v: Int = 0
end

T8.a = fn(self) do
  return self.v
end

T8.b = fn(self) do
  self.v = self.v + 1
  return self.v
end

fn main() do
  let t = T8 { v = 1 }
  print(t:b())
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("impl T8 {"));
    assert_eq!(rust.matches("impl T8 {").count(), 1);
    assert!(rust.contains("fn a(&self)"));
    assert!(rust.contains("fn b(&mut self)"));
}

#[test]
fn method_codegen_grouped_9() {
    let (rust, diags) = compile_to_rust(r#"T9 = struct
  v: Int = 0
end

T9.a = fn(self) do
  return self.v
end

T9.b = fn(self) do
  self.v = self.v + 1
  return self.v
end

fn main() do
  let t = T9 { v = 1 }
  print(t:b())
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("impl T9 {"));
    assert_eq!(rust.matches("impl T9 {").count(), 1);
    assert!(rust.contains("fn a(&self)"));
    assert!(rust.contains("fn b(&mut self)"));
}

#[test]
fn method_codegen_grouped_10() {
    let (rust, diags) = compile_to_rust(r#"T10 = struct
  v: Int = 0
end

T10.a = fn(self) do
  return self.v
end

T10.b = fn(self) do
  self.v = self.v + 1
  return self.v
end

fn main() do
  let t = T10 { v = 1 }
  print(t:b())
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("impl T10 {"));
    assert_eq!(rust.matches("impl T10 {").count(), 1);
    assert!(rust.contains("fn a(&self)"));
    assert!(rust.contains("fn b(&mut self)"));
}
