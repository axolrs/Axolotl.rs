// Owner: PascalElixir / axolrs (GitHub org)
// File: crates/bucket/tests/doctor_tests.rs - health check tests for the doctor command.

use bucket::doctor::{main_axol_status, manifest_status_text};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn bucket_binary() -> String {
    let mut path = std::env::current_exe().unwrap();
    path.pop();
    path.pop();
    path.join("bucket").to_string_lossy().to_string()
}

/// Build a toolchain sibling binary if it is missing from the target directory.
fn ensure_sibling(name: &str) {
    let mut path = std::env::current_exe().unwrap();
    path.pop();
    path.pop();
    if path.join(name).is_file() {
        return;
    }
    let repo = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf();
    let status = Command::new("cargo")
        .args(["build", "-p", name])
        .current_dir(&repo)
        .status()
        .expect("cargo build sibling");
    assert!(status.success(), "building {name} failed");
}

/// Run the bucket binary with arguments in a working directory.
fn run_bucket_in(dir: &Path, args: &[&str]) -> (String, String, i32) {
    let output = Command::new(bucket_binary())
        .args(args)
        .current_dir(dir)
        .output()
        .expect("run bucket");
    (
        String::from_utf8_lossy(&output.stdout).to_string(),
        String::from_utf8_lossy(&output.stderr).to_string(),
        output.status.code().unwrap_or(-1),
    )
}

/// Create a unique temp directory for a test scenario.
fn temp_dir(tag: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("bucket-doctor-{}-{}", tag, std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    dir
}

/// Manifest content classification table: (content, expected status).
#[test]
fn manifest_status_text_table() {
    let clean: Vec<&str> = vec![
        "{ \"name\": \"x\", \"version\": \"1\" }",
        "{ \"name\": \"x\", \"version\": \"1\", \"dependencies\": {} }",
        "// comment\n{ \"name\": \"x\", \"version\": \"1\" }",
        "/* block */\n{ \"name\": \"x\", \"version\": \"1\" }",
        "{\n    \"name\": \"x\",\n    \"version\": \"1\",\n    \"scripts\": {\n        \"dev\": \"run\"\n    }\n}",
        "{ \"name\": \"x\", \"version\": \"1\", \"language\": { \"edition\": \"2026\" } }",
        "   { \"name\": \"x\", \"version\": \"1\" }   ",
        "{ \"name\": \"x\", \"version\": \"1\", \"build\": { \"target\": \"t\", \"release\": { \"lto\": \"thin\", \"codegen-units\": 1 } } }",
        "{ \"name\": \"x\", \"version\": \"1\", \"lsp\": { \"trace\": \"verbose\" } }",
        "{ \"name\": \"x\", \"version\": \"1\", \"interpreted_dev_mode\": true }",
    ];
    for content in clean {
        assert_eq!(manifest_status_text(content), "present (parses cleanly)", "content: {content}");
    }
    let broken: Vec<&str> = vec![
        "{ not json at all",
        "{ \"name\": }",
        "[1, 2, 3]",
        "\"just a string\"",
        "",
        "{}",
        "{ \"name\": \"x\" }",
        "{ \"name\": \"x\", }",
        "{ \"name\": 5, \"version\": \"1\" }",
        "{ \"name\": \"x\", \"version\": \"1\", \"scripts\": 7 }",
    ];
    for content in broken {
        assert_eq!(manifest_status_text(content), "present (parse failed)", "content: {content}");
    }
}

/// src/main.axol presence flips between present and missing with the cwd.
#[test]
fn main_axol_status_table() {
    let dir = temp_dir("main-status");
    assert_eq!(main_axol_status(), "missing");
    fs::create_dir_all(dir.join("src")).unwrap();
    fs::write(dir.join("src/main.axol"), "fn main()\nend\n").unwrap();
    let current = std::env::current_dir().unwrap();
    std::env::set_current_dir(&dir).unwrap();
    assert_eq!(main_axol_status(), "present");
    std::env::set_current_dir(&current).unwrap();
    let _ = fs::remove_dir_all(&dir);
}

/// A healthy project reports every tool and file.
#[test]
fn cli_doctor_reports_a_healthy_project() {
    ensure_sibling("axolc");
    ensure_sibling("axol-analyzer");
    ensure_sibling("axol-hot-runner");
    let dir = temp_dir("healthy");
    fs::create_dir_all(dir.join("src")).unwrap();
    fs::write(
        dir.join("Bucket.jsonc"),
        "{ \"name\": \"h\", \"version\": \"0.1.0\", \"language\": { \"edition\": \"2026\" }, \"dependencies\": {} }",
    )
    .unwrap();
    fs::write(dir.join("src/main.axol"), "fn main()\n    print(1)\nend\n").unwrap();
    let (out, err, code) = run_bucket_in(&dir, &["doctor"]);
    assert_eq!(code, 0, "stderr: {err}");
    assert!(out.starts_with("Bucket doctor - health check"));
    assert!(out.contains("\n  rustc: rustc "));
    assert!(!out.contains("  rustc: missing"));
    assert!(out.contains("\n  cargo: cargo "));
    assert!(!out.contains("  cargo: missing"));
    assert!(out.contains("\n  axolc: axolc "));
    assert!(!out.contains("  axolc: missing"));
    assert!(out.contains("\n  bucket: "));
    assert!(out.contains(env!("CARGO_PKG_VERSION")));
    assert!(out.contains("\n  axol-analyzer: present"));
    assert!(!out.contains("  axol-analyzer: missing"));
    assert!(out.contains("\n  axol-hot-runner: present"));
    assert!(!out.contains("  axol-hot-runner: missing"));
    assert!(out.contains("\n  Bucket.jsonc: present (parses cleanly)"));
    assert!(out.contains("\n  src/main.axol: present"));
    assert!(out.ends_with("  src/main.axol: present\n"));
    let _ = fs::remove_dir_all(&dir);
}

/// A corrupt manifest is reported as a parse failure without failing the check.
#[test]
fn cli_doctor_reports_a_corrupt_manifest() {
    let dir = temp_dir("corrupt");
    fs::write(dir.join("Bucket.jsonc"), "{ not json at all").unwrap();
    let (out, _err, code) = run_bucket_in(&dir, &["doctor"]);
    assert_eq!(code, 0);
    assert!(out.contains("\n  Bucket.jsonc: present (parse failed)"));
    assert!(!out.contains("parses cleanly"));
    let _ = fs::remove_dir_all(&dir);
}

/// An empty directory reports the manifest and entry point as missing.
#[test]
fn cli_doctor_reports_missing_project_files() {
    let dir = temp_dir("missing-files");
    let (out, _err, code) = run_bucket_in(&dir, &["doctor"]);
    assert_eq!(code, 0);
    assert!(out.contains("\n  Bucket.jsonc: missing"));
    assert!(out.contains("\n  src/main.axol: missing"));
    let _ = fs::remove_dir_all(&dir);
}

/// A manifest without the entry point reports main.axol missing only.
#[test]
fn cli_doctor_reports_missing_entry_point() {
    let dir = temp_dir("no-main");
    fs::write(
        dir.join("Bucket.jsonc"),
        "{ \"name\": \"n\", \"version\": \"0.1.0\", \"dependencies\": {} }",
    )
    .unwrap();
    let (out, _err, code) = run_bucket_in(&dir, &["doctor"]);
    assert_eq!(code, 0);
    assert!(out.contains("\n  Bucket.jsonc: present (parses cleanly)"));
    assert!(out.contains("\n  src/main.axol: missing"));
    let _ = fs::remove_dir_all(&dir);
}

/// A manifest with a comment still parses cleanly for the doctor.
#[test]
fn cli_doctor_accepts_commented_manifest() {
    let dir = temp_dir("commented");
    fs::write(
        dir.join("Bucket.jsonc"),
        "// my manifest\n{\n    \"name\": \"c\",\n    \"version\": \"0.1.0\",\n    \"dependencies\": {}\n}",
    )
    .unwrap();
    let (out, _err, code) = run_bucket_in(&dir, &["doctor"]);
    assert_eq!(code, 0);
    assert!(out.contains("\n  Bucket.jsonc: present (parses cleanly)"));
    let _ = fs::remove_dir_all(&dir);
}

/// The doctor output lists all nine report lines in order.
#[test]
fn cli_doctor_line_order() {
    ensure_sibling("axolc");
    ensure_sibling("axol-analyzer");
    ensure_sibling("axol-hot-runner");
    let dir = temp_dir("order");
    fs::create_dir_all(dir.join("src")).unwrap();
    fs::write(dir.join("Bucket.jsonc"), "{ \"name\": \"o\", \"version\": \"0.1.0\" }").unwrap();
    fs::write(dir.join("src/main.axol"), "fn main()\nend\n").unwrap();
    let (out, _err, code) = run_bucket_in(&dir, &["doctor"]);
    assert_eq!(code, 0);
    let lines: Vec<&str> = out.lines().collect();
    assert_eq!(lines.len(), 9);
    assert!(lines[0].starts_with("Bucket doctor"));
    assert!(lines[1].starts_with("  rustc: "));
    assert!(lines[2].starts_with("  cargo: "));
    assert!(lines[3].starts_with("  axolc: "));
    assert!(lines[4].starts_with("  bucket: "));
    assert!(lines[5].starts_with("  axol-analyzer: "));
    assert!(lines[6].starts_with("  axol-hot-runner: "));
    assert!(lines[7].starts_with("  Bucket.jsonc: "));
    assert!(lines[8].starts_with("  src/main.axol: "));
    let _ = fs::remove_dir_all(&dir);
}
