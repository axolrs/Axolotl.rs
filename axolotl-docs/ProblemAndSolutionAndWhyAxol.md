# Problem And Solution And Why Axolotl

This document is the **decision log** for the Axolotl language. Every problem raised during the design conversation is listed, paired with the solution that was settled on, and tied back to the reason Axolotl exists at all.

If a future contributor asks "why is it this way and not that way?" - this document is the answer.

---

## The umbrella problem

> **How do you give programmers the safety and performance of Rust, the ergonomics of Lua, the entire Cargo ecosystem on day one, and a smarter compiler than either - without inventing a runtime, a new backend, a new package manager, or a new ecosystem?**

That is the question. Every problem below is a sub-question of it. Every solution is a sub-answer. The "Why Axolotl" section at the end is the final synthesis.

---

## P1. The simplicity problem

**Problem.** Rust makes the programmer think about ownership, borrowing, lifetimes, traits, generics, async, smart pointers, `Send`/`Sync`, pinning, interior mutability - even to write a "Hello, world." That cognitive load is the price of Rust's safety, but it's a real barrier for game developers, web developers, scientists, and beginners.

**Solution.** Axolotl **infers ownership** by default. The programmer writes Lua-like code; the compiler figures out whether a value is copied, moved, borrowed, or mutably borrowed. Lifetimes are inferred, never written. The borrow checker still runs - it's just hidden from the user 95% of the time.

**Why Axolotl.** This is the central design decision. Every other language that tried to be "Rust but easier" either sacrificed safety (Nim, Zig), gave up the ecosystem (Carbon, Mojo), or invented a parallel runtime (TypeScript, Kotlin/Native). Axolotl keeps Rust's safety and Rust's ecosystem by moving the complexity from the **programmer** into the **compiler**.

---

## P2. The GC problem

**Problem.** Lua, Python, JavaScript, Go, Java, C#, and most modern languages use a garbage collector. GC is convenient but disqualifies a language from real-time systems, embedded programming, game engines, and high-performance servers. Lua programmers know this; Rust programmers know this; game engine developers know this.

**Solution.** Axolotl has **zero GC**. Memory is managed through Rust's ownership model with deterministic destruction. Values are dropped the moment their owner goes out of scope. There is no tracing collector, no generational heap, no GC pause, no finalizer queue.

**Why Axolotl.** A language that calls itself "no GC" but secretly injects a runtime interpreter, a boxed-value heap, or a reference-counting layer isn't "no GC" - it's GC with extra steps. Axolotl's "no runtime" invariant is the honest version.

---

## P3. The ecosystem problem

**Problem.** A new language is dead on arrival if it has no ecosystem. Building a package manager, populating it, getting authors to publish, convincing users to depend on it - that's a decade of work. Languages like Carbon, Mojo, and Zig are running this marathon right now.

**Solution.** Axolotl **inherits crates.io** by compiling to Rust. The Cargo crate graph is the Axolotl crate graph. Every Rust crate ever published is, on day one, an Axolotl library. There is no `luma install`, no `axolotl packages`, no parallel ecosystem to build.

**Why Axolotl.** This is the single biggest adoption accelerator. A team that already uses `wgpu`, `tokio`, `serde`, `axum`, `bevy`, `rapier`, `glam` can keep using all of it from `.axol` files.

---

## P4. The custom-backend problem

**Problem.** Building a competitive compiler backend (LLVM frontend, optimizer, linker, platform support, target triples, sanitizer support, debug info, PGO) is a multi-decade effort. Languages that try it (Carbon, Mojo) spend years on infrastructure instead of language design and adoption.

**Solution.** Axolotl **stops at Rust source generation**. The compiler's job is to produce idiomatic Rust; `rustc` and `LLVM` do the actual compilation. Every improvement to `rustc` (better trait solver, better monomorphization, better LTO, new targets) flows to Axolotl automatically.

**Why Axolotl.** The user explicitly said it: *"To make this language happen, I don't think I can think of anything outside of Rust anyways."* Reusing rustc is the single most leverage-positive decision in the entire project.

---

## P5. The game-DSL trap

**Problem.** The first version of the language (called Luma at the time) tried to be a **game-specific DSL** with `entity`, `game`, `update`, `draw`, `spawn`, `arena` as keywords. That meant the language could only write games - not databases, web servers, compilers, or flight controllers. The user killed it: *"It's horrible. It's not just for scripting. It should be on top of Rust."*

**Solution.** Axolotl is a **general-purpose language**. Game development is a library problem, not a grammar problem. The game-related concepts (frame arenas, level arenas, ECS, etc.) are libraries, not keywords. You can build a game engine **in Axolotl** if you want to - or you can use `bevy` from a Rust crate.

**Why Axolotl.** Confining a language to a single domain is the fastest way to make it irrelevant. Axolotl's all-purpose identity is what makes it viable as a long-term project.

---

## P6. The "it's basically Rust" problem

**Problem.** The second version of the language (still Luma) was Rust with prettier punctuation. `let`/`var`/`fn`/`impl`/`match` - same model, different surface. The user said it: *"It's not much different than Rust though."*

**Solution.** Axolotl is **Lua extended**, not Rust simplified. The surface syntax is Lua's: `if/then/elseif/else/end`, `while/do/end`, `function ... end`, `:` method calls, `.` field access, `[...]` index, `{ ... }` table constructors, multiple return values, `:` for method call, `repeat/until`. TypeScript-style patterns layer on top (`let x: Int = ...`, inferred types, `?` for nullable).

**Why Axolotl.** The change from Rust surface to Lua surface is what makes the language feel like a *new* language rather than a theme. Programmers who already know Lua (game developers, scripting users) can read Axolotl immediately.

---

## P7. The no-runtime hard rule

**Problem.** The user kept emphasizing: *"Axol should never ever have any runtime. That will break it."* This isn't a perf optimization; it's a design constraint. Languages that "have a minimal runtime" eventually have a large runtime, and that breaks the "0 overhead" claim.

**Solution.** The full set of things that **do not exist** at Axolotl runtime:

```text
❌ Axolotl VM
❌ Axolotl interpreter
❌ Axolotl GC
❌ Axolotl scheduler
❌ Axolotl object system
❌ Axolotl heap
❌ Axolotl reflection runtime
❌ Axolotl standard runtime
```

The compiler lowers the program into ordinary Rust; whatever the generated Rust needs at runtime is exactly what the binary contains. No more, no less.

**Why Axolotl.** Without this rule, the language slides into "yet another runtime language" and loses the most important property: the ability to read the generated Rust and trust it.

---

## P8. The transpile-to-source problem

**Problem.** The first attempt at Cargo compatibility was **transpile to Rust source**. The compiler reads `.axol`, infers ownership, and emits a `.rs` file tree that you compile with Cargo. This works for the first program, but fails on:

- generic-heavy APIs
- trait-heavy APIs
- async traits
- procedural macros
- associated types
- higher-ranked lifetimes
- macro-syntax expectations
- closure types
- exotic ownership patterns

The result: you'd end up writing a parallel "Axolotl std library" to wrap every crate. That's a maintenance nightmare.

**Solution.** Axolotl **imports Rust crate metadata** directly. The compiler reads the crate's `.rmeta`, understands the real Rust API, and generates calls into it. No wrappers, no porting, no "AxolotlWgpu." `wgpu` is the actual `wgpu`. Source generation (`bucket emit-rust`) survives as a **debugging tool**, not the primary path.

**Why Axolotl.** Cargo compatibility has to be **native and seamless**, or it's not compatibility at all.

---

## P9. The dynamic-Lua trap

**Problem.** The conversation flirted with keeping the Lua C API and LuaJIT alive - a "two-mode" language where static Lua compiles to Rust and dynamic Lua runs on LuaJIT. This was the user's first sketch, then rejected: *"You're not putting Rust safety into Lua. You're making a new native language whose syntax is inspired by Lua, then using Rust as its compilation target."*

**Solution.** Axolotl is a **single static language**. There is no Lua VM, no Lua C API, no LuaJIT. The surface syntax is Lua-flavored, but the semantics are fully static. If you want to script a host application, use `rblock` or embed the Axolotl compiler - but the language itself doesn't ship a scripting runtime.

**Why Axolotl.** Two execution modes means two backends, two sets of bugs, two sets of semantics, and a permanent backward-compat tax. The user recognized this and killed the dual-mode idea early.

---

## P10. The Lua ↔ Rust bidirectional problem

**Problem.** The user said: *"If you write Rust you should be able to import in Lua. If you write Lua you should be able to import in Rust. I don't know what I'm saying."* The actual intent: a **mixed-language workspace** where Rust and Axolotl are peers in the same project, each callable from the other.

**Solution.** Axolotl supports **`.axol` + `.rs` mixed projects**. The two languages live in the same `src/` directory. Axolotl can `use "crypto"` where `crypto.rs` is a hand-written Rust module. Rust can `use crate::server` where `server.axol` is Axolotl. The boundary is a normal Rust ABI boundary, not a foreign-function interface. Bucket understands both file types. Cargo's CLI understands only `.rs`; that's fine, because Bucket is the orchestrator.

**Why Axolotl.** This is the feature that makes adoption painless. A team can introduce Axolotl into a Rust project file by file. Nothing that works stops working. The Rust codebase continues to evolve; the new features get written in Axolotl.

---

## P11. The build-tool problem

**Problem.** Cargo is excellent but limited:

- It doesn't understand `.axol`.
- It produces a `target/` directory that balloons to 20-60 GB per project.
- It doesn't produce a useful build log.
- It doesn't surface Axolotl-level diagnostics.
- It doesn't expose a friendly TUI.
- It doesn't have a pnpm-style global cache.
- It doesn't manage Axolotl-specific configuration.

**Solution.** **Bucket** is a new project / build / dependency / package tool that sits above Cargo. Bucket understands `.axol` files, `.rs` files, foreign blocks, the Pond cache, the `Bucket.jsonc` manifest, the smart compiler, the Gills LSP, the Neoten linter, the Shed formatter, the Regrow auto-fixer, the Ambystoma doc generator, the Salamander test runner, the Larva scaffolder, the Molt migrator, the Eggbox registry.

`bucket cargo <anything>` is a transparent passthrough, so the user never loses access to Cargo.

**Why Axolotl.** Cargo is the engine. Bucket is the control plane. The user lives in `bucket ...` and never types `cargo ...` unless they want to.

---

## P12. The 20-60 GB `target/` problem

**Problem.** Cargo's per-project build directory can grow to 20-60 GB. Every project recompiles every dependency, even when the dependency hasn't changed and the project isn't using anything new.

**Solution.** The **Pond cache** is a pnpm-style global content-addressed cache:

```text
~/.bucket/pond/
├── store/
│   ├── sources/        # crate source: serde@1.0.219/, tokio@1.47.1/, ...
│   └── artifacts/      # compiled rlibs keyed by content hash
├── objects/            # content-addressed blobs
└── index/              # which project needs which artifacts

project/pond/
├── debug/              # per-project build outputs (linked, not copied)
├── release/            # fresh production artifacts
└── logs/               # structured build logs
```

The artifact key is comprehensive (source hash + dep graph + features + rustc version + target + profile + flags). Same key → reuse. Different key → recompile.

**Why Axolotl.** This is a pnpm-style insight applied to Rust artifacts. It saves disk space, speeds up builds, and is the kind of quality-of-life win that makes developers stay with a tool.

---

## P13. The release-build purity problem

**Problem.** Should `bucket build --release` reuse debug artifacts? If yes, you save time. If no, you guarantee a clean production build. Mixing them is a foot-gun.

**Solution.** **`bucket build --release` does not inherit debug artifacts**. Release gets a clean profile; debug gets aggressive reuse. Both share the same global Pond store. Production builds are never accidentally polluted with development artifacts.

**Why Axolotl.** "Release mode should rebuild everything from scratch" was the user's exact words. The two profiles are deliberately separated.

---

## P14. The missing-build-log problem

**Problem.** Cargo's build output scrolls past the terminal and is gone forever. When a build fails or is slow, you have no record of what happened. "Why did this build take 18 seconds instead of 3?" is unanswerable.

**Solution.** Every Bucket build produces two files in `pond/{profile}/logs/`:

1. **`.log`** - human-readable complete build transcript.
2. **`.jsonl`** - structured events for tooling, CI, IDEs.

Plus metadata: git revision, Bucket version, axolc version, Rust version, Cargo version, target triple, profile, features, dependency lock hash, environment fingerprint, build duration, cache statistics.

`bucket logs` opens a TUI showing recent builds. `bucket logs --errors` extracts diagnostics. `bucket logs --latest` opens the most recent. **Failed builds are never thrown away**.

**Why Axolotl.** Build logs are first-class artifacts. The fancy TUI is ephemeral UX; the log is the source of truth.

---

## P15. The foreign-FFI pollution problem

**Problem.** Most languages with FFI make every project pay the cost of every possible FFI target, or make FFI a miserable manual experience. C++, Rust, and even LuaJIT have this: writing FFI bindings is tedious, and the build system is hard to configure.

**Solution.** Axolotl has four opt-in foreign blocks:

- `cblock { ... }` - embeds raw C
- `cppblock { ... }` - embeds raw C++
- `rblock { ... }` - embeds raw Rust
- `pyblock { ... }` - embeds raw Python

**If a project has no `cblock`, Bucket generates zero C build machinery. No `cc` configuration, no `build.rs` for C, no linker flags, nothing.** Same for the others. The "what you didn't use doesn't exist" principle applied to FFI.

**Why Axolotl.** This is the same philosophy as the no-runtime rule. Unused features must not cost anything.

---

## P16. The "should I bring LuaJIT" question

**Problem.** The user asked: *"Bring Lua and Rust in head. Lua first. Bring Lua C compiler library as well, LuaJIT, and rustc in head. Remake Lua. Lua compiler that produces idiomatic Rust."* This initially suggested keeping LuaJIT and the Lua C API as part of the project.

**Solution.** **Don't.** LuaJIT is a separate runtime. The Lua C API is a foreign-function interface. Neither fits the "no runtime" rule. Axolotl takes Lua's *syntax* and Lua's *table-as-record* idea, but compiles to Rust, not to LuaJIT. The Lua C API is not exposed. There is no Lua VM underneath.

**Why Axolotl.** The user clarified the intent: they want a new language that **inherits Lua's surface**, not "a Lua compiler." Once that was clear, LuaJIT dropped out of the architecture.

---

## P17. The `if/else` ceremony problem

**Problem.** Rust makes you write `if condition { ... } else { ... }` - fine, but you also need braces on every block, semicolons, `let` for bindings, `mut`, `&` for borrows, `'` for lifetimes. A 10-line program in Rust is a 30-line program in Axolotl.

**Solution.** Axolotl uses Lua-style control flow:

```axol
if health <= 0 then
    die()
elseif health < 25 then
    show_warning()
else
    update_hud()
end

while running do
    tick()
end

for i = 1, 10 do
    process(i)
end

for enemy in enemies do
    enemy:update(dt)
end
```

No braces for top-level blocks. `then`/`do` mark the start of a block. `end` marks the end. Expression-oriented (the last expression is the return value).

**Why Axolotl.** Lua's control flow is famously readable. A first-grader can understand `if/then/else/end`.

---

## P18. The `impl` ceremony problem

**Problem.** Rust's `impl Foo for Bar { ... }` syntax is verbose. For a small struct, the impl block can be larger than the struct itself.

**Solution.** Axolotl attaches methods by name:

```axol
Player.update = fn(self, dt)
    self.position = self.position + self.velocity * dt
end

Player.damage = fn(self, amount)
    self.health -= amount
end
```

No `impl` block. No separate `impl Player` for the methods and `impl Drawable for Player` for the trait impls. Just `Type.method_name = fn(self, ...) ... end`.

**Why Axolotl.** The compiler still generates ordinary Rust `impl` blocks underneath. The user just doesn't write them.

---

## P19. The `Result<T, E>` ceremony problem

**Problem.** Rust's `Result<T, E>` is the right abstraction, but propagating errors with `?` everywhere, or matching `Ok`/`Err` everywhere, gets old. The `Box<dyn Error>` workaround is even worse.

**Solution.** Axolotl has four error-handling forms, all compiling to ordinary Rust `Result`:

```axol
-- 1. `?` propagation
let tex = load_texture("player.png")?

-- 2. `??` fallback
let tex = load_texture("missing.png") ?? default_texture()

-- 3. `try/catch` block
try
    let tex = load_texture("player.png")?
    renderer.draw(tex)
catch e: IoError
    print("failed: ${e}")
end

-- 4. Lua-style multiple return
value, err = load_texture("player.png")
```

**Why Axolotl.** Each form is right for a different context. The user picks the one that reads best. The compiler picks the right Rust lowering.

---

## P20. The null / nullable problem

**Problem.** Rust doesn't have nullable types in the traditional `null` sense. It uses `Option<T>`, which is safe but verbose. Languages that *do* have `null` (C, C++, Java, JavaScript) pay for it in bugs - the "billion-dollar mistake."

**Solution.** Axolotl adds `T?` as **syntactic sugar over `Option<T>`**, plus three operators:

- `x?.field` - optional chaining (produces a nullable value)
- `x!.field` - assert non-null (panics if wrong)
- `x ?? default` - null coalescing

`null` is **not** a universal inhabitant. It only exists in `T?` contexts. A `String` can never be null; only a `String?` can.

**Why Axolotl.** This gives Lua/Python/JavaScript programmers the ergonomics of nullable types without sacrificing Rust's null-pointer safety.

---

## P21. The "smart compiler" requirement

**Problem.** The user said: *"Smartest compiler who will help you write code as well as manage the rest."* Cargo's error messages are good but Rust's are famously dense for beginners. Most "language design" projects stop at syntax.

**Solution.** The Axolotl compiler is genuinely intelligent:

- **Recovers from parse errors** and continues analysis so you see all errors in one build.
- **Catches what it can before generating Rust**, so most diagnostics are Axolotl-level, not rustc-level.
- **Maps generated-Rust spans back to Axolotl source spans** - you see your line, not `target/debug/build/.../out/generated_34891.rs:1738`.
- **Suggests fixes the user can apply with one keystroke** - including ownership rewrites ("did you mean to borrow this instead of moving?").
- **Shows ownership as a visual diff**:
  ```text
  - renderer.draw(player)
  + renderer.draw(&player)
  ```
- **Has a live TUI build dashboard** with parallel activity, not `Compiling foo... Compiling bar...`.
- **Supports `--plain`** for CI and **`--json`** for tooling.

The toolchain around the compiler:

- **Gills** - LSP / IDE intelligence (autocomplete, go-to-def, references, rename, inline diagnostics, Axolotl ↔ Rust navigation)
- **Neoten** - a linter broader than Clippy, with Axolotl-specific checks
- **Regrow** - automatic fixes, refactoring, ownership-rewrite suggestions
- **Shed** - formatter
- **Ambystoma** - documentation generator
- **Salamander** - testing and benchmarking
- **Larva** - `bucket new` - project scaffolding
- **Molt** - toolchain / project migration
- **Eggbox** - package publishing / registry

**Why Axolotl.** Most languages ship a compiler and a `fmt` tool and call it a day. Axolotl ships an engineering environment. The compiler is the product, not just the binary that produces the binary.

---

## P22. The naming problem

**Problem.** The user wanted tool names "related to axolotl" - not "Clippy," not "rustfmt," not "rust-analyzer." The whole ecosystem should feel cohesive.

**Solution.** Real Axolotl biology drives the names:

| Tool | Job | Axolotl biology |
|---|---|---|
| `axolc` | The compiler | the animal |
| Bucket | Project / build / dependency / package manager | the bucket that holds everything |
| Gills | LSP / IDE intelligence | axolotls breathe through external gills |
| Neoten | Linter and static analysis | axolotls stay juvenile (neoteny) |
| Shed | Formatter | shedding old skin |
| Regrow | Automatic fixes / refactoring | axolotls famously regenerate limbs |
| Ambystoma | Documentation generator | the axolotl's scientific genus |
| Pond | Build / output / cache directory | axolotls live in ponds |
| Larva | `bucket new` - scaffolding | the larval form |
| Molt | Toolchain / project migration | shedding old code for new code |
| Salamander | Testing and benchmarking | axolotls are salamanders |
| Eggbox | Package publishing / registry | where the eggs go |

**Why Axolotl.** The names aren't decoration. They make the toolchain feel like one ecosystem. "Gills flagged this" and "Neoten warned that" and "Regrow fixed it" are recognizable shorthand.

---

## P23. The file-extension / manifest / build-dir problem

**Problem.** What extension? What manifest format? What build directory? If these clash with Cargo's conventions, integration gets awkward.

**Solution.**

- Source extension: **`.axol`**
- Compiler: **`axolc`**
- Project tool: **`bucket`**
- Build directory: **`pond/`** (not `target/`)
- Project manifest: **`Bucket.jsonc`** (JSONC, not TOML)
- Lockfile: **`bucket.lock`**

Cargo still uses `target/` for its internal state; Bucket uses `pond/` as the user-facing build directory. Bucket tells Cargo `CARGO_TARGET_DIR=./pond` so Cargo's outputs land in `pond/` too. If you `cargo build` directly, it works as expected (using `target/`). If you `bucket build`, the outputs go to `pond/`.

**Why Axolotl.** The user wanted a clean separation between the Cargo engine and the Bucket control plane. The naming reinforces that.

---

## P24. The "what's the use case" problem

**Problem.** The user corrected me repeatedly: *"I said a million times it's not a game-development focused language. It's an all-rounder for everything including rocket science."* I kept framing games as the primary use case. That was wrong.

**Solution.** Axolotl's identity is **all-purpose native systems + applications + science**. The use-case list is broad:

- **Applications** - web backends, CLIs, desktop, mobile, tools
- **Systems** - OS components, drivers, embedded, networking
- **Science** - simulation, numerical, aerospace, robotics, HPC
- **Games and game engines** - one workload among many
- **Compilers and developer tooling** - including Axolotl itself
- **Infrastructure** - distributed systems, databases, cryptography, servers

**Why Axolotl.** A language with a single domain is a niche language. A language that handles "everything" is a platform. Axolotl is the latter.

---

## P25. The "use the real Rust types" problem

**Problem.** Some languages invent a parallel type system (`AxolArc`, `AxolMutex`, `AxolVec`, `AxolFuture`, `AxolHashMap`). This bloats the runtime and creates a maintenance burden.

**Solution.** When you need an `Arc`, a `Mutex`, a `RefCell`, a `HashMap`, a `Vec`, a `Future` - you import the **real Rust type**:

```axol
use "std::sync::Arc"
use "std::sync::Mutex"
use "std::collections::HashMap"
```

Axolotl doesn't ship parallel types. The rule: **if Rust already provides it, Axolotl doesn't reinvent it unless there's a compelling language-level reason.**

**Why Axolotl.** This is the rule that keeps the generated Rust readable and the binary small. It's also the rule that prevents "Axolotl runtime" from creeping back in.

---

## P26. The "what should the language be called" problem

**Problem.** The user iterated through names - Luma, then Luma-rust, then Axolotle, finally **Axolotl**. Each rename marked a reframe of the language's identity.

**Solution.** The name is **Axolotl**. Not "Luma," not "Axolotle." The animal is the brand.

| Earlier name | What it meant | Why it was renamed |
|---|---|---|
| Luma | the first game-DSL prototype | killed when the language became general-purpose |
| Luma-rust | the first Rust-frontend version | killed when the user realized "Luma" was tied to a specific pivot |
| Axolotle | the user's first attempt at the new name | typo / misspelling |
| **Axolotl** | the final, correct name | the animal, the brand, the ecosystem |

**Why Axolotl.** The axolotl is famous for regenerating limbs (Regrow), staying juvenile (Neoten), and breathing through gills (Gills). The toolchain tells the axolotl's story.

---

## P27. The "JSONC vs TOML" problem

**Problem.** Cargo uses TOML for `Cargo.toml`. TOML is fine but is harder to read for most people than JSON.

**Solution.** `Bucket.jsonc` - JSON with comments. Bucket formats and lints it. The user can `// comment` their way through the manifest.

**Why Axolotl.** Easier to read, easier to write, easier to format, and the user explicitly asked for it.

---

## P28. The "languages don't matter, you have to actually build it" reality check

**Problem.** Designing a language is easy. Implementing a compiler, a build orchestrator, an LSP, a linter, a formatter, an auto-fixer, a doc generator, a test runner, a scaffolder, a migrator, a package registry, and a global cache is hard.

**Solution.** The architecture is designed to minimize the new-code surface:

- The **compiler** is a frontend, not a backend.
- The **build engine** is Cargo + rustc + LLVM.
- The **package ecosystem** is crates.io.
- The **target types** are Rust's standard types.
- The **async runtime** is whatever the user imports.
- The **FFI bindings** are generated, not handwritten.

What's left to build:

- The Axolotl lexer, parser, type checker, ownership inferencer, diagnostics engine, and Rust code generator.
- The Rust metadata importer.
- Bucket - the orchestrator, dependency resolver, cache manager, TUI, log system.
- Gills - the LSP server, basically a Rust Analyzer fork with Axolotl awareness.
- Neoten - the linter.
- Shed - the formatter.
- Regrow - the auto-fixer.
- Ambystoma - the doc generator.
- Salamander - the test runner.
- Larva - the scaffolder.
- Molt - the migrator.
- Eggbox - the registry (optional, can use crates.io initially).

**Why Axolotl.** The "what you don't reinvent" principle applies to the *implementation* as well. The hard parts (compiler backend, package ecosystem, target types) are inherited from Rust. The new parts (compiler frontend, toolchain, UX) are the differentiator.

---

## Why Axolotl exists - the synthesis

Pulling all the answers together:

1. **Rust's safety** is real. Ownership, borrowing, and lifetimes eliminate entire classes of bugs. Nobody else does this at the same level.

2. **Rust's ergonomics are a barrier.** The borrow checker, the lifetimes, the `impl` blocks, the `Result<T, E>` ceremony - these are real costs that limit who can use the language.

3. **Lua's ergonomics are the opposite.** The syntax is famously approachable. The semantics are simple. People who know Lua can read any Lua program.

4. **No existing language combines (1), (2), and (3) without compromise.** Lua is ergonomic but unsafe and GC'd. Rust is safe but hard. TypeScript is ergonomic but GC'd. C++ is fast but unsafe. Zig is fast and unsafe-by-default. Nim has a small ecosystem. Carbon is C++ in new clothes. Mojo is Python with extra steps.

5. **The Cargo ecosystem is enormous.** Tens of thousands of crates. A new language without this ecosystem is dead on arrival.

6. **The only way to get (1), (2), (3), and (5) is to compile a Lua-flavored language to Rust.** That is Axolotl. There is no other architecture that delivers all four.

7. **The "no runtime" rule is non-negotiable.** Without it, the language is just "another runtime language." With it, the language is honest about its costs and suitable for every workload Rust can do.

8. **The smart compiler is the differentiator.** Most languages stop at syntax. Axolotl treats the engineering experience as a first-class design concern. Gills, Neoten, Regrow, Shed, Ambystoma, Salamander - these aren't bolt-ons; they're the product.

9. **The Bucket / Pond / Gills / Neoten / Shed / Regrow / Ambystoma / Pond / Larva / Molt / Salamander / Eggbox toolchain is the engineering environment.** Cargo is the engine. Bucket is the control plane. The user lives in `bucket ...` and never types `cargo ...` unless they want to.

10. **The mixed `.axol` + `.rs` project model removes the adoption wall.** A team can introduce Axolotl into a Rust project file by file. Nothing that works stops working.

11. **The pnpm-style Pond cache fixes the 20-60 GB problem.** Identical compilation identities are reused across projects. Disk usage goes down, build times go up.

12. **The "if you didn't use it, you don't exist" rule keeps binaries small.** No C build machinery unless you used a `cblock`. No Python runtime unless you used a `pyblock`. No async unless you used `async`. No arena unless you used an arena. No dependency unless you imported it.

13. **The zero-overhead promise is honest.** The generated Rust is ordinary Rust. The binary contains exactly what the program needs and nothing more.

14. **The tool names tell the axolotl's story.** Regrow fixes code. Gills sense the project. Neoten enforces juvenile rigor. Shed sloughs off ugliness. Ambystoma documents. Pond stores. Bucket orchestrates. Larva scaffolds. Molt migrates. Salamander tests. Eggbox publishes.

15. **The all-purpose identity makes it viable long-term.** Games are one workload. Rocket science is on the list. So are databases, compilers, web servers, embedded, networking, scientific computing, desktop apps, CLIs, infrastructure, and developer tooling.

That's the answer to the umbrella problem. **Axolotl is the language that finally delivers Rust's safety, Lua's ergonomics, Cargo's ecosystem, and a smart compiler's productivity - without inventing a runtime, a backend, or a parallel ecosystem.**

Welcome.
