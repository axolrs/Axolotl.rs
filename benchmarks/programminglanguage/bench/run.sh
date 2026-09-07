#!/usr/bin/env bash
# Owner: PascalElixir / axolrs (GitHub org)
# File: benchmarks/programminglanguage/bench/run.sh - 5% gate harness for programminglanguage.
#
# Builds the Rust reference, emits+compiles the Axolotl port through axolc,
# diffs the outputs byte-for-byte, then times best-of-5 each and asserts
# axol_time / rust_time <= 1.05.

set -euo pipefail

cd "$(dirname "$0")/.."

REPO_ROOT="$(cd ../.. && pwd)"
AXOLC="${AXOLC:-$REPO_ROOT/target/release/axolc}"

if [[ ! -x "$AXOLC" ]]; then
    echo "[programminglanguage] axolc not found at $AXOLC; build with 'cargo build -p axolc --release'." >&2
    exit 2
fi

echo "[programminglanguage] building rust reference..."
( cd rust && cargo build --release --quiet 2>&1 | tail -3 || true )
RUST_BIN="rust/target/release/programminglanguage-rust"
[[ -x "$RUST_BIN" ]] || { echo "[programminglanguage] rust binary missing" >&2; exit 2; }

echo "[programminglanguage] emitting+compiling axolotl port..."
"$AXOLC" emit-rust axol/src/main.axol > /tmp/programminglanguage_axol.rs 2>/tmp/programminglanguage.err || {
    echo "[programminglanguage] emit-rust failed:" >&2
    cat /tmp/programminglanguage.err >&2
    exit 2
}
rustc -O -C opt-level=3 -C lto=thin -C codegen-units=1 -C panic=abort -C strip=symbols /tmp/programminglanguage_axol.rs -o /tmp/programminglanguage-axol 2>/tmp/programminglanguage.err || {
    echo "[programminglanguage] rustc failed:" >&2
    cat /tmp/programminglanguage.err >&2
    exit 2
}
[[ -x "/tmp/programminglanguage-axol" ]] || { echo "[programminglanguage] axol binary missing" >&2; exit 2; }

echo "[programminglanguage] diffing outputs..."
WORKLOAD="${WORKLOAD:-default}"
RUST_OUT="$("$RUST_BIN" "$WORKLOAD" 2>&1)"
AXOL_OUT="$(/tmp/programminglanguage-axol "$WORKLOAD" 2>&1)"
if [[ "$RUST_OUT" != "$AXOL_OUT" ]]; then
    echo "[programminglanguage] OUTPUT MISMATCH" >&2
    diff <(echo "$RUST_OUT") <(echo "$AXOL_OUT") | head -40 >&2
    exit 1
fi
echo "[programminglanguage] outputs match."

echo "[programminglanguage] timing best-of-5..."
best_rust=999999999
best_axol=999999999
for i in 1 2 3 4 5; do
    start=$(date +%s%N)
    "$RUST_BIN" "$WORKLOAD" > /dev/null 2>&1
    end=$(date +%s%N)
    bc=$(( (end - start) / 1000000 ))
    if [[ $bc -lt $best_rust ]]; then best_rust=$bc; fi
    start=$(date +%s%N)
    /tmp/programminglanguage-axol "$WORKLOAD" > /dev/null 2>&1
    end=$(date +%s%N)
    bc=$(( (end - start) / 1000000 ))
    if [[ $bc -lt $best_axol ]]; then best_axol=$bc; fi
done
ratio=$(awk -v a="$best_axol" -v r="$best_rust" 'BEGIN{printf "%.3f", a/r}')
echo "[programminglanguage] rust=${best_rust}ms axol=${best_axol}ms ratio=$ratio"

awk -v r="$ratio" 'BEGIN{ if (r>1.05) exit 1; else exit 0 }' || {
    echo "[programminglanguage] REGRESSION: ratio $ratio > 1.05" >&2
    exit 1
}
echo "[programminglanguage] PASS (ratio $ratio <= 1.05)"
