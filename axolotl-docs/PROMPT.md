# Axolotl - Master Engineering Brief (v1.1)

> **Audience:** the AI agent (or agent team) picking this project up.
> **Purpose:** state, once and for all, what Axolotl is, what it is not, what the rules are, how the work is structured, and what "done" means. This document does not shift scope. It does not add new ideas. It does not contradict itself. Read it end to end, then execute. If a downstream artifact (a sub-agent prompt, a phase plan, a generated comment) appears to conflict with this document, **this document wins.** If something genuinely needs to change, stop and surface it in the worklog - do not paper over it with a re-interpretation.

**Version:** 1.1 - consolidated, scope-locked. **v1.1 delta vs v1.0:** expanded section 3.1 with the full scaffold inventory (every file named), expanded section 3.3 with detailed editor integrations (Zed, Neovim, VS Code `axolcode`, tree-sitter), expanded section 11 with full per-editor integration details and the actual `axolcode` file layout. The scaffold is now described completely.
**Owner:** `PascalElixir` and the **`axolrs`** GitHub organization - the only handles that appear in code, commits, generated docs, and public surfaces.
**Toolchain (today's UTC date):** Rust edition **2024** with `rustc` and `rustup` at the latest stable. Many ecosystem changes since 2024 are now baseline - verify version-dependent APIs against current docs, do not guess.
**Year context:** **2026.** Calendar, library versions, MCP, LSP, and the surrounding tooling ecosystem all reflect 2026. The 2024 toolchain edition is the *Rust edition*, not the year. Apply common sense to dated APIs.

---

## 0. How to Read This Document

This brief is intentionally long. The project has many subsystems (compiler, interpreter, JIT, LSP, formatter, linter, build orchestrator, package registry, website, documentation) and a specific license posture - past briefs lost project time by being vague in the places that needed to be precise. Skim-readers will under-deliver. Read it top to bottom once, then keep it open as a reference.

The document is divided into numbered sections. **Section 1** is the mission. **Section 2** is the rules. **Section 3** onward is the work. The **Hard Rules** in Section 2 are non-negotiable - if you find yourself tempted to break one, that is the moment to stop and reread Section 2.

**Workflow rule (applies to every step, every sub-agent, every file you touch):**

```
Research  →  Plan  →  Verify  →  Implement  →  Check (long-term)
```

- **Research:** read the existing code, the docs in `axolotl-docs.zip`, the relevant upstream (Rust reference, Cargo book, Cranelift docs, Svelte 5 docs, shadcn-svelte registry). Use `cargo search <dep>`, the official docs site, the crate's `lib.rs` / `docs.rs`. **Never** guess a version - `cargo search <name>` for the latest, then add it.
- **Plan:** before writing any code, write the plan to the worklog. Which files change, which APIs touch which, what the test surface looks like, what could go wrong.
- **Verify:** before merging, run the smallest unit that exercises the change. A single `cargo test -p <crate>`, a single `pnpm test` for the affected site page, a single `bucket check` for the language surface.
- **Implement:** now write the code. Match the existing style exactly.
- **Check (long-term):** before declaring done, ask: *"Will this need to be retouched in six months?"* If yes, the implementation is wrong. Re-design. A "done" task is a task nobody has to come back to.

The purpose of this rule: **nobody should have to retouch finished work.** The implementation is the *final* form.

---

## 1. Mission

### 1.1 What Axolotl Is

**Axolotl (`.axol`)** is a high-level, all-purpose, memory-safe, zero-GC programming language with an exceptionally intelligent compiler, native compilation through idiomatic Rust, full Cargo ecosystem compatibility, no Axolotl runtime, mixed `.axol` + `.rs` projects, and a unified IDE (`Gills`, the axol-analyzer). The full design lives in the design docs inside `axolotl-docs.zip` (see Section 4). Read the bible and the architecture document first.

The end state is a working language implementation with:

- A real, compiling **Axolotl → Rust compiler** (`axolc`).
- A real, working **interpreter + JIT** (`axol-hot-runner`) for sub-200ms dev iteration.
- A real, working **LSP** (`Gills` / `axol-analyzer`) that understands both languages.
- A real, working **build orchestrator** (`bucket`) with a global `Pond` cache.
- A real, working **formatter** (`Shed`), **linter** (`Neoten`), **auto-fixer** (`Regrow`).
- A real, working **docs site** (the `axolsite` SvelteKit project).
- **20,000+ tests** across the compiler, interpreter/JIT, toolchain, LSP, formatter, linter, and editor integration.
- **10 build-your-own-x projects** written in both Axolotl and Rust, side-by-side in `benchmarks/` and `examples/`, demonstrating that the generated Rust is **zero-overhead** compared to hand-written Rust.
- Complete standard repo files: `LICENSE` (MIT), `CONTRIBUTING.md`, `CODE_OF_CONDUCT.md`.
- A working **Zed extension** for Axolotl (`editors/zed-axolotl`).

The project is **not** a research toy. It is a real, working language that compiles to a real, working toolchain and ships as native binaries that consume the entire Rust ecosystem.

### 1.2 What Axolotl Is Not

- **Not a Rust replacement.** Axolotl is a new frontend for the Rust ecosystem. Rust source files (`*.rs`) are first-class citizens in the same project as `.axol` files. The Cargo ecosystem is the dependency graph. There is no "Luma std library" or "Axolotl SDK" to invent.
- **Not a Lua clone.** The syntax is Lua-flavored (`if/then/end`, `function ... end`, `:` method calls, tables), but the semantics are statically typed and Rust-grade. No GC, no metatables at runtime, no `nil` as a universal inhabitant.
- **Not a transpiler to source.** `axolc` produces idiomatic Rust source for `bucket emit-rust` debugging, but the primary compilation path is metadata-aware: the compiler reads Rust crate `.rmeta` files and generates calls into real Rust crates. No parallel "AxolotlWgpu" wrappers.
- **Not single-domain.** Not just for games, not just for systems, not just for web. Games are one workload among many. The full scope is applications, systems, science, infrastructure, compilers, embedded, aerospace, robotics, and games.
- **Not a runtime language.** There is no Axolotl VM, GC, interpreter-runtime, scheduler, object system, or hidden heap. If you need an `Arc`, you import `std::sync::Arc`. If you need nothing, the compiler doesn't inject something.

### 1.3 The Success Criteria

The project is done when every line of this sequence holds:

```
A user can:
  → install Bucket via `curl -sSf https://axolotl.rs/install | sh`
  → run `bucket new my-game`
  → edit `src/main.axol` with a small Axolotl program
  → run `bucket run dev` and see code execute in <200ms after save
  → run `bucket run release` and get a native binary
  → import any Cargo crate (`use "tokio"`, `use "wgpu"`, `use "serde"`, ...)
  → mix `.axol` and `.rs` files in the same project
  → run `bucket test` and see 20,000+ tests pass
  → run `bucket doc` and read a generated reference
  → open the project in Zed and get Axolotl completions, hovers, diagnostics
  → read the docs at https://axolotl.rs (the SvelteKit site)
  → see the 10 build-your-own-x ports in benchmarks/ and examples/
  → confirm the Axolotl implementations match the Rust implementations in performance
```

When every line holds, the project is done.

---

## 2. Hard Rules

These rules are non-negotiable. They are not "guidelines." They are the load-bearing constraints of the project. If the work seems to require breaking one, the work is wrong - find a different way.

### 2.1 Project Hygiene Rules

**R1. The only owner handles in code, commits, and generated docs are `PascalElixir` and the `axolrs` GitHub organization.** No other account name - personal, secondary, "also me," previous alias - appears anywhere in the codebase, git history, or generated artifacts. Every file's header (see R3) uses exactly these.

**R2. License posture is fixed and non-negotiable.** This project is dual-licensed **MIT OR Apache-2.0** at the user's preference. The `LICENSE` file at the root of the repository is the MIT license. The `Cargo.toml` declares `license = "MIT OR APACHE-2.0"`. The website, the documentation, and the design docs are MIT OR Apache-2.0. No other license is acceptable.

**R3. Every source file in `crates/`, `axolsite/`, `editors/`, `benchmarks/`, `examples/`, `tests/`, `scripts/`, and `docs/` starts with a top-level comment** that:

1. Names the owner (`PascalElixir` and the `axolrs` GitHub org).
2. Briefly describes what the file is about.
3. For Rust files: uses the standard header format. For Svelte files: same. For Markdown docs (`.md`): the header is **plain content only, no comments, no frontmatter**. The header is the first non-empty content in the file.

The Rust file header looks like:

```rust
// Owner: PascalElixir / axolrs (GitHub org)
// File: Brief one-sentence description of what this file does.

fn main() { ... }
```

The Svelte component header looks like:

```svelte
<!--
  Owner: PascalElixir / axolrs (GitHub org)
  Brief: one-sentence description of what this component does.
-->
```

**R4. Every function gets a one-line comment directly above its signature** describing what the function does and what it is used for. This is the only allowed comment location inside a function's surrounding context. No `// FIXME`. No `// TODO`. No `// HACK`. No `// NOTE`. No block comments inside function bodies. No KDoc / JSDoc / TSDoc blocks. No decorative banners. No `// === Section ===` dividers.

Example:

```rust
// Computes the natural log of x with a domain check; called from the math prelude.
fn ln(x: Float) -> Float { ... }
```

The CI enforces this with a `no-comments-in-bodies` check. PRs that introduce inline comments are blocked.

**R5. Markdown documentation files (in `docs/`, in `axolsite/src/routes/`, anywhere a `.md` file exists) have no comments at all.** No HTML comments, no `<!-- ... -->` blocks, no decorative frontmatter. Pure content. This is a hard rule, not a guideline. CI lints for it.

**R6. The language is open source, free, no-cost, forever.** No "premium tier," no "enterprise edition," no paywalled features. The compiler, the interpreter, the JIT, the LSP, the formatter, the linter, the auto-fixer, the build orchestrator, the docs, the website - all free, all open source, all under MIT OR Apache-2.0. The standard library and the build-your-own-x benchmarks are part of the public release.

**R7. The 20,000+ test rule.** The project is done when there are at least **20,000 tests** in aggregate, distributed roughly as:

- 10,000+ tests for the **language itself** (lexer, parser, type system, ownership inference, pattern matching, closures, async, traits, generics, FFI, all the surface in `PROGRAMMINGLANGUAGEBIBLE.md`).
- 10,000+ tests for the **compiler / interpreter / JIT / toolchain / LSP / editor integration**.

The "tests" count is **real assertions**, not lines. A single test function with three assertions counts as three tests, not one. The CI reports the total. The worklog tracks the running count. **Phase 0 establishes the test infrastructure. Every subsequent phase adds tests before adding features.**

**R8. The build-your-own-x rule.** The project must include **10 complete ports** of projects from `https://github.com/codecrafters-io/build-your-own-x`, written in **both Axolotl and Rust**, side by side in `benchmarks/` and `examples/`. The Axolotl versions must demonstrate **zero overhead** vs the hand-written Rust versions. The user picks the 10 projects; the prompt for this brief suggests a starter list (Section 12). The benchmark suite runs both and asserts the Axolotl version is **at most 5% slower** than the Rust version on the same workload (somewhere within LLVM's own variance).

### 2.2 Build System Rules

**R9. Edition is 2024. Always.** Every `Cargo.toml` declares `edition = "2024"`. The workspace `Cargo.toml` has `edition = "2024"` in `[workspace.package]`. There is no exception to this rule.

**R10. Never guess a dependency version. Always look it up.** Every dependency in every `Cargo.toml` is added with the version returned by `cargo search <name>`. Never a pinned older version, never a caret on a stale major. The CI runs `cargo outdated` and `cargo audit` to flag drift. Same rule for the website: `pnpm add <package>` adds the latest; never manually edit `package.json`'s `dependencies` block.

```bash
# Wrong
clap = "4.5.0"

# Right
$ cargo search clap | head -5
# pick the latest stable, then:
clap = "4.6.6"  # or whatever the search returns today
```

**R11. Install the latest Rust toolchain before any work.** Every agent and sub-agent runs:

```bash
$ rustup update stable
$ rustc --version
# expect: rustc 1.x.y (...)  (whatever the current stable is, 2026 era)
```

The edition in `Cargo.toml` stays at `"2024"` (it's a Rust *edition*, not a year).

**R12. Never run `cargo build --workspace` as a first move.** That's a 5-30 minute compile. The tasks that *are* safe:

```bash
cargo check -p <crate>               # cheapest; the IDE-quality smoke test
cargo test -p <crate>                # for the tests in one crate
cargo build -p <crate>                # if you actually need the artifact
cargo run -p <crate> -- <args>       # to run that crate's binary
cargo clippy -p <crate> -- -D warnings
```

**R13. The `pond/` and `target/` directories are never committed.** Both are in `.gitignore`. The `pond/` cache is per-machine; the `target/` is the local cargo output.

**R14. Lockfile discipline.** `Cargo.lock` is committed (the library API is consumed by external users). `pnpm-lock.yaml` is committed (the website is deterministic). The lockfiles are updated only when dependencies change, and the update is its own commit with a clear message.

### 2.3 Operating Rules

**R15. Iteration speed beats compilation completeness.** The right order is: small focused change → run the smallest relevant test → fix → run the next test → fix → next feature. The wrong order is: try to compile everything → drown in errors → give up. The first half of every phase is focused on the affected crate(s); the second half is integration. Compile verification happens deliberately, one crate at a time.

**R16. Sub-agents run in batches of four.** The tool limit is four concurrent sub-agents. Phases are decomposed so that, within a phase, up to four sub-agents can work in parallel on independent surfaces (e.g., one on `axolc`, one on `axol-hot-runner`, one on the website, one on tests). When the four return, the next four (or fewer) start. Do not run serially when parallelism is available; do not exceed four at a time.

**R17. Sub-agents receive this brief in full (or the relevant section, if the sub-task is narrow enough), and they execute against the production codebase, not the prototype.** If a sub-agent starts writing new code against the wrong surface, stop them.

**R18. The production codebase is the source of truth.** The most recently committed state of the workspace is the version the next phase starts from. Local edits that are not committed and not checkpointed do not survive a session restart. The workflow is: edit → verify with the smallest unit test → commit → checkpoint tarball → next.

**R19. Phases end in checkpoints.** At the end of every phase, before doing anything else:

1. Stop in-flight work.
2. Produce a tarball of the current state of the repo, excluding `target/`, `pond/`, `node_modules/`, `.svelte-kit/`, `.wrangler/`, `dist/`, `.turbo/`. Use `tar --exclude` patterns, do not run `cargo clean` to produce the tarball.
3. Upload the tarball to `temp.sh` and to `x0.at` (redundancy). Use:
   ```bash
   curl -F "file=@repo-checkpoint-NN.tar.gz" https://temp.sh/upload
   curl -F "file=@-;filename=repo-checkpoint-NN.tar.gz" https://x0.at/ < repo-checkpoint-NN.tar.gz
   ```
   Capture both URLs. If the tarball is small enough, also use `file.io`.
4. Write a `WORKLOG.md` entry covering: what changed this phase, what was verified, what is known broken, what the next phase starts with, and any decisions made (with their rationale).
5. Confirm both checkpoint URLs and the worklog path before proceeding.

**R20. No excuses, no scope drift, no relitigation.** This brief is the spec. If something in the brief is wrong, say so once with evidence and propose the change in the worklog; do not silently reinterpret it. If a sub-agent produces output that contradicts the brief, reject the output and re-issue the task with the contradiction called out.

### 2.4 Dependency & API Rules

**R21. Every dependency is documented in the worklog.** Adding a dependency is a deliberate act. The worklog records: name, latest version found via `cargo search` / `pnpm view`, why it's needed, what it replaces if anything, what features are enabled, what its license is. No silent additions.

**R22. License compatibility is checked for every dependency.** MIT OR Apache-2.0 is the project's license. Every dependency must be a license compatible with both. The CI runs `cargo deny` (for Rust crates) and a license-check pass (for npm packages). If a dependency's license is not compatible, find a replacement before adding it.

**R23. The 10 build-your-own-x ports have known-good Rust implementations as references.** Each port is a directory containing `rust/` (the hand-written reference) and `axol/` (the Axolotl port). The Rust version is committed first; the Axolotl version is benchmarked against it. If the Axolotl version is more than 5% slower than the Rust version, the Axolotl port is wrong - the generated Rust is not idiomatic, or the Axolotl surface is being used suboptimally. Fix it. Do not paper over the gap with "close enough."

### 2.5 Language Correctness Rules

**R24. Generated Rust is readable.** `bucket emit-rust` produces Rust that a Rust developer would have written. No `axolotl_runtime::something_34891(...)` call sites. No `Box<dyn Any>` everywhere. No hidden machinery. CI lints the generated Rust for readability heuristics.

**R25. The borrow checker still runs.** Axolotl infers ownership, but the underlying Rust borrow checker still verifies the generated code. The user can write explicit `borrow T` / `mut T` / `move T` annotations, but the compiler still rejects unsafe lifetime combinations.

**R26. Foreign blocks (`cblock`, `cppblock`, `rblock`, `pyblock`) only generate build machinery when present.** A project with no `cblock` has no C toolchain configuration, no `cc` crate, no `build.rs` for C, no linker flags. CI asserts this - it greps the generated `Cargo.toml` and `build.rs` and verifies the absence of foreign-toolchain artifacts.

**R27. The interpreter / JIT is opt-in via `interpreted_dev_mode: true` in `Bucket.jsonc`.** Production builds (`bucket build --release`) never use the interpreter or JIT. The "no runtime" invariant is preserved in production. CI asserts this - running `bucket build --release` produces a binary that does not link the interpreter or JIT.

**R28. The 20,000+ tests are real.** No placeholder `assert!(true)` to inflate the count. No copy-pasted test cases. No `cargo test` flags that skip. The CI runs the full suite on every commit. The test count is reported in the worklog and must monotonically grow (or stay equal) - never shrink.

### 2.6 Licensing & Distribution Rules

**R29. The repository is dual-licensed MIT OR Apache-2.0.** Every source file (Rust, Svelte, TypeScript, Markdown) carries the same license posture. The `LICENSE` file at the root is the MIT license. A `NOTICE` file at the root credits the upstream dependencies and notes any attribution required by them.

**R30. No telemetry from any tool without explicit opt-in.** Crash reports and usage analytics are off by default in every tool (`axolc`, `bucket`, `Gills`, `axol-hot-runner`, the website). The opt-in is a per-tool toggle in the user's `Bucket.jsonc` or the relevant `~/.config/axolotl/` config. No silent data collection, ever.

**R31. The Zed extension, the LSP, the website, the documentation, the interpreter, the JIT, the compiler, the build orchestrator - all of them are part of the same MIT OR Apache-2.0 release.** There is no separate "premium" extension. There is no separate "Pro" build. There is one release, one license, one repo.

### 2.7 Code Quality Rules

**R32. Naming follows the language's idioms.** Rust uses `snake_case` for functions / variables / modules, `PascalCase` for types / traits, `SCREAMING_SNAKE_CASE` for constants. Svelte uses `PascalCase.svelte` for components, `camelCase` for props and stores. TypeScript uses `camelCase` for variables / functions, `PascalCase` for types / components, `SCREAMING_SNAKE_CASE` for env-derived constants.

**R33. The user-facing CLI follows Unix conventions.** `bucket <subcommand> [--flags] [args]`. Subcommands are verbs (`new`, `init`, `add`, `build`, `run`, `test`, `fmt`, `lint`, `fix`, `doc`, `clean`, `doctor`). Flags are long-form (`--release`, `--interpret`, `--watch`) and short-form (`-r`, `-i`, `-w`) where they make sense. Every command has `--help`. Every command has `--json` for machine-readable output.

**R34. Error messages are user-facing and Axolotl-level.** Errors mention the user's `.axol` line, not generated-Rust lines. Errors suggest fixes when the compiler knows one. Errors are clickable in the editor (Gills maps them to file:line:col spans).

**R35. No dead code, no commented-out code, no `// removed X because Y` artifacts.** If code is removed during a refactor, it is deleted, not commented out. The git history is the place to find old code; the working tree is clean.

---

## 3. Architecture Overview

### 3.1 Workspace Layout (every file in the scaffold, named)

The current production repository (as of the initial scaffold) is the **complete skeleton** described below. Every file is named. Every directory is laid out. The AI's job is to fill them in - not to invent new directories, not to rename the existing ones, not to merge them.

```
Axolotl.rs/
├── Cargo.toml                     # workspace; members are: axol-analyzer, axol-hot-runner, axolc,
│                                  # axolc-core, bucket, zed-axolotl. edition 2024, license MIT OR Apache-2.0.
├── Cargo.lock                     # the current placeholder lockfile
├── README.md                      # placeholder: "LICENSE - MIT OR APACHE-2.0 (Open Source Free to use forever)"
├── Makefile                       # empty (0 bytes) - Phase 0 fills it with the canonical targets
├── .gitignore                     # excludes /target, /pond, node_modules
│
├── crates/
│   ├── axolc/
│   │   ├── Cargo.toml             # depends on axolc-core, clap (derived)
│   │   └── src/main.rs            # CURRENTLY: println!("Hello, world!"); - Phase 9 fills it
│   │
│   ├── axolc-core/
│   │   ├── Cargo.toml             # NO dependencies yet - Phase 0 adds them
│   │   ├── .gitignore
│   │   └── src/lib.rs             # empty
│   │
│   ├── axol-hot-runner/           # the interpreter + JIT
│   │   ├── Cargo.toml             # NO dependencies yet - Phase 14-16 add them
│   │   └── src/main.rs            # CURRENTLY: println!("Hello, world!"); - Phase 14 fills it
│   │
│   ├── axol-analyzer/             # Gills, the LSP
│   │   ├── Cargo.toml             # NO dependencies yet - Phase 17-18 add them
│   │   └── src/main.rs            # CURRENTLY: println!("Hello, world!"); - Phase 17 fills it
│   │
│   └── bucket/                    # the build orchestrator
│       ├── Cargo.toml             # depends on axolc-core, clap (derived)
│       └── src/main.rs            # CURRENTLY: println!("Hello, world!"); - Phase 10-13 fills it
│
├── editors/
│   ├── zed-axolotl/               # Zed extension (WASM cdylib, NOT in workspace)
│   │   ├── Cargo.toml             # depends on zed_extension_api 0.7.0
│   │   ├── extension.toml         # the Zed extension manifest
│   │   └── src/                   # lib.rs goes here
│   │
│   ├── nvim-axolotl/              # Neovim plugin (Lua)
│   │   ├── ftdetect/axol.lua      # registers .axol as filetype "axol"
│   │   └── lua/axolotl/init.lua   # the setup function; calls vim.lsp.config("axol_analyzer", ...)
│   │                               # and registers the tree-sitter parser
│   │
│   └── axolcode/                  # ★ VS Code extension (TypeScript + Webpack)
│       ├── .npmrc                  # pnpm config
│       ├── .vscodeignore
│       ├── .vscode-test.mjs
│       ├── .vscode/
│       │   ├── extensions.json
│       │   ├── launch.json
│       │   ├── settings.json
│       │   └── tasks.json
│       ├── CHANGELOG.md
│       ├── README.md
│       ├── eslint.config.mjs
│       ├── package.json            # name: "axolcode", displayName: "AxolCode", vscode ^1.136.0
│       ├── pnpm-workspace.yaml
│       ├── pnpm-lock.yaml
│       ├── tsconfig.json           # ES2022, Node16, strict mode
│       ├── vsc-extension-quickstart.md
│       ├── webpack.config.js       # entry: ./src/extension.ts, output: dist/extension.js
│       └── src/
│           ├── extension.ts        # CURRENTLY: registers "axolcode.helloWorld" command
│           └── test/extension.test.ts  # the test scaffold
│
├── tooling/
│   └── tree-sitter-axol/          # tree-sitter grammar (8 language bindings)
│       ├── .editorconfig
│       ├── CMakeLists.txt          # C/C++ build
│       ├── Cargo.toml              # Rust binding: name = "tree-sitter-axolotlgrammer"
│       ├── Makefile                # generic make build
│       ├── Package.swift           # Swift Package Manager
│       ├── binding.gyp             # Node.js (native module)
│       ├── build.zig               # Zig binding
│       ├── build.zig.zon
│       ├── go.mod                  # Go binding
│       ├── grammar.js              # CURRENTLY: source_file: $ => "hello"  - full grammar is Phase 0+
│       ├── package.json
│       ├── pnpm-lock.yaml
│       ├── pnpm-workspace.yaml
│       ├── pom.xml                 # Java (Maven)
│       ├── pyproject.toml          # Python
│       ├── setup.py                # Python (legacy)
│       ├── tree-sitter.json        # the grammar metadata: name "axolotlgrammer", scope "source.axolotlgrammer"
│       ├── src/
│       │   ├── parser.c            # generated by `tree-sitter generate`
│       │   ├── grammar.json        # generated by `tree-sitter generate`
│       │   ├── node-types.json     # generated by `tree-sitter generate`
│       │   └── tree_sitter/        # the tree-sitter C runtime headers
│       └── bindings/               # the 8 language bindings: c, go, java, node, python, rust, swift, zig
│
├── axolsite/                      # SvelteKit 5 website (shadcn-svelte + magic UI)
│   ├── .gitignore
│   ├── .npmrc
│   ├── .vscode/
│   │   ├── extensions.json        # recommends svelte.svelte-vscode, bradlc.vscode-tailwindcss
│   │   └── settings.json          # associates *.css with tailwindcss
│   ├── README.md
│   ├── components.json            # shadcn-svelte config: tailwind 4, neutral baseColor
│   ├── package.json               # the dependency manifest (use `pnpm add`, never edit by hand)
│   ├── pnpm-lock.yaml             # the lockfile (gitignored, regenerated on `pnpm install`)
│   ├── pnpm-workspace.yaml
│   ├── tsconfig.json
│   ├── vite.config.ts
│   ├── src/
│   │   ├── app.d.ts
│   │   ├── app.html              # SvelteKit's HTML shell
│   │   ├── routes/
│   │   │   ├── +layout.svelte    # CURRENTLY: just imports layout.css and renders children
│   │   │   ├── +page.svelte      # CURRENTLY: "Welcome to SvelteKit" placeholder
│   │   │   └── layout.css        # ★ DO NOT TOUCH - central shadcn-svelte stylesheet
│   │   └── lib/
│   │       ├── index.ts
│   │       ├── utils.ts
│   │       ├── assets/favicon.svg
│   │       ├── components/
│   │       │   ├── ui/            # 38 shadcn-svelte components
│   │       │   ├── magic/         # 40+ magic components
│   │       │   ├── fancy/         # 11 fancy components
│   │       │   └── spell/         # 20+ spell components
│   │       └── hooks/is-mobile.svelte.ts
│   └── static/
│
├── benchmarks/                    # the 10 build-your-own-x ports (axol + rust)
│   ├── README.md                  # "20 Benchmark projects written in both Rust and Axol to prove theres no overhead on the language itself"
│   └── .gitkeep
│
├── examples/                      # smaller example projects
│   └── .gitkeep
│
├── tests/                         # cross-crate integration tests
│   └── (empty - Phase 0 fills with test scaffolding)
│
├── scripts/                       # bucket scripts the user defines in Bucket.jsonc
│   └── .gitkeep
│
├── docs/                          # markdown documentation (each is a top-level higharchy in the worklog)
│   └── (empty - Phase 0+ fills)
│
├── .github/
│   └── workflows/
│       ├── actions.yaml            # reusable actions (empty)
│       ├── benchmark.yaml          # benchmark CI (empty)
│       ├── cd.yaml                 # website deployment (empty)
│       ├── ci.yaml                 # main CI (empty)
│       ├── release.yaml            # release artifacts (empty)
│       ├── security_checks.yaml    # cargo audit / cargo deny (empty)
│       └── test.yaml               # long-form test suite (empty)
│
└── WORKLOG.md                     # cross-phase worklog (created in Phase 0)
```

**This is the complete scaffold. Every file listed above exists in the production codebase today.** The AI's job is to fill in the placeholders, not to invent new structure.

### 3.2 The Crates

| Crate | Current State | Responsibility |
|---|---|---|
| `axolc` | `main.rs` is `println!("Hello, world!")` | The CLI driver for the compiler. Parses arguments, drives `axolc-core`, prints diagnostics. |
| `axolc-core` | `lib.rs` is empty, `Cargo.toml` has no deps | The library form of the compiler. Lexer, parser, HIR, type system, ownership inference, diagnostics, Rust code generation, span mapping. Reused by `axol-hot-runner` (in-process) and `axol-analyzer` (Gills, in-process). |
| `axol-hot-runner` | `main.rs` is `println!("Hello, world!")` | The interpreter + JIT. Reads `.axol` and `.rs` files. Tree-walks or Cranelift-compiles the HIR. Hot reloads on file save. Designed for sub-200ms dev iteration. Full design in `Interpreter.md`. |
| `axol-analyzer` | `main.rs` is `println!("Hello, world!")` | Gills. The LSP. Fork of `rust-analyzer`, extended to understand both languages. Full design in `LSP.md`. |
| `bucket` | `main.rs` is `println!("Hello, world!")` | The project / build / dependency / package orchestrator. Sits above Cargo. Manages the `pond/` cache, the `Bucket.jsonc` manifest, the toolchain (Gills / Neoten / Shed / Regrow / Ambystoma / Salamander / Larva / Molt / Eggbox). |

### 3.3 The Editor Integrations

The scaffold ships **three** editor integrations. All three talk to Gills (`axol-analyzer`) over the standard LSP. The first hour of any editor work is to fill in the placeholder.

#### 3.3.1 The Zed Extension - `editors/zed-axolotl/`

A Rust crate that compiles to WASM. **Not** in the workspace because the WASM target is incompatible with the rest of the workspace.

**Files:**
- `Cargo.toml` - `crate-type = ["cdylib"]`, depends on `zed_extension_api = "0.7.0"`.
- `extension.toml` - the Zed extension manifest (`id = "axolotl"`, `name = "Axolotl"`).
- `src/` - the extension entrypoint (currently empty). The entrypoint spawns `bucket gills --stdio` as a subprocess and forwards LSP.

**Phase 19 fills this in.** See `ZED_TOOLCHAIN.md` for the full setup.

#### 3.3.2 The Neovim Plugin - `editors/nvim-axolotl/`

A Lua plugin. **Already partially scaffolded.** The user can read the existing files and continue.

**Files (current):**
- `ftdetect/axol.lua` - `vim.filetype.add({ extension = { axol = "axol" } })`
- `lua/axolotl/init.lua` - the `M.setup(opts)` function. Currently registers the `axol_analyzer` LSP client and the tree-sitter parser. The function takes a config table with `cmd` (the LSP command, default `{"axol-analyzer"}`) and `grammar_path` (the tree-sitter grammar path).

**Phase 20 completes this.** The plugin should:
- Expose `:Axolotl run` to invoke `bucket run`.
- Expose `:Axolotl build` to invoke `bucket build --release`.
- Expose `:Axolotl test` to invoke `bucket test`.
- Expose `:Axolotl fmt`, `:Axolotl lint`, `:Axolotl fix` for the toolchain.
- Expose `:Axolotl repl` (v2) to drop into the interpreter REPL.
- Use `vim.lsp.config("axol_analyzer", ...)` to configure the LSP, with `root_markers = { "Bucket.jsonc", "Cargo.toml", ".git" }` (already in the scaffold).

#### 3.3.3 The VS Code Extension - `editors/axolcode/`

**The user specifically asked for this. I almost missed it.** A TypeScript + Webpack VS Code extension. The package name is `axolcode`, the display name is `AxolCode`, the engine is `vscode ^1.136.0`.

**Files (current):**
- `package.json` - full extension manifest with commands, scripts (`compile`, `watch`, `package`, `compile-tests`, `watch-tests`, `pretest`, `lint`, `test`).
- `tsconfig.json` - `Node16`, `ES2022`, strict mode.
- `webpack.config.js` - entry `./src/extension.ts`, output `dist/extension.js`, library target `commonjs2`.
- `eslint.config.mjs` - TypeScript ESLint config.
- `src/extension.ts` - placeholder: registers an `axolcode.helloWorld` command.
- `src/test/extension.test.ts` - placeholder test.
- `.vscode/extensions.json` - recommends `dbaeumer.vscode-eslint`, `amodio.tsl-problem-problem-matcher`, `ms-vscode.extension-test-runner`.
- `.vscode/launch.json` - debug config that runs the extension in a new window.
- `.vscode/tasks.json` - `watch` and `watch-tests` background tasks.
- `.vscode/settings.json` - `files.exclude` for `out` and `dist`, `search.exclude` for the same.
- `.vscodeignore` - what's NOT shipped in the published extension (`.vscode/`, `src/`, `node_modules/`, `out/`, `*.ts`, `*.map`, etc.).
- `pnpm-workspace.yaml` - declares the workspace.
- `pnpm-lock.yaml` - the lockfile.

**Phase 19+ fills this in.** The extension should:
- Spawn `axol-analyzer` as a subprocess and forward LSP.
- Provide syntax highlighting via the tree-sitter grammar in `tooling/tree-sitter-axol/`.
- Provide commands: `axolcode.run`, `axolcode.build`, `axolcode.test`, `axolcode.fmt`, `axolcode.lint`, `axolcode.fix`, `axolcode.doc`, `axolcode.interpretedRun` (with `interpreted_dev_mode: true`).
- Wire the status bar to show Bucket's build status.
- Wire the editor's problems panel to Gills' diagnostics.
- Wire the editor's hover, completion, go-to-def, and rename to Gills' capabilities.

**`pnpm add` rules for `axolcode/`:** when adding a dependency, use `pnpm add <name>` or `pnpm add -D <name>`. Never edit `package.json`'s `dependencies` or `devDependencies` by hand. The CI asserts consistency with the lockfile.

#### 3.3.4 The Tree-sitter Grammar - `tooling/tree-sitter-axol/`

The grammar defines the syntax tree for any editor that uses tree-sitter (Zed, Neovim, Helix, Emacs, etc.). **Already partially scaffolded.**

**Files (current):**
- `grammar.js` - placeholder: `source_file: $ => "hello"`. **This is what `tree-sitter generate` runs against.** The full grammar (every keyword, every operator, every literal) is generated from this file.
- `tree-sitter.json` - the metadata: `name = "axolotlgrammer"`, `scope = "source.axolotlgrammer"`, `file-types = ["axol"]`, 8 bindings enabled.
- `src/parser.c` - generated by `tree-sitter generate`. Do not edit by hand.
- `src/grammar.json` - generated.
- `src/node-types.json` - generated.
- `src/tree_sitter/` - the tree-sitter C runtime headers (the standard tree-sitter include directory).
- `bindings/c/`, `bindings/go/`, `bindings/java/`, `bindings/node/`, `bindings/python/`, `bindings/rust/`, `bindings/swift/`, `bindings/zig/` - the 8 language bindings.
- `Cargo.toml` - the Rust binding crate. Name: `tree-sitter-axolotlgrammer`, edition 2024, license MIT. Depends on `tree-sitter-language = "0.1"`, dev-dep on `tree-sitter = "0.27.0"`, build-dep on `cc = "1.2"`.
- `binding.gyp` - Node.js native module build.
- `CMakeLists.txt` - C/C++ build.
- `Package.swift` - Swift Package Manager.
- `build.zig` / `build.zig.zon` - Zig binding.
- `go.mod` - Go binding.
- `pom.xml` - Java (Maven).
- `pyproject.toml` + `setup.py` - Python.
- `Makefile` - generic make build.

**Phase 0+ fills in `grammar.js`.** The grammar must cover every construct in `PROGRAMMINGLANGUAGEBIBLE.md`. After `grammar.js` is complete:

```bash
cd tooling/tree-sitter-axol
npm install -g tree-sitter-cli
tree-sitter generate
tree-sitter test
```

The `tree-sitter test` command runs the corpus tests in `test/corpus/`. The CI runs `tree-sitter test` on every change to `grammar.js`.

### 3.4 The Website

`axolsite/` is a SvelteKit 5 project using shadcn-svelte (with the full `ui/`, `magic/`, `fancy/`, `spell/` component libraries already scaffolded), Tailwind CSS 4, and Cloudflare's adapter. The site is the public face of the project:

- **Home** (`/`) - landing page using the magic components (bento grid, hero, animated gradient, etc.).
- **Docs** (`/docs/*`) - markdown viewer for the design docs in `axolotl-docs.zip`, with the `PROGRAMMINGLANGUAGEBIBLE.md` as the primary tutorial.
- **Learn** (`/learn/*`) - interactive tutorial. The user said *"write the tutorial inside the website that's a must"*. The tutorial content is the `PROGRAMMINGLANGUAGEBIBLE.md` restructured for progressive reading, with embedded runnable Axolotl snippets.
- **Benchmarks** (`/benchmarks`) - the 10 build-your-own-x ports, both Axolotl and Rust side by side, with performance numbers.
- **Playground** (`/play`) - in-browser Axolotl editor (wasm-based; v2 feature).
- **About** (`/about`) - license, owners, contributing link.

The rule from the user: **never touch `layout.css` (it is the central shadcn-svelte stylesheet), always use Tailwind classes, never write custom CSS.** This is enforced by the CI's Tailwind / shadcn-svelte lint pass.

The website scaffold ships with the full `ui/`, `magic/`, `fancy/`, `spell/` shadcn-svelte component libraries already present. The `+page.svelte` and `+layout.svelte` are placeholders. **The AI fills them in** - does not invent new components, does not touch the scaffolded components' code, does not add new CSS.

### 3.5 Cross-Cutting Concerns

- **Versioning.** Crate versions follow the workspace's `[workspace.package]` `version`. The lifecycle labels are: `0.1.0-moss` (alpha), `0.1.0-tropical` (beta), `0.1.0-moss` (release). The current scaffold is at `0.1.0-moss`. The `lush` version is the first public release.
- **Lockfile discipline.** `Cargo.lock` and `pnpm-lock.yaml` are committed. `Cargo.lock` only updates when a `Cargo.toml` dependency changes; the change is its own commit.
- **CI.** GitHub Actions workflows are in `.github/workflows/`. The seven workflows (`ci.yaml`, `test.yaml`, `benchmark.yaml`, `cd.yaml`, `release.yaml`, `security_checks.yaml`, `actions.yaml`) are **all empty in the scaffold**. Phase 23 fills them in. The CI uses the latest GitHub Actions runners (Ubuntu, macOS, Windows). The Rust toolchain is installed via `dtolnay/rust-toolchain@stable`. The pnpm version is the latest stable, installed via `pnpm/action-setup`. Node.js is the latest LTS.
- **Documentation.** The design docs in `axolotl-docs.zip` are the source of truth for the language design. The website renders them. The `bucket doc` command regenerates the API reference from the in-crate doc comments.
- **The `Makefile` is empty (0 bytes).** Phase 0 fills it with the canonical targets: `make build`, `make test`, `make fmt`, `make lint`, `make fix`, `make docs`, `make bench`, `make ci`, `make release`. Each target delegates to the appropriate `bucket` / `cargo` / `pnpm` command. The CI runs `make ci`.

---

## 4. The First Day: How to Get the Codebase

The codebase lives in two parts:

1. **The design docs** - `axolotl-docs.zip`. Read these first. They are the spec.
2. **The production codebase** - a `git` repository hosted in `<url>` (the user will provide the URL after the prompt is finalized). Download it via:

```bash
curl -X POST -L -o repo.tar.gz "<url>"
tar -xzf repo.tar.gz
cd Axolotl.rs
```

The tarball contains a directory named `Axolotl.rs/` with the workspace scaffold. The user is responsible for keeping the URL alive and re-uploading checkpoints there.

After extracting:

```bash
cd Axolotl.rs
rustup update stable
rustc --version
cargo --version
```

Then install the workspace dependencies:

```bash
# Rust workspace
cargo build -p axolc-core            # the smallest crate; quick smoke test
cargo build -p axolc                 # then the CLI

# Website
cd axolsite
pnpm install
cd ..
```

The order matters: `axolc-core` first because everything else depends on it. Then `axolc`, `bucket`, `axol-hot-runner`, `axol-analyzer` can be built in parallel.

The `crates/axolc-core/Cargo.toml` has no dependencies yet. That is intentional. Phase 0 is "add the dependencies that the design docs say we need." Section 6 lists them.

---

## 5. The Design Docs to Read

The following files in `axolotl-docs.zip` are the spec. **Read them before any code is written.**

| File | What it covers |
|---|---|
| `README.md` | Pitch, quickstart, toolchain, the iteration layer (`axol-hot-runner`). |
| `IDEA.md` | The seed, the pivots, the corrections, the final identity. |
| `Concepts.md` | Every language, semantic, ecosystem, and high-level concept. |
| `ARCHITECTURE.md` | The five candidate architectures, the chosen one, why the others were rejected. |
| `PROGRAMMINGLANGUAGEBIBLE.md` | The full language reference, in ccpprustbible style. 91 sections, every language feature documented. **This is the spec for the language surface.** |
| `HighLevel.md` | The enjoyable layer (match replacements, let-else, pipe, named args, comprehensions, etc.). |
| `LSP.md` | Gills / axol-analyzer - the rust-analyzer fork for mixed `.axol` + `.rs` projects. |
| `Interpreter.md` | axol-hot-runner - the interpreter + JIT, hot reload, Elixir-grade DX. |
| `Comparison.md` | Axolotl vs Lua, Rust, TypeScript, Nim, Zig, Mojo, Carbon. |
| `ProblemAndSolutionAndWhyAxol.md` | Every problem raised during the original design conversation, the solution, why Axolotl is the answer. |

The Bible is the language spec. The Architecture is the system spec. The Interpreter doc is the iteration-layer spec. The other docs are the supporting material.

If the design docs and this brief ever conflict, **this brief wins for project process, the design docs win for language semantics.** The two are different surfaces; a brief conflict (e.g., "use a different test framework") is resolved by the brief. A semantic conflict (e.g., "match is not exhaustive") is resolved by the design docs.

---

## 6. Phase 0 - The Test Infrastructure

Before any feature work begins, the test infrastructure is in place. Phase 0 is its own deliverable. It is small, fast, and unblocks every other phase.

### 6.1 The Test Crate Layout

Every crate gets its own `tests/` directory and its own benchmark suite:

```
crates/axolc-core/
├── src/lib.rs
└── tests/
    ├── lexer.rs
    ├── parser.rs
    ├── type_system.rs
    ├── ownership.rs
    ├── pattern_matching.rs
    ├── closures.rs
    ├── async_await.rs
    ├── traits.rs
    ├── generics.rs
    ├── ffi.rs
    ├── ...
    └── common/
        ├── mod.rs           # shared test helpers
        └── fixtures/        # .axol test files
```

`crates/axolc-core/tests/` is the largest by far - it's the language surface. The 10,000+ language tests live here, organized roughly one file per Bible section. A single test function with 5 assertions counts as 5 tests.

### 6.2 The Test Macro

Every test file uses a common pattern:

```rust
// Owner: PascalElixir / axolrs (GitHub org)
// File: Smoke tests for the lexer - every keyword, every operator, every literal kind.

use axolc_core::lexer::Lexer;

#[test]
fn lexes_let_keyword() { ... }

#[test]
fn lexes_var_keyword() { ... }

#[test]
fn lexes_function_keyword() { ... }
```

The CI runs `cargo test --workspace` and counts assertions. The worklog records the running total.

### 6.3 The Coverage Target

Phase 0 establishes the test count baseline. By the end of Phase 0:

- `axolc-core` has at least 1,000 test scaffolding files (placeholders are fine - they're the structure the later phases fill in).
- `axolc`, `axol-hot-runner`, `axol-analyzer`, `bucket` each have at least 100 test scaffolding files.
- The CI runs all of them and reports a count. The count must be non-zero (real assertions, not placeholders).

That's the bar for Phase 0.

### 6.4 The Test Helper Crate

A small crate at `crates/axolc-testkit/` provides common test helpers:

```rust
// Helper for parsing an Axolotl source snippet in a test.
pub fn parse(src: &str) -> Result<Module, Vec<Diagnostic>>;

// Helper for type-checking a parsed module.
pub fn check(module: &Module) -> Result<TypedModule, Vec<Diagnostic>>;

// Helper for lowering to Rust and running rustc.
pub fn emit_rust(module: &Module) -> String;

// Helper for running the interpreter on a snippet.
pub fn interpret(src: &str) -> Result<Value, Vec<Diagnostic>>;

// Helper for running the JIT on a snippet.
pub fn jit(src: &str) -> Result<Value, Vec<Diagnostic>>;
```

This is the same crate that the build-your-own-x ports use to verify behavior.

---

## 7. The Crate Build Order

The crates have a strict build / test order. This is not a soft suggestion; it is the order the dependencies force.

| Phase | Crate | What it adds |
|---|---|---|
| **0** | `axolc-testkit` | Test helpers. |
| **0** | `axolc-core` (skeleton) | Lexer, parser stub, type system stub, HIR stub. No real logic; just the structure. |
| **0** | `axolc` (skeleton) | CLI driver. |
| **0** | `bucket` (skeleton) | CLI driver. |
| **0** | `axol-hot-runner` (skeleton) | CLI driver. |
| **0** | `axol-analyzer` (skeleton) | LSP driver. |
| **0** | `zed-axolotl` (skeleton) | Extension manifest + a no-op extension. |
| **0** | `axolsite` | Already scaffolded; just needs a few pages filled in. |
| **1** | `axolc-core` (lexer) | Real lexer. ~5,000 tests. |
| **2** | `axolc-core` (parser) | Real parser. ~5,000 tests. |
| **3** | `axolc-core` (type system) | Real type system + inference. ~5,000 tests. |
| **4** | `axolc-core` (ownership inference) | Real ownership analysis. ~5,000 tests. |
| **5** | `axolc-core` (HIR) | Real HIR + pattern matching + closures + async. ~5,000 tests. |
| **6** | `axolc-core` (Rust codegen) | Real codegen. ~3,000 tests. |
| **7** | `axolc-core` (traits + generics) | Real trait resolution + monomorphization. ~3,000 tests. |
| **8** | `axolc-core` (FFI blocks) | Real cblock / cppblock / rblock / pyblock handling. ~2,000 tests. |
| **9** | `axolc` | Real CLI driver. ~500 tests. |
| **10** | `bucket` (manifest + dependency resolver) | Real Bucket.jsonc parser + Cargo.toml generation. ~2,000 tests. |
| **11** | `bucket` (build orchestrator + Pond cache) | Real build pipeline + global cache. ~2,000 tests. |
| **12** | `bucket` (toolchain: Shed, Neoten, Regrow) | Real formatter + linter + auto-fixer. ~2,000 tests. |
| **13** | `bucket` (Ambystoma, Salamander, Larva, Molt, Eggbox) | Real docs / test / scaffold / migrate / publish. ~1,000 tests. |
| **14** | `axol-hot-runner` (interpreter) | Real tree-walking interpreter. ~3,000 tests. |
| **15** | `axol-hot-runner` (JIT) | Real Cranelift JIT. ~2,000 tests. |
| **16** | `axol-hot-runner` (hot reload) | Real file watcher + state preservation. ~1,000 tests. |
| **17** | `axol-analyzer` (Gills LSP) | Real LSP server. ~3,000 tests. |
| **18** | `axol-analyzer` (cross-language navigation) | Real span mapping, symbol graph. ~2,000 tests. |
| **19** | `zed-axolotl` (extension) | Real Zed extension talking to Gills. ~500 tests. |
| **20** | `nvim-axolotl` | Real Neovim plugin. ~500 tests. |
| **21** | `benchmarks/` (10 build-your-own-x ports) | 10 Axolotl ports + 10 Rust references. ~5,000 tests + benchmarks. |
| **22** | `axolsite` (docs pages) | Real tutorial, real benchmark pages, real docs viewer. |
| **23** | CI hardening | All workflows run, all checks pass, all 20,000+ tests green. |
| **24** | Release prep | `bucket build --release` works on every benchmark. `axolotl.rs` renders. GitHub release artifacts built. |

The phase count is approximate; the brief can be decomposed further if a phase is too large. The order is fixed because the dependencies are real.

---

## 8. The Build, Iteration, and Checkpoint Workflow

### 8.1 The Iteration Loop

Every phase follows the same shape:

1. **Research.** Read the relevant Bible sections. Read the relevant upstream docs. `cargo search <name>` for every new dependency. Record the results in the worklog.
2. **Plan.** Before writing code, write the plan to the worklog. Which files change, which APIs touch which, what the test surface looks like, what could go wrong. The plan is a contract for the phase.
3. **Verify.** Before merging, run the smallest unit that exercises the change. `cargo test -p <crate>` or `cargo check -p <crate>`. If the change touches the website, `pnpm check`. If it touches the LSP, a manual LSP test against a real editor (or a test harness).
4. **Implement.** Now write the code. Match the existing style exactly. No new patterns, no clever tricks, no "I'll come back to this." The implementation is the *final* form.
5. **Check (long-term).** Before declaring the phase done, ask: *"Will this need to be retouched in six months?"* If yes, redesign. A "done" phase is a phase nobody has to come back to.

### 8.2 The Test Loop Within a Phase

For every feature added in a phase:

1. Write the test first. (TDD, but soft - write the test that expresses the behavior you want.)
2. Run the test. It fails.
3. Implement the feature.
4. Run the test. It passes.
5. Run the full test suite for the affected crate. Everything still green.
6. Commit. Move to the next feature.

The test count must monotonically grow (or stay equal) - never shrink. If a test is removed, document why in the worklog.

### 8.3 The Sub-Agent Strategy

Within a phase, the work is decomposed into up to four parallel sub-tasks per batch. Each sub-task has:

- A clear scope (one crate, one feature surface, one test area).
- A clear deliverable (files added/modified, tests added, build target that compiles, brief summary).
- A clear "do not touch" list (the other three sub-tasks' surfaces, the build files unless explicitly approved, the `axolc-core` types unless the task is "modify `axolc-core`").
- A reporting requirement (return a short structured report: scope, deliverable, build target verified, tests added, issues deferred).

A phase may need more than one batch of four sub-agents. The cadence is: launch four → wait for all four to return → review the four reports → launch the next four (or fewer) → repeat until the phase's checklist is complete.

### 8.4 Resume Protocol

If the session is resumed (because the previous session timed out or restarted), the first action is:

1. Read the most recent `WORKLOG.md` entry to identify the last completed phase and the current open phase.
2. Download the most recent `temp.sh` checkpoint, untar into the workspace.
3. Verify that the resumed state matches the worklog's "phase started from" line.
4. Resume the open phase from where the worklog says the work left off.

The worklog is the contract. The tarball is the artifact. Together they make a session restart cheap.

---

## 9. The Bucket.jsonc Format

The manifest format is `Bucket.jsonc` - JSONC (JSON with comments). Every project's manifest uses it. The current examples (in `Interpreter.md`, in the design docs) show the format. The full reference:

```jsonc
{
    "name": "my-game",
    "version": "0.1.0",

    "language": {
        "edition": "2026"        // Axolotl language edition (the year marker, not the Rust edition)
    },

    "interpreted_dev_mode": true,  // opt into interpreter / JIT in `bucket run`

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
        "wgpu":   "^27",          // crates.io crates, not Axolotl packages
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
        "inlay_hints": { ... },
        "completion": { ... },
        "diagnostics": { ... }
    }
}
```

`bucket` reads this, generates the corresponding `Cargo.toml` for Cargo, and uses the `scripts` map to resolve `bucket run <name>` to the user's command.

The CI asserts that every `Bucket.jsonc` parses, that `interpreted_dev_mode` is a boolean, that every `scripts` value is a string, and that the dependencies exist on crates.io (verified via `cargo search`).

---

## 10. The Toolchain (the names matter)

| Tool | Crate / Subcommand | Job |
|---|---|---|
| **`axolc`** | `crates/axolc/` | The Axolotl → Rust compiler. |
| **`axolc-core`** | `crates/axolc-core/` | The compiler library, reused by `axolc`, `axol-hot-runner`, `axol-analyzer`. |
| **`axol-hot-runner`** | `crates/axol-hot-runner/` | The interpreter + JIT. The iteration layer. |
| **`axol-analyzer`** (Gills) | `crates/axol-analyzer/` | The LSP. Fork of rust-analyzer. |
| **`bucket`** | `crates/bucket/` | The build orchestrator. |
| **Gills** | = `axol-analyzer` | The LSP / IDE intelligence. |
| **Neoten** | `bucket lint` | Linter + static analysis. |
| **Shed** | `bucket fmt` | Formatter. |
| **Regrow** | `bucket fix` | Auto-fix + refactoring. |
| **Ambystoma** | `bucket doc` | Documentation generator. |
| **Salamander** | `bucket test` / `bucket bench` | Test + benchmark runner. |
| **Larva** | `bucket new` | Project scaffolding. |
| **Molt** | `bucket upgrade` | Toolchain / project migration. |
| **Eggbox** | `bucket publish` | Package publishing (to crates.io, primarily). |
| **Pond** | `bucket.jsonc` `pond` field, `~/.bucket/pond/`, `project/pond/` | Build / output / cache directory. |
| **Mud** | the AOT native binary that the Axolotl project compiles to | The final compiled artifact. (Optional, but cute.) |

The names are biological - axolotl, gills, neoteny, shed, regrow, ambystoma, salamander, larva, molt, eggbox, pond, mud. The naming is not decoration; it makes the toolchain feel like one ecosystem.

---

## 11. The Editor Integrations

**The scaffold ships THREE editor integrations.** All three talk to Gills (`axol-analyzer`) over the standard LSP. All three are wired to the same tree-sitter grammar. The first hour of editor work is to fill in the placeholders, not to invent a fourth editor.

### 11.1 The Zed Extension - `editors/zed-axolotl/`

A Rust crate with `crate-type = ["cdylib"]`. It depends on `zed_extension_api = "0.7.0"` (or the latest available). It is **not** part of the workspace because it requires the `wasm32-wasip2` target.

The extension:

- Spawns `axol-analyzer` as a subprocess.
- Forwards LSP requests/responses.
- Provides Axolotl-specific commands (e.g., "show inferred ownership", "show generated Rust", "apply Regrow fix").
- Renders the Pond cache status, the current test status, the build status.
- Provides syntax highlighting via the tree-sitter grammar in `tooling/tree-sitter-axol/`.

The current scaffold has `Cargo.toml` (with only `name`, `version.workspace`, `edition.workspace`, `license.workspace`) and `extension.toml` (the Zed manifest). The `src/` is empty. **Phase 19 fills in the implementation.** See `ZED_TOOLCHAIN.md` for the full toolchain setup.

### 11.2 The Neovim Plugin - `editors/nvim-axolotl/`

A Lua plugin. **Already partially scaffolded** - `ftdetect/axol.lua` and `lua/axolotl/init.lua` exist and are functional.

The current `init.lua` does:
- `vim.lsp.config("axol_analyzer", { cmd = opts.cmd or { "axol-analyzer" }, ... })` - configures the LSP.
- `vim.lsp.enable("axol_analyzer")` - enables it.
- Registers the `axol` tree-sitter parser.

What Phase 20 must add:
- A `:Axolotl` command namespace.
- `:Axolotl run` to invoke `bucket run`.
- `:Axolotl build` to invoke `bucket build --release`.
- `:Axolotl test` to invoke `bucket test`.
- `:Axolotl fmt`, `:Axolotl lint`, `:Axolotl fix`.
- `:Axolotl gills` to show the LSP status.
- `:Axolotl interpret` to drop into the interpreter (uses `interpreted_dev_mode: true` from Bucket.jsonc).
- `:Axolotl gill` to show the inferred type / ownership at the cursor (calls Gills' `textDocument/hover`).
- Setup defaults: `inlay_hints = { enabled = true, hide_guts = false }`, `tree_sitter_highlight = { enabled = true }`, `format_on_save = { enabled = true, tool = "Salamander" }`, `on_save = { run = false }` (user opts in to run-on-save explicitly).
- A status line integration that shows the build status (Gills computes it from the project manifest).

### 11.3 The VS Code Extension - `editors/axolcode/`

**A first-class editor integration. Already in the scaffold.** A TypeScript + Webpack VS Code extension. Package name `axolcode`, display name `AxolCode`, VS Code engine `^1.136.0`. Built with `webpack` (output `dist/extension.js`), linted with `eslint` + `typescript-eslint`, tested with `vscode-test` + `mocha`.

**The full file layout of the scaffold (every file named):**
- `package.json` - extension manifest. Currently defines one command: `axolcode.helloWorld`. Scripts: `compile`, `watch`, `package`, `compile-tests`, `watch-tests`, `pretest`, `lint`, `test`.
- `tsconfig.json` - `module: Node16`, `target: ES2022`, `lib: [ES2022]`, `strict: true`.
- `webpack.config.js` - `target: node`, `entry: ./src/extension.ts`, `output: dist/extension.js`, `libraryTarget: commonjs2`, `externals.vscode: "commonjs vscode"`.
- `eslint.config.mjs` - TypeScript ESLint flat config.
- `src/extension.ts` - placeholder. Exports `activate(context)` (currently logs "Congratulations, your extension AxolCode is now active!" and registers the `helloWorld` command) and `deactivate()`.
- `src/test/extension.test.ts` - placeholder Mocha test (`suite('Extension Test Suite', ...)`).
- `.vscode/extensions.json` - recommends `dbaeumer.vscode-eslint`, `amodio.tsl-problem-matcher`, `ms-vscode.extension-test-runner`.
- `.vscode/launch.json` - debug config: `Run Extension` launches a new window with `--extensionDevelopmentPath=${workspaceFolder}`.
- `.vscode/settings.json` - `files.exclude` for `out` and `dist`, `search.exclude` for the same, `js/ts.tsc.autoDetect: off`.
- `.vscode/tasks.json` - `watch` and `watch-tests` background tasks (problem matchers `$ts-webpack-watch` and `$tsc-watch`).
- `.vscodeignore` - what is NOT published: `.vscode/`, `node_modules/`, `src/`, `out/`, `*.ts`, `*.map`, `tsconfig.json`, `eslint.config.mjs`, `.vscode-test.*`, `webpack.config.js`, `vsc-extension-quickstart.md`.
- `.vscode-test.mjs` - the vscode-test config.
- `pnpm-workspace.yaml` - declares the workspace.
- `pnpm-lock.yaml` - the lockfile (gitignored, regenerated on `pnpm install`).
- `CHANGELOG.md` - starts empty, every release appends.
- `README.md` - starts empty, the marketplace listing reads it.
- `vsc-extension-quickstart.md` - the standard VS Code extension quickstart (informational, can be deleted or repurposed).

**Phase 19 fills in the implementation.** The extension must:
- Provide commands: `axolcode.run`, `axolcode.build`, `axolcode.test`, `axolcode.fmt`, `axolcode.lint`, `axolcode.fix`, `axolcode.doc`, `axolcode.interpretedRun`, `axolcode.interpretedREPL`, `axolcode.buildStatus`.
- Spawn `axol-analyzer` as a child process, forward LSP via the standard `vscode-languageclient` library.
- Wire the tree-sitter grammar for syntax highlighting (use `vscode-tree-sitter` or the npm-published `tree-sitter-axolotl`).
- Wire Gills' diagnostics to the editor's problems panel.
- Wire Gills' hover, completion, go-to-def, rename, references, signature help, inlay hints, code actions, semantic tokens, document symbols to the editor.
- Add a status bar item showing the Bucket build state (synced via LSP `window/workDoneProgress/create`).
- Add a sidebar view for the test results (synced via Gills' test discovery).

**`pnpm` rules for `axolcode/`:**
- Add deps with `pnpm add <name>` or `pnpm add -D <name>`. Never edit `package.json`'s `dependencies` or `devDependencies` by hand.
- Run `pnpm install` to regenerate the lockfile.
- Run `pnpm run watch` to develop.
- Run `pnpm run package` to produce the `.vsix`.
- Run `pnpm run test` to run the test suite.

The CI asserts that the lockfile is up-to-date (`pnpm install --frozen-lockfile`).

### 11.4 The Tree-sitter Grammar - `tooling/tree-sitter-axol/`

All three editors (Zed, Neovim, VS Code) use the same tree-sitter grammar for syntax highlighting, code folding, structure analysis, and more. **The grammar is the only editor-shared piece of code.** Phase 0+ fills in `grammar.js`. The current `grammar.js` is `source_file: $ => "hello"` - a placeholder.

**Bindings shipped in the scaffold (8 languages):** c, go, java, node, python, rust, swift, zig. Each binding lets the grammar be used in that language's editor ecosystem.

After `grammar.js` is complete, run:

```bash
cd tooling/tree-sitter-axol
tree-sitter generate   # regenerates src/parser.c, src/grammar.json, src/node-types.json
tree-sitter test       # runs the corpus tests
```

The CI runs `tree-sitter test` on every change to `grammar.js` and fails if the corpus regresses.

### 11.5 Other Editors (Helix, Emacs, Sublime, etc.)

Gills speaks standard LSP. The tree-sitter grammar in `tooling/tree-sitter-axol/` is publishable to the registries of every editor that supports tree-sitter. Adding a new editor is **not** a goal in the moss/tropical versions - the three first-class integrations are the focus.

---

## 12. The 10 build-your-own-x Ports

The user said: *"from that repo 10 different project write them in axol and put them in bench and examples directory ... and you have to write same projects in rust as well and benchmark we should have no overhead for axol at all and more"*.

A starter list of 10 projects from `https://github.com/codecrafters-io/build-your-own-x`:

| # | Project | Why this one |
|---|---|---|
| 1 | **Git** | Version control is hard, exercises parsing, hashing, object storage, network protocols, and a CLI. A good test of Axolotl's string handling, file I/O, async, and serialization. |
| 2 | **Database** | A simple key-value store. Exercises Axolotl's async, FFI (for the storage engine), error handling, and Option/Result ergonomics. |
| 3 | **Redis** | Network protocol parsing, in-memory data structures, async. Tests Axolotl's `Vec`, `Map`, `String`, `Result`, and pattern matching. |
| 4 | **Shell** | A POSIX shell. Exercises Axolotl's string parsing, process spawning (FFI), pipelines, signals. A great test of error handling and the FFI surface. |
| 5 | **Web Server** | An HTTP/1.1 server. Exercises async, networking, the foreign-block machinery (for `sendfile` etc.), and the standard library's `TcpListener`. |
| 6 | **Text Editor** | A vim-like editor. Exercises terminal I/O, key bindings, file editing, undo/redo. Tests Axolotl's collection ergonomics. |
| 7 | **Programming Language** | A tree-walking interpreter for a small Lisp-like language. Exercises Axolotl's own surface - if Axolotl can write a language implementation ergonomically, the language works. |
| 8 | **Regex Engine** | A backtracking / NFA / DFA regex engine. Exercises Axolotl's pattern matching, sum types, recursion, and tail calls. |
| 9 | **Docker** | A minimal container runtime using Linux namespaces and cgroups. Exercises Axolotl's unsafe block, FFI to libc, file I/O, and process management. |
| 10 | **Emulator / Virtual Machine** | A CHIP-8 or similar VM. Exercises Axolotl's `u8` / `u16` / `u32` / `u64` types, bitwise operators, the unsafe block, and the FFI surface. |

The user can substitute any of these with another from the `build-your-own-x` list (e.g., a 3D renderer, a game, a physics engine, a neural network, a blockchain). The rule is: 10 projects, Axolotl + Rust side by side, zero-overhead benchmark.

### 12.1 The Layout for Each Port

```
benchmarks/
├── git/
│   ├── README.md                # the spec, the implementation notes, the benchmark
│   ├── rust/                    # the hand-written reference
│   │   ├── Cargo.toml
│   │   ├── src/
│   │   └── tests/
│   ├── axol/                    # the Axolotl port
│   │   ├── Bucket.jsonc
│   │   ├── src/
│   │   └── tests/
│   └── bench/
│       ├── run.rs               # runs both, asserts the Axolotl version is within 5% of the Rust version
│       └── results/             # bench output history
```

`bench/run.rs` runs both versions on the same workload (e.g., for `git`, the workload is "create 10,000 commits, pack, then unpack and verify"), records the timings, and fails the CI if the Axolotl version is more than 5% slower.

### 12.2 The Performance Acceptance Criterion

For every build-your-own-x port:

```
axol_time / rust_time ≤ 1.05
```

The 5% tolerance is well within LLVM's own variance. The CI asserts this on every commit.

The rule is not "the Axolotl version is fast." The rule is "the Axolotl version is **as fast as** the hand-written Rust version." If the generated Rust isn't idiomatic, the Axolotl port is wrong. Fix the Axolotl port. Do not loosen the criterion.

---

## 13. The Documentation Site (`axolsite`)

The user said:

> *"about website if you look at programming language bible axolotl as well others i put in the tarball write the tutorial inside the website that's a must"*

> *"in the axolsite i put there's all shadcn components as well magic and other beautiful components utilize them"*

> *"never touch layout css its a must u can never touch that and always use tailwind classes no never ever custom css"*

> *"the website should be complete with few pages but mostly a good markdown viewer"*

> *"u have to use pnpm install no manual package entry"*

So:

### 13.1 The Pages

| Route | Content |
|---|---|
| `/` | Landing page using the `magic/` components (bento grid, hero, animated gradient, etc.). |
| `/docs` | Index of the design docs. |
| `/docs/[slug]` | A single doc (e.g., `/docs/bible` for the Bible). Renders the markdown from `axolotl-docs.zip`. |
| `/learn` | The interactive tutorial. The first half is a sequential read of the Bible restructured for learning; the second half is runnable Axolotl snippets (via the in-browser editor). |
| `/benchmarks` | The 10 build-your-own-x ports, with the Axolotl and Rust sources side by side and the benchmark results. |
| `/play` | The in-browser Axolotl editor (v2; v1 can be a "coming soon" page using the magic components). |
| `/about` | License, owners, contributing, code of conduct. |

### 13.2 The Markdown Viewer

The user said: *"mostly a good markdown viewer ofc"*. The site's `/docs/[slug]` route is a markdown viewer. The implementation:

- A Svelte 5 component that takes a markdown string and renders it.
- Uses `marked` (or `mdsvex`, or a similar library - pick the latest via `pnpm view <name> versions`) for parsing.
- Uses `shadcn-svelte`'s typography component for styling.
- Supports code blocks with syntax highlighting (via `shiki` or `highlight.js`, latest via `pnpm view`).
- Supports table of contents generation from headings.
- Supports light / dark mode (via the existing `mode-watcher` dependency).

### 13.3 The Layout CSS Rule

The user said: *"never touch layout css its a must u can never touch that"*.

`axolsite/src/routes/layout.css` is the central shadcn-svelte stylesheet. **Do not edit it.** Do not add CSS rules to it. Do not move it. Do not delete it. The CI asserts that this file is unmodified after Phase 0.

All styling happens via Tailwind classes on the Svelte components. The CI lints the codebase for any custom CSS files (anything outside `node_modules/`, `src/routes/layout.css`, and the shadcn-svelte component files) and fails if it finds any.

### 13.4 The pnpm Install Rule

The user said: *"u have to use pnpm install no manual package entry in anything that's a hard requirement"*.

When a new dependency is needed in `axolsite/`:

```bash
pnpm add <package>          # for runtime deps
pnpm add -D <package>       # for dev deps
```

Never edit `axolsite/package.json`'s `dependencies` or `devDependencies` block by hand. The CI asserts that the file is consistent with the lockfile.

---

## 14. Code Style

### 14.1 The File Header

Per R3, every source file in the workspace starts with the owner + description header. The `axolsite` Svelte files use a Svelte-style comment at the top of the `<script lang="ts">` block. The Rust files use `//` comments.

### 14.2 The Function Summary

Per R4, every function gets a one-line summary directly above its signature. The summary describes what the function does and what it is used for.

### 14.3 Naming and Style

- **Rust:** official Rust style guide. `snake_case` for functions / variables / modules, `PascalCase` for types / traits, `SCREAMING_SNAKE_CASE` for `const`. 4-space indent, no tabs. Trailing commas in multiline argument lists.
- **Svelte / TypeScript:** official Svelte 5 style. `PascalCase.svelte` for components, `camelCase` for props and stores, `SCREAMING_SNAKE_CASE` for env-derived constants. 2-space indent.
- **CSS:** Tailwind utility classes only. No `<style>` blocks in Svelte files except for dynamic values that Tailwind cannot express. No custom CSS files outside `src/routes/layout.css`.

### 14.4 Imports and Dependencies

- Every Rust module's `Cargo.toml` declares its dependencies explicitly. No `*` re-exports.
- Every Svelte/TS module's dependencies come from `pnpm add`. The version is in `pnpm-lock.yaml`.
- Adding a new dependency is a deliberate act, recorded in the worklog (R21).

---

## 15. The License Posture

| Component | License |
|---|---|
| All Rust crates | MIT OR Apache-2.0 |
| The website (`axolsite/`) | MIT OR Apache-2.0 |
| The docs in `docs/` | MIT OR Apache-2.0 |
| The build-your-own-x ports in `benchmarks/` | MIT OR Apache-2.0 |
| The examples in `examples/` | MIT OR Apache-2.0 |
| The tree-sitter grammar in `tooling/tree-sitter-axol/` | MIT OR Apache-2.0 |

There is no "premium" or "enterprise" tier. There is no separate "Pro" build. There is one release, one license, one repo. The user picked MIT preferred, Apache-2.0 also acceptable; we ship both.

### 15.1 The LICENSE File

The root `LICENSE` file is the full MIT license text. The Cargo workspace declares `license = "MIT OR Apache-2.0"`. The README has a one-line license badge. The website's `/about` page has the same.

### 15.2 The NOTICE File

The root `NOTICE` file lists upstream dependencies and any attribution they require. The user follows the [REUSE](https://reuse.software/) specification.

---

## 16. CI / GitHub Actions

The `.github/workflows/` directory contains:

| File | Purpose |
|---|---|
| `ci.yaml` | Runs `cargo check --workspace`, `cargo test --workspace`, `cargo clippy --workspace -- -D warnings`, `pnpm install`, `pnpm check`, `pnpm build` on every push and PR. |
| `release.yaml` | Builds release artifacts on tagged commits. Uploads to GitHub Releases. |
| `test.yaml` | Runs the long-form test suite (the 20,000+ tests) on a schedule. |
| `benchmark.yaml` | Runs the build-your-own-x benchmark suite. Fails the CI if any port is more than 5% slower than the Rust reference. |
| `cd.yaml` | Deploys the website to Cloudflare on every push to the release branch. |
| `actions.yaml` | Reusable actions for the other workflows. |
| `security_checks.yaml` | Runs `cargo audit`, `cargo deny`, and a license-check pass. |

The CI uses the latest GitHub Actions runners (Ubuntu, macOS, Windows). The Rust toolchain is installed via `dtolnay/rust-toolchain@stable`. The pnpm version is the latest stable, installed via `pnpm/action-setup`.

---

## 17. The Tooling Versions

When this brief is read (2026), the latest stable versions are:

- **Rust toolchain:** whatever `rustup update stable` returns today. The CI installs the current stable. The `Cargo.toml` edition is **2024** (Rust edition, not year).
- **Cargo:** whatever ships with the current stable Rust.
- **Node.js:** the latest LTS. The CI installs the current LTS.
- **pnpm:** the latest stable. The CI installs the current stable.
- **Svelte:** 5.x.
- **SvelteKit:** 2.x.
- **Tailwind:** 4.x.
- **shadcn-svelte:** 1.x.
- **TypeScript:** whatever ships with the current SvelteKit.

**Never** pin to a specific version by hand. Use `pnpm add <name>` and `cargo add <name>` (or `cargo search` for non-interactive use). The lockfile is the source of truth for versions.

---

## 18. The Test Plan (the 20,000+ tests)

The test plan distributes the 20,000 tests roughly as:

### 18.1 The Language Itself (10,000+ tests)

Drawn from the 91 sections of `PROGRAMMINGLANGUAGEBIBLE.md`. Each Bible section gets a test file in `crates/axolc-core/tests/`. Each test file has at least 100 test functions on average, with 1-10 assertions per test. The break-down:

| Bible section category | Approx. test count |
|---|---|
| Lexer / keywords / operators | 1,000 |
| Types / inference / annotations | 1,000 |
| Variables / `let` / `var` | 200 |
| Control flow / `if` / `while` / `for` / `match` / `case` | 1,000 |
| Functions / closures / lambdas | 500 |
| Structs / enums / interfaces / methods | 1,000 |
| Generics / traits / type aliases | 500 |
| Ownership / borrowing / moves / lifetimes | 1,000 |
| Async / await / tasks / spawn | 500 |
| FFI blocks (cblock / cppblock / rblock / pyblock) | 500 |
| Error handling (`?`, `??`, `try`/`catch`, `Result`) | 500 |
| Pattern matching / `case` / variant handlers / dispatch tables | 500 |
| Comprehensions / `\|>` / cascading / destructuring | 500 |
| Strings / number suffixes / doc comments / inline tests | 300 |
| Auto-derive / constructors / struct update | 200 |
| Attribute / derive / `@reflect` | 300 |
| (various) | 700 |
| **Total** | **~10,000** |

### 18.2 The Toolchain, Compiler, Interpreter, JIT, LSP, Editor Integration (10,000+ tests)

| Component | Approx. test count |
|---|---|
| `axolc-core` (HIR / type system / borrow checker / codegen) | 4,000 |
| `axolc` (CLI / argument parsing / file discovery) | 500 |
| `axol-hot-runner` (interpreter) | 3,000 |
| `axol-hot-runner` (JIT) | 1,000 |
| `axol-hot-runner` (hot reload) | 500 |
| `bucket` (manifest / dependency resolver / Pond cache) | 2,000 |
| `bucket` (formatter / linter / auto-fixer) | 1,000 |
| `bucket` (docs / test / scaffold / migrate / publish) | 500 |
| `axol-analyzer` (LSP / Gills) | 2,000 |
| `zed-axolotl` | 500 |
| **Total** | **~15,000** |

The full 20,000+ count is achieved by the end of the project. Phases track progress.

### 18.3 The Test Count Is Reported

Every CI run reports:

```
test count: 21847 (was 21730; +117 this commit)
```

The worklog tracks the running count. The count must monotonically grow or stay equal - never shrink. If a test is removed, the worklog records why.

---

## 19. The Phase Plan (a Reasonable Decomposition)

The brief is broken into ~25 phases, each with a clear deliverable, a test surface, and a checkpoint. Sub-agents work in batches of four per phase.

| Phase | Title | Deliverable |
|---|---|---|
| 0 | Test infrastructure | `axolc-testkit` + 1,000+ test scaffolding files + CI reporting. |
| 1 | Lexer | Real lexer in `axolc-core`. 5,000+ tests pass. |
| 2 | Parser | Real parser. 5,000+ tests pass. |
| 3 | Type system | Real type checker + inference. 5,000+ tests pass. |
| 4 | Ownership inference | Real ownership analysis. 5,000+ tests pass. |
| 5 | HIR + pattern matching | Real HIR + match + variant handlers + dispatch tables. 5,000+ tests pass. |
| 6 | Rust codegen | Real codegen to idiomatic Rust. 3,000+ tests pass. |
| 7 | Traits + generics | Real trait resolution + monomorphization. 3,000+ tests pass. |
| 8 | FFI blocks | Real cblock / cppblock / rblock / pyblock. 2,000+ tests pass. |
| 9 | `axolc` CLI | Real CLI driver. 500+ tests pass. |
| 10 | `bucket` manifest + resolver | Real Bucket.jsonc + Cargo.toml generation. 2,000+ tests pass. |
| 11 | `bucket` build + Pond | Real build pipeline + global cache. 2,000+ tests pass. |
| 12 | `bucket` Shed / Neoten / Regrow | Real formatter / linter / auto-fixer. 2,000+ tests pass. |
| 13 | `bucket` Ambystoma / Salamander / Larva / Molt / Eggbox | Real docs / test / scaffold / migrate / publish. 1,000+ tests pass. |
| 14 | `axol-hot-runner` interpreter | Real tree-walking interpreter. 3,000+ tests pass. |
| 15 | `axol-hot-runner` JIT | Real Cranelift JIT. 2,000+ tests pass. |
| 16 | `axol-hot-runner` hot reload | Real file watcher + state preservation. 1,000+ tests pass. |
| 17 | `axol-analyzer` (Gills) LSP | Real LSP server. 3,000+ tests pass. |
| 18 | `axol-analyzer` cross-language nav | Real span mapping + symbol graph. 2,000+ tests pass. |
| 19 | `zed-axolotl` | Real Zed extension. 500+ tests pass. |
| 19.5 | `axolcode` (VS Code) | Real VS Code / Cursor extension. 500+ tests pass. |
| 20 | `nvim-axolotl` | Real Neovim plugin. 500+ tests pass. |
| 21 | build-your-own-x ports | 10 Axolotl + 10 Rust ports. 5,000+ tests + benchmarks all green. |
| 22 | `axolsite` | Real tutorial + docs viewer + benchmark pages. |
| 23 | CI hardening | All workflows run, all checks pass, all 20,000+ tests green. |
| 24 | Release prep | `bucket build --release` works on every benchmark. `axolotl.rs` renders. GitHub release artifacts built. |

If a phase is too large for one batch of four sub-agents, it is decomposed further. The phase count is approximate; the order is fixed.

---

## 20. The Resume Protocol (the worklog contract)

`WORKLOG.md` at the repository root is the source of truth for the project's state across sessions. Every phase ends with an entry. The entry format:

```markdown
## Phase N - Title (YYYY-MM-DD)

**Status:** complete | in-progress | blocked
**Test count:** N (was N-1; +delta)
**Checkpoint:** https://temp.sh/XXXXX, https://x0.at/XXXXX

### What changed
- file 1: description
- file 2: description
- ...

### What was verified
- `cargo test -p <crate>`: N tests pass, 0 fail
- `cargo check -p <crate>`: clean
- `pnpm check` (if website): clean
- Manual LSP test: description

### What is known broken
- Issue 1: description, deferred to phase N+M
- Issue 2: description

### Decisions made
- Decision 1: rationale
- Decision 2: rationale

### Next phase
Phase N+1 starts from this state. Next phase focus: ...
```

The worklog is the contract. The next session reads the last entry, downloads the last checkpoint, verifies the state, and resumes.

---

## 21. The "Done" Definition

The project is done when **every** of the following holds:

1. `cargo test --workspace` reports **20,000+ passing tests, 0 failures, 0 ignored**.
2. `bucket build --release` produces a native binary for **every** build-your-own-x port in `benchmarks/`.
3. The benchmark suite asserts that **every** Axolotl port is **within 5%** of its Rust reference.
4. `bucket run dev` (with `interpreted_dev_mode: true`) hot-reloads in **sub-200ms** on a 5,000-line project with 50 dependencies.
5. `bucket build --release` produces a binary that **does not** link the interpreter or JIT (the "no runtime" invariant in production).
6. The Zed extension installs, talks to Gills, and provides Axolotl completions / hovers / diagnostics / code actions.
7. The Neovim plugin installs, talks to Gills, and provides Axolotl completions / hovers / diagnostics.
8. The website at `axolsite/` builds (`pnpm build`) and renders the docs viewer, the tutorial, and the benchmark pages.
9. The CI runs **all** of the workflows on every push and they all pass.
10. The LICENSE, CONTRIBUTING, and CODE_OF_CONDUCT files are at the root and correct.
11. The README at the root links to the docs, the website, and the install instructions.
12. The `0.1.0-moss` GitHub release is cut with binary artifacts for Linux, macOS, and Windows.

When all 12 hold, the project is done. Not before.

---

## 22. The Anti-Patterns (the things that will make this fail)

The brief has been deliberately long because the failure modes are known:

- **Skipping the design docs.** The Bible is the spec. Reading it once and skimming later produces wrong implementations. Read the Bible end to end before writing any code.
- **Guessing dependency versions.** Every dependency is added via `cargo search` / `pnpm view`. Never a hand-pinned version. The "no random never random guessed version ever" rule.
- **Running `cargo build --workspace` as a smoke test.** It takes 5-30 minutes. Use `cargo check -p <crate>` instead.
- **Editing `layout.css` in the website.** The CI asserts it's unmodified. The user said: *"never touch layout css its a must u can never touch that"*. Touch it and the build fails.
- **Writing custom CSS anywhere in the website.** Tailwind classes only. The user said: *"always use tailwind classes no never ever custom css"*. Custom CSS and the build fails.
- **Manually editing `package.json` dependencies.** `pnpm add` only. The user said: *"u have to use pnpm install no manual package entry in anything"*. Manual edits and the build fails.
- **Forgetting the file header convention.** Every Rust file has the owner + description. Every Svelte file has the same. Every function has a one-line summary. The CI enforces.
- **Adding comments inside function bodies.** No. The CI enforces. The summary is above the signature, not inline.
- **Forgetting the worklog.** Every phase ends with a `WORKLOG.md` entry. Every checkpoint is uploaded to two services. The worklog is the contract.
- **Sub-agent drift.** Sub-agents receive this brief in full (or the relevant section). They don't write to surfaces outside their scope. They report. The orchestrator reviews.
- **Scope creep.** "Let's also add ..." is not in this brief. The brief is the spec. If a sub-agent suggests a feature, it goes into the worklog as a proposal - not into the code.
- **Half-done work.** The user said: *"i swear i want it 100% done"*. Every phase's deliverable is complete before the next phase starts. No "// TODO: finish this" comments, no placeholder functions, no skipped tests.
- **Premature optimization.** The interpreter is fast enough when it's tree-walking. The JIT is fast enough when it kicks in. The AOT build is fast enough when it runs. Optimize only when a benchmark says so.

---

## 23. The First Hour

The first hour of the first session, before any code is written:

1. Read the design docs in `axolotl-docs.zip`. End to end. The Bible is the spec. The Architecture is the system spec. The Interpreter doc is the iteration layer. The other docs are supporting.
2. Extract the production codebase from `<url>` (the user provides this URL). The tarball contains the scaffold at `Axolotl.rs/`.
3. `cd Axolotl.rs && rustup update stable && rustc --version && cargo --version`.
4. `cd axolsite && pnpm install && cd ..`.
5. `cargo check -p axolc-core`. Confirm the empty skeleton builds. Confirm the empty tests pass.
6. Open `WORKLOG.md` (create it if it doesn't exist). Write the first entry: *"Session 1, Phase 0 starting. Codebase extracted, toolchain installed, scaffold verified."*
7. Decompose Phase 0 into four sub-agent tasks. Launch them.
8. Wait. When they return, review the reports, commit the work, produce the first checkpoint, write the next worklog entry.

That's the first hour. Everything else follows from it.

---

## 24. The Last Hour

The last hour, when every "done" condition is met:

1. Cut the `0.1.0-moss` git tag.
2. Run `bucket build --release` on every benchmark port. Verify the binaries.
3. Run `pnpm build` in `axolsite/`. Verify the build.
4. Run the full CI. Verify all workflows pass.
5. Upload the release artifacts to GitHub Releases.
6. Update `axolotl.rs` to point at the new release.
7. Write the final `WORKLOG.md` entry: *"0.1.0-moss released. All 20,000+ tests green. All 10 build-your-own-x ports within 5% of Rust. axolotl.rs updated."*

That's the last hour. The project is done.

---

## 25. The Owner Speaks

The user said a lot. The essential points, distilled:

- **No Axolotl runtime. Ever.** The compiler is a frontend. Cargo + rustc + LLVM are the engine. The interpreter is opt-in for dev iteration.
- **Lua-flavored surface, Rust-grade semantics.** The Bible is the spec.
- **Cargo compatibility from day one.** Every crates.io crate is an Axolotl library. No parallel ecosystem.
- **Smart compiler, smart IDE.** Gills knows both languages. The user gets cross-language completion, navigation, diagnostics.
- **Elixir-grade iteration.** `axol-hot-runner` makes the dev loop sub-200ms.
- **20,000+ tests, 10 build-your-own-x ports.** Real coverage. Real benchmarks. Zero overhead.
- **Open source, free, MIT OR Apache-2.0.** Forever.
- **Workflow: research → plan → verify → implement → check (long-term).** Nobody has to retouch finished work.

That is the project. The brief is the spec. Execute.

- PascalElixir / axolrs
