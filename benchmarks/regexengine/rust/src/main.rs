// Owner: PascalElixir / axolrs (GitHub org)
// File: benchmarks/regexengine/rust/src/main.rs - NFA regex matcher with literal-char patterns.
//
// Reference. Mirrors ../axol/src/main.axol exactly. Pattern and subject
// bytes are looked up by ID (avoids moving out of Vec<Vec<i64>>).

use std::env;

const DEFAULT_WORKLOAD: i64 = 40;

fn subject_byte(sub_id: i64, idx: i64) -> i64 {
    match sub_id {
        0 => match idx { 0 => 97, 1 => 98, 2 => 99, _ => -1 },
        1 => match idx { 0 => 120, 1 => 121, 2 => 122, _ => -1 },
        2 => match idx { 0 => 104, 1 => 101, 2 => 108, 3 => 108, 4 => 111, _ => -1 },
        3 => match idx { 0 => 116, 1 => 101, 2 => 115, 3 => 116, _ => -1 },
        4 => match idx { 0 => 114, 1 => 117, 2 => 115, 3 => 116, _ => -1 },
        5 => match idx { 0 => 1, 1 => 2, 2 => 3, _ => -1 },
        6 => match idx { 0 => 97, 1 => 98, 2 => 99, 3 => 100, _ => -1 },
        7 => match idx { 0 => 104, 1 => 101, 2 => 108, 3 => 108, 4 => 111, 5 => 33, _ => -1 },
        _ => -1,
    }
}

fn subject_len(sub_id: i64) -> i64 {
    match sub_id {
        0 => 3, 1 => 3, 2 => 5, 3 => 4, 4 => 4, 5 => 3, 6 => 4, 7 => 6, _ => 0,
    }
}

fn pat_byte(pat_id: i64, idx: i64) -> i64 {
    match pat_id {
        0 => match idx { 0 => 97, 1 => 98, 2 => 99, _ => -1 },
        1 => match idx { 0 => 120, 1 => 121, 2 => 122, _ => -1 },
        2 => match idx { 0 => 104, 1 => 101, 2 => 108, 3 => 108, 4 => 111, _ => -1 },
        3 => match idx { 0 => 116, 1 => 101, 2 => 115, 3 => 116, _ => -1 },
        4 => match idx { 0 => 114, 1 => 117, 2 => 115, 3 => 116, _ => -1 },
        _ => -1,
    }
}

fn pat_len(pat_id: i64) -> i64 {
    match pat_id {
        0 => 3, 1 => 3, 2 => 5, 3 => 4, 4 => 4, _ => 0,
    }
}

fn match_at_subject(sub_id: i64, pat_id: i64) -> bool {
    let llen = subject_len(sub_id);
    let plen = pat_len(pat_id);
    if plen == 0 { return true; }
    let mut i: i64 = 0;
    while i < llen {
        if i + plen > llen { break; }
        let mut ok = true;
        let mut j: i64 = 0;
        while j < plen {
            if subject_byte(sub_id, i + j) != pat_byte(pat_id, j) { ok = false; break; }
            j += 1;
        }
        if ok { return true; }
        i += 1;
    }
    false
}

fn main() {
    let workload: i64 = env::args().nth(1).and_then(|s| s.parse::<i64>().ok()).unwrap_or(DEFAULT_WORKLOAD);
    let mut matches: i64 = 0;
    let mut runs: i64 = 0;
    let mut i: i64 = 0;
    while i < workload {
        let pat_id = i % 5;
        let mut s_idx: i64 = 0;
        while s_idx < 8 {
            runs += 1;
            if match_at_subject(s_idx, pat_id) { matches += 1; }
            s_idx += 1;
        }
        i += 1;
    }
    println!("{} {} {} {} {} {}", "regexengine: patterns=", workload, "runs=", runs, "matches=", matches);
}
