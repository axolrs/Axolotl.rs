// Owner: PascalElixir / axolrs (GitHub org)
// File: benchmarks/kvstore/rust/src/main.rs - LSM-tree-style store with forward scan lookup
//
// Reference implementation. Mirrors ../axol/src/main.axol exactly.

use std::env;

const DEFAULT_WORKLOAD: i64 = 2_000_000;
const MEMTABLE_LIMIT: i64 = 1024;

fn hash(k: i64) -> i64 { k.wrapping_mul(2654435761) }

fn main() {
    let workload: i64 = env::args().nth(1).and_then(|s| s.parse::<i64>().ok()).unwrap_or(DEFAULT_WORKLOAD);
    let mut memtable_k: Vec<i64> = Vec::with_capacity(MEMTABLE_LIMIT as usize);
    let mut memtable_v: Vec<i64> = Vec::with_capacity(MEMTABLE_LIMIT as usize);
    let mut sstable_count: i64 = 0;
    let mut n_writes: i64 = 0;
    let mut n_reads: i64 = 0;
    let mut n_hits: i64 = 0;
    let mut i: i64 = 0;
    while i < workload {
        let h = hash(i);
        memtable_k.push(h);
        memtable_v.push(i * 7);
        n_writes += 1;
        if memtable_k.len() as i64 >= MEMTABLE_LIMIT {
            sstable_count += 1;
            memtable_k.clear();
            memtable_v.clear();
        }
        if i % 3 == 0 {
            n_reads += 1;
            let target = hash(i);
            // Forward scan (faster than reverse in axolc)
            let mut j: i64 = 0;
            let len = memtable_k.len() as i64;
            while j < len {
                if memtable_k[j as usize] == target { n_hits += 1; break; }
                j += 1;
            }
        }
        i += 1;
    }
    println!("{} {} {} {} {} {} {} {} {} {}", "kvstore: writes=", n_writes, "reads=", n_reads, "hits=", n_hits, "sstables=", sstable_count);
}
