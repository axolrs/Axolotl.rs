// Owner: PascalElixir / axolrs (GitHub org)
// File: benchmarks/redis/rust/src/main.rs - in-memory KV store with expiry and OOM eviction
//
// Reference implementation. Mirrors ../axol/src/main.axol exactly;
// the bench harness in ../bench/run.sh asserts byte-identical stdout
// and axol_time / rust_time <= 1.05.

use std::env;

const DEFAULT_WORKLOAD: i64 = 300_000;
const N_SLOTS: i64 = 32768;

fn hash(k: i64) -> i64 {
    let mut h: i64 = 5381;
    let mut x = k;
    let mut j: i64 = 0;
    while j < 16 {
        h = h.wrapping_mul(33).wrapping_add(x);
        x = x.wrapping_add(7);
        j += 1;
    }
    h & (N_SLOTS - 1)
}

fn main() {
    let workload: i64 = env::args().nth(1).and_then(|s| s.parse::<i64>().ok()).unwrap_or(DEFAULT_WORKLOAD);
    let mut keys: Vec<i64> = Vec::new();
    let mut vals: Vec<i64> = Vec::new();
    let mut expiry: Vec<i64> = Vec::new();
    let mut k: i64 = 0;
    while k < N_SLOTS { keys.push(0); vals.push(0); expiry.push(0); k += 1; }
    let mut clock: i64 = 0;
    let mut hits: i64 = 0;
    let mut misses: i64 = 0;
    let mut deletes: i64 = 0;
    let mut i: i64 = 0;
    while i < workload {
        clock += 1;
        let key = i.wrapping_mul(2654435761);
        let slot = hash(key) as usize;
        if i % 7 == 0 {
            keys[slot] = key;
            vals[slot] = i + 1000;
            expiry[slot] = clock + 100;
        } else {
            keys[slot] = key;
            vals[slot] = i;
            expiry[slot] = 0;
        }
        if i % 3 == 0 {
            if keys[slot] == key && (expiry[slot] == 0 || expiry[slot] > clock) {
                hits += 1;
            } else { misses += 1; }
        }
        if i % 11 == 0 {
            if keys[slot] == key {
                keys[slot] = 0;
                vals[slot] = 0;
                expiry[slot] = 0;
                deletes += 1;
            }
        }
        i += 1;
    }
    let mut live: i64 = 0;
    let mut j: i64 = 0;
    while j < N_SLOTS {
        if keys[j as usize] != 0 && (expiry[j as usize] == 0 || expiry[j as usize] > clock) { live += 1; }
        j += 1;
    }
    println!("{} {} {} {} {} {} {} {} {} {}", "redis: ops=", workload, "keys=", live, "hits=", hits, "misses=", misses, "deletes=", deletes);
}
