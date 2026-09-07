# IDEA - Axolotl

> **Axolotl (`.axol`)** is a high-level, all-purpose, memory-safe, zero-GC programming language whose programs compile to **idiomatic Rust** and ship as native binaries - with **no Axolotl runtime, no VM, no garbage collector, and no interpreter ever sitting underneath.**

---

## 1. The seed

The idea didn't start as "a new language." It started as a complaint:

> *"I want to write games (and later, anything) with Rust-level safety and Lua-level simplicity, but Rust makes me fight the borrow checker, and Lua makes me give up safety, performance, and the entire Cargo ecosystem."*

That tension is real. Lua is the most approachable language most game developers have ever used. Rust is the safest, fastest, best-tooled mainstream systems language. There is no language that gives you **both** without forcing you to choose.

Axolotl is the answer to that question: **what if Lua's surface sat on top of Rust's spine?**

---

## 2. The first prototype: Luma (game-only)

The first pass was a **game DSL** - `entity`, `game`, `update`, `draw`, `spawn`, `arena`, `frame` as keywords. The compiler was supposed to be its own thing: parse → typed AST → ownership analysis → ECS transformation → LLVM.

It worked on paper. It failed in practice. The moment you put `entity Player { ... }` into the grammar, you have **a scripting language**, not a general-purpose language. You can write a game. You cannot write a database, a compiler, an operating-system component, a flight controller, or a CLI tool. You also cannot call `wgpu` or `tokio` without writing FFI bindings for every crate you'll ever touch.

The user said it bluntly:

> *"It's horrible. It's not just for scripting. It should be on top of Rust, transpile to idiomatic Rust, and then compile."*

That was the kill shot for Luma-as-DSL. The language needs to be **general purpose from minute one**, and games are a library problem, not a grammar problem.

---

## 3. The first realignment: Luma as a Rust frontend

The second pass was much sharper:

```text
Luma → Luma AST → semantic analysis → idiomatic Rust → rustc → native binary
```

This was the moment the idea became credible. Instead of inventing a backend, optimizer, linker, package manager, and platform support, you **steal the entire Rust toolchain** and build a better surface language on top. The compiler is just a frontend. The hardest parts of running real software on real hardware are already solved.

But the user wasn't done:

> *"I need it to be compatible with Cargo crates."*

That requirement is the difference between "a fun weekend project" and "a language that could actually be adopted." Without crates.io compatibility, the language is dead on arrival - nobody will rewrite the Rust ecosystem for you. With it, every Cargo crate ever published is, on day one, an Axolotl library.

That pulled the architecture into its final shape: **Axolotl is a Cargo crate's peer, not its replacement.**

---

## 4. The first architecture debate: transpile vs. import

There were two ways to ship Cargo compatibility:

1. **Transpile to Rust source** and let Cargo compile that.
2. **Import Rust crate metadata** directly and emit calls into the Cargo build.

Transpile-to-source is the lazier path. It works for the first program. It falls apart the moment you want to consume a generic, trait-heavy, macro-using crate. You end up writing a parallel "Luma std library" to wrap every crate.

The right answer - and the one the user pushed toward - is to read Rust's `.rmeta` metadata and **call real Rust crates as if they were native Axolotl modules**. No wrapping. No porting. The same `wgpu`, the same `serde`, the same `tokio`, the same `axum` - but called from `.axol` source.

Source generation (`bucket emit-rust`) survives as a **debugging tool**, not the primary compilation path.

---

## 5. The Lua reframe

The third shift was bigger than any technical debate. The user said:

> *"It's not much different from Rust though."*

And they were right. The previous draft was Rust with prettier punctuation. `let`/`var`/`fn`/`impl`/`match` - same model, different surface. That isn't a new language. That's a theme.

The user clarified what they actually wanted:

> *"Bring Lua and Rust in head. Lua first. Bring Lua C compiler library, LuaJIT, and rustc in head. Remake Lua. Lua compiler that produces idiomatic Rust. Both are compatible. If you write Rust you should be able to import in Lua. If you write Lua you should be able to import in Rust."*

So the language is **Lua extended**, not Rust simplified. Tables survive (as statically inferred records), `function ... end` blocks survive, `if/then/elseif/else/end` survives, `:` method-call syntax survives, multiple returns survive. The semantics underneath are Rust's, but the **programmer never sees Rust unless they ask to**.

That was the reframe that made the language feel real.

---

## 6. The "high-level, all-purpose" correction

The user corrected me several times. The framing kept drifting toward "game language," and every time the user pulled it back:

> *"I said a million times it's not a game-development focused language. It's an all-rounder for everything including rocket science."*

So the final scope is:

- applications (web backends, CLIs, desktop, mobile)
- systems (OS components, drivers, embedded, networking)
- science (simulation, numerical, aerospace, robotics, HPC)
- games and game engines (one workload among many)
- compilers and developer tooling
- infrastructure, distributed systems, databases, cryptography

Games are **one workload**. Rocket science, literally, is on the list.

---

## 7. The "no runtime" rule

The user kept returning to one absolute:

> *"Axol should never ever have any runtime. That will break it."*

This is not a perf optimization. It is a **design constraint**. No Axolotl VM. No Axolotl GC. No Axolotl interpreter. No Axolotl scheduler. No Axolotl object system. No hidden heap. No boxed everything. No mandatory standard runtime.

If you need an `Arc`, you import Rust's `std::sync::Arc`. If you need a `Mutex`, you import Rust's `std::sync::Mutex`. If you need a `Vec`, you import Rust's `Vec`. If you need nothing, **Axolotl doesn't secretly inject something.** The compiler lowers the program into ordinary Rust and rustc does what rustc always does.

The generated Rust has to be **readable**. A Rust developer should be able to look at `bucket emit-rust` output and think "yeah, a Rust developer could have written this." No `axolotl_runtime::something_34891(...)` call sites. No `Box<dyn Any>` everywhere. Just code.

That's the rule that makes "0 overhead" honest.

---

## 8. The "smart compiler" requirement

The user didn't just want a compiler. They wanted a compiler that **helps**:

> *"Smartest compiler who will help you write code as well as manage the rest."*

That requirement pulled in:

- **Gills** - LSP / IDE intelligence (autocomplete, go-to-def, inline diagnostics, Axolotl↔Rust navigation)
- **Neoten** - a linter broader than Clippy, with Axolotl-specific checks
- **Regrow** - automatic fixes, refactoring, ownership-rewrite suggestions
- **Shed** - formatter
- **Ambystoma** - documentation generator
- **Salamander** - testing and benchmarking

The compiler itself must:

- recover from parse errors and keep analyzing
- show **Axolotl-level** reasons for failures, not raw rustc noise
- map generated-Rust spans back to Axolotl source so errors point at the line the user wrote
- suggest fixes that the user can apply with one keystroke
- have a live TUI build dashboard instead of `Compiling foo... Compiling bar...`

The smart compiler is the differentiator. Anybody can build a transpiler. Almost nobody builds the engineering environment around it.

---

## 9. The Bucket / Pond / Larva / Molt / Salamander / Eggbox toolchain

The user wanted more than `cargo build`. They wanted an entire project, dependency, build, packaging, migration, testing, and registry toolchain named after the axolotl's biology and habitat:

| Tool | Job |
|---|---|
| **`axolc`** | The compiler itself |
| **Bucket** | Project / build / dependency / package manager (above Cargo) |
| **Gills** | LSP / editor intelligence |
| **Neoten** | Linter and static analysis |
| **Shed** | Formatter |
| **Regrow** | Automatic fixes and refactors |
| **Ambystoma** | Documentation generator |
| **Pond** | The output / build artifact / cache directory (`pond/`, not `target/`) |
| **Larva** | `bucket new` - project scaffolding |
| **Molt** | Toolchain and project migration / upgrade |
| **Salamander** | Testing and benchmarking |
| **Eggbox** | Package publishing and registry |

These are not cute names for prototypes. They are a **cohesive developer ecosystem** where the user can live in `bucket ...` commands and never type `cargo` unless they explicitly want to.

---

## 10. The Pond cache (pnpm for Rust)

The user pointed at a real pain:

> *"Cargo is horrible. It compiles everything and puts in target. Sometimes that becomes 20–60 GB. Every crate should be stored in a global pond as cache instead of recompiling the same version."*

The fix is a pnpm-style global content-addressed cache:

```text
~/.bucket/pond/
├── store/        # global source + artifact store
├── objects/      # content-addressed
└── index/

project/pond/
├── debug/        # per-project build outputs (linked, not copied)
├── release/      # fresh production artifacts
└── logs/         # structured build logs
```

Bucket maintains the global Pond. Cargo does the actual compilation. But Cargo's `target/` becomes `pond/`, and identical compilation identities (source hash + features + target triple + rustc version + profile) are reused across every project on the machine.

`bucket build --release` deliberately does **not** inherit debug artifacts. Production gets a clean profile. Development gets aggressive reuse. Both share the same global store.

---

## 11. The magic blocks

The user wanted foreign-language escape hatches that **don't pollute projects that don't use them**:

```axol
cblock   { ... }   -- raw C, generates a build script only if present
cppblock { ... }   -- raw C++
rblock   { ... }   -- raw Rust
pyblock  { ... }   -- raw Python
```

If your project has no `cblock`, Bucket generates **zero C build machinery**. No `cc` configuration. No `build.rs` for C. No linker flags. Nothing.

That is the "what you never used doesn't exist" rule, applied to FFI.

---

## 12. The "if you didn't use it, you don't pay for it" principle

This became the **central invariant** of the entire project. It applies to:

- runtimes (no Axolotl runtime exists)
- foreign-language integrations (only generated if used)
- dependencies (only pulled in if imported)
- caches (linked, not copied)
- features (compile-time, never runtime)
- diagnostics (only the ones the user actually hit)
- build scripts (only generated when the project needs them)
- toolchain bloat (Cargo remains the engine; Bucket only adds the Axolotl layer)

Every feature has to defend what it costs at compile time, runtime, memory, and complexity. If it can't defend its existence, it doesn't ship.

---

## 13. The final identity

Pulling it all together, Axolotl is:

> **A high-level, all-purpose, memory-safe, zero-GC programming language with an exceptionally intelligent compiler, that compiles to idiomatic Rust, lives alongside Rust source in the same project, consumes the entire Cargo ecosystem, has no runtime of its own, and is wrapped in a project toolchain (Bucket + Pond + Gills + Neoten + Shed + Regrow + Ambystoma + Larva + Molt + Salamander + Eggbox) that makes building native software feel as approachable as writing Lua.**

That is the **end product** of every pivot, every reframe, every correction, and every new idea in the conversation.

What started as a hand-rolled game DSL ended as a language that can write a database, an OS component, a flight controller, a web server, a compiler, a CLI, a desktop app, a scientific simulator, **and** a game engine - without ever asking the programmer to give up safety, performance, or the Rust ecosystem underneath.

---

## 14. The enjoyable layer (the "match is too much code" reframe)

After the architecture was settled, the user came back with one more critique: the surface was still too verbose. Specifically, `match` - a powerful construct, but used everywhere, with a lot of ceremony for the common case.

> *"Match is okay but still who wants to write too much code. Why is there no other solution that achieves the same thing as match but less code."*

That single question pulled a whole new layer into the language. The result is a family of **match replacements** (4 forms, all compiling to the same Rust `match`) and a dozen other ergonomic features - `let-else`, pipe `|>`, named arguments, default values, spread, destructuring, if-let chains, comprehensions, method cascading, multi-line / raw / tagged strings, number-suffix literals, doc comments, inline tests, auto-derived traits, quick constructors, struct update syntax.

Every one of them is sugar - they compile to the same idiomatic Rust as their verbose equivalents. Every one has a verbose form the user can drop down to. The high-level layer is opt-in, not mandatory. Beginners use the sugar; advanced users reach for the verbose forms when clarity demands it.

The full design: `HighLevel.md`. The full reference: Sections 71-91 of `PROGRAMMINGLANGUAGEBIBLE.md`.

---

## 15. The LSP / cross-language IDE reframe

The user also pointed at a real adoption problem with mixed projects: default rust-analyzer doesn't understand `.axol`. When you `use crate::server::Server;` in a `.rs` file, rust-analyzer finds `Server` in the generated Rust but has no idea the user wrote it in `server.axol`. Cross-language navigation, refactoring, and diagnostics are broken.

> *"default rust-analyzer will not work like if u put .rs alongside .axol Bucket, axol analyzer will know and send lsp diagnosis suggestions etc but rust analyzer doesn't know if u try to import something that was writen in .axol it will complain yk what i mean?"*

The answer: **Gills**, a fork of rust-analyzer that:

- Understands both `.axol` and `.rs` in the same project.
- Maintains a **unified symbol graph** with both source and generated spans.
- Always returns the **Axolotl source** for go-to-def, hover, references.
- Remaps diagnostics to Axolotl spans.
- Refactors across both languages (rename, extract, inline, signature).
- Surfaces `axolc` and `Neoten` diagnostics with full source mapping.
- Speaks standard LSP - works with every editor.
- Adds Axolotl-specific extensions (live diagnostic diff, cross-language rename, match preview, arena suggestions, source↔generated navigation).

Full architecture, capabilities, configuration, editor integrations: `LSP.md`.

---

## 16. The complete identity (final)

Pulling everything together, Axolotl is:

> **A high-level, all-purpose, memory-safe, zero-GC programming language with an exceptionally intelligent compiler, an enjoyable Lua-and-TypeScript-flavored surface, native compilation through idiomatic Rust, full Cargo ecosystem compatibility, no Axolotl runtime, mixed `.axol` + `.rs` projects, opt-in foreign-language blocks, a global pnpm-style Pond cache, a smart compiler toolchain (Gills / Neoten / Shed / Regrow / Ambystoma / Salamander / Larva / Molt / Eggbox), and a unified IDE (Gills) that knows both languages.**

Three layers:

1. **Spine** - Rust's semantics, Cargo's ecosystem, no runtime.
2. **Surface** - Lua's syntax, TypeScript-style types, high-level enjoyable features.
3. **IDE** - Gills, the rust-analyzer fork that makes both languages feel like one.

The user lives in `bucket ...` commands, edits `.axol` and `.rs` files in the same project, gets cross-language completions and diagnostics, runs inline tests with `Salamander`, formats with `Shed`, lints with `Neoten`, fixes with `Regrow`, documents with `Ambystoma`, publishes with `Eggbox`, and never touches Cargo unless they want to.

That's the language. Welcome.
