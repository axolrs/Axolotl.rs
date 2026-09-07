#!/usr/bin/env python3
"""Generate the 10 real benchmark harness crates under benchmarks/*/bench/.
Each harness compiles the .axol port via axolc-core + rustc, builds the
hand-written Rust reference, runs both with the same workload, asserts the
outputs match and that axol_time / rust_time <= 1.05, and prints a results
line. Also removes the old fake bench/run.rs stubs."""

import os
import shutil

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))

PORTS = {
    "git": 4_000_000,
    "database": 3_000_000,
    "redis": 5_000_000,
    "shell": 4_000_000,
    "webserver": 6_000_000,
    "texteditor": 6_000_000,
    "programminglanguage": 6_000_000,
    "regexengine": 5_000_000,
    "docker": 3_000_000,
    "emulator": 6_000_000,
}

WORKLOAD_DESC = {
    "git": "content-addressed blob store: a djb2-style 64-step blob hash per object, a store pass that pushes every blob hash into a vector, and a verify pass that re-hashes and folds the stored values back in",
    "database": "open-addressing key-value store: a 32-step key hash, a grow-to-2N table, a linear-probe insert pass, and a linear-probe lookup pass over N distinct keys",
    "redis": "command dispatch over a 4096-slot cache: a 16-step slot hash per key, SET/GET selection by command id, and eviction/hit accounting on slot collisions",
    "shell": "command tokenizer: a fixed 32-word table, a per-command argument sweep (1-8 words), and a divide-by-3 word-length loop per token",
    "webserver": "request router: per-request method/path derivation, a two-level routing function returning status codes, an access-log push for 200/404, and a masked status sum",
    "texteditor": "line buffer with undo: a history push per inserted line, a 1-in-64 line overwrite, a masked checksum, and a bounded undo loop at the end",
    "programminglanguage": "stack VM interpreter: an 8-slot stack, opcode dispatch (push/add/mul/pop/read) keyed on i mod 8, stack-pointer wrap, and a final stack fold",
    "regexengine": "backtracking matcher: a fixed 64-symbol input, a greedy a-star sweep, and a backtracking b search per probe, with a matched count and masked checksum",
    "docker": "layered filesystem digests: a 48-step layer digest per layer, a push into the layer vector, a 1-in-64 zero-layer count, and a bounded read pass folding digests back in",
    "emulator": "CHIP-8-style decode/execute loop: a 4096-word memory, opcode fetch across the program counter, nibble decode via multiply/divide, register writes, jumps, and a masked checksum",
}

CARGO_TOML = """[package]
name = "{name}-bench"
version = "0.1.0"
edition = "2024"
license = "MIT OR Apache-2.0"
authors = ["PascalElixir / axolrs"]

[dependencies]
axolc-core = {{ path = "../../../crates/axolc-core" }}

[workspace]
"""

MAIN_RS = """// Owner: PascalElixir / axolrs (GitHub org)
// File: benchmarks/{name}/bench - real comparison harness: axolc emit + rustc vs the hand-written reference.

use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::time::Instant;

const PORT: &str = "{name}";
const DEFAULT_WORKLOAD: u64 = {workload};

/// Run a tool, falling back to ~/.cargo/bin when PATH does not carry it.
fn run_tool(name: &str, args: &[String]) -> std::process::Output {{
    let home = env::var("HOME").unwrap_or_else(|_| ".".to_string());
    let candidates = [name.to_string(), format!("{{}}/.cargo/bin/{{}}", home, name)];
    let mut last = None;
    for cand in candidates {{
        match Command::new(&cand).args(args).output() {{
            Ok(out) => return out,
            Err(e) => last = Some(format!("{{}}: {{}}", cand, e)),
        }}
    }}
    panic!("could not execute {{}}: {{:?}}", name, last);
}}

/// Run one benchmark binary, returning (elapsed milliseconds, stdout).
fn timed_run(bin: &PathBuf, args: &[String]) -> (f64, String) {{
    let start = Instant::now();
    let out = Command::new(bin).args(args).output().expect("run benchmark binary");
    let ms = start.elapsed().as_secs_f64() * 1000.0;
    assert!(out.status.success(), "{{}} exited with {{:?}}", bin.display(), out.status.code());
    (ms, String::from_utf8_lossy(&out.stdout).to_string())
}}

fn main() {{
    let mut workload = DEFAULT_WORKLOAD;
    let mut args = env::args().skip(1);
    while let Some(arg) = args.next() {{
        if arg == "--workload" {{
            if let Some(v) = args.next() {{
                workload = v.parse().unwrap_or(DEFAULT_WORKLOAD);
            }}
        }} else if let Ok(v) = arg.parse::<u64>() {{
            workload = v;
        }}
    }}

    let bench_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let port_dir = bench_dir.parent().expect("bench dir parent").to_path_buf();

    let axol_src_path = port_dir.join("axol/src/main.axol");
    let axol_src = fs::read_to_string(&axol_src_path).expect("read axol port source");
    let src = axol_src.replace("__WORKLOAD__", &workload.to_string());
    let (rust, diags) = axolc_core::compile_to_rust(&src, 0);
    assert!(!diags.has_errors(), "{{}} axol port did not compile: {{:?}}", PORT, diags.items);

    let target = bench_dir.join("target");
    fs::create_dir_all(&target).expect("create bench target dir");
    let gen_path = target.join(format!("gen_{{}}.rs", PORT));
    fs::write(&gen_path, &rust).expect("write generated rust");
    let axol_bin = target.join(format!("axol_{{}}", PORT));
    let rustc_args: Vec<String> = [
        "--edition".to_string(),
        "2024".to_string(),
        "-O".to_string(),
        "-o".to_string(),
        axol_bin.to_string_lossy().into_owned(),
        gen_path.to_string_lossy().into_owned(),
    ]
    .to_vec();
    let rustc_out = run_tool("rustc", &rustc_args);
    assert!(
        rustc_out.status.success(),
        "rustc rejected the generated code:\\n{{}}",
        String::from_utf8_lossy(&rustc_out.stderr)
    );

    let rust_manifest = port_dir.join("rust/Cargo.toml");
    let cargo_args: Vec<String> = [
        "build".to_string(),
        "--release".to_string(),
        "--manifest-path".to_string(),
        rust_manifest.to_string_lossy().into_owned(),
    ]
    .to_vec();
    let cargo_out = run_tool("cargo", &cargo_args);
    assert!(
        cargo_out.status.success(),
        "cargo build failed for the reference:\\n{{}}",
        String::from_utf8_lossy(&cargo_out.stderr)
    );
    let rust_bin = port_dir.join(format!("rust/target/release/{{}}-rust", PORT));

    let warg: Vec<String> = vec![workload.to_string()];
    let _ = timed_run(&axol_bin, &[]);
    let _ = timed_run(&rust_bin, &warg);

    let mut axol_best = f64::MAX;
    let mut rust_best = f64::MAX;
    let mut axol_out = String::new();
    let mut rust_out = String::new();
    for _ in 0..3 {{
        let (ms, out) = timed_run(&axol_bin, &[]);
        axol_best = axol_best.min(ms);
        axol_out = out;
        let (ms, out) = timed_run(&rust_bin, &warg);
        rust_best = rust_best.min(ms);
        rust_out = out;
    }}

    assert_eq!(
        axol_out, rust_out,
        "{{}} outputs diverged between the port and the reference",
        PORT
    );

    let ratio = axol_best / rust_best;
    println!(
        "{{}}: rust={{:.1}}ms axol={{:.1}}ms ratio={{:.3}}",
        PORT, rust_best, axol_best, ratio
    );
    println!("{{}}: workload={{}} stdout={{}}", PORT, workload, rust_out.trim());
    assert!(
        ratio <= 1.05,
        "{{}}: axolotl is {{:.1}}% slower than rust (ratio {{:.3}} > 1.05)",
        PORT,
        (ratio - 1.0) * 100.0,
        ratio
    );
}}
"""

README_MD = """# {name} benchmark harness

Owner: PascalElixir / axolrs.

## What it measures

{name} ({desc}).

Both sides run the identical algorithm: `../axol/src/main.axol` is the Axolotl
port and `../rust/src/main.rs` is the hand-written Rust reference. The two
programs print the same result line, which the harness compares byte-for-byte.

## How it works

1. Reads `../axol/src/main.axol`, substitutes the `__WORKLOAD__` marker with
   the iteration count, and compiles it with `axolc_core::compile_to_rust`.
2. Writes the generated Rust to `target/gen_{name}.rs` and compiles it with
   `rustc --edition 2024 -O` (the axolc emit + rustc pipeline).
3. Builds `../rust` (the hand-written reference) with `cargo build --release`.
4. Runs one warmup of each binary, then three timed runs of each, keeping the
   best (minimum) time per side.
5. Asserts the two stdout lines are identical and that
   `axol_time / rust_time <= 1.05`.

## Usage

```sh
cargo run --manifest-path benchmarks/{name}/bench/Cargo.toml --release -- [--workload N]
```

The default workload is {workload} iterations, tuned to roughly 100-500ms per
run. Use `--workload` (or a bare number) to change the size; the axol port is
recompiled with the substituted bound each run.

## Result line

```
{name}: rust=Xms axol=Yms ratio=Z
{name}: workload=N stdout=<the shared result line>
```

The harness exits non-zero when the outputs diverge or the ratio exceeds 1.05.
"""


def main():
    for name, workload in PORTS.items():
        bench = os.path.join(ROOT, "benchmarks", name, "bench")
        os.makedirs(os.path.join(bench, "src"), exist_ok=True)
        with open(os.path.join(bench, "Cargo.toml"), "w") as f:
            f.write(CARGO_TOML.format(name=name))
        with open(os.path.join(bench, "src", "main.rs"), "w") as f:
            f.write(MAIN_RS.format(name=name, workload=workload))
        with open(os.path.join(bench, "README.md"), "w") as f:
            f.write(
                README_MD.format(
                    name=name, workload="{:,}".format(workload), desc=WORKLOAD_DESC[name]
                )
            )
        old = os.path.join(bench, "run.rs")
        if os.path.exists(old):
            os.remove(old)
            print("removed fake stub %s" % old)
        print("wrote %s (default workload %d)" % (bench, workload))


if __name__ == "__main__":
    main()
