// Owner: PascalElixir / axolrs (GitHub org)
// File: benchmarks/grep/rust/src/main.rs - integer subsequence matcher.
//
// Reference. Mirrors ../axol/src/main.axol exactly.

use std::env;

const DEFAULT_WORKLOAD: i64 = 200_000;

fn line_byte(line_id: i64, idx: i64) -> i64 {
    let lines: [[i64; 10]; 8] = [
        [116, 104, 101, 32, 113, 117, 105, 99, 107, 0],
        [114, 117, 115, 116, 32, 105, 115, 32, 97, 0],
        [97, 120, 111, 108, 111, 116, 108, 32, 0, 0],
        [108, 117, 97, 32, 102, 108, 97, 118, 111, 114],
        [99, 97, 114, 103, 111, 32, 101, 99, 111, 0],
        [110, 111, 32, 114, 117, 110, 116, 105, 109, 101],
        [115, 117, 98, 32, 50, 48, 48, 109, 115, 0],
        [115, 109, 97, 114, 116, 32, 99, 111, 109, 112],
    ];
    lines[line_id as usize][idx as usize]
}

fn line_len(line_id: i64) -> i64 {
    [9, 9, 8, 10, 9, 10, 9, 10][line_id as usize]
}

fn pat_byte(pat_id: i64, idx: i64) -> i64 {
    let patterns: [[i64; 5]; 5] = [
        [114, 117, 115, 116, 0],
        [97, 120, 111, 108, 0],
        [116, 104, 101, 0, 0],
        [99, 97, 114, 103, 111],
        [99, 111, 109, 112, 0],
    ];
    patterns[pat_id as usize][idx as usize]
}

fn pat_len(pat_id: i64) -> i64 {
    [4, 4, 3, 5, 4][pat_id as usize]
}

fn match_pattern(line_id: i64, pat_id: i64) -> bool {
    let llen = line_len(line_id);
    let plen = pat_len(pat_id);
    if plen == 0 { return true; }
    let mut i: i64 = 0;
    while i < llen {
        if i + plen > llen { break; }
        let mut ok = true;
        let mut j: i64 = 0;
        while j < plen {
            if line_byte(line_id, i + j) != pat_byte(pat_id, j) { ok = false; break; }
            j += 1;
        }
        if ok { return true; }
        i += 1;
    }
    false
}

fn main() {
    let workload: i64 = env::args().nth(1).and_then(|s| s.parse::<i64>().ok()).unwrap_or(DEFAULT_WORKLOAD);
    let mut total_matches: i64 = 0;
    let mut i: i64 = 0;
    while i < workload {
        let line_id = i % 8;
        let pat_id = i % 5;
        if match_pattern(line_id, pat_id) { total_matches += 1; }
        i += 1;
    }
    println!("{} {} {} {}", "grep: lines=", workload, "matches=", total_matches);
}
