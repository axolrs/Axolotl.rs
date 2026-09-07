// Owner: PascalElixir / axolrs (GitHub org)
// File: crates/bucket/src/project.rs - project scaffolding, manifest mutation, scripts, and clean.

use crate::build::load_manifest;
use std::fs;
use std::path::Path;

/// Create a new Axolotl project directory with manifest, source, and test scaffolding.
pub fn new_project(name: &str) {
    let dir = Path::new(name);
    fs::create_dir_all(dir.join("src")).unwrap();
    fs::create_dir_all(dir.join("tests")).unwrap();
    fs::write(dir.join("Bucket.jsonc"), manifest_template(name)).unwrap();
    fs::write(dir.join("src/main.axol"), main_template()).unwrap();
    fs::write(dir.join("src/lib.axol"), lib_template()).unwrap();
    println!("Created new Axolotl project `{}` in `{}/`", name, name);
}

/// Initialize a Bucket.jsonc in the current directory named after it.
pub fn init_project() {
    let name = std::env::current_dir().ok()
        .and_then(|d| d.file_name().map(|n| n.to_string_lossy().to_string()))
        .unwrap_or_else(|| "axolotl_project".to_string());
    fs::write("Bucket.jsonc", manifest_template(&name)).unwrap();
    fs::create_dir_all("src").ok();
    fs::write("src/main.axol", main_template()).unwrap();
    println!("Initialized Axolotl project `{}`", name);
}

/// Add a dependency to Bucket.jsonc, persisting the entry with a surgical text edit.
pub fn add_dep(name: &str, version: Option<String>) {
    let version = version.unwrap_or_else(|| "*".to_string());
    if let Err(msg) = validate_dep(name, &version) {
        eprintln!("add: {}", msg);
        std::process::exit(1);
    }
    let Ok(text) = fs::read_to_string("Bucket.jsonc") else {
        eprintln!("add: Bucket.jsonc not found in the current directory");
        std::process::exit(1);
    };
    let edited = match apply_dependency_edit(&text, name, &version) {
        Ok(t) => t,
        Err(msg) => {
            eprintln!("add: {}", msg);
            std::process::exit(1);
        }
    };
    if let Err(msg) = validate_jsonc_syntax(&edited) {
        eprintln!("add: edited manifest no longer parses: {}", msg);
        std::process::exit(1);
    }
    fs::write("Bucket.jsonc", &edited).unwrap();
    println!("Added dependency `{} {}` to Bucket.jsonc", name, version);
}

/// Validate a dependency name and version for safe insertion into Bucket.jsonc.
pub fn validate_dep(name: &str, version: &str) -> Result<(), String> {
    if name.trim().is_empty() {
        return Err("dependency name must not be empty".to_string());
    }
    if name.contains('"') {
        return Err(format!("dependency name `{}` contains a double quote", name));
    }
    if name.contains('\\') {
        return Err(format!("dependency name `{}` contains a backslash", name));
    }
    if name.chars().any(|c| c.is_control()) {
        return Err(format!("dependency name `{}` contains a control character", name));
    }
    if version.trim().is_empty() {
        return Err("dependency version must not be empty".to_string());
    }
    if version.contains('"') || version.contains('\\') {
        return Err(format!("dependency version `{}` must not contain quotes or backslashes", version));
    }
    if version.chars().any(|c| c.is_control()) {
        return Err(format!("dependency version `{}` contains a control character", version));
    }
    Ok(())
}

/// A significant item encountered while scanning JSONC text.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ScanEvent {
    /// A string literal: open quote, content start, content end, and close quote offsets.
    Str { open: usize, content_start: usize, content_end: usize, close: usize },
    /// One non-whitespace character outside strings and comments.
    Char { ch: char, at: usize },
}

/// A forward cursor over JSONC text that skips whitespace and comments.
pub struct JsonScanner<'a> {
    text: &'a str,
    pos: usize,
    end: usize,
}

impl<'a> JsonScanner<'a> {
    /// Create a scanner over the absolute byte range [from, end) of text.
    pub fn new(text: &'a str, from: usize, end: usize) -> JsonScanner<'a> {
        JsonScanner { text, pos: from, end }
    }

    /// Yield the next string literal or structural character, skipping trivia.
    pub fn next_event(&mut self) -> Option<ScanEvent> {
        let bytes = self.text.as_bytes();
        loop {
            while self.pos < self.end && matches!(bytes[self.pos], b' ' | b'\t' | b'\r' | b'\n') {
                self.pos += 1;
            }
            if self.pos >= self.end {
                return None;
            }
            if self.pos + 1 < self.end && bytes[self.pos] == b'/' && bytes[self.pos + 1] == b'/' {
                while self.pos < self.end && bytes[self.pos] != b'\n' {
                    self.pos += 1;
                }
                continue;
            }
            if self.pos + 1 < self.end && bytes[self.pos] == b'/' && bytes[self.pos + 1] == b'*' {
                self.pos += 2;
                while self.pos + 1 < self.end && !(bytes[self.pos] == b'*' && bytes[self.pos + 1] == b'/') {
                    self.pos += 1;
                }
                self.pos = (self.pos + 2).min(self.end);
                continue;
            }
            break;
        }
        if bytes[self.pos] == b'"' {
            let open = self.pos;
            self.pos += 1;
            let content_start = self.pos;
            while self.pos < self.end {
                let c = bytes[self.pos];
                if c == b'\\' {
                    self.pos = (self.pos + 2).min(self.end);
                } else if c == b'"' {
                    let content_end = self.pos;
                    self.pos += 1;
                    return Some(ScanEvent::Str {
                        open,
                        content_start,
                        content_end,
                        close: self.pos - 1,
                    });
                } else {
                    self.pos += 1;
                }
            }
            return Some(ScanEvent::Str { open, content_start, content_end: self.end, close: self.end });
        }
        let at = self.pos;
        let ch = bytes[at] as char;
        self.pos += 1;
        Some(ScanEvent::Char { ch, at })
    }
}

/// One key/value entry inside a JSONC object.
#[derive(Debug, Clone)]
pub struct Entry {
    /// The key text with quotes stripped.
    pub key: String,
    /// The offset of the key string's closing quote.
    pub key_close: usize,
    /// The offset of the colon after the key.
    pub colon: usize,
    /// The start offset of the value.
    pub value_start: usize,
    /// The end offset of the value (exclusive).
    pub value_end: usize,
}

/// Walk the object opening at byte offset open, returning its entries and closing brace offset.
pub fn object_entries(text: &str, open: usize) -> Result<(Vec<Entry>, usize), String> {
    let mut scanner = JsonScanner::new(text, open + 1, text.len());
    let mut events = Vec::new();
    while let Some(ev) = scanner.next_event() {
        events.push(ev);
    }
    let mut entries = Vec::new();
    let mut depth: i64 = 1;
    let mut i = 0;
    while i < events.len() {
        match events[i] {
            ScanEvent::Char { ch: '{', .. } | ScanEvent::Char { ch: '[', .. } => {
                depth += 1;
                i += 1;
            }
            ScanEvent::Char { ch: '}', at } | ScanEvent::Char { ch: ']', at } => {
                depth -= 1;
                if depth == 0 {
                    return Ok((entries, at));
                }
                i += 1;
            }
            ScanEvent::Str { content_start, content_end, close, .. } if depth == 1 => {
                let colon = match events.get(i + 1) {
                    Some(ScanEvent::Char { ch: ':', at }) => *at,
                    _ => {
                        i += 1;
                        continue;
                    }
                };
                let (value_start, value_end, next_i) = value_span(&events, i + 2)?;
                entries.push(Entry {
                    key: text.get(content_start..content_end).unwrap_or("").to_string(),
                    key_close: close,
                    colon,
                    value_start,
                    value_end,
                });
                i = next_i;
            }
            _ => {
                i += 1;
            }
        }
    }
    Err("unbalanced braces in JSONC object".to_string())
}

/// Compute the span of the JSON value at events index from, returning (start, end, next index).
fn value_span(events: &[ScanEvent], from: usize) -> Result<(usize, usize, usize), String> {
    let first = events
        .get(from)
        .ok_or_else(|| "unbalanced JSONC object: missing value after colon".to_string())?;
    match first {
        ScanEvent::Str { open, close, .. } => Ok((*open, close + 1, from + 1)),
        ScanEvent::Char { ch: '{', at } | ScanEvent::Char { ch: '[', at } => {
            let open = *at;
            let mut depth: i64 = 1;
            let mut j = from + 1;
            while j < events.len() {
                match events[j] {
                    ScanEvent::Char { ch: '{', .. } | ScanEvent::Char { ch: '[', .. } => depth += 1,
                    ScanEvent::Char { ch: '}', at } | ScanEvent::Char { ch: ']', at } => {
                        depth -= 1;
                        if depth == 0 {
                            return Ok((open, at + 1, j + 1));
                        }
                    }
                    _ => {}
                }
                j += 1;
            }
            Err("unbalanced braces in JSONC value".to_string())
        }
        ScanEvent::Char { ch, at } => {
            if matches!(ch, ',' | '}' | ']' | ':') {
                return Err("missing value after colon".to_string());
            }
            let mut j = from;
            let mut last_end = at + 1;
            while j < events.len() {
                match events[j] {
                    ScanEvent::Char { ch: c, at: a } if !matches!(c, ',' | '}' | ']' | ':') => {
                        last_end = a + 1;
                        j += 1;
                    }
                    _ => break,
                }
            }
            Ok((*at, last_end, j))
        }
    }
}

/// Edit a Bucket.jsonc source string, inserting or updating one dependency entry surgically.
pub fn apply_dependency_edit(text: &str, name: &str, version: &str) -> Result<String, String> {
    let root_open = find_root_open(text)?;
    let (entries, root_close) = object_entries(text, root_open)?;
    if let Some(deps) = entries.iter().find(|e| e.key == "dependencies") {
        if char_at(text, deps.value_start) != Some('{') {
            return Err("`dependencies` is not an object".to_string());
        }
        let dep_open = deps.value_start;
        let (dep_entries, _) = object_entries(text, dep_open)?;
        if let Some(entry) = dep_entries.iter().find(|e| e.key == name) {
            if char_at(text, entry.value_start) == Some('"') {
                replace_range(text, entry.value_start + 1, entry.value_end - 1, version)
            } else {
                replace_range(text, entry.value_start, entry.value_end, &format!("\"{}\"", version))
            }
        } else {
            let insertion = if dep_entries.is_empty() {
                format!(" \"{}\": \"{}\"", name, version)
            } else {
                format!(" \"{}\": \"{}\",", name, version)
            };
            replace_range(text, dep_open + 1, dep_open + 1, &insertion)
        }
    } else if let Some(scripts) = entries.iter().find(|e| e.key == "scripts") {
        if char_at(text, scripts.value_start) != Some('{') {
            let block = format!(",\n    \"dependencies\": {{\n        \"{}\": \"{}\"\n    }}\n", name, version);
            return replace_range(text, root_close, root_close, &block);
        }
        let block = format!(",\n\n    \"dependencies\": {{\n        \"{}\": \"{}\"\n    }}", name, version);
        replace_range(text, scripts.value_end, scripts.value_end, &block)
    } else {
        let block = if entries.is_empty() {
            format!("\"dependencies\": {{ \"{}\": \"{}\" }}", name, version)
        } else {
            format!(",\n    \"dependencies\": {{\n        \"{}\": \"{}\"\n    }}\n", name, version)
        };
        replace_range(text, root_close, root_close, &block)
    }
}

/// Find the byte offset of the root object's opening brace.
fn find_root_open(text: &str) -> Result<usize, String> {
    let mut scanner = JsonScanner::new(text, 0, text.len());
    match scanner.next_event() {
        Some(ScanEvent::Char { ch: '{', at }) => Ok(at),
        Some(_) => Err("manifest root is not a JSON object".to_string()),
        None => Err("manifest is empty".to_string()),
    }
}

/// Return the character at a byte offset as a raw byte value.
fn char_at(text: &str, at: usize) -> Option<char> {
    text.as_bytes().get(at).map(|b| *b as char)
}

/// Rebuild text by replacing [start, end) with replacement, failing on invalid boundaries.
fn replace_range(text: &str, start: usize, end: usize, replacement: &str) -> Result<String, String> {
    if start <= end && end <= text.len() && text.is_char_boundary(start) && text.is_char_boundary(end) {
        Ok(format!("{}{}{}", &text[..start], replacement, &text[end..]))
    } else {
        Err("manifest byte offsets are not on character boundaries".to_string())
    }
}

/// Validate that edited manifest text is still syntactically valid JSONC.
fn validate_jsonc_syntax(text: &str) -> Result<(), String> {
    let mut scanner = JsonScanner::new(text, 0, text.len());
    let mut compact = String::new();
    while let Some(ev) = scanner.next_event() {
        match ev {
            ScanEvent::Str { content_start, content_end, .. } => {
                compact.push('"');
                compact.push_str(text.get(content_start..content_end).unwrap_or(""));
                compact.push('"');
            }
            ScanEvent::Char { ch, .. } => compact.push(ch),
        }
    }
    serde_json::from_str::<serde_json::Value>(&compact)
        .map(|_| ())
        .map_err(|e| e.to_string())
}

/// Remove build artifact directories from the current project.
pub fn clean_cmd() {
    let dirs = ["target", "pond", ".axol-gen"];
    for d in &dirs {
        let _ = fs::remove_dir_all(d);
    }
    println!("Cleaned.");
}

/// Run a script defined in the project's Bucket.jsonc.
pub fn script(name: &str, _args: &[String]) {
    let manifest = load_manifest();
    if let Some(cmd) = manifest.scripts.get(name) {
        let status = std::process::Command::new("sh").arg("-c").arg(cmd).status();
        if let Ok(s) = status {
            if !s.success() {
                eprintln!("Script `{}` failed", name);
            }
        }
    } else {
        eprintln!("Script `{}` not found in Bucket.jsonc", name);
    }
}

/// Render the Bucket.jsonc template for a project with the given name.
fn manifest_template(name: &str) -> String {
    format!(r#"{{
    "name": "{}",
    "version": "0.1.0",

    "language": {{
        "edition": "2026"
    }},

    "interpreted_dev_mode": true,

    "scripts": {{
        "dev":     "bucket run --watch",
        "play":    "bucket run",
        "test":    "bucket test",
        "bench":   "bucket bench",
        "release": "bucket build --release",
        "fmt":     "bucket fmt",
        "lint":    "bucket lint",
        "fix":     "bucket fix",
        "doc":     "bucket doc",
        "clean":   "bucket clean",
        "doctor":  "bucket doctor"
    }},

    "dependencies": {{}},

    "build": {{
        "target": "x86_64-unknown-linux-gnu",
        "release": {{
            "lto": "thin",
            "codegen-units": 1
        }}
    }}
}}"#, name)
}

/// Render the src/main.axol template.
fn main_template() -> String {
    r#"-- Main Axolotl entry point
-- Owner: PascalElixir / axolrs (GitHub org)

fn main()
    print("Hello, Axolotl!")
end
"#.to_string()
}

/// Render the src/lib.axol template.
fn lib_template() -> String {
    r#"-- Library Axolotl module
-- Owner: PascalElixir / axolrs (GitHub org)
"#.to_string()
}
