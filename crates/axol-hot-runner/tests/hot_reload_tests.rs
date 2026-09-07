// Owner: PascalElixir / axolrs (GitHub org)
// File: Hot-reload state preservation tests - snapshots round-trip across reloads.

use axol_hot_runner::reload::{run_with_state, StateSnapshot};

/// Assert one run captures a snapshot with the expected single entry.
#[test]
fn snapshot_captures_int_const() {
    let (snap, errs) = StateSnapshot::capture("const count = 42\nfn main() print(count) end", &StateSnapshot::empty());
    assert!(errs.is_empty(), "errors: {:?}", errs);
    assert_eq!(snap.get("count").map(|v| v.render()), Some("42".to_string()));
}

#[test]
fn snapshot_captures_bool_const() {
    let (snap, errs) = StateSnapshot::capture("const flag = true\nfn main() print(flag) end", &StateSnapshot::empty());
    assert!(errs.is_empty(), "errors: {:?}", errs);
    assert_eq!(snap.get("flag").map(|v| v.render()), Some("true".to_string()));
}

#[test]
fn snapshot_captures_string_const() {
    let (snap, errs) = StateSnapshot::capture("const name = \"Mohi\"\nfn main() print(name) end", &StateSnapshot::empty());
    assert!(errs.is_empty(), "errors: {:?}", errs);
    assert_eq!(snap.get("name").map(|v| v.render()), Some("Mohi".to_string()));
}

#[test]
fn snapshot_captures_typed_const() {
    let (snap, errs) = StateSnapshot::capture("const MAX: Int = 100\nfn main() print(MAX) end", &StateSnapshot::empty());
    assert!(errs.is_empty());
    assert_eq!(snap.get("MAX").map(|v| v.render()), Some("100".to_string()));
}

#[test]
fn snapshot_seeds_from_previous() {
    let (snap1, _) = StateSnapshot::capture("const n = 5\nfn main() print(n) end", &StateSnapshot::empty());
    let (snap2, errs) = StateSnapshot::capture("const n = 5\nfn main() print(n) end", &snap1);
    assert!(errs.is_empty());
    assert_eq!(snap2.get("n").map(|v| v.render()), Some("5".to_string()));
}

#[test]
fn snapshot_seeded_value_wins() {
    let (snap1, _) = StateSnapshot::capture("const n = 5\nfn main() print(n) end", &StateSnapshot::empty());
    let (snap2, errs) = StateSnapshot::capture("const n = 9\nfn main() print(n) end", &snap1);
    assert!(errs.is_empty());
    assert_eq!(snap2.get("n").map(|v| v.render()), Some("5".to_string()));
}

#[test]
fn snapshot_unseeded_uses_source_value() {
    let (snap, errs) = StateSnapshot::capture("const n = 9\nfn main() print(n) end", &StateSnapshot::empty());
    assert!(errs.is_empty());
    assert_eq!(snap.get("n").map(|v| v.render()), Some("9".to_string()));
}

#[test]
fn snapshot_round_trips_pairs() {
    let (snap, _) = StateSnapshot::capture("const a = 1\nconst b = false\nfn main() print(a) end", &StateSnapshot::empty());
    let pairs = snap.pairs();
    assert_eq!(pairs.len(), 2, "pairs: {:?}", pairs);
    assert!(pairs.iter().any(|(n, v)| n == "a" && v == "1"));
    assert!(pairs.iter().any(|(n, v)| n == "b" && v == "false"));
}

#[test]
fn snapshot_len_and_empty() {
    assert!(StateSnapshot::empty().is_empty());
    assert_eq!(StateSnapshot::empty().len(), 0);
    let (snap, _) = StateSnapshot::capture("const x = 1\nfn main() print(x) end", &StateSnapshot::empty());
    assert!(!snap.is_empty());
    assert_eq!(snap.len(), 1);
}

#[test]
fn snapshot_parse_error_keeps_previous() {
    let (prev, _) = StateSnapshot::capture("const ok = 1\nfn main() print(ok) end", &StateSnapshot::empty());
    let (snap, errs) = StateSnapshot::capture("fn broken( end", &prev);
    assert!(!errs.is_empty());
    assert_eq!(snap, prev);
}

#[test]
fn run_with_state_reports_parse_errors() {
    let (out, errs) = run_with_state("fn broken( end", &StateSnapshot::empty());
    assert!(out.is_empty());
    assert!(!errs.is_empty());
}

#[test]
fn run_with_state_runs_clean_program() {
    let (out, errs) = run_with_state("fn main() print(3 + 4) end", &StateSnapshot::empty());
    assert!(errs.is_empty());
    assert_eq!(out.trim(), "7");
}

#[test]
fn run_with_state_injects_snapshot() {
    let (snap, _) = StateSnapshot::capture("const n = 41\nfn main() print(n) end", &StateSnapshot::empty());
    let (out, errs) = run_with_state("fn main() print(n) end", &snap);
    assert!(errs.is_empty(), "errors: {:?}", errs);
    assert!(out.contains("41"), "output: {}", out);
}

#[test]
fn snapshot_captures_local_mutation_in_main() {
    let (snap, errs) = StateSnapshot::capture(
        "fn main()\n    var hits = 3\n    print(hits)\nend",
        &StateSnapshot::empty(),
    );
    assert!(errs.is_empty());
    assert_eq!(snap.get("hits").map(|v| v.render()), Some("3".to_string()));
}
