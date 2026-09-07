# The Interpreter - axol-hot-runner

`axol-hot-runner` is Axolotl's interpreter + JIT. It is the iteration
layer for development. It reads `.axol` and `.rs` files, executes them
directly, and re-runs them on save. The goal: **never spend hours compiling
just to test something.** The goal: **Elixir-grade iteration speed** for
a systems language.

## Why an Interpreter?

A systems language needs to compile to native code for production. But
during development, waiting 30 seconds for `cargo build` to test a single
change kills the feedback loop. `axol-hot-runner` solves this by running
the Axolotl source directly - no compile step - for sub-200ms iteration.

The interpreter is **dev-only**. It is never linked into a release
binary. `bucket build --release` produces a binary with **no Axolotl
runtime** linked in.

## How It Works

The hot runner has three modes:

1. **Tree-walk interpreter** - parses the AST, walks it, evaluates.
   Slowest, but the simplest. Used for cold starts.
2. **Bytecode interpreter** - compiles the AST to bytecode, then
   interprets the bytecode. Faster than tree-walk.
3. **Cranelift JIT** - for hot loops, compiles the bytecode to native
   code on the fly. Fastest, but takes a few iterations to warm up.

The hot runner starts in tree-walk mode. As it identifies hot loops
(via counters), it promotes them to bytecode, then to JIT. The user
perceives a single "it just runs fast" experience.

## Hot Reload

When `bucket run --watch` is invoked, the hot runner:

1. Starts the program.
2. Watches the project's `.axol` files for changes.
3. On save, re-parses the changed files.
4. Diff the new AST against the previous AST.
5. Identifies the changed top-level items.
6. Hot-swaps them in the running process (preserving state where
   possible).
7. Re-runs the program from the entry point.

The state preservation is best-effort. Global state is preserved across
reloads unless the structure of the global changes. Local state (inside
a function) is reset on reload.

For a 5,000-line project with 50 dependencies, the hot reload cycle is
sub-200ms on commodity hardware.

## Bucket.jsonc Configuration

```jsonc
{
    "interpreted_dev_mode": true,
    "scripts": {
        "dev":  "bucket run --watch",
        "play": "bucket run"
    }
}
```

When `interpreted_dev_mode: true` is set, `bucket run` uses the hot
runner. When `false` (or absent), `bucket run` uses the compiler.

## The JIT

The JIT is built on Cranelift. It compiles bytecode to native code on
the fly. The compilation happens in a background thread; the user
perceives no pause.

The JIT is opt-in for the interpreter - it is never linked into a
release binary. To disable the JIT (e.g., for debugging), set:

```jsonc
{
    "interpreted_dev_mode": true,
    "lsp": {
        "trace": "messages"
    },
    "interpreter": {
        "jit": false
    }
}
```

## The No-Runtime Invariant

`bucket build --release` produces a binary that:

- Does not link the interpreter.
- Does not link the JIT.
- Does not link Cranelift.
- Does not link any Axolotl-specific runtime.

The CI asserts this. Running `nm` on the release binary should show zero
`axol_*` symbols.

## Mixed .axol + .rs Projects

The hot runner can execute mixed `.axol` and `.rs` projects. The `.axol`
files are interpreted; the `.rs` files are compiled to a shared object
and dynamically linked. The two communicate via the standard Rust ABI.

This means you can have a hot-reloading `.axol` codebase that calls into
performance-critical `.rs` code (e.g., a physics engine) without giving
up the dev iteration speed.

## REPL

The hot runner ships with a REPL:

```bash
bucket repl
```

The REPL evaluates expressions one at a time. It preserves state between
expressions. It supports multi-line input (the parser knows when an
expression is incomplete).

```
> let x = 5
> let y = 10
> x + y
15
> function square(n) return n * n end
> square(x)
25
```

## Performance

| Mode | Time to start | Time per iteration |
|------|----------------|--------------------|
| Tree-walk | <50ms | depends on code |
| Bytecode | <100ms (after warmup) | 5-10x faster than tree-walk |
| Cranelift JIT | <500ms (after warmup) | within 5% of AOT |

For a 5,000-line project with 50 dependencies:

| Operation | Time |
|-----------|------|
| Cold start (tree-walk) | 80ms |
| First hot reload | 50ms |
| Steady-state hot reload | 25ms |
| First JIT kick-in | 300ms |
| Steady-state execution | within 5% of release |

## What's Next

- Read the [Toolchain](/docs/toolchain) guide for the full `bucket`
  command reference.
- Read the [Gills (LSP)](/docs/gills) guide for the LSP design.
- Read the [Architecture](/docs/architecture) guide for the system
  architecture.
