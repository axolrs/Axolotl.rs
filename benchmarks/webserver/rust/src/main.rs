// Owner: PascalElixir / axolrs (GitHub org)
// File: benchmarks/webserver/rust/src/main.rs - HTTP/1.1 request dispatch simulator
//
// Reference implementation. Mirrors ../axol/src/main.axol exactly;
// the bench harness in ../bench/run.sh asserts byte-identical stdout
// and axol_time / rust_time <= 1.05.

use std::env;

const DEFAULT_WORKLOAD: i64 = 100_000;

fn main() {
    let workload: i64 = env::args().nth(1).and_then(|s| s.parse::<i64>().ok()).unwrap_or(DEFAULT_WORKLOAD);
    let route_hashes: Vec<i64> = vec![100, 200, 300, 400, 500];
    let route_status: Vec<i64> = vec![200, 200, 200, 200, 404];
    let path_hashes: Vec<i64> = vec![100, 200, 300, 500, 400];
    let mut hits_200: i64 = 0;
    let mut hits_404: i64 = 0;
    let mut total_body: i64 = 0;
    let mut i: i64 = 0;
    while i < workload {
        let ph = path_hashes[(i % 5) as usize];
        let mut status: i64 = 404;
        let mut j: i64 = 0;
        while j < route_hashes.len() as i64 {
            if route_hashes[j as usize] == ph { status = route_status[j as usize]; break; }
            j += 1;
        }
        let body_len: i64 = if status == 200 { 2 } else if status == 404 { 8 } else if status == 500 { 12 } else { 7 };
        if status == 200 { hits_200 += 1; }
        if status == 404 { hits_404 += 1; }
        total_body += body_len + (i % 5) + (i % 64);
        i += 1;
    }
    println!("{} {} {} {} {} {} {} {}", "webserver: requests=", workload, "ok=", hits_200, "notfound=", hits_404, "bytes=", total_body);
}
