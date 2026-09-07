# Contributing to Axolotl

> **Axolotl is a high-level, all-purpose, memory-safe, zero-GC programming language with native compilation through idiomatic Rust. We welcome contributions that advance the language, the compiler, the interpreter, the LSP, the toolchain, the documentation, the benchmarks, the website, or anything else in the project.**


## 1. Code of Conduct

This project adheres to a [Code of Conduct](CODE_OF_CONDUCT.md). By participating, you are expected to uphold it. Report unacceptable behavior to the maintainers ([PascalElixir](https://github.com/PascalElixir), the `axolrs` GitHub organization).

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
```

## 3. How to get the codebase

```bash
git clone --recurse-submodules git@github.com:axolrs/Axolotl.rs.git
cd Axolotl.rs
```

## 4. The toolchain

```bash
rustup update stable
rustc --version    # expect 1.98+
cargo --version
pnpm --version
node --version     # node 26 was used during dev so 26 or later ig.
nvim --version | head -1
code --version
tree-sitter --version
cc --version
```

## 5. The workflow

Every change follows **research → plan → verify → implement → check (long-term)**.

- **Research:** read the relevant section of `PROGRAMMINGLANGUAGEBIBLE.md` (the spec), the relevant upstream docs (Rust, Cargo, Cranelift, Svelte, shadcn-svelte), and any existing code in the affected crate.
- **Plan:** before writing code, write the plan to `yourname/idea_and_plan.md`. Which files change, which APIs touch which, what the test surface looks like, what could go wrong.
- **Verify:** before merging, run the smallest unit that exercises the change. `cargo check -p <crate>` for a smoke test, `cargo test -p <crate>` for tests, `cargo clippy -p <crate> -- -D warnings` for lints, `pnpm check` for the website.
- **Implement:** now write the code.It is encouraged to match the existing style. The implementation should be it's *final* form (yeah after testing).
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

> NOTE: you will put your name + axolrs if you're part of the organization (encouraged to join it's welcoming)

## 7. The function summary convention

Every function gets a one-line or multiline comment directly above its signature describing what the function does and what it is used for. It's fine if its more than that, it is me finding hard time reading code with a lot of unstructured comments that's it.

```rust
// Parses an Axolotl source string into a module; called from the CLI driver and the interpreter.
pub fn parse(src: &str) -> Result<Module, Vec<Diagnostic>> { ... }
```

## 8. The naming conventions

- **Rust:** `snake_case` for functions / variables / modules, `PascalCase` for types / traits, `SCREAMING_SNAKE_CASE` for `const`. 4-space indent, no tabs. (basically what default rust looks like)
- **Svelte / TypeScript:** `PascalCase.svelte` for components, `camelCase` for props and stores, `SCREAMING_SNAKE_CASE` for env-derived constants. 2-space indent. (I'm fine if you use different cases for front end but this one is encouraged)
- **CSS:** Tailwind utility classes only. No custom CSS files outside `axolsite/src/routes/layout.css` (the shadcn-svelte stylesheet, which is not what I would recommend someone edit but cases exits so please put at the bottom whatever you write).


## 9. How to write a test

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

Tests are real assertions, not placeholders. The total test count is reported by the CI on every commit. The count must monotonically grow (or stay equal or if reduced it should come with some reasoning).

## 10. How to add a benchmark

Each build-your-own-x port (These are AI generated and looks good to me so I kept them) lives in `benchmarks/<name>/` with two subdirectories: `rust/` and `axol/` (the Axolotl port). The benchmark harness is in `benchmarks/<name>/bench/run.rs`.

The Axolotl port must be **within 5%** of the Rust reference on the same workload. The CI asserts this. If your port exceeds the threshold, the generated Rust isn't idiomatic enough - fix the Axolotl surface or the codegen, do not loosen the threshold.

## 11. How to add a documentation page

The website at `axolsite/` is a SvelteKit 5 project. To add a page:

1. Create a new directory under `axolsite/src/routes/`.
2. Add a `+page.svelte` file inside.
3. Use the existing `shadcn-svelte` components from `$lib/components/ui/`, `$lib/components/magic/`, `$lib/components/fancy/`, `$lib/components/spell/`.
4. Style with Tailwind classes.

The CI lints the codebase for any custom CSS files (anything outside `node_modules/`, `src/routes/layout.css`, and the shadcn-svelte component files) and fails if it finds any.
NOTE: modify if it's a bad idea I always liked to keep css in one file for the front end.

## 12. Submitting a change

1. Make your change on a feature branch. e.g. `git checkout -b lake/feature`
2. Run the tests: `cargo test -p <affected-crate>`.
3. Run the lints: `cargo clippy -p <affected-crate> -- -D warnings`.
4. Run the website checks if you touched the website: `pnpm check`.
5. Commit with a clear message: what changed, why, what it does.
6. Open a pull request. The PR description should explain the change + modified files (well IK git diff exists but I like it), link to any relevant issue, and reference the relevant section of `PROGRAMMINGLANGUAGEBIBLE.md` (if the change is to the language surface).
7. The CI runs the full test suite. The PR must pass before merge.
8. The PR must be reviewed by at least one maintainer.

## 13. Release process

The release process is:

1. Cut a `0.1.0-moss` (alpha) tag. 
2. Cut `0.1.0-tropical` (beta).
3. Cut `0.1.0-lush` (release).
4. The `lush` release is the first public release. After that, point releases follow semver: `0.1.1`, `0.1.2`, etc.

The release artifacts include:

- The workspace's compiled binaries.
- The website's static build.
- The bench results.
- The generated documentation.

---

Welcome to the project. The compiler, the interpreter, the LSP, the toolchain, the website, the docs, the benchmarks - all of it is open source, all of it is MIT OR Apache-2.0, all of it is free.

The most useful contribution you can make is to read `PROGRAMMINGLANGUAGEBIBLE.md` end to end, then look at the code, then write the test that should pass before the implementation exists, then implement it. That loop is how the project grows.

# Personal Notes

The Core Compiler was written by me, with help from this repo: [Write a C Interpreter](https://github.com/lotabout/write-a-C-interpreter), and another repo it recommended. I checked those out and then started writing the code myself.

The code was audited with AI, had some problems, and I fixed them with help from AI and myself. I only wrote the core compiler. The benchmarks and the other files were handled with AI assistance. I also used AI to add important comments to the code.

I wrote the Neovim editor integration myself. Tree-sitter, WASM, and the other editor integrations were handled with AI.

I did read all of the code, including the parts written with AI, and it all makes sense to me. (hey we face problems in prod thats classic)

I'm 99% open to criticism, but I want patches sent as well.

"Talk is Cheap, Send Patches" - FFmpeg devs

Anyways, It was a project to improve my skills but I figured it could acually solve some problems such as the interpreter for development it will save a ton of time probably
