# Contributing to Axolotl

Axolotl is a high-level, all-purpose, memory-safe, zero-GC programming
language with native compilation through idiomatic Rust. We welcome
contributions that advance the language, the compiler, the interpreter,
the LSP, the toolchain, the documentation, the benchmarks, the website,
or anything else in the project.

This document explains how to contribute.

## Code of Conduct

Participation in this project is governed by the [Code of Conduct](/about#code-of-conduct).
Please be excellent to each other.

## Project Structure

The repository is a Cargo workspace with the following layout:

```
axolotl/
├── crates/
│   ├── axolc/              # the compiler CLI
│   ├── axolc-core/         # the compiler library (lexer, parser, HIR, codegen)
│   ├── axolc-testkit/      # common test helpers
│   ├── axol-hot-runner/    # the interpreter + JIT
│   ├── axol-analyzer/      # Gills, the LSP
│   └── bucket/             # the build orchestrator
├── editors/
│   ├── zed-axolotl/        # Zed extension
│   ├── nvim-axolotl/       # Neovim plugin
│   └── axolcode/           # VS Code extension
├── tooling/
│   └── tree-sitter-axol/   # tree-sitter grammar (8 language bindings)
├── benchmarks/             # 20 build-your-own-x ports
├── examples/               # smaller example projects
├── axolsite/               # the SvelteKit website
├── scripts/                # generator and migration scripts
└── docs/                   # markdown documentation
```

## The Workflow Rule

Every change follows the workflow:

```
Research  ->  Plan  ->  Verify  ->  Implement  ->  Check (long-term)
```

- **Research**: read the existing code, the docs, the relevant upstream
  (Rust reference, Cargo book, etc.). Use `cargo search <dep>` for the
  latest version of any Rust crate; never guess.
- **Plan**: before writing code, write the plan to the worklog. Which
  files change, which APIs touch which, what the test surface looks
  like, what could go wrong.
- **Verify**: before merging, run the smallest unit that exercises the
  change. A single `cargo test -p <crate>`, a single `pnpm test` for
  the affected site page.
- **Implement**: now write the code. Match the existing style exactly.
- **Check (long-term)**: before declaring done, ask: "Will this need to
  be retouched in six months?" If yes, redesign.

## The Hard Rules

These are non-negotiable:

1. **The only owner handles in code, commits, and generated docs are
   `PascalElixir` and the `axolrs` GitHub organization.** No other account
   name appears anywhere.
2. **License is `MIT OR Apache-2.0`.** Every `Cargo.toml` declares
   `license = "MIT OR APACHE-2.0"`.
3. **Rust edition is `2024`.** Every `Cargo.toml` declares `edition = "2024"`.
4. **Never guess a dependency version.** Use `cargo search <name>` for
   Rust crates and `pnpm view <name>` for npm packages. Use the version
   it returns.
5. **Every source file has a header comment** naming the owner and
   briefly describing what the file does.
6. **Every function has a one-line summary above its signature.** No
   inline comments inside function bodies. No `// TODO`, `// FIXME`,
   `// HACK`, `// NOTE`.
7. **Markdown docs have no comments.** No HTML comments, no `<!-- -->`,
   no frontmatter. Pure content.
8. **Never touch `axolsite/src/routes/layout.css`.** It is the central
   shadcn-svelte stylesheet. Always use Tailwind classes. No `<style>`
   blocks in Svelte files.
9. **Use `pnpm add` for the website.** Never edit `package.json` by
   hand. The CI asserts the lockfile is consistent.
10. **Sub-agents run in batches of four.** Each sub-agent gets the full
    brief.

## Setting Up Your Environment

```bash
git clone https://github.com/axolrs/axolotl.git
cd axolotl

# Rust
rustup update stable
rustc --version
cargo --version

# Website
cd axolsite
pnpm install
cd ..

# Smoke test
cargo check -p axolc-core
cargo test -p axolc-core
```

## Running the Tests

```bash
# All Rust tests
cargo test --workspace

# Single crate
cargo test -p axolc-core

# Website
cd axolsite
pnpm check
pnpm build
```

The project has 20,000+ tests. A test function with three `assert_eq!`
calls counts as three tests. The CI reports the total; the worklog
tracks the running count.

## Running the Benchmarks

```bash
# Build axolc first
cargo build -p axolc --release

# Run one benchmark
cd benchmarks/git
bash bench/run.sh

# Run all benchmarks
bash benchmarks/run_all.sh
```

The 5% performance gate is enforced: `axol_time / rust_time <= 1.05`.

## Submitting Changes

1. Fork the repository.
2. Create a feature branch: `git checkout -b my-feature`.
3. Make your changes. Follow the hard rules. Run the tests.
4. Commit with a clear message:
   ```
   [axolc-core] Add support for the `??` operator

   Adds the `??` (default) operator, which returns the inner value of
   a `Result` on `Ok` and the right-hand side on `Err`. Desugars to a
   match expression.

   Closes #42.
   ```
5. Push to your fork: `git push origin my-feature`.
6. Open a pull request. Reference the issue if there is one.

## The Worklog

Every phase ends with a `WORKLOG.md` entry at the repo root. The entry
format:

```markdown
## Phase N - Title (YYYY-MM-DD)

**Status:** complete | in-progress | blocked
**Test count:** N (was N-1; +delta)
**Checkpoint:** https://temp.sh/XXXXX, https://x0.at/XXXXX

### What changed
- file 1: description
- file 2: description

### What was verified
- `cargo test -p <crate>`: N tests pass, 0 fail
- `cargo check -p <crate>`: clean
- `pnpm check` (if website): clean

### What is known broken
- Issue 1: description, deferred to phase N+M

### Decisions made
- Decision 1: rationale

### Next phase
Phase N+1 starts from this state.
```

The worklog is the contract. The next session reads the last entry and
continues from there.

## License

By contributing, you agree that your contributions are licensed under the
**MIT OR Apache-2.0** license, the same as the rest of the project.

## What's Next

- Read the [Architecture](/docs/architecture) guide to understand the
  system design.
- Read the [Toolchain](/docs/toolchain) guide for the `bucket` command
  reference.
- Read the [Getting Started](/docs/getting-started) guide to install
  Axolotl.
