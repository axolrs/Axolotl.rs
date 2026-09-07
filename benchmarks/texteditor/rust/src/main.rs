// Owner: PascalElixir / axolrs (GitHub org)
// File: benchmarks/texteditor/rust/src/main.rs - kilo-style row buffer with insert/delete/split.
//
// Reference. Mirrors ../axol/src/main.axol exactly. Uses the same `handled` flag
// dispatch pattern as the axol port.

use std::env;

const DEFAULT_WORKLOAD: i64 = 2000;
const ROWS: i64 = 10_000;

fn main() {
    let workload: i64 = env::args().nth(1).and_then(|s| s.parse::<i64>().ok()).unwrap_or(DEFAULT_WORKLOAD);
    let mut row_lens: Vec<i64> = Vec::new();
    let mut row_hashes: Vec<i64> = Vec::new();
    let mut i: i64 = 0;
    while i < ROWS {
        row_lens.push(16);
        row_hashes.push(i * 31 + 7);
        i += 1;
    }
    i = 0;
    while i < workload {
        let r = i % ROWS;
        let c = i % 80;
        let m = i % 5;
        let mut handled: i64 = 0;
        if m == 0 {
            row_lens[r as usize] += 1;
            row_hashes[r as usize] = row_hashes[r as usize] * 31 + 120;
            handled = 1;
        }
        if handled == 0 {
            if m == 1 {
                if row_lens[r as usize] > c {
                    row_lens[r as usize] -= 1;
                    row_hashes[r as usize] /= 31;
                }
                handled = 1;
            }
        }
        if handled == 0 {
            if m == 2 {
                let n = row_lens[r as usize];
                let mut k = c;
                if k > n { k = n; }
                row_lens[r as usize] = k;
                row_lens.push(n - k);
                row_hashes.push(row_hashes[r as usize] * 7);
                handled = 1;
            }
        }
        if handled == 0 {
            if m == 3 {
                if row_lens.len() > 1 {
                    row_lens[r as usize] = 0;
                    row_hashes[r as usize] = 0;
                }
                handled = 1;
            }
        }
        if handled == 0 {
            row_lens[r as usize] += 1;
            row_hashes[r as usize] = row_hashes[r as usize] * 31 + 122;
        }
        i += 1;
    }
    let mut total_bytes: i64 = 0;
    let mut h: i64 = 0;
    let mut k: i64 = 0;
    while k < row_lens.len() as i64 {
        total_bytes += row_lens[k as usize];
        h = h.wrapping_mul(31).wrapping_add(row_hashes[k as usize]);
        k += 1;
    }
    println!("{} {} {} {} {} {} {} {}", "texteditor: ops=", workload, "rows=", row_lens.len(), "bytes=", total_bytes, "checksum=", h);
}
