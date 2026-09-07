// Owner: PascalElixir / axolrs (GitHub org)
// File: crates/bucket/src/doctor.rs - the health check: toolchain versions, manifest state, tool availability.

use crate::build::{find_sibling_tool, find_tool};
use std::fs;
use std::path::Path;
use std::process::Command;

/// Run the doctor health check over the toolchain and project.
pub fn doctor() {
    println!("Bucket doctor - health check");
    println!("  rustc: {}", resolved_tool_version("rustc"));
    println!("  cargo: {}", resolved_tool_version("cargo"));
    println!("  axolc: {}", sibling_tool_version("axolc"));
    println!("  bucket: {}", env!("CARGO_PKG_VERSION"));
    println!("  axol-analyzer: {}", tool_presence("axol-analyzer"));
    println!("  axol-hot-runner: {}", tool_presence("axol-hot-runner"));
    println!("  Bucket.jsonc: {}", manifest_status());
    println!("  src/main.axol: {}", main_axol_status());
}

/// Render the health status of src/main.axol in the current directory.
pub fn main_axol_status() -> String {
    if Path::new("src/main.axol").is_file() {
        "present".to_string()
    } else {
        "missing".to_string()
    }
}

/// Render the presence and parse status of Bucket.jsonc in the current directory.
pub fn manifest_status() -> String {
    if !Path::new("Bucket.jsonc").is_file() {
        return "missing".to_string();
    }
    match fs::read_to_string("Bucket.jsonc") {
        Ok(content) => manifest_status_text(&content),
        Err(_) => "present (unreadable)".to_string(),
    }
}

/// Classify manifest content as cleanly parsed or failed.
pub fn manifest_status_text(content: &str) -> String {
    match crate::manifest::parse(content) {
        Ok(_) => "present (parses cleanly)".to_string(),
        Err(_) => "present (parse failed)".to_string(),
    }
}

/// Capture a tool's --version line via PATH or ~/.cargo/bin, or report it missing.
fn resolved_tool_version(tool: &str) -> String {
    version_of(&find_tool(tool))
}

/// Capture a sibling-or-PATH tool's version, preferring the binary next to this executable.
fn sibling_tool_version(tool: &str) -> String {
    let resolved = find_sibling_tool(tool).unwrap_or_else(|| find_tool(tool));
    version_of(&resolved)
}

/// Run a resolved binary with --version and take its first output line.
fn version_of(resolved: &str) -> String {
    Command::new(resolved)
        .arg("--version")
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .map(|s| s.lines().next().unwrap_or("").to_string())
        .unwrap_or_else(|_| "missing".to_string())
}

/// Report whether a toolchain binary is present as a sibling or on PATH.
fn tool_presence(tool: &str) -> String {
    if let Some(path) = find_sibling_tool(tool) {
        return format!("present ({})", path);
    }
    let candidate = find_tool(tool);
    if Path::new(&candidate).is_file() {
        format!("present ({})", candidate)
    } else {
        "missing".to_string()
    }
}
