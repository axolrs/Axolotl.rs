// Owner: PascalElixir / axolrs (GitHub org)
// File: crates/bucket/src/run.rs - dev-mode execution, watch-mode handoff, and the test runner.

use crate::build::{collect_axol_files, find_sibling_tool, find_tool};
use std::fs;

/// Run the project: interpret src/main.axol in dev mode, or hand off to the hot runner with --watch.
pub fn run(watch: bool, args: &[String]) {
    let dir = std::env::current_dir().unwrap();
    let main_file = dir.join("src/main.axol");
    if !main_file.exists() {
        eprintln!("no src/main.axol found");
        std::process::exit(1);
    }
    if watch {
        let hot = find_sibling_tool("axol-hot-runner")
            .or_else(|| Some(find_tool("axol-hot-runner")))
            .unwrap();
        let status = std::process::Command::new(&hot)
            .arg("watch")
            .arg(&main_file)
            .status();
        match status {
            Ok(s) if s.success() => {}
            _ => std::process::exit(1),
        }
        return;
    }
    let src = fs::read_to_string(&main_file).unwrap();
    let (out, diags) = axolc_core::interpret(&src, 0);
    print!("{}", out);
    for d in &diags.items {
        if d.severity == axolc_core::diag::Severity::Error {
            eprintln!("{}", d);
        }
    }
    let _ = args;
}

/// Interpret every .axol file in the project and report the pass/error summary.
pub fn test_cmd() {
    let dir = std::env::current_dir().unwrap();
    let axol_files = collect_axol_files(&dir);
    let mut errors = 0;
    let mut passed = 0;
    for f in &axol_files {
        let src = fs::read_to_string(f).unwrap_or_default();
        let (out, diags) = axolc_core::interpret(&src, 0);
        let has_err = diags.has_errors();
        for d in &diags.items {
            if d.severity == axolc_core::diag::Severity::Error {
                eprintln!("{}: {}", f.display(), d);
                errors += 1;
            }
        }
        if !has_err {
            passed += 1;
            if !out.is_empty() {
                print!("{}", out);
            }
        }
    }
    println!("\nTest summary: {} passed, {} errors", passed, errors);
}
