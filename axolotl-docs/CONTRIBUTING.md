# Contributing to Axolotl

> **Axolotl is a high-level, all-purpose, memory-safe, zero-GC programming language with native compilation through idiomatic Rust. We welcome contributions that advance the language, the compiler, the interpreter, the LSP, the toolchain, the documentation, the benchmarks, the website, or anything else in the project.**

## Table of Contents

1. Code of Conduct
2. Project structure
3. How to get the codebase
4. The toolchain
5. The workflow
6. The file header convention
7. The function summary convention
8. The no-comments-in-bodies rule
9. The Markdown no-comments rule
10. The naming conventions
11. How to add a dependency
12. How to write a test
13. How to add a benchmark
14. How to add a documentation page
15. Submitting a change
16. Release process

## 1. Code of Conduct

This project adheres to a [Code of Conduct](CODE_OF_CONDUCT.md). By participating, you are expected to uphold it. Report unacceptable behavior to the maintainers (PascalElixir, the `axolrs` GitHub organization).

## 2. Project structure

```
Axolotl.rs/
├── Cargo.toml                     # workspace
├── Cargo.lock
├── LICENSE                        # MIT OR Apache-2.0
├── CONTRIBUTING.md                # this file
├── CODE_OF_CONDUCT.md
├── NOTICE
│
├── crates/
│   ├── axolc/                     # the Axolotl → Rust compiler (CLI)
│   ├── axolc-core/                # the compiler library
│   ├── axol-hot-runner/           # the interpreter + JIT
│   ├── axol-analyzer/             # the LSP (Gills)
│   ├── bucket/                    # the build orchestrator
│   └── axolc-testkit/             # test helpers
│
├── editors/
│   ├── zed-axolotl/               # Zed extension (WASM)
│   └── nvim-axolotl/              # Neovim plugin
│
├── tooling/
│   └── tree-sitter-axol/          # tree-sitter grammar
│
├── axolsite/                      # SvelteKit 5 website
│
├── benchmarks/                    # 10 build-your-own-x ports (Axolotl + Rust)
├── examples/                      # example projects
├── tests/                         # cross-crate integration tests
├── scripts/                       # bucket scripts
├── docs/                          # markdown documentation
│
└── WORKLOG.md                     # cross-session worklog
```

The `axolotl-docs.zip` archive contains the design documents: the language spec (`PROGRAMMINGLANGUAGEBIBLE.md`), the architecture, the concepts, the high-level features, the LSP story, the interpreter story, the comparison with other languages, and the master engineering brief (`PROMPT.md`). Read these before contributing.

## 3. How to get the codebase

```bash
curl -X POST -L -o repo.tar.gz "<url>"
tar -xzf repo.tar.gz
cd Axolotl.rs
```

The user maintains the URL. The tarball is regenerated per release / per major worklog checkpoint.

## 4. The toolchain

Every contributor installs the latest stable Rust toolchain:

```bash
rustup update stable
rustc --version
cargo --version
```

The Cargo workspace is on **edition 2024**. The `Cargo.toml` files declare `edition.workspace = true` to inherit.

For the website, install the latest Node.js LTS and pnpm:

```bash
node --version
pnpm --version
```

Then install the website dependencies:

```bash
cd axolsite
pnpm install
cd ..
```

## 5. The workflow

Every change follows **research → plan → verify → implement → check (long-term)**.

- **Research:** read the relevant section of `PROGRAMMINGLANGUAGEBIBLE.md` (the spec), the relevant upstream docs (Rust, Cargo, Cranelift, Svelte, shadcn-svelte), and any existing code in the affected crate.
- **Plan:** before writing code, write the plan to `WORKLOG.md`. Which files change, which APIs touch which, what the test surface looks like, what could go wrong.
- **Verify:** before merging, run the smallest unit that exercises the change. `cargo check -p <crate>` for a smoke test, `cargo test -p <crate>` for tests, `cargo clippy -p <crate> -- -D warnings` for lints, `pnpm check` for the website.
- **Implement:** now write the code. Match the existing style exactly. No new patterns, no clever tricks, no "I'll come back to this." The implementation is the *final* form.
- **Check (long-term):** before declaring done, ask *"Will this need to be retouched in six months?"* If yes, redesign. A "done" change is a change nobody has to come back to.

## 6. The file header convention

Every source file starts with a top-level comment that:

1. Names the owner: `PascalElixir` and the `axolrs` GitHub organization.
2. Briefly describes what the file is about.

For Rust:

```rust
// Owner: PascalElixir / axolrs (GitHub org)
// File: Brief one-sentence description of what this file does.

fn main() { ... }
```

For Svelte:

```svelte
<!--
  Owner: PascalElixir / axolrs (GitHub org)
  Brief: one-sentence description of what this component does.
-->
```

For Markdown documentation: **no header comment, no frontmatter, pure content.** The first line of the file is the first line of content.

## 7. The function summary convention

Every function gets a one-line comment directly above its signature describing what the function does and what it is used for.

```rust
// Parses an Axolotl source string into a module; called from the CLI driver and the interpreter.
pub fn parse(src: &str) -> Result<Module, Vec<Diagnostic>> { ... }
```

## 8. The no-comments-in-bodies rule

There are no comments inside function bodies. The summary is above the signature, not inline. No `// FIXME`, no `// TODO`, no `// HACK`, no `// NOTE`, no block comments inside function bodies. The CI enforces this with a `no-comments-in-bodies` check.

## 9. The Markdown no-comments rule

Markdown documentation files (in `docs/`, in `axolsite/src/routes/`, anywhere a `.md` file exists) have no comments at all. No HTML comments, no `<!-- ... -->` blocks, no frontmatter. Pure content. The CI lints for this.

## 10. The naming conventions

- **Rust:** `snake_case` for functions / variables / modules, `PascalCase` for types / traits, `SCREAMING_SNAKE_CASE` for `const`. 4-space indent, no tabs.
- **Svelte / TypeScript:** `PascalCase.svelte` for components, `camelCase` for props and stores, `SCREAMING_SNAKE_CASE` for env-derived constants. 2-space indent.
- **CSS:** Tailwind utility classes only. No custom CSS files outside `axolsite/src/routes/layout.css` (the shadcn-svelte stylesheet, which is not to be edited).

## 11. How to add a dependency

For Rust:

```bash
cargo search <name>                # find the latest version
cargo add <name>@<version>         # add it to the current crate
```

For the website:

```bash
cd axolsite
pnpm add <name>                    # for runtime deps
pnpm add -D <name>                 # for dev deps
```

Never edit `package.json` dependencies by hand. The lockfile is the source of truth.

Every dependency is recorded in `WORKLOG.md` with: name, version, why it's needed, what it replaces if anything, what its license is.

## 12. How to write a test

Tests live in `tests/` subdirectories inside each crate, or in `#[cfg(test)] mod tests` blocks inside `src/`. Use the standard Rust test framework plus the helpers from `axolc-testkit`:

```rust
use axolc_testkit::*;

#[test]
fn parses_let_keyword() {
    let module = parse("let x = 1").unwrap();
    assert_eq!(module.statements().len(), 1);
    assert!(matches!(module.statements()[0], Statement::Let { .. }));
}
```

Tests are real assertions, not placeholders. The total test count is reported by the CI on every commit. The count must monotonically grow (or stay equal) - never shrink.

## 13. How to add a benchmark

Each build-your-own-x port lives in `benchmarks/<name>/` with two subdirectories: `rust/` (the hand-written reference) and `axol/` (the Axolotl port). The benchmark harness is in `benchmarks/<name>/bench/run.rs`.

The Axolotl port must be **within 5%** of the Rust reference on the same workload. The CI asserts this. If your port exceeds the threshold, the generated Rust isn't idiomatic enough - fix the Axolotl surface or the codegen, do not loosen the threshold.

## 14. How to add a documentation page

The website at `axolsite/` is a SvelteKit 5 project. To add a page:

1. Create a new directory under `axolsite/src/routes/`.
2. Add a `+page.svelte` file inside.
3. Use the existing `shadcn-svelte` components from `$lib/components/ui/`, `$lib/components/magic/`, `$lib/components/fancy/`, `$lib/components/spell/`.
4. Style with Tailwind classes. Do not add a `<style>` block. Do not touch `axolsite/src/routes/layout.css`.

The CI lints the codebase for any custom CSS files (anything outside `node_modules/`, `src/routes/layout.css`, and the shadcn-svelte component files) and fails if it finds any.

## 15. Submitting a change

1. Make your change on a feature branch.
2. Run the tests: `cargo test -p <affected-crate>`.
3. Run the lints: `cargo clippy -p <affected-crate> -- -D warnings`.
4. Run the website checks if you touched the website: `pnpm check`.
5. Commit with a clear message: what changed, why, what it does.
6. Open a pull request. The PR description should explain the change, link to any relevant issue, and reference the relevant section of `PROGRAMMINGLANGUAGEBIBLE.md` (if the change is to the language surface).
7. The CI runs the full test suite. The PR must pass before merge.
8. The PR must be reviewed by at least one maintainer.
9. The PR must not introduce any inline comments (R4), any custom CSS in the website (R3.5 of the website rules), any manual `package.json` edits, or any hand-pinned dependency versions.

## 16. Release process

The release process is:

1. Cut a `0.1.0-moss` (alpha) tag. Smoke-test on the website.
2. Cut `0.1.0-tropical` (beta). Run the full benchmark suite. Verify the 5% threshold on every port.
3. Cut `0.1.0-moss` (release). Update the website. Publish to crates.io. Publish the GitHub release.
4. The `lush` release is the first public release. After that, point releases follow semver: `0.1.1`, `0.1.2`, etc.

The release artifacts include:

- The workspace's compiled binaries.
- The website's static build.
- The bench results.
- The generated documentation.

---

Welcome to the project. The compiler, the interpreter, the LSP, the toolchain, the website, the docs, the benchmarks - all of it is open source, all of it is MIT OR Apache-2.0, all of it is free.

The most useful contribution you can make is to read `PROGRAMMINGLANGUAGEBIBLE.md` end to end, then look at the code, then write the test that should pass before the implementation exists, then implement it. That loop is how the project grows.
