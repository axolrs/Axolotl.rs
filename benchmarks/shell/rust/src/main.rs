// Owner: PascalElixir / axolrs (GitHub org)
// File: benchmarks/shell/rust/src/main.rs - POSIX-like shell token pipeline simulator
//
// Reference implementation. Mirrors ../axol/src/main.axol exactly;
// the bench harness in ../bench/run.sh asserts byte-identical stdout
// and axol_time / rust_time <= 1.05.

use std::env;

const DEFAULT_WORKLOAD: i64 = 30_000;

fn main() {
    let workload: i64 = env::args().nth(1).and_then(|s| s.parse::<i64>().ok()).unwrap_or(DEFAULT_WORKLOAD);
    let mut log: Vec<i64> = Vec::new();
    let mut args_total: i64 = 0;
    let mut redirects_total: i64 = 0;
    let mut i: i64 = 0;
    while i < workload {
        let n_stages = (i % 4) + 1;
        let n_args = (i % 5) + 1;
        let n_redirects = i % 3;
        let signature = n_stages * 1000 + n_args * 10 + n_redirects;
        log.push(signature);
        args_total += n_args;
        redirects_total += n_redirects;
        i += 1;
    }
    let mut h: i64 = 0;
    let mut k: i64 = 0;
    while k < log.len() as i64 { h = h.wrapping_mul(31).wrapping_add(log[k as usize]); k += 1; }
    println!("{} {} {} {} {} {} {} {} {} {}", "shell: lines=", workload, "stages=", log.len(), "args=", args_total, "redirects=", redirects_total, "checksum=", h);
}
