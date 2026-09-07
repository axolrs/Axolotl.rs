#!/usr/bin/env python3
# Owner: PascalElixir / axolrs (GitHub org)
# File: scripts/gen_hotreload_project.py - generate a 5,000-line project with 50 dependencies for the hot-reload benchmark.

import pathlib
import random

ROOT = pathlib.Path("/home/z/my-project/hotreload-bench")

deps = ", ".join(f'"dep{i:02d}": "0.1.{i}"' for i in range(1, 51))
manifest = f'''{{
    "name": "hotreload-bench",
    "version": "0.1.0",
    "language": {{ "edition": "2026" }},
    "interpreted_dev_mode": true,
    "dependencies": {{ {deps} }}
}}
'''

random.seed(20260905)
parts = []
lines = 0

parts.append("-- Owner: PascalElixir / axolrs (GitHub org)")
parts.append("-- File: hotreload-bench/src/main.axol - 5,000-line hot-reload benchmark project.")
parts.append("")
lines += 3

fn_i = 0
while lines < 4930:
    fn_i += 1
    n_params = random.randint(1, 4)
    params = ", ".join(f"p{j}: Int" for j in range(n_params))
    body_ops = random.randint(4, 8)
    body = []
    acc = " + ".join([f"p{j}" for j in range(n_params)] + [str(random.randint(1, 999))])
    body.append(f"    var acc = {acc}")
    for k in range(body_ops):
        op = random.choice(["+", "-", "*"])
        val = random.randint(2, 500)
        body.append(f"    acc = (acc {op} {val}) & 1048575")
    body.append(f"    return acc")
    fn_src = f"fn work{fn_i}({params}) -> Int\n" + "\n".join(body) + "\nend"
    fn_lines = fn_src.count("\n") + 1
    parts.append(fn_src)
    parts.append("")
    lines += fn_lines + 1

calls = []
for i in range(1, min(fn_i, 40) + 1):
    args = ", ".join(str(random.randint(1, 100)) for _ in range(4))
    calls.append(f'print("work{i}=", work{i}({args}))')
main_body = "\n".join(calls)
main_src = f"fn main()\n{main_body}\nend"
parts.append(main_src)
lines += main_src.count("\n") + 1

while lines < 5000:
    parts.append("")
    lines += 1

src = "\n".join(parts)
(ROOT / "src").mkdir(parents=True, exist_ok=True)
(ROOT / "Bucket.jsonc").write_text(manifest)
(ROOT / "src" / "main.axol").write_text(src)
print(f"lines: {src.count(chr(10)) + 1}, fns: {fn_i}, deps: 50")
