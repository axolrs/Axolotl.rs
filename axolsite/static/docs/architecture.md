# Architecture

This document is the system architecture for the Axolotl language and
toolchain. It covers the compiler pipeline, the build orchestrator, the
interpreter, the LSP, the editor integrations, and the website.

## The Constraints

Every architecture decision in Axolotl is driven by these constraints:

1. **No Axolotl runtime in production.** `bucket build --release` produces
   a binary that does not link the interpreter, the JIT, Cranelift, or any
   Axolotl-specific runtime. The binary is just LLVM-optimized native code.
2. **Cargo compatibility from day one.** Every crates.io crate is an
   Axolotl library. There is no parallel ecosystem.
3. **Mixed `.axol` + `.rs` projects.** The same project can contain both
   languages. They share the same Cargo dependency graph.
4. **Sub-200ms dev iteration.** `bucket run --watch` hot-reloads in well
   under a second, even on a 5,000-line project with 50 dependencies.
5. **Smart compiler.** Gills (the LSP) understands both languages and
   provides cross-language navigation, completion, and diagnostics.
6. **Lua-flavored surface, Rust-grade semantics.** The syntax is
   Lua-like (familiar, friendly); the semantics are Rust-grade (memory
   safe, zero-cost abstractions).

## The Five Candidate Architectures

During the design conversation, five architectures were considered:

1. **Transpile to Rust source.** Generate `.rs` files from `.axol`, then
   hand them to Cargo. Simple, but the generated Rust was ugly and hard
   to debug.
2. **Transpile to LLVM IR directly.** Generate LLVM IR from `.axol`,
   bypassing Rust entirely. Fast, but loses the Cargo ecosystem and the
   borrow checker.
3. **Tree-walk interpreter only.** Never compile. Fast dev iteration, but
   no production mode.
4. **Custom VM.** Design a custom bytecode VM and compile `.axol` to it.
   Full control, but reinvents the wheel.
5. **Metadata-aware codegen to Rust.** Read `.rmeta` files from Rust
   crates, generate calls into real Rust crates. The generated Rust is
   idiomatic and readable. Cargo + rustc + LLVM do the heavy lifting.

The chosen architecture is **#5: metadata-aware codegen to Rust.** The
compiler is a frontend that produces idiomatic Rust; Cargo + rustc + LLVM
do the optimization and code generation. The interpreter is a separate
dev tool, never linked into production.

## The Pipeline

```
.axol source
     |
     v
   axolc
     |  (emits idiomatic Rust)
     v
  .rs source
     |
     v
   cargo + rustc + LLVM
     |
     v
  native binary
```

The interpreter runs in parallel:

```
.axol source
     |
     v
axol-hot-runner
     |  (tree-walk -> bytecode -> Cranelift JIT)
     v
  direct execution
```

## The Crates

| Crate | Role |
|-------|------|
| `axolc` | The CLI driver. Parses arguments, drives `axolc-core`, prints diagnostics. |
| `axolc-core` | The compiler library. Lexer, parser, HIR, type system, ownership inference, diagnostics, Rust codegen, span mapping. Reused by `axol-hot-runner` and `axol-analyzer`. |
| `axol-hot-runner` | The interpreter + JIT. Reads `.axol` and `.rs`, executes directly, hot reloads on save. |
| `axol-analyzer` | Gills, the LSP. Fork of rust-analyzer, extended for mixed projects. |
| `bucket` | The build orchestrator. Sits above Cargo. Manages the Pond cache, Bucket.jsonc, the toolchain (Shed, Neoten, Regrow, etc.). |
| `axolc-testkit` | Common test helpers used by all crates and by the build-your-own-x benchmarks. |

## The Build Orchestrator - bucket

`bucket` sits above Cargo. It reads `Bucket.jsonc`, generates the
corresponding `Cargo.toml`, and invokes `cargo` underneath. The user
never touches `Cargo.toml` directly - `bucket` regenerates it whenever
`Bucket.jsonc` changes.

The Pond (`~/.bucket/pond/` and `project/pond/`) is the build cache. It
caches compiled artifacts across projects, so compiling a dependency
once means it is available to every project on the machine.

## The Test Plan

The project has 20,000+ tests, distributed roughly as:

- 10,000+ tests for the language itself (lexer, parser, type system,
  ownership inference, pattern matching, closures, async, traits,
  generics, FFI).
- 10,000+ tests for the toolchain (compiler, interpreter, JIT, LSP,
  formatter, linter, auto-fixer, editor integration).

A test function with three `assert_eq!` calls counts as three tests.
The CI reports the total; the worklog tracks the running count. The
count grows monotonically - never shrinks.

## The Benchmarks

The project ships 20 build-your-own-x ports in `benchmarks/`. Each port
has:

- `rust/` - hand-written Rust reference.
- `axol/` - Axolotl port of the same algorithm.
- `bench/run.sh` - the 5% gate harness.

The harness compiles both versions, runs them on the same workload,
diffs their outputs byte-for-byte, then times best-of-5 each. The CI
fails if any port's `axol_time / rust_time > 1.05`.

## The Website

`axolsite/` is a SvelteKit 5 project using shadcn-svelte, Tailwind CSS 4,
and Cloudflare's adapter. It is the public face of the project:

- **Home** (`/`) - landing page with hero, feature highlights, key stats.
- **Docs** (`/docs/*`) - markdown viewer for the language documentation.
- **Learn** (`/learn/*`) - interactive tutorial with runnable Axolotl
  snippets.
- **Benchmarks** (`/benchmarks`) - the 20 build-your-own-x ports with
  performance numbers.
- **Playground** (`/play`) - in-browser Axolotl editor (wasm-based).
- **About** (`/about`) - license, owners, contributing link.

The rule: **never touch `layout.css`** (the central shadcn-svelte
stylesheet). **Always use Tailwind classes. No `<style>` blocks.** The
CI enforces this.

## The Editor Integrations

Three first-class editor integrations:

| Editor | Plugin | Language |
|--------|--------|----------|
| Zed | `editors/zed-axolotl/` | Rust (WASM cdylib) |
| Neovim | `editors/nvim-axolotl/` | Lua |
| VS Code | `editors/axolcode/` | TypeScript + Webpack |

All three talk to Gills over LSP. All three use the same tree-sitter
grammar (`tooling/tree-sitter-axol/`) for syntax highlighting.

## The License

The project is dual-licensed **MIT OR Apache-2.0** at the user's
preference. Every source file (Rust, Svelte, TypeScript, Markdown) carries
the same license posture. The `LICENSE` file at the root is the MIT
license. Every `Cargo.toml` declares `license = "MIT OR APACHE-2.0"`.

There is no "premium" tier, no "enterprise" edition, no paywalled
features. The compiler, the interpreter, the JIT, the LSP, the
formatter, the linter, the auto-fixer, the build orchestrator, the docs,
the website - all free, all open source, forever.

## The Owners

The only handles that appear in code, commits, generated docs, and
public surfaces are `PascalElixir` and the `axolrs` GitHub organization.
No other account name appears anywhere in the codebase, git history, or
generated artifacts.

## What's Next

- Read the [Language Reference](/docs/language-reference) for the full
  syntax.
- Read the [Toolchain](/docs/toolchain) guide for the `bucket` command
  reference.
- Read the [Comparison](/docs/comparison) to see how Axolotl compares to
  other languages.
