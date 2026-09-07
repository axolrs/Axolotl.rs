# FAQ

Frequently asked questions about Axolotl.

## General

### What is Axolotl?

Axolotl is a high-level, all-purpose, memory-safe, zero-GC programming
language with Lua-flavored syntax and Rust-grade semantics. It compiles
to idiomatic Rust and ships as a native binary through Cargo + rustc +
LLVM. There is no Axolotl runtime, no VM, no GC, no interpreter sitting
underneath a release binary.

### Why "Axolotl"?

The axolotl is a neotenic salamander - it keeps its gills into adulthood
(never has to "grow up" into a different form). The naming pun: Axolotl
keeps its friendly scripting-language surface into adulthood, never
forcing you to drop into a different (lower-level) language for
performance. The toolchain naming continues the biological theme: Gills
(the LSP), Neoten (the linter), Shed (the formatter), Regrow (the
auto-fixer), Ambystoma (the docs generator), Salamander (the test
runner), Larva (the scaffolder), Molt (the migrator), Eggbox (the
publisher), Pond (the build cache), Mud (the native binary).

### Is Axolotl a Rust replacement?

No. Axolotl is a new frontend for the Rust ecosystem. Rust source files
(`*.rs`) are first-class citizens in the same project as `.axol` files.
The Cargo ecosystem is the dependency graph. There is no Axolotl std
library or Axolotl SDK to invent - you use Cargo crates directly.

### Is Axolotl a Lua clone?

No. The syntax is Lua-flavored (`if/then/end`, `function ... end`, `:`
method calls, tables), but the semantics are statically typed and
Rust-grade. No GC, no metatables at runtime, no `nil` as a universal
inhabitant.

### Is Axolotl a transpiler?

Not exactly. `axolc` produces idiomatic Rust source for `bucket emit-rust`
debugging, but the primary compilation path is metadata-aware: the
compiler reads Rust crate `.rmeta` files and generates calls into real
Rust crates. There are no parallel "AxolotlWgpu" wrappers.

## Performance

### How fast is Axolotl?

As fast as the equivalent hand-written Rust, because the Axolotl
compiler generates idiomatic Rust and lets LLVM optimize it. The
build-your-own-x benchmarks in `benchmarks/` demonstrate that the
Axolotl versions are within 5% of the Rust references.

### Is there a runtime?

No. There is no Axolotl runtime in production. The `bucket build --release`
binary does not link the interpreter, the JIT, or any Axolotl-specific
runtime. The CI asserts this.

### How fast is the dev iteration?

Sub-200ms for a 5,000-line project with 50 dependencies. The
`axol-hot-runner` is the interpreter + JIT for dev mode; it
hot-reloads on file save.

## Ecosystem

### Can I use Cargo crates?

Yes. Every crates.io crate is an Axolotl library. Add it with
`bucket add <crate>` and import it with `use "<crate>"`.

### Can I mix `.axol` and `.rs` files?

Yes. The same project can contain both. They share the same Cargo
dependency graph. Gills (the LSP) understands both languages and
provides cross-language navigation.

### Is there an Axolotl package registry?

No, and there never will be. Axolotl uses crates.io directly. There is
no parallel ecosystem.

## Ownership

### Who owns Axolotl?

The owner handles are `PascalElixir` and the `axolrs` GitHub
organization. These are the only handles that appear in code, commits,
generated docs, and public surfaces.

### What is the license?

Dual-licensed **MIT OR Apache-2.0** at the user's preference. The
`LICENSE` file at the repo root is the MIT license. Every `Cargo.toml`
declares `license = "MIT OR APACHE-2.0"`.

### Is there a premium tier?

No. There is no "premium tier," no "enterprise edition," no paywalled
features. The compiler, the interpreter, the JIT, the LSP, the
formatter, the linter, the auto-fixer, the build orchestrator, the docs,
the website - all free, all open source, forever.

## Toolchain

### What is the Rust edition?

Rust edition **2024**. Every `Cargo.toml` declares `edition = "2024"`.

### What is the calendar year?

The year is **2026**. The Rust edition (2024) is a Rust concept, not a
calendar year.

### How do I install the toolchain?

```bash
curl -sSf https://axolotl.rs/install | sh
```

This installs `bucket`, `axolc`, `axol-analyzer`, and `axol-hot-runner`
on your PATH.

### How do I update the toolchain?

```bash
bucket upgrade
```

This runs Molt, which migrates the project to a newer Axolotl version.

## Tests

### How many tests are there?

20,000+ at the project level, growing. The worklog tracks the running
count. The count grows monotonically - never shrinks.

### How are tests counted?

A test function with three `assert_eq!` calls counts as three tests. The
CI reports the total. The worklog tracks the running count.

### What is the test split?

- 10,000+ tests for the language itself (lexer, parser, type system,
  ownership inference, pattern matching, closures, async, traits,
  generics, FFI).
- 10,000+ tests for the toolchain (compiler, interpreter, JIT, LSP,
  formatter, linter, auto-fixer, editor integration).

## Benchmarks

### How many build-your-own-x ports are there?

20. The first 10 are required by the master brief: Git, Database, Redis,
Shell, Web Server, Text Editor, Programming Language, Regex Engine,
Docker, Emulator. The next 10 are additional: JSON, KV Store, Cron,
Grep, Neural Network, Physics, Renderer, Blockchain, BitTorrent,
Compiler.

### What is the performance gate?

`axol_time / rust_time <= 1.05` - within 5% of the hand-written Rust
reference. The CI fails on regression.

### Where do the benchmarks live?

In `benchmarks/<name>/` with `rust/`, `axol/`, and `bench/`
subdirectories.

## Editor Support

### Which editors are supported?

Three first-class integrations: Zed, Neovim, VS Code. All three talk
to Gills over LSP and use the same tree-sitter grammar.

### What about Helix, Emacs, Sublime?

Gills speaks standard LSP. Any editor that supports LSP can use it.
The tree-sitter grammar is publishable to every editor that supports
tree-sitter. Adding a new editor is not a goal in the current release;
the three first-class integrations are the focus.

## What's Next

- Read the [Getting Started](/docs/getting-started) guide to install
  Axolotl.
- Read the [Language Reference](/docs/language-reference) for the full
  syntax.
- Read the [Toolchain](/docs/toolchain) guide for the `bucket` command
  reference.
