// Owner: PascalElixir / axolrs (GitHub org)
// File: benchmarks/compiler/rust/src/main.rs - stack-based bytecode VM running a factorial-like program
//
// Reference implementation. Mirrors ../axol/src/main.axol exactly;
// the bench harness in ../bench/run.sh asserts byte-identical stdout
// and axol_time / rust_time <= 1.05.

use std::env;

const DEFAULT_WORKLOAD: i64 = 200_000;

fn main() {
    let workload: i64 = env::args().nth(1).and_then(|s| s.parse::<i64>().ok()).unwrap_or(DEFAULT_WORKLOAD);
    let mut sum: i64 = 0;
    let mut i: i64 = 0;
    while i < workload {
        let n: i64 = if i % 100 == 0 { 1 } else { i % 100 };
        let mut stack: Vec<i64> = Vec::new();
        stack.push(1);
        let mut k: i64 = 1;
        while k <= n {
            let top = *stack.last().unwrap();
            stack.push(top.wrapping_mul(k));
            k += 1;
        }
        if !stack.is_empty() { sum = sum.wrapping_add(*stack.last().unwrap()); }
        i += 1;
    }
    println!("{} {} {} {}", "compiler: runs=", workload, "sum=", sum);
}
