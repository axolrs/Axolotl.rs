// Owner: PascalElixir / axolrs (GitHub org)
// File: Bucket.jsonc manifest parsing.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A Bucket.jsonc manifest.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Manifest {
    pub name: String,
    pub version: String,
    #[serde(default)]
    pub language: Language,
    #[serde(default)]
    pub interpreted_dev_mode: bool,
    #[serde(default)]
    pub scripts: HashMap<String, String>,
    #[serde(default)]
    pub dependencies: HashMap<String, String>,
    #[serde(default)]
    pub build: Build,
    #[serde(default)]
    pub lsp: Lsp,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Language {
    pub edition: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Build {
    pub target: String,
    #[serde(default)]
    pub release: Release,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Release {
    pub lto: String,
    #[serde(rename = "codegen-units", default)]
    pub codegen_units: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Lsp {
    pub trace: String,
}

/// Parse a Bucket.jsonc string (allowing comments) into a Manifest.
pub fn parse(content: &str) -> Result<Manifest, String> {
    let cleaned = strip_jsonc_comments(content);
    serde_json::from_str(&cleaned).map_err(|e| e.to_string())
}

fn strip_jsonc_comments(s: &str) -> String {
    let mut out = String::new();
    let bytes: Vec<char> = s.chars().collect();
    let mut i = 0;
    let mut in_string = false;
    let mut escape = false;
    while i < bytes.len() {
        let c = bytes[i];
        if in_string {
            out.push(c);
            if escape { escape = false; }
            else if c == '\\' { escape = true; }
            else if c == '"' { in_string = false; }
            i += 1;
        } else if c == '"' {
            in_string = true;
            out.push(c);
            i += 1;
        } else if c == '/' && i + 1 < bytes.len() && bytes[i + 1] == '/' {
            while i < bytes.len() && bytes[i] != '\n' { i += 1; }
        } else if c == '/' && i + 1 < bytes.len() && bytes[i + 1] == '*' {
            i += 2;
            while i + 1 < bytes.len() && !(bytes[i] == '*' && bytes[i + 1] == '/') { i += 1; }
            i += 2;
        } else {
            out.push(c);
            i += 1;
        }
    }
    out
}
