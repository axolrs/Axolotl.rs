# ARCHITECTURE - Axolotl

This document lays out **five candidate architectures** for the Axolotl language, evaluates each one against the constraints established in the conversation (Cargo compatibility, no runtime, mixed `.axol`+`.rs`, smart compiler, pnpm-style cache, all-purpose scope, full Rust toolchain underneath), then **picks one** and explains why the others were rejected.

---

## The constraints every architecture must satisfy

Before listing candidates, the non-negotiables from the conversation:

1. **No Axolotl runtime.** No VM, no GC, no interpreter, no scheduler, no object system.
2. **Cargo / crates.io compatibility** - every Rust crate should be a first-class Axolotl dependency.
3. **Mixed `.axol` + `.rs` projects** - Rust files live alongside Axolotl files in the same project; both can call each other.
4. **Idiomatic Rust output** - `bucket emit-rust` should produce code a Rust developer wrote.
5. **Zero overhead** - no inherent runtime cost compared with hand-written Rust.
6. **Smart compiler** - diagnostics, ownership reasoning, fix suggestions, source mapping.
7. **No overhead for unused features** - if you didn't use a `cblock`, no C build machinery is generated.
8. **pnpm-style global cache** - `~/.bucket/pond/` shared across projects, project `pond/` for outputs.
9. **All-purpose** - games are one workload, not the identity.
10. **Lua-extended surface** - Lua's syntax and table model are the starting point; TypeScript-style types layered on.
11. **`.jsonc` manifest, `.axol` extension, `pond/` build dir, Bucket / Gills / Neoten / Shed / Regrow / Ambystoma / Pond / Larva / Molt / Salamander / Eggbox toolchain.**

Any architecture that violates one of these is disqualified.

---

## Architecture 1 - The Game-DSL Compiler (Luma v1)

The first draft, before the user killed it.

```text
.axol (game DSL)
   ↓
parser
   ↓
typed AST
   ↓
ownership analysis
   ↓
ECS transformation
   ↓
optimization
   ↓
LLVM / Cranelift
   ↓
native executable
```

The language has game-specific keywords: `entity Player { transform; collider; health; }`, `game MyGame { start { ... } update { ... } draw { ... } }`, `spawn(Enemy)`, `arena`, `frame`, `level`, `parallel { ... }`. The compiler is its own thing - parser → typed AST → ownership analysis → ECS transform → LLVM.

### Why it was rejected

- **Game-only by grammar.** Can't write a CLI, a database, a web server, an embedded controller, a compiler, a flight system. The grammar makes "this is a game language" an identity, not a feature.
- **No Cargo compatibility.** A separate `luma install` / `luma packages` ecosystem has to be invented, populated, and maintained. That's a decade of work before you can write a real program.
- **Custom backend.** Maintaining an LLVM frontend is a serious project; maintaining a Cranelift backend plus a custom linker story plus platform support plus target triples plus sanitizers is impossible for a new language.
- **No mixed `.axol` + `.rs`.** The compiler is the only path to native code. Rust files would need to go through a custom FFI bridge.
- **Inventing arena / frame / level as keywords** instead of importing `bumpalo` / `slotmap` is the wrong design - those are library concerns, not language concerns.

### Verdict

**Rejected.** The user said it best: *"It's horrible. It's not just for scripting. It should be on top of Rust."*

---

## Architecture 2 - The Transpile-to-Rust-Source Compiler (Luma v2)

The second pass, after the user asked for "transpile to idiomatic Rust."

```text
.axol
   ↓
parser
   ↓
type checker
   ↓
ownership / lifetime inference
   ↓
Luma IR
   ↓
Rust SOURCE generator
   ↓
rustc / cargo
   ↓
native binary
```

The compiler is a transpiler. It reads `.axol`, infers ownership, and emits a Rust source tree under `luma_out/` that you can read, edit, and compile with ordinary Cargo. `luma build --emit-rust` is the primary mode.

### Why it was rejected

- **Cargo compatibility is shallow.** You can `use serde` only after the Luma compiler writes a parallel "Luma serde" wrapper that exposes serde's types through Luma syntax. For every crate, every version, every feature set.
- **Trait-heavy APIs don't transpile cleanly.** Crates with associated types, higher-ranked trait bounds, async traits, generic-associated types, complex procedural-macro DSLs are essentially unrepresentable in a transpiler that produces source instead of metadata-aware calls.
- **The generated source is a real source tree.** It must compile with the same `rustc` that compiles handwritten Rust. Any drift between handwritten-Rust expectations and generated-Rust quirks is a long-tail pain.
- **You can never compete with rustc.** Every improvement rustc makes - better diagnostics, better monomorphization, better trait solver, better LTO - is invisible to Luma's transpiler. The user is stuck with whatever the transpiler emits.
- **Two sources of truth.** The Luma source and the generated Rust source have to stay in sync. Source maps, span mapping, format round-trips - all become a chore.

### Verdict

**Rejected as the primary path.** Source generation survives as `bucket emit-rust` for debugging, but the actual build pipeline cannot be source transpilation if Cargo compatibility is to be honest.

---

## Architecture 3 - The Hybrid (Transpile + Rust Metadata Import)

A middle ground: do source transpilation for Axolotl's own modules, and import Rust crate metadata for external dependencies.

```text
.axol
   ↓
parser
   ↓
type checker
   ↓
ownership / lifetime inference
   ↓
┌──────────────┬──────────────┐
↓              ↓              ↓
local .rs     foreign crate  Lua-style
(transpiled)  (metadata      coroutines
              imported)      (LuaJIT)
   ↓              ↓              ↓
rustc        rustc         LuaJIT
   ↓              ↓              ↓
              native binary
```

Keeps the Lua C API alive. Keeps LuaJIT around for hot paths. Generates Rust source for Axolotl's own code. Imports Rust metadata for crates.

### Why it was rejected

- **Three backends.** Transpile to Rust, import Rust metadata, *and* keep LuaJIT. Each backend has bugs, semantics drift, and update lag.
- **The Lua C API is a foreign-function interface.** The user said the goal is a *new* language with Lua syntax, not "Lua glued to Rust glued to LuaJIT." Maintaining Lua source compatibility is a permanent backward-compat tax.
- **LuaJIT doesn't speak Rust ownership.** When you go from a Lua coroutine into Rust code, you cross an FFI boundary. You lose the borrow checker's protection on the way through.
- **Dynamic scripting inside a static language is a niche use case.** Most Axolotl programs will be fully static. The dual-mode complexity isn't worth it.
- **Two languages, not one.** The user explicitly said: *"You're not putting Rust safety into Lua. You're making a new native language whose syntax is inspired by Lua, then using Rust as its compilation target."*

### Verdict

**Rejected.** The dynamic-Lua / static-Axolotl split was an early sketch that the user themselves disowned. The language should not be "Lua plus extras." It is a single static language with Lua-like syntax and Rust's semantics.

---

## Architecture 4 - The Custom Backend (Cranelift / LLVM / MLIR Frontend)

Skip Rust entirely. Build Axolotl's own compiler that goes all the way to native code.

```text
.axol
   ↓
parser
   ↓
type checker + ownership inference
   ↓
Axolotl IR
   ↓
optimization
   ↓
Cranelift / LLVM / MLIR
   ↓
machine code + custom linker
   ↓
native executable
```

Drop-in replacement for the toolchain. Target multiple platforms. Invent a package manager. Invent a runtime library. Become a competitor to Rust from day one.

### Why it was rejected

- **The hard part is the easy part to skip.** Building a parser / type checker / ownership inferencer is hard but tractable. Building a competitive backend, optimizer, linker, ABI, sanitizer support, target triples, platform support, embedded targets, debug info, profile-guided optimization - that is **decades of work**.
- **No Cargo compatibility.** Cargo compatibility requires either importing Rust metadata (which requires you to *be* Rust) or hand-writing FFI bindings for every crate. There is no third option.
- **Reinvents crates.io.** A new package ecosystem has to be built, populated, and maintained from zero. Without crates.io, Axolotl is a research language, not a production tool.
- **Slower to mature.** Every month spent on a custom backend is a month not spent on language design, diagnostics, tooling, and adoption. The user said it directly: *"To make this language happen, I don't think I can think of anything outside of Rust anyways."*

### Verdict

**Rejected.** The whole point of Axolotl is to inherit Rust's backend, not to replace it. Building a custom backend is a multi-decade project that doesn't deliver more than rustc already does.

---

## Architecture 5 - The Cargo Frontend (chosen)

The architecture that was actually settled on.

```text
                  AXOLOTL
              (the language)
                     │
                     ▼
                ┌─────────┐
                │  axolc  │  Axolotl compiler
                └────┬────┘
                     │
        ┌────────────┼────────────┐
        │            │            │
   Axolotl          Axolotl      Foreign
   semantic       Rust generator  blocks
   analysis            │       (cblock/cppblock/
        │              │         rblock/pyblock)
        │              │
        ▼              ▼
   diagnostics     idiomatic Rust source
        │              │
        │              ▼
        │         ┌────────┐
        │         │  Cargo │  (managed by Bucket)
        │         └────┬───┘
        │              │
        │              ▼
        │           rustc
        │              │
        │              ▼
        │            LLVM
        │              │
        └──────────────┤
                       ▼
                native binary
```

And below the AOT path, a parallel **iteration path** through `axol-hot-runner`:

```text
        .axol
          │
          ▼
   ┌─────────────┐
   │  axolc-core │ (in-process)
   └──────┬──────┘
          │
    ┌─────┴──────┐
    ▼            ▼
interpreter    JIT
(tree-walk)  (Cranelift)
    │            │
    └─────┬──────┘
          ▼
   shared runtime
   (memory, FFI, panic, task scheduler)
          │
          ▼
    hot reload on save
    (notify-based watcher)
```

`axol-hot-runner` is the iteration path. It uses `axolc-core` in-process, runs the same HIR the AOT pipeline produces, and either interprets it directly or JIT-compiles it with Cranelift. Mixed `.axol` + `.rs` execution is handled by a unified symbol graph and a shared runtime environment. Opt-in via `interpreted_dev_mode: true` in `Bucket.jsonc`. The full design is in `Interpreter.md`.

### The six architectural decisions

**Decision A: Axolotl compiles to Rust, not to machine code.**

The compiler's job ends at producing idiomatic Rust source plus the right Cargo metadata. rustc, LLVM, and the linker are downstream of Axolotl. Every improvement to rustc - better trait solver, better monomorphization, better diagnostics, better LTO, better sanitizers, new targets - flows to Axolotl automatically.

**Decision B: Cargo is the build engine. Bucket is the orchestrator.**

`cargo build` doesn't know `.axol` exists. That's fine. Bucket owns the user-facing build, the dependency graph, the cache, the logs, the TUI. When Bucket invokes Cargo, it does so with a generated `Cargo.toml` that points at the Axolotl-generated Rust and at the user's `.rs` files. Cargo does the actual compilation. Bucket does the orchestration.

**Decision C: Rust crate metadata is imported, not re-implemented.**

When an Axolotl program writes `use "wgpu"`, the compiler reads `wgpu`'s `.rmeta` (or equivalent rustc metadata), understands the actual Rust API - generics, traits, associated types, async, macros - and generates calls into it. No parallel `AxolotlWgpu` wrapper crate. No hand-written FFI bindings. The real `wgpu` is linked in.

**Decision D: `.axol` and `.rs` are first-class siblings.**

A project can contain both. Axolotl can `use "crypto"` where `crypto.rs` is a hand-written Rust module. Rust can `use crate::server` where `server.axol` is Axolotl. The boundary between the two is a normal Rust ABI boundary, not a foreign-function boundary. Cargo's CLI sees the `.rs` files. Bucket's CLI sees both. Mixed projects are first-class.

**Decision E: Foreign blocks are opt-in machinery, not defaults.**

`cblock { ... }`, `cppblock { ... }`, `rblock { ... }`, `pyblock { ... }` only generate the corresponding build machinery (C compiler invocation, `build.rs`, linker flags, Python runtime) **when present**. A pure Axolotl project with no foreign blocks has zero foreign build configuration in its `pond/`.

### Why this architecture is best

- **It satisfies every constraint.** No runtime, Cargo compatible, mixed projects, idiomatic output, zero overhead, smart compiler, usage-driven build, global cache, all-purpose, Lua-extended surface, full toolchain - every one is honored.
- **It minimizes the new-code surface.** Instead of writing a backend, optimizer, linker, package manager, ecosystem, target support, and platform story, the project writes a frontend, a metadata importer, an ownership inferencer, a code generator, a build orchestrator, and a toolchain. The hard parts of running real software on real hardware are inherited from rustc/Cargo/LLVM, which the user explicitly endorsed.
- **It has a credible day-one story.** The first useful Axolotl program is one that calls `wgpu` or `tokio` from `.axol` source. With this architecture, that's a metadata import plus a thin codegen, not a parallel ecosystem.
- **It has a credible adoption path.** A team can introduce Axolotl into a Rust project file by file. They keep their existing `crypto.rs`, `protocol.rs`, `hardware.rs`. They write new features in `.axol`. They never migrate away from anything that works.
- **It makes the smart compiler a real possibility.** Because errors caught before Rust generation produce Axolotl-level diagnostics, the user never has to read generated-Rust error text. Because the compiler sees the Axolotl AST, it can suggest fixes that the user can apply with one keystroke. Because spans map back, errors point at the line the user wrote.
- **It is coherent.** The same principle - *the user pays only for what they use* - drives every layer: no runtime, conditional FFI machinery, conditional Python integration, content-addressed global cache, debug-vs-release separation, optional features. That single principle unifies the language design, the compiler design, and the toolchain design.

### Verdict

**Chosen.** This is the architecture the conversation converged on. Every other option is a dead end, a regression, or a multi-decade project that doesn't deliver more than what Rust already provides.

---

## The final architecture in one diagram

```text
                         ┌─────────────────────────────┐
                         │            USER             │
                         │  edits .axol + .rs files    │
                         └──────────────┬──────────────┘
                                        │
                                        ▼
                              ┌──────────────────┐
                              │  Bucket.jsonc    │
                              │  bucket.lock     │
                              └────────┬─────────┘
                                       │
                                       ▼
                              ┌──────────────────┐
                              │      BUCKET      │
                              │ (the orchestrator)│
                              │                   │
                              │  dependency graph │
                              │  cache lookup     │
                              │  TUI / logs       │
                              │  diagnostics      │
                              └────────┬──────────┘
                                       │
              ┌────────────────────────┼────────────────────────┐
              │                        │                        │
              ▼                        ▼                        ▼
       ┌────────────┐          ┌────────────┐          ┌────────────┐
       │   axolc    │          │  Cargo /   │          │ Foreign    │
       │  compiler  │          │  rustc     │          │ build      │
       │            │          │            │          │ (cc, g++,  │
       │ parser     │          │            │          │  python)   │
       │ types      │          │            │          │ only if    │
       │ ownership  │          │            │          │ cblock /   │
       │ diagnostics│          │            │          │ cppblock / │
       │ codegen    │          │            │          │ rblock /   │
       └─────┬──────┘          └─────┬──────┘          │ pyblock    │
             │                       │                 │ present    │
             ▼                       │                 └─────┬──────┘
      generated Rust                 │                       │
      + Rust crate                   │                       │
      metadata imports               │                       │
             │                       │                       │
             └───────────┬───────────┴───────────────────────┘
                         │
                         ▼
                   native binary

            Global cache:    ~/.bucket/pond/store/
            Project cache:   project/pond/{debug,release}/

            Tooling:    Gills (LSP), Neoten (lint), Shed (fmt),
                        Regrow (fix), Ambystoma (docs),
                        Salamander (test/bench), Larva (scaffold),
                        Molt (migrate), Eggbox (publish)
```

That is the Axolotl architecture, end to end. The compiler is a frontend. The orchestrator is Bucket. The engine is Cargo. The optimizer is rustc + LLVM. The linker is the system linker. The package manager is Cargo plus Bucket's global Pond. The IDE is Gills. The linter is Neoten. The formatter is Shed. The fixer is Regrow. The doc generator is Ambystoma. The runtime is whatever the user's explicit dependencies require, and nothing more.

There is no Axolotl runtime. There never will be.
