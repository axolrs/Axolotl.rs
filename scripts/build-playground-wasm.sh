#!/usr/bin/env bash
# Owner: PascalElixir / axolrs (GitHub org)
# File: scripts/build-playground-wasm.sh - build the axolc wasm module and install it as an axolsite static asset.

set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
CRATE_DIR="$ROOT/tooling/axolc-wasm"
TARGET_DIR="$CRATE_DIR/target/wasm32-unknown-unknown/release"
ASSET_PATH="$ROOT/axolsite/static/axolc.wasm"

if ! command -v cargo >/dev/null 2>&1; then
  # shellcheck disable=SC1091
  [ -f "$HOME/.cargo/env" ] && . "$HOME/.cargo/env"
fi

cd "$CRATE_DIR"
cargo build --release --target wasm32-unknown-unknown

mkdir -p "$ROOT/axolsite/static"
cp "$TARGET_DIR/axolc_wasm.wasm" "$ASSET_PATH"

echo "axolc playground wasm installed at axolsite/static/axolc.wasm ($(stat -c%s "$ASSET_PATH") bytes)"
