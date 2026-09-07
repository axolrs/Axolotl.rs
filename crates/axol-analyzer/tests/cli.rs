// Owner: PascalElixir / axolrs (GitHub org)
// File: CLI and stdio-transport tests for the axol-analyzer (Gills) binary.

use std::io::{BufRead, BufReader, Read, Write};
use std::process::{Child, Command, Stdio};
use std::sync::mpsc::{self, Receiver};
use std::time::Duration;

use serde_json::Value;

fn analyzer_binary() -> String {
    let mut path = std::env::current_exe().unwrap();
    path.pop();
    path.pop();
    path.join("axol-analyzer").to_string_lossy().to_string()
}

fn run_analyzer(args: &[&str]) -> (String, String, i32) {
    let output = Command::new(analyzer_binary())
        .args(args)
        .output()
        .expect("run axol-analyzer");
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    let code = output.status.code().unwrap_or(-1);
    (stdout, stderr, code)
}

fn write_temp_axol(name: &str, src: &str) -> String {
    let mut path = std::env::temp_dir();
    path.push(name);
    std::fs::write(&path, src).unwrap();
    path.to_string_lossy().to_string()
}

/// A spawned Gills stdio server with a background framed-message reader.
struct LspChild {
    child: Child,
    rx: Receiver<Value>,
}

impl LspChild {
    /// Spawn the server and start reading framed JSON-RPC messages.
    fn spawn() -> LspChild {
        let mut child = Command::new(analyzer_binary())
            .arg("stdio")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .expect("spawn axol-analyzer stdio");
        let stdout = child.stdout.take().expect("stdout piped");
        let (tx, rx) = mpsc::channel();
        std::thread::spawn(move || pump_messages(stdout, tx));
        LspChild { child, rx }
    }

    /// Send one JSON-RPC message body with Content-Length framing.
    fn send(&mut self, body: &str) {
        let frame = format!("Content-Length: {}\r\n\r\n{}", body.len(), body);
        let stdin = self.child.stdin.as_mut().expect("stdin piped");
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
        let _ = self.recv_discard();
        self.child.wait().expect("server exits").code().unwrap_or(-1)
    }

    /// Drain any pending messages without failing on channel closure.
    fn recv_discard(&self) -> Option<Value> {
        self.rx.recv_timeout(Duration::from_secs(5)).ok()
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

const GAME_SRC: &str = "\
Player = struct
    hp: Int
end

fn damage(player: & var Player, amount: Int)
    player.hp = player.hp - amount
end

fn take(x: move Int)
    print(x)
end

fn main()
    let p = Player { hp = 100 }
    damage(p, 10)
    let a = 5
    take(a)
    print(a)
end
";

/// `info` prints the Gills banner and exits zero.
#[test]
fn info_command() {
    let (out, _err, code) = run_analyzer(&["info"]);
    assert_eq!(code, 0);
    assert!(out.contains("Gills"));
}

/// `info` reports the toolchain version.
#[test]
fn info_contains_version() {
    let (out, _, _) = run_analyzer(&["info"]);
    assert!(out.contains("0.1.0"));
}

/// `info` names the owners.
#[test]
fn info_contains_owner() {
    let (out, _, _) = run_analyzer(&["info"]);
    assert!(out.contains("PascalElixir"));
    assert!(out.contains("axolrs"));
}

/// `check` on a clean file exits zero with no diagnostics.
#[test]
fn check_clean_file_exits_zero() {
    let path = write_temp_axol("gills_clean.axol", "fn main()\n    print(1)\nend\n");
    let (_out, err, code) = run_analyzer(&["check", &path]);
    assert_eq!(code, 0);
    assert!(err.is_empty());
}

/// `check` on a file with errors prints diagnostics to stderr and exits one.
#[test]
fn check_broken_file_exits_one() {
    let path = write_temp_axol("gills_broken.axol", "fn main()\n    let x =\nend\n");
    let (_out, err, code) = run_analyzer(&["check", &path]);
    assert_eq!(code, 1);
    assert!(err.contains("error"));
    assert!(err.contains(&path));
}

/// `check` prints ownership error codes like E0101.
#[test]
fn check_reports_ownership_error_code() {
    let src = "fn take(x: move Int)\n    print(x)\nend\n\nfn main()\n    let a = 1\n    take(a)\n    print(a)\nend\n";
    let path = write_temp_axol("gills_moved.axol", src);
    let (_out, err, code) = run_analyzer(&["check", &path]);
    assert_eq!(code, 1);
    assert!(err.contains("E0101"));
    assert!(err.contains("use of moved value"));
}

/// `check` on an unreadable file exits with a distinct code.
#[test]
fn check_missing_file_exits_two() {
    let (_out, err, code) = run_analyzer(&["check", "/nonexistent/gills_probe.axol"]);
    assert_eq!(code, 2);
    assert!(err.contains("cannot read"));
}

/// `stdio` with an immediately closed stdin exits zero.
#[test]
fn stdio_eof_exits_zero() {
    let (_out, _err, code) = run_analyzer(&["stdio"]);
    assert_eq!(code, 0);
}

/// `stdio` with no subcommand behaves the same as `stdio`.
#[test]
fn stdio_is_the_default_subcommand() {
    let output = Command::new(analyzer_binary())
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("spawn default");
    let mut child = output;
    drop(child.stdin.take());
    let code = child.wait().expect("server exits").code().unwrap_or(-1);
    assert_eq!(code, 0);
}

/// A full framed-stdio LSP session: initialize, open, diagnostics, hover, shutdown, exit.
#[test]
fn stdio_full_lifecycle() {
    let mut lsp = LspChild::spawn();
    lsp.send(r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"processId":null,"rootUri":null,"capabilities":{}}}"#);
    let init = lsp.recv();
    assert_eq!(init["id"], 1);
    assert!(init["result"]["capabilities"]["hoverProvider"].as_bool().unwrap());
    assert!(init["result"]["capabilities"]["definitionProvider"].as_bool().unwrap());
    assert!(init["result"]["capabilities"]["documentSymbolProvider"].as_bool().unwrap());
    assert!(init["result"]["capabilities"]["renameProvider"].as_bool().unwrap());
    assert!(init["result"]["capabilities"]["referencesProvider"].as_bool().unwrap());
    assert_eq!(init["result"]["serverInfo"]["name"], "Gills");

    lsp.send(r#"{"jsonrpc":"2.0","method":"initialized","params":{}}"#);
    let did_open = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "textDocument/didOpen",
        "params": {
            "textDocument": {
                "uri": "file:///session.axol",
                "languageId": "axol",
                "version": 1,
                "text": GAME_SRC,
            }
        }
    })
    .to_string();
    lsp.send(&did_open);
    let publish = lsp.recv();
    assert_eq!(publish["method"], "textDocument/publishDiagnostics");
    let diags = publish["params"]["diagnostics"].as_array().unwrap();
    assert!(diags.iter().any(|d| d["code"] == "E0101" && d["severity"] == 1));

    let hover = serde_json::json!({
        "jsonrpc": "2.0",
        "id": 2,
        "method": "textDocument/hover",
        "params": {
            "textDocument": { "uri": "file:///session.axol" },
            "position": { "line": 5, "character": 7 }
        }
    })
    .to_string();
    lsp.send(&hover);
    let hover_reply = lsp.recv();
    assert_eq!(hover_reply["id"], 2);
    let value = hover_reply["result"]["contents"]["value"].as_str().unwrap();
    assert!(value.contains("player: &mut Player"));
    assert!(value.contains("`&mut`"));

    lsp.send(r#"{"jsonrpc":"2.0","id":3,"method":"shutdown"}"#);
    let shutdown_reply = lsp.recv();
    assert_eq!(shutdown_reply["id"], 3);
    assert!(shutdown_reply.get("result").is_some());

    lsp.send(r#"{"jsonrpc":"2.0","method":"exit"}"#);
    let code = lsp.finish();
    assert_eq!(code, 0);
}

/// A didChange with a full-text replacement republishes cleared diagnostics.
#[test]
fn stdio_did_change_updates_diagnostics() {
    let mut lsp = LspChild::spawn();
    lsp.send(r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"processId":null,"capabilities":{}}}"#);
    let _ = lsp.recv();
    lsp.send(r#"{"jsonrpc":"2.0","method":"initialized","params":{}}"#);
    let did_open = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "textDocument/didOpen",
        "params": {
            "textDocument": {
                "uri": "file:///session2.axol",
                "languageId": "axol",
                "version": 1,
                "text": "fn take(x: move Int)\n    print(x)\nend\n\nfn main()\n    let a = 1\n    take(a)\n    print(a)\nend\n",
            }
        }
    })
    .to_string();
    lsp.send(&did_open);
    let publish = lsp.recv();
    assert!(
        publish["params"]["diagnostics"]
            .as_array()
            .unwrap()
            .iter()
            .any(|d| d["code"] == "E0101")
    );

    let fixed = "fn take(x: move Int)\n    print(x)\nend\n\nfn main()\n    let a = 1\n    peek(a)\nend\n";
    let did_change = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "textDocument/didChange",
        "params": {
            "textDocument": {"uri": "file:///session2.axol", "version": 2},
            "contentChanges": [{"text": fixed}]
        }
    })
    .to_string();
    lsp.send(&did_change);
    let republish = lsp.recv();
    assert_eq!(republish["params"]["diagnostics"].as_array().unwrap().len(), 0);
    assert_eq!(republish["params"]["version"], 2);

    lsp.send(r#"{"jsonrpc":"2.0","method":"exit"}"#);
    let code = lsp.finish();
    assert_eq!(code, 0);
}
