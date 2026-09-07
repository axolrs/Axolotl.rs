# LSP - Gills / axol-analyzer

> **Gills** (the **axol-analyzer**) is Axolotl's language server. It is a **fork of rust-analyzer** that understands both `.axol` and `.rs` files in the same project, provides cross-language navigation, and ships first-class IDE support for the entire Axolotl ecosystem.

The name fits: axolotls breathe through external gills, and **Gills is how the editor breathes the project** - autocomplete, hover, go-to-definition, references, rename, refactor, inline diagnostics, code actions, type info, and intelligent suggestions, all unified across both languages.

---

## Table of Contents

1. The problem rust-analyzer can't solve
2. The Gills architecture (a rust-analyzer fork, not a wrapper)
3. The two-language symbol graph
4. Cross-language navigation
5. The diagnostic pipeline
6. The completion engine
7. The refactoring engine
8. The inlay hints system
9. The in-editor TUI
10. The build / workspace model
11. Editor integrations
12. The `bucket gills` CLI
13. Performance
14. The Bucket.jsonc LSP section
15. Gills + Neoten + Regrow collaboration
16. Future extensions (AI hooks, debugger hooks)

---

## 1. The problem rust-analyzer can't solve

A typical mixed project:

```text
my-game/
├── bucket.jsonc
├── Cargo.toml
└── src/
    ├── main.axol
    ├── server.axol
    ├── database.axol
    ├── crypto.rs        -- pure Rust
    ├── protocol.rs      -- pure Rust
    └── hardware.rs      -- pure Rust
```

In `main.axol`:
```axol
use "crypto"
use "protocol"
use "server"
```

In `crypto.rs`:
```rust
use crate::server::Server;
use crate::database::Database;

pub fn encrypt(data: &[u8], key: &[u8; 32]) -> Vec<u8> { ... }
```

### What rust-analyzer sees

Cargo's CLI doesn't know `.axol` exists. Bucket compiles `.axol` to Rust, and the generated `.rs` lives in `pond/generated/rust/`. The Cargo build tells rust-analyzer about the crate; rust-analyzer indexes the **generated** Rust source, plus the user's `.rs` files.

This works for **completion and navigation within `.rs` files**. It breaks in three important ways:

1. **`.rs` files can't see symbols defined in `.axol`.** When the user writes `use crate::server::Server;` in `crypto.rs`, rust-analyzer **does** find `Server` in the generated Rust. But it has no idea the user originally wrote it in `main.axol`. The "go to definition" jumps to `pond/generated/rust/server.rs:42` instead of `src/server.axol:14`. The user is confused.

2. **`.axol` files get no IDE support at all.** No completion, no hover, no go-to-def, no inline types, no error squiggles. The editor treats `.axol` as a plain text file with the Axolotl TextMate grammar (if the user installed one).

3. **Mixed-language refactors are impossible.** Renaming `Server` in `server.axol` doesn't update references in `crypto.rs`. Moving a function from `.axol` to `.rs` is a manual find-and-replace.

### What the user said

> *"default rust-analyzer will not work like if u put .rs alongside .axol Bucket, axol analyzer will know and send lsp diagnosis suggestions etc but rust analyzer doesn't know. if u try to import something that was writen in .axol it will complain yk what i mean?"*

Exactly. That's the problem Gills solves.

---

## 2. The Gills architecture (a rust-analyzer fork, not a wrapper)

Gills is a **fork of rust-analyzer**, not a wrapper around it. The reason is fundamental: rust-analyzer's whole architecture assumes "Rust is the only language in the project." Trying to bolt Axolotl on top would mean fighting rust-analyzer at every layer. Forking means we get to extend the data model at the foundation.

```text
                          ┌──────────────────────────────────────┐
                          │              Gills                   │
                          │      (axol-analyzer fork)             │
                          │                                       │
                          │  ┌───────────────────────────────┐    │
                          │  │   LSP / JSON-RPC frontend    │    │
                          │  └────────────┬──────────────────┘    │
                          │               │                         │
                          │  ┌────────────┴──────────────────┐    │
                          │  │     Multi-language            │    │
                          │  │     symbol graph              │    │
                          │  │                                │    │
                          │  │  ┌──────────┐  ┌──────────┐    │    │
                          │  │  │ Axolotl  │  │   Rust   │    │    │
                          │  │  │  (axolc  │  │ (r-a     │    │    │
                          │  │  │  as a    │  │  base,   │    │    │
                          │  │  │  library)│  │  extended)│    │    │
                          │  │  └────┬─────┘  └────┬─────┘    │    │
                          │  │       │             │           │    │
                          │  │       └──────┬──────┘           │    │
                          │  │              │                  │    │
                          │  │       ┌──────┴──────┐           │    │
                          │  │       │   Unified   │           │    │
                          │  │       │   span map  │           │    │
                          │  │       └──────┬──────┘           │    │
                          │  │              │                  │    │
                          │  │       ┌──────┴──────┐           │    │
                          │  │       │ Diagnostic  │           │    │
                          │  │       │ pipeline   │           │    │
                          │  │       └──────┬──────┘           │    │
                          │  │              │                  │    │
                          │  │       ┌──────┴──────┐           │    │
                          │  │       │  Result +   │           │    │
                          │  │       │  Neoten +   │           │    │
                          │  │       │  Regrow     │           │    │
                          │  │       └─────────────┘           │    │
                          │  └────────────────────────────────┘    │
                          └──────────────────────────────────────┘
```

### What we keep from rust-analyzer

- The Salsa-based incremental query engine.
- The HIR / MIR representation and the type / trait inference.
- The completion engine's filtering and ranking.
- The inlay hints system.
- The LSP / JSON-RPC frontend.
- The IDE-oriented data structures (`Structure`, `Definition`, `Reference`, `CompletionItem`, etc.).

### What we extend

- A new **file kind** `Axolotl` alongside `Rust` in the salsa database.
- A new **crate graph node type** `AxolotlSource` that wraps an `axolc::Module` plus its generated `RustSource`.
- A new **span mapping** between Axolotl source spans and Rust source spans, used for every diagnostic.
- A new **diagnostic producer** that runs `axolc` on `.axol` files and integrates its results with the rust-analyzer pipeline.
- A new **completion provider** that knows about Axolotl-specific completions (`cblock`, `rblock`, `match`, `case`, `let`, `var`, etc., and the user's own types).
- A new **rename engine** that updates references across both languages.
- A new **workspace understanding** that treats `Bucket.jsonc` as the project manifest and `bucket.lock` as the lockfile, in addition to `Cargo.toml` and `Cargo.lock`.

### What we replace

- The Cargo workspace loader. Gills reads `Bucket.jsonc` and asks Bucket for the resolved dependency graph and the generated Rust source. It uses rust-analyzer's analysis pipeline on the **combined** virtual file set (user's `.rs` + Bucket's generated Rust), then layers Axolotl-level understanding on top.

---

## 3. The two-language symbol graph

The fundamental abstraction in Gills is the **unified symbol graph**. Every symbol in the project - whether defined in `.axol` or `.rs` - has one entry.

```text
Symbol {
    id: SymbolId,
    name: "Player",
    kind: Struct,
    language: Axolotl,
    source: {
        file: "src/player.axol",
        span: 14..22,
    },
    generated: {
        file: "pond/generated/rust/player.rs",
        span: 1..9,
    },
    definition: DefinitionId(42),
    references: [ReferenceId(1), ReferenceId(2), ...],
    type: TypeId(7),
    visibility: Public,
    documentation: "The player entity.",
}
```

The `source` and `generated` spans are both tracked. The LSP frontend always returns `source` to the editor (so the user sees their `.axol` line). The Rust analysis pipeline uses `generated` (so rust-analyzer's internals work unchanged).

When the user does "go to definition" on `Player`:

1. LSP request comes in for `Player`.
2. Gills looks up `Player` in the unified symbol graph.
3. Returns the `source` location: `src/player.axol:14`.

When rust-analyzer's internal analysis needs to resolve `Player` (during type inference, trait resolution, etc.):

1. Internal query: "what's the type of `player` here?"
2. Gills looks up `Player`'s `generated` location.
3. Returns the generated `pub struct Player { ... }` to the analysis pipeline.

Both paths use the **same** symbol graph, just exposed at different layers.

### Cross-references

When the user clicks on `Player` in `crypto.rs`:

```rust
use crate::server::Server;
use crate::database::Database;

pub fn register_player(player: &Player) { ... }   // <- here
```

rust-analyzer (in stock form) jumps to `pond/generated/rust/player.rs:1`. **Gills instead jumps to `src/player.axol:14`**, because it knows `Player` was originally defined in Axolotl.

This is the single most important UX improvement over stock rust-analyzer in a mixed project.

---

## 4. Cross-language navigation

Six navigation operations, all unified:

| Operation | LSP request | Behavior in mixed project |
|---|---|---|
| Go to definition | `textDocument/definition` | Jumps to the **source** (`.axol` if the symbol was Axolotl-defined, `.rs` if Rust-defined) |
| Go to type definition | `textDocument/typeDefinition` | Jumps to the struct / enum / interface declaration in either language |
| Go to implementation | `textDocument/implementation` | Jumps to `Type.method = fn(self, ...) ... end` blocks in Axolotl, `impl` blocks in Rust, both shown together |
| Find references | `textDocument/references` | Searches both languages, deduplicates, groups by file |
| Hover | `textDocument/hover` | Shows type, signature, doc comment, source location, **and** the original Axolotl declaration if the symbol came from `.axol` |
| Document symbols | `textDocument/documentSymbol` | Returns symbols in source order, with proper file kind markers |

### The "smart go-to" feature

For symbols that exist in **both** languages (e.g., a method declared in `.axol` and called from `.rs`), Gills provides a "smart go-to" affordance: the first click jumps to the most relevant definition; a peek view shows both.

```text
src/player.axol:14
  Player = struct
      position: Vec3
  end

src/crypto.rs:8
  pub fn process(player: &Player) { ... }

peeking from src/crypto.rs:8
  ┌─ Player ─────────────────────────────┐
  │  Defined in  src/player.axol:14     │
  │  Generated  pond/.../player.rs:1    │
  │  Used in    3 files, 12 references  │
  │                                       │
  │  [Enter] go to source (.axol)        │
  │  [Alt+Enter] go to generated (.rs)   │
  │  [Ctrl+Enter] find all references    │
  └───────────────────────────────────────┘
```

The user always lands in the language they expect.

---

## 5. The diagnostic pipeline

Gills produces diagnostics from three sources, all unified:

1. **Axolotl compiler** (`axolc`) - runs on `.axol` files, produces Axolotl-level diagnostics.
2. **Neoten** - the Axolotl linter, produces lints with fix suggestions.
3. **rust-analyzer's own diagnostics** - for the user's `.rs` files, plus the generated Rust (used internally only).

The user **never** sees diagnostics about the generated Rust. They see diagnostics about their `.axol` and their `.rs`.

### Span mapping

When `axolc` reports an error in the generated Rust:

```rust
// generated
fn main() {
    player.damage(20);   // line 1738, column 13
}
//                        ^^^ cannot move out of `player`
```

Gills looks up the span mapping for `(generated/.../main.rs, 1738, 13)` and finds `(src/main.axol, 48, 13)`. The user sees:

```text
src/main.axol:48:13
    player:damage(20)
                ^^ cannot move out of `player`
```

If `axolc` already produced a better Axolotl-level error, Gills uses that instead - no span mapping needed.

### Severity levels

| Source | Default severity | Can suppress? |
|---|---|---|
| `axolc` error | Error | No (must fix) |
| `axolc` warning | Warning | Yes (with `// @allow`) |
| Neoten lint | Hint / Warning | Yes (with `// @allow` or `// @lint:disable=NAME`) |
| rust-analyzer (user `.rs`) | All levels | Yes (with `#[allow(...)]`) |
| rust-analyzer (generated) | Suppressed | Always |

### Code actions

Every diagnostic can have code actions - quick fixes, refactorings, "explain" links. The code actions are produced by:

- **Regrow** - for "fix this" actions (auto-apply ownership rewrites, import suggestions, type annotation insertion).
- **Neoten** - for "apply suggestion" actions (style fixes, simpler-form rewrites).
- **rust-analyzer** - for Rust-side code actions, scoped to user `.rs` files only.

The user can configure which actions to auto-apply on save in `Bucket.jsonc`:

```jsonc
{
    "lsp": {
        "actions": {
            "on_save": ["imports", "remove_unused", "fix_ownership"],
            "confirm_before": ["rename_public_api"]
        }
    }
}
```

---

## 6. The completion engine

Gills's completion engine is rust-analyzer's, extended with Axolotl-specific knowledge.

### What you get in `.axol` files

- **Keywords** - `fn`, `let`, `var`, `if`, `while`, `for`, `match`, `case`, `struct`, `enum`, `interface`, `use`, `return`, `cblock`, `cppblock`, `rblock`, `pyblock`, `unsafe`, `async`, `spawn`, `task`, `try`, `catch`, `const`, `static`, `arena`, `persistent`, `move`, `borrow`, `mut`, `ref`, `mutref`.
- **Types** - `Int`, `UInt`, `Float`, `Double`, `Bool`, `String`, `Char`, `Byte`, `I8`..`I64`, `U8`..`U64`, `F32`, `F64`, `Array<T>`, `Map<K, V>`, plus the user's own.
- **Modules** - every imported crate, with their public types, functions, and traits.
- **Field access** - `player.` → `position`, `velocity`, `health`, `name`, `damage`, `update`, etc.
- **Method call** - `player:damage(20)` suggests `damage` first because it's the most recently used method on `Player`.
- **Snippet completions** - `fn` expands to a function skeleton, `struct` to a struct skeleton, `match` to a match skeleton with arms, etc.
- **AI suggestions** (if a model is configured) - see Section 16.

### What you get in `.rs` files

- Everything stock rust-analyzer provides, plus
- **Awareness of `.axol` symbols** - `use crate::server::Server` in `.rs` completes `Server` even though it was defined in `.axol`. The completion item shows the Axolotl source location as a secondary label.
- **Cross-language snippet completions** - typing `axol` in a `.rs` file can expand to `rblock { ... }` for inline Axolotl, or `bucket.add("...")` for adding an Axolotl dependency.

### Ranking

Gills uses the same context-aware ranking as rust-analyzer (most-recently-used, type-aware, name-prefix match) plus Axolotl-specific boosts:

- `Player.damage` is ranked above `Other.damage` when the receiver is `Player`.
- Methods on the inferred type are ranked above methods on other types.
- `cblock` / `cppblock` / `rblock` / `pyblock` are ranked highest when inside a `// @ffi` zone (see Section 16).
- Recently used Axolotl-specific symbols (e.g., `Vec3`, `spawn`, `arena`) get a small boost to surface them faster.

---

## 7. The refactoring engine

Gills ships refactorings that work across both languages.

### Rename

`F2` (or your editor's rename shortcut) on any symbol in any file:

1. Gills finds the symbol's definition (in either language).
2. Identifies all references across the project.
3. Shows a preview of the change.
4. On confirm, renames in all files (`.axol` and `.rs`).
5. Updates `Bucket.jsonc` and `bucket.lock` if the renamed symbol is a dependency or a config key.
6. Triggers an incremental rebuild.

The preview is critical for safety - renaming `Server` shouldn't accidentally rename `ServerConfig` in an unrelated file.

### Extract function

Select a block of code, `Ctrl+Alt+M` (or your editor's shortcut):

```axol
fn update_world(dt: Float)
    -- selected:
    for enemy in enemies do
        enemy:position = enemy:position + enemy:velocity * dt
    end
    -- end selected
    world:render()
end
```

Becomes:

```axol
fn update_world(dt: Float)
    update_enemies(dt)
    world:render()
end

fn update_enemies(dt: Float)
    for enemy in enemies do
        enemy:position = enemy:position + enemy:velocity * dt
    end
end
```

Gills infers the parameters (`dt`), the return type (none), and the visibility (private). If the function needs to be `pub` for cross-module use, Gills suggests that with a one-click option.

### Extract variable

Select an expression, `Ctrl+Alt+V`:

```axol
let pos = enemy:position + enemy:velocity * dt
```

Becomes:

```axol
let velocity_step = enemy:velocity * dt
let pos = enemy:position + velocity_step
```

### Inline

Reverse of extract. `Ctrl+Alt+N` on a function or variable replaces all uses with the body, then deletes the definition.

### Change signature

Reorder, add, remove, or rename parameters across both `.axol` and `.rs` files. All call sites are updated.

### Convert `.axol` ↔ `.rs`

For a single function or struct, Gills can convert from Axolotl to Rust or from Rust to Axolotl. This is the migration tool for teams that want to gradually move a file from one language to the other.

---

## 8. The inlay hints system

Inlay hints are the small annotations that appear inline in the editor. Gills ships them for both languages.

### Type hints

```axol
let x = compute()
--     ^ Int (inferred)
```

### Parameter hints

```axol
spawn_actor("Hero", 100, 5.0, Vec3(0, 0, 0))
--             ^ name: String  ^ health: Int  ^ speed: Float  ^ position: Vec3
```

### Lifetime hints

```axol
fn longest(a: &String, b: &String) -> &String
--            ^ inferred: 'a        ^ inferred: 'a        ^ returns: 'a (compiler chose)
```

Lifetimes are usually invisible in Axolotl. Gills shows them in hints when the user is debugging an ownership issue, or when the inferred lifetime is non-obvious.

### Borrow hints

```axol
renderer.draw(player)              -- hint: borrow
renderer:update_state(player)      -- hint: mut
inventory:add(sword)               -- hint: move
```

These are the most educational hints. They show what the compiler inferred about each argument, which helps users build intuition about ownership.

### Configuration

Inlay hints are configurable in `Bucket.jsonc`:

```jsonc
{
    "lsp": {
        "inlay_hints": {
            "types": "always",          // "always" | "never" | "on_demand"
            "parameters": "named_only", // "always" | "named_only" | "never"
            "lifetimes": "complex",     // "always" | "complex" | "never"
            "borrows": "verbose",       // "always" | "verbose" | "never"
            "chalk": "full"             // "full" | "minimal" | "off"
        }
    }
}
```

---

## 9. The in-editor TUI

For users who want more than hints can provide, Gills exposes a terminal UI inside the editor (via the editor's terminal panel):

```bash
:axolotl
```

Opens:

```text
╭─ AXOLOTL LSP TUI ────────────────────────────────────────────╮
│                                                               │
│  Workspace: my-game              Profile: debug              │
│  Files:     312 .axol + 1,842 .rs (generated)               │
│  Symbols:   47,213 indexed                                    │
│  Crates:    84 (resolved from bucket.lock)                    │
│                                                               │
│  Recent diagnostics                                           │
│  ─────────────────────                                        │
│  [E1024] src/player.axol:48  cannot move `player`             │
│  [W0201] src/world.axol:14   unused variable `gravity`        │
│  [E0001] src/crypto.rs:8     type mismatch                   │
│                                                               │
│  Open symbols                                                 │
│  ─────────────                                                │
│  Player           struct  src/player.axol:1                   │
│  Server           struct  src/server.axol:1  ◀ used by .rs    │
│  Vec3             struct  use "glam"          ◀ foreign       │
│  Damageable       trait   src/combat.axol:42                  │
│                                                               │
│  Indexing:  ████████████████████░░░  91%                      │
│  Memory:    312 MB                                            │
│                                                               │
│  [r] reload   [c] cache info   [d] dump diagnostics           │
│  [l] logs   [s] symbol search   [q] quit                      │
│                                                               │
╰───────────────────────────────────────────────────────────────╯
```

The TUI is the same one Bucket exposes at the CLI level. Gills surfaces it inside the editor for users who want a workspace-level view.

---

## 10. The build / workspace model

Gills understands a Bucket project as a unified workspace.

### Workspace resolution

```text
my-game/
├── bucket.jsonc                ◀── Gills reads this
├── bucket.lock                 ◀── Gills reads this
├── Cargo.toml                  ◀── generated by Bucket, Gills uses as fallback
├── Cargo.lock                  ◀── generated by Cargo, Gills ignores for symbol resolution
├── src/
│   ├── main.axol
│   ├── player.axol
│   ├── server.axol
│   ├── database.axol
│   ├── crypto.rs               ◀── pure Rust
│   ├── protocol.rs             ◀── pure Rust
│   └── hardware.rs             ◀── pure Rust
└── pond/
    └── generated/
        └── rust/               ◀── Gills reads this for analysis, never displays
            ├── main.rs
            ├── player.rs
            ├── server.rs
            └── database.rs
```

### The "what files does Gills see" answer

- **Always**: every `.axol` file under `src/`, `tests/`, `examples/`, `benches/`, plus any directory listed in `Bucket.jsonc`'s `build.sources` array.
- **Always**: every `.rs` file under `src/`, `tests/`, `examples/`, `benches/` of the project **and** every `.rs` file in the user's dependencies (resolved from `bucket.lock`).
- **For analysis only**: every `.rs` file under `pond/generated/rust/`. These are the files rust-analyzer actually indexes. Gills never shows them to the user.
- **Never**: build artifacts, target directories, foreign-language outputs (unless the user explicitly asks for them).

### Workspace-wide operations

| Operation | Scope |
|---|---|
| Find all references | Project-wide, both languages |
| Rename symbol | Project-wide, both languages |
| Code search | Project-wide, both languages |
| Type hierarchy | Project-wide, both languages |
| Call hierarchy | Project-wide, both languages |
| Implementation hierarchy | Project-wide, both languages |
| Workspace symbol search | Project-wide, both languages |
| Workspace diagnostic refresh | Project-wide, both languages |

### Cross-crate navigation

When the user clicks on a symbol imported from a Rust crate:

```axol
use "wgpu"
use "winit"

let instance = wgpu.Instance.new()
--                       ^ what is `Instance`?
```

Gills looks up `Instance` in the `wgpu` crate's documentation (downloaded as part of the dependency cache). The user sees:

```text
struct wgpu::Instance

The wgpu Instance. Created via Instance::new() or Instance::init().

[wgpu 27.0, src/instance.rs:42]

// Original Rust signature
pub fn new() -> Self
```

For Axolotl-defined crates, Gills shows the **Axolotl** source, not the generated Rust.

---

## 11. Editor integrations

Gills speaks the standard **Language Server Protocol (LSP)**, plus a few Axolotl-specific extensions.

### Stock LSP support

Works out of the box with any LSP-compatible editor:

- **VS Code** - install the Axolotl extension, which bundles Gills.
- **Zed** - Gills is the official Axolotl language server; Zed auto-detects it when you open an Axolotl project.
- **Helix** - `helix --health-check axolotl` verifies Gills; built-in support.
- **Neovim** - `nvim-lspconfig` includes an `axolotl` entry that points at `bucket gills`.
- **Sublime Text** - LSP-axolotl package.
- **Emacs** - `lsp-mode` + `lsp-axolotl`.
- **IntelliJ / CLion** - Axolotl plugin (uses Gills for completion, IntelliJ for its own features).
- **Lapce** - Gills auto-detected.
- **Code** (the new Code-OSS) - install via Open VSX.

### Axolotl-specific LSP extensions

A few features that don't fit standard LSP but are first-class in Gills:

- `axolotl/liveDiagnosticDiff` - see the effect of a code change in real time.
- `axolotl/crossLanguageRename` - rename across `.axol` and `.rs`.
- `axolotl/matchPreview` - see what a `match` would do for a given value, before you write it.
- `axolotl/arenaSuggestion` - see where to insert `arena { ... }` blocks for memory optimization.
- `axolotl/bucketLink` - jump from a `use "name"` to the resolved dependency in `Bucket.jsonc`.
- `axolotl/inferredOwnership` - show what the compiler inferred about each argument's borrow kind.
- `axolotl/sourceToGenerated` - open the generated Rust for the current `.axol` file in a side panel.
- `axolotl/generatedToSource` - open the source `.axol` for the current generated `.rs`.

These are exposed via a `Window/_axolotl/customRequest` channel. Editors that don't support custom LSP methods can ignore them; the standard features still work.

### Editor-specific niceties

For **VS Code**:

- The Axolotl extension renders inline type hints, parameter hints, and borrow hints using VS Code's standard `InlayHint` API.
- The diagnostic panel groups errors by file and severity.
- A side panel shows the project symbol tree (the same data the TUI shows).
- A "Tasks" panel runs `bucket build`, `bucket test`, `bucket run`, etc., and streams output.

For **Zed**:

- Zed's language registry includes Axolotl as a first-class language.
- Outlines, breadcrumbs, and the symbol panel are populated from Gills.
- Code actions and refactorings appear in the command palette.

For **Neovim**:

- `:Lsp axolotl` starts the LSP client.
- `<C-space>` for completion (with `nvim-cmp`).
- `gd` for go-to-definition (lands in `.axol` source).
- `gr` for references.
- `:Axolotl openTUI` opens the LSP TUI in a split.

For **IntelliJ / CLion**:

- The plugin reuses IntelliJ's powerful indexing for navigation and search, while delegating completion and type inference to Gills.
- The IntelliJ debugger understands Axolotl source-mapped stack frames (see Section 16).

---

## 12. The `bucket gills` CLI

Bucket manages Gills like the other tools:

```bash
bucket gills                 # start Gills in stdio mode (LSP)
bucket gills --tcp          # start in TCP mode
bucket gills --port 7331    # listen on a custom port
bucket gills status         # check if Gills is responsive
bucket gills logs           # show Gills logs
bucket gills stop           # stop any running Gills instance
bucket gills update         # update Gills to the latest version
bucket gills doctor         # health check (configures, tests LSP requests, etc.)
```

`bucket gills` is what every editor launches in the background. Bucket ensures only one Gills instance runs per project.

### Gills's own configuration

`Bucket.jsonc` has an `lsp` section:

```jsonc
{
    "lsp": {
        "trace": "verbose",                  // "off" | "messages" | "verbose"
        "inlay_hints": { ... },              // see Section 8
        "actions": { ... },                  // see Section 5
        "completion": {
            "snippets": true,
            "ai": false                      // AI completions (Section 16)
        },
        "diagnostics": {
            "neoten": true,                  // surface Neoten lints
            "axolc": true,                   // surface axolc errors
            "experimental": false           // surface experimental lints
        },
        "performance": {
            "max_memory_mb": 2048,
            "cache_size_mb": 512,
            "index_concurrency": 4
        }
    }
}
```

---

## 13. Performance

A modern project can have thousands of `.axol` files and tens of thousands of generated `.rs` lines. Gills needs to stay responsive.

### Incremental updates

Gills uses rust-analyzer's **Salsa** incremental query engine. When the user types in a file, only the affected queries re-run. A typical edit re-runs ~1% of the analysis.

### Caching

Three layers of cache:

1. **Disk cache** (project-level, in `pond/lsp/`) - persists across Gills restarts. Invalidation is content-hash-based.
2. **Memory cache** (process-level) - Salsa's internal cache.
3. **File watcher cache** - debounced by `notify` crate, coalesces rapid changes.

### Memory budget

Configurable via `Bucket.jsonc`. Default is 2 GB. A typical Axolotl project uses 300-800 MB.

### Startup time

Cold start (no disk cache) for a 1,000-file project: ~5-10 seconds.
Warm start (disk cache present): ~1-2 seconds.

Gills is **preloadable**: editors can request a Gills instance at workspace open, and Bucket keeps a long-running Gills pool that hands off pre-warmed instances on demand. Pool size is configurable in user-level Bucket settings.

### What we don't analyze

- Files in `pond/generated/rust/` are indexed by rust-analyzer internally, but Gills strips them from the user-facing symbol graph.
- Build artifacts, target directories, foreign-language outputs.
- Dependency source code - only public API surface is indexed, not the internals.

---

## 14. The Bucket.jsonc LSP section

A complete reference for the `lsp` section of `Bucket.jsonc`:

```jsonc
{
    "lsp": {
        // Trace level: "off" | "messages" | "verbose"
        "trace": "messages",

        // Inlay hints configuration
        "inlay_hints": {
            "types": "always",          // "always" | "on_demand" | "never"
            "parameters": "named_only", // "always" | "named_only" | "never"
            "lifetimes": "complex",     // "always" | "complex" | "never"
            "borrows": "verbose",       // "always" | "verbose" | "never"
            "chalk": "minimal"          // "full" | "minimal" | "off"
        },

        // Code actions
        "actions": {
            "on_save": ["imports", "remove_unused"],
            "confirm_before": ["rename_public_api", "move_function"],
            "auto_apply": {
                "imports": true,
                "format": true,
                "ownership_rewrite": "ask"   // "always" | "ask" | "never"
            }
        },

        // Completion configuration
        "completion": {
            "snippets": true,
            "auto_import": true,
            "ai": {
                "enabled": false,
                "provider": "..."   // see Section 16
            }
        },

        // Diagnostic configuration
        "diagnostics": {
            "neoten": true,
            "axolc": true,
            "experimental_lints": false,
            "max_diagnostics": 1000
        },

        // Performance configuration
        "performance": {
            "max_memory_mb": 2048,
            "cache_size_mb": 512,
            "index_concurrency": 4,
            "preload_pool_size": 1
        },

        // Editor integrations
        "editors": {
            "vscode": { ... },
            "zed": { ... },
            "neovim": { ... }
        }
    }
}
```

Bucket validates this section on `bucket check` and warns about unknown keys.

---

## 15. Gills + Neoten + Regrow collaboration

Gills is the **frontend**. Neoten is the **linter**. Regrow is the **fixer**. They collaborate through a shared message bus.

```text
User edits .axol file
   ↓
Gills receives change
   ↓
┌─────────────┬─────────────┐
│             │             │
▼             ▼             ▼
axolc       Neoten       Regrow
(compiles)  (lints)      (suggests fixes)
   │             │             │
   └─────────────┴─────────────┘
                 │
                 ▼
        Unified diagnostic stream
                 │
                 ▼
        Editor display
```

When the user opens a file:

1. **axolc** parses and type-checks the file. Errors are produced with full Axolotl-level diagnostics.
2. **Neoten** runs the linter. Warnings/hints are produced with code-action suggestions.
3. **Regrow** is on standby - when the user hovers over a diagnostic with a `// @fix` marker, Regrow computes the fix and offers it as a code action.

When the user invokes a code action:

1. Regrow computes the change in the AST.
2. Gills applies the edit to the source file.
3. axolc and Neoten re-run on the changed range (incremental).
4. The new diagnostic state is sent to the editor.

This is the same architecture rust-analyzer uses for its clippy integration, but tighter and more deeply embedded.

### The `// @allow` and `// @lint:disable` comments

Inline lint suppression, like Rust's `#[allow(...)]` but more flexible:

```axol
-- @allow unused-variable
let temp = compute()

-- @lint:disable=unused-import,inefficient-collection
use "stale_module"
-- @lint:enable

-- @lint:disable=NEOTEN_LINT_NAME
-- @reason: this is intentionally subtle; see issue #42
let tricky = ...
```

Gills surfaces these in the hover UI ("this is suppressed because: ...").

---

## 16. Future extensions

### AI hooks

Gills exposes a **completion request** hook that any AI model provider can fill:

```jsonc
{
    "lsp": {
        "completion": {
            "ai": {
                "enabled": true,
                "provider": "openai",
                "model": "gpt-5",
                "max_tokens": 256,
                "context_lines": 50
            }
        }
    }
}
```

The AI provider receives the current file, the user's cursor position, the type information, and the recent diagnostic state. It returns completions that Gills ranks alongside the static completions. AI completions are clearly marked in the UI (different icon, "AI" label).

Gills doesn't ship its own AI model. It integrates with whatever the user has configured. The hook is also exposed for local models (Ollama, llama.cpp, etc.).

### Debugger integration

Gills ships debug adapter support for `lldb-dap` and `gdb-dap`. The debugger:

- Steps through Axolotl source (using span mapping from generated Rust to `.axol`).
- Shows local variables with their Axolotl names and inferred types.
- Sets breakpoints in `.axol` files; Gills maps them to the appropriate generated Rust lines.
- Inspects `enum` variants with their declared fields.
- Shows `Option<T>` values as `Some(...)` / `None`, not as `enum { Some, None }` underneath.

### Inlay AI hints

Beyond type hints, Gills can show **natural-language explanations** of complex code:

```axol
fn process(data: mut Buffer)
    -- (hint) process: Takes a Buffer by mutable reference, modifies it
    --        in place, returns nothing. Used in 14 places.
    data:clear()
    data:append(payload)
end
```

These are computed by sending the snippet to the configured AI model with a small prompt. The model returns a 1-2 sentence explanation, which Gills caches per-AST-node.

### Test integration

Gills runs `bucket test` and shows inline test status in the editor:

- Green checkmarks next to passing test functions.
- Red Xs next to failing test functions.
- Inline diff for assertion failures.
- "Run test" / "Debug test" code lenses above each test.

### Format-on-save integration

When the user saves an `.axol` file, Gills invokes `bucket fmt` (which invokes Shed) automatically. The formatted source replaces the saved content. Configurable per-project.

---

## Why Gills is the differentiator

Most language forks build a separate LSP for the new language and tell the user "switch your editor." That breaks the moment you have a mixed project.

Gills is built **on top of rust-analyzer**, not next to it. The user gets:

- One LSP process, not two.
- One symbol graph, not two.
- One set of refactorings, not two.
- One set of completions, spanning both languages.
- One set of diagnostics, with cross-language understanding.
- One set of keybindings, working across both files.

That's the experience the user is asking for: **Gills, not rust-analyzer-plus-a-thing-on-top**.

If you have a mixed `.axol` + `.rs` project and you don't run Gills, you're using a half-broken tool. If you do run Gills, the two languages feel like one ecosystem.

Welcome to breathing through external gills.
