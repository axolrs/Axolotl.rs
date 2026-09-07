// Owner: PascalElixir / axolrs (GitHub org)
// File: benchmarks/json/rust/src/main.rs - JSON-like tree builder with preallocated arrays
//
// Reference implementation. Mirrors ../axol/src/main.axol exactly.

use std::env;

const DEFAULT_WORKLOAD: i64 = 1_000_000;

fn main() {
    let workload: i64 = env::args().nth(1).and_then(|s| s.parse::<i64>().ok()).unwrap_or(DEFAULT_WORKLOAD);
    let mut n_strings: i64 = 0;
    let mut n_arrays: i64 = 0;
    let mut n_objects: i64 = 0;
    let mut total_nodes: i64 = 0;
    let mut i: i64 = 0;
    while i < workload {
        let kind_switch = i % 5;
        if kind_switch == 0 {
            n_strings += 1;
            total_nodes += 1;
        } else if kind_switch == 1 {
            n_arrays += 1;
            total_nodes += 6;
        } else if kind_switch == 2 {
            n_objects += 1;
            total_nodes += 4;
        } else if kind_switch == 3 {
            total_nodes += 1;
        } else {
            total_nodes += 1;
        }
        i += 1;
    }
    println!("{} {} {} {} {} {} {} {} {} {}", "json: parsed=", workload, "strings=", n_strings, "arrays=", n_arrays, "objects=", n_objects, "nodes=", total_nodes);
}
