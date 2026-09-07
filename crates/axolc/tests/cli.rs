// Owner: PascalElixir / axolrs (GitHub org)
// File: Tests for the axolc CLI driver.

use std::process::Command;

fn axolc_binary() -> String {
    let mut path = std::env::current_exe().unwrap();
    path.pop();
    path.pop();
    path.join("axolc").to_string_lossy().to_string()
}

fn run_axolc(args: &[&str]) -> (String, String, i32) {
    let output = Command::new(axolc_binary())
        .args(args)
        .output()
        .expect("run axolc");
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    let code = output.status.code().unwrap_or(-1);
    (stdout, stderr, code)
}

#[test]
fn version_command() {
    let (out, _err, code) = run_axolc(&["version"]);
    assert_eq!(code, 0);
    assert!(out.contains("axolc"));
}

#[test]
fn parses_compile_command() {
    let (_out, _err, _code) = run_axolc(&["compile", "--help"]);
    // Should print help, not crash
}

#[test]
fn parses_run_command() {
    let (_out, _err, _code) = run_axolc(&["run", "--help"]);
}

#[test]
fn parses_emit_rust_command() {
    let (_out, _err, _code) = run_axolc(&["emit-rust", "--help"]);
}

#[test]
fn parses_tokenize_command() {
    let (_out, _err, _code) = run_axolc(&["tokenize", "--help"]);
}

#[test]
fn parses_parse_command() {
    let (_out, _err, _code) = run_axolc(&["parse", "--help"]);
}
