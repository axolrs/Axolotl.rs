// Owner: PascalElixir / axolrs (GitHub org)
// File: crates/bucket/tests/cli.rs - end-to-end CLI tests for the commands with stable behavior.

use std::process::Command;

fn bucket_binary() -> String {
    let mut path = std::env::current_exe().unwrap();
    path.pop();
    path.pop();
    path.join("bucket").to_string_lossy().to_string()
}

fn run_bucket(args: &[&str]) -> (String, String, i32) {
    let output = Command::new(bucket_binary())
        .args(args)
        .output()
        .expect("run bucket");
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    let code = output.status.code().unwrap_or(-1);
    (stdout, stderr, code)
}

#[test]
fn doctor_command() {
    let (out, _err, code) = run_bucket(&["doctor"]);
    assert_eq!(code, 0);
    assert!(out.contains("Bucket doctor"));
}

#[test]
fn clean_command() {
    let (out, _err, code) = run_bucket(&["clean"]);
    assert_eq!(code, 0);
    assert!(out.contains("Cleaned"));
}

#[test]
fn new_scaffolds_a_project() {
    let dir = std::env::temp_dir().join(format!("bucket-new-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    let name = dir.join("proj");
    let (out, _err, code) = run_bucket(&["new", name.to_str().unwrap()]);
    assert_eq!(code, 0);
    assert!(out.contains("Created new Axolotl project"));
    assert!(name.join("Bucket.jsonc").is_file());
    assert!(name.join("src/main.axol").is_file());
    assert!(name.join("src/lib.axol").is_file());
    assert!(name.join("tests").is_dir());
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn script_reports_missing_entry() {
    let dir = std::env::temp_dir().join(format!("bucket-script-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    let output = Command::new(bucket_binary())
        .arg("script")
        .arg("nope")
        .current_dir(&dir)
        .output()
        .expect("run bucket script");
    let err = String::from_utf8_lossy(&output.stderr).to_string();
    assert!(err.contains("not found in Bucket.jsonc"));
    let _ = std::fs::remove_dir_all(&dir);
}
