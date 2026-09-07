// Owner: PascalElixir / axolrs (GitHub org)
// File: Auto-generated tests for ownership inference: E0101 use-after-move, E0102 move-out-of-borrow, E0103 dangling return, E0104 write-through-borrow, parameter modes, advisory hints, move sites, and clean programs.

use axolc_core::infer_ownership;
use axolc_core::ownership::ParamMode;

/// Collect the diagnostic codes produced by ownership inference for a source.
fn own_codes(src: &str) -> Vec<String> {
    let (_, diags) = infer_ownership(src, 0);
    diags.items.iter().filter_map(|d| d.code.clone()).collect()
}

/// Count error-severity diagnostics from ownership inference.
fn own_errors(src: &str) -> usize {
    let (_, diags) = infer_ownership(src, 0);
    diags.error_count()
}

/// Resolve one parameter mode.
fn own_mode(src: &str, fname: &str, pname: &str) -> Option<ParamMode> {
    let (table, _) = infer_ownership(src, 0);
    table.param_mode(fname, pname)
}

/// Collect the advisory hints for one function.
fn own_hints(src: &str, fname: &str) -> Vec<(String, ParamMode)> {
    let (table, _) = infer_ownership(src, 0);
    table.hints_for(fname)
}

/// Resolve all parameter modes for one function.
fn own_modes(src: &str, fname: &str) -> Vec<(String, ParamMode)> {
    let (table, _) = infer_ownership(src, 0);
    table.modes_for(fname)
}

/// Collect move sites (function, moved name, callee).
fn own_moves(src: &str) -> Vec<(String, String, String)> {
    let (table, _) = infer_ownership(src, 0);
    table.moves.iter().map(|(f, m)| (f.clone(), m.name.clone(), m.callee.clone())).collect()
}

#[test]
fn own_e0101_fires_1() {
    let codes = own_codes(r#"fn take(p: move P) do
  print(p.x)
end

fn main() do
  let a = P { x = 1 }
  take(a)
  print(a.x)
end"#);
    assert!(codes.iter().any(|c| c == "E0101"), "expected E0101, got {:?}", codes);
    assert!(own_errors(r#"fn take(p: move P) do
  print(p.x)
end

fn main() do
  let a = P { x = 1 }
  take(a)
  print(a.x)
end"#) >= 1);
}

#[test]
fn own_e0101_fires_2() {
    let codes = own_codes(r#"fn take(p: move P) do
  print(p.x)
end

fn main() do
  let a = P { x = 2 }
  take(a)
  print(a.x)
end"#);
    assert!(codes.iter().any(|c| c == "E0101"), "expected E0101, got {:?}", codes);
    assert!(own_errors(r#"fn take(p: move P) do
  print(p.x)
end

fn main() do
  let a = P { x = 2 }
  take(a)
  print(a.x)
end"#) >= 1);
}

#[test]
fn own_e0101_fires_3() {
    let codes = own_codes(r#"fn take(p: move P) do
  print(p.x)
end

fn main() do
  let a = P { x = 3 }
  take(a)
  print(a.x)
end"#);
    assert!(codes.iter().any(|c| c == "E0101"), "expected E0101, got {:?}", codes);
    assert!(own_errors(r#"fn take(p: move P) do
  print(p.x)
end

fn main() do
  let a = P { x = 3 }
  take(a)
  print(a.x)
end"#) >= 1);
}

#[test]
fn own_e0101_fires_4() {
    let codes = own_codes(r#"fn take(p: move P) do
  print(p.x)
end

fn main() do
  let a = P { x = 4 }
  take(a)
  print(a.x)
end"#);
    assert!(codes.iter().any(|c| c == "E0101"), "expected E0101, got {:?}", codes);
    assert!(own_errors(r#"fn take(p: move P) do
  print(p.x)
end

fn main() do
  let a = P { x = 4 }
  take(a)
  print(a.x)
end"#) >= 1);
}

#[test]
fn own_e0101_fires_5() {
    let codes = own_codes(r#"fn take(p: move P) do
  print(p.x)
end

fn main() do
  let a = P { x = 5 }
  take(a)
  print(a.x)
end"#);
    assert!(codes.iter().any(|c| c == "E0101"), "expected E0101, got {:?}", codes);
    assert!(own_errors(r#"fn take(p: move P) do
  print(p.x)
end

fn main() do
  let a = P { x = 5 }
  take(a)
  print(a.x)
end"#) >= 1);
}

#[test]
fn own_e0101_fires_6() {
    let codes = own_codes(r#"fn take(p: move P) do
  print(p.x)
end

fn main() do
  let a = P { x = 6 }
  take(a)
  print(a.x)
end"#);
    assert!(codes.iter().any(|c| c == "E0101"), "expected E0101, got {:?}", codes);
    assert!(own_errors(r#"fn take(p: move P) do
  print(p.x)
end

fn main() do
  let a = P { x = 6 }
  take(a)
  print(a.x)
end"#) >= 1);
}

#[test]
fn own_e0101_fires_7() {
    let codes = own_codes(r#"fn take(p: move P) do
  print(p.x)
end

fn main() do
  let a = P { x = 7 }
  take(a)
  print(a.x)
end"#);
    assert!(codes.iter().any(|c| c == "E0101"), "expected E0101, got {:?}", codes);
    assert!(own_errors(r#"fn take(p: move P) do
  print(p.x)
end

fn main() do
  let a = P { x = 7 }
  take(a)
  print(a.x)
end"#) >= 1);
}

#[test]
fn own_e0101_fires_8() {
    let codes = own_codes(r#"fn take(p: move P) do
  print(p.x)
end

fn main() do
  let a = P { x = 8 }
  take(a)
  print(a.x)
end"#);
    assert!(codes.iter().any(|c| c == "E0101"), "expected E0101, got {:?}", codes);
    assert!(own_errors(r#"fn take(p: move P) do
  print(p.x)
end

fn main() do
  let a = P { x = 8 }
  take(a)
  print(a.x)
end"#) >= 1);
}

#[test]
fn own_e0101_fires_9() {
    let codes = own_codes(r#"fn take(p: move P) do
  print(p.x)
end

fn main() do
  let a = P { x = 9 }
  take(a)
  print(a.x)
end"#);
    assert!(codes.iter().any(|c| c == "E0101"), "expected E0101, got {:?}", codes);
    assert!(own_errors(r#"fn take(p: move P) do
  print(p.x)
end

fn main() do
  let a = P { x = 9 }
  take(a)
  print(a.x)
end"#) >= 1);
}

#[test]
fn own_e0101_fires_10() {
    let codes = own_codes(r#"fn take(p: move P) do
  print(p.x)
end

fn main() do
  let a = P { x = 10 }
  take(a)
  print(a.x)
end"#);
    assert!(codes.iter().any(|c| c == "E0101"), "expected E0101, got {:?}", codes);
    assert!(own_errors(r#"fn take(p: move P) do
  print(p.x)
end

fn main() do
  let a = P { x = 10 }
  take(a)
  print(a.x)
end"#) >= 1);
}

#[test]
fn own_e0101_fires_11() {
    let codes = own_codes(r#"fn take(p: move P) do
  print(p.x)
end

fn main() do
  let a = P { x = 11 }
  take(a)
  print(a.x)
end"#);
    assert!(codes.iter().any(|c| c == "E0101"), "expected E0101, got {:?}", codes);
    assert!(own_errors(r#"fn take(p: move P) do
  print(p.x)
end

fn main() do
  let a = P { x = 11 }
  take(a)
  print(a.x)
end"#) >= 1);
}

#[test]
fn own_e0101_fires_12() {
    let codes = own_codes(r#"fn take(p: move P) do
  print(p.x)
end

fn main() do
  let a = P { x = 12 }
  take(a)
  print(a.x)
end"#);
    assert!(codes.iter().any(|c| c == "E0101"), "expected E0101, got {:?}", codes);
    assert!(own_errors(r#"fn take(p: move P) do
  print(p.x)
end

fn main() do
  let a = P { x = 12 }
  take(a)
  print(a.x)
end"#) >= 1);
}

#[test]
fn own_e0101_fires_13() {
    let codes = own_codes(r#"fn take(p: move P) do
  print(p.x)
end

fn main() do
  let a = P { x = 13 }
  take(a)
  print(a.x)
end"#);
    assert!(codes.iter().any(|c| c == "E0101"), "expected E0101, got {:?}", codes);
    assert!(own_errors(r#"fn take(p: move P) do
  print(p.x)
end

fn main() do
  let a = P { x = 13 }
  take(a)
  print(a.x)
end"#) >= 1);
}

#[test]
fn own_e0101_fires_14() {
    let codes = own_codes(r#"fn take(p: move P) do
  print(p.x)
end

fn main() do
  let a = P { x = 14 }
  take(a)
  print(a.x)
end"#);
    assert!(codes.iter().any(|c| c == "E0101"), "expected E0101, got {:?}", codes);
    assert!(own_errors(r#"fn take(p: move P) do
  print(p.x)
end

fn main() do
  let a = P { x = 14 }
  take(a)
  print(a.x)
end"#) >= 1);
}

#[test]
fn own_e0101_fires_15() {
    let codes = own_codes(r#"fn take(p: move P) do
  print(p.x)
end

fn main() do
  let a = P { x = 15 }
  take(a)
  print(a.x)
end"#);
    assert!(codes.iter().any(|c| c == "E0101"), "expected E0101, got {:?}", codes);
    assert!(own_errors(r#"fn take(p: move P) do
  print(p.x)
end

fn main() do
  let a = P { x = 15 }
  take(a)
  print(a.x)
end"#) >= 1);
}

#[test]
fn own_e0101_fires_16() {
    let codes = own_codes(r#"fn take(p: move P) do
  print(p.x)
end

fn main() do
  let a = P { x = 16 }
  take(a)
  print(a.x)
end"#);
    assert!(codes.iter().any(|c| c == "E0101"), "expected E0101, got {:?}", codes);
    assert!(own_errors(r#"fn take(p: move P) do
  print(p.x)
end

fn main() do
  let a = P { x = 16 }
  take(a)
  print(a.x)
end"#) >= 1);
}

#[test]
fn own_e0101_fires_17() {
    let codes = own_codes(r#"fn take(p: move P) do
  print(p.x)
end

fn main() do
  let a = P { x = 17 }
  take(a)
  print(a.x)
end"#);
    assert!(codes.iter().any(|c| c == "E0101"), "expected E0101, got {:?}", codes);
    assert!(own_errors(r#"fn take(p: move P) do
  print(p.x)
end

fn main() do
  let a = P { x = 17 }
  take(a)
  print(a.x)
end"#) >= 1);
}

#[test]
fn own_e0101_fires_18() {
    let codes = own_codes(r#"fn take(p: move P) do
  print(p.x)
end

fn main() do
  let a = P { x = 18 }
  take(a)
  print(a.x)
end"#);
    assert!(codes.iter().any(|c| c == "E0101"), "expected E0101, got {:?}", codes);
    assert!(own_errors(r#"fn take(p: move P) do
  print(p.x)
end

fn main() do
  let a = P { x = 18 }
  take(a)
  print(a.x)
end"#) >= 1);
}

#[test]
fn own_e0101_fires_19() {
    let codes = own_codes(r#"fn take(p: move P) do
  print(p.x)
end

fn main() do
  let a = P { x = 19 }
  take(a)
  print(a.x)
end"#);
    assert!(codes.iter().any(|c| c == "E0101"), "expected E0101, got {:?}", codes);
    assert!(own_errors(r#"fn take(p: move P) do
  print(p.x)
end

fn main() do
  let a = P { x = 19 }
  take(a)
  print(a.x)
end"#) >= 1);
}

#[test]
fn own_e0101_fires_20() {
    let codes = own_codes(r#"fn take(p: move P) do
  print(p.x)
end

fn main() do
  let a = P { x = 20 }
  take(a)
  print(a.x)
end"#);
    assert!(codes.iter().any(|c| c == "E0101"), "expected E0101, got {:?}", codes);
    assert!(own_errors(r#"fn take(p: move P) do
  print(p.x)
end

fn main() do
  let a = P { x = 20 }
  take(a)
  print(a.x)
end"#) >= 1);
}

#[test]
fn own_e0101_fires_21() {
    let codes = own_codes(r#"fn take(p: move P) do
  print(p.x)
end

fn main() do
  let a = P { x = 21 }
  take(a)
  print(a.x)
end"#);
    assert!(codes.iter().any(|c| c == "E0101"), "expected E0101, got {:?}", codes);
    assert!(own_errors(r#"fn take(p: move P) do
  print(p.x)
end

fn main() do
  let a = P { x = 21 }
  take(a)
  print(a.x)
end"#) >= 1);
}

#[test]
fn own_e0101_fires_22() {
    let codes = own_codes(r#"fn take(p: move P) do
  print(p.x)
end

fn main() do
  let a = P { x = 22 }
  take(a)
  print(a.x)
end"#);
    assert!(codes.iter().any(|c| c == "E0101"), "expected E0101, got {:?}", codes);
    assert!(own_errors(r#"fn take(p: move P) do
  print(p.x)
end

fn main() do
  let a = P { x = 22 }
  take(a)
  print(a.x)
end"#) >= 1);
}

#[test]
fn own_e0101_fires_23() {
    let codes = own_codes(r#"fn take(p: move P) do
  print(p.x)
end

fn main() do
  let a = P { x = 23 }
  take(a)
  print(a.x)
end"#);
    assert!(codes.iter().any(|c| c == "E0101"), "expected E0101, got {:?}", codes);
    assert!(own_errors(r#"fn take(p: move P) do
  print(p.x)
end

fn main() do
  let a = P { x = 23 }
  take(a)
  print(a.x)
end"#) >= 1);
}

#[test]
fn own_e0101_fires_24() {
    let codes = own_codes(r#"fn take(p: move P) do
  print(p.x)
end

fn main() do
  let a = P { x = 24 }
  take(a)
  print(a.x)
end"#);
    assert!(codes.iter().any(|c| c == "E0101"), "expected E0101, got {:?}", codes);
    assert!(own_errors(r#"fn take(p: move P) do
  print(p.x)
end

fn main() do
  let a = P { x = 24 }
  take(a)
  print(a.x)
end"#) >= 1);
}

#[test]
fn own_e0101_fires_25() {
    let codes = own_codes(r#"fn take(p: move P) do
  print(p.x)
end

fn main() do
  let a = P { x = 25 }
  take(a)
  print(a.x)
end"#);
    assert!(codes.iter().any(|c| c == "E0101"), "expected E0101, got {:?}", codes);
    assert!(own_errors(r#"fn take(p: move P) do
  print(p.x)
end

fn main() do
  let a = P { x = 25 }
  take(a)
  print(a.x)
end"#) >= 1);
}

#[test]
fn own_e0101_not_before_1() {
    let codes = own_codes(r#"fn take(p: move P) do
  print(p.x)
end

fn main() do
  let a = P { x = 1 }
  print(a.x)
  take(a)
  print(1)
end"#);
    assert!(!codes.iter().any(|c| c == "E0101"), "unexpected E0101, got {:?}", codes);
}

#[test]
fn own_e0101_not_before_2() {
    let codes = own_codes(r#"fn take(p: move P) do
  print(p.x)
end

fn main() do
  let a = P { x = 2 }
  print(a.x)
  take(a)
  print(1)
end"#);
    assert!(!codes.iter().any(|c| c == "E0101"), "unexpected E0101, got {:?}", codes);
}

#[test]
fn own_e0101_not_before_3() {
    let codes = own_codes(r#"fn take(p: move P) do
  print(p.x)
end

fn main() do
  let a = P { x = 3 }
  print(a.x)
  take(a)
  print(1)
end"#);
    assert!(!codes.iter().any(|c| c == "E0101"), "unexpected E0101, got {:?}", codes);
}

#[test]
fn own_e0101_not_before_4() {
    let codes = own_codes(r#"fn take(p: move P) do
  print(p.x)
end

fn main() do
  let a = P { x = 4 }
  print(a.x)
  take(a)
  print(1)
end"#);
    assert!(!codes.iter().any(|c| c == "E0101"), "unexpected E0101, got {:?}", codes);
}

#[test]
fn own_e0101_not_before_5() {
    let codes = own_codes(r#"fn take(p: move P) do
  print(p.x)
end

fn main() do
  let a = P { x = 5 }
  print(a.x)
  take(a)
  print(1)
end"#);
    assert!(!codes.iter().any(|c| c == "E0101"), "unexpected E0101, got {:?}", codes);
}

#[test]
fn own_e0101_not_before_6() {
    let codes = own_codes(r#"fn take(p: move P) do
  print(p.x)
end

fn main() do
  let a = P { x = 6 }
  print(a.x)
  take(a)
  print(1)
end"#);
    assert!(!codes.iter().any(|c| c == "E0101"), "unexpected E0101, got {:?}", codes);
}

#[test]
fn own_e0101_not_before_7() {
    let codes = own_codes(r#"fn take(p: move P) do
  print(p.x)
end

fn main() do
  let a = P { x = 7 }
  print(a.x)
  take(a)
  print(1)
end"#);
    assert!(!codes.iter().any(|c| c == "E0101"), "unexpected E0101, got {:?}", codes);
}

#[test]
fn own_e0101_not_before_8() {
    let codes = own_codes(r#"fn take(p: move P) do
  print(p.x)
end

fn main() do
  let a = P { x = 8 }
  print(a.x)
  take(a)
  print(1)
end"#);
    assert!(!codes.iter().any(|c| c == "E0101"), "unexpected E0101, got {:?}", codes);
}

#[test]
fn own_e0101_not_before_9() {
    let codes = own_codes(r#"fn take(p: move P) do
  print(p.x)
end

fn main() do
  let a = P { x = 9 }
  print(a.x)
  take(a)
  print(1)
end"#);
    assert!(!codes.iter().any(|c| c == "E0101"), "unexpected E0101, got {:?}", codes);
}

#[test]
fn own_e0101_not_before_10() {
    let codes = own_codes(r#"fn take(p: move P) do
  print(p.x)
end

fn main() do
  let a = P { x = 10 }
  print(a.x)
  take(a)
  print(1)
end"#);
    assert!(!codes.iter().any(|c| c == "E0101"), "unexpected E0101, got {:?}", codes);
}

#[test]
fn own_e0101_not_before_11() {
    let codes = own_codes(r#"fn take(p: move P) do
  print(p.x)
end

fn main() do
  let a = P { x = 11 }
  print(a.x)
  take(a)
  print(1)
end"#);
    assert!(!codes.iter().any(|c| c == "E0101"), "unexpected E0101, got {:?}", codes);
}

#[test]
fn own_e0101_not_before_12() {
    let codes = own_codes(r#"fn take(p: move P) do
  print(p.x)
end

fn main() do
  let a = P { x = 12 }
  print(a.x)
  take(a)
  print(1)
end"#);
    assert!(!codes.iter().any(|c| c == "E0101"), "unexpected E0101, got {:?}", codes);
}

#[test]
fn own_e0101_not_before_13() {
    let codes = own_codes(r#"fn take(p: move P) do
  print(p.x)
end

fn main() do
  let a = P { x = 13 }
  print(a.x)
  take(a)
  print(1)
end"#);
    assert!(!codes.iter().any(|c| c == "E0101"), "unexpected E0101, got {:?}", codes);
}

#[test]
fn own_e0101_not_before_14() {
    let codes = own_codes(r#"fn take(p: move P) do
  print(p.x)
end

fn main() do
  let a = P { x = 14 }
  print(a.x)
  take(a)
  print(1)
end"#);
    assert!(!codes.iter().any(|c| c == "E0101"), "unexpected E0101, got {:?}", codes);
}

#[test]
fn own_e0101_not_before_15() {
    let codes = own_codes(r#"fn take(p: move P) do
  print(p.x)
end

fn main() do
  let a = P { x = 15 }
  print(a.x)
  take(a)
  print(1)
end"#);
    assert!(!codes.iter().any(|c| c == "E0101"), "unexpected E0101, got {:?}", codes);
}

#[test]
fn own_e0102_fires_1() {
    let codes = own_codes(r#"fn take(p: move P) do
  print(p.x)
end

fn f(p: &P) do
  take(p)
  print(1)
end"#);
    assert!(codes.iter().any(|c| c == "E0102"), "expected E0102, got {:?}", codes);
}

#[test]
fn own_e0102_fires_2() {
    let codes = own_codes(r#"fn take(p: move P) do
  print(p.x)
end

fn f(p: &P) do
  take(p)
  print(1)
end"#);
    assert!(codes.iter().any(|c| c == "E0102"), "expected E0102, got {:?}", codes);
}

#[test]
fn own_e0102_fires_3() {
    let codes = own_codes(r#"fn take(p: move P) do
  print(p.x)
end

fn f(p: &P) do
  take(p)
  print(1)
end"#);
    assert!(codes.iter().any(|c| c == "E0102"), "expected E0102, got {:?}", codes);
}

#[test]
fn own_e0102_fires_4() {
    let codes = own_codes(r#"fn take(p: move P) do
  print(p.x)
end

fn f(p: &P) do
  take(p)
  print(1)
end"#);
    assert!(codes.iter().any(|c| c == "E0102"), "expected E0102, got {:?}", codes);
}

#[test]
fn own_e0102_fires_5() {
    let codes = own_codes(r#"fn take(p: move P) do
  print(p.x)
end

fn f(p: &P) do
  take(p)
  print(1)
end"#);
    assert!(codes.iter().any(|c| c == "E0102"), "expected E0102, got {:?}", codes);
}

#[test]
fn own_e0102_fires_6() {
    let codes = own_codes(r#"fn take(p: move P) do
  print(p.x)
end

fn f(p: &P) do
  take(p)
  print(1)
end"#);
    assert!(codes.iter().any(|c| c == "E0102"), "expected E0102, got {:?}", codes);
}

#[test]
fn own_e0102_fires_7() {
    let codes = own_codes(r#"fn take(p: move P) do
  print(p.x)
end

fn f(p: &P) do
  take(p)
  print(1)
end"#);
    assert!(codes.iter().any(|c| c == "E0102"), "expected E0102, got {:?}", codes);
}

#[test]
fn own_e0102_fires_8() {
    let codes = own_codes(r#"fn take(p: move P) do
  print(p.x)
end

fn f(p: &P) do
  take(p)
  print(1)
end"#);
    assert!(codes.iter().any(|c| c == "E0102"), "expected E0102, got {:?}", codes);
}

#[test]
fn own_e0102_fires_9() {
    let codes = own_codes(r#"fn take(p: move P) do
  print(p.x)
end

fn f(p: &P) do
  take(p)
  print(1)
end"#);
    assert!(codes.iter().any(|c| c == "E0102"), "expected E0102, got {:?}", codes);
}

#[test]
fn own_e0102_fires_10() {
    let codes = own_codes(r#"fn take(p: move P) do
  print(p.x)
end

fn f(p: &P) do
  take(p)
  print(1)
end"#);
    assert!(codes.iter().any(|c| c == "E0102"), "expected E0102, got {:?}", codes);
}

#[test]
fn own_e0102_fires_11() {
    let codes = own_codes(r#"fn take(p: move P) do
  print(p.x)
end

fn f(p: &P) do
  take(p)
  print(1)
end"#);
    assert!(codes.iter().any(|c| c == "E0102"), "expected E0102, got {:?}", codes);
}

#[test]
fn own_e0102_fires_12() {
    let codes = own_codes(r#"fn take(p: move P) do
  print(p.x)
end

fn f(p: &P) do
  take(p)
  print(1)
end"#);
    assert!(codes.iter().any(|c| c == "E0102"), "expected E0102, got {:?}", codes);
}

#[test]
fn own_e0102_fires_13() {
    let codes = own_codes(r#"fn take(p: move P) do
  print(p.x)
end

fn f(p: &P) do
  take(p)
  print(1)
end"#);
    assert!(codes.iter().any(|c| c == "E0102"), "expected E0102, got {:?}", codes);
}

#[test]
fn own_e0102_fires_14() {
    let codes = own_codes(r#"fn take(p: move P) do
  print(p.x)
end

fn f(p: &P) do
  take(p)
  print(1)
end"#);
    assert!(codes.iter().any(|c| c == "E0102"), "expected E0102, got {:?}", codes);
}

#[test]
fn own_e0102_fires_15() {
    let codes = own_codes(r#"fn take(p: move P) do
  print(p.x)
end

fn f(p: &P) do
  take(p)
  print(1)
end"#);
    assert!(codes.iter().any(|c| c == "E0102"), "expected E0102, got {:?}", codes);
}

#[test]
fn own_e0103_fires_1() {
    let codes = own_codes(r#"fn f() -> &Int do
  let n = 4
  return n
end"#);
    assert!(codes.iter().any(|c| c == "E0103"), "expected E0103, got {:?}", codes);
}

#[test]
fn own_e0103_fires_2() {
    let codes = own_codes(r#"fn f() -> &Int do
  let n = 5
  return n
end"#);
    assert!(codes.iter().any(|c| c == "E0103"), "expected E0103, got {:?}", codes);
}

#[test]
fn own_e0103_fires_3() {
    let codes = own_codes(r#"fn f() -> &Int do
  let n = 6
  return n
end"#);
    assert!(codes.iter().any(|c| c == "E0103"), "expected E0103, got {:?}", codes);
}

#[test]
fn own_e0103_fires_4() {
    let codes = own_codes(r#"fn f() -> &Int do
  let n = 7
  return n
end"#);
    assert!(codes.iter().any(|c| c == "E0103"), "expected E0103, got {:?}", codes);
}

#[test]
fn own_e0103_fires_5() {
    let codes = own_codes(r#"fn f() -> &Int do
  let n = 8
  return n
end"#);
    assert!(codes.iter().any(|c| c == "E0103"), "expected E0103, got {:?}", codes);
}

#[test]
fn own_e0103_fires_6() {
    let codes = own_codes(r#"fn f() -> &Int do
  let n = 9
  return n
end"#);
    assert!(codes.iter().any(|c| c == "E0103"), "expected E0103, got {:?}", codes);
}

#[test]
fn own_e0103_fires_7() {
    let codes = own_codes(r#"fn f() -> &Int do
  let n = 10
  return n
end"#);
    assert!(codes.iter().any(|c| c == "E0103"), "expected E0103, got {:?}", codes);
}

#[test]
fn own_e0103_fires_8() {
    let codes = own_codes(r#"fn f() -> &Int do
  let n = 11
  return n
end"#);
    assert!(codes.iter().any(|c| c == "E0103"), "expected E0103, got {:?}", codes);
}

#[test]
fn own_e0103_fires_9() {
    let codes = own_codes(r#"fn f() -> &Int do
  let n = 12
  return n
end"#);
    assert!(codes.iter().any(|c| c == "E0103"), "expected E0103, got {:?}", codes);
}

#[test]
fn own_e0103_fires_10() {
    let codes = own_codes(r#"fn f() -> &Int do
  let n = 13
  return n
end"#);
    assert!(codes.iter().any(|c| c == "E0103"), "expected E0103, got {:?}", codes);
}

#[test]
fn own_e0103_fires_11() {
    let codes = own_codes(r#"fn f() -> &Int do
  let n = 14
  return n
end"#);
    assert!(codes.iter().any(|c| c == "E0103"), "expected E0103, got {:?}", codes);
}

#[test]
fn own_e0103_fires_12() {
    let codes = own_codes(r#"fn f() -> &Int do
  let n = 15
  return n
end"#);
    assert!(codes.iter().any(|c| c == "E0103"), "expected E0103, got {:?}", codes);
}

#[test]
fn own_e0103_fires_13() {
    let codes = own_codes(r#"fn f() -> &Int do
  let n = 16
  return n
end"#);
    assert!(codes.iter().any(|c| c == "E0103"), "expected E0103, got {:?}", codes);
}

#[test]
fn own_e0103_fires_14() {
    let codes = own_codes(r#"fn f() -> &Int do
  let n = 17
  return n
end"#);
    assert!(codes.iter().any(|c| c == "E0103"), "expected E0103, got {:?}", codes);
}

#[test]
fn own_e0103_fires_15() {
    let codes = own_codes(r#"fn f() -> &Int do
  let n = 18
  return n
end"#);
    assert!(codes.iter().any(|c| c == "E0103"), "expected E0103, got {:?}", codes);
}

#[test]
fn own_e0103_fires_16() {
    let codes = own_codes(r#"fn f() -> &Int do
  let n = 19
  return n
end"#);
    assert!(codes.iter().any(|c| c == "E0103"), "expected E0103, got {:?}", codes);
}

#[test]
fn own_e0103_fires_17() {
    let codes = own_codes(r#"fn f() -> &Int do
  let n = 20
  return n
end"#);
    assert!(codes.iter().any(|c| c == "E0103"), "expected E0103, got {:?}", codes);
}

#[test]
fn own_e0103_fires_18() {
    let codes = own_codes(r#"fn f() -> &Int do
  let n = 21
  return n
end"#);
    assert!(codes.iter().any(|c| c == "E0103"), "expected E0103, got {:?}", codes);
}

#[test]
fn own_e0103_fires_19() {
    let codes = own_codes(r#"fn f() -> &Int do
  let n = 22
  return n
end"#);
    assert!(codes.iter().any(|c| c == "E0103"), "expected E0103, got {:?}", codes);
}

#[test]
fn own_e0103_fires_20() {
    let codes = own_codes(r#"fn f() -> &Int do
  let n = 23
  return n
end"#);
    assert!(codes.iter().any(|c| c == "E0103"), "expected E0103, got {:?}", codes);
}

#[test]
fn own_e0103_value_ok_1() {
    let codes = own_codes(r#"fn f() -> Int do
  let n = 1
  return n
end"#);
    assert!(!codes.iter().any(|c| c == "E0103"), "unexpected E0103, got {:?}", codes);
}

#[test]
fn own_e0103_value_ok_2() {
    let codes = own_codes(r#"fn f() -> Int do
  let n = 2
  return n
end"#);
    assert!(!codes.iter().any(|c| c == "E0103"), "unexpected E0103, got {:?}", codes);
}

#[test]
fn own_e0103_value_ok_3() {
    let codes = own_codes(r#"fn f() -> Int do
  let n = 3
  return n
end"#);
    assert!(!codes.iter().any(|c| c == "E0103"), "unexpected E0103, got {:?}", codes);
}

#[test]
fn own_e0103_value_ok_4() {
    let codes = own_codes(r#"fn f() -> Int do
  let n = 4
  return n
end"#);
    assert!(!codes.iter().any(|c| c == "E0103"), "unexpected E0103, got {:?}", codes);
}

#[test]
fn own_e0103_value_ok_5() {
    let codes = own_codes(r#"fn f() -> Int do
  let n = 5
  return n
end"#);
    assert!(!codes.iter().any(|c| c == "E0103"), "unexpected E0103, got {:?}", codes);
}

#[test]
fn own_e0103_value_ok_6() {
    let codes = own_codes(r#"fn f() -> Int do
  let n = 6
  return n
end"#);
    assert!(!codes.iter().any(|c| c == "E0103"), "unexpected E0103, got {:?}", codes);
}

#[test]
fn own_e0103_value_ok_7() {
    let codes = own_codes(r#"fn f() -> Int do
  let n = 7
  return n
end"#);
    assert!(!codes.iter().any(|c| c == "E0103"), "unexpected E0103, got {:?}", codes);
}

#[test]
fn own_e0103_value_ok_8() {
    let codes = own_codes(r#"fn f() -> Int do
  let n = 8
  return n
end"#);
    assert!(!codes.iter().any(|c| c == "E0103"), "unexpected E0103, got {:?}", codes);
}

#[test]
fn own_e0103_value_ok_9() {
    let codes = own_codes(r#"fn f() -> Int do
  let n = 9
  return n
end"#);
    assert!(!codes.iter().any(|c| c == "E0103"), "unexpected E0103, got {:?}", codes);
}

#[test]
fn own_e0103_value_ok_10() {
    let codes = own_codes(r#"fn f() -> Int do
  let n = 10
  return n
end"#);
    assert!(!codes.iter().any(|c| c == "E0103"), "unexpected E0103, got {:?}", codes);
}

#[test]
fn own_e0104_fires_1() {
    let codes = own_codes(r#"fn f(p: &P) do
  p.x = 1
  print(p.x)
end"#);
    assert!(codes.iter().any(|c| c == "E0104"), "expected E0104, got {:?}", codes);
}

#[test]
fn own_e0104_fires_2() {
    let codes = own_codes(r#"fn f(p: &P) do
  p.x = 2
  print(p.x)
end"#);
    assert!(codes.iter().any(|c| c == "E0104"), "expected E0104, got {:?}", codes);
}

#[test]
fn own_e0104_fires_3() {
    let codes = own_codes(r#"fn f(p: &P) do
  p.x = 3
  print(p.x)
end"#);
    assert!(codes.iter().any(|c| c == "E0104"), "expected E0104, got {:?}", codes);
}

#[test]
fn own_e0104_fires_4() {
    let codes = own_codes(r#"fn f(p: &P) do
  p.x = 4
  print(p.x)
end"#);
    assert!(codes.iter().any(|c| c == "E0104"), "expected E0104, got {:?}", codes);
}

#[test]
fn own_e0104_fires_5() {
    let codes = own_codes(r#"fn f(p: &P) do
  p.x = 5
  print(p.x)
end"#);
    assert!(codes.iter().any(|c| c == "E0104"), "expected E0104, got {:?}", codes);
}

#[test]
fn own_e0104_fires_6() {
    let codes = own_codes(r#"fn f(p: &P) do
  p.x = 6
  print(p.x)
end"#);
    assert!(codes.iter().any(|c| c == "E0104"), "expected E0104, got {:?}", codes);
}

#[test]
fn own_e0104_fires_7() {
    let codes = own_codes(r#"fn f(p: &P) do
  p.x = 7
  print(p.x)
end"#);
    assert!(codes.iter().any(|c| c == "E0104"), "expected E0104, got {:?}", codes);
}

#[test]
fn own_e0104_fires_8() {
    let codes = own_codes(r#"fn f(p: &P) do
  p.x = 8
  print(p.x)
end"#);
    assert!(codes.iter().any(|c| c == "E0104"), "expected E0104, got {:?}", codes);
}

#[test]
fn own_e0104_fires_9() {
    let codes = own_codes(r#"fn f(p: &P) do
  p.x = 9
  print(p.x)
end"#);
    assert!(codes.iter().any(|c| c == "E0104"), "expected E0104, got {:?}", codes);
}

#[test]
fn own_e0104_fires_10() {
    let codes = own_codes(r#"fn f(p: &P) do
  p.x = 10
  print(p.x)
end"#);
    assert!(codes.iter().any(|c| c == "E0104"), "expected E0104, got {:?}", codes);
}

#[test]
fn own_e0104_fires_11() {
    let codes = own_codes(r#"fn f(p: &P) do
  p.x = 11
  print(p.x)
end"#);
    assert!(codes.iter().any(|c| c == "E0104"), "expected E0104, got {:?}", codes);
}

#[test]
fn own_e0104_fires_12() {
    let codes = own_codes(r#"fn f(p: &P) do
  p.x = 12
  print(p.x)
end"#);
    assert!(codes.iter().any(|c| c == "E0104"), "expected E0104, got {:?}", codes);
}

#[test]
fn own_e0104_fires_13() {
    let codes = own_codes(r#"fn f(p: &P) do
  p.x = 13
  print(p.x)
end"#);
    assert!(codes.iter().any(|c| c == "E0104"), "expected E0104, got {:?}", codes);
}

#[test]
fn own_e0104_fires_14() {
    let codes = own_codes(r#"fn f(p: &P) do
  p.x = 14
  print(p.x)
end"#);
    assert!(codes.iter().any(|c| c == "E0104"), "expected E0104, got {:?}", codes);
}

#[test]
fn own_e0104_fires_15() {
    let codes = own_codes(r#"fn f(p: &P) do
  p.x = 15
  print(p.x)
end"#);
    assert!(codes.iter().any(|c| c == "E0104"), "expected E0104, got {:?}", codes);
}

#[test]
fn own_e0104_fires_16() {
    let codes = own_codes(r#"fn f(p: &P) do
  p.x = 16
  print(p.x)
end"#);
    assert!(codes.iter().any(|c| c == "E0104"), "expected E0104, got {:?}", codes);
}

#[test]
fn own_e0104_fires_17() {
    let codes = own_codes(r#"fn f(p: &P) do
  p.x = 17
  print(p.x)
end"#);
    assert!(codes.iter().any(|c| c == "E0104"), "expected E0104, got {:?}", codes);
}

#[test]
fn own_e0104_fires_18() {
    let codes = own_codes(r#"fn f(p: &P) do
  p.x = 18
  print(p.x)
end"#);
    assert!(codes.iter().any(|c| c == "E0104"), "expected E0104, got {:?}", codes);
}

#[test]
fn own_e0104_fires_19() {
    let codes = own_codes(r#"fn f(p: &P) do
  p.x = 19
  print(p.x)
end"#);
    assert!(codes.iter().any(|c| c == "E0104"), "expected E0104, got {:?}", codes);
}

#[test]
fn own_e0104_fires_20() {
    let codes = own_codes(r#"fn f(p: &P) do
  p.x = 20
  print(p.x)
end"#);
    assert!(codes.iter().any(|c| c == "E0104"), "expected E0104, got {:?}", codes);
}

#[test]
fn own_e0104_mut_ok_1() {
    let codes = own_codes(r#"fn f(p: &var P) do
  p.x = 1
  print(p.x)
end"#);
    assert!(!codes.iter().any(|c| c == "E0104"), "unexpected E0104, got {:?}", codes);
}

#[test]
fn own_e0104_mut_ok_2() {
    let codes = own_codes(r#"fn f(p: &var P) do
  p.x = 2
  print(p.x)
end"#);
    assert!(!codes.iter().any(|c| c == "E0104"), "unexpected E0104, got {:?}", codes);
}

#[test]
fn own_e0104_mut_ok_3() {
    let codes = own_codes(r#"fn f(p: &var P) do
  p.x = 3
  print(p.x)
end"#);
    assert!(!codes.iter().any(|c| c == "E0104"), "unexpected E0104, got {:?}", codes);
}

#[test]
fn own_e0104_mut_ok_4() {
    let codes = own_codes(r#"fn f(p: &var P) do
  p.x = 4
  print(p.x)
end"#);
    assert!(!codes.iter().any(|c| c == "E0104"), "unexpected E0104, got {:?}", codes);
}

#[test]
fn own_e0104_mut_ok_5() {
    let codes = own_codes(r#"fn f(p: &var P) do
  p.x = 5
  print(p.x)
end"#);
    assert!(!codes.iter().any(|c| c == "E0104"), "unexpected E0104, got {:?}", codes);
}

#[test]
fn own_e0104_mut_ok_6() {
    let codes = own_codes(r#"fn f(p: &var P) do
  p.x = 6
  print(p.x)
end"#);
    assert!(!codes.iter().any(|c| c == "E0104"), "unexpected E0104, got {:?}", codes);
}

#[test]
fn own_e0104_mut_ok_7() {
    let codes = own_codes(r#"fn f(p: &var P) do
  p.x = 7
  print(p.x)
end"#);
    assert!(!codes.iter().any(|c| c == "E0104"), "unexpected E0104, got {:?}", codes);
}

#[test]
fn own_e0104_mut_ok_8() {
    let codes = own_codes(r#"fn f(p: &var P) do
  p.x = 8
  print(p.x)
end"#);
    assert!(!codes.iter().any(|c| c == "E0104"), "unexpected E0104, got {:?}", codes);
}

#[test]
fn own_e0104_mut_ok_9() {
    let codes = own_codes(r#"fn f(p: &var P) do
  p.x = 9
  print(p.x)
end"#);
    assert!(!codes.iter().any(|c| c == "E0104"), "unexpected E0104, got {:?}", codes);
}

#[test]
fn own_e0104_mut_ok_10() {
    let codes = own_codes(r#"fn f(p: &var P) do
  p.x = 10
  print(p.x)
end"#);
    assert!(!codes.iter().any(|c| c == "E0104"), "unexpected E0104, got {:?}", codes);
}

#[test]
fn own_modes_1() {
    let src = r#"fn reader(p: P) do
  print(p.x)
end

fn writer(p: P) do
  p.x = 1
  print(p.x)
end

fn mover(p: move P) do
  print(p.x)
end

fn valuer(n: Int) do
  print(n)
end

fn valuewriter(n: Int) do
  n = 1
  print(n)
end"#;
    assert_eq!(own_mode(src, "reader", "p"), Some(ParamMode::Borrow));
    assert_eq!(own_mode(src, "writer", "p"), Some(ParamMode::BorrowMut));
    assert_eq!(own_mode(src, "mover", "p"), Some(ParamMode::Move));
    assert_eq!(own_mode(src, "valuer", "n"), Some(ParamMode::Value));
    assert_eq!(own_mode(src, "valuewriter", "n"), Some(ParamMode::Value));
    assert_eq!(own_mode(src, "reader", "n"), None);
}

#[test]
fn own_modes_2() {
    let src = r#"fn reader(p: P) do
  print(p.x)
end

fn writer(p: P) do
  p.x = 2
  print(p.x)
end

fn mover(p: move P) do
  print(p.x)
end

fn valuer(n: Int) do
  print(n)
end

fn valuewriter(n: Int) do
  n = 2
  print(n)
end"#;
    assert_eq!(own_mode(src, "reader", "p"), Some(ParamMode::Borrow));
    assert_eq!(own_mode(src, "writer", "p"), Some(ParamMode::BorrowMut));
    assert_eq!(own_mode(src, "mover", "p"), Some(ParamMode::Move));
    assert_eq!(own_mode(src, "valuer", "n"), Some(ParamMode::Value));
    assert_eq!(own_mode(src, "valuewriter", "n"), Some(ParamMode::Value));
    assert_eq!(own_mode(src, "reader", "n"), None);
}

#[test]
fn own_modes_3() {
    let src = r#"fn reader(p: P) do
  print(p.x)
end

fn writer(p: P) do
  p.x = 3
  print(p.x)
end

fn mover(p: move P) do
  print(p.x)
end

fn valuer(n: Int) do
  print(n)
end

fn valuewriter(n: Int) do
  n = 3
  print(n)
end"#;
    assert_eq!(own_mode(src, "reader", "p"), Some(ParamMode::Borrow));
    assert_eq!(own_mode(src, "writer", "p"), Some(ParamMode::BorrowMut));
    assert_eq!(own_mode(src, "mover", "p"), Some(ParamMode::Move));
    assert_eq!(own_mode(src, "valuer", "n"), Some(ParamMode::Value));
    assert_eq!(own_mode(src, "valuewriter", "n"), Some(ParamMode::Value));
    assert_eq!(own_mode(src, "reader", "n"), None);
}

#[test]
fn own_modes_4() {
    let src = r#"fn reader(p: P) do
  print(p.x)
end

fn writer(p: P) do
  p.x = 4
  print(p.x)
end

fn mover(p: move P) do
  print(p.x)
end

fn valuer(n: Int) do
  print(n)
end

fn valuewriter(n: Int) do
  n = 4
  print(n)
end"#;
    assert_eq!(own_mode(src, "reader", "p"), Some(ParamMode::Borrow));
    assert_eq!(own_mode(src, "writer", "p"), Some(ParamMode::BorrowMut));
    assert_eq!(own_mode(src, "mover", "p"), Some(ParamMode::Move));
    assert_eq!(own_mode(src, "valuer", "n"), Some(ParamMode::Value));
    assert_eq!(own_mode(src, "valuewriter", "n"), Some(ParamMode::Value));
    assert_eq!(own_mode(src, "reader", "n"), None);
}

#[test]
fn own_modes_5() {
    let src = r#"fn reader(p: P) do
  print(p.x)
end

fn writer(p: P) do
  p.x = 5
  print(p.x)
end

fn mover(p: move P) do
  print(p.x)
end

fn valuer(n: Int) do
  print(n)
end

fn valuewriter(n: Int) do
  n = 5
  print(n)
end"#;
    assert_eq!(own_mode(src, "reader", "p"), Some(ParamMode::Borrow));
    assert_eq!(own_mode(src, "writer", "p"), Some(ParamMode::BorrowMut));
    assert_eq!(own_mode(src, "mover", "p"), Some(ParamMode::Move));
    assert_eq!(own_mode(src, "valuer", "n"), Some(ParamMode::Value));
    assert_eq!(own_mode(src, "valuewriter", "n"), Some(ParamMode::Value));
    assert_eq!(own_mode(src, "reader", "n"), None);
}

#[test]
fn own_modes_6() {
    let src = r#"fn reader(p: P) do
  print(p.x)
end

fn writer(p: P) do
  p.x = 6
  print(p.x)
end

fn mover(p: move P) do
  print(p.x)
end

fn valuer(n: Int) do
  print(n)
end

fn valuewriter(n: Int) do
  n = 6
  print(n)
end"#;
    assert_eq!(own_mode(src, "reader", "p"), Some(ParamMode::Borrow));
    assert_eq!(own_mode(src, "writer", "p"), Some(ParamMode::BorrowMut));
    assert_eq!(own_mode(src, "mover", "p"), Some(ParamMode::Move));
    assert_eq!(own_mode(src, "valuer", "n"), Some(ParamMode::Value));
    assert_eq!(own_mode(src, "valuewriter", "n"), Some(ParamMode::Value));
    assert_eq!(own_mode(src, "reader", "n"), None);
}

#[test]
fn own_modes_7() {
    let src = r#"fn reader(p: P) do
  print(p.x)
end

fn writer(p: P) do
  p.x = 7
  print(p.x)
end

fn mover(p: move P) do
  print(p.x)
end

fn valuer(n: Int) do
  print(n)
end

fn valuewriter(n: Int) do
  n = 7
  print(n)
end"#;
    assert_eq!(own_mode(src, "reader", "p"), Some(ParamMode::Borrow));
    assert_eq!(own_mode(src, "writer", "p"), Some(ParamMode::BorrowMut));
    assert_eq!(own_mode(src, "mover", "p"), Some(ParamMode::Move));
    assert_eq!(own_mode(src, "valuer", "n"), Some(ParamMode::Value));
    assert_eq!(own_mode(src, "valuewriter", "n"), Some(ParamMode::Value));
    assert_eq!(own_mode(src, "reader", "n"), None);
}

#[test]
fn own_modes_8() {
    let src = r#"fn reader(p: P) do
  print(p.x)
end

fn writer(p: P) do
  p.x = 8
  print(p.x)
end

fn mover(p: move P) do
  print(p.x)
end

fn valuer(n: Int) do
  print(n)
end

fn valuewriter(n: Int) do
  n = 8
  print(n)
end"#;
    assert_eq!(own_mode(src, "reader", "p"), Some(ParamMode::Borrow));
    assert_eq!(own_mode(src, "writer", "p"), Some(ParamMode::BorrowMut));
    assert_eq!(own_mode(src, "mover", "p"), Some(ParamMode::Move));
    assert_eq!(own_mode(src, "valuer", "n"), Some(ParamMode::Value));
    assert_eq!(own_mode(src, "valuewriter", "n"), Some(ParamMode::Value));
    assert_eq!(own_mode(src, "reader", "n"), None);
}

#[test]
fn own_modes_9() {
    let src = r#"fn reader(p: P) do
  print(p.x)
end

fn writer(p: P) do
  p.x = 9
  print(p.x)
end

fn mover(p: move P) do
  print(p.x)
end

fn valuer(n: Int) do
  print(n)
end

fn valuewriter(n: Int) do
  n = 9
  print(n)
end"#;
    assert_eq!(own_mode(src, "reader", "p"), Some(ParamMode::Borrow));
    assert_eq!(own_mode(src, "writer", "p"), Some(ParamMode::BorrowMut));
    assert_eq!(own_mode(src, "mover", "p"), Some(ParamMode::Move));
    assert_eq!(own_mode(src, "valuer", "n"), Some(ParamMode::Value));
    assert_eq!(own_mode(src, "valuewriter", "n"), Some(ParamMode::Value));
    assert_eq!(own_mode(src, "reader", "n"), None);
}

#[test]
fn own_modes_10() {
    let src = r#"fn reader(p: P) do
  print(p.x)
end

fn writer(p: P) do
  p.x = 10
  print(p.x)
end

fn mover(p: move P) do
  print(p.x)
end

fn valuer(n: Int) do
  print(n)
end

fn valuewriter(n: Int) do
  n = 10
  print(n)
end"#;
    assert_eq!(own_mode(src, "reader", "p"), Some(ParamMode::Borrow));
    assert_eq!(own_mode(src, "writer", "p"), Some(ParamMode::BorrowMut));
    assert_eq!(own_mode(src, "mover", "p"), Some(ParamMode::Move));
    assert_eq!(own_mode(src, "valuer", "n"), Some(ParamMode::Value));
    assert_eq!(own_mode(src, "valuewriter", "n"), Some(ParamMode::Value));
    assert_eq!(own_mode(src, "reader", "n"), None);
}

#[test]
fn own_modes_11() {
    let src = r#"fn reader(p: P) do
  print(p.x)
end

fn writer(p: P) do
  p.x = 11
  print(p.x)
end

fn mover(p: move P) do
  print(p.x)
end

fn valuer(n: Int) do
  print(n)
end

fn valuewriter(n: Int) do
  n = 11
  print(n)
end"#;
    assert_eq!(own_mode(src, "reader", "p"), Some(ParamMode::Borrow));
    assert_eq!(own_mode(src, "writer", "p"), Some(ParamMode::BorrowMut));
    assert_eq!(own_mode(src, "mover", "p"), Some(ParamMode::Move));
    assert_eq!(own_mode(src, "valuer", "n"), Some(ParamMode::Value));
    assert_eq!(own_mode(src, "valuewriter", "n"), Some(ParamMode::Value));
    assert_eq!(own_mode(src, "reader", "n"), None);
}

#[test]
fn own_modes_12() {
    let src = r#"fn reader(p: P) do
  print(p.x)
end

fn writer(p: P) do
  p.x = 12
  print(p.x)
end

fn mover(p: move P) do
  print(p.x)
end

fn valuer(n: Int) do
  print(n)
end

fn valuewriter(n: Int) do
  n = 12
  print(n)
end"#;
    assert_eq!(own_mode(src, "reader", "p"), Some(ParamMode::Borrow));
    assert_eq!(own_mode(src, "writer", "p"), Some(ParamMode::BorrowMut));
    assert_eq!(own_mode(src, "mover", "p"), Some(ParamMode::Move));
    assert_eq!(own_mode(src, "valuer", "n"), Some(ParamMode::Value));
    assert_eq!(own_mode(src, "valuewriter", "n"), Some(ParamMode::Value));
    assert_eq!(own_mode(src, "reader", "n"), None);
}

#[test]
fn own_modes_13() {
    let src = r#"fn reader(p: P) do
  print(p.x)
end

fn writer(p: P) do
  p.x = 13
  print(p.x)
end

fn mover(p: move P) do
  print(p.x)
end

fn valuer(n: Int) do
  print(n)
end

fn valuewriter(n: Int) do
  n = 13
  print(n)
end"#;
    assert_eq!(own_mode(src, "reader", "p"), Some(ParamMode::Borrow));
    assert_eq!(own_mode(src, "writer", "p"), Some(ParamMode::BorrowMut));
    assert_eq!(own_mode(src, "mover", "p"), Some(ParamMode::Move));
    assert_eq!(own_mode(src, "valuer", "n"), Some(ParamMode::Value));
    assert_eq!(own_mode(src, "valuewriter", "n"), Some(ParamMode::Value));
    assert_eq!(own_mode(src, "reader", "n"), None);
}

#[test]
fn own_modes_14() {
    let src = r#"fn reader(p: P) do
  print(p.x)
end

fn writer(p: P) do
  p.x = 14
  print(p.x)
end

fn mover(p: move P) do
  print(p.x)
end

fn valuer(n: Int) do
  print(n)
end

fn valuewriter(n: Int) do
  n = 14
  print(n)
end"#;
    assert_eq!(own_mode(src, "reader", "p"), Some(ParamMode::Borrow));
    assert_eq!(own_mode(src, "writer", "p"), Some(ParamMode::BorrowMut));
    assert_eq!(own_mode(src, "mover", "p"), Some(ParamMode::Move));
    assert_eq!(own_mode(src, "valuer", "n"), Some(ParamMode::Value));
    assert_eq!(own_mode(src, "valuewriter", "n"), Some(ParamMode::Value));
    assert_eq!(own_mode(src, "reader", "n"), None);
}

#[test]
fn own_modes_15() {
    let src = r#"fn reader(p: P) do
  print(p.x)
end

fn writer(p: P) do
  p.x = 15
  print(p.x)
end

fn mover(p: move P) do
  print(p.x)
end

fn valuer(n: Int) do
  print(n)
end

fn valuewriter(n: Int) do
  n = 15
  print(n)
end"#;
    assert_eq!(own_mode(src, "reader", "p"), Some(ParamMode::Borrow));
    assert_eq!(own_mode(src, "writer", "p"), Some(ParamMode::BorrowMut));
    assert_eq!(own_mode(src, "mover", "p"), Some(ParamMode::Move));
    assert_eq!(own_mode(src, "valuer", "n"), Some(ParamMode::Value));
    assert_eq!(own_mode(src, "valuewriter", "n"), Some(ParamMode::Value));
    assert_eq!(own_mode(src, "reader", "n"), None);
}

#[test]
fn own_modes_16() {
    let src = r#"fn reader(p: P) do
  print(p.x)
end

fn writer(p: P) do
  p.x = 16
  print(p.x)
end

fn mover(p: move P) do
  print(p.x)
end

fn valuer(n: Int) do
  print(n)
end

fn valuewriter(n: Int) do
  n = 16
  print(n)
end"#;
    assert_eq!(own_mode(src, "reader", "p"), Some(ParamMode::Borrow));
    assert_eq!(own_mode(src, "writer", "p"), Some(ParamMode::BorrowMut));
    assert_eq!(own_mode(src, "mover", "p"), Some(ParamMode::Move));
    assert_eq!(own_mode(src, "valuer", "n"), Some(ParamMode::Value));
    assert_eq!(own_mode(src, "valuewriter", "n"), Some(ParamMode::Value));
    assert_eq!(own_mode(src, "reader", "n"), None);
}

#[test]
fn own_modes_17() {
    let src = r#"fn reader(p: P) do
  print(p.x)
end

fn writer(p: P) do
  p.x = 17
  print(p.x)
end

fn mover(p: move P) do
  print(p.x)
end

fn valuer(n: Int) do
  print(n)
end

fn valuewriter(n: Int) do
  n = 17
  print(n)
end"#;
    assert_eq!(own_mode(src, "reader", "p"), Some(ParamMode::Borrow));
    assert_eq!(own_mode(src, "writer", "p"), Some(ParamMode::BorrowMut));
    assert_eq!(own_mode(src, "mover", "p"), Some(ParamMode::Move));
    assert_eq!(own_mode(src, "valuer", "n"), Some(ParamMode::Value));
    assert_eq!(own_mode(src, "valuewriter", "n"), Some(ParamMode::Value));
    assert_eq!(own_mode(src, "reader", "n"), None);
}

#[test]
fn own_modes_18() {
    let src = r#"fn reader(p: P) do
  print(p.x)
end

fn writer(p: P) do
  p.x = 18
  print(p.x)
end

fn mover(p: move P) do
  print(p.x)
end

fn valuer(n: Int) do
  print(n)
end

fn valuewriter(n: Int) do
  n = 18
  print(n)
end"#;
    assert_eq!(own_mode(src, "reader", "p"), Some(ParamMode::Borrow));
    assert_eq!(own_mode(src, "writer", "p"), Some(ParamMode::BorrowMut));
    assert_eq!(own_mode(src, "mover", "p"), Some(ParamMode::Move));
    assert_eq!(own_mode(src, "valuer", "n"), Some(ParamMode::Value));
    assert_eq!(own_mode(src, "valuewriter", "n"), Some(ParamMode::Value));
    assert_eq!(own_mode(src, "reader", "n"), None);
}

#[test]
fn own_modes_19() {
    let src = r#"fn reader(p: P) do
  print(p.x)
end

fn writer(p: P) do
  p.x = 19
  print(p.x)
end

fn mover(p: move P) do
  print(p.x)
end

fn valuer(n: Int) do
  print(n)
end

fn valuewriter(n: Int) do
  n = 19
  print(n)
end"#;
    assert_eq!(own_mode(src, "reader", "p"), Some(ParamMode::Borrow));
    assert_eq!(own_mode(src, "writer", "p"), Some(ParamMode::BorrowMut));
    assert_eq!(own_mode(src, "mover", "p"), Some(ParamMode::Move));
    assert_eq!(own_mode(src, "valuer", "n"), Some(ParamMode::Value));
    assert_eq!(own_mode(src, "valuewriter", "n"), Some(ParamMode::Value));
    assert_eq!(own_mode(src, "reader", "n"), None);
}

#[test]
fn own_modes_20() {
    let src = r#"fn reader(p: P) do
  print(p.x)
end

fn writer(p: P) do
  p.x = 20
  print(p.x)
end

fn mover(p: move P) do
  print(p.x)
end

fn valuer(n: Int) do
  print(n)
end

fn valuewriter(n: Int) do
  n = 20
  print(n)
end"#;
    assert_eq!(own_mode(src, "reader", "p"), Some(ParamMode::Borrow));
    assert_eq!(own_mode(src, "writer", "p"), Some(ParamMode::BorrowMut));
    assert_eq!(own_mode(src, "mover", "p"), Some(ParamMode::Move));
    assert_eq!(own_mode(src, "valuer", "n"), Some(ParamMode::Value));
    assert_eq!(own_mode(src, "valuewriter", "n"), Some(ParamMode::Value));
    assert_eq!(own_mode(src, "reader", "n"), None);
}

#[test]
fn own_modes_borrow_1() {
    let src = r#"fn ro(p: &P) do
  print(p.x)
end

fn rw(p: &var P) do
  p.x = 1
end"#;
    assert_eq!(own_mode(src, "ro", "p"), Some(ParamMode::Borrow));
    assert_eq!(own_mode(src, "rw", "p"), Some(ParamMode::BorrowMut));
}

#[test]
fn own_modes_borrow_2() {
    let src = r#"fn ro(p: &P) do
  print(p.x)
end

fn rw(p: &var P) do
  p.x = 2
end"#;
    assert_eq!(own_mode(src, "ro", "p"), Some(ParamMode::Borrow));
    assert_eq!(own_mode(src, "rw", "p"), Some(ParamMode::BorrowMut));
}

#[test]
fn own_modes_borrow_3() {
    let src = r#"fn ro(p: &P) do
  print(p.x)
end

fn rw(p: &var P) do
  p.x = 3
end"#;
    assert_eq!(own_mode(src, "ro", "p"), Some(ParamMode::Borrow));
    assert_eq!(own_mode(src, "rw", "p"), Some(ParamMode::BorrowMut));
}

#[test]
fn own_modes_borrow_4() {
    let src = r#"fn ro(p: &P) do
  print(p.x)
end

fn rw(p: &var P) do
  p.x = 4
end"#;
    assert_eq!(own_mode(src, "ro", "p"), Some(ParamMode::Borrow));
    assert_eq!(own_mode(src, "rw", "p"), Some(ParamMode::BorrowMut));
}

#[test]
fn own_modes_borrow_5() {
    let src = r#"fn ro(p: &P) do
  print(p.x)
end

fn rw(p: &var P) do
  p.x = 5
end"#;
    assert_eq!(own_mode(src, "ro", "p"), Some(ParamMode::Borrow));
    assert_eq!(own_mode(src, "rw", "p"), Some(ParamMode::BorrowMut));
}

#[test]
fn own_modes_borrow_6() {
    let src = r#"fn ro(p: &P) do
  print(p.x)
end

fn rw(p: &var P) do
  p.x = 6
end"#;
    assert_eq!(own_mode(src, "ro", "p"), Some(ParamMode::Borrow));
    assert_eq!(own_mode(src, "rw", "p"), Some(ParamMode::BorrowMut));
}

#[test]
fn own_modes_borrow_7() {
    let src = r#"fn ro(p: &P) do
  print(p.x)
end

fn rw(p: &var P) do
  p.x = 7
end"#;
    assert_eq!(own_mode(src, "ro", "p"), Some(ParamMode::Borrow));
    assert_eq!(own_mode(src, "rw", "p"), Some(ParamMode::BorrowMut));
}

#[test]
fn own_modes_borrow_8() {
    let src = r#"fn ro(p: &P) do
  print(p.x)
end

fn rw(p: &var P) do
  p.x = 8
end"#;
    assert_eq!(own_mode(src, "ro", "p"), Some(ParamMode::Borrow));
    assert_eq!(own_mode(src, "rw", "p"), Some(ParamMode::BorrowMut));
}

#[test]
fn own_modes_borrow_9() {
    let src = r#"fn ro(p: &P) do
  print(p.x)
end

fn rw(p: &var P) do
  p.x = 9
end"#;
    assert_eq!(own_mode(src, "ro", "p"), Some(ParamMode::Borrow));
    assert_eq!(own_mode(src, "rw", "p"), Some(ParamMode::BorrowMut));
}

#[test]
fn own_modes_borrow_10() {
    let src = r#"fn ro(p: &P) do
  print(p.x)
end

fn rw(p: &var P) do
  p.x = 10
end"#;
    assert_eq!(own_mode(src, "ro", "p"), Some(ParamMode::Borrow));
    assert_eq!(own_mode(src, "rw", "p"), Some(ParamMode::BorrowMut));
}

#[test]
fn own_hints_1() {
    let src = r#"fn mover(p: move P) do
  print(p.x)
end

fn writer(p: P) do
  p.x = 1
end"#;
    assert_eq!(own_hints(src, "mover"), vec![("p".to_string(), ParamMode::Borrow)]);
    assert_eq!(own_hints(src, "writer"), vec![("p".to_string(), ParamMode::BorrowMut)]);
    assert_eq!(own_mode(src, "mover", "p"), Some(ParamMode::Move));
}

#[test]
fn own_hints_2() {
    let src = r#"fn mover(p: move P) do
  print(p.x)
end

fn writer(p: P) do
  p.x = 2
end"#;
    assert_eq!(own_hints(src, "mover"), vec![("p".to_string(), ParamMode::Borrow)]);
    assert_eq!(own_hints(src, "writer"), vec![("p".to_string(), ParamMode::BorrowMut)]);
    assert_eq!(own_mode(src, "mover", "p"), Some(ParamMode::Move));
}

#[test]
fn own_hints_3() {
    let src = r#"fn mover(p: move P) do
  print(p.x)
end

fn writer(p: P) do
  p.x = 3
end"#;
    assert_eq!(own_hints(src, "mover"), vec![("p".to_string(), ParamMode::Borrow)]);
    assert_eq!(own_hints(src, "writer"), vec![("p".to_string(), ParamMode::BorrowMut)]);
    assert_eq!(own_mode(src, "mover", "p"), Some(ParamMode::Move));
}

#[test]
fn own_hints_4() {
    let src = r#"fn mover(p: move P) do
  print(p.x)
end

fn writer(p: P) do
  p.x = 4
end"#;
    assert_eq!(own_hints(src, "mover"), vec![("p".to_string(), ParamMode::Borrow)]);
    assert_eq!(own_hints(src, "writer"), vec![("p".to_string(), ParamMode::BorrowMut)]);
    assert_eq!(own_mode(src, "mover", "p"), Some(ParamMode::Move));
}

#[test]
fn own_hints_5() {
    let src = r#"fn mover(p: move P) do
  print(p.x)
end

fn writer(p: P) do
  p.x = 5
end"#;
    assert_eq!(own_hints(src, "mover"), vec![("p".to_string(), ParamMode::Borrow)]);
    assert_eq!(own_hints(src, "writer"), vec![("p".to_string(), ParamMode::BorrowMut)]);
    assert_eq!(own_mode(src, "mover", "p"), Some(ParamMode::Move));
}

#[test]
fn own_hints_6() {
    let src = r#"fn mover(p: move P) do
  print(p.x)
end

fn writer(p: P) do
  p.x = 6
end"#;
    assert_eq!(own_hints(src, "mover"), vec![("p".to_string(), ParamMode::Borrow)]);
    assert_eq!(own_hints(src, "writer"), vec![("p".to_string(), ParamMode::BorrowMut)]);
    assert_eq!(own_mode(src, "mover", "p"), Some(ParamMode::Move));
}

#[test]
fn own_hints_7() {
    let src = r#"fn mover(p: move P) do
  print(p.x)
end

fn writer(p: P) do
  p.x = 7
end"#;
    assert_eq!(own_hints(src, "mover"), vec![("p".to_string(), ParamMode::Borrow)]);
    assert_eq!(own_hints(src, "writer"), vec![("p".to_string(), ParamMode::BorrowMut)]);
    assert_eq!(own_mode(src, "mover", "p"), Some(ParamMode::Move));
}

#[test]
fn own_hints_8() {
    let src = r#"fn mover(p: move P) do
  print(p.x)
end

fn writer(p: P) do
  p.x = 8
end"#;
    assert_eq!(own_hints(src, "mover"), vec![("p".to_string(), ParamMode::Borrow)]);
    assert_eq!(own_hints(src, "writer"), vec![("p".to_string(), ParamMode::BorrowMut)]);
    assert_eq!(own_mode(src, "mover", "p"), Some(ParamMode::Move));
}

#[test]
fn own_hints_9() {
    let src = r#"fn mover(p: move P) do
  print(p.x)
end

fn writer(p: P) do
  p.x = 9
end"#;
    assert_eq!(own_hints(src, "mover"), vec![("p".to_string(), ParamMode::Borrow)]);
    assert_eq!(own_hints(src, "writer"), vec![("p".to_string(), ParamMode::BorrowMut)]);
    assert_eq!(own_mode(src, "mover", "p"), Some(ParamMode::Move));
}

#[test]
fn own_hints_10() {
    let src = r#"fn mover(p: move P) do
  print(p.x)
end

fn writer(p: P) do
  p.x = 10
end"#;
    assert_eq!(own_hints(src, "mover"), vec![("p".to_string(), ParamMode::Borrow)]);
    assert_eq!(own_hints(src, "writer"), vec![("p".to_string(), ParamMode::BorrowMut)]);
    assert_eq!(own_mode(src, "mover", "p"), Some(ParamMode::Move));
}

#[test]
fn own_modes_for_1() {
    let src = r#"fn three(a: Int, b: P, c: move P) do
  print(a)
  print(b.x)
  print(c.x)
end"#;
    let modes = own_modes(src, "three");
    assert_eq!(modes.len(), 3);
    assert_eq!(modes[0], ("a".to_string(), ParamMode::Value));
    assert_eq!(modes[1], ("b".to_string(), ParamMode::Borrow));
    assert_eq!(modes[2], ("c".to_string(), ParamMode::Move));
    assert!(own_modes(src, "missing").is_empty());
}

#[test]
fn own_modes_for_2() {
    let src = r#"fn three(a: Int, b: P, c: move P) do
  print(a)
  print(b.x)
  print(c.x)
end"#;
    let modes = own_modes(src, "three");
    assert_eq!(modes.len(), 3);
    assert_eq!(modes[0], ("a".to_string(), ParamMode::Value));
    assert_eq!(modes[1], ("b".to_string(), ParamMode::Borrow));
    assert_eq!(modes[2], ("c".to_string(), ParamMode::Move));
    assert!(own_modes(src, "missing").is_empty());
}

#[test]
fn own_modes_for_3() {
    let src = r#"fn three(a: Int, b: P, c: move P) do
  print(a)
  print(b.x)
  print(c.x)
end"#;
    let modes = own_modes(src, "three");
    assert_eq!(modes.len(), 3);
    assert_eq!(modes[0], ("a".to_string(), ParamMode::Value));
    assert_eq!(modes[1], ("b".to_string(), ParamMode::Borrow));
    assert_eq!(modes[2], ("c".to_string(), ParamMode::Move));
    assert!(own_modes(src, "missing").is_empty());
}

#[test]
fn own_modes_for_4() {
    let src = r#"fn three(a: Int, b: P, c: move P) do
  print(a)
  print(b.x)
  print(c.x)
end"#;
    let modes = own_modes(src, "three");
    assert_eq!(modes.len(), 3);
    assert_eq!(modes[0], ("a".to_string(), ParamMode::Value));
    assert_eq!(modes[1], ("b".to_string(), ParamMode::Borrow));
    assert_eq!(modes[2], ("c".to_string(), ParamMode::Move));
    assert!(own_modes(src, "missing").is_empty());
}

#[test]
fn own_modes_for_5() {
    let src = r#"fn three(a: Int, b: P, c: move P) do
  print(a)
  print(b.x)
  print(c.x)
end"#;
    let modes = own_modes(src, "three");
    assert_eq!(modes.len(), 3);
    assert_eq!(modes[0], ("a".to_string(), ParamMode::Value));
    assert_eq!(modes[1], ("b".to_string(), ParamMode::Borrow));
    assert_eq!(modes[2], ("c".to_string(), ParamMode::Move));
    assert!(own_modes(src, "missing").is_empty());
}

#[test]
fn own_modes_for_6() {
    let src = r#"fn three(a: Int, b: P, c: move P) do
  print(a)
  print(b.x)
  print(c.x)
end"#;
    let modes = own_modes(src, "three");
    assert_eq!(modes.len(), 3);
    assert_eq!(modes[0], ("a".to_string(), ParamMode::Value));
    assert_eq!(modes[1], ("b".to_string(), ParamMode::Borrow));
    assert_eq!(modes[2], ("c".to_string(), ParamMode::Move));
    assert!(own_modes(src, "missing").is_empty());
}

#[test]
fn own_modes_for_7() {
    let src = r#"fn three(a: Int, b: P, c: move P) do
  print(a)
  print(b.x)
  print(c.x)
end"#;
    let modes = own_modes(src, "three");
    assert_eq!(modes.len(), 3);
    assert_eq!(modes[0], ("a".to_string(), ParamMode::Value));
    assert_eq!(modes[1], ("b".to_string(), ParamMode::Borrow));
    assert_eq!(modes[2], ("c".to_string(), ParamMode::Move));
    assert!(own_modes(src, "missing").is_empty());
}

#[test]
fn own_modes_for_8() {
    let src = r#"fn three(a: Int, b: P, c: move P) do
  print(a)
  print(b.x)
  print(c.x)
end"#;
    let modes = own_modes(src, "three");
    assert_eq!(modes.len(), 3);
    assert_eq!(modes[0], ("a".to_string(), ParamMode::Value));
    assert_eq!(modes[1], ("b".to_string(), ParamMode::Borrow));
    assert_eq!(modes[2], ("c".to_string(), ParamMode::Move));
    assert!(own_modes(src, "missing").is_empty());
}

#[test]
fn own_modes_for_9() {
    let src = r#"fn three(a: Int, b: P, c: move P) do
  print(a)
  print(b.x)
  print(c.x)
end"#;
    let modes = own_modes(src, "three");
    assert_eq!(modes.len(), 3);
    assert_eq!(modes[0], ("a".to_string(), ParamMode::Value));
    assert_eq!(modes[1], ("b".to_string(), ParamMode::Borrow));
    assert_eq!(modes[2], ("c".to_string(), ParamMode::Move));
    assert!(own_modes(src, "missing").is_empty());
}

#[test]
fn own_modes_for_10() {
    let src = r#"fn three(a: Int, b: P, c: move P) do
  print(a)
  print(b.x)
  print(c.x)
end"#;
    let modes = own_modes(src, "three");
    assert_eq!(modes.len(), 3);
    assert_eq!(modes[0], ("a".to_string(), ParamMode::Value));
    assert_eq!(modes[1], ("b".to_string(), ParamMode::Borrow));
    assert_eq!(modes[2], ("c".to_string(), ParamMode::Move));
    assert!(own_modes(src, "missing").is_empty());
}

#[test]
fn own_moves_1() {
    let moves = own_moves(r#"fn take(p: move P) do
  print(p.x)
end

fn main() do
  let a = P { x = 2 }
  take(a)
  print(1)
end"#);
    assert_eq!(moves.len(), 1);
    assert_eq!(moves[0], ("main".to_string(), "a".to_string(), "take".to_string()));
}

#[test]
fn own_moves_2() {
    let moves = own_moves(r#"fn take(p: move P) do
  print(p.x)
end

fn main() do
  let a = P { x = 4 }
  take(a)
  print(1)
end"#);
    assert_eq!(moves.len(), 1);
    assert_eq!(moves[0], ("main".to_string(), "a".to_string(), "take".to_string()));
}

#[test]
fn own_moves_3() {
    let moves = own_moves(r#"fn take(p: move P) do
  print(p.x)
end

fn main() do
  let a = P { x = 6 }
  take(a)
  print(1)
end"#);
    assert_eq!(moves.len(), 1);
    assert_eq!(moves[0], ("main".to_string(), "a".to_string(), "take".to_string()));
}

#[test]
fn own_moves_4() {
    let moves = own_moves(r#"fn take(p: move P) do
  print(p.x)
end

fn main() do
  let a = P { x = 8 }
  take(a)
  print(1)
end"#);
    assert_eq!(moves.len(), 1);
    assert_eq!(moves[0], ("main".to_string(), "a".to_string(), "take".to_string()));
}

#[test]
fn own_moves_5() {
    let moves = own_moves(r#"fn take(p: move P) do
  print(p.x)
end

fn main() do
  let a = P { x = 10 }
  take(a)
  print(1)
end"#);
    assert_eq!(moves.len(), 1);
    assert_eq!(moves[0], ("main".to_string(), "a".to_string(), "take".to_string()));
}

#[test]
fn own_moves_6() {
    let moves = own_moves(r#"fn take(p: move P) do
  print(p.x)
end

fn main() do
  let a = P { x = 12 }
  take(a)
  print(1)
end"#);
    assert_eq!(moves.len(), 1);
    assert_eq!(moves[0], ("main".to_string(), "a".to_string(), "take".to_string()));
}

#[test]
fn own_moves_7() {
    let moves = own_moves(r#"fn take(p: move P) do
  print(p.x)
end

fn main() do
  let a = P { x = 14 }
  take(a)
  print(1)
end"#);
    assert_eq!(moves.len(), 1);
    assert_eq!(moves[0], ("main".to_string(), "a".to_string(), "take".to_string()));
}

#[test]
fn own_moves_8() {
    let moves = own_moves(r#"fn take(p: move P) do
  print(p.x)
end

fn main() do
  let a = P { x = 16 }
  take(a)
  print(1)
end"#);
    assert_eq!(moves.len(), 1);
    assert_eq!(moves[0], ("main".to_string(), "a".to_string(), "take".to_string()));
}

#[test]
fn own_moves_9() {
    let moves = own_moves(r#"fn take(p: move P) do
  print(p.x)
end

fn main() do
  let a = P { x = 18 }
  take(a)
  print(1)
end"#);
    assert_eq!(moves.len(), 1);
    assert_eq!(moves[0], ("main".to_string(), "a".to_string(), "take".to_string()));
}

#[test]
fn own_moves_10() {
    let moves = own_moves(r#"fn take(p: move P) do
  print(p.x)
end

fn main() do
  let a = P { x = 20 }
  take(a)
  print(1)
end"#);
    assert_eq!(moves.len(), 1);
    assert_eq!(moves[0], ("main".to_string(), "a".to_string(), "take".to_string()));
}

#[test]
fn own_clean_1() {
    assert_eq!(own_errors(r#"fn main() do
  var s = 0
  for i in 10 do
    s = s + i
  end
  print(s)
end"#), 0);
    assert!(own_codes(r#"fn main() do
  var s = 0
  for i in 10 do
    s = s + i
  end
  print(s)
end"#).is_empty());
}

#[test]
fn own_clean_2() {
    assert_eq!(own_errors(r#"fn add(a: Int, b: Int) do
  return a + b
end

fn main() do
  print(add(2, 3))
end"#), 0);
    assert!(own_codes(r#"fn add(a: Int, b: Int) do
  return a + b
end

fn main() do
  print(add(2, 3))
end"#).is_empty());
}

#[test]
fn own_clean_3() {
    assert_eq!(own_errors(r#"P = struct
  x: Int = 0
end

fn main() do
  let p = P { x = 1 }
  print(p.x)
end"#), 0);
    assert!(own_codes(r#"P = struct
  x: Int = 0
end

fn main() do
  let p = P { x = 1 }
  print(p.x)
end"#).is_empty());
}

#[test]
fn own_clean_4() {
    assert_eq!(own_errors(r#"fn main() do
  var c = 0
  repeat
    c = c + 1
  until c >= 3
  print(c)
end"#), 0);
    assert!(own_codes(r#"fn main() do
  var c = 0
  repeat
    c = c + 1
  until c >= 3
  print(c)
end"#).is_empty());
}

#[test]
fn own_clean_5() {
    assert_eq!(own_errors(r#"fn main() do
  let a = [1, 2, 3]
  print(a[1])
end"#), 0);
    assert!(own_codes(r#"fn main() do
  let a = [1, 2, 3]
  print(a[1])
end"#).is_empty());
}

#[test]
fn own_clean_6() {
    assert_eq!(own_errors(r#"fn main() do
  match 5
    1 => print("one")
    5 => print("five")
    _ => print("other")
  end
end"#), 0);
    assert!(own_codes(r#"fn main() do
  match 5
    1 => print("one")
    5 => print("five")
    _ => print("other")
  end
end"#).is_empty());
}

#[test]
fn own_clean_7() {
    assert_eq!(own_errors(r#"fn main() do
  var s = 0
  for i in 10 do
    s = s + i
  end
  print(s)
end"#), 0);
    assert!(own_codes(r#"fn main() do
  var s = 0
  for i in 10 do
    s = s + i
  end
  print(s)
end"#).is_empty());
}

#[test]
fn own_clean_8() {
    assert_eq!(own_errors(r#"fn add(a: Int, b: Int) do
  return a + b
end

fn main() do
  print(add(2, 3))
end"#), 0);
    assert!(own_codes(r#"fn add(a: Int, b: Int) do
  return a + b
end

fn main() do
  print(add(2, 3))
end"#).is_empty());
}

#[test]
fn own_clean_9() {
    assert_eq!(own_errors(r#"P = struct
  x: Int = 0
end

fn main() do
  let p = P { x = 1 }
  print(p.x)
end"#), 0);
    assert!(own_codes(r#"P = struct
  x: Int = 0
end

fn main() do
  let p = P { x = 1 }
  print(p.x)
end"#).is_empty());
}

#[test]
fn own_clean_10() {
    assert_eq!(own_errors(r#"fn main() do
  var c = 0
  repeat
    c = c + 1
  until c >= 3
  print(c)
end"#), 0);
    assert!(own_codes(r#"fn main() do
  var c = 0
  repeat
    c = c + 1
  until c >= 3
  print(c)
end"#).is_empty());
}

#[test]
fn own_clean_11() {
    assert_eq!(own_errors(r#"fn main() do
  let a = [1, 2, 3]
  print(a[1])
end"#), 0);
    assert!(own_codes(r#"fn main() do
  let a = [1, 2, 3]
  print(a[1])
end"#).is_empty());
}

#[test]
fn own_clean_12() {
    assert_eq!(own_errors(r#"fn main() do
  match 5
    1 => print("one")
    5 => print("five")
    _ => print("other")
  end
end"#), 0);
    assert!(own_codes(r#"fn main() do
  match 5
    1 => print("one")
    5 => print("five")
    _ => print("other")
  end
end"#).is_empty());
}

#[test]
fn own_clean_13() {
    assert_eq!(own_errors(r#"fn main() do
  var s = 0
  for i in 10 do
    s = s + i
  end
  print(s)
end"#), 0);
    assert!(own_codes(r#"fn main() do
  var s = 0
  for i in 10 do
    s = s + i
  end
  print(s)
end"#).is_empty());
}

#[test]
fn own_clean_14() {
    assert_eq!(own_errors(r#"fn add(a: Int, b: Int) do
  return a + b
end

fn main() do
  print(add(2, 3))
end"#), 0);
    assert!(own_codes(r#"fn add(a: Int, b: Int) do
  return a + b
end

fn main() do
  print(add(2, 3))
end"#).is_empty());
}

#[test]
fn own_clean_15() {
    assert_eq!(own_errors(r#"P = struct
  x: Int = 0
end

fn main() do
  let p = P { x = 1 }
  print(p.x)
end"#), 0);
    assert!(own_codes(r#"P = struct
  x: Int = 0
end

fn main() do
  let p = P { x = 1 }
  print(p.x)
end"#).is_empty());
}

#[test]
fn own_clean_16() {
    assert_eq!(own_errors(r#"fn main() do
  var c = 0
  repeat
    c = c + 1
  until c >= 3
  print(c)
end"#), 0);
    assert!(own_codes(r#"fn main() do
  var c = 0
  repeat
    c = c + 1
  until c >= 3
  print(c)
end"#).is_empty());
}

#[test]
fn own_clean_17() {
    assert_eq!(own_errors(r#"fn main() do
  let a = [1, 2, 3]
  print(a[1])
end"#), 0);
    assert!(own_codes(r#"fn main() do
  let a = [1, 2, 3]
  print(a[1])
end"#).is_empty());
}

#[test]
fn own_clean_18() {
    assert_eq!(own_errors(r#"fn main() do
  match 5
    1 => print("one")
    5 => print("five")
    _ => print("other")
  end
end"#), 0);
    assert!(own_codes(r#"fn main() do
  match 5
    1 => print("one")
    5 => print("five")
    _ => print("other")
  end
end"#).is_empty());
}

#[test]
fn own_moves_two_1() {
    let moves = own_moves(r#"fn take(p: move P) do
  print(p.x)
end

fn main() do
  let a = P { x = 1 }
  let b = P { x = 2 }
  take(a)
  take(b)
  print(1)
end"#);
    assert_eq!(moves.len(), 2);
    assert!(moves.iter().any(|m| m.1 == "a"));
    assert!(moves.iter().any(|m| m.1 == "b"));
}

#[test]
fn own_moves_two_2() {
    let moves = own_moves(r#"fn take(p: move P) do
  print(p.x)
end

fn main() do
  let a = P { x = 2 }
  let b = P { x = 3 }
  take(a)
  take(b)
  print(1)
end"#);
    assert_eq!(moves.len(), 2);
    assert!(moves.iter().any(|m| m.1 == "a"));
    assert!(moves.iter().any(|m| m.1 == "b"));
}

#[test]
fn own_moves_two_3() {
    let moves = own_moves(r#"fn take(p: move P) do
  print(p.x)
end

fn main() do
  let a = P { x = 3 }
  let b = P { x = 4 }
  take(a)
  take(b)
  print(1)
end"#);
    assert_eq!(moves.len(), 2);
    assert!(moves.iter().any(|m| m.1 == "a"));
    assert!(moves.iter().any(|m| m.1 == "b"));
}

#[test]
fn own_moves_two_4() {
    let moves = own_moves(r#"fn take(p: move P) do
  print(p.x)
end

fn main() do
  let a = P { x = 4 }
  let b = P { x = 5 }
  take(a)
  take(b)
  print(1)
end"#);
    assert_eq!(moves.len(), 2);
    assert!(moves.iter().any(|m| m.1 == "a"));
    assert!(moves.iter().any(|m| m.1 == "b"));
}

#[test]
fn own_moves_two_5() {
    let moves = own_moves(r#"fn take(p: move P) do
  print(p.x)
end

fn main() do
  let a = P { x = 5 }
  let b = P { x = 6 }
  take(a)
  take(b)
  print(1)
end"#);
    assert_eq!(moves.len(), 2);
    assert!(moves.iter().any(|m| m.1 == "a"));
    assert!(moves.iter().any(|m| m.1 == "b"));
}

#[test]
fn own_moves_two_6() {
    let moves = own_moves(r#"fn take(p: move P) do
  print(p.x)
end

fn main() do
  let a = P { x = 6 }
  let b = P { x = 7 }
  take(a)
  take(b)
  print(1)
end"#);
    assert_eq!(moves.len(), 2);
    assert!(moves.iter().any(|m| m.1 == "a"));
    assert!(moves.iter().any(|m| m.1 == "b"));
}

#[test]
fn own_moves_two_7() {
    let moves = own_moves(r#"fn take(p: move P) do
  print(p.x)
end

fn main() do
  let a = P { x = 7 }
  let b = P { x = 8 }
  take(a)
  take(b)
  print(1)
end"#);
    assert_eq!(moves.len(), 2);
    assert!(moves.iter().any(|m| m.1 == "a"));
    assert!(moves.iter().any(|m| m.1 == "b"));
}

#[test]
fn own_moves_two_8() {
    let moves = own_moves(r#"fn take(p: move P) do
  print(p.x)
end

fn main() do
  let a = P { x = 8 }
  let b = P { x = 9 }
  take(a)
  take(b)
  print(1)
end"#);
    assert_eq!(moves.len(), 2);
    assert!(moves.iter().any(|m| m.1 == "a"));
    assert!(moves.iter().any(|m| m.1 == "b"));
}

#[test]
fn own_moves_two_9() {
    let moves = own_moves(r#"fn take(p: move P) do
  print(p.x)
end

fn main() do
  let a = P { x = 9 }
  let b = P { x = 10 }
  take(a)
  take(b)
  print(1)
end"#);
    assert_eq!(moves.len(), 2);
    assert!(moves.iter().any(|m| m.1 == "a"));
    assert!(moves.iter().any(|m| m.1 == "b"));
}

#[test]
fn own_moves_two_10() {
    let moves = own_moves(r#"fn take(p: move P) do
  print(p.x)
end

fn main() do
  let a = P { x = 10 }
  let b = P { x = 11 }
  take(a)
  take(b)
  print(1)
end"#);
    assert_eq!(moves.len(), 2);
    assert!(moves.iter().any(|m| m.1 == "a"));
    assert!(moves.iter().any(|m| m.1 == "b"));
}

/// A move call in tail position is the move itself, not a use-after-move.
#[test]
fn e0101_tail_move_call_is_not_use_after_move() {
    let (_, diags) = infer_ownership(r#"fn take(name: move String) -> String
    return name
end

fn main() do
    var s = "hello"
    take(s)
end"#, 0);
    let codes: Vec<String> = diags.items.iter().filter(|d| d.severity == axolc_core::diag::Severity::Error).map(|d| d.code.clone().unwrap_or_default()).collect();
    assert!(!codes.iter().any(|c| c == "E0101"), "tail move call must not be flagged, got {:?}", codes);
}

/// A genuine tail-position use after an earlier move still reports E0101.
#[test]
fn e0101_tail_use_after_move_still_flags() {
    let (_, diags) = infer_ownership(r#"fn take(name: move String) -> String
    return name
end

fn main() do
    var s = "hello"
    var t = take(s)
    s
end"#, 0);
    let codes: Vec<String> = diags.items.iter().filter(|d| d.severity == axolc_core::diag::Severity::Error).map(|d| d.code.clone().unwrap_or_default()).collect();
    assert!(codes.iter().any(|c| c == "E0101"), "expected E0101 for tail use, got {:?}", codes);
}

/// A later argument using the moved name in the same call expression flags.
#[test]
fn e0101_same_expr_later_arg_still_flags() {
    let (_, diags) = infer_ownership(r#"fn take(name: move String) -> String
    return name
end

fn pair(a: String, b: String) -> String
    return a
end

fn main() do
    var s = "hello"
    var t = pair(take(s), s)
    print(t)
end"#, 0);
    let codes: Vec<String> = diags.items.iter().filter(|d| d.severity == axolc_core::diag::Severity::Error).map(|d| d.code.clone().unwrap_or_default()).collect();
    assert!(codes.iter().any(|c| c == "E0101"), "expected E0101 for later arg use, got {:?}", codes);
}

/// A non-Copy local captured by a closure moves; later use reports E0101.
#[test]
fn closure_capture_moves_noncopy_local() {
    let (_, diags) = infer_ownership(r#"fn main() do
    var s = "hello"
    var f = fn() do
        print(s)
    end
    print(s)
end"#, 0);
    let codes: Vec<String> = diags.items.iter().filter(|d| d.severity == axolc_core::diag::Severity::Error).map(|d| d.code.clone().unwrap_or_default()).collect();
    assert!(codes.iter().any(|c| c == "E0101"), "expected E0101 for capture-as-move, got {:?}", codes);
}

/// A Copy local captured by a closure is cloned; later use stays clean.
#[test]
fn closure_capture_keeps_copy_local() {
    let (_, diags) = infer_ownership(r#"fn main() do
    var n = 5
    var f = fn() do
        print(n)
    end
    print(n)
end"#, 0);
    let codes: Vec<String> = diags.items.iter().filter(|d| d.severity == axolc_core::diag::Severity::Error).map(|d| d.code.clone().unwrap_or_default()).collect();
    assert!(!codes.iter().any(|c| c == "E0101"), "Copy capture must not be flagged, got {:?}", codes);
}

/// Closure params and inner bindings shadow, not capture.
#[test]
fn closure_bindings_shadow_not_capture() {
    let (_, diags) = infer_ownership(r#"fn main() do
    var s = "hello"
    var f = fn(s: String) do
        print(s)
    end
    print(s)
end"#, 0);
    let codes: Vec<String> = diags.items.iter().filter(|d| d.severity == axolc_core::diag::Severity::Error).map(|d| d.code.clone().unwrap_or_default()).collect();
    assert!(!codes.iter().any(|c| c == "E0101"), "shadowed param is not a capture, got {:?}", codes);
}
