// Owner: PascalElixir / axolrs (GitHub org)
// File: benchmarks/database/rust/src/main.rs - hash-indexed pager with leaf pages (flat layout)
//
// Reference implementation. Mirrors ../axol/src/main.axol exactly;
// the bench harness in ../bench/run.sh asserts byte-identical stdout
// and axol_time / rust_time <= 1.05.

use std::env;

const DEFAULT_WORKLOAD: i64 = 100_000;
const FANOUT: i64 = 32;
const LEAF_CAP: i64 = 256;

fn hash(key: i64) -> i64 { key.wrapping_mul(2654435761) & (FANOUT - 1) }

fn main() {
    let workload: i64 = env::args().nth(1).and_then(|s| s.parse::<i64>().ok()).unwrap_or(DEFAULT_WORKLOAD);
    // Flat: leaf_keys[slot * LEAF_CAP + i], leaf_vals[slot * LEAF_CAP + i]
    let mut leaf_keys: Vec<i64> = vec![0; (FANOUT * LEAF_CAP) as usize];
    let mut leaf_vals: Vec<i64> = vec![0; (FANOUT * LEAF_CAP) as usize];
    let mut leaf_counts: Vec<i64> = vec![0; FANOUT as usize];
    let mut count: i64 = 0;
    let mut i: i64 = 0;
    while i < workload {
        let key = hash(i);
        let slot = (key & (FANOUT - 1)) as usize;
        let idx = slot * LEAF_CAP as usize + leaf_counts[slot] as usize;
        if leaf_counts[slot] < LEAF_CAP {
            leaf_keys[idx] = key;
            leaf_vals[idx] = i + 7;
            leaf_counts[slot] += 1;
            count += 1;
        }
        i += 1;
    }
    let mut hits: i64 = 0;
    i = 0;
    while i < workload {
        let key = hash(i);
        let slot = (key & (FANOUT - 1)) as usize;
        let mut j: i64 = 0;
        while j < leaf_counts[slot] {
            if leaf_keys[slot * LEAF_CAP as usize + j as usize] == key {
                hits += 1;
                break;
            }
            j += 1;
        }
        i += 1;
    }
    let mut csum: i64 = 0;
    let mut slot: i64 = 0;
    while slot < FANOUT {
        let mut j: i64 = 0;
        while j < leaf_counts[slot as usize] {
            csum = csum.wrapping_add(leaf_keys[slot as usize * LEAF_CAP as usize + j as usize]);
            csum = csum.wrapping_add(leaf_vals[slot as usize * LEAF_CAP as usize + j as usize]);
            j += 1;
        }
        slot += 1;
    }
    println!("{} {} {} {} {} {} {} {}", "database: slots=", FANOUT, "rows=", count, "hits=", hits, "checksum=", csum);
}
