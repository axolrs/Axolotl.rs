// Owner: PascalElixir / axolrs (GitHub org)
// File: crates/bucket/tests/gills_tests.rs - LSP spawn tests for the gills command.

use bucket::gills;
use std::io::{BufRead, BufReader, Read, Write};
use std::path::PathBuf;
use std::process::{Child, ChildStdin, Command, Stdio};
use std::sync::mpsc::{self, Receiver};
use std::time::Duration;

use serde_json::Value;

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

/// Run the bucket binary with the given arguments and capture its output.
fn run_bucket(args: &[&str]) -> (String, String, i32) {
    let output = Command::new(bucket_binary())
        .args(args)
        .output()
        .expect("run bucket");
    (
        String::from_utf8_lossy(&output.stdout).to_string(),
        String::from_utf8_lossy(&output.stderr).to_string(),
        output.status.code().unwrap_or(-1),
    )
}

/// A spawned `bucket gills --stdio` with a background framed-message reader.
struct GillsChild {
    child: Child,
    rx: Receiver<Value>,
}

impl GillsChild {
    /// Spawn bucket gills --stdio with piped stdio and start reading framed messages.
    fn spawn() -> GillsChild {
        let mut child = Command::new(bucket_binary())
            .args(["gills", "--stdio"])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .expect("spawn bucket gills --stdio");
        let stdout = child.stdout.take().expect("stdout piped");
        let (tx, rx) = mpsc::channel();
        std::thread::spawn(move || pump_messages(stdout, tx));
        GillsChild { child, rx }
    }

    /// Take the child's stdin handle for framed writes.
    fn stdin(&mut self) -> &mut ChildStdin {
        self.child.stdin.as_mut().expect("stdin piped")
    }

    /// Send one JSON-RPC message body with Content-Length framing.
    fn send(&mut self, body: &str) {
        let frame = format!("Content-Length: {}\r\n\r\n{}", body.len(), body);
        let stdin = self.stdin();
        stdin.write_all(frame.as_bytes()).expect("write frame");
        stdin.flush().expect("flush frame");
    }

    /// Receive the next framed message, failing after a bounded wait.
    fn recv(&self) -> Value {
        self.rx
            .recv_timeout(Duration::from_secs(30))
            .expect("server reply within 30s")
    }

    /// Close stdin and collect the process exit status.
    fn finish(&mut self) -> i32 {
        drop(self.child.stdin.take());
        self.child.wait().expect("bucket gills exits").code().unwrap_or(-1)
    }
}

/// Read Content-Length framed JSON-RPC messages into a channel.
fn pump_messages<R: Read + Send + 'static>(stdout: R, tx: mpsc::Sender<Value>) {
    let mut reader = BufReader::new(stdout);
    loop {
        let mut content_length: Option<usize> = None;
        loop {
            let mut line = String::new();
            match reader.read_line(&mut line) {
                Ok(0) => return,
                Ok(_) => {}
                Err(_) => return,
            }
            if line.trim_end().is_empty() {
                break;
            }
            if let Some(rest) = line.to_ascii_lowercase().strip_prefix("content-length:") {
                content_length = rest.trim().parse().ok();
            }
        }
        let Some(len) = content_length else { continue };
        let mut body = vec![0u8; len];
        if reader.read_exact(&mut body).is_err() {
            return;
        }
        if let Ok(value) = serde_json::from_slice(&body) {
            if tx.send(value).is_err() {
                return;
            }
        }
    }
}

/// A full framed-stdio handshake through bucket gills: initialize, shutdown, exit.
#[test]
fn gills_stdio_handshake_reaches_the_analyzer() {
    ensure_sibling("axol-analyzer");
    let mut gills_proc = GillsChild::spawn();
    gills_proc.send(
        r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"processId":null,"rootUri":null,"capabilities":{}}}"#,
    );
    let init = gills_proc.recv();
    assert_eq!(init["id"], 1);
    assert_eq!(init["jsonrpc"], "2.0");
    assert_eq!(init["result"]["serverInfo"]["name"], "Gills");
    assert!(init["result"]["serverInfo"]["version"].is_string());
    assert!(init["result"]["capabilities"]["hoverProvider"].as_bool().unwrap());
    assert!(init["result"]["capabilities"]["completionProvider"].is_object());
    assert!(init["result"]["capabilities"]["definitionProvider"].as_bool().unwrap());
    assert!(init["result"]["capabilities"]["referencesProvider"].as_bool().unwrap());
    assert!(init["result"]["capabilities"]["documentSymbolProvider"].as_bool().unwrap());
    assert!(init["result"]["capabilities"]["renameProvider"].as_bool().unwrap());
    gills_proc.send(r#"{"jsonrpc":"2.0","method":"initialized","params":{}}"#);
    gills_proc.send(r#"{"jsonrpc":"2.0","id":2,"method":"shutdown"}"#);
    let shutdown_reply = gills_proc.recv();
    assert_eq!(shutdown_reply["id"], 2);
    assert!(shutdown_reply.get("result").is_some());
    assert!(shutdown_reply.get("error").is_none());
    gills_proc.send(r#"{"jsonrpc":"2.0","method":"exit"}"#);
    let code = gills_proc.finish();
    assert_eq!(code, 0);
}

/// A didOpen through the passthrough publishes diagnostics for a moved value.
#[test]
fn gills_stdio_passthrough_publishes_diagnostics() {
    ensure_sibling("axol-analyzer");
    let mut gills_proc = GillsChild::spawn();
    gills_proc.send(
        r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"processId":null,"rootUri":null,"capabilities":{}}}"#,
    );
    let _ = gills_proc.recv();
    gills_proc.send(r#"{"jsonrpc":"2.0","method":"initialized","params":{}}"#);
    let did_open = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "textDocument/didOpen",
        "params": {
            "textDocument": {
                "uri": "file:///gills_probe.axol",
                "languageId": "axol",
                "version": 1,
                "text": "fn take(x: move Int)\n    print(x)\nend\n\nfn main()\n    let a = 1\n    take(a)\n    print(a)\nend\n",
            }
        }
    })
    .to_string();
    gills_proc.send(&did_open);
    let publish = gills_proc.recv();
    assert_eq!(publish["method"], "textDocument/publishDiagnostics");
    assert_eq!(publish["params"]["uri"], "file:///gills_probe.axol");
    let diagnostics = publish["params"]["diagnostics"].as_array().unwrap();
    assert!(!diagnostics.is_empty());
    assert!(diagnostics.iter().any(|d| d["code"] == "E0101"));
    assert!(diagnostics.iter().any(|d| d["severity"] == 1));
    gills_proc.send(r#"{"jsonrpc":"2.0","id":3,"method":"shutdown"}"#);
    let _ = gills_proc.recv();
    gills_proc.send(r#"{"jsonrpc":"2.0","method":"exit"}"#);
    let code = gills_proc.finish();
    assert_eq!(code, 0);
}

/// gills without --stdio refuses to start and explains the only transport.
#[test]
fn gills_without_stdio_flag_exits_one() {
    let (out, err, code) = run_bucket(&["gills"]);
    assert_eq!(code, 1);
    assert!(out.is_empty());
    assert!(err.contains("stdio is the only transport today"));
    assert!(err.contains("bucket gills --stdio"));
}

/// gills cannot find the analyzer when run from an isolated directory without PATH access.
#[test]
fn gills_reports_missing_analyzer() {
    ensure_sibling("axol-analyzer");
    let isolated = std::env::temp_dir().join(format!("bucket-gills-isolated-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&isolated);
    std::fs::create_dir_all(&isolated).unwrap();
    let copied = isolated.join("bucket");
    std::fs::copy(bucket_binary(), &copied).unwrap();
    let output = Command::new(&copied)
        .args(["gills", "--stdio"])
        .env("PATH", "/usr/bin:/bin")
        .output()
        .expect("run isolated bucket gills");
    let err = String::from_utf8_lossy(&output.stderr).to_string();
    assert_eq!(output.status.code(), Some(1));
    assert!(err.contains("gills: axol-analyzer not found on PATH"));
    assert!(err.contains("cargo install --path crates/axol-analyzer"));
    let _ = std::fs::remove_dir_all(&isolated);
}

/// The analyzer resolver finds the sibling binary in the shared target directory.
#[test]
fn resolve_analyzer_finds_the_sibling_binary() {
    ensure_sibling("axol-analyzer");
    let resolved = gills::resolve_analyzer().expect("analyzer resolves");
    assert!(resolved.contains("axol-analyzer"));
    assert!(std::path::Path::new(&resolved).is_file());
}

/// The transport help message names the required flag.
#[test]
fn transport_help_message_is_actionable() {
    let msg = gills::transport_help_message();
    assert!(msg.starts_with("gills: "));
    assert!(msg.contains("stdio"));
    assert!(msg.contains("bucket gills --stdio"));
}

/// The missing-analyzer message names the install command.
#[test]
fn missing_analyzer_message_is_actionable() {
    let msg = gills::missing_analyzer_message();
    assert!(msg.starts_with("gills: "));
    assert!(msg.contains("axol-analyzer not found"));
    assert!(msg.contains("cargo install --path crates/axol-analyzer"));
}
