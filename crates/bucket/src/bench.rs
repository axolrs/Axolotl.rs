// Owner: PascalElixir / axolrs (GitHub org)
// File: crates/bucket/src/bench.rs - the Salamander benchmark runner for .axol projects.

use crate::build::find_tool;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{Duration, Instant};

/// Number of interpreted runs per bench file in project mode.
pub const BENCH_RUNS: u32 = 50;

/// Run project benchmarks with Salamander: repo port harnesses or interpreted benches.
pub fn bench() {
    let dir = std::env::current_dir().unwrap();
    let benchmarks = dir.join("benchmarks");
    if benchmarks.is_dir() {
        bench_ports(&benchmarks);
    } else {
        bench_project(&dir);
    }
}

/// Run `cargo run --release` inside every benchmarks/*/bench port harness, exiting non-zero on failure.
fn bench_ports(root: &Path) {
    let ports = port_dirs(root);
    if ports.is_empty() {
        println!("Salamander: no benchmarks found");
        return;
    }
    let cargo = find_tool("cargo");
    let mut failed: Vec<String> = Vec::new();
    for (name, bench_dir) in &ports {
        println!("Salamander: running port {} (cargo run --release)", name);
        let status = Command::new(&cargo)
            .arg("run")
            .arg("--release")
            .current_dir(bench_dir)
            .status();
        match status {
            Ok(s) if s.success() => println!("Salamander: port {} passed", name),
            Ok(s) => {
                eprintln!("Salamander: port {} FAILED (exit {:?})", name, s.code());
                failed.push(name.clone());
            }
            Err(e) => {
                eprintln!("Salamander: port {} could not run: {}", name, e);
                failed.push(name.clone());
            }
        }
    }
    println!("Salamander: {}/{} ports passed", ports.len() - failed.len(), ports.len());
    if !failed.is_empty() {
        std::process::exit(1);
    }
}

/// Time N interpreted runs per benches/*.axol file and report per-file means.
fn bench_project(dir: &Path) {
    let files = find_bench_files(dir);
    if files.is_empty() {
        println!("Salamander: no benchmarks found");
        return;
    }
    let mut failed = false;
    for f in &files {
        let src = fs::read_to_string(f).unwrap_or_default();
        let rel = relative_path_string(dir, f);
        let start = Instant::now();
        let mut had_errors = false;
        for _ in 0..BENCH_RUNS {
            let (_out, diags) = axolc_core::interpret(&src, 0);
            if diags.has_errors() {
                had_errors = true;
                break;
            }
        }
        if had_errors {
            eprintln!("Salamander: {} failed to interpret", rel);
            failed = true;
            continue;
        }
        let mean = mean_us(start.elapsed(), BENCH_RUNS);
        println!("{}", bench_file_line(&rel, mean, BENCH_RUNS));
    }
    if failed {
        std::process::exit(1);
    }
}

/// List the benches/*.axol files of a project, sorted by path.
pub fn find_bench_files(dir: &Path) -> Vec<PathBuf> {
    let benches = dir.join("benches");
    let mut out = Vec::new();
    if let Ok(entries) = fs::read_dir(&benches) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() && path.extension().map_or(false, |e| e == "axol") {
                out.push(path);
            }
        }
    }
    out.sort();
    out
}

/// List the port harness directories (name, benchmarks/name/bench), sorted by name.
pub fn port_dirs(root: &Path) -> Vec<(String, PathBuf)> {
    let mut out = Vec::new();
    if let Ok(entries) = fs::read_dir(root) {
        for entry in entries.flatten() {
            let path = entry.path();
            let bench_dir = path.join("bench");
            if path.is_dir() && bench_dir.is_dir() {
                let name = path.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default();
                out.push((name, bench_dir));
            }
        }
    }
    out.sort();
    out
}

/// Compute the mean duration of one run in microseconds.
pub fn mean_us(total: Duration, runs: u32) -> f64 {
    if runs == 0 {
        return 0.0;
    }
    total.as_nanos() as f64 / 1000.0 / runs as f64
}

/// Render the per-file timing report line.
pub fn bench_file_line(rel: &str, mean_us: f64, runs: u32) -> String {
    format!("Salamander: {} mean {:.1} µs over {} runs", rel, mean_us, runs)
}

/// Render a path relative to root, falling back to the full display path.
pub fn relative_path_string(root: &Path, path: &Path) -> String {
    match path.strip_prefix(root) {
        Ok(rel) => rel.to_string_lossy().to_string(),
        Err(_) => path.display().to_string(),
    }
}
