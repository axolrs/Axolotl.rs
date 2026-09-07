// Owner: PascalElixir / axolrs (GitHub org)
// File: benchmarks/physics/rust/src/main.rs - 2D N-body gravity simulator (integer math, scaled)
//
// Reference implementation. Mirrors ../axol/src/main.axol exactly;
// the bench harness in ../bench/run.sh asserts byte-identical stdout
// and axol_time / rust_time <= 1.05.

use std::env;

const DEFAULT_WORKLOAD: i64 = 100_000;
const N_BODIES: i64 = 50;
const SCALE: i64 = 1000;

fn main() {
    let workload: i64 = env::args().nth(1).and_then(|s| s.parse::<i64>().ok()).unwrap_or(DEFAULT_WORKLOAD);
    let mut bx: Vec<i64> = Vec::new();
    let mut by: Vec<i64> = Vec::new();
    let mut bvx: Vec<i64> = Vec::new();
    let mut bvy: Vec<i64> = Vec::new();
    let mut bmass: Vec<i64> = Vec::new();
    let mut i: i64 = 0;
    while i < N_BODIES {
        let angle = (i * 31416) / N_BODIES;
        let r = 1000 + i * 50;
        bx.push((r * (angle - 7854)) / 5000);
        by.push((r * (31416 - angle)) / 5000);
        bvx.push(-(angle / 100) - 5);
        bvy.push((angle / 100) + 5);
        bmass.push(1000 + i * 100);
        i += 1;
    }
    let mut step: i64 = 0;
    while step < workload {
        let mut i: i64 = 0;
        while i < N_BODIES {
            let mut fx: i64 = 0;
            let mut fy: i64 = 0;
            let xi = bx[i as usize];
            let yi = by[i as usize];
            let mi = bmass[i as usize];
            let mut j: i64 = 0;
            while j < N_BODIES {
                if i != j {
                    let dx = bx[j as usize] - xi;
                    let dy = by[j as usize] - yi;
                    let d2 = dx * dx + dy * dy + 1;
                    let f = bmass[j as usize] * mi / d2;
                    fx += dx * f / d2;
                    fy += dy * f / d2;
                }
                j += 1;
            }
            bvx[i as usize] += fx / mi * 10 / SCALE;
            bvy[i as usize] += fy / mi * 10 / SCALE;
            i += 1;
        }
        let mut i: i64 = 0;
        while i < N_BODIES {
            bx[i as usize] += bvx[i as usize] / SCALE;
            by[i as usize] += bvy[i as usize] / SCALE;
            i += 1;
        }
        step += 1;
    }
    let mut energy: i64 = 0;
    let mut i: i64 = 0;
    while i < N_BODIES {
        energy += bmass[i as usize] * (bvx[i as usize] * bvx[i as usize] + bvy[i as usize] * bvy[i as usize]) / 2;
        i += 1;
    }
    println!("{} {} {} {} {} {}", "physics: steps=", workload, "bodies=", N_BODIES, "energy=", energy);
}
