# emulator benchmark

Owner: PascalElixir / axolrs
License: MIT OR Apache-2.0

Runs the Rust reference and the Axolotl port on the same workload,
asserts byte-identical stdout, then enforces the 5% gate:

    axol_time / rust_time <= 1.05

## Run

```bash
cd benchmarks/emulator
bash bench/run.sh
```

Override the workload via `WORKLOAD=...` (defaults to the in-source const).
Override the axolc binary via `AXOLC=/path/to/axolc`.
