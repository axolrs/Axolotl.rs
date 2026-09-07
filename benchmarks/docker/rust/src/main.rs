// Owner: PascalElixir / axolrs (GitHub org)
// File: benchmarks/docker/rust/src/main.rs - layer store + container lifecycle + cgroup accounting
//
// Reference implementation. Mirrors ../axol/src/main.axol exactly;
// the bench harness in ../bench/run.sh asserts byte-identical stdout
// and axol_time / rust_time <= 1.05.

use std::env;

const DEFAULT_WORKLOAD: i64 = 50_000;

fn main() {
    let workload: i64 = env::args().nth(1).and_then(|s| s.parse::<i64>().ok()).unwrap_or(DEFAULT_WORKLOAD);
    let mut layer_hash: Vec<i64> = Vec::new();
    let mut layer_parent: Vec<i64> = Vec::new();
    let mut image_top: Vec<i64> = Vec::new();
    let mut c_pid: Vec<i64> = Vec::new();
    let mut c_mem: Vec<i64> = Vec::new();
    let mut c_alive: Vec<i64> = Vec::new();
    let mut pids: Vec<i64> = Vec::new();
    let mut next_pid: i64 = 1;
    let mut oom_kills: i64 = 0;
    let mut i: i64 = 0;
    while i < workload {
        let parent: i64 = if i % 7 == 0 { -1 } else { (i - 1) / 2 };
        let content_hash = i.wrapping_mul(2654435761);
        layer_hash.push(content_hash);
        layer_parent.push(parent);
        let layer_idx = layer_hash.len() as i64 - 1;
        if i % 10 == 0 {
            image_top.push(layer_idx);
            let pid = next_pid;
            next_pid += 1;
            c_pid.push(pid);
            c_mem.push(1024);
            c_alive.push(1);
            pids.push(pid);
        }
        if !pids.is_empty() {
            let pid = pids[(i % pids.len() as i64) as usize];
            let mut j: i64 = 0;
            while j < c_pid.len() as i64 {
                if c_pid[j as usize] == pid && c_alive[j as usize] == 1 {
                    c_mem[j as usize] += i * 8;
                    if c_mem[j as usize] > 1048576 { c_alive[j as usize] = 0; oom_kills += 1; }
                    break;
                }
                j += 1;
            }
        }
        i += 1;
    }
    let mut chain_sum: i64 = 0;
    let mut k: i64 = 0;
    while k < image_top.len() as i64 {
        let mut idx = image_top[k as usize];
        while idx >= 0 {
            chain_sum = chain_sum.wrapping_add(layer_hash[idx as usize]);
            idx = layer_parent[idx as usize];
        }
        k += 1;
    }
    let mut alive: i64 = 0;
    k = 0;
    while k < c_alive.len() as i64 {
        if c_alive[k as usize] == 1 { alive += 1; }
        k += 1;
    }
    println!("{} {} {} {} {} {} {} {} {} {} {} {}", "docker: ops=", workload, "layers=", layer_hash.len(), "images=", image_top.len(), "containers=", alive, "oom_kills=", oom_kills, "chain=", chain_sum);
}
