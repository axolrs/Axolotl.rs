// Owner: PascalElixir / axolrs (GitHub org)
// File: crates/bucket/tests/publish_tests.rs - Eggbox publisher tests.

use bucket::publish::{final_step_hint, find_crate_files};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn bucket_binary() -> String {
    let mut path = std::env::current_exe().unwrap();
    path.pop();
    path.pop();
    path.join("bucket").to_string_lossy().to_string()
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
    let dir = std::env::temp_dir().join(format!("bucket-publish-{}-{}", tag, std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    dir
}

/// Scaffold a minimal dependency-free project for packaging.
fn scaffold_project(dir: &Path, name: &str) {
    fs::write(
        dir.join("Bucket.jsonc"),
        format!("{{ \"name\": \"{name}\", \"version\": \"0.1.0\", \"dependencies\": {{}} }}"),
    )
    .unwrap();
    fs::create_dir_all(dir.join("src")).unwrap();
    fs::write(dir.join("src/main.axol"), "fn main()\n    print(\"packaged\")\nend\n").unwrap();
}

/// find_crate_files only picks up .crate artifacts, sorted.
#[test]
fn find_crate_files_table() {
    let dir = temp_dir("find");
    assert!(find_crate_files(&dir).is_empty());
    let missing = dir.join("nowhere");
    assert!(find_crate_files(&missing).is_empty());
    fs::write(dir.join("demo-0.1.0.crate"), "crate").unwrap();
    fs::write(dir.join("demo-0.1.0.crate.sha256"), "checksum").unwrap();
    fs::write(dir.join("other-0.2.0.crate"), "crate").unwrap();
    fs::write(dir.join("notes.txt"), "text").unwrap();
    fs::write(dir.join("subdir"), "not a dir").unwrap();
    let found = find_crate_files(&dir);
    assert_eq!(found.len(), 2);
    assert_eq!(found[0], dir.join("demo-0.1.0.crate"));
    assert_eq!(found[1], dir.join("other-0.2.0.crate"));
    assert!(found.iter().all(|f| f.extension().map_or(false, |e| e == "crate")));
    let _ = fs::remove_dir_all(&dir);
}

/// find_crate_files ignores subdirectories of the package directory.
#[test]
fn find_crate_files_skips_directories() {
    let dir = temp_dir("find-dirs");
    fs::create_dir_all(dir.join("tmp-crate")).unwrap();
    fs::write(dir.join("demo-0.1.0.crate"), "crate").unwrap();
    let found = find_crate_files(&dir);
    assert_eq!(found.len(), 1);
    assert_eq!(found[0], dir.join("demo-0.1.0.crate"));
    let _ = fs::remove_dir_all(&dir);
}

/// The final-step hint names the publish command and the login requirement.
#[test]
fn final_step_hint_is_actionable() {
    let hint = final_step_hint();
    assert!(hint.starts_with("Eggbox: final step"));
    assert!(hint.contains("crates.io login"));
    assert!(hint.contains("cargo publish --allow-dirty"));
    assert!(hint.contains(".axol-gen"));
}

/// A full publish produces a .crate artifact under .axol-gen/target/package.
#[test]
fn cli_publish_packages_the_project() {
    let dir = temp_dir("full");
    scaffold_project(&dir, "eggpkg");
    let (out, err, code) = run_bucket_in(&dir, &["publish"]);
    assert_eq!(code, 0, "stderr: {err}");
    assert!(out.contains("Eggbox: no release build found; building the project first"));
    assert!(out.contains("Eggbox: packaging .axol-gen (cargo package --allow-dirty)"));
    assert!(out.contains("Eggbox: packaged .axol-gen/target/package/eggpkg-0.1.0.crate"));
    assert!(out.contains(&final_step_hint()));
    let package_dir = dir.join(".axol-gen/target/package");
    let crates = find_crate_files(&package_dir);
    assert_eq!(crates.len(), 1);
    assert!(crates[0].file_name().unwrap().to_string_lossy().contains("eggpkg-0.1.0.crate"));
    assert!(dir.join(".axol-gen/Cargo.toml").is_file());
    let _ = fs::remove_dir_all(&dir);
}

/// A second publish reuses the existing build and packages again.
#[test]
fn cli_publish_reuses_the_existing_build() {
    let dir = temp_dir("republish");
    scaffold_project(&dir, "againpkg");
    let (_out, _err, first) = run_bucket_in(&dir, &["publish"]);
    assert_eq!(first, 0);
    let (out, _err, code) = run_bucket_in(&dir, &["publish"]);
    assert_eq!(code, 0);
    assert!(!out.contains("no release build found"));
    assert!(out.contains("Eggbox: packaged .axol-gen/target/package/againpkg-0.1.0.crate"));
    let _ = fs::remove_dir_all(&dir);
}

/// Publish in a project without sources fails like a build would.
#[test]
fn cli_publish_fails_without_sources() {
    let dir = temp_dir("nosrc");
    fs::write(
        dir.join("Bucket.jsonc"),
        "{ \"name\": \"nosrc\", \"version\": \"0.1.0\", \"dependencies\": {} }",
    )
    .unwrap();
    let (_out, err, code) = run_bucket_in(&dir, &["publish"]);
    assert_eq!(code, 1);
    assert!(err.contains("no .axol files found"));
    assert!(!dir.join(".axol-gen").exists());
    let _ = fs::remove_dir_all(&dir);
}
