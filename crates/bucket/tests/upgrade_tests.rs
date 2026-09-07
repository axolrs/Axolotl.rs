// Owner: PascalElixir / axolrs (GitHub org)
// File: crates/bucket/tests/upgrade_tests.rs - Molt toolchain upgrade and migration tests.

use bucket::upgrade::migration_report;
use bucket::Manifest;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::Command;

fn bucket_binary() -> String {
    let mut path = std::env::current_exe().unwrap();
    path.pop();
    path.pop();
    path.join("bucket").to_string_lossy().to_string()
}

/// Run the bucket binary with arguments, environment overrides, and a working directory.
fn run_bucket_env(dir: &Path, args: &[&str], env: &[(&str, &str)]) -> (String, String, i32) {
    let mut command = Command::new(bucket_binary());
    command.args(args).current_dir(dir);
    for (key, value) in env {
        command.env(key, value);
    }
    let output = command.output().expect("run bucket");
    (
        String::from_utf8_lossy(&output.stdout).to_string(),
        String::from_utf8_lossy(&output.stderr).to_string(),
        output.status.code().unwrap_or(-1),
    )
}

/// Create a unique temp directory for a test scenario.
fn temp_dir(tag: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("bucket-upgrade-{}-{}", tag, std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    dir
}

/// Build a manifest struct by hand for report rendering.
fn manifest(name: &str, version: &str, edition: &str) -> Manifest {
    Manifest {
        name: name.to_string(),
        version: version.to_string(),
        language: bucket::manifest::Language {
            edition: edition.to_string(),
        },
        ..Manifest::default()
    }
}

/// Migration report lines reflect the manifest identity.
#[test]
fn migration_report_table() {
    let cases: Vec<(&str, &str, &str, &str)> = vec![
        ("demo", "0.1.0", "2026", "Molt: manifest name=demo version=0.1.0 edition=2026"),
        ("app", "1.2.3", "2024", "Molt: manifest name=app version=1.2.3 edition=2024"),
        ("", "", "", "Molt: manifest name= version= edition="),
        ("x", "0.0.1", "", "Molt: manifest name=x version=0.0.1 edition="),
        ("x", "", "2026", "Molt: manifest name=x version= edition=2026"),
        ("long-project-name", "10.20.30", "2027", "Molt: manifest name=long-project-name version=10.20.30 edition=2027"),
        ("dashed-1", "1.0.0-alpha.1", "2026", "Molt: manifest name=dashed-1 version=1.0.0-alpha.1 edition=2026"),
    ];
    for (name, version, edition, expected) in cases {
        assert_eq!(migration_report(&manifest(name, version, edition)), expected);
    }
}

/// The migration report matches the parsed manifest of a real Bucket.jsonc.
#[test]
fn migration_report_matches_parsed_manifest() {
    let content = "{\n    \"name\": \"molting\",\n    \"version\": \"2.1.0\",\n    \"language\": {\n        \"edition\": \"2026\"\n    },\n    \"dependencies\": {}\n}";
    let parsed = bucket::manifest::parse(content).expect("manifest parses");
    assert_eq!(
        migration_report(&parsed),
        "Molt: manifest name=molting version=2.1.0 edition=2026"
    );
}

/// The migration report renders the scaffolded template manifest.
#[test]
fn migration_report_renders_template_manifest() {
    let content = "{\n    \"name\": \"template\",\n    \"version\": \"0.1.0\",\n    \"language\": {\n        \"edition\": \"2026\"\n    },\n    \"scripts\": {\n        \"dev\": \"bucket run --watch\"\n    },\n    \"dependencies\": {}\n}";
    let parsed = bucket::manifest::parse(content).expect("manifest parses");
    assert_eq!(
        migration_report(&parsed),
        "Molt: manifest name=template version=0.1.0 edition=2026"
    );
}

/// A manifest without a language block reports the default edition.
#[test]
fn migration_report_handles_missing_language() {
    let content = "{ \"name\": \"nolang\", \"version\": \"1.0.0\", \"dependencies\": {} }";
    let parsed = bucket::manifest::parse(content).expect("manifest parses");
    assert_eq!(
        migration_report(&parsed),
        "Molt: manifest name=nolang version=1.0.0 edition="
    );
}

/// Write a fake rustup shim script and return its directory.
fn rustup_shim(dir: &Path) -> PathBuf {
    let shim_dir = dir.join("shim");
    fs::create_dir_all(&shim_dir).unwrap();
    let script = shim_dir.join("rustup");
    fs::write(&script, "#!/bin/sh\necho \"rustup shim: stable updated (fake)\"\n").unwrap();
    let mut perms = fs::metadata(&script).unwrap().permissions();
    perms.set_mode(0o755);
    fs::set_permissions(&script, perms).unwrap();
    shim_dir
}

/// The current PATH with any directory containing rustup filtered out.
fn path_without_rustup() -> String {
    let entries: Vec<String> = std::env::split_paths(&std::env::var("PATH").unwrap_or_default())
        .filter(|dir| !dir.join("rustup").exists())
        .map(|dir| dir.to_string_lossy().to_string())
        .collect();
    entries.join(":")
}

/// The full PATH with a rustup shim directory prepended.
fn path_with_shim(shim_dir: &Path) -> String {
    format!("{}:{}", shim_dir.display(), std::env::var("PATH").unwrap_or_default())
}

/// Upgrade with a shimmed rustup rebuilds the project and prints the report.
#[test]
fn cli_upgrade_with_shimmed_rustup() {
    let dir = temp_dir("shim");
    let shim_dir = rustup_shim(&dir);
    let project = dir.join("proj");
    fs::create_dir_all(project.join("src")).unwrap();
    fs::write(
        project.join("Bucket.jsonc"),
        "{ \"name\": \"moltproj\", \"version\": \"0.2.0\", \"language\": { \"edition\": \"2026\" }, \"dependencies\": {} }",
    )
    .unwrap();
    fs::write(project.join("src/main.axol"), "fn main()\n    print(\"molted\")\nend\n").unwrap();
    let (out, err, code) = run_bucket_env(&project, &["upgrade"], &[("PATH", &path_with_shim(&shim_dir))]);
    assert_eq!(code, 0, "stderr: {err}");
    assert!(out.contains("Molt: toolchain upgrade and project migration"));
    assert!(out.contains("Molt: current toolchain: rustc"));
    assert!(out.contains("rustup shim: stable updated (fake)"));
    assert!(out.contains("Molt: rustup update stable completed"));
    assert!(out.contains("Molt: rebuilding the project in dev mode"));
    assert!(out.contains("Molt: dev build succeeded"));
    assert!(out.contains("Molt: manifest name=moltproj version=0.2.0 edition=2026"));
    assert!(project.join(".axol-gen/target/debug/moltproj").is_file());
    let _ = fs::remove_dir_all(&dir);
}

/// Upgrade without a manifest skips the rebuild and the migration report.
#[test]
fn cli_upgrade_without_manifest_skips_rebuild() {
    let dir = temp_dir("nomanifest");
    let shim_dir = rustup_shim(&dir);
    let (out, _err, code) = run_bucket_env(&dir, &["upgrade"], &[("PATH", &path_with_shim(&shim_dir))]);
    assert_eq!(code, 0);
    assert!(out.contains("Molt: no Bucket.jsonc; skipping project rebuild"));
    assert!(!out.contains("Molt: dev build succeeded"));
    assert!(!out.contains("Molt: manifest name="));
    let _ = fs::remove_dir_all(&dir);
}

/// Upgrade continues past a missing rustup and reports it honestly.
#[test]
fn cli_upgrade_survives_missing_rustup() {
    let dir = temp_dir("norustup");
    let (out, _err, code) = run_bucket_env(&dir, &["upgrade"], &[("PATH", &path_without_rustup())]);
    assert_eq!(code, 0);
    assert!(out.contains("Molt: rustup not found; skipping toolchain update"));
    assert!(out.contains("Molt: current toolchain: rustc"));
    assert!(out.contains("Molt: no Bucket.jsonc; skipping project rebuild"));
    let _ = fs::remove_dir_all(&dir);
}

/// Upgrade dies when the dev rebuild fails, because Molt refuses a broken project.
#[test]
fn cli_upgrade_fails_when_the_rebuild_fails() {
    let dir = temp_dir("broken");
    let shim_dir = rustup_shim(&dir);
    fs::write(
        dir.join("Bucket.jsonc"),
        "{ \"name\": \"broken\", \"version\": \"0.1.0\", \"dependencies\": {} }",
    )
    .unwrap();
    fs::create_dir_all(dir.join("src")).unwrap();
    fs::write(dir.join("src/main.axol"), "fn main()\n    let x =\nend\n").unwrap();
    let (_out, _err, code) = run_bucket_env(&dir, &["upgrade"], &[("PATH", &path_with_shim(&shim_dir))]);
    assert_eq!(code, 1);
    let _ = fs::remove_dir_all(&dir);
}
