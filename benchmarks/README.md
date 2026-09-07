# Axolotl Build-Your-Own-X Benchmark Suite

Owner: PascalElixir / axolrs (GitHub org)
License: MIT OR Apache-2.0

Twenty build-your-own-x ports written in both Rust (reference) and Axolotl (port),
side by side. Each port has the same algorithm and the same expected output.
The bench harness in `bench/run.sh` asserts the two outputs are byte-identical
and that `axol_time / rust_time <= 1.05`.

## The 20 Ports

### Original 10 (required by the master brief)

| # | Port | Why |
|---|------|-----|
| 1 | `git` | Content-addressed blob store, hashing, tree/commit objects |
| 2 | `database` | Hash-indexed pager with leaf pages and overflow handling |
| 3 | `redis` | In-memory KV store with expiry and OOM eviction |
| 4 | `shell` | POSIX-like token pipeline simulator with redirects |
| 5 | `webserver` | HTTP/1.1 request dispatch simulator with router |
| 6 | `texteditor` | Kilo-style row buffer with insert/delete/split |
| 7 | `programminglanguage` | Stack-based bytecode VM |
| 8 | `regexengine` | NFA matcher with literal-char patterns |
| 9 | `docker` | Layer store + container lifecycle + cgroup accounting |
| 10 | `emulator` | CHIP-8 full opcode set with 500k cycles |

### Additional 10 (per user directive: 2-3x more)

| # | Port | Why |
|---|------|-----|
| 11 | `json` | JSON-like tree builder with template-driven shapes |
| 12 | `kvstore` | LSM-tree-style store with memtable flush to sstables |
| 13 | `cron` | 5-field cron scheduler with bitmask field matching |
| 14 | `grep` | Integer subsequence matcher (line-by-line scan) |
| 15 | `neuralnet` | Feedforward neural network forward pass (integer math) |
| 16 | `physics` | 2D N-body gravity simulator (integer math, scaled) |
| 17 | `renderer` | Software 3D wireframe rasterizer with Bresenham line drawing |
| 18 | `blockchain` | Proof-of-work chain with mine + verify across 100k blocks |
| 19 | `bittorrent` | Piece-hashing swarm tracker with peer announce + completion |
| 20 | `compiler` | Stack-based bytecode VM running a factorial-like program |

## Layout

Each port has:

- `rust/` - hand-written Rust reference (`cargo build --release`)
- `axol/` - Axolotl port (compiles via `axolc emit-rust` + `rustc -O`)
- `bench/run.sh` - the 5% gate harness

## Running

```bash
# Build axolc first
cargo build -p axolc --release

# Run one benchmark
cd benchmarks/git
bash bench/run.sh

# Run all benchmarks
bash run_all.sh   # (in this directory)
```

## The 5% Performance Gate

For every port:

```
axol_time / rust_time <= 1.05
```

The bench harness compiles both versions, runs them on the same workload,
diffs their outputs byte-for-byte, then times best-of-5 each. The CI
fails on regression.

## Implementation Notes

The Axolotl ports are written to operate within the syntax supported by
the current `axolc` compiler (release 0.1.0-moss). Specifically:

- All `Vec` declarations are local to `main()` (axolc does not emit
  global `Vec` declarations).
- No `^`, `<<`, or `>>` operators in expressions (only `+ - * / % & |`).
- All `fn` parameters are typed (`Int`, `Float`, `[Int]`, etc.).
- Length comparisons use manually-tracked `i64` counters (because
  `arr.len()` returns `usize` in Rust and `axolc` does not auto-cast).

The Rust references mirror the .axol structure exactly so outputs match.
