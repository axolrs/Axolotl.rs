// Owner: PascalElixir / axolrs (GitHub org)
// File: crates/bucket/src/build.rs - project compilation to native binaries and generated-Rust emission.

use std::fs;
use std::path::{Path, PathBuf};

/// Build the project: lower every .axol file to Rust and drive cargo to a native binary.
pub fn build(release: bool) {
    println!("Building project (release={})", release);
    let dir = std::env::current_dir().unwrap();
    let axol_files = collect_axol_files(&dir);
    if axol_files.is_empty() {
        eprintln!("no .axol files found");
        std::process::exit(1);
    }
    let manifest = fs::read_to_string(dir.join("Bucket.jsonc"))
        .ok()
        .and_then(|c| axolc_core::manifest::parse(&c).ok())
        .unwrap_or_default();
    let pkg_name = if manifest.name.is_empty() {
        "app".to_string()
    } else {
        manifest.name.replace(|c: char| !(c.is_alphanumeric() || c == '-' || c == '_'), "-")
    };
    let pkg_version = if manifest.version.is_empty() { "0.1.0".to_string() } else { manifest.version.clone() };
    let gen_dir = dir.join(".axol-gen");
    let src_dir = gen_dir.join("src");
    fs::create_dir_all(&src_dir).unwrap();
    let mut dep_lines = String::new();
    for (dep, ver) in &manifest.dependencies {
        dep_lines.push_str(&format!("{} = \"{}\"\n", dep, ver));
    }
    let cargo_toml = format!(
        "[package]\nname = \"{}\"\nversion = \"{}\"\nedition = \"2024\"\nlicense = \"MIT OR APACHE-2.0\"\nauthors = [\"PascalElixir / axolrs\"]\n\n[workspace]\n\n[dependencies]\n{}",
        pkg_name, pkg_version, dep_lines
    );
    let mut main_rust = String::new();
    let mut had_errors = false;
    for f in &axol_files {
        let src = fs::read_to_string(f).unwrap_or_default();
        let (rust, diags) = axolc_core::compile_to_rust(&src, 0);
        for d in &diags.items {
            if d.severity == axolc_core::diag::Severity::Error {
                eprintln!("{}: {}", f.display(), d);
                had_errors = true;
            }
        }
        if f.ends_with("main.axol") {
            main_rust.push_str(&rust);
            main_rust.push('\n');
        }
    }
    if had_errors {
        eprintln!("Build failed: compilation errors");
        std::process::exit(1);
    }
    fs::write(src_dir.join("main.rs"), main_rust).unwrap();
    fs::write(gen_dir.join("Cargo.toml"), cargo_toml).unwrap();
    let cargo = find_tool("cargo");
    let mut cmd = std::process::Command::new(&cargo);
    cmd.arg("build").current_dir(&gen_dir);
    if release {
        cmd.arg("--release");
    }
    match cmd.status() {
        Ok(s) if s.success() => {
            let profile = if release { "release" } else { "debug" };
            let bin = gen_dir.join("target").join(profile).join(&pkg_name);
            println!("Build succeeded: {}", bin.display());
        }
        _ => {
            eprintln!("Build failed");
            std::process::exit(1);
        }
    }
}

/// Emit the generated Rust for one file, or for every .axol file in the project.
pub fn emit_rust(file: Option<&Path>) {
    if let Some(f) = file {
        let src = fs::read_to_string(f).expect("read file");
        let (rust, diags) = axolc_core::compile_to_rust(&src, 0);
        print!("{}", rust);
        for d in &diags.items {
            eprintln!("{}", d);
        }
    } else {
        let dir = std::env::current_dir().unwrap();
        for f in collect_axol_files(&dir) {
            let src = fs::read_to_string(&f).unwrap_or_default();
            let (rust, _) = axolc_core::compile_to_rust(&src, 0);
            println!("// === {} ===", f.display());
            println!("{}", rust);
        }
    }
}

/// Resolve an executable by PATH, falling back to ~/.cargo/bin for toolchain binaries.
pub fn find_tool(name: &str) -> String {
    if let Ok(path) = std::env::var("PATH") {
        for dir in std::env::split_paths(&path) {
            let cand = dir.join(name);
            if cand.is_file() {
                return cand.to_string_lossy().into_owned();
            }
        }
    }
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    let fallback = format!("{}/.cargo/bin/{}", home, name);
    if Path::new(&fallback).is_file() {
        return fallback;
    }
    name.to_string()
}

/// Find a sibling toolchain binary next to this executable (cargo install layout).
pub fn find_sibling_tool(name: &str) -> Option<String> {
    let exe = std::env::current_exe().ok()?;
    let sibling = exe.parent()?.join(name);
    if sibling.is_file() {
        Some(sibling.to_string_lossy().into_owned())
    } else {
        None
    }
}

/// Load the project's Bucket.jsonc, defaulting when absent or unreadable.
pub fn load_manifest() -> axolc_core::manifest::Manifest {
    let content = fs::read_to_string("Bucket.jsonc").unwrap_or_default();
    axolc_core::manifest::parse(&content).unwrap_or_default()
}

/// Recursively collect .axol files, skipping build and vendor directories.
pub fn collect_axol_files(dir: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                if path.file_name().map_or(false, |n| is_skipped_dir(n)) {
                    continue;
                }
                out.extend(collect_axol_files(&path));
            } else if path.extension().map_or(false, |e| e == "axol") {
                out.push(path);
            }
        }
    }
    out
}

/// Report whether a directory name is excluded from .axol collection.
fn is_skipped_dir(name: &std::ffi::OsStr) -> bool {
    name == "target" || name == "pond" || name == "node_modules" || name == ".git"
}
