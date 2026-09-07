// Owner: PascalElixir / axolrs (GitHub org)
// File: crates/bucket/src/upgrade.rs - the Molt migrator: toolchain upgrades and project migration.

use crate::build::{build, find_tool, load_manifest};
use crate::Manifest;
use std::io::ErrorKind;
use std::path::Path;
use std::process::Command;

/// Upgrade the toolchain and migrate the project with Molt.
pub fn upgrade() {
    println!("Molt: toolchain upgrade and project migration");
    let rustc = find_tool("rustc");
    println!("Molt: current toolchain: {}", tool_output(&rustc, &["--version"]));
    match Command::new("rustup").arg("update").arg("stable").status() {
        Ok(s) if s.success() => println!("Molt: rustup update stable completed"),
        Ok(s) => eprintln!("Molt: rustup update stable failed (exit {:?})", s.code()),
        Err(e) if e.kind() == ErrorKind::NotFound => {
            println!("Molt: rustup not found; skipping toolchain update")
        }
        Err(e) => eprintln!("Molt: could not run rustup: {}", e),
    }
    if Path::new("Bucket.jsonc").is_file() {
        println!("Molt: rebuilding the project in dev mode");
        build(false);
        println!("Molt: dev build succeeded");
        let manifest = load_manifest();
        println!("{}", migration_report(&manifest));
    } else {
        println!("Molt: no Bucket.jsonc; skipping project rebuild");
    }
}

/// Render the manifest migration report line.
pub fn migration_report(m: &Manifest) -> String {
    format!("Molt: manifest name={} version={} edition={}", m.name, m.version, m.language.edition)
}

/// Capture a tool's trimmed stdout for one invocation, or report it not found.
fn tool_output(tool: &str, args: &[&str]) -> String {
    Command::new(tool)
        .args(args)
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_else(|_| "not found".to_string())
}
