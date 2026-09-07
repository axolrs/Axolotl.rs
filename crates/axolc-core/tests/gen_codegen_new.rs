// Owner: PascalElixir / axolrs (GitHub org)
// File: Auto-generated tests for new codegen shapes: impl grouping, &mut self, Type::static() calls, println!, for-range, loop+break from repeat, .to_string() for String fields, let mut inference, pipe calls, struct literals, and FFI extraction for cblocks.

use axolc_core::compile_to_rust;
use axolc_core::ffi;

#[test]
fn cg_mut_self_1() {
    let (rust, diags) = compile_to_rust(r#"W = struct
  v: Int = 0
end

W.grow = fn(self, k) do
  self.v = self.v * k
  return self.v
end

fn main() do
  let w = W { v = 1 }
  print(w:grow(2))
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("impl W {"));
    assert!(rust.contains("fn grow(&mut self, k)"));
}

#[test]
fn cg_mut_self_2() {
    let (rust, diags) = compile_to_rust(r#"W = struct
  v: Int = 0
end

W.grow = fn(self, k) do
  self.v = self.v * k
  return self.v
end

fn main() do
  let w = W { v = 2 }
  print(w:grow(3))
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("impl W {"));
    assert!(rust.contains("fn grow(&mut self, k)"));
}

#[test]
fn cg_mut_self_3() {
    let (rust, diags) = compile_to_rust(r#"W = struct
  v: Int = 0
end

W.grow = fn(self, k) do
  self.v = self.v * k
  return self.v
end

fn main() do
  let w = W { v = 3 }
  print(w:grow(4))
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("impl W {"));
    assert!(rust.contains("fn grow(&mut self, k)"));
}

#[test]
fn cg_mut_self_4() {
    let (rust, diags) = compile_to_rust(r#"W = struct
  v: Int = 0
end

W.grow = fn(self, k) do
  self.v = self.v * k
  return self.v
end

fn main() do
  let w = W { v = 4 }
  print(w:grow(5))
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("impl W {"));
    assert!(rust.contains("fn grow(&mut self, k)"));
}

#[test]
fn cg_mut_self_5() {
    let (rust, diags) = compile_to_rust(r#"W = struct
  v: Int = 0
end

W.grow = fn(self, k) do
  self.v = self.v * k
  return self.v
end

fn main() do
  let w = W { v = 5 }
  print(w:grow(6))
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("impl W {"));
    assert!(rust.contains("fn grow(&mut self, k)"));
}

#[test]
fn cg_mut_self_6() {
    let (rust, diags) = compile_to_rust(r#"W = struct
  v: Int = 0
end

W.grow = fn(self, k) do
  self.v = self.v * k
  return self.v
end

fn main() do
  let w = W { v = 6 }
  print(w:grow(7))
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("impl W {"));
    assert!(rust.contains("fn grow(&mut self, k)"));
}

#[test]
fn cg_mut_self_7() {
    let (rust, diags) = compile_to_rust(r#"W = struct
  v: Int = 0
end

W.grow = fn(self, k) do
  self.v = self.v * k
  return self.v
end

fn main() do
  let w = W { v = 7 }
  print(w:grow(8))
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("impl W {"));
    assert!(rust.contains("fn grow(&mut self, k)"));
}

#[test]
fn cg_mut_self_8() {
    let (rust, diags) = compile_to_rust(r#"W = struct
  v: Int = 0
end

W.grow = fn(self, k) do
  self.v = self.v * k
  return self.v
end

fn main() do
  let w = W { v = 8 }
  print(w:grow(9))
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("impl W {"));
    assert!(rust.contains("fn grow(&mut self, k)"));
}

#[test]
fn cg_mut_self_9() {
    let (rust, diags) = compile_to_rust(r#"W = struct
  v: Int = 0
end

W.grow = fn(self, k) do
  self.v = self.v * k
  return self.v
end

fn main() do
  let w = W { v = 9 }
  print(w:grow(10))
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("impl W {"));
    assert!(rust.contains("fn grow(&mut self, k)"));
}

#[test]
fn cg_mut_self_10() {
    let (rust, diags) = compile_to_rust(r#"W = struct
  v: Int = 0
end

W.grow = fn(self, k) do
  self.v = self.v * k
  return self.v
end

fn main() do
  let w = W { v = 10 }
  print(w:grow(11))
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("impl W {"));
    assert!(rust.contains("fn grow(&mut self, k)"));
}

#[test]
fn cg_mut_self_11() {
    let (rust, diags) = compile_to_rust(r#"W = struct
  v: Int = 0
end

W.grow = fn(self, k) do
  self.v = self.v * k
  return self.v
end

fn main() do
  let w = W { v = 11 }
  print(w:grow(12))
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("impl W {"));
    assert!(rust.contains("fn grow(&mut self, k)"));
}

#[test]
fn cg_mut_self_12() {
    let (rust, diags) = compile_to_rust(r#"W = struct
  v: Int = 0
end

W.grow = fn(self, k) do
  self.v = self.v * k
  return self.v
end

fn main() do
  let w = W { v = 12 }
  print(w:grow(13))
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("impl W {"));
    assert!(rust.contains("fn grow(&mut self, k)"));
}

#[test]
fn cg_mut_self_13() {
    let (rust, diags) = compile_to_rust(r#"W = struct
  v: Int = 0
end

W.grow = fn(self, k) do
  self.v = self.v * k
  return self.v
end

fn main() do
  let w = W { v = 13 }
  print(w:grow(14))
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("impl W {"));
    assert!(rust.contains("fn grow(&mut self, k)"));
}

#[test]
fn cg_mut_self_14() {
    let (rust, diags) = compile_to_rust(r#"W = struct
  v: Int = 0
end

W.grow = fn(self, k) do
  self.v = self.v * k
  return self.v
end

fn main() do
  let w = W { v = 14 }
  print(w:grow(15))
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("impl W {"));
    assert!(rust.contains("fn grow(&mut self, k)"));
}

#[test]
fn cg_mut_self_15() {
    let (rust, diags) = compile_to_rust(r#"W = struct
  v: Int = 0
end

W.grow = fn(self, k) do
  self.v = self.v * k
  return self.v
end

fn main() do
  let w = W { v = 15 }
  print(w:grow(16))
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("impl W {"));
    assert!(rust.contains("fn grow(&mut self, k)"));
}

#[test]
fn cg_ref_self_1() {
    let (rust, diags) = compile_to_rust(r#"R = struct
  v: Int = 0
end

R.get = fn(self) do
  return self.v
end

fn main() do
  let r = R { v = 1 }
  print(r:get())
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("fn get(&self)"));
    assert!(!rust.contains("fn get(&mut self)"));
}

#[test]
fn cg_ref_self_2() {
    let (rust, diags) = compile_to_rust(r#"R = struct
  v: Int = 0
end

R.get = fn(self) do
  return self.v
end

fn main() do
  let r = R { v = 2 }
  print(r:get())
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("fn get(&self)"));
    assert!(!rust.contains("fn get(&mut self)"));
}

#[test]
fn cg_ref_self_3() {
    let (rust, diags) = compile_to_rust(r#"R = struct
  v: Int = 0
end

R.get = fn(self) do
  return self.v
end

fn main() do
  let r = R { v = 3 }
  print(r:get())
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("fn get(&self)"));
    assert!(!rust.contains("fn get(&mut self)"));
}

#[test]
fn cg_ref_self_4() {
    let (rust, diags) = compile_to_rust(r#"R = struct
  v: Int = 0
end

R.get = fn(self) do
  return self.v
end

fn main() do
  let r = R { v = 4 }
  print(r:get())
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("fn get(&self)"));
    assert!(!rust.contains("fn get(&mut self)"));
}

#[test]
fn cg_ref_self_5() {
    let (rust, diags) = compile_to_rust(r#"R = struct
  v: Int = 0
end

R.get = fn(self) do
  return self.v
end

fn main() do
  let r = R { v = 5 }
  print(r:get())
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("fn get(&self)"));
    assert!(!rust.contains("fn get(&mut self)"));
}

#[test]
fn cg_ref_self_6() {
    let (rust, diags) = compile_to_rust(r#"R = struct
  v: Int = 0
end

R.get = fn(self) do
  return self.v
end

fn main() do
  let r = R { v = 6 }
  print(r:get())
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("fn get(&self)"));
    assert!(!rust.contains("fn get(&mut self)"));
}

#[test]
fn cg_ref_self_7() {
    let (rust, diags) = compile_to_rust(r#"R = struct
  v: Int = 0
end

R.get = fn(self) do
  return self.v
end

fn main() do
  let r = R { v = 7 }
  print(r:get())
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("fn get(&self)"));
    assert!(!rust.contains("fn get(&mut self)"));
}

#[test]
fn cg_ref_self_8() {
    let (rust, diags) = compile_to_rust(r#"R = struct
  v: Int = 0
end

R.get = fn(self) do
  return self.v
end

fn main() do
  let r = R { v = 8 }
  print(r:get())
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("fn get(&self)"));
    assert!(!rust.contains("fn get(&mut self)"));
}

#[test]
fn cg_ref_self_9() {
    let (rust, diags) = compile_to_rust(r#"R = struct
  v: Int = 0
end

R.get = fn(self) do
  return self.v
end

fn main() do
  let r = R { v = 9 }
  print(r:get())
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("fn get(&self)"));
    assert!(!rust.contains("fn get(&mut self)"));
}

#[test]
fn cg_ref_self_10() {
    let (rust, diags) = compile_to_rust(r#"R = struct
  v: Int = 0
end

R.get = fn(self) do
  return self.v
end

fn main() do
  let r = R { v = 10 }
  print(r:get())
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("fn get(&self)"));
    assert!(!rust.contains("fn get(&mut self)"));
}

#[test]
fn cg_ref_self_11() {
    let (rust, diags) = compile_to_rust(r#"R = struct
  v: Int = 0
end

R.get = fn(self) do
  return self.v
end

fn main() do
  let r = R { v = 11 }
  print(r:get())
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("fn get(&self)"));
    assert!(!rust.contains("fn get(&mut self)"));
}

#[test]
fn cg_ref_self_12() {
    let (rust, diags) = compile_to_rust(r#"R = struct
  v: Int = 0
end

R.get = fn(self) do
  return self.v
end

fn main() do
  let r = R { v = 12 }
  print(r:get())
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("fn get(&self)"));
    assert!(!rust.contains("fn get(&mut self)"));
}

#[test]
fn cg_ref_self_13() {
    let (rust, diags) = compile_to_rust(r#"R = struct
  v: Int = 0
end

R.get = fn(self) do
  return self.v
end

fn main() do
  let r = R { v = 13 }
  print(r:get())
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("fn get(&self)"));
    assert!(!rust.contains("fn get(&mut self)"));
}

#[test]
fn cg_ref_self_14() {
    let (rust, diags) = compile_to_rust(r#"R = struct
  v: Int = 0
end

R.get = fn(self) do
  return self.v
end

fn main() do
  let r = R { v = 14 }
  print(r:get())
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("fn get(&self)"));
    assert!(!rust.contains("fn get(&mut self)"));
}

#[test]
fn cg_ref_self_15() {
    let (rust, diags) = compile_to_rust(r#"R = struct
  v: Int = 0
end

R.get = fn(self) do
  return self.v
end

fn main() do
  let r = R { v = 15 }
  print(r:get())
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("fn get(&self)"));
    assert!(!rust.contains("fn get(&mut self)"));
}

#[test]
fn cg_static_call_1() {
    let (rust, diags) = compile_to_rust(r#"S = struct
  v: Int = 0
end

S.make = fn(v) do
  return S { v = v }
end

fn main() do
  let s = S:make(3)
  print(s.v)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("S::make(3)"));
    assert!(!rust.contains("let s = S.make"));
}

#[test]
fn cg_static_call_2() {
    let (rust, diags) = compile_to_rust(r#"S = struct
  v: Int = 0
end

S.make = fn(v) do
  return S { v = v }
end

fn main() do
  let s = S:make(6)
  print(s.v)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("S::make(6)"));
    assert!(!rust.contains("let s = S.make"));
}

#[test]
fn cg_static_call_3() {
    let (rust, diags) = compile_to_rust(r#"S = struct
  v: Int = 0
end

S.make = fn(v) do
  return S { v = v }
end

fn main() do
  let s = S:make(9)
  print(s.v)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("S::make(9)"));
    assert!(!rust.contains("let s = S.make"));
}

#[test]
fn cg_static_call_4() {
    let (rust, diags) = compile_to_rust(r#"S = struct
  v: Int = 0
end

S.make = fn(v) do
  return S { v = v }
end

fn main() do
  let s = S:make(12)
  print(s.v)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("S::make(12)"));
    assert!(!rust.contains("let s = S.make"));
}

#[test]
fn cg_static_call_5() {
    let (rust, diags) = compile_to_rust(r#"S = struct
  v: Int = 0
end

S.make = fn(v) do
  return S { v = v }
end

fn main() do
  let s = S:make(15)
  print(s.v)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("S::make(15)"));
    assert!(!rust.contains("let s = S.make"));
}

#[test]
fn cg_static_call_6() {
    let (rust, diags) = compile_to_rust(r#"S = struct
  v: Int = 0
end

S.make = fn(v) do
  return S { v = v }
end

fn main() do
  let s = S:make(18)
  print(s.v)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("S::make(18)"));
    assert!(!rust.contains("let s = S.make"));
}

#[test]
fn cg_static_call_7() {
    let (rust, diags) = compile_to_rust(r#"S = struct
  v: Int = 0
end

S.make = fn(v) do
  return S { v = v }
end

fn main() do
  let s = S:make(21)
  print(s.v)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("S::make(21)"));
    assert!(!rust.contains("let s = S.make"));
}

#[test]
fn cg_static_call_8() {
    let (rust, diags) = compile_to_rust(r#"S = struct
  v: Int = 0
end

S.make = fn(v) do
  return S { v = v }
end

fn main() do
  let s = S:make(24)
  print(s.v)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("S::make(24)"));
    assert!(!rust.contains("let s = S.make"));
}

#[test]
fn cg_static_call_9() {
    let (rust, diags) = compile_to_rust(r#"S = struct
  v: Int = 0
end

S.make = fn(v) do
  return S { v = v }
end

fn main() do
  let s = S:make(27)
  print(s.v)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("S::make(27)"));
    assert!(!rust.contains("let s = S.make"));
}

#[test]
fn cg_static_call_10() {
    let (rust, diags) = compile_to_rust(r#"S = struct
  v: Int = 0
end

S.make = fn(v) do
  return S { v = v }
end

fn main() do
  let s = S:make(30)
  print(s.v)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("S::make(30)"));
    assert!(!rust.contains("let s = S.make"));
}

#[test]
fn cg_static_call_11() {
    let (rust, diags) = compile_to_rust(r#"S = struct
  v: Int = 0
end

S.make = fn(v) do
  return S { v = v }
end

fn main() do
  let s = S:make(33)
  print(s.v)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("S::make(33)"));
    assert!(!rust.contains("let s = S.make"));
}

#[test]
fn cg_static_call_12() {
    let (rust, diags) = compile_to_rust(r#"S = struct
  v: Int = 0
end

S.make = fn(v) do
  return S { v = v }
end

fn main() do
  let s = S:make(36)
  print(s.v)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("S::make(36)"));
    assert!(!rust.contains("let s = S.make"));
}

#[test]
fn cg_static_call_13() {
    let (rust, diags) = compile_to_rust(r#"S = struct
  v: Int = 0
end

S.make = fn(v) do
  return S { v = v }
end

fn main() do
  let s = S:make(39)
  print(s.v)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("S::make(39)"));
    assert!(!rust.contains("let s = S.make"));
}

#[test]
fn cg_static_call_14() {
    let (rust, diags) = compile_to_rust(r#"S = struct
  v: Int = 0
end

S.make = fn(v) do
  return S { v = v }
end

fn main() do
  let s = S:make(42)
  print(s.v)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("S::make(42)"));
    assert!(!rust.contains("let s = S.make"));
}

#[test]
fn cg_static_call_15() {
    let (rust, diags) = compile_to_rust(r#"S = struct
  v: Int = 0
end

S.make = fn(v) do
  return S { v = v }
end

fn main() do
  let s = S:make(45)
  print(s.v)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("S::make(45)"));
    assert!(!rust.contains("let s = S.make"));
}

#[test]
fn cg_println_1() {
    let (rust, diags) = compile_to_rust(r#"fn main() do
  print(4)
  print("x", 1)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("println!(\"{}\", 4)"));
    assert!(rust.contains("println!(\"{} {}\", \"x\", 1)"));
    assert!(!rust.contains("print("));
}

#[test]
fn cg_println_2() {
    let (rust, diags) = compile_to_rust(r#"fn main() do
  print(8)
  print("x", 2)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("println!(\"{}\", 8)"));
    assert!(rust.contains("println!(\"{} {}\", \"x\", 2)"));
    assert!(!rust.contains("print("));
}

#[test]
fn cg_println_3() {
    let (rust, diags) = compile_to_rust(r#"fn main() do
  print(12)
  print("x", 3)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("println!(\"{}\", 12)"));
    assert!(rust.contains("println!(\"{} {}\", \"x\", 3)"));
    assert!(!rust.contains("print("));
}

#[test]
fn cg_println_4() {
    let (rust, diags) = compile_to_rust(r#"fn main() do
  print(16)
  print("x", 4)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("println!(\"{}\", 16)"));
    assert!(rust.contains("println!(\"{} {}\", \"x\", 4)"));
    assert!(!rust.contains("print("));
}

#[test]
fn cg_println_5() {
    let (rust, diags) = compile_to_rust(r#"fn main() do
  print(20)
  print("x", 5)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("println!(\"{}\", 20)"));
    assert!(rust.contains("println!(\"{} {}\", \"x\", 5)"));
    assert!(!rust.contains("print("));
}

#[test]
fn cg_println_6() {
    let (rust, diags) = compile_to_rust(r#"fn main() do
  print(24)
  print("x", 6)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("println!(\"{}\", 24)"));
    assert!(rust.contains("println!(\"{} {}\", \"x\", 6)"));
    assert!(!rust.contains("print("));
}

#[test]
fn cg_println_7() {
    let (rust, diags) = compile_to_rust(r#"fn main() do
  print(28)
  print("x", 7)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("println!(\"{}\", 28)"));
    assert!(rust.contains("println!(\"{} {}\", \"x\", 7)"));
    assert!(!rust.contains("print("));
}

#[test]
fn cg_println_8() {
    let (rust, diags) = compile_to_rust(r#"fn main() do
  print(32)
  print("x", 8)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("println!(\"{}\", 32)"));
    assert!(rust.contains("println!(\"{} {}\", \"x\", 8)"));
    assert!(!rust.contains("print("));
}

#[test]
fn cg_println_9() {
    let (rust, diags) = compile_to_rust(r#"fn main() do
  print(36)
  print("x", 9)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("println!(\"{}\", 36)"));
    assert!(rust.contains("println!(\"{} {}\", \"x\", 9)"));
    assert!(!rust.contains("print("));
}

#[test]
fn cg_println_10() {
    let (rust, diags) = compile_to_rust(r#"fn main() do
  print(40)
  print("x", 10)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("println!(\"{}\", 40)"));
    assert!(rust.contains("println!(\"{} {}\", \"x\", 10)"));
    assert!(!rust.contains("print("));
}

#[test]
fn cg_println_11() {
    let (rust, diags) = compile_to_rust(r#"fn main() do
  print(44)
  print("x", 11)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("println!(\"{}\", 44)"));
    assert!(rust.contains("println!(\"{} {}\", \"x\", 11)"));
    assert!(!rust.contains("print("));
}

#[test]
fn cg_println_12() {
    let (rust, diags) = compile_to_rust(r#"fn main() do
  print(48)
  print("x", 12)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("println!(\"{}\", 48)"));
    assert!(rust.contains("println!(\"{} {}\", \"x\", 12)"));
    assert!(!rust.contains("print("));
}

#[test]
fn cg_println_13() {
    let (rust, diags) = compile_to_rust(r#"fn main() do
  print(52)
  print("x", 13)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("println!(\"{}\", 52)"));
    assert!(rust.contains("println!(\"{} {}\", \"x\", 13)"));
    assert!(!rust.contains("print("));
}

#[test]
fn cg_println_14() {
    let (rust, diags) = compile_to_rust(r#"fn main() do
  print(56)
  print("x", 14)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("println!(\"{}\", 56)"));
    assert!(rust.contains("println!(\"{} {}\", \"x\", 14)"));
    assert!(!rust.contains("print("));
}

#[test]
fn cg_println_15() {
    let (rust, diags) = compile_to_rust(r#"fn main() do
  print(60)
  print("x", 15)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("println!(\"{}\", 60)"));
    assert!(rust.contains("println!(\"{} {}\", \"x\", 15)"));
    assert!(!rust.contains("print("));
}

#[test]
fn cg_for_range_1() {
    let (rust, diags) = compile_to_rust(r#"fn main() do
  var s = 0
  for i in 11 do
    s = s + i
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("for i in 0..11 {"));
}

#[test]
fn cg_for_range_2() {
    let (rust, diags) = compile_to_rust(r#"fn main() do
  var s = 0
  for i in 12 do
    s = s + i
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("for i in 0..12 {"));
}

#[test]
fn cg_for_range_3() {
    let (rust, diags) = compile_to_rust(r#"fn main() do
  var s = 0
  for i in 13 do
    s = s + i
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("for i in 0..13 {"));
}

#[test]
fn cg_for_range_4() {
    let (rust, diags) = compile_to_rust(r#"fn main() do
  var s = 0
  for i in 14 do
    s = s + i
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("for i in 0..14 {"));
}

#[test]
fn cg_for_range_5() {
    let (rust, diags) = compile_to_rust(r#"fn main() do
  var s = 0
  for i in 15 do
    s = s + i
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("for i in 0..15 {"));
}

#[test]
fn cg_for_range_6() {
    let (rust, diags) = compile_to_rust(r#"fn main() do
  var s = 0
  for i in 16 do
    s = s + i
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("for i in 0..16 {"));
}

#[test]
fn cg_for_range_7() {
    let (rust, diags) = compile_to_rust(r#"fn main() do
  var s = 0
  for i in 17 do
    s = s + i
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("for i in 0..17 {"));
}

#[test]
fn cg_for_range_8() {
    let (rust, diags) = compile_to_rust(r#"fn main() do
  var s = 0
  for i in 18 do
    s = s + i
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("for i in 0..18 {"));
}

#[test]
fn cg_for_range_9() {
    let (rust, diags) = compile_to_rust(r#"fn main() do
  var s = 0
  for i in 19 do
    s = s + i
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("for i in 0..19 {"));
}

#[test]
fn cg_for_range_10() {
    let (rust, diags) = compile_to_rust(r#"fn main() do
  var s = 0
  for i in 20 do
    s = s + i
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("for i in 0..20 {"));
}

#[test]
fn cg_for_range_11() {
    let (rust, diags) = compile_to_rust(r#"fn main() do
  var s = 0
  for i in 21 do
    s = s + i
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("for i in 0..21 {"));
}

#[test]
fn cg_for_range_12() {
    let (rust, diags) = compile_to_rust(r#"fn main() do
  var s = 0
  for i in 22 do
    s = s + i
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("for i in 0..22 {"));
}

#[test]
fn cg_for_range_13() {
    let (rust, diags) = compile_to_rust(r#"fn main() do
  var s = 0
  for i in 23 do
    s = s + i
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("for i in 0..23 {"));
}

#[test]
fn cg_for_range_14() {
    let (rust, diags) = compile_to_rust(r#"fn main() do
  var s = 0
  for i in 24 do
    s = s + i
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("for i in 0..24 {"));
}

#[test]
fn cg_for_range_15() {
    let (rust, diags) = compile_to_rust(r#"fn main() do
  var s = 0
  for i in 25 do
    s = s + i
  end
  print(s)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("for i in 0..25 {"));
}

#[test]
fn cg_repeat_loop_1() {
    let (rust, diags) = compile_to_rust(r#"fn main() do
  var c = 0
  repeat
    c = c + 1
  until c >= 3
  print(c)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("loop {"));
    assert!(rust.contains("break;"));
    assert!(rust.contains("if c >= 3 {"));
}

#[test]
fn cg_repeat_loop_2() {
    let (rust, diags) = compile_to_rust(r#"fn main() do
  var c = 0
  repeat
    c = c + 1
  until c >= 4
  print(c)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("loop {"));
    assert!(rust.contains("break;"));
    assert!(rust.contains("if c >= 4 {"));
}

#[test]
fn cg_repeat_loop_3() {
    let (rust, diags) = compile_to_rust(r#"fn main() do
  var c = 0
  repeat
    c = c + 1
  until c >= 5
  print(c)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("loop {"));
    assert!(rust.contains("break;"));
    assert!(rust.contains("if c >= 5 {"));
}

#[test]
fn cg_repeat_loop_4() {
    let (rust, diags) = compile_to_rust(r#"fn main() do
  var c = 0
  repeat
    c = c + 1
  until c >= 6
  print(c)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("loop {"));
    assert!(rust.contains("break;"));
    assert!(rust.contains("if c >= 6 {"));
}

#[test]
fn cg_repeat_loop_5() {
    let (rust, diags) = compile_to_rust(r#"fn main() do
  var c = 0
  repeat
    c = c + 1
  until c >= 7
  print(c)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("loop {"));
    assert!(rust.contains("break;"));
    assert!(rust.contains("if c >= 7 {"));
}

#[test]
fn cg_repeat_loop_6() {
    let (rust, diags) = compile_to_rust(r#"fn main() do
  var c = 0
  repeat
    c = c + 1
  until c >= 8
  print(c)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("loop {"));
    assert!(rust.contains("break;"));
    assert!(rust.contains("if c >= 8 {"));
}

#[test]
fn cg_repeat_loop_7() {
    let (rust, diags) = compile_to_rust(r#"fn main() do
  var c = 0
  repeat
    c = c + 1
  until c >= 9
  print(c)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("loop {"));
    assert!(rust.contains("break;"));
    assert!(rust.contains("if c >= 9 {"));
}

#[test]
fn cg_repeat_loop_8() {
    let (rust, diags) = compile_to_rust(r#"fn main() do
  var c = 0
  repeat
    c = c + 1
  until c >= 10
  print(c)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("loop {"));
    assert!(rust.contains("break;"));
    assert!(rust.contains("if c >= 10 {"));
}

#[test]
fn cg_repeat_loop_9() {
    let (rust, diags) = compile_to_rust(r#"fn main() do
  var c = 0
  repeat
    c = c + 1
  until c >= 11
  print(c)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("loop {"));
    assert!(rust.contains("break;"));
    assert!(rust.contains("if c >= 11 {"));
}

#[test]
fn cg_repeat_loop_10() {
    let (rust, diags) = compile_to_rust(r#"fn main() do
  var c = 0
  repeat
    c = c + 1
  until c >= 12
  print(c)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("loop {"));
    assert!(rust.contains("break;"));
    assert!(rust.contains("if c >= 12 {"));
}

#[test]
fn cg_repeat_loop_11() {
    let (rust, diags) = compile_to_rust(r#"fn main() do
  var c = 0
  repeat
    c = c + 1
  until c >= 13
  print(c)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("loop {"));
    assert!(rust.contains("break;"));
    assert!(rust.contains("if c >= 13 {"));
}

#[test]
fn cg_repeat_loop_12() {
    let (rust, diags) = compile_to_rust(r#"fn main() do
  var c = 0
  repeat
    c = c + 1
  until c >= 14
  print(c)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("loop {"));
    assert!(rust.contains("break;"));
    assert!(rust.contains("if c >= 14 {"));
}

#[test]
fn cg_repeat_loop_13() {
    let (rust, diags) = compile_to_rust(r#"fn main() do
  var c = 0
  repeat
    c = c + 1
  until c >= 15
  print(c)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("loop {"));
    assert!(rust.contains("break;"));
    assert!(rust.contains("if c >= 15 {"));
}

#[test]
fn cg_repeat_loop_14() {
    let (rust, diags) = compile_to_rust(r#"fn main() do
  var c = 0
  repeat
    c = c + 1
  until c >= 16
  print(c)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("loop {"));
    assert!(rust.contains("break;"));
    assert!(rust.contains("if c >= 16 {"));
}

#[test]
fn cg_repeat_loop_15() {
    let (rust, diags) = compile_to_rust(r#"fn main() do
  var c = 0
  repeat
    c = c + 1
  until c >= 17
  print(c)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("loop {"));
    assert!(rust.contains("break;"));
    assert!(rust.contains("if c >= 17 {"));
}

#[test]
fn cg_to_string_1() {
    let (rust, diags) = compile_to_rust(r#"N = struct
  id: Int = 0
  label: String = "x"
end

fn main() do
  let n = N { id = 8, label = "tag1" }
  print(n.id)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("label: \"tag1\".to_string()"));
    assert!(rust.contains("id: 8,"));
}

#[test]
fn cg_to_string_2() {
    let (rust, diags) = compile_to_rust(r#"N = struct
  id: Int = 0
  label: String = "x"
end

fn main() do
  let n = N { id = 16, label = "tag2" }
  print(n.id)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("label: \"tag2\".to_string()"));
    assert!(rust.contains("id: 16,"));
}

#[test]
fn cg_to_string_3() {
    let (rust, diags) = compile_to_rust(r#"N = struct
  id: Int = 0
  label: String = "x"
end

fn main() do
  let n = N { id = 24, label = "tag3" }
  print(n.id)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("label: \"tag3\".to_string()"));
    assert!(rust.contains("id: 24,"));
}

#[test]
fn cg_to_string_4() {
    let (rust, diags) = compile_to_rust(r#"N = struct
  id: Int = 0
  label: String = "x"
end

fn main() do
  let n = N { id = 32, label = "tag4" }
  print(n.id)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("label: \"tag4\".to_string()"));
    assert!(rust.contains("id: 32,"));
}

#[test]
fn cg_to_string_5() {
    let (rust, diags) = compile_to_rust(r#"N = struct
  id: Int = 0
  label: String = "x"
end

fn main() do
  let n = N { id = 40, label = "tag5" }
  print(n.id)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("label: \"tag5\".to_string()"));
    assert!(rust.contains("id: 40,"));
}

#[test]
fn cg_to_string_6() {
    let (rust, diags) = compile_to_rust(r#"N = struct
  id: Int = 0
  label: String = "x"
end

fn main() do
  let n = N { id = 48, label = "tag6" }
  print(n.id)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("label: \"tag6\".to_string()"));
    assert!(rust.contains("id: 48,"));
}

#[test]
fn cg_to_string_7() {
    let (rust, diags) = compile_to_rust(r#"N = struct
  id: Int = 0
  label: String = "x"
end

fn main() do
  let n = N { id = 56, label = "tag7" }
  print(n.id)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("label: \"tag7\".to_string()"));
    assert!(rust.contains("id: 56,"));
}

#[test]
fn cg_to_string_8() {
    let (rust, diags) = compile_to_rust(r#"N = struct
  id: Int = 0
  label: String = "x"
end

fn main() do
  let n = N { id = 64, label = "tag8" }
  print(n.id)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("label: \"tag8\".to_string()"));
    assert!(rust.contains("id: 64,"));
}

#[test]
fn cg_to_string_9() {
    let (rust, diags) = compile_to_rust(r#"N = struct
  id: Int = 0
  label: String = "x"
end

fn main() do
  let n = N { id = 72, label = "tag9" }
  print(n.id)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("label: \"tag9\".to_string()"));
    assert!(rust.contains("id: 72,"));
}

#[test]
fn cg_to_string_10() {
    let (rust, diags) = compile_to_rust(r#"N = struct
  id: Int = 0
  label: String = "x"
end

fn main() do
  let n = N { id = 80, label = "tag10" }
  print(n.id)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("label: \"tag10\".to_string()"));
    assert!(rust.contains("id: 80,"));
}

#[test]
fn cg_to_string_11() {
    let (rust, diags) = compile_to_rust(r#"N = struct
  id: Int = 0
  label: String = "x"
end

fn main() do
  let n = N { id = 88, label = "tag11" }
  print(n.id)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("label: \"tag11\".to_string()"));
    assert!(rust.contains("id: 88,"));
}

#[test]
fn cg_to_string_12() {
    let (rust, diags) = compile_to_rust(r#"N = struct
  id: Int = 0
  label: String = "x"
end

fn main() do
  let n = N { id = 96, label = "tag12" }
  print(n.id)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("label: \"tag12\".to_string()"));
    assert!(rust.contains("id: 96,"));
}

#[test]
fn cg_to_string_13() {
    let (rust, diags) = compile_to_rust(r#"N = struct
  id: Int = 0
  label: String = "x"
end

fn main() do
  let n = N { id = 104, label = "tag13" }
  print(n.id)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("label: \"tag13\".to_string()"));
    assert!(rust.contains("id: 104,"));
}

#[test]
fn cg_to_string_14() {
    let (rust, diags) = compile_to_rust(r#"N = struct
  id: Int = 0
  label: String = "x"
end

fn main() do
  let n = N { id = 112, label = "tag14" }
  print(n.id)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("label: \"tag14\".to_string()"));
    assert!(rust.contains("id: 112,"));
}

#[test]
fn cg_to_string_15() {
    let (rust, diags) = compile_to_rust(r#"N = struct
  id: Int = 0
  label: String = "x"
end

fn main() do
  let n = N { id = 120, label = "tag15" }
  print(n.id)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("label: \"tag15\".to_string()"));
    assert!(rust.contains("id: 120,"));
}

#[test]
fn cg_let_mut_1() {
    let (rust, diags) = compile_to_rust(r#"fn main() do
  let a = 1
  let b = a + 1
  var c = b
  c = c + 2
  print(c)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("let a: i64 = 1;"));
    assert!(rust.contains("let b = a + 1;"));
    assert!(rust.contains("let mut c = b;"));
}

#[test]
fn cg_let_mut_2() {
    let (rust, diags) = compile_to_rust(r#"fn main() do
  let a = 2
  let b = a + 1
  var c = b
  c = c + 2
  print(c)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("let a: i64 = 2;"));
    assert!(rust.contains("let b = a + 1;"));
    assert!(rust.contains("let mut c = b;"));
}

#[test]
fn cg_let_mut_3() {
    let (rust, diags) = compile_to_rust(r#"fn main() do
  let a = 3
  let b = a + 1
  var c = b
  c = c + 2
  print(c)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("let a: i64 = 3;"));
    assert!(rust.contains("let b = a + 1;"));
    assert!(rust.contains("let mut c = b;"));
}

#[test]
fn cg_let_mut_4() {
    let (rust, diags) = compile_to_rust(r#"fn main() do
  let a = 4
  let b = a + 1
  var c = b
  c = c + 2
  print(c)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("let a: i64 = 4;"));
    assert!(rust.contains("let b = a + 1;"));
    assert!(rust.contains("let mut c = b;"));
}

#[test]
fn cg_let_mut_5() {
    let (rust, diags) = compile_to_rust(r#"fn main() do
  let a = 5
  let b = a + 1
  var c = b
  c = c + 2
  print(c)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("let a: i64 = 5;"));
    assert!(rust.contains("let b = a + 1;"));
    assert!(rust.contains("let mut c = b;"));
}

#[test]
fn cg_let_mut_6() {
    let (rust, diags) = compile_to_rust(r#"fn main() do
  let a = 6
  let b = a + 1
  var c = b
  c = c + 2
  print(c)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("let a: i64 = 6;"));
    assert!(rust.contains("let b = a + 1;"));
    assert!(rust.contains("let mut c = b;"));
}

#[test]
fn cg_let_mut_7() {
    let (rust, diags) = compile_to_rust(r#"fn main() do
  let a = 7
  let b = a + 1
  var c = b
  c = c + 2
  print(c)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("let a: i64 = 7;"));
    assert!(rust.contains("let b = a + 1;"));
    assert!(rust.contains("let mut c = b;"));
}

#[test]
fn cg_let_mut_8() {
    let (rust, diags) = compile_to_rust(r#"fn main() do
  let a = 8
  let b = a + 1
  var c = b
  c = c + 2
  print(c)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("let a: i64 = 8;"));
    assert!(rust.contains("let b = a + 1;"));
    assert!(rust.contains("let mut c = b;"));
}

#[test]
fn cg_let_mut_9() {
    let (rust, diags) = compile_to_rust(r#"fn main() do
  let a = 9
  let b = a + 1
  var c = b
  c = c + 2
  print(c)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("let a: i64 = 9;"));
    assert!(rust.contains("let b = a + 1;"));
    assert!(rust.contains("let mut c = b;"));
}

#[test]
fn cg_let_mut_10() {
    let (rust, diags) = compile_to_rust(r#"fn main() do
  let a = 10
  let b = a + 1
  var c = b
  c = c + 2
  print(c)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("let a: i64 = 10;"));
    assert!(rust.contains("let b = a + 1;"));
    assert!(rust.contains("let mut c = b;"));
}

#[test]
fn cg_let_mut_11() {
    let (rust, diags) = compile_to_rust(r#"fn main() do
  let a = 11
  let b = a + 1
  var c = b
  c = c + 2
  print(c)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("let a: i64 = 11;"));
    assert!(rust.contains("let b = a + 1;"));
    assert!(rust.contains("let mut c = b;"));
}

#[test]
fn cg_let_mut_12() {
    let (rust, diags) = compile_to_rust(r#"fn main() do
  let a = 12
  let b = a + 1
  var c = b
  c = c + 2
  print(c)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("let a: i64 = 12;"));
    assert!(rust.contains("let b = a + 1;"));
    assert!(rust.contains("let mut c = b;"));
}

#[test]
fn cg_let_mut_13() {
    let (rust, diags) = compile_to_rust(r#"fn main() do
  let a = 13
  let b = a + 1
  var c = b
  c = c + 2
  print(c)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("let a: i64 = 13;"));
    assert!(rust.contains("let b = a + 1;"));
    assert!(rust.contains("let mut c = b;"));
}

#[test]
fn cg_let_mut_14() {
    let (rust, diags) = compile_to_rust(r#"fn main() do
  let a = 14
  let b = a + 1
  var c = b
  c = c + 2
  print(c)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("let a: i64 = 14;"));
    assert!(rust.contains("let b = a + 1;"));
    assert!(rust.contains("let mut c = b;"));
}

#[test]
fn cg_let_mut_15() {
    let (rust, diags) = compile_to_rust(r#"fn main() do
  let a = 15
  let b = a + 1
  var c = b
  c = c + 2
  print(c)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("let a: i64 = 15;"));
    assert!(rust.contains("let b = a + 1;"));
    assert!(rust.contains("let mut c = b;"));
}

#[test]
fn cg_let_reassign_mut_1() {
    let (rust, diags) = compile_to_rust(r#"fn main() do
  let t = 2
  t = t + 5
  print(t)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("let mut t: i64 = 2;"));
}

#[test]
fn cg_let_reassign_mut_2() {
    let (rust, diags) = compile_to_rust(r#"fn main() do
  let t = 4
  t = t + 5
  print(t)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("let mut t: i64 = 4;"));
}

#[test]
fn cg_let_reassign_mut_3() {
    let (rust, diags) = compile_to_rust(r#"fn main() do
  let t = 6
  t = t + 5
  print(t)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("let mut t: i64 = 6;"));
}

#[test]
fn cg_let_reassign_mut_4() {
    let (rust, diags) = compile_to_rust(r#"fn main() do
  let t = 8
  t = t + 5
  print(t)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("let mut t: i64 = 8;"));
}

#[test]
fn cg_let_reassign_mut_5() {
    let (rust, diags) = compile_to_rust(r#"fn main() do
  let t = 10
  t = t + 5
  print(t)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("let mut t: i64 = 10;"));
}

#[test]
fn cg_let_reassign_mut_6() {
    let (rust, diags) = compile_to_rust(r#"fn main() do
  let t = 12
  t = t + 5
  print(t)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("let mut t: i64 = 12;"));
}

#[test]
fn cg_let_reassign_mut_7() {
    let (rust, diags) = compile_to_rust(r#"fn main() do
  let t = 14
  t = t + 5
  print(t)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("let mut t: i64 = 14;"));
}

#[test]
fn cg_let_reassign_mut_8() {
    let (rust, diags) = compile_to_rust(r#"fn main() do
  let t = 16
  t = t + 5
  print(t)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("let mut t: i64 = 16;"));
}

#[test]
fn cg_let_reassign_mut_9() {
    let (rust, diags) = compile_to_rust(r#"fn main() do
  let t = 18
  t = t + 5
  print(t)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("let mut t: i64 = 18;"));
}

#[test]
fn cg_let_reassign_mut_10() {
    let (rust, diags) = compile_to_rust(r#"fn main() do
  let t = 20
  t = t + 5
  print(t)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("let mut t: i64 = 20;"));
}

#[test]
fn cg_let_reassign_mut_11() {
    let (rust, diags) = compile_to_rust(r#"fn main() do
  let t = 22
  t = t + 5
  print(t)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("let mut t: i64 = 22;"));
}

#[test]
fn cg_let_reassign_mut_12() {
    let (rust, diags) = compile_to_rust(r#"fn main() do
  let t = 24
  t = t + 5
  print(t)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("let mut t: i64 = 24;"));
}

#[test]
fn cg_let_reassign_mut_13() {
    let (rust, diags) = compile_to_rust(r#"fn main() do
  let t = 26
  t = t + 5
  print(t)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("let mut t: i64 = 26;"));
}

#[test]
fn cg_let_reassign_mut_14() {
    let (rust, diags) = compile_to_rust(r#"fn main() do
  let t = 28
  t = t + 5
  print(t)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("let mut t: i64 = 28;"));
}

#[test]
fn cg_let_reassign_mut_15() {
    let (rust, diags) = compile_to_rust(r#"fn main() do
  let t = 30
  t = t + 5
  print(t)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("let mut t: i64 = 30;"));
}

#[test]
fn cg_pipe_1() {
    let (rust, diags) = compile_to_rust(r#"fn double(x) do
  return x * 2
end

fn main() do
  let y = 1 |> double
  print(y)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("let y = double(1);"));
    assert!(!rust.contains("|>"));
}

#[test]
fn cg_pipe_2() {
    let (rust, diags) = compile_to_rust(r#"fn double(x) do
  return x * 2
end

fn main() do
  let y = 2 |> double
  print(y)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("let y = double(2);"));
    assert!(!rust.contains("|>"));
}

#[test]
fn cg_pipe_3() {
    let (rust, diags) = compile_to_rust(r#"fn double(x) do
  return x * 2
end

fn main() do
  let y = 3 |> double
  print(y)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("let y = double(3);"));
    assert!(!rust.contains("|>"));
}

#[test]
fn cg_pipe_4() {
    let (rust, diags) = compile_to_rust(r#"fn double(x) do
  return x * 2
end

fn main() do
  let y = 4 |> double
  print(y)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("let y = double(4);"));
    assert!(!rust.contains("|>"));
}

#[test]
fn cg_pipe_5() {
    let (rust, diags) = compile_to_rust(r#"fn double(x) do
  return x * 2
end

fn main() do
  let y = 5 |> double
  print(y)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("let y = double(5);"));
    assert!(!rust.contains("|>"));
}

#[test]
fn cg_pipe_6() {
    let (rust, diags) = compile_to_rust(r#"fn double(x) do
  return x * 2
end

fn main() do
  let y = 6 |> double
  print(y)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("let y = double(6);"));
    assert!(!rust.contains("|>"));
}

#[test]
fn cg_pipe_7() {
    let (rust, diags) = compile_to_rust(r#"fn double(x) do
  return x * 2
end

fn main() do
  let y = 7 |> double
  print(y)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("let y = double(7);"));
    assert!(!rust.contains("|>"));
}

#[test]
fn cg_pipe_8() {
    let (rust, diags) = compile_to_rust(r#"fn double(x) do
  return x * 2
end

fn main() do
  let y = 8 |> double
  print(y)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("let y = double(8);"));
    assert!(!rust.contains("|>"));
}

#[test]
fn cg_pipe_9() {
    let (rust, diags) = compile_to_rust(r#"fn double(x) do
  return x * 2
end

fn main() do
  let y = 9 |> double
  print(y)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("let y = double(9);"));
    assert!(!rust.contains("|>"));
}

#[test]
fn cg_pipe_10() {
    let (rust, diags) = compile_to_rust(r#"fn double(x) do
  return x * 2
end

fn main() do
  let y = 10 |> double
  print(y)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("let y = double(10);"));
    assert!(!rust.contains("|>"));
}

#[test]
fn cg_pipe_11() {
    let (rust, diags) = compile_to_rust(r#"fn double(x) do
  return x * 2
end

fn main() do
  let y = 11 |> double
  print(y)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("let y = double(11);"));
    assert!(!rust.contains("|>"));
}

#[test]
fn cg_pipe_12() {
    let (rust, diags) = compile_to_rust(r#"fn double(x) do
  return x * 2
end

fn main() do
  let y = 12 |> double
  print(y)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("let y = double(12);"));
    assert!(!rust.contains("|>"));
}

#[test]
fn cg_pipe_13() {
    let (rust, diags) = compile_to_rust(r#"fn double(x) do
  return x * 2
end

fn main() do
  let y = 13 |> double
  print(y)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("let y = double(13);"));
    assert!(!rust.contains("|>"));
}

#[test]
fn cg_pipe_14() {
    let (rust, diags) = compile_to_rust(r#"fn double(x) do
  return x * 2
end

fn main() do
  let y = 14 |> double
  print(y)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("let y = double(14);"));
    assert!(!rust.contains("|>"));
}

#[test]
fn cg_pipe_15() {
    let (rust, diags) = compile_to_rust(r#"fn double(x) do
  return x * 2
end

fn main() do
  let y = 15 |> double
  print(y)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("let y = double(15);"));
    assert!(!rust.contains("|>"));
}

#[test]
fn cg_struct_lit_1() {
    let (rust, diags) = compile_to_rust(r#"V = struct
  x: Int = 0
  y: Int = 0
end

fn main() do
  let v = V { x = 3, y = 5 }
  print(v.x)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("V { x: 3, y: 5 }"));
}

#[test]
fn cg_struct_lit_2() {
    let (rust, diags) = compile_to_rust(r#"V = struct
  x: Int = 0
  y: Int = 0
end

fn main() do
  let v = V { x = 6, y = 10 }
  print(v.x)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("V { x: 6, y: 10 }"));
}

#[test]
fn cg_struct_lit_3() {
    let (rust, diags) = compile_to_rust(r#"V = struct
  x: Int = 0
  y: Int = 0
end

fn main() do
  let v = V { x = 9, y = 15 }
  print(v.x)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("V { x: 9, y: 15 }"));
}

#[test]
fn cg_struct_lit_4() {
    let (rust, diags) = compile_to_rust(r#"V = struct
  x: Int = 0
  y: Int = 0
end

fn main() do
  let v = V { x = 12, y = 20 }
  print(v.x)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("V { x: 12, y: 20 }"));
}

#[test]
fn cg_struct_lit_5() {
    let (rust, diags) = compile_to_rust(r#"V = struct
  x: Int = 0
  y: Int = 0
end

fn main() do
  let v = V { x = 15, y = 25 }
  print(v.x)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("V { x: 15, y: 25 }"));
}

#[test]
fn cg_struct_lit_6() {
    let (rust, diags) = compile_to_rust(r#"V = struct
  x: Int = 0
  y: Int = 0
end

fn main() do
  let v = V { x = 18, y = 30 }
  print(v.x)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("V { x: 18, y: 30 }"));
}

#[test]
fn cg_struct_lit_7() {
    let (rust, diags) = compile_to_rust(r#"V = struct
  x: Int = 0
  y: Int = 0
end

fn main() do
  let v = V { x = 21, y = 35 }
  print(v.x)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("V { x: 21, y: 35 }"));
}

#[test]
fn cg_struct_lit_8() {
    let (rust, diags) = compile_to_rust(r#"V = struct
  x: Int = 0
  y: Int = 0
end

fn main() do
  let v = V { x = 24, y = 40 }
  print(v.x)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("V { x: 24, y: 40 }"));
}

#[test]
fn cg_struct_lit_9() {
    let (rust, diags) = compile_to_rust(r#"V = struct
  x: Int = 0
  y: Int = 0
end

fn main() do
  let v = V { x = 27, y = 45 }
  print(v.x)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("V { x: 27, y: 45 }"));
}

#[test]
fn cg_struct_lit_10() {
    let (rust, diags) = compile_to_rust(r#"V = struct
  x: Int = 0
  y: Int = 0
end

fn main() do
  let v = V { x = 30, y = 50 }
  print(v.x)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("V { x: 30, y: 50 }"));
}

#[test]
fn cg_struct_lit_11() {
    let (rust, diags) = compile_to_rust(r#"V = struct
  x: Int = 0
  y: Int = 0
end

fn main() do
  let v = V { x = 33, y = 55 }
  print(v.x)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("V { x: 33, y: 55 }"));
}

#[test]
fn cg_struct_lit_12() {
    let (rust, diags) = compile_to_rust(r#"V = struct
  x: Int = 0
  y: Int = 0
end

fn main() do
  let v = V { x = 36, y = 60 }
  print(v.x)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("V { x: 36, y: 60 }"));
}

#[test]
fn cg_struct_lit_13() {
    let (rust, diags) = compile_to_rust(r#"V = struct
  x: Int = 0
  y: Int = 0
end

fn main() do
  let v = V { x = 39, y = 65 }
  print(v.x)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("V { x: 39, y: 65 }"));
}

#[test]
fn cg_struct_lit_14() {
    let (rust, diags) = compile_to_rust(r#"V = struct
  x: Int = 0
  y: Int = 0
end

fn main() do
  let v = V { x = 42, y = 70 }
  print(v.x)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("V { x: 42, y: 70 }"));
}

#[test]
fn cg_struct_lit_15() {
    let (rust, diags) = compile_to_rust(r#"V = struct
  x: Int = 0
  y: Int = 0
end

fn main() do
  let v = V { x = 45, y = 75 }
  print(v.x)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("V { x: 45, y: 75 }"));
}

#[test]
fn cg_struct_types_1() {
    let (rust, diags) = compile_to_rust(r#"T1 = struct
  a: Int
  b: Float
  c: Bool
  d: String
end

fn main() do
  print(1)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("a: i64,"));
    assert!(rust.contains("b: f64,"));
    assert!(rust.contains("c: bool,"));
    assert!(rust.contains("d: String,"));
}

#[test]
fn cg_struct_types_2() {
    let (rust, diags) = compile_to_rust(r#"T2 = struct
  a: Int
  b: Float
  c: Bool
  d: String
end

fn main() do
  print(1)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("a: i64,"));
    assert!(rust.contains("b: f64,"));
    assert!(rust.contains("c: bool,"));
    assert!(rust.contains("d: String,"));
}

#[test]
fn cg_struct_types_3() {
    let (rust, diags) = compile_to_rust(r#"T3 = struct
  a: Int
  b: Float
  c: Bool
  d: String
end

fn main() do
  print(1)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("a: i64,"));
    assert!(rust.contains("b: f64,"));
    assert!(rust.contains("c: bool,"));
    assert!(rust.contains("d: String,"));
}

#[test]
fn cg_struct_types_4() {
    let (rust, diags) = compile_to_rust(r#"T4 = struct
  a: Int
  b: Float
  c: Bool
  d: String
end

fn main() do
  print(1)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("a: i64,"));
    assert!(rust.contains("b: f64,"));
    assert!(rust.contains("c: bool,"));
    assert!(rust.contains("d: String,"));
}

#[test]
fn cg_struct_types_5() {
    let (rust, diags) = compile_to_rust(r#"T5 = struct
  a: Int
  b: Float
  c: Bool
  d: String
end

fn main() do
  print(1)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("a: i64,"));
    assert!(rust.contains("b: f64,"));
    assert!(rust.contains("c: bool,"));
    assert!(rust.contains("d: String,"));
}

#[test]
fn cg_struct_types_6() {
    let (rust, diags) = compile_to_rust(r#"T6 = struct
  a: Int
  b: Float
  c: Bool
  d: String
end

fn main() do
  print(1)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("a: i64,"));
    assert!(rust.contains("b: f64,"));
    assert!(rust.contains("c: bool,"));
    assert!(rust.contains("d: String,"));
}

#[test]
fn cg_struct_types_7() {
    let (rust, diags) = compile_to_rust(r#"T7 = struct
  a: Int
  b: Float
  c: Bool
  d: String
end

fn main() do
  print(1)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("a: i64,"));
    assert!(rust.contains("b: f64,"));
    assert!(rust.contains("c: bool,"));
    assert!(rust.contains("d: String,"));
}

#[test]
fn cg_struct_types_8() {
    let (rust, diags) = compile_to_rust(r#"T8 = struct
  a: Int
  b: Float
  c: Bool
  d: String
end

fn main() do
  print(1)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("a: i64,"));
    assert!(rust.contains("b: f64,"));
    assert!(rust.contains("c: bool,"));
    assert!(rust.contains("d: String,"));
}

#[test]
fn cg_struct_types_9() {
    let (rust, diags) = compile_to_rust(r#"T9 = struct
  a: Int
  b: Float
  c: Bool
  d: String
end

fn main() do
  print(1)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("a: i64,"));
    assert!(rust.contains("b: f64,"));
    assert!(rust.contains("c: bool,"));
    assert!(rust.contains("d: String,"));
}

#[test]
fn cg_struct_types_10() {
    let (rust, diags) = compile_to_rust(r#"T10 = struct
  a: Int
  b: Float
  c: Bool
  d: String
end

fn main() do
  print(1)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("a: i64,"));
    assert!(rust.contains("b: f64,"));
    assert!(rust.contains("c: bool,"));
    assert!(rust.contains("d: String,"));
}

#[test]
fn cg_fn_types_1() {
    let (rust, diags) = compile_to_rust(r#"fn calc(a: Int, b: Float) -> Int do
  return a
end

fn main() do
  print(calc(1, 2))
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("fn calc(a: i64, b: f64) -> i64 {"));
}

#[test]
fn cg_fn_types_2() {
    let (rust, diags) = compile_to_rust(r#"fn calc(a: Int, b: Float) -> Int do
  return a
end

fn main() do
  print(calc(1, 2))
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("fn calc(a: i64, b: f64) -> i64 {"));
}

#[test]
fn cg_fn_types_3() {
    let (rust, diags) = compile_to_rust(r#"fn calc(a: Int, b: Float) -> Int do
  return a
end

fn main() do
  print(calc(1, 2))
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("fn calc(a: i64, b: f64) -> i64 {"));
}

#[test]
fn cg_fn_types_4() {
    let (rust, diags) = compile_to_rust(r#"fn calc(a: Int, b: Float) -> Int do
  return a
end

fn main() do
  print(calc(1, 2))
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("fn calc(a: i64, b: f64) -> i64 {"));
}

#[test]
fn cg_fn_types_5() {
    let (rust, diags) = compile_to_rust(r#"fn calc(a: Int, b: Float) -> Int do
  return a
end

fn main() do
  print(calc(1, 2))
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("fn calc(a: i64, b: f64) -> i64 {"));
}

#[test]
fn cg_fn_types_6() {
    let (rust, diags) = compile_to_rust(r#"fn calc(a: Int, b: Float) -> Int do
  return a
end

fn main() do
  print(calc(1, 2))
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("fn calc(a: i64, b: f64) -> i64 {"));
}

#[test]
fn cg_fn_types_7() {
    let (rust, diags) = compile_to_rust(r#"fn calc(a: Int, b: Float) -> Int do
  return a
end

fn main() do
  print(calc(1, 2))
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("fn calc(a: i64, b: f64) -> i64 {"));
}

#[test]
fn cg_fn_types_8() {
    let (rust, diags) = compile_to_rust(r#"fn calc(a: Int, b: Float) -> Int do
  return a
end

fn main() do
  print(calc(1, 2))
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("fn calc(a: i64, b: f64) -> i64 {"));
}

#[test]
fn cg_fn_types_9() {
    let (rust, diags) = compile_to_rust(r#"fn calc(a: Int, b: Float) -> Int do
  return a
end

fn main() do
  print(calc(1, 2))
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("fn calc(a: i64, b: f64) -> i64 {"));
}

#[test]
fn cg_fn_types_10() {
    let (rust, diags) = compile_to_rust(r#"fn calc(a: Int, b: Float) -> Int do
  return a
end

fn main() do
  print(calc(1, 2))
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("fn calc(a: i64, b: f64) -> i64 {"));
}

#[test]
fn cg_ffi_extract_1() {
    let fns = ffi::extract_c_functions(r#"int native_add(int a, int b) { return a + b; }"#);
    assert_eq!(fns.len(), 1);
    assert_eq!(fns[0].name, "native_add");
    assert_eq!(fns[0].ret, "i32");
}

#[test]
fn cg_ffi_extract_2() {
    let fns = ffi::extract_c_functions(r#"long native_sum(long a, long b) { return a + b; }"#);
    assert_eq!(fns.len(), 1);
    assert_eq!(fns[0].name, "native_sum");
    assert_eq!(fns[0].ret, "i64");
}

#[test]
fn cg_ffi_extract_3() {
    let fns = ffi::extract_c_functions(r#"double native_mul(double a, double b) { return a * b; }"#);
    assert_eq!(fns.len(), 1);
    assert_eq!(fns[0].name, "native_mul");
    assert_eq!(fns[0].ret, "f64");
}

#[test]
fn cg_ffi_extract_4() {
    let fns = ffi::extract_c_functions(r#"float native_half(float a) { return a / 2; }"#);
    assert_eq!(fns.len(), 1);
    assert_eq!(fns[0].name, "native_half");
    assert_eq!(fns[0].ret, "f32");
}

#[test]
fn cg_ffi_extract_5() {
    let fns = ffi::extract_c_functions(r#"bool native_eq(int a, int b) { return a == b; }"#);
    assert_eq!(fns.len(), 1);
    assert_eq!(fns[0].name, "native_eq");
    assert_eq!(fns[0].ret, "bool");
}

#[test]
fn cg_ffi_extract_6() {
    let fns = ffi::extract_c_functions(r#"void native_noop() { }"#);
    assert_eq!(fns.len(), 1);
    assert_eq!(fns[0].name, "native_noop");
    assert_eq!(fns[0].ret, "()");
}

#[test]
fn cg_ffi_extract_7() {
    let fns = ffi::extract_c_functions(r#"short native_short(short s) { return s; }"#);
    assert_eq!(fns.len(), 1);
    assert_eq!(fns[0].name, "native_short");
    assert_eq!(fns[0].ret, "i16");
}

#[test]
fn cg_ffi_extract_8() {
    let fns = ffi::extract_c_functions(r#"char native_char(char c) { return c; }"#);
    assert_eq!(fns.len(), 1);
    assert_eq!(fns[0].name, "native_char");
    assert_eq!(fns[0].ret, "i8");
}

#[test]
fn cg_ffi_extract_9() {
    let fns = ffi::extract_c_functions(r#"size_t native_len(const char* s) { return 0; }"#);
    assert_eq!(fns.len(), 1);
    assert_eq!(fns[0].name, "native_len");
    assert_eq!(fns[0].ret, "usize");
}

#[test]
fn cg_ffi_extract_10() {
    let fns = ffi::extract_c_functions(r#"int64_t native_i64(int64_t v) { return v; }"#);
    assert_eq!(fns.len(), 1);
    assert_eq!(fns[0].name, "native_i64");
    assert_eq!(fns[0].ret, "i64");
}

#[test]
fn cg_ffi_extract_11() {
    let fns = ffi::extract_c_functions(r#"int32_t native_i32(int32_t v) { return v; }"#);
    assert_eq!(fns.len(), 1);
    assert_eq!(fns[0].name, "native_i32");
    assert_eq!(fns[0].ret, "i32");
}

#[test]
fn cg_ffi_extract_12() {
    let fns = ffi::extract_c_functions(r#"int8_t native_i8(int8_t v) { return v; }"#);
    assert_eq!(fns.len(), 1);
    assert_eq!(fns[0].name, "native_i8");
    assert_eq!(fns[0].ret, "i8");
}

#[test]
fn cg_ffi_extract_13() {
    let fns = ffi::extract_c_functions(r#"int16_t native_i16(int16_t v) { return v; }"#);
    assert_eq!(fns.len(), 1);
    assert_eq!(fns[0].name, "native_i16");
    assert_eq!(fns[0].ret, "i16");
}

#[test]
fn cg_ffi_extract_14() {
    let fns = ffi::extract_c_functions(r#"void* native_alloc(size_t n) { return 0; }"#);
    assert_eq!(fns.len(), 1);
    assert_eq!(fns[0].name, "native_alloc");
    assert_eq!(fns[0].ret, "*mut std::ffi::c_void");
}

#[test]
fn cg_ffi_extract_15() {
    let fns = ffi::extract_c_functions(r#"const char* native_name() { return "x"; }"#);
    assert_eq!(fns.len(), 1);
    assert_eq!(fns[0].name, "native_name");
    assert_eq!(fns[0].ret, "*const std::ffi::c_char");
}

#[test]
fn cg_ffi_mangle_1() {
    let mangled = ffi::cpp_mangle("cpp_fn_1", &[]);
    assert_eq!(mangled, "_Z8cpp_fn_1v");
}

#[test]
fn cg_ffi_mangle_2() {
    let mangled = ffi::cpp_mangle("cpp_fn_2", &[]);
    assert_eq!(mangled, "_Z8cpp_fn_2v");
}

#[test]
fn cg_ffi_mangle_3() {
    let mangled = ffi::cpp_mangle("cpp_fn_3", &[]);
    assert_eq!(mangled, "_Z8cpp_fn_3v");
}

#[test]
fn cg_ffi_mangle_4() {
    let mangled = ffi::cpp_mangle("cpp_fn_4", &[]);
    assert_eq!(mangled, "_Z8cpp_fn_4v");
}

#[test]
fn cg_ffi_mangle_5() {
    let mangled = ffi::cpp_mangle("cpp_fn_5", &[]);
    assert_eq!(mangled, "_Z8cpp_fn_5v");
}

#[test]
fn cg_ffi_mangle_6() {
    let mangled = ffi::cpp_mangle("cpp_fn_6", &[]);
    assert_eq!(mangled, "_Z8cpp_fn_6v");
}

#[test]
fn cg_ffi_mangle_7() {
    let mangled = ffi::cpp_mangle("cpp_fn_7", &[]);
    assert_eq!(mangled, "_Z8cpp_fn_7v");
}

#[test]
fn cg_ffi_mangle_8() {
    let mangled = ffi::cpp_mangle("cpp_fn_8", &[]);
    assert_eq!(mangled, "_Z8cpp_fn_8v");
}

#[test]
fn cg_ffi_mangle_9() {
    let mangled = ffi::cpp_mangle("cpp_fn_9", &[]);
    assert_eq!(mangled, "_Z8cpp_fn_9v");
}

#[test]
fn cg_ffi_mangle_10() {
    let mangled = ffi::cpp_mangle("cpp_fn_10", &[]);
    assert_eq!(mangled, "_Z9cpp_fn_10v");
}

#[test]
fn cg_pipeline_text_1() {
    let (rust, diags) = compile_to_rust(r#"fn work(n: Int) -> Int do
  var t = 0
  for i in 100 do
    t = t + i
  end
  return t
end

fn main() do
  print(work(3))
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("fn work(n: i64) -> i64 {"));
    assert!(rust.contains("for i in 0..100 {"));
    assert!(rust.contains("return t;"));
    assert!(rust.contains("fn main() {"));
    assert!(rust.contains("println!"));
}

#[test]
fn cg_pipeline_text_2() {
    let (rust, diags) = compile_to_rust(r#"fn work(n: Int) -> Int do
  var t = 0
  for i in 200 do
    t = t + i
  end
  return t
end

fn main() do
  print(work(3))
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("fn work(n: i64) -> i64 {"));
    assert!(rust.contains("for i in 0..200 {"));
    assert!(rust.contains("return t;"));
    assert!(rust.contains("fn main() {"));
    assert!(rust.contains("println!"));
}

#[test]
fn cg_pipeline_text_3() {
    let (rust, diags) = compile_to_rust(r#"fn work(n: Int) -> Int do
  var t = 0
  for i in 300 do
    t = t + i
  end
  return t
end

fn main() do
  print(work(3))
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("fn work(n: i64) -> i64 {"));
    assert!(rust.contains("for i in 0..300 {"));
    assert!(rust.contains("return t;"));
    assert!(rust.contains("fn main() {"));
    assert!(rust.contains("println!"));
}

#[test]
fn cg_pipeline_text_4() {
    let (rust, diags) = compile_to_rust(r#"fn work(n: Int) -> Int do
  var t = 0
  for i in 400 do
    t = t + i
  end
  return t
end

fn main() do
  print(work(3))
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("fn work(n: i64) -> i64 {"));
    assert!(rust.contains("for i in 0..400 {"));
    assert!(rust.contains("return t;"));
    assert!(rust.contains("fn main() {"));
    assert!(rust.contains("println!"));
}

#[test]
fn cg_pipeline_text_5() {
    let (rust, diags) = compile_to_rust(r#"fn work(n: Int) -> Int do
  var t = 0
  for i in 500 do
    t = t + i
  end
  return t
end

fn main() do
  print(work(3))
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("fn work(n: i64) -> i64 {"));
    assert!(rust.contains("for i in 0..500 {"));
    assert!(rust.contains("return t;"));
    assert!(rust.contains("fn main() {"));
    assert!(rust.contains("println!"));
}

#[test]
fn cg_pipeline_text_6() {
    let (rust, diags) = compile_to_rust(r#"fn work(n: Int) -> Int do
  var t = 0
  for i in 600 do
    t = t + i
  end
  return t
end

fn main() do
  print(work(3))
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("fn work(n: i64) -> i64 {"));
    assert!(rust.contains("for i in 0..600 {"));
    assert!(rust.contains("return t;"));
    assert!(rust.contains("fn main() {"));
    assert!(rust.contains("println!"));
}

#[test]
fn cg_pipeline_text_7() {
    let (rust, diags) = compile_to_rust(r#"fn work(n: Int) -> Int do
  var t = 0
  for i in 700 do
    t = t + i
  end
  return t
end

fn main() do
  print(work(3))
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("fn work(n: i64) -> i64 {"));
    assert!(rust.contains("for i in 0..700 {"));
    assert!(rust.contains("return t;"));
    assert!(rust.contains("fn main() {"));
    assert!(rust.contains("println!"));
}

#[test]
fn cg_pipeline_text_8() {
    let (rust, diags) = compile_to_rust(r#"fn work(n: Int) -> Int do
  var t = 0
  for i in 800 do
    t = t + i
  end
  return t
end

fn main() do
  print(work(3))
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("fn work(n: i64) -> i64 {"));
    assert!(rust.contains("for i in 0..800 {"));
    assert!(rust.contains("return t;"));
    assert!(rust.contains("fn main() {"));
    assert!(rust.contains("println!"));
}

#[test]
fn cg_pipeline_text_9() {
    let (rust, diags) = compile_to_rust(r#"fn work(n: Int) -> Int do
  var t = 0
  for i in 900 do
    t = t + i
  end
  return t
end

fn main() do
  print(work(3))
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("fn work(n: i64) -> i64 {"));
    assert!(rust.contains("for i in 0..900 {"));
    assert!(rust.contains("return t;"));
    assert!(rust.contains("fn main() {"));
    assert!(rust.contains("println!"));
}

#[test]
fn cg_pipeline_text_10() {
    let (rust, diags) = compile_to_rust(r#"fn work(n: Int) -> Int do
  var t = 0
  for i in 1000 do
    t = t + i
  end
  return t
end

fn main() do
  print(work(3))
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("fn work(n: i64) -> i64 {"));
    assert!(rust.contains("for i in 0..1000 {"));
    assert!(rust.contains("return t;"));
    assert!(rust.contains("fn main() {"));
    assert!(rust.contains("println!"));
}

#[test]
fn cg_array_1() {
    let (rust, diags) = compile_to_rust(r#"fn main() do
  var xs = [1, 2, 3]
  xs.push(10)
  print(xs.len())
  print(xs[2])
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("let mut xs = vec![1, 2, 3];"));
    assert!(rust.contains("xs.push(10)"));
    assert!(rust.contains("xs.len()"));
    assert!(rust.contains("xs[2]"));
}

#[test]
fn cg_array_2() {
    let (rust, diags) = compile_to_rust(r#"fn main() do
  var xs = [2, 3, 4]
  xs.push(20)
  print(xs.len())
  print(xs[2])
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("let mut xs = vec![2, 3, 4];"));
    assert!(rust.contains("xs.push(20)"));
    assert!(rust.contains("xs.len()"));
    assert!(rust.contains("xs[2]"));
}

#[test]
fn cg_array_3() {
    let (rust, diags) = compile_to_rust(r#"fn main() do
  var xs = [3, 4, 5]
  xs.push(30)
  print(xs.len())
  print(xs[2])
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("let mut xs = vec![3, 4, 5];"));
    assert!(rust.contains("xs.push(30)"));
    assert!(rust.contains("xs.len()"));
    assert!(rust.contains("xs[2]"));
}

#[test]
fn cg_array_4() {
    let (rust, diags) = compile_to_rust(r#"fn main() do
  var xs = [4, 5, 6]
  xs.push(40)
  print(xs.len())
  print(xs[2])
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("let mut xs = vec![4, 5, 6];"));
    assert!(rust.contains("xs.push(40)"));
    assert!(rust.contains("xs.len()"));
    assert!(rust.contains("xs[2]"));
}

#[test]
fn cg_array_5() {
    let (rust, diags) = compile_to_rust(r#"fn main() do
  var xs = [5, 6, 7]
  xs.push(50)
  print(xs.len())
  print(xs[2])
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("let mut xs = vec![5, 6, 7];"));
    assert!(rust.contains("xs.push(50)"));
    assert!(rust.contains("xs.len()"));
    assert!(rust.contains("xs[2]"));
}

#[test]
fn cg_array_6() {
    let (rust, diags) = compile_to_rust(r#"fn main() do
  var xs = [6, 7, 8]
  xs.push(60)
  print(xs.len())
  print(xs[2])
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("let mut xs = vec![6, 7, 8];"));
    assert!(rust.contains("xs.push(60)"));
    assert!(rust.contains("xs.len()"));
    assert!(rust.contains("xs[2]"));
}

#[test]
fn cg_array_7() {
    let (rust, diags) = compile_to_rust(r#"fn main() do
  var xs = [7, 8, 9]
  xs.push(70)
  print(xs.len())
  print(xs[2])
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("let mut xs = vec![7, 8, 9];"));
    assert!(rust.contains("xs.push(70)"));
    assert!(rust.contains("xs.len()"));
    assert!(rust.contains("xs[2]"));
}

#[test]
fn cg_array_8() {
    let (rust, diags) = compile_to_rust(r#"fn main() do
  var xs = [8, 9, 10]
  xs.push(80)
  print(xs.len())
  print(xs[2])
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("let mut xs = vec![8, 9, 10];"));
    assert!(rust.contains("xs.push(80)"));
    assert!(rust.contains("xs.len()"));
    assert!(rust.contains("xs[2]"));
}

#[test]
fn cg_array_9() {
    let (rust, diags) = compile_to_rust(r#"fn main() do
  var xs = [9, 10, 11]
  xs.push(90)
  print(xs.len())
  print(xs[2])
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("let mut xs = vec![9, 10, 11];"));
    assert!(rust.contains("xs.push(90)"));
    assert!(rust.contains("xs.len()"));
    assert!(rust.contains("xs[2]"));
}

#[test]
fn cg_array_10() {
    let (rust, diags) = compile_to_rust(r#"fn main() do
  var xs = [10, 11, 12]
  xs.push(100)
  print(xs.len())
  print(xs[2])
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("let mut xs = vec![10, 11, 12];"));
    assert!(rust.contains("xs.push(100)"));
    assert!(rust.contains("xs.len()"));
    assert!(rust.contains("xs[2]"));
}
