// Owner: PascalElixir / axolrs (GitHub org)
// File: benchmarks/bittorrent/rust/src/main.rs - piece-hashing swarm tracker with precomputed bitmask lookup
//
// Reference implementation. Mirrors ../axol/src/main.axol exactly.

use std::env;

const DEFAULT_WORKLOAD: i64 = 500_000;
const N_PIECES: i64 = 256;
const MAX_BITS: i64 = 64;

fn main() {
    let workload: i64 = env::args().nth(1).and_then(|s| s.parse::<i64>().ok()).unwrap_or(DEFAULT_WORKLOAD);
    // Precompute powers of 2
    let mut pow2: Vec<i64> = Vec::with_capacity(MAX_BITS as usize);
    let mut p: i64 = 1;
    let mut i: i64 = 0;
    while i < MAX_BITS {
        pow2.push(p);
        p = p * 2;
        i += 1;
    }
    let mut p_hash: Vec<i64> = Vec::new();
    let mut p_have: Vec<i64> = Vec::new();
    let mut p_peer_count: Vec<i64> = Vec::new();
    i = 0;
    while i < N_PIECES {
        p_hash.push(i.wrapping_mul(2654435761));
        p_have.push(0);
        p_peer_count.push(0);
        i += 1;
    }
    let mut peer_next: i64 = 1;
    let mut completed: i64 = 0;
    i = 0;
    while i < workload {
        let peer = peer_next;
        peer_next += 1;
        let have_mask = i.wrapping_mul(7);
        let mut j: i64 = 0;
        while j < N_PIECES {
            let bit = j % 64;
            if (have_mask & pow2[bit as usize]) != 0 {
                p_peer_count[j as usize] += 1;
            }
            j += 1;
        }
        j = 0;
        while j < N_PIECES {
            if p_have[j as usize] == 0 && p_peer_count[j as usize] > 0 {
                p_have[j as usize] = 1;
                completed += 1;
            }
            j += 1;
        }
        i += 1;
    }
    let mut have_count: i64 = 0;
    let mut j: i64 = 0;
    while j < p_have.len() as i64 {
        if p_have[j as usize] == 1 { have_count += 1; }
        j += 1;
    }
    println!("{} {} {} {} {} {}", "bittorrent: ops=", workload, "peers=", peer_next - 1, "pieces=", N_PIECES);
}
