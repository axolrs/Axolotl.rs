// Owner: PascalElixir / axolrs (GitHub org)
// File: benchmarks/neuralnet/rust/src/main.rs - feedforward neural network with flattened weight arrays
//
// Reference implementation. Mirrors ../axol/src/main.axol exactly.

use std::env;

const DEFAULT_WORKLOAD: i64 = 50_000;
const N_IN: i64 = 8;
const N_HIDDEN: i64 = 16;
const N_OUT: i64 = 4;

fn hash_seed(s: i64) -> i64 {
    s.wrapping_mul(2654435761).wrapping_add(s / 65536)
}

fn main() {
    let workload: i64 = env::args().nth(1).and_then(|s| s.parse::<i64>().ok()).unwrap_or(DEFAULT_WORKLOAD);
    // Flatten weights: w1[i * N_IN + j] instead of w1[i][j]
    let mut w1: Vec<i64> = Vec::with_capacity((N_IN * N_HIDDEN) as usize);
    let mut b1: Vec<i64> = Vec::with_capacity(N_HIDDEN as usize);
    let mut w2: Vec<i64> = Vec::with_capacity((N_HIDDEN * N_OUT) as usize);
    let mut b2: Vec<i64> = Vec::with_capacity(N_OUT as usize);
    let mut i: i64 = 0;
    while i < N_HIDDEN {
        let mut j: i64 = 0;
        while j < N_IN {
            let s = hash_seed(42 + i * 31 + j);
            w1.push((s & 1023) - 512);
            j += 1;
        }
        b1.push((hash_seed(1000 + i) & 1023) - 512);
        i += 1;
    }
    i = 0;
    while i < N_OUT {
        let mut j: i64 = 0;
        while j < N_HIDDEN {
            let s = hash_seed(819 + i * 31 + j);
            w2.push((s & 1023) - 512);
            j += 1;
        }
        b2.push((hash_seed(2000 + i) & 1023) - 512);
        i += 1;
    }
    let mut input: Vec<i64> = vec![0; N_IN as usize];
    let mut hidden: Vec<i64> = vec![0; N_HIDDEN as usize];
    let mut sum: i64 = 0;
    let mut k: i64 = 0;
    while k < workload {
        let mut j: i64 = 0;
        while j < N_IN { input[j as usize] = (k + j) & 1023; j += 1; }
        let mut i2: i64 = 0;
        while i2 < N_HIDDEN {
            let mut acc = b1[i2 as usize];
            let base = i2 * N_IN;
            let mut j: i64 = 0;
            while j < N_IN {
                acc += w1[(base + j) as usize] * input[j as usize];
                j += 1;
            }
            if acc > 32767 { acc = 32767; }
            if acc < -32768 { acc = -32768; }
            hidden[i2 as usize] = acc;
            i2 += 1;
        }
        let mut i2: i64 = 0;
        while i2 < N_OUT {
            let mut acc = b2[i2 as usize];
            let base = i2 * N_HIDDEN;
            let mut j: i64 = 0;
            while j < N_HIDDEN {
                acc += w2[(base + j) as usize] * hidden[j as usize];
                j += 1;
            }
            sum += acc;
            i2 += 1;
        }
        k += 1;
    }
    println!("{} {} {} {} {} {}", "neuralnet: forwards=", workload, "sum=", sum, "in=", N_IN);
}
