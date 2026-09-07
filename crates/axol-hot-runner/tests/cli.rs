// Owner: PascalElixir / axolrs (GitHub org)
// File: Tests for the axol-hot-runner interpreter CLI.

use std::process::Command;

fn runner_binary() -> String {
    let mut path = std::env::current_exe().unwrap();
    path.pop();
    path.pop();
    path.join("axol-hot-runner").to_string_lossy().to_string()
}

fn run_runner(args: &[&str]) -> (String, String, i32) {
    let output = Command::new(runner_binary())
        .args(args)
        .output()
        .expect("run axol-hot-runner");
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    let code = output.status.code().unwrap_or(-1);
    (stdout, stderr, code)
}

#[test]
fn run_hello_world() {
    let tmp = std::env::temp_dir().join("axoltest_hello.axol");
    std::fs::write(&tmp, "fn main() print(\"hello\") end").unwrap();
    let (out, _err, code) = run_runner(&["run", tmp.to_str().unwrap()]);
    assert_eq!(code, 0);
    assert!(out.contains("hello"));
}

#[test]
fn run_arithmetic() {
    let tmp = std::env::temp_dir().join("axoltest_arith.axol");
    std::fs::write(&tmp, "fn main() print(2 + 3) end").unwrap();
    let (out, _err, _code) = run_runner(&["run", tmp.to_str().unwrap()]);
    assert!(out.contains("5"));
}

#[test]
fn run_loop() {
    let tmp = std::env::temp_dir().join("axoltest_loop.axol");
    std::fs::write(&tmp, "fn main() var i = 0 while i < 5 do print(i) i = i + 1 end end").unwrap();
    let (out, _err, _code) = run_runner(&["run", tmp.to_str().unwrap()]);
    assert!(out.contains("0"));
    assert!(out.contains("4"));
}

#[test]
fn run_function_call() {
    let tmp = std::env::temp_dir().join("axoltest_fn.axol");
    std::fs::write(&tmp, "fn double(x) return x * 2 end fn main() print(double(7)) end").unwrap();
    let (out, _err, _code) = run_runner(&["run", tmp.to_str().unwrap()]);
    assert!(out.contains("14"));
}

#[test]
fn run_if_else() {
    let tmp = std::env::temp_dir().join("axoltest_if.axol");
    std::fs::write(&tmp, "fn main() if true then print(\"yes\") else print(\"no\") end end").unwrap();
    let (out, _err, _code) = run_runner(&["run", tmp.to_str().unwrap()]);
    assert!(out.contains("yes"));
}

#[test]
fn watch_runs_initial() {
    let tmp = std::env::temp_dir().join("axoltest_watch.axol");
    std::fs::write(&tmp, "fn main() print(\"watched\") end").unwrap();
    let (out, _err, _code) = run_runner(&["watch", "--once", tmp.to_str().unwrap()]);
    assert!(out.contains("watched"));
}
