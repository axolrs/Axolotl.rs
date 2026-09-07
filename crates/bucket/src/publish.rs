// Owner: PascalElixir / axolrs (GitHub org)
// File: crates/bucket/src/publish.rs - the Eggbox publisher: packages the generated Rust crate for crates.io.

use crate::build::{build, find_tool};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Publish the project's generated crate via Eggbox: release build, package, then hand off.
pub fn publish() {
    if !Path::new(".axol-gen/Cargo.toml").is_file() {
        println!("Eggbox: no release build found; building the project first");
        build(true);
    }
    let cargo = find_tool("cargo");
    println!("Eggbox: packaging .axol-gen (cargo package --allow-dirty)");
    let status = Command::new(&cargo)
        .arg("package")
        .arg("--allow-dirty")
        .current_dir(".axol-gen")
        .status();
    let code = match status {
        Ok(s) if s.success() => 0,
        Ok(s) => s.code().unwrap_or(1),
        Err(e) => {
            eprintln!("Eggbox: failed to run cargo: {}", e);
            1
        }
    };
    if code != 0 {
        std::process::exit(code);
    }
    let crates = find_crate_files(Path::new(".axol-gen/target/package"));
    if crates.is_empty() {
        eprintln!("Eggbox: cargo package reported success but no .crate file was found");
        std::process::exit(1);
    }
    for c in &crates {
        println!("Eggbox: packaged {}", c.display());
    }
    println!("{}", final_step_hint());
}

/// Locate the .crate artifacts produced by cargo package under a package directory, sorted by path.
pub fn find_crate_files(package_dir: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    if let Ok(entries) = fs::read_dir(package_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() && path.extension().map_or(false, |e| e == "crate") {
                out.push(path);
            }
        }
    }
    out.sort();
    out
}

/// The guidance line printed after a successful package.
pub fn final_step_hint() -> String {
    "Eggbox: final step (requires a crates.io login): run `cargo publish --allow-dirty` inside .axol-gen".to_string()
}
