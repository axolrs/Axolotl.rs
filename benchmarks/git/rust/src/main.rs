// Owner: PascalElixir / axolrs (GitHub org)
// File: benchmarks/git/rust/src/main.rs - content-addressed blob store with djb2 hashing
//
// Reference implementation. Mirrors ../axol/src/main.axol exactly;
// the bench harness in ../bench/run.sh asserts byte-identical stdout
// and axol_time / rust_time <= 1.05.

use std::env;

const DEFAULT_WORKLOAD: i64 = 200_000;

fn djb2(seed: i64, salt: i64) -> i64 {
    let mut h: i64 = 5381;
    let mut x = seed;
    let mut j: i64 = 0;
    while j < 64 {
        h = h.wrapping_mul(33).wrapping_add((x + salt) & 0xFFFF_FFFF);
        x = x.wrapping_add(7);
        j += 1;
    }
    h & 0x7FFF_FFFF_FFFF_FFFF
}

fn main() {
    let workload: i64 = env::args().nth(1).and_then(|s| s.parse::<i64>().ok()).unwrap_or(DEFAULT_WORKLOAD);
    let mut blobs: Vec<i64> = Vec::new();
    let mut trees: Vec<i64> = Vec::new();
    let mut commits: Vec<i64> = Vec::new();
    let mut last_commit: i64 = 0;
    let mut blob_buf: Vec<i64> = Vec::new();
    let mut i: i64 = 0;
    while i < workload {
        let h = djb2(i, i + 11);
        blobs.push(h);
        blob_buf.push(h);
        if blob_buf.len() == 64 {
            let mut sum: i64 = 0;
            let mut k: i64 = 0;
            while k < blob_buf.len() as i64 { sum = sum.wrapping_add(blob_buf[k as usize]); k += 1; }
            trees.push(sum);
            let csum = sum.wrapping_mul(31);
            commits.push(csum);
            last_commit = commits.len() as i64 - 1;
            blob_buf.clear();
        }
        i += 1;
    }
    if !blob_buf.is_empty() {
        let mut sum: i64 = 0;
        let mut k: i64 = 0;
        while k < blob_buf.len() as i64 { sum = sum.wrapping_add(blob_buf[k as usize]); k += 1; }
        trees.push(sum);
        let csum = sum.wrapping_mul(31);
        commits.push(csum);
        last_commit = commits.len() as i64 - 1;
    }
    let mut acc: i64 = 0;
    let mut k: i64 = 0;
    while k < commits.len() as i64 { acc = acc.wrapping_add(commits[k as usize]); k += 1; }
    k = 0;
    while k < trees.len() as i64 { acc = acc.wrapping_sub(trees[k as usize]); k += 1; }
    k = 0;
    while k < blobs.len() as i64 { acc = acc.wrapping_add(blobs[k as usize]); k += 1; }
    let checksum = acc & 0x7FFF_FFFF_FFFF_FFFF;
    println!("git: blobs= {} trees= {} commits= {} head= {} checksum= {}", blobs.len(), trees.len(), commits.len(), last_commit, checksum);
}
