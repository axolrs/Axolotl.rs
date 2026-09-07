// Owner: PascalElixir / axolrs (GitHub org)
// File: crates/bucket/src/gills.rs - starts the Gills language server with stdio passthrough.

use crate::build::{find_sibling_tool, find_tool};
use std::path::Path;
use std::process::{Command, Stdio};

/// Start the Gills LSP server, spawning axol-analyzer with true stdio passthrough.
pub fn gills(stdio: bool) {
    if !stdio {
        eprintln!("{}", transport_help_message());
        std::process::exit(1);
    }
    let Some(binary) = resolve_analyzer() else {
        eprintln!("{}", missing_analyzer_message());
        std::process::exit(1);
    };
    let status = Command::new(&binary)
        .arg("stdio")
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status();
    match status {
        Ok(s) => std::process::exit(s.code().unwrap_or(1)),
        Err(e) => {
            eprintln!("gills: failed to spawn {}: {}", binary, e);
            std::process::exit(1);
        }
    }
}

/// Resolve the axol-analyzer binary, preferring a sibling of this executable over PATH lookup.
pub fn resolve_analyzer() -> Option<String> {
    if let Some(sibling) = find_sibling_tool("axol-analyzer") {
        return Some(sibling);
    }
    if let Some(target_dir) = workspace_target_dir() {
        let candidate = target_dir.join("axol-analyzer");
        if candidate.is_file() {
            return Some(candidate.to_string_lossy().into_owned());
        }
    }
    let candidate = find_tool("axol-analyzer");
    if Path::new(&candidate).is_file() {
        Some(candidate)
    } else {
        None
    }
}

/// Return the workspace target directory when this executable runs from its deps subdirectory.
fn workspace_target_dir() -> Option<std::path::PathBuf> {
    let exe = std::env::current_exe().ok()?;
    let parent = exe.parent()?;
    if parent.file_name()? == "deps" {
        parent.parent().map(|p| p.to_path_buf())
    } else {
        None
    }
}

/// The guidance shown when gills is started without the stdio flag.
pub fn transport_help_message() -> String {
    "gills: stdio is the only transport today; start the server with `bucket gills --stdio`".to_string()
}

/// The error shown when the axol-analyzer binary cannot be resolved anywhere.
pub fn missing_analyzer_message() -> String {
    "gills: axol-analyzer not found on PATH; install with `cargo install --path crates/axol-analyzer`".to_string()
}
