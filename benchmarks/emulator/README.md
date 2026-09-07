# emulator - build-your-own-x port

Owner: PascalElixir / axolrs (GitHub org)
License: MIT OR Apache-2.0

Part of the Axolotl build-your-own-x benchmark suite. The Rust reference
and the Axolotl port implement the same algorithm and produce the same
output. The bench harness in `bench/` enforces the 5% performance gate.

## Layout

- `rust/` - hand-written Rust reference
- `axol/` - Axolotl port (compiles via `axolc` to idiomatic Rust)
- `bench/` - the 5% gate harness

## Why emulator

A CHIP-8 VM exercises u8/u16/u32/u64 types, bitwise operators, the unsafe block, and the FFI surface.
