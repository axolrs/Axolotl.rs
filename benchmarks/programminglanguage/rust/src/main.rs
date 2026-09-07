// Owner: PascalElixir / axolrs (GitHub org)
// File: benchmarks/programminglanguage/rust/src/main.rs - stack-based VM running a factorial-like program
//
// Reference implementation. Mirrors ../axol/src/main.axol exactly;
// the bench harness in ../bench/run.sh asserts byte-identical stdout
// and axol_time / rust_time <= 1.05.

use std::env;

const DEFAULT_WORKLOAD: i64 = 10_000;

fn main() {
    let workload: i64 = env::args().nth(1).and_then(|s| s.parse::<i64>().ok()).unwrap_or(DEFAULT_WORKLOAD);
    let mut sum: i64 = 0;
    let mut i: i64 = 0;
    while i < workload {
        let n: i64 = if i % 12 == 0 { 1 } else { i % 12 };
        let mut stack: Vec<i64> = Vec::new();
        let mut acc: i64 = 1;
        let mut k: i64 = 1;
        while k <= n {
            acc = acc.wrapping_mul(k);
            stack.push(acc);
            k += 1;
        }
        if !stack.is_empty() { sum = sum.wrapping_add(*stack.last().unwrap()); }
        i += 1;
    }
    println!("{} {} {} {}", "programminglanguage: forms=", workload, "sum=", sum);
}
