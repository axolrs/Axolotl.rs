#!/usr/bin/env python3
# Owner: PascalElixir / axolrs (GitHub org)
# File: scripts/const_workload_migration.py - replace __WORKLOAD__ markers with a real const per benchmark port.

import re
import pathlib

BENCH = pathlib.Path("/home/z/my-project/axolotl-proj/benchmarks")

OLD_NOTE = """-- The __WORKLOAD__ marker is replaced with the iteration count by the bench
-- harness before compilation (the compiler requires literal loop bounds)."""

NEW_NOTE = """-- The bench harness overrides the WORKLOAD const (in memory, before
-- compiling) with the run's iteration count; `bucket build` uses the default."""

for port_dir in sorted(BENCH.iterdir()):
    if not port_dir.is_dir():
        continue
    axol_src = port_dir / "axol/src/main.axol"
    harness_src = port_dir / "bench/src/main.rs"
    if not axol_src.exists() or not harness_src.exists():
        continue
    port = port_dir.name

    harness = harness_src.read_text()
    m = re.search(r"const DEFAULT_WORKLOAD: u64 = (\d+);", harness)
    if not m:
        print(f"{port}: no DEFAULT_WORKLOAD found, skipping")
        continue
    default = m.group(1)

    src = axol_src.read_text()
    if "__WORKLOAD__" not in src:
        print(f"{port}: no marker in source, skipping")
        continue
    uses = src.count("__WORKLOAD__") - 2 if OLD_NOTE in src else src.count("__WORKLOAD__")
    src = src.replace("__WORKLOAD__", "WORKLOAD")
    if OLD_NOTE in src:
        src = src.replace(OLD_NOTE, NEW_NOTE)
    else:
        src = src.replace(
            "-- The WORKLOAD marker is replaced",
            "-- The WORKLOAD marker is replaced",
        )
    lines = src.split("\n")
    insert_at = 0
    for i, line in enumerate(lines):
        if line.startswith("--"):
            insert_at = i + 1
        elif line.strip() == "" and insert_at > 0:
            insert_at = i + 1
            break
    lines.insert(insert_at, f"const WORKLOAD: Int = {default}")
    src = "\n".join(lines)
    axol_src.write_text(src)

    old_sub = 'let src = axol_src.replace("__WORKLOAD__", &workload.to_string());'
    new_sub = (
        f'let src = axol_src\n'
        f'        .replace("const WORKLOAD: Int = {default}", &format!("const WORKLOAD: Int = {{}}", workload));'
    )
    if old_sub in harness:
        harness = harness.replace(old_sub, new_sub)
        harness_src.write_text(harness)
        print(f"{port}: migrated (default={default}, {max(uses, 0)} marker uses)")
    else:
        print(f"{port}: harness substitution line not found - MANUAL FIX NEEDED")
