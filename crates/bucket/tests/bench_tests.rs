// Owner: PascalElixir / axolrs (GitHub org)
// File: crates/bucket/tests/bench_tests.rs - Salamander benchmark runner tests.

use bucket::bench::{bench_file_line, find_bench_files, mean_us, port_dirs, BENCH_RUNS};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Duration;

fn bucket_binary() -> String {
    let mut path = std::env::current_exe().unwrap();
    path.pop();
    path.pop();
    path.join("bucket").to_string_lossy().to_string()
}

/// Run the bucket binary with arguments in a working directory.
fn run_bucket_in(dir: &Path, args: &[&str]) -> (String, String, i32) {
    let output = Command::new(bucket_binary())
        .args(args)
        .current_dir(dir)
        .output()
        .expect("run bucket");
    (
        String::from_utf8_lossy(&output.stdout).to_string(),
        String::from_utf8_lossy(&output.stderr).to_string(),
        output.status.code().unwrap_or(-1),
    )
}

/// Create a unique temp directory for a test scenario.
fn temp_dir(tag: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("bucket-bench-{}-{}", tag, std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    dir
}

/// The number of interpreted runs per bench file.
#[test]
fn bench_runs_constant() {
    assert_eq!(BENCH_RUNS, 50);
}

/// The per-file timing line format is stable.
#[test]
fn bench_file_line_table() {
    assert_eq!(
        bench_file_line("benches/fib.axol", 123.4, 50),
        "Salamander: benches/fib.axol mean 123.4 µs over 50 runs"
    );
    assert_eq!(
        bench_file_line("benches/a.axol", 0.0, 50),
        "Salamander: benches/a.axol mean 0.0 µs over 50 runs"
    );
    assert_eq!(
        bench_file_line("benches/a.axol", 1.05, 50),
        "Salamander: benches/a.axol mean 1.1 µs over 50 runs"
    );
    assert_eq!(
        bench_file_line("benches/a.axol", 1.04, 50),
        "Salamander: benches/a.axol mean 1.0 µs over 50 runs"
    );
    assert_eq!(
        bench_file_line("benches/a.axol", 999999.99, 50),
        "Salamander: benches/a.axol mean 1000000.0 µs over 50 runs"
    );
    assert_eq!(
        bench_file_line("nested/dir/b.axol", 42.0, 50),
        "Salamander: nested/dir/b.axol mean 42.0 µs over 50 runs"
    );
    assert!(bench_file_line("benches/x.axol", 5.0, 50).starts_with("Salamander: "));
    assert!(bench_file_line("benches/x.axol", 5.0, 50).ends_with("over 50 runs"));
    assert!(bench_file_line("benches/x.axol", 5.0, 50).contains("µs"));
}

/// The mean computation divides total microseconds by the run count.
#[test]
fn mean_us_table() {
    assert_eq!(mean_us(Duration::from_micros(500), 50), 10.0);
    assert_eq!(mean_us(Duration::from_micros(0), 50), 0.0);
    assert_eq!(mean_us(Duration::from_micros(1), 50), 0.02);
    assert_eq!(mean_us(Duration::from_millis(1), 100), 10.0);
    assert_eq!(mean_us(Duration::from_secs(1), 1_000_000), 1.0);
    assert_eq!(mean_us(Duration::from_secs(2), 4), 500_000.0);
    assert_eq!(mean_us(Duration::from_micros(250), 0), 0.0);
    assert_eq!(mean_us(Duration::from_nanos(500), 1), 0.5);
}

/// find_bench_files collects only top-level .axol files, sorted.
#[test]
fn find_bench_files_table() {
    let dir = temp_dir("find");
    assert!(find_bench_files(&dir).is_empty());
    let benches = dir.join("benches");
    fs::create_dir_all(&benches).unwrap();
    assert!(find_bench_files(&dir).is_empty());
    fs::write(benches.join("b.axol"), "fn main()\nend\n").unwrap();
    fs::write(benches.join("a.axol"), "fn main()\nend\n").unwrap();
    fs::write(benches.join("c.txt"), "not a bench").unwrap();
    fs::write(benches.join("d.md"), "not a bench").unwrap();
    fs::create_dir_all(benches.join("nested")).unwrap();
    fs::write(benches.join("nested/e.axol"), "fn main()\nend\n").unwrap();
    let found = find_bench_files(&dir);
    assert_eq!(found.len(), 2);
    assert_eq!(found[0], benches.join("a.axol"));
    assert_eq!(found[1], benches.join("b.axol"));
    assert!(found.iter().all(|f| f.extension().map_or(false, |e| e == "axol")));
    let _ = fs::remove_dir_all(&dir);
}

/// port_dirs lists only directories that contain a bench subdirectory, sorted by name.
#[test]
fn port_dirs_table() {
    let dir = temp_dir("ports");
    assert!(port_dirs(&dir).is_empty());
    let zport = dir.join("benchmarks/zeta");
    let aport = dir.join("benchmarks/alpha");
    let plain = dir.join("benchmarks/plain");
    fs::create_dir_all(zport.join("bench")).unwrap();
    fs::create_dir_all(aport.join("bench")).unwrap();
    fs::create_dir_all(plain).unwrap();
    let ports = port_dirs(&dir.join("benchmarks"));
    assert_eq!(ports.len(), 2);
    assert_eq!(ports[0].0, "alpha");
    assert_eq!(ports[1].0, "zeta");
    assert_eq!(ports[0].1, aport.join("bench"));
    assert_eq!(ports[1].1, zport.join("bench"));
    let empty_root = dir.join("benchmarks2");
    fs::create_dir_all(empty_root.join("x")).unwrap();
    assert!(port_dirs(&empty_root).is_empty());
    let _ = fs::remove_dir_all(&dir);
}

/// A project with a benches file gets a timing report and exit zero.
#[test]
fn cli_bench_reports_timings_for_a_project() {
    let dir = temp_dir("project");
    fs::write(
        dir.join("Bucket.jsonc"),
        "{ \"name\": \"b\", \"version\": \"0.1.0\", \"dependencies\": {} }",
    )
    .unwrap();
    fs::create_dir_all(dir.join("benches")).unwrap();
    fs::write(
        dir.join("benches/count.axol"),
        "fn main()\n    var i = 0\n    var s = 0\n    while i < 100 do\n        s = s + i\n        i = i + 1\n    end\n    print(s)\nend\n",
    )
    .unwrap();
    let (out, err, code) = run_bucket_in(&dir, &["bench"]);
    assert_eq!(code, 0, "stderr: {err}");
    assert!(out.contains("Salamander: benches/count.axol mean "));
    assert!(out.contains("µs over 50 runs"));
    assert!(!out.contains("no benchmarks found"));
    let _ = fs::remove_dir_all(&dir);
}

/// A project with multiple bench files reports each one.
#[test]
fn cli_bench_reports_each_bench_file() {
    let dir = temp_dir("multi");
    fs::create_dir_all(dir.join("benches")).unwrap();
    fs::write(dir.join("benches/one.axol"), "fn main()\n    print(1)\nend\n").unwrap();
    fs::write(dir.join("benches/two.axol"), "fn main()\n    print(2)\nend\n").unwrap();
    let (out, _err, code) = run_bucket_in(&dir, &["bench"]);
    assert_eq!(code, 0);
    assert!(out.contains("benches/one.axol mean"));
    assert!(out.contains("benches/two.axol mean"));
    let _ = fs::remove_dir_all(&dir);
}

/// An empty project honestly reports that there is nothing to bench.
#[test]
fn cli_bench_reports_nothing_to_bench() {
    let dir = temp_dir("empty");
    fs::write(
        dir.join("Bucket.jsonc"),
        "{ \"name\": \"b\", \"version\": \"0.1.0\", \"dependencies\": {} }",
    )
    .unwrap();
    let (out, err, code) = run_bucket_in(&dir, &["bench"]);
    assert_eq!(code, 0, "stderr: {err}");
    assert!(out.contains("Salamander: no benchmarks found"));
    let _ = fs::remove_dir_all(&dir);
}

/// A bench file that fails to interpret fails the run.
#[test]
fn cli_bench_fails_on_uninterpretable_bench() {
    let dir = temp_dir("broken");
    fs::create_dir_all(dir.join("benches")).unwrap();
    fs::write(dir.join("benches/bad.axol"), "fn main()\n    let x =\nend\n").unwrap();
    let (_out, err, code) = run_bucket_in(&dir, &["bench"]);
    assert_eq!(code, 1);
    assert!(err.contains("Salamander: benches/bad.axol failed to interpret"));
    let _ = fs::remove_dir_all(&dir);
}

/// Repo mode runs cargo in each port harness and passes when the port passes.
#[test]
fn cli_bench_repo_mode_runs_ports() {
    let dir = temp_dir("repo-ok");
    let bench_dir = dir.join("benchmarks/tiny/bench/src");
    fs::create_dir_all(&bench_dir).unwrap();
    fs::write(
        dir.join("benchmarks/tiny/bench/Cargo.toml"),
        "[package]\nname = \"tiny-bench\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\n[workspace]\n",
    )
    .unwrap();
    fs::write(
        bench_dir.join("main.rs"),
        "fn main() {\n    println!(\"tiny port ok\");\n}\n",
    )
    .unwrap();
    let (out, err, code) = run_bucket_in(&dir, &["bench"]);
    assert_eq!(code, 0, "stderr: {err}");
    assert!(out.contains("Salamander: running port tiny"));
    assert!(out.contains("tiny port ok"));
    assert!(out.contains("Salamander: port tiny passed"));
    assert!(out.contains("Salamander: 1/1 ports passed"));
    let _ = fs::remove_dir_all(&dir);
}

/// Repo mode exits non-zero when a port harness fails its own assertion.
#[test]
fn cli_bench_repo_mode_fails_when_a_port_fails() {
    let dir = temp_dir("repo-fail");
    let bench_dir = dir.join("benchmarks/broken/bench/src");
    fs::create_dir_all(&bench_dir).unwrap();
    fs::write(
        dir.join("benchmarks/broken/bench/Cargo.toml"),
        "[package]\nname = \"broken-bench\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\n[workspace]\n",
    )
    .unwrap();
    fs::write(
        bench_dir.join("main.rs"),
        "fn main() {\n    println!(\"broken port done\");\n    std::process::exit(7);\n}\n",
    )
    .unwrap();
    let (out, err, code) = run_bucket_in(&dir, &["bench"]);
    assert_eq!(code, 1);
    assert!(out.contains("Salamander: running port broken"));
    assert!(err.contains("Salamander: port broken FAILED"));
    assert!(out.contains("Salamander: 0/1 ports passed"));
    let _ = fs::remove_dir_all(&dir);
}

/// Repo mode with an empty benchmarks directory reports no benchmarks.
#[test]
fn cli_bench_repo_mode_empty_root() {
    let dir = temp_dir("repo-empty");
    fs::create_dir_all(dir.join("benchmarks")).unwrap();
    let (out, _err, code) = run_bucket_in(&dir, &["bench"]);
    assert_eq!(code, 0);
    assert!(out.contains("Salamander: no benchmarks found"));
    let _ = fs::remove_dir_all(&dir);
}

/// A benchmarks directory without harness subdirectories reports no benchmarks.
#[test]
fn cli_bench_repo_mode_without_bench_dirs() {
    let dir = temp_dir("repo-plain");
    fs::create_dir_all(dir.join("benchmarks/docs")).unwrap();
    fs::write(dir.join("benchmarks/docs/README.md"), "# notes\n").unwrap();
    let (out, _err, code) = run_bucket_in(&dir, &["bench"]);
    assert_eq!(code, 0);
    assert!(out.contains("Salamander: no benchmarks found"));
    let _ = fs::remove_dir_all(&dir);
}
