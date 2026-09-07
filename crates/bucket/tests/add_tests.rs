// Owner: PascalElixir / axolrs (GitHub org)
// File: crates/bucket/tests/add_tests.rs - dependency persistence tests for bucket add.

use bucket::project::{
    apply_dependency_edit, object_entries, validate_dep, JsonScanner, ScanEvent,
};
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
    let dir = std::env::temp_dir().join(format!("bucket-add-{}-{}", tag, std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    dir
}

/// The freshly scaffolded template manifest used by bucket new.
fn template_manifest() -> String {
    r#"{
    "name": "demo",
    "version": "0.1.0",

    "language": {
        "edition": "2026"
    },

    "interpreted_dev_mode": true,

    "scripts": {
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
    },

    "dependencies": {},

    "build": {
        "target": "x86_64-unknown-linux-gnu",
        "release": {
            "lto": "thin",
            "codegen-units": 1
        }
    }
}"#
    .to_string()
}

/// Verify an edited manifest is valid JSON with the dependency at its version.
fn assert_json_with_dep(text: &str, name: &str, version: &str) {
    let value: serde_json::Value = serde_json::from_str(&strip_jsonc(text))
        .unwrap_or_else(|e| panic!("edited manifest is not valid JSON: {e}\n{text}"));
    assert_eq!(value["dependencies"][name].as_str(), Some(version), "in {text}");
}

/// Strip JSONC comments for JSON-level validation in tests.
fn strip_jsonc(s: &str) -> String {
    let mut out = String::new();
    let chars: Vec<char> = s.chars().collect();
    let mut i = 0;
    let mut in_string = false;
    let mut escape = false;
    while i < chars.len() {
        let c = chars[i];
        if in_string {
            out.push(c);
            if escape {
                escape = false;
            } else if c == '\\' {
                escape = true;
            } else if c == '"' {
                in_string = false;
            }
            i += 1;
        } else if c == '"' {
            in_string = true;
            out.push(c);
            i += 1;
        } else if c == '/' && i + 1 < chars.len() && chars[i + 1] == '/' {
            while i < chars.len() && chars[i] != '\n' {
                i += 1;
            }
        } else if c == '/' && i + 1 < chars.len() && chars[i + 1] == '*' {
            i += 2;
            while i + 1 < chars.len() && !(chars[i] == '*' && chars[i + 1] == '/') {
                i += 1;
            }
            i += 2;
        } else {
            out.push(c);
            i += 1;
        }
    }
    out
}

/// Surgical edit table: (input manifest, dep name, version, expected output).
fn edit_cases() -> Vec<(String, &'static str, &'static str, String)> {
    let t = template_manifest();
    let mut edited_template = t.replace("\"dependencies\": {},", "\"dependencies\": { \"serde\": \"*\"},");
    let _ = &mut edited_template;
    vec![
        (
            "{\n    \"dependencies\": {}\n}".to_string(),
            "serde",
            "*",
            "{\n    \"dependencies\": { \"serde\": \"*\"}\n}".to_string(),
        ),
        (
            "{\n    \"dependencies\": { \"serde\": \"1\" }\n}".to_string(),
            "serde",
            "2.0",
            "{\n    \"dependencies\": { \"serde\": \"2.0\" }\n}".to_string(),
        ),
        (
            "{\n    \"dependencies\": { \"serde\": \"1\" }\n}".to_string(),
            "serde",
            "1",
            "{\n    \"dependencies\": { \"serde\": \"1\" }\n}".to_string(),
        ),
        (
            "{\n    \"dependencies\": { \"serde\": \"1\", \"regex\": \"2\" }\n}".to_string(),
            "tokio",
            "*",
            "{\n    \"dependencies\": { \"tokio\": \"*\", \"serde\": \"1\", \"regex\": \"2\" }\n}".to_string(),
        ),
        (
            "{\n    \"dependencies\": { \"serde\": \"1\" }\n}".to_string(),
            "regex",
            "2",
            "{\n    \"dependencies\": { \"regex\": \"2\", \"serde\": \"1\" }\n}".to_string(),
        ),
        (
            "{\n    \"dependencies\": {\n        \"serde\": \"1\"\n    }\n}".to_string(),
            "regex",
            "2",
            "{\n    \"dependencies\": { \"regex\": \"2\",\n        \"serde\": \"1\"\n    }\n}".to_string(),
        ),
        (
            "{\n    \"dependencies\": { \"serde\": { \"version\": \"1\", \"path\": \"x\" } }\n}".to_string(),
            "serde",
            "2",
            "{\n    \"dependencies\": { \"serde\": \"2\" }\n}".to_string(),
        ),
        (
            "{\n    \"dependencies\": { \"serde\": 1 }\n}".to_string(),
            "serde",
            "2",
            "{\n    \"dependencies\": { \"serde\": \"2\" }\n}".to_string(),
        ),
        (
            "{\n    \"dependencies\": { \"serde\": true }\n}".to_string(),
            "serde",
            "2",
            "{\n    \"dependencies\": { \"serde\": \"2\" }\n}".to_string(),
        ),
        (
            "{\n    \"dependencies\": { \"serde\": [\"a\", \"b\"] }\n}".to_string(),
            "serde",
            "2",
            "{\n    \"dependencies\": { \"serde\": \"2\" }\n}".to_string(),
        ),
        (
            "{\n    \"dependencies\": { \"serde\": null }\n}".to_string(),
            "serde",
            "2",
            "{\n    \"dependencies\": { \"serde\": \"2\" }\n}".to_string(),
        ),
        (
            "// bucket manifest\n{\n    \"dependencies\": { \"serde\": \"1\" }\n}".to_string(),
            "serde",
            "9",
            "// bucket manifest\n{\n    \"dependencies\": { \"serde\": \"9\" }\n}".to_string(),
        ),
        (
            "{\n    /* deps live here */ \"dependencies\": { \"serde\": \"1\" }\n}".to_string(),
            "serde",
            "9",
            "{\n    /* deps live here */ \"dependencies\": { \"serde\": \"9\" }\n}".to_string(),
        ),
        (
            "{\n    \"dependencies\": { \"serde\": \"1\" } /* trailing */\n}".to_string(),
            "serde",
            "9",
            "{\n    \"dependencies\": { \"serde\": \"9\" } /* trailing */\n}".to_string(),
        ),
        (
            "{\n    \"dependencies\": { /* only a comment */ }\n}".to_string(),
            "serde",
            "*",
            "{\n    \"dependencies\": { \"serde\": \"*\" /* only a comment */ }\n}".to_string(),
        ),
        (
            "{\n    \"dependencies\": { \"serde\" : \"1\" }\n}".to_string(),
            "serde",
            "2",
            "{\n    \"dependencies\": { \"serde\" : \"2\" }\n}".to_string(),
        ),
        (
            "{\n    \"dependencies\" : {\n        \"serde\" : \"1\"\n    }\n}".to_string(),
            "serde",
            "2",
            "{\n    \"dependencies\" : {\n        \"serde\" : \"2\"\n    }\n}".to_string(),
        ),
        (
            "{\n    \"dependencies\": { \"serde\": \"1\", \"other\": { \"nested\": \"x\" } }\n}".to_string(),
            "serde",
            "3",
            "{\n    \"dependencies\": { \"serde\": \"3\", \"other\": { \"nested\": \"x\" } }\n}".to_string(),
        ),
        (
            "{\n    \"dependencies\": { \"other\": { \"serde\": \"x\" } }\n}".to_string(),
            "serde",
            "3",
            "{\n    \"dependencies\": { \"serde\": \"3\", \"other\": { \"serde\": \"x\" } }\n}".to_string(),
        ),
        (
            "{\n    \"dependencies\": { \"quoted\": \"serde\" }\n}".to_string(),
            "serde",
            "3",
            "{\n    \"dependencies\": { \"serde\": \"3\", \"quoted\": \"serde\" }\n}".to_string(),
        ),
        (
            "{\n    \"dependencies\": { \"regex\": \"serde\" }\n}".to_string(),
            "serde",
            "3",
            "{\n    \"dependencies\": { \"serde\": \"3\", \"regex\": \"serde\" }\n}".to_string(),
        ),
        (
            "{\n    \"dependencies\": {\n        \"a\": \"1\",\n        \"b\": \"2\",\n        \"c\": \"3\"\n    }\n}".to_string(),
            "b",
            "9",
            "{\n    \"dependencies\": {\n        \"a\": \"1\",\n        \"b\": \"9\",\n        \"c\": \"3\"\n    }\n}".to_string(),
        ),
        (
            "{\n    \"scripts\": {\n        \"dev\": \"bucket run\"\n    }\n}".to_string(),
            "serde",
            "*",
            "{\n    \"scripts\": {\n        \"dev\": \"bucket run\"\n    },\n\n    \"dependencies\": {\n        \"serde\": \"*\"\n    }\n}".to_string(),
        ),
        (
            "{\n    \"name\": \"x\",\n    \"scripts\": {\n        \"dev\": \"bucket run\"\n    },\n    \"build\": {}\n}".to_string(),
            "serde",
            "*",
            "{\n    \"name\": \"x\",\n    \"scripts\": {\n        \"dev\": \"bucket run\"\n    },\n\n    \"dependencies\": {\n        \"serde\": \"*\"\n    },\n    \"build\": {}\n}".to_string(),
        ),
        (
            "{\n    \"name\": \"x\",\n    \"version\": \"1.0\"\n}".to_string(),
            "serde",
            "*",
            "{\n    \"name\": \"x\",\n    \"version\": \"1.0\"\n,\n    \"dependencies\": {\n        \"serde\": \"*\"\n    }\n}".to_string(),
        ),
        (
            "{}".to_string(),
            "serde",
            "*",
            "{\"dependencies\": { \"serde\": \"*\" }}".to_string(),
        ),
        (
            "{ \"name\": \"x\" }".to_string(),
            "serde",
            "*",
            "{ \"name\": \"x\" ,\n    \"dependencies\": {\n        \"serde\": \"*\"\n    }\n}".to_string(),
        ),
        (
            "{ \"name\": \"x\", \"scripts\": \"not-an-object\" }".to_string(),
            "serde",
            "*",
            "{ \"name\": \"x\", \"scripts\": \"not-an-object\" ,\n    \"dependencies\": {\n        \"serde\": \"*\"\n    }\n}".to_string(),
        ),
        (
            t.clone(),
            "serde",
            "*",
            edited_template.clone(),
        ),
        (
            "{\n    \"scripts\": {\n        \"dev\": \"run\"\n    },\n\n    \"build\": {}\n}".to_string(),
            "serde",
            "*",
            "{\n    \"scripts\": {\n        \"dev\": \"run\"\n    },\n\n    \"dependencies\": {\n        \"serde\": \"*\"\n    },\n\n    \"build\": {}\n}".to_string(),
        ),
        (
            "{\n\t\"dependencies\": {\n\t\t\"serde\": \"1\"\n\t}\n}".to_string(),
            "serde",
            "2",
            "{\n\t\"dependencies\": {\n\t\t\"serde\": \"2\"\n\t}\n}".to_string(),
        ),
        (
            "{\"dependencies\":{\"serde\":\"1\"}}".to_string(),
            "serde",
            "2",
            "{\"dependencies\":{\"serde\":\"2\"}}".to_string(),
        ),
        (
            "{\"dependencies\":{}}".to_string(),
            "serde",
            "2",
            "{\"dependencies\":{ \"serde\": \"2\"}}".to_string(),
        ),
        (
            "{\r\n    \"dependencies\": { \"serde\": \"1\" }\r\n}".to_string(),
            "serde",
            "2",
            "{\r\n    \"dependencies\": { \"serde\": \"2\" }\r\n}".to_string(),
        ),
        (
            "{\n    \"dependencies\": { \"a\": \"1\" },\n    \"scripts\": {}\n}".to_string(),
            "a",
            "2",
            "{\n    \"dependencies\": { \"a\": \"2\" },\n    \"scripts\": {}\n}".to_string(),
        ),
        (
            "{\n    \"dependencies\": { \"serde\": \"1.0.0-alpha.7+build\" }\n}".to_string(),
            "serde",
            ">=1.0",
            "{\n    \"dependencies\": { \"serde\": \">=1.0\" }\n}".to_string(),
        ),
        (
            "{\n    \"dependencies\": { \"serde\": \"1\" }\n}".to_string(),
            "serde",
            "with spaces",
            "{\n    \"dependencies\": { \"serde\": \"with spaces\" }\n}".to_string(),
        ),
        (
            "{\n    \"scripts\": { \"dependencies\": \"bucket x\" }\n}".to_string(),
            "serde",
            "*",
            "{\n    \"scripts\": { \"dependencies\": \"bucket x\" },\n\n    \"dependencies\": {\n        \"serde\": \"*\"\n    }\n}".to_string(),
        ),
        (
            "{\n    \"dependencies\": { \"with-dash\": \"1\" }\n}".to_string(),
            "with-dash",
            "2",
            "{\n    \"dependencies\": { \"with-dash\": \"2\" }\n}".to_string(),
        ),
        (
            "{\n    \"dependencies\": { \"with.dot\": \"1\" }\n}".to_string(),
            "with.dot",
            "2",
            "{\n    \"dependencies\": { \"with.dot\": \"2\" }\n}".to_string(),
        ),
        (
            "{\n    \"dependencies\": { \"with/slash\": \"1\" }\n}".to_string(),
            "with/slash",
            "2",
            "{\n    \"dependencies\": { \"with/slash\": \"2\" }\n}".to_string(),
        ),
        (
            "{\n    \"dependencies\": { \"déjà\": \"1\" }\n}".to_string(),
            "déjà",
            "2",
            "{\n    \"dependencies\": { \"déjà\": \"2\" }\n}".to_string(),
        ),
        (
            "{\n    \"dependencies\": {}\n    ,\n    \"scripts\": {}\n}".to_string(),
            "serde",
            "*",
            "{\n    \"dependencies\": { \"serde\": \"*\"}\n    ,\n    \"scripts\": {}\n}".to_string(),
        ),
        (
            "{\n    \"dependencies\": { \"a\": \"1\" , \"b\": \"2\" }\n}".to_string(),
            "c",
            "3",
            "{\n    \"dependencies\": { \"c\": \"3\", \"a\": \"1\" , \"b\": \"2\" }\n}".to_string(),
        ),
        (
            "{\n    \"build\": {},\n    \"dependencies\": {}\n}".to_string(),
            "serde",
            "*",
            "{\n    \"build\": {},\n    \"dependencies\": { \"serde\": \"*\"}\n}".to_string(),
        ),
        (
            "{\n    \"dependencies\": { \"serde\": \"1\" }\n}\n// end of file\n".to_string(),
            "serde",
            "2",
            "{\n    \"dependencies\": { \"serde\": \"2\" }\n}\n// end of file\n".to_string(),
        ),
    ]
}

/// Error table: (input manifest, dep name, version, expected error substring).
fn edit_error_cases() -> Vec<(String, &'static str, &'static str, &'static str)> {
    vec![
        (
            "{\"dependencies\": \"not-an-object\"}".to_string(),
            "serde",
            "*",
            "`dependencies` is not an object",
        ),
        (
            "{\"dependencies\": [\"a\", \"b\"]}".to_string(),
            "serde",
            "*",
            "`dependencies` is not an object",
        ),
        (
            "[1, 2, 3]".to_string(),
            "serde",
            "*",
            "manifest root is not a JSON object",
        ),
        (
            "\"just a string\"".to_string(),
            "serde",
            "*",
            "manifest root is not a JSON object",
        ),
        (
            "   ".to_string(),
            "serde",
            "*",
            "manifest is empty",
        ),
        (
            "".to_string(),
            "serde",
            "*",
            "manifest is empty",
        ),
        (
            "{\n    \"dependencies\": { \"serde\": \"1\"\n}".to_string(),
            "serde",
            "2",
            "unbalanced",
        ),
        (
            "{\n    \"name\": ".to_string(),
            "serde",
            "*",
            "unbalanced",
        ),
        (
            "{ \"a\": }".to_string(),
            "serde",
            "*",
            "missing value after colon",
        ),
        (
            "{ \"a\": \"1\", \"b\": }".to_string(),
            "serde",
            "*",
            "missing value after colon",
        ),
        (
            "// leading comment\n\"stray\"".to_string(),
            "serde",
            "*",
            "manifest root is not a JSON object",
        ),
        (
            "/* c */ 42".to_string(),
            "serde",
            "*",
            "manifest root is not a JSON object",
        ),
        (
            "{ \"dependencies\": { \"a\": } }".to_string(),
            "serde",
            "*",
            "missing value after colon",
        ),
    ]
}

/// Every successful edit case round-trips to the exact expected bytes and parses.
#[test]
fn apply_dependency_edit_matches_expected_output() {
    for (input, name, version, expected) in edit_cases() {
        let actual = apply_dependency_edit(&input, name, version)
            .unwrap_or_else(|e| panic!("edit failed for {name} {version}: {e}\ninput: {input}"));
        assert_eq!(
            actual, expected,
            "edit of {name} {version}\ninput:    {input}\nexpected: {expected}\nactual:   {actual}"
        );
    }
}

/// Every successful edit case still parses with the dependency at the requested version.
#[test]
fn apply_dependency_edit_preserves_parsability() {
    for (input, name, version, expected) in edit_cases() {
        assert_json_with_dep(&expected, name, version);
        let roundtrip = apply_dependency_edit(&expected, name, version).expect("second edit works");
        assert_json_with_dep(&roundtrip, name, version);
        let _ = input;
    }
}

/// Re-editing an already-edited manifest with the same version is idempotent.
#[test]
fn apply_dependency_edit_is_idempotent_for_same_version() {
    for (input, name, version, expected) in edit_cases() {
        let twice = apply_dependency_edit(&expected, name, version).expect("second edit works");
        if input.contains("\"dependencies\"") {
            assert_eq!(
                twice, expected,
                "idempotent re-edit for {name} {version}\nexpected: {expected}\nactual:   {twice}"
            );
        }
    }
}

/// Every error case fails with the expected message.
#[test]
fn apply_dependency_edit_reports_expected_errors() {
    for (input, name, version, expected_msg) in edit_error_cases() {
        let err = apply_dependency_edit(&input, name, version)
            .err()
            .unwrap_or_else(|| panic!("edit should fail for {name} {version} on {input}"));
        assert!(
            err.contains(expected_msg),
            "error {err:?} should contain {expected_msg:?} for input {input}"
        );
    }
}

/// Validation table: (name, version, should_pass).
fn validation_cases() -> Vec<(&'static str, &'static str, bool)> {
    vec![
        ("serde", "*", true),
        ("serde", "1.0.229", true),
        ("serde", ">=1.0, <2.0", true),
        ("with spaces", "1", true),
        ("with-dash", "1", true),
        ("with.dot", "1", true),
        ("with/slash", "1", true),
        ("with:colon", "1", true),
        ("DéjàVu", "1", true),
        ("a", "", false),
        ("a", "   ", false),
        ("a", "has \" quote", false),
        ("a", "has\\backslash", false),
        ("a", "has\nnewline", false),
        ("a", "has\ttab", false),
        ("", "*", false),
        ("   ", "*", false),
        ("\"", "*", false),
        ("na\"me", "*", false),
        ("na\\me", "*", false),
        ("na\nme", "*", false),
        ("na\tme", "*", false),
        ("serde", "1", true),
        ("tokio", "*", true),
        ("rand", "0.8.5", true),
        ("clap", "4.6.6", true),
    ]
}

/// Name and version validation accepts and rejects the table entries.
#[test]
fn validate_dep_table() {
    for (name, version, should_pass) in validation_cases() {
        let result = validate_dep(name, version);
        assert_eq!(
            result.is_ok(),
            should_pass,
            "validate_dep({name:?}, {version:?}) should be ok={should_pass}, got {result:?}"
        );
    }
}

/// Invalid names produce specific error messages.
#[test]
fn validate_dep_error_messages() {
    let empty = validate_dep("", "*").unwrap_err();
    assert!(empty.contains("name must not be empty"));
    let blank = validate_dep("   ", "*").unwrap_err();
    assert!(blank.contains("name must not be empty"));
    let quoted = validate_dep("na\"me", "*").unwrap_err();
    assert!(quoted.contains("double quote"));
    let backslash = validate_dep("na\\me", "*").unwrap_err();
    assert!(backslash.contains("backslash"));
    let control = validate_dep("na\nme", "*").unwrap_err();
    assert!(control.contains("control character"));
    let empty_version = validate_dep("serde", "").unwrap_err();
    assert!(empty_version.contains("version must not be empty"));
    let quote_version = validate_dep("serde", "a\"b").unwrap_err();
    assert!(quote_version.contains("quotes or backslashes"));
    let ctrl_version = validate_dep("serde", "a\nb").unwrap_err();
    assert!(ctrl_version.contains("control character"));
}

/// The scanner skips line comments, block comments, and whitespace.
#[test]
fn scanner_events_table() {
    let cases: Vec<(&str, Vec<ScanEvent>)> = vec![
        (
            "{}",
            vec![
                ScanEvent::Char { ch: '{', at: 0 },
                ScanEvent::Char { ch: '}', at: 1 },
            ],
        ),
        (
            "  {  }  ",
            vec![
                ScanEvent::Char { ch: '{', at: 2 },
                ScanEvent::Char { ch: '}', at: 5 },
            ],
        ),
        (
            "// c\n{}",
            vec![
                ScanEvent::Char { ch: '{', at: 5 },
                ScanEvent::Char { ch: '}', at: 6 },
            ],
        ),
        (
            "/* c */ {}",
            vec![
                ScanEvent::Char { ch: '{', at: 8 },
                ScanEvent::Char { ch: '}', at: 9 },
            ],
        ),
        (
            "\"a\" \"b\"",
            vec![
                ScanEvent::Str { open: 0, content_start: 1, content_end: 2, close: 2 },
                ScanEvent::Str { open: 4, content_start: 5, content_end: 6, close: 6 },
            ],
        ),
        (
            "\"a\\\"b\"",
            vec![ScanEvent::Str { open: 0, content_start: 1, content_end: 5, close: 5 }],
        ),
        (
            "{\"k\": 1}",
            vec![
                ScanEvent::Char { ch: '{', at: 0 },
                ScanEvent::Str { open: 1, content_start: 2, content_end: 3, close: 3 },
                ScanEvent::Char { ch: ':', at: 4 },
                ScanEvent::Char { ch: '1', at: 6 },
                ScanEvent::Char { ch: '}', at: 7 },
            ],
        ),
        (
            "\"/* not a comment */\"",
            vec![ScanEvent::Str { open: 0, content_start: 1, content_end: 20, close: 20 }],
        ),
        (
            "// only comment",
            vec![],
        ),
        ("", vec![]),
    ];
    for (text, expected) in cases {
        let mut scanner = JsonScanner::new(text, 0, text.len());
        let mut actual = Vec::new();
        while let Some(ev) = scanner.next_event() {
            actual.push(ev);
        }
        assert_eq!(actual, expected, "scanner events for {text:?}");
    }
}

/// object_entries walks keys, nested values, and reports the closing brace.
#[test]
fn object_entries_walks_the_root() {
    let text = "{ \"name\": \"x\", \"scripts\": { \"dev\": \"run\" }, \"flag\": true }";
    let (entries, close) = object_entries(text, 0).expect("root walks");
    assert_eq!(entries.len(), 3);
    assert_eq!(entries[0].key, "name");
    assert_eq!(entries[1].key, "scripts");
    assert_eq!(entries[2].key, "flag");
    assert_eq!(&text[entries[0].value_start..entries[0].value_end], "\"x\"");
    assert_eq!(&text[entries[1].value_start..entries[1].value_end], "{ \"dev\": \"run\" }");
    assert_eq!(&text[entries[2].value_start..entries[2].value_end], "true");
    assert_eq!(text.as_bytes()[close] as char, '}');
    assert_eq!(close, text.len() - 1);
}

/// object_entries walks a nested dependencies block independently.
#[test]
fn object_entries_walks_nested_dependencies() {
    let text = "{ \"dependencies\": { \"serde\": \"1\", \"nested\": { \"k\": \"v\" } } }";
    let (root, _) = object_entries(text, 0).expect("root walks");
    assert_eq!(root.len(), 1);
    let deps_open = root[0].value_start;
    assert_eq!(text.as_bytes()[deps_open] as char, '{');
    let (deps, deps_close) = object_entries(text, deps_open).expect("deps walk");
    assert_eq!(deps.len(), 2);
    assert_eq!(deps[0].key, "serde");
    assert_eq!(deps[1].key, "nested");
    assert_eq!(&text[deps[0].value_start..deps[0].value_end], "\"1\"");
    assert_eq!(text.as_bytes()[deps_close] as char, '}');
}

/// object_entries fails on unbalanced input.
#[test]
fn object_entries_rejects_unbalanced_input() {
    assert!(object_entries("{ \"a\": \"1\"", 0).is_err());
    assert!(object_entries("{", 0).is_err());
    assert!(object_entries("{ \"a\": { \"b\": \"1\" }", 0).is_err());
}

/// Adding a dependency to a freshly scaffolded project persists the entry.
#[test]
fn cli_add_persists_to_a_fresh_project() {
    let dir = temp_dir("fresh");
    let project = dir.join("proj");
    run_bucket_in(&dir, &["new", "proj"]);
    let manifest_path = project.join("Bucket.jsonc");
    let before = fs::read_to_string(&manifest_path).unwrap();
    let (out, err, code) = run_bucket_in(&project, &["add", "serde"]);
    assert_eq!(code, 0, "stderr: {err}");
    assert!(err.is_empty());
    assert!(out.contains("Added dependency `serde *` to Bucket.jsonc"));
    let after = fs::read_to_string(&manifest_path).unwrap();
    let expected = before.replace("\"dependencies\": {},", "\"dependencies\": { \"serde\": \"*\"},");
    assert_eq!(after, expected);
    let manifest = bucket::manifest::parse(&after).expect("template manifest parses");
    assert_eq!(manifest.dependencies.get("serde").map(|s| s.as_str()), Some("*"));
    let _ = fs::remove_dir_all(&dir);
}

/// Adding with an explicit version pins it, and re-adding updates the version.
#[test]
fn cli_add_with_version_updates_existing_entry() {
    let dir = temp_dir("version");
    let project = dir.join("proj");
    run_bucket_in(&dir, &["new", "proj"]);
    let (out, _err, code) = run_bucket_in(&project, &["add", "serde", "1.0.229"]);
    assert_eq!(code, 0);
    assert!(out.contains("Added dependency `serde 1.0.229`"));
    let manifest_path = project.join("Bucket.jsonc");
    let pinned = fs::read_to_string(&manifest_path).unwrap();
    assert!(pinned.contains("\"serde\": \"1.0.229\""));
    let manifest = bucket::manifest::parse(&pinned).expect("pinned manifest parses");
    assert_eq!(manifest.dependencies.get("serde").map(|s| s.as_str()), Some("1.0.229"));
    let (out, _err, code) = run_bucket_in(&project, &["add", "serde", "2.0"]);
    assert_eq!(code, 0);
    assert!(out.contains("Added dependency `serde 2.0`"));
    let updated = fs::read_to_string(&manifest_path).unwrap();
    assert!(updated.contains("\"serde\": \"2.0\""));
    assert!(!updated.contains("\"serde\": \"1.0.229\""));
    let updated_manifest = bucket::manifest::parse(&updated).expect("updated manifest parses");
    assert_eq!(updated_manifest.dependencies.get("serde").map(|s| s.as_str()), Some("2.0"));
    let _ = fs::remove_dir_all(&dir);
}

/// Comments elsewhere in the manifest survive an add byte-for-byte.
#[test]
fn cli_add_preserves_comments_outside_the_block() {
    let dir = temp_dir("comments");
    let manifest = "{\n    // project name\n    \"name\": \"c\",\n\n    \"dependencies\": {},\n\n    // tail\n    \"scripts\": {}\n}";
    fs::write(dir.join("Bucket.jsonc"), manifest).unwrap();
    let (_out, err, code) = run_bucket_in(&dir, &["add", "serde"]);
    assert_eq!(code, 0, "stderr: {err}");
    let after = fs::read_to_string(dir.join("Bucket.jsonc")).unwrap();
    assert!(after.contains("// project name"));
    assert!(after.contains("// tail"));
    assert!(after.contains("\"serde\": \"*\""));
    assert_json_with_dep(&after, "serde", "*");
    let _ = fs::remove_dir_all(&dir);
}

/// A manifest without a dependencies block gets one created after scripts.
#[test]
fn cli_add_creates_missing_dependencies_block() {
    let dir = temp_dir("missing-block");
    let manifest = "{\n    \"name\": \"m\",\n    \"scripts\": {\n        \"dev\": \"bucket run\"\n    }\n}";
    fs::write(dir.join("Bucket.jsonc"), manifest).unwrap();
    let (_out, err, code) = run_bucket_in(&dir, &["add", "tokio"]);
    assert_eq!(code, 0, "stderr: {err}");
    let after = fs::read_to_string(dir.join("Bucket.jsonc")).unwrap();
    let expected = "{\n    \"name\": \"m\",\n    \"scripts\": {\n        \"dev\": \"bucket run\"\n    },\n\n    \"dependencies\": {\n        \"tokio\": \"*\"\n    }\n}";
    assert_eq!(after, expected);
    assert_json_with_dep(&after, "tokio", "*");
    let _ = fs::remove_dir_all(&dir);
}

/// Adding without a Bucket.jsonc fails with a clear error.
#[test]
fn cli_add_fails_without_manifest() {
    let dir = temp_dir("missing-manifest");
    let (_out, err, code) = run_bucket_in(&dir, &["add", "serde"]);
    assert_eq!(code, 1);
    assert!(err.contains("add: Bucket.jsonc not found"));
    assert!(!dir.join("Bucket.jsonc").exists());
    let _ = fs::remove_dir_all(&dir);
}

/// Invalid dependency names are rejected by the CLI.
#[test]
fn cli_add_rejects_invalid_names() {
    let dir = temp_dir("bad-names");
    fs::write(dir.join("Bucket.jsonc"), template_manifest()).unwrap();
    let before = fs::read_to_string(dir.join("Bucket.jsonc")).unwrap();
    for name in ["", "   ", "na\"me"] {
        let (_out, err, code) = run_bucket_in(&dir, &["add", name]);
        assert_eq!(code, 1, "name {name:?} should be rejected");
        assert!(err.starts_with("add: "), "stderr should explain: {err}");
    }
    let after = fs::read_to_string(dir.join("Bucket.jsonc")).unwrap();
    assert_eq!(after, before);
    let _ = fs::remove_dir_all(&dir);
}

/// Invalid versions are rejected without touching the manifest.
#[test]
fn cli_add_rejects_invalid_versions() {
    let dir = temp_dir("bad-versions");
    fs::write(dir.join("Bucket.jsonc"), template_manifest()).unwrap();
    let before = fs::read_to_string(dir.join("Bucket.jsonc")).unwrap();
    for version in ["", "a\"b", "a\\b"] {
        let (_out, err, code) = run_bucket_in(&dir, &["add", "serde", version]);
        assert_eq!(code, 1, "version {version:?} should be rejected");
        assert!(err.starts_with("add: "));
    }
    let after = fs::read_to_string(dir.join("Bucket.jsonc")).unwrap();
    assert_eq!(after, before);
    let _ = fs::remove_dir_all(&dir);
}

/// A corrupt manifest fails the add rather than corrupting it further.
#[test]
fn cli_add_fails_on_unparseable_edit_root() {
    let dir = temp_dir("corrupt");
    fs::write(dir.join("Bucket.jsonc"), "{ not json at all").unwrap();
    let (_out, err, code) = run_bucket_in(&dir, &["add", "serde"]);
    assert_eq!(code, 1);
    assert!(err.contains("add:"));
    let _ = fs::remove_dir_all(&dir);
}

/// Dependencies as a non-object value are rejected with a clear message.
#[test]
fn cli_add_rejects_non_object_dependencies() {
    let dir = temp_dir("non-object");
    fs::write(dir.join("Bucket.jsonc"), "{ \"dependencies\": \"nope\" }").unwrap();
    let (_out, err, code) = run_bucket_in(&dir, &["add", "serde"]);
    assert_eq!(code, 1);
    assert!(err.contains("`dependencies` is not an object"));
    let _ = fs::remove_dir_all(&dir);
}

/// Sequential adds accumulate distinct dependencies in one block.
#[test]
fn cli_add_accumulates_multiple_dependencies() {
    let dir = temp_dir("accumulate");
    let manifest = "{\n    \"name\": \"acc\",\n    \"version\": \"0.1.0\",\n    \"dependencies\": {}\n}";
    fs::write(dir.join("Bucket.jsonc"), manifest).unwrap();
    run_bucket_in(&dir, &["add", "serde"]);
    run_bucket_in(&dir, &["add", "tokio", "1"]);
    run_bucket_in(&dir, &["add", "rand"]);
    let after = fs::read_to_string(dir.join("Bucket.jsonc")).unwrap();
    assert!(after.contains("\"rand\": \"*\""));
    assert!(after.contains("\"tokio\": \"1\""));
    assert!(after.contains("\"serde\": \"*\""));
    let parsed = bucket::manifest::parse(&after).expect("accumulated manifest parses");
    assert_eq!(parsed.dependencies.len(), 3);
    assert_eq!(parsed.dependencies.get("serde").map(|s| s.as_str()), Some("*"));
    assert_eq!(parsed.dependencies.get("tokio").map(|s| s.as_str()), Some("1"));
    assert_eq!(parsed.dependencies.get("rand").map(|s| s.as_str()), Some("*"));
    let _ = fs::remove_dir_all(&dir);
}

/// The default version is the star wildcard.
#[test]
fn cli_add_defaults_to_star_version() {
    let dir = temp_dir("default-star");
    fs::write(dir.join("Bucket.jsonc"), "{ \"dependencies\": {} }").unwrap();
    let (out, _err, code) = run_bucket_in(&dir, &["add", "serde"]);
    assert_eq!(code, 0);
    assert!(out.contains("serde *"));
    let after = fs::read_to_string(dir.join("Bucket.jsonc")).unwrap();
    assert!(after.contains("\"serde\": \"*\""));
    let _ = fs::remove_dir_all(&dir);
}
