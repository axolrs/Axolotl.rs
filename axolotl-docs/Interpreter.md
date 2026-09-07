# Interpreter - axol-hot-runner

> **axol-hot-runner** is Axolotl's interpreter + JIT. It reads both `.axol` and `.rs` files, executes them directly, and re-runs them on save. Goal: **never spend hours compiling just to test something**. Goal: **Elixir-grade iteration speed** for a systems language.

The crate name is `axol-hot-runner` (already in the workspace as `crates/axol-hot-runner/`). The CLI invocation is `bucket run --interpret` or just `bucket run` when `interpreted_dev_mode: true` is set in `Bucket.jsonc`.

---

## 1. Why this exists

The user said it directly:

> *"nobody should ever spend hours compiling code to just test out we need interpreter made for rust as well fitting our own axol programming language"*

> *"i think we could add some of that elixirs expressiveness so its super fast to iterate something in this language"*

> *"DX is my goal for this interpreter/JIT thingy"*

The current `bucket build` flow is: `.axol` → `axolc` → idiomatic Rust → Cargo → rustc → LLVM → native binary. For a 200-line game, that loop can be **30 seconds to 5 minutes** depending on dependency size. For a 5,000-line game with `wgpu`, `winit`, `glam`, `tokio`, `serde`, `bevy_ecs`, the loop can be **2-10 minutes**. That's not a development loop - that's a coffee break.

`axol-hot-runner` is the answer. The development loop becomes:

```text
edit .axol file
   ↓
save
   ↓
axol-hot-runner picks up the change
   ↓
executes the new code in-process (interpreter)
   ↓
or hot-swaps the JIT-compiled native code (JIT)
   ↓
result visible in ~50-200ms
```

```text
edit .rs file
   ↓
save
   ↓
axol-hot-runner picks up the change
   ↓
executes the new code in-process (interpreter)
   ↓
or hot-swaps the JIT-compiled native code (JIT)
   ↓
result visible in ~50-200ms
```

For a game, the window stays open, the game state stays alive, the function you changed re-runs with the new code. You don't relaunch. You don't recompile. You don't wait.

For a CLI tool, the same loop - edit, save, run, see output.

For a web server, edit, save, the next request is handled by the new code. No restart.

That is the development experience the user is asking for.

---

## 2. Two execution modes

`axol-hot-runner` ships two modes. The user picks per project (or per command) which one to use.

### 2a. Interpreter mode (`--interpret`)

Pure tree-walking interpreter. Reads `.axol` source directly, evaluates it. No compilation step. Slow (10-100x slower than native for tight loops), but **zero startup time, zero codegen**.

```bash
bucket run --interpret                   # explicit
bucket run                               # auto if interpreted_dev_mode: true
```

### 2b. JIT mode (`--jit`)

Cranelift-based JIT. Reads `.axol` source, compiles to Cranelift IR on the fly, executes native code. Fast (within 2-5x of AOT for most code), but **slight startup cost** (typically 200ms-1s for the first function).

```bash
bucket run --jit
```

### 2c. When to use which

| Workload | Best mode |
|---|---|
| Game dev, REPL-style exploration, prototyping | `--interpret` |
| Server, CLI tool, anything CPU-bound | `--jit` |
| Production deployment | `bucket build` (the regular AOT pipeline) |

Both modes support the same debugging primitives: `print` works the same, errors are mapped back to Axolotl source spans, breakpoints hit on the user's `.axol` line.

---

## 3. Mixed `.axol` + `.rs` execution

The interpreter reads **both** `.axol` and `.rs` files. A project can be:

- **All Axolotl** - `main.axol` calls into `player.axol` calls into `inventory.axol`.
- **All Rust** - `main.rs` calls into `player.rs` calls into `inventory.rs`.
- **Mixed** - `main.axol` calls into `inventory.rs` (which was ported from Axolotl to Rust for performance reasons), which calls back into `player.axol`.

The interpreter handles all three cases through the same mechanism: **a unified symbol graph** (same idea as Gills/axol-analyzer) plus a shared runtime environment.

### How mixed execution works

1. The interpreter starts in `main.axol` (or `main.rs`).
2. It loads every `.axol` and `.rs` file in the project, plus the dependencies resolved from `Bucket.jsonc`.
3. Each module is parsed and bound to a runtime representation:
   - `.axol` modules → Axolotl HIR (compiled in-memory by `axolc`).
   - `.rs` modules → Rust MIR (compiled in-memory by `rustc` in JIT mode, or evaluated by the interpreter in pure-interp mode).
4. The interpreter dispatches calls between modules based on the type of the receiver / the signature of the called function.
5. Memory is shared through a common heap. The Axolotl side and the Rust side see the same objects.

The boundary is the **Axolotl ↔ Rust ABI**, which is just a normal Rust ABI because Axolotl compiles to Rust. There's no FFI, no marshalling, no serialization. A `Vec3` defined in Axolotl and a `Vec3` defined in Rust are the same `Vec3` in memory.

---

## 4. Hot reload

Hot reload is the killer feature. The user edits a function, saves the file, and the next call to that function runs the new code. No restart. No rebuild. State is preserved.

### How it works

1. `axol-hot-runner` watches the project directory (using `notify`).
2. When a file changes, it parses the new version and produces a new HIR / MIR.
3. It compares the new version against the old version. Three cases:
   - **Function body changed only** - replace the function. Existing calls in progress complete with the old code; new calls use the new code.
   - **Function signature changed** - replace the function. Existing callers compiled against the old signature may need recompilation, but the interpreter handles this transparently.
   - **Struct / enum changed** - replace the type. Existing instances are kept if their layout is compatible; if not, the interpreter logs a warning and rebuilds affected state.
4. Globals, modules, and other top-level state are preserved across the hot reload.

### The dev loop

```text
$ bucket run
[hot-runner] watching src/ ...
[hot-runner] axol: 47 files, 3,210 lines
[hot-runner] rust: 12 files, 1,840 lines
[hot-runner] interpreter ready in 380ms

Game running. Player at (0, 0). Score 0.

# User edits player.axol, changes update() logic, saves

[hot-runner] change detected: src/player.axol
[hot-runner] reloading 3 functions
[hot-runner] reload complete in 47ms
[hot-runner] no state migration needed

Game continues. Player at (0, 0). Score 0.
# New update() logic is now in effect.
```

That's 47ms from save to new code in effect. Versus 30-300 seconds for a full AOT rebuild.

---

## 5. Bucket integration

`axol-hot-runner` is invoked through `bucket`, never directly. The CLI surface:

```bash
bucket run                              # uses interpreted_dev_mode from Bucket.jsonc, or falls back to AOT
bucket run --interpret                  # explicit interpreter
bucket run --jit                        # explicit JIT
bucket run --watch                      # hot reload on file change
bucket run --entry src/main.axol        # custom entry point
bucket run --release                    # AOT build (no hot reload)
bucket run --no-watch                   # run once, no watcher
bucket run --profile interpret          # profile interpreter overhead
bucket run --profile jit                # profile JIT compilation
```

`bucket run` is the dev command. `bucket build` is the AOT build. The two share 90% of the code; `bucket run` is just `bucket build` with `interpreted_dev_mode: true` in the manifest, plus a watcher.

---

## 6. The `interpreted_dev_mode` field

The user said:

> *"in Bucket.json you should be able to add a field interpreted_dev_mode: true"*

So the manifest supports an explicit opt-in:

```jsonc
{
    "name": "my-game",
    "version": "0.1.0",
    "language": { "edition": "2026" },

    "interpreted_dev_mode": true,

    "scripts": {
        "dev":       "bucket run --watch",
        "play":      "bucket run",
        "test":      "bucket test",
        "bench":     "bucket bench",
        "release":   "bucket build --release",
        "fmt":       "bucket fmt",
        "lint":      "bucket lint",
        "fix":       "bucket fix",
        "doc":       "bucket doc",
        "check":     "bucket check",
        "clean":     "bucket clean",
        "doctor":    "bucket doctor"
    },

    "dependencies": {
        "wgpu": "^27",
        "winit": "^30",
        "glam": "^0.30"
    }
}
```

When `interpreted_dev_mode: true`, `bucket run` defaults to interpreter mode. When `false` or absent, `bucket run` defaults to AOT build + run.

The user can override per-call:

```bash
bucket run --interpret       # forces interpreter even if manifest says AOT
bucket run --jit             # forces JIT
bucket run --release         # forces AOT release build
```

The CLI flags always win.

### The `scripts` field

The user said:

> *"in Bucket.json ni scripts (just like package.json in pnpm bun whatever u got you could make custom script just adding a key with value in scripts object same here)"*

`scripts` is a `string -> string` map. The user defines their own dev / build / test / release / etc. commands. Bucket can also run them by name:

```bash
bucket run dev                # equivalent to: bucket run --watch
bucket run play               # equivalent to: bucket run
bucket run release            # equivalent to: bucket build --release
bucket run any-custom-script  # runs whatever the user defined
```

`Bucket.jsonc` is the same JSONC format as before - JSON with comments. Scripts are first-class values; nothing hidden in TOML.

---

## 7. How it's implemented (the rough plan)

`axol-hot-runner` is a Rust crate that depends on `axolc-core` (the Axolotl compiler) and on `rustc` / `cranelift` (the JIT backends). The architecture:

```text
                  axol-hot-runner
                          │
        ┌─────────────────┼─────────────────┐
        │                 │                 │
        ▼                 ▼                 ▼
   axolc-core         rustc/Cranelift    File watcher
   (Axolotl HIR)      (JIT backend)     (notify)
        │                 │                 │
        └────────┬────────┴────────┬────────┘
                 │                 │
                 ▼                 ▼
            Interpreter         JIT
            (tree walker)       (native codegen)
                 │                 │
                 └────────┬────────┘
                          │
                          ▼
                 Shared runtime
                 (memory, GC for interp, FFI, panic handling)
```

### Stack

| Component | Crate |
|---|---|
| File watcher | `notify` |
| Axolotl parser / HIR | `axolc-core` (reused) |
| Rust parser / MIR | `syn` + `rustc` driver API |
| Tree-walking interpreter | custom, in `axol-hot-runner` |
| JIT | `cranelift-module` + `cranelift-jit` |
| Shared runtime memory | custom arena allocator |
| Hot reload diffing | `axolc-core`'s HIR delta + `cargo_metadata` for Rust |
| FFI dispatch | `libloading` for dynamic .so loading |
| Error mapping | `axolc-core`'s span mapping, reused |

### Performance target

| Mode | Target throughput vs AOT |
|---|---|
| Interpreter | 10-30% of AOT (CPU-bound), near-AOT (I/O-bound) |
| JIT (after warmup) | 70-95% of AOT |

JIT warmup is per-function: the first time a function is called, it's interpreted; once it's "hot" (called >100 times by default), it gets JIT-compiled. The user can tune this with `interpreted_dev_mode.jit_threshold: 100` in the manifest.

### Memory

| Mode | Memory overhead vs AOT |
|---|---|
| Interpreter | +50-100 MB (the interpreter + the HIR) |
| JIT | +20-50 MB (the Cranelift module + cached code) |

For a typical game project, total memory in interpreted mode is ~150-300 MB. AOT mode is ~50-100 MB. Acceptable for a dev workflow.

---

## 8. Debugging

`axol-hot-runner` integrates with the same debug tools as the AOT build:

- **Print / `print(...)`** - works identically in interpreter and JIT.
- **Panic messages** - mapped back to Axolotl source spans.
- **Stack traces** - show Axolotl function names, not generated-Rust names.
- **Breakpoints** - supported via GDB / LLDB attach. Source-mapped stack frames show the user's `.axol` line.
- **`assert` / `assert_eq!`** - work in interpreter and JIT.
- **Profiler integration** - `cargo flamegraph` works on the JIT output; `perf` works on the JIT output. The interpreter can also emit flamegraph data for the interpreted frames.

---

## 9. The "what you didn't use doesn't exist" rule still applies

`axol-hot-runner` only does work when it's invoked. If the user runs `bucket build`, the interpreter isn't loaded, the JIT isn't loaded, no interpreter-related code is in the binary. The "if you didn't use it, it doesn't exist" principle holds:

- A pure `.axol` project with no foreign blocks → interpreter + JIT, no C/C++/Python machinery.
- A mixed project with `cblock` → interpreter + JIT + C compilation.
- A pure AOT build (`bucket build --release`) → no interpreter, no JIT, no hot reload. Just a normal Rust binary.

---

## 10. Limitations (honest list)

`axol-hot-runner` is a tool. It's not magic. The following are **not** supported in interpreter/JIT mode (in v1):

- Some `unsafe` operations (raw pointer arithmetic is interpreted, but limited).
- SIMD intrinsics (run in interpreter at scalar speed; the JIT falls back to scalar for intrinsics it doesn't recognize).
- GPU compute (must be AOT-compiled; the interpreter doesn't have a GPU).
- Long-running FFI callbacks (work, but with performance penalty).
- Debug-info parity with AOT (improving over time).

These limitations are surfaced in the help text and in the build log when the user hits them. The fallback is always: `bucket build --release` for a full AOT build.

---

## 11. The "Elixir expressiveness" piece

The user said:

> *"i think we could add some of that elixirs expressiveness so its super fast to iterate something"*

What does that mean concretely? In Elixir / Erlang, the killer DX features are:

1. **No compile step.** Change code, run, see result. → `axol-hot-runner` does this.
2. **Hot code reload.** Process state survives across code changes. → `axol-hot-runner` does this for most state.
3. **Lightweight processes.** Millions of isolated processes, message passing. → Axolotl has `task` blocks and `spawn`, but doesn't have BEAM-style isolated heaps. v2 of `axol-hot-runner` could add "Erlang-mode" where each `task` runs in its own heap.
4. **`iex` REPL.** A live REPL connected to the running system. → `bucket repl` is a v2 feature. v1 has hot reload but no live REPL yet.
5. **Pattern matching in function heads.** → Axolotl has variant-handler methods, which give a similar DX.
6. **`|>` pipe operator.** → Axolotl has `|>`.
7. **Protocol consolidation / behaviour declarations.** → Axolotl has interfaces.

The pieces are there. `axol-hot-runner` provides the runtime layer. The language features (pipe, patterns, tasks, interfaces) provide the syntactic expressiveness. Together: Elixir-grade DX on top of Rust-grade performance.

---

## 12. Acceptance criteria for v1

`axol-hot-runner` v1 is done when:

- `bucket run` invokes the interpreter or JIT when `interpreted_dev_mode: true` in `Bucket.jsonc`.
- The interpreter reads `.axol` and `.rs` files, mixed in any combination.
- Hot reload works for at least: function body changes, struct field additions, enum variant additions.
- `print`, `assert`, basic types, basic control flow, basic collections, async/await, tasks, channels all work in both modes.
- Error messages map back to Axolotl source spans.
- The interpreter doesn't break for any of the 10 build-your-own-x benchmark programs in `benchmarks/`.
- JIT warmup happens automatically based on call count.
- The `scripts` field in `Bucket.jsonc` is respected (`bucket run dev` etc.).
- Documentation, tests, and CI workflows are in place.

That's the bar.

---

## 13. Files to be created in `crates/axol-hot-runner/`

Per the user's file-header convention, every source file gets a top-level comment with the file's purpose and the owner (`PascalElixir` / `axolrs` GitHub org), and every function gets a one-line description above its signature.

```text
crates/axol-hot-runner/
├── Cargo.toml
├── README.md
├── src/
│   ├── main.rs                       # CLI entrypoint for `bucket run --interpret/--jit`
│   ├── lib.rs                        # library root
│   ├── interp/
│   │   ├── mod.rs                    # interpreter module
│   │   ├── value.rs                  # Value enum: Int, Float, String, Array, Map, Function, ...
│   │   ├── env.rs                    # Environment (scope chain, globals, modules)
│   │   ├── eval.rs                   # tree-walking evaluator
│   │   ├── builtin.rs                # built-in functions (print, type, etc.)
│   │   └── call.rs                   # function call dispatch (Axolotl ↔ Rust boundary)
│   ├── jit/
│   │   ├── mod.rs                    # JIT module
│   │   ├── codegen.rs                # Cranelift IR generation
│   │   ├── runtime.rs                # runtime helpers (called from JIT'd code)
│   │   └── cache.rs                  # compiled-function cache
│   ├── runtime/
│   │   ├── mod.rs                    # shared runtime
│   │   ├── memory.rs                 # arena allocator, GC for interp, native heap for JIT
│   │   ├── panic.rs                  # panic handling, span mapping
│   │   └── task.rs                   # task scheduler (cooperative, used by both modes)
│   ├── hot/
│   │   ├── mod.rs                    # hot reload coordinator
│   │   ├── watcher.rs                # notify-based file watcher
│   │   ├── diff.rs                   # HIR / MIR diffing
│   │   └── state.rs                  # state preservation across reloads
│   ├── project/
│   │   ├── mod.rs                    # project loader
│   │   ├── manifest.rs               # Bucket.jsonc parsing
│   │   ├── modules.rs                # module discovery (.axol + .rs)
│   │   └── resolve.rs                # dependency resolution
│   ├── bridge/
│   │   ├── mod.rs                    # Axolotl ↔ Rust ABI bridge
│   │   ├── axol_from_rust.rs         # load .axol and bind for use from .rs
│   │   └── rust_from_axol.rs         # load .rs and bind for use from .axol
│   └── util/
│       ├── mod.rs
│       ├── span.rs                   # span mapping utilities
│       └── log.rs                    # structured logging
└── tests/
    ├── interp_basic.rs               # basic interpreter tests
    ├── interp_mixed.rs               # mixed .axol + .rs tests
    ├── jit_warmup.rs                 # JIT warmup tests
    ├── hot_reload.rs                 # hot reload tests
    ├── scripts.rs                    # Bucket.jsonc scripts tests
    └── bench_programs.rs             # 10 build-your-own-x programs must run
```

All files follow the file-header convention. All functions have a one-line summary above the signature.

---

## 14. The complete `axol-hot-runner` rule

> **The user should never have to wait more than 200ms between "save my code" and "see my new code run."** If they do, the tooling has failed. The 30-second compile loop is over. The interpreter and JIT exist to make that loop disappear.

That is the bar. v1 ships when the loop is sub-200ms for a 5,000-line project with 50 dependencies.

Welcome to fast iteration.
