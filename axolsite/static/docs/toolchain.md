# Toolchain

The Axolotl toolchain is a single coherent ecosystem. The names are
biological - axolotl, gills, neoteny, shed, regrow, ambystoma, salamander,
larva, molt, eggbox, pond, mud - but they map to standard concepts you
already know from Cargo and rust-analyzer.

## The Binaries

| Binary | What it does |
|--------|--------------|
| `bucket` | The project orchestrator. Wraps Cargo. |
| `axolc` | The Axolotl → Rust compiler. |
| `axol-analyzer` | Gills. The LSP. Fork of rust-analyzer concepts. |
| `axol-hot-runner` | The interpreter + JIT. Dev iteration only. |

## The Subcommands

`bucket` exposes the standard Unix subcommand set:

| Command | What it does |
|---------|--------------|
| `bucket new <name>` | Scaffold a new project (Larva). |
| `bucket init` | Initialize a project in the current directory. |
| `bucket add <crate>` | Add a Cargo crate to `Bucket.jsonc`. |
| `bucket build [--release]` | Build the project. `--release` for native binary. |
| `bucket run [--watch] [--interpret]` | Run the project. `--watch` for hot reload. |
| `bucket test` | Run all tests (Salamander). |
| `bucket bench` | Run all benchmarks (Salamander). |
| `bucket fmt` | Format the codebase (Shed). |
| `bucket lint` | Lint the codebase (Neoten). |
| `bucket fix` | Auto-fix lint warnings (Regrow). |
| `bucket doc` | Generate API docs (Ambystoma). |
| `bucket clean` | Remove `pond/` and `target/`. |
| `bucket doctor` | Diagnose environment issues. |
| `bucket gills --stdio` | Run Gills (the LSP) over stdio. |
| `bucket upgrade` | Migrate the project to a newer Axolotl version (Molt). |
| `bucket publish` | Publish a crate to crates.io (Eggbox). |

Every command has `--help` and `--json` (machine-readable output).

## Bucket.jsonc - The Manifest

```jsonc
{
    "name": "my-game",
    "version": "0.1.0",

    "language": {
        "edition": "2026"        // Axolotl language edition
    },

    "interpreted_dev_mode": true,  // opt into interpreter in `bucket run`

    "scripts": {
        "dev":     "bucket run --watch",
        "play":    "bucket run",
        "test":    "bucket test",
        "bench":   "bucket bench",
        "release": "bucket build --release",
        "fmt":     "bucket fmt",
        "lint":    "bucket lint",
        "fix":     "bucket fix",
        "doc":     "bucket doc",
        "clean":   "bucket clean",
        "doctor":  "bucket doctor"
    },

    "dependencies": {
        "wgpu":   "^27",      // crates.io crates, not Axolotl packages
        "winit":  "^30",
        "glam":   "^0.30",
        "serde":  "^1",
        "tokio":  "^1"
    },

    "build": {
        "target": "x86_64-unknown-linux-gnu",
        "release": {
            "lto": "thin",
            "codegen-units": 1
        }
    },

    "lsp": {
        "trace": "messages",
        "inlay_hints": { "enabled": true },
        "completion": { "enabled": true },
        "diagnostics": { "enabled": true }
    }
}
```

## The Tools

### axolc - The Compiler

`axolc` is the Axolotl → Rust compiler. It reads `.axol` source and emits
idiomatic Rust source that Cargo then compiles to a native binary.

```bash
axolc compile src/main.axol --out out.rs
rustc -O out.rs -o my-app
```

The generated Rust is readable - it does not contain `axolotl_runtime::`
calls or `Box<dyn Any>` everywhere. A Rust developer reading the emitted
code would not know it was machine-generated.

### axol-hot-runner - The Interpreter + JIT

`axol-hot-runner` is the dev iteration layer. When `interpreted_dev_mode: true`
is set in `Bucket.jsonc`, `bucket run` uses the hot runner instead of the
compiler.

The hot runner:

1. Parses the `.axol` source.
2. Builds an in-memory AST.
3. Tree-walks the AST to execute it directly.
4. Hot-reloads on file save.

For hot loops, the JIT (Cranelift) kicks in and compiles the loop body to
native code on the fly. The JIT is opt-in for the interpreter - it is
never linked into a release binary.

### Gills (axol-analyzer) - The LSP

Gills is the language server. It speaks standard LSP, so any editor that
supports LSP (Zed, Neovim, VS Code, Helix, Emacs) can use it. Gills
understands both `.axol` and `.rs` files in the same project, providing
cross-language navigation, completion, hover, diagnostics, and rename.

Gills is a fork of the rust-analyzer architecture, extended with the
Axolotl frontend.

### Shed (bucket fmt) - The Formatter

`bucket fmt` runs Shed. Shed formats `.axol` source files. The style is
opinionated and not configurable - the goal is that every Axolotl codebase
in the world looks the same.

### Neoten (bucket lint) - The Linter

`bucket lint` runs Neoten. Neoten lints `.axol` source files. It catches
common mistakes (unused variables, unreachable code, suspicious ownership
patterns, etc.) and reports them as Axolotl-level diagnostics (the user
sees `.axol` line numbers, not generated-Rust line numbers).

### Regrow (bucket fix) - The Auto-Fixer

`bucket fix` runs Regrow. Regrow applies Neoten's suggestions automatically.
It can also apply semantic refactors: extract function, inline variable,
convert to comprehension, etc.

### Ambystoma (bucket doc) - The Documentation Generator

`bucket doc` runs Ambystoma. Ambystoma generates API documentation from
the in-source doc comments. The output is a static HTML site that can be
hosted anywhere.

### Salamander (bucket test, bucket bench) - The Test Runner

`bucket test` runs Salamander. Salamander runs every `#[test]` function in
the project plus the upstream Cargo tests. `bucket bench` runs the
benchmark suite.

### Larva (bucket new) - The Scaffolder

`bucket new <name>` runs Larva. Larva scaffolds a new project with the
standard layout: `Bucket.jsonc`, `src/main.axol`, `.gitignore`, etc.

### Molt (bucket upgrade) - The Migrator

`bucket upgrade` runs Molt. Molt migrates a project from an older Axolotl
edition to a newer one. It rewrites source files automatically when the
language changes.

### Eggbox (bucket publish) - The Publisher

`bucket publish` runs Eggbox. Eggbox publishes the project's primary crate
to crates.io (since Axolotl crates ARE Rust crates).

## The Pond (Cache)

`pond/` is the build / output / cache directory. It is the Axolotl
equivalent of `target/` in Cargo. Per-project `pond/` lives in the project
root; per-machine `pond/` lives at `~/.bucket/pond/`.

The Pond caches compiled artifacts across projects, so compiling a
dependency once means it is available to every project on the machine.

## The Mud (Native Binary)

The AOT-compiled native binary that an Axolotl project compiles to is
called "Mud". It is just a name for the final artifact - there is no
runtime, no VM, no interpreter linked into a Mud binary. The binary is
LLVM-optimized native code.

## Editor Integrations

The toolchain ships three first-class editor integrations:

| Editor | Plugin |
|--------|--------|
| Zed | `editors/zed-axolotl/` |
| Neovim | `editors/nvim-axolotl/` |
| VS Code | `editors/axolcode/` |

All three talk to Gills over LSP. All three use the same tree-sitter grammar
for syntax highlighting (`tooling/tree-sitter-axol/`).

## What's Next

- Read the [Interpreter](/docs/interpreter) guide for the hot runner design.
- Read the [Gills (LSP)](/docs/gills) guide for the LSP design.
- Read the [Editor Setup](/docs/editor-setup) guide for your specific editor.
