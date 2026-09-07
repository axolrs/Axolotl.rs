#!/usr/bin/env bash
# Owner: PascalElixir / axolrs (GitHub org)
# File: benchmarks/git/bench/run.sh - 5% gate harness for git.
#
# Builds the Rust reference, emits+compiles the Axolotl port through axolc,
# diffs the outputs byte-for-byte, then times best-of-5 each and asserts
# axol_time / rust_time <= 1.05.

set -euo pipefail

cd "$(dirname "$0")/.."

REPO_ROOT="$(cd ../.. && pwd)"
AXOLC="${AXOLC:-$REPO_ROOT/target/release/axolc}"

if [[ ! -x "$AXOLC" ]]; then
    echo "[git] axolc not found at $AXOLC; build with 'cargo build -p axolc --release'." >&2
    exit 2
fi

echo "[git] building rust reference..."
( cd rust && cargo build --release --quiet 2>&1 | tail -3 || true )
RUST_BIN="rust/target/release/git-rust"
[[ -x "$RUST_BIN" ]] || { echo "[git] rust binary missing" >&2; exit 2; }

echo "[git] emitting+compiling axolotl port..."
"$AXOLC" emit-rust axol/src/main.axol > /tmp/git_axol.rs 2>/tmp/git.err || {
    echo "[git] emit-rust failed:" >&2
    cat /tmp/git.err >&2
    exit 2
}
rustc -O -C opt-level=3 -C lto=thin -C codegen-units=1 -C panic=abort -C strip=symbols /tmp/git_axol.rs -o /tmp/git-axol 2>/tmp/git.err || {
    echo "[git] rustc failed:" >&2
    cat /tmp/git.err >&2
    exit 2
}
[[ -x "/tmp/git-axol" ]] || { echo "[git] axol binary missing" >&2; exit 2; }

echo "[git] diffing outputs..."
WORKLOAD="${WORKLOAD:-default}"
RUST_OUT="$("$RUST_BIN" "$WORKLOAD" 2>&1)"
AXOL_OUT="$(/tmp/git-axol "$WORKLOAD" 2>&1)"
if [[ "$RUST_OUT" != "$AXOL_OUT" ]]; then
    echo "[git] OUTPUT MISMATCH" >&2
    diff <(echo "$RUST_OUT") <(echo "$AXOL_OUT") | head -40 >&2
    exit 1
fi
echo "[git] outputs match."

echo "[git] timing best-of-5..."
best_rust=999999999
best_axol=999999999
for i in 1 2 3 4 5; do
    start=$(date +%s%N)
    "$RUST_BIN" "$WORKLOAD" > /dev/null 2>&1
    end=$(date +%s%N)
    bc=$(( (end - start) / 1000000 ))
    if [[ $bc -lt $best_rust ]]; then best_rust=$bc; fi
    start=$(date +%s%N)
    /tmp/git-axol "$WORKLOAD" > /dev/null 2>&1
    end=$(date +%s%N)
    bc=$(( (end - start) / 1000000 ))
    if [[ $bc -lt $best_axol ]]; then best_axol=$bc; fi
done
ratio=$(awk -v a="$best_axol" -v r="$best_rust" 'BEGIN{printf "%.3f", a/r}')
echo "[git] rust=${best_rust}ms axol=${best_axol}ms ratio=$ratio"

awk -v r="$ratio" 'BEGIN{ if (r>1.05) exit 1; else exit 0 }' || {
    echo "[git] REGRESSION: ratio $ratio > 1.05" >&2
    exit 1
}
echo "[git] PASS (ratio $ratio <= 1.05)"
