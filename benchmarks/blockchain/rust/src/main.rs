// Owner: PascalElixir / axolrs (GitHub org)
// File: benchmarks/blockchain/rust/src/main.rs - proof-of-work chain with mine + verify across 100k blocks
//
// Reference implementation. Mirrors ../axol/src/main.axol exactly;
// the bench harness in ../bench/run.sh asserts byte-identical stdout
// and axol_time / rust_time <= 1.05.

use std::env;

const DEFAULT_WORKLOAD: i64 = 2000;
const DIFFICULTY: i64 = 4;

fn hash_block(prev: i64, data: i64, nonce: i64) -> i64 {
    let mut h = prev + data * 2654435761 + nonce * 40503;
    h = h & 0x0FFF_FFFF_FFFF_FFFF;  // 60-bit positive
    let mut k: i64 = 0;
    while k < 5 {
        h = h + (h / 65536) * 31 + (h / 256) * 7;
        h = h & 0x0FFF_FFFF_FFFF_FFFF;
        k += 1;
    }
    h
}

fn main() {
    let workload: i64 = env::args().nth(1).and_then(|s| s.parse::<i64>().ok()).unwrap_or(DEFAULT_WORKLOAD);
    let mut b_data: Vec<i64> = vec![0];
    let mut b_nonce: Vec<i64> = vec![0];
    let mut b_hash: Vec<i64> = vec![0];
    let mut mask: i64 = 1;
    let mut k: i64 = 0;
    while k < DIFFICULTY { mask = mask * 2; k += 1; }
    mask -= 1;
    let mut i: i64 = 0;
    while i < workload {
        let prev = *b_hash.last().unwrap();
        let mut nonce: i64 = 0;
        loop {
            let h = hash_block(prev, i, nonce);
            if (h & mask) == 0 {
                b_data.push(i);
                b_nonce.push(nonce);
                b_hash.push(h);
                break;
            }
            nonce += 1;
        }
        i += 1;
    }
    let mut verified: i64 = 1;
    let mut i: i64 = 1;
    while i < b_data.len() as i64 {
        let prev = b_hash[(i - 1) as usize];
        let h = hash_block(prev, b_data[i as usize], b_nonce[i as usize]);
        if h != b_hash[i as usize] || (h & mask) != 0 { verified = 0; break; }
        i += 1;
    }
    println!("{} {} {} {} {} {}", "blockchain: blocks=", b_data.len(), "verified=", verified, "last=", *b_hash.last().unwrap());
}
