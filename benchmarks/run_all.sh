#!/usr/bin/env bash
# Owner: PascalElixir / axolrs (GitHub org)
# File: benchmarks/run_all.sh - run all 20 build-your-own-x benchmarks.
#
# Builds axolc if needed, then runs each benchmark's bench/run.sh and
# reports PASS/FAIL with the ratio.

set -uo pipefail

cd "$(dirname "$0")"
REPO_ROOT="$(cd .. && pwd)"
AXOLC="${AXOLC:-$REPO_ROOT/target/release/axolc}"

if [[ ! -x "$AXOLC" ]]; then
    echo "[run_all] axolc not found at $AXOLC; building..."
    ( cd "$REPO_ROOT" && cargo build -p axolc --release ) || { echo "[run_all] cargo build failed" >&2; exit 2; }
fi

PASS=0
FAIL=0
SKIPPED=0
for d in */; do
    name="${d%/}"
    [[ "$name" == "README.md" ]] && continue
    [[ -x "$name/bench/run.sh" ]] || continue
    echo "=== $name ==="
    if bash "$name/bench/run.sh" 2>&1 | tail -3; then
        PASS=$((PASS + 1))
    else
        FAIL=$((FAIL + 1))
    fi
done

echo
echo "============================="
echo "PASS: $PASS"
echo "FAIL: $FAIL"
echo "============================="
