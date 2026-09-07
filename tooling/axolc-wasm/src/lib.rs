// Owner: PascalElixir / axolrs (GitHub org)
// File: tooling/axolc-wasm/src/lib.rs - flat C-ABI exports exposing the axolc compiler front-end to the browser playground.

use axolc_core::diag::Diagnostics;
use axolc_core::diag::Severity;
use axolc_core::{compile_to_rust, interpret};
use serde_json::{Value, json};

/// Allocate a caller-owned buffer of `len` bytes in linear memory that the host may write into.
#[unsafe(no_mangle)]
pub extern "C" fn axolc_alloc(len: usize) -> *mut u8 {
    let buffer = vec![0u8; len];
    Box::into_raw(buffer.into_boxed_slice()) as *mut u8
}

/// Return a NUL-terminated string with the axolc version backing this build.
#[unsafe(no_mangle)]
pub extern "C" fn axolc_version() -> *const u8 {
    concat!(env!("CARGO_PKG_VERSION"), "\0").as_ptr()
}

/// Interpret an Axolotl program and return a NUL-terminated JSON result string.
#[unsafe(no_mangle)]
pub extern "C" fn axolc_run(src: *const u8, src_len: usize) -> *mut u8 {
    let source = read_source(src, src_len);
    leak_nul_terminated(run_json(&source))
}

/// Compile an Axolotl program to Rust source and return a NUL-terminated JSON result string.
#[unsafe(no_mangle)]
pub extern "C" fn axolc_compile(src: *const u8, src_len: usize) -> *mut u8 {
    let source = read_source(src, src_len);
    leak_nul_terminated(compile_json(&source))
}

/// Read `src_len` bytes from `src` as UTF-8, degrading malformed bytes to replacement characters.
fn read_source(src: *const u8, src_len: usize) -> String {
    if src.is_null() {
        return String::new();
    }
    let bytes = unsafe { core::slice::from_raw_parts(src, src_len) };
    String::from_utf8_lossy(bytes).into_owned()
}

/// Serialize an interpreter run into the playground JSON envelope.
fn run_json(src: &str) -> String {
    let (output, diags) = interpret(src, 0);
    envelope(Some(&output), None, &diags, src).to_string()
}

/// Serialize a Rust compilation into the playground JSON envelope.
fn compile_json(src: &str) -> String {
    let (rust, diags) = compile_to_rust(src, 0);
    envelope(None, Some(&rust), &diags, src).to_string()
}

/// Build the wire envelope shared by both playground entry points.
fn envelope(output: Option<&str>, rust: Option<&str>, diags: &Diagnostics, src: &str) -> Value {
    json!({
        "output": output.unwrap_or_default(),
        "rust": rust.unwrap_or_default(),
        "diagnostics": diagnostics_json(diags, src),
    })
}

/// Map diagnostics to the flat JSON rows consumed by the playground UI.
fn diagnostics_json(diags: &Diagnostics, src: &str) -> Vec<Value> {
    diags
        .items
        .iter()
        .map(|d| {
            let (line, col) = line_col(src, d.span.start as usize);
            json!({
                "severity": severity_str(d.severity),
                "code": d.code,
                "message": d.message,
                "notes": d.notes,
                "line": line,
                "col": col,
            })
        })
        .collect()
}

/// Lower a severity to its lowercase wire name.
fn severity_str(severity: Severity) -> &'static str {
    match severity {
        Severity::Error => "error",
        Severity::Warning => "warning",
        Severity::Info => "info",
        Severity::Hint => "hint",
    }
}

/// Translate a byte offset into 1-based (line, column) coordinates counted in characters.
fn line_col(src: &str, offset: usize) -> (usize, usize) {
    let mut clamped = offset.min(src.len());
    while !src.is_char_boundary(clamped) {
        clamped -= 1;
    }
    let before = &src[..clamped];
    let line = 1 + before.matches('\n').count();
    let line_start = before.rfind('\n').map_or(0, |i| i + 1);
    let col = 1 + before[line_start..].chars().count();
    (line, col)
}

/// Leak a JSON string as a NUL-terminated buffer owned by the wasm linear memory.
fn leak_nul_terminated(json: String) -> *mut u8 {
    let mut bytes = json.into_bytes();
    bytes.push(0);
    Box::into_raw(bytes.into_boxed_slice()) as *mut u8
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Value;

    /// Run the canonical hello-world program and assert the greeting appears in the JSON.
    #[test]
    fn hello_world_run_json_contains_greeting() {
        let src = "fn main()\n    print(\"Hello, Axolotl!\")\nend";
        let json = run_json(src);
        assert!(json.contains("Hello, Axolotl!"), "json was: {json}");
        let parsed: Value = serde_json::from_str(&json).unwrap();
        assert!(parsed["output"].as_str().unwrap().contains("Hello, Axolotl!"));
        assert!(parsed["diagnostics"].as_array().unwrap().is_empty());
    }

    /// Run the README player example end-to-end through the interpreter envelope.
    #[test]
    fn player_example_run_json_reports_final_hp() {
        let src = "Player = struct\n    name: String\n    hp: Int\nend\n\nPlayer.attack = fn(self, damage: Int)\n    self.hp = self.hp - damage\n    return self.hp\nend\n\nfn main()\n    var hero = Player { name = \"axol\", hp = 100 }\n    hero:attack(12)\n    print(\"hero hp = ${hero.hp}\")\nend";
        let parsed: Value = serde_json::from_str(&run_json(src)).unwrap();
        assert!(parsed["diagnostics"].as_array().unwrap().is_empty());
        assert_eq!(parsed["output"].as_str().unwrap(), "hero hp = 88\n");
    }

    /// Feed a broken program and assert an error diagnostic with line and column arrives.
    #[test]
    fn broken_program_reports_error_with_position() {
        let src = "fn main()\n    let x =\nend";
        let parsed: Value = serde_json::from_str(&run_json(src)).unwrap();
        let diags = parsed["diagnostics"].as_array().unwrap();
        assert!(!diags.is_empty(), "expected diagnostics, json was: {parsed}");
        let first = &diags[0];
        assert_eq!(first["severity"].as_str().unwrap(), "error");
        assert!(!first["message"].as_str().unwrap().is_empty());
        assert!(first["line"].as_u64().unwrap() >= 1);
        assert!(first["col"].as_u64().unwrap() >= 1);
    }

    /// Compile the hello-world program and assert generated Rust carries a main function.
    #[test]
    fn compile_json_contains_fn_main() {
        let src = "fn main()\n    print(\"Hello, Axolotl!\")\nend";
        let parsed: Value = serde_json::from_str(&compile_json(src)).unwrap();
        assert!(parsed["rust"].as_str().unwrap().contains("fn main()"));
        assert_eq!(parsed["output"].as_str().unwrap(), "");
    }

    /// Exercise the raw run ABI and assert the returned buffer is valid UTF-8 with a NUL sentinel.
    #[test]
    fn run_abi_returns_nul_terminated_utf8() {
        let src = b"fn main()\n    print(\"Hello, Axolotl!\")\nend";
        let ptr = axolc_run(src.as_ptr(), src.len());
        let s = read_c_string(ptr);
        let parsed: Value = serde_json::from_str(&s).unwrap();
        assert!(parsed["output"].as_str().unwrap().contains("Hello, Axolotl!"));
    }

    /// Exercise the raw compile ABI and assert the returned buffer is valid UTF-8 with a NUL sentinel.
    #[test]
    fn compile_abi_returns_nul_terminated_utf8() {
        let src = b"fn main()\n    print(\"Hello, Axolotl!\")\nend";
        let ptr = axolc_compile(src.as_ptr(), src.len());
        let s = read_c_string(ptr);
        let parsed: Value = serde_json::from_str(&s).unwrap();
        assert!(parsed["rust"].as_str().unwrap().contains("fn main()"));
    }

    /// Assert the version export returns the crate version as a C string.
    #[test]
    fn version_abi_returns_crate_version() {
        let s = read_c_string(axolc_version() as *const u8);
        assert_eq!(s, env!("CARGO_PKG_VERSION"));
    }

    /// Assert the allocator hands back a writable buffer of the requested length.
    #[test]
    fn alloc_returns_writable_buffer() {
        let len = 16;
        let ptr = axolc_alloc(len);
        let slice = unsafe { core::slice::from_raw_parts_mut(ptr, len) };
        slice.copy_from_slice(&[7u8; 16]);
        assert!(slice.iter().all(|&b| b == 7));
    }

    /// Read a NUL-terminated byte buffer as an owned UTF-8 string.
    fn read_c_string(ptr: *const u8) -> String {
        let mut len = 0;
        while unsafe { *ptr.add(len) } != 0 {
            len += 1;
        }
        let bytes = unsafe { core::slice::from_raw_parts(ptr, len) };
        String::from_utf8(bytes.to_vec()).unwrap()
    }
}
