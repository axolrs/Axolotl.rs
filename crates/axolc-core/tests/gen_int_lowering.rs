// Owner: PascalElixir / axolrs (GitHub org)
// File: crates/axolc-core/tests/gen_int_lowering.rs - golden tests for 64-bit Int lowering of bare integer literals.

use axolc_core::compile_to_rust;

/// Bare let with an integer literal emits an i64 ascription.
#[test]
fn int_lowering_let_literal() {
    let (rust, diags) = compile_to_rust(r#"fn main() do
  let a = 5
  print(a)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("let a: i64 = 5;"));
}

/// Bare var with an integer literal emits a mutable i64 binding.
#[test]
fn int_lowering_var_literal() {
    let (rust, diags) = compile_to_rust(r#"fn main() do
  var x = 0
  x = x + 1
  print(x)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("let mut x: i64 = 0;"));
}

/// Ident propagation copies the Int kind to the new binding.
#[test]
fn int_lowering_ident_propagation() {
    let (rust, diags) = compile_to_rust(r#"fn main() do
  let a = 5
  var c = a
  c = c + 2
  print(c)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("let a: i64 = 5;"));
    assert!(rust.contains("let mut c: i64 = a;"));
}

/// Binary-expression initializers stay unannotated (inference anchors elsewhere).
#[test]
fn int_lowering_binexpr_no_annotation() {
    let (rust, diags) = compile_to_rust(r#"fn main() do
  let a = 5
  let b = a + 1
  print(b)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("let a: i64 = 5;"));
    assert!(rust.contains("let b = a + 1;"));
}

/// Float literals keep Rust's f64 default without an ascription.
#[test]
fn int_lowering_float_untouched() {
    let (rust, diags) = compile_to_rust(r#"fn main() do
  let f = 1.5
  print(f)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("let f = 1.5;"));
    assert!(!rust.contains("let f: i64"));
}

/// Bool literals keep Rust's bool default without an ascription.
#[test]
fn int_lowering_bool_untouched() {
    let (rust, diags) = compile_to_rust(r#"fn main() do
  let flag = true
  print(flag)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("let flag = true;"));
    assert!(!rust.contains("let flag: i64"));
}

/// String literals never receive an Int ascription.
#[test]
fn int_lowering_string_untouched() {
    let (rust, diags) = compile_to_rust(r#"fn main() do
  let s = "hi"
  print(s)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("let s ="));
    assert!(!rust.contains("let s: i64"));
}

/// An explicit Int annotation still emits i64 through the typed path.
#[test]
fn int_lowering_explicit_annotation() {
    let (rust, diags) = compile_to_rust(r#"fn main() do
  let x: Int = 0
  print(x)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("let x: i64 = 0;"));
}

/// Literals beyond i32 range lower safely because the binding is i64.
#[test]
fn int_lowering_large_literal_fits() {
    let (rust, diags) = compile_to_rust(r#"fn main() do
  let big = 3000000000
  print(big)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("let big: i64 = 3000000000;"));
}

/// Negative literal initializers (unary minus over IntLit) still annotate.
#[test]
fn int_lowering_negative_literal() {
    let (rust, diags) = compile_to_rust(r#"fn main() do
  let n = -7
  print(n)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("let n: i64 = -(7);"));
}

/// For-range loop variables anchored by an i64 accumulator unify to i64.
#[test]
fn int_lowering_for_range_anchor() {
    let (rust, diags) = compile_to_rust(r#"fn main() do
  var total = 0
  for i in 0..10 do
    total = total + i
  end
  print(total)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("let mut total: i64 = 0;"));
    assert!(rust.contains("for i in 0..10"));
}

/// Array and map literals never receive an Int ascription.
#[test]
fn int_lowering_collections_untouched() {
    let (rust, diags) = compile_to_rust(r#"fn main() do
  let xs = [1, 2, 3]
  print(xs.len())
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("let xs = vec![1, 2, 3]"));
    assert!(!rust.contains("let xs: i64"));
}

/// Assignment to an Int-kind var after declaration keeps the i64 binding.
#[test]
fn int_lowering_assign_keeps_type() {
    let (rust, diags) = compile_to_rust(r#"fn main() do
  var c = 0
  c = 42
  print(c)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("let mut c: i64 = 0;"));
    assert!(rust.contains("c = 42;"));
}

/// Function-local lets inside nested blocks also get the Int lowering.
#[test]
fn int_lowering_nested_block() {
    let (rust, diags) = compile_to_rust(r#"fn main() do
  if true then
    let inner = 3
    print(inner)
  end
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("let inner: i64 = 3;"));
}

/// A top-level Int const used as a count-loop bound emits `0..NAME`.
#[test]
fn int_lowering_const_count_loop() {
    let (rust, diags) = compile_to_rust(r#"const WORKLOAD: Int = 6000000

fn main() do
  var total = 0
  for i in WORKLOAD do
    total = total + 1
  end
  print(total)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("const WORKLOAD: i64 = 6000000;"));
    assert!(rust.contains("for i in 0..WORKLOAD"));
}

/// A const defined after its use still normalizes the count loop.
#[test]
fn int_lowering_const_after_use() {
    let (rust, diags) = compile_to_rust(r#"fn main() do
  var total = 0
  for i in WORKLOAD do
    total = total + 1
  end
  print(total)
end

const WORKLOAD: Int = 42"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("for i in 0..WORKLOAD"));
}

/// A non-integer const never triggers the count-loop rewrite.
#[test]
fn int_lowering_string_const_not_count_loop() {
    let (rust, diags) = compile_to_rust(r#"const NAME: String = "axol"

fn main() do
  print(NAME)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(!rust.contains("0..NAME"));
}
