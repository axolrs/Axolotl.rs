# Concepts - Axolotl

This document captures every language, semantic, and ecosystem concept that was decided during the Axolotl design conversation. Each concept is named, motivated, and tied to either the Lua heritage, the Rust heritage, or the new layer Axolotl adds on top.

---

## 0. The three philosophical pillars

1. **Lua-like surface** - approachable syntax, lightweight grammar, table-style data.
2. **Rust-like spine** - memory safety, ownership, zero-cost abstractions, no GC.
3. **Toolchain above Cargo** - Bucket orchestrates everything; rustc does the heavy lifting; crates.io is the ecosystem.

Everything else in this document is a consequence of those three.

---

## 1. The file extension and the language name

| Concept | Decision |
|---|---|
| Language name | **Axolotl** (not "Axolotle", not "Luma") |
| Source extension | **`.axol`** |
| Project repository | `Axolotl.rs` (the project itself, not the language) |
| Compiler | **`axolc`** |
| Project tool | **`bucket`** |
| Build directory | **`pond/`** (not `target/`) |
| Project manifest | **`Bucket.jsonc`** (JSONC, not TOML) |
| Lockfile | **`bucket.lock`** |
| Configuration sub-ext | `.jsonc` (JSON with comments) |

The choice of `.axol` is deliberate: short, recognizable, distinct from Rust's `.rs` and Lua's `.lua`, and instantly greppable.

---

## 2. The "no runtime" concept

**There is no Axolotl runtime.** Not optional. Not minimal. None.

The full set of things that do not exist at runtime:

```text
❌ Axolotl VM
❌ Axolotl interpreter
❌ Axolotl GC
❌ Axolotl scheduler
❌ Axolotl object system
❌ Axolotl heap
❌ Axolotl boxed-value machinery
❌ Axolotl reflection runtime
❌ Axolotl standard runtime
❌ Axolotl prelude injected into every binary
```

If a runtime concept is needed, the user imports it from a Rust crate. The program compiles to whatever the generated Rust plus the user's explicit dependencies require - and **only** those things.

This is a **design constraint**, not a perf optimization. It is what makes "0 overhead" an honest claim instead of marketing.

---

## 3. The "what you didn't use, doesn't exist" principle

This is the central invariant. It applies to:

| Domain | Rule |
|---|---|
| Runtimes | none unless imported |
| Foreign blocks | only generated build machinery for the blocks present |
| Dependencies | only pulled in if imported |
| Caches | reused via links, not copies |
| Build scripts | only generated when needed |
| Features | compile-time, not runtime |
| Toolchain | Cargo stays; Bucket adds only what Axolotl needs |
| Diagnostic output | only the ones the user actually hit |

Every feature has to defend its cost in compile time, runtime, memory, and complexity. If it can't, it doesn't ship.

---

## 4. The transpile-to-Rust vs. import-Rust-metadata distinction

There are two ways to give a language Cargo compatibility:

| Approach | Reality |
|---|---|
| **Transpile to Rust source** | Works for the first crate. Fails on generics, trait-heavy APIs, async traits, procedural macros, associated types, higher-ranked lifetimes, unsafe APIs, macro-syntax expectations, closure types, exotic ownership patterns. Forces you to write a parallel "Axolotl std" wrapping every crate. |
| **Import Rust metadata** | Read `.rmeta` directly. The Axolotl compiler understands the real Rust API of the real crate and generates calls into it. No wrappers. `wgpu` is the same `wgpu`. |

Axolotl chose the second. Source generation is kept only as `bucket emit-rust` for debugging - never the primary path.

---

## 5. The Lua-extended surface

The grammar borrows heavily from Lua, then layers TypeScript-style ergonomics on top:

| Lua concept | Axolotl equivalent | Notes |
|---|---|---|
| `if/then/elseif/else/end` | kept verbatim | control flow feels Lua |
| `while/do/end` | kept | loops |
| `repeat/until` | kept | post-test loops |
| `for k,v in pairs(t) do ... end` | `for k, v in t do ... end` | generic iteration |
| numeric `for i = 1, 10 do ... end` | kept | numeric loops |
| `function ... end` | kept | function literals |
| `local x = ...` | `local x` (immutable), `var x` (mutable) | the only variable modifiers |
| `:` method syntax | kept | `player:damage(20)` is sugar for `player.damage(player, 20)` |
| `.` field access | kept | structs and records |
| `[...]` index | kept | arrays, maps |
| table constructors `{ k = v, ... }` | kept, **statically typed** | the central data construct |
| `return a, b, c` | kept | multiple return values |
| anonymous functions | kept | first-class closures |
| `nil` | **replaced** with explicit `null` only for `T?` types | see nullable concept below |
| require | `use "crate"` or `use "module"` | unifies Lua's require and Rust's use |
| metatables | **replaced** with statically resolved operator overloading | no runtime dispatch |
| `coroutine.create` | replaced with `task` blocks / `spawn` / `await` | compiles to Rust futures |

Lua's **tables** survive but become **statically inferred records** - `{ name = "Mohi", age = 22, active = true }` infers `{ name: String, age: Int, active: Bool }` without an explicit type. When you want stable layout / ABI, you write a `struct` instead.

---

## 6. The Rust-extended spine

Every concept in this section is lifted from Rust but exposed through Axolotl syntax:

| Concept | Axolotl form |
|---|---|
| Static typing | optional / inferred; explicit when desired |
| Type inference | default; very aggressive |
| Generics | `fn identity<T>(x: T) -> T`; usually inferred |
| Traits | `interface ... end` |
| Trait implementation | `impl Foo for Bar ... end` |
| Structs | `struct ... end` |
| Enums (algebraic) | `enum ... end` with variants `Name`, `Name(x)`, `Name { x: T }` |
| Pattern matching | `match value { Pattern => ... }` |
| `Option<T>` | `T?` |
| `Result<T, E>` | `T ! E` |
| Ownership | inferred by default; explicit `borrow`/`mut`/`move` annotations available |
| Borrowing | `borrow T` (read) / `mut T` (write) |
| Move semantics | `move T` (explicit) |
| Lifetimes | **inferred**; never written by the user |
| `&T`, `&mut T` | `ref T`, `mutref T` (advanced escape hatch) |
| `*const T`, `*mut T` | `unsafe` block with `raw Pointer<T>` |
| Async/await | `async fn ... await ...` |
| Threads | `spawn fn ... end` |
| Channels | from `std::sync::mpsc` or `tokio::sync` |
| `unsafe` | `unsafe ... end` block, or `unsafe fn` |
| FFI | `cblock`, `cppblock`, `rblock`, `pyblock` (only generated if used) |
| Procedural macros / derive | `@derive(Serialize, Deserialize)` |
| Attributes | `@attr` syntax, mapped to `#[attr]` |
| `const` evaluation | `const NAME = expr` when expr is compile-time evaluable |
| `repr(C)` | `@repr(C)` for FFI-compatible layout |
| `Send` / `Sync` | inferred; no annotation required |
| Closures | `fn(x) ... end` literal; captures inferred |
| `Box<T>`, `Arc<T>`, `Rc<T>` | imported from Rust; no Axolotl equivalent |
| `Mutex<T>`, `RefCell<T>` | imported from Rust; no Axolotl equivalent |
| `Vec<T>` | `Array<T>` (Axolotl alias) or `Vec<T>` directly |
| `HashMap<K, V>` | `Map<K, V>` (Axolotl alias) or `HashMap<K, V>` directly |
| `String` / `&str` | `String` for owned, inferred slices |
| Iterators | `for x in collection do ... end` over anything that implements the `IntoIterator` interface |
| `panic!` | mapped to Rust `panic!` |
| `println!` | `print(...)` with implicit `Display` |
| `format!` | `format("...{}...", x)` |
| `dbg!` | mapped to Rust `dbg!` |

---

## 7. The nullable concept

Rust has **no nullable types** in the traditional `null` sense. It uses `Option<T>` to make absence explicit. Axolotl adds a small syntactic layer on top:

```axol
name: String? = null   -- Option<String> = None
name = "Mohi"          -- Some("Mohi".to_string())
```

Rules:

- `x: T?` is `Option<T>`. `null` is a valid value.
- `x: T` **can never be null**. The compiler guarantees it.
- `null` is **not** a universal inhabitant of every type. It only exists in `T?` contexts.
- `user?.name` is **optional chaining** - produces another nullable value.
- `user!.name` is "I guarantee this isn't null" - inserts the appropriate unwrap/expect.
- `user?.name ?? "Anonymous"` is null coalescing - produces the value or a default.

Underneath, the compiler emits `Option<T>` and Rust's existing machinery. There is no nullable pointer. There is no null-deref class of bug. The syntax is ergonomic; the semantics are Rust's.

---

## 8. The ownership concept (Axolotl's hardest design problem)

Axolotl keeps Rust's ownership semantics but **infers** them by default. There are four levels of explicitness:

| Level | Form | Audience |
|---|---|---|
| **L1 - Inferred** | `fn update(player, dt) ... end` | beginners, app code, gameplay code |
| **L2 - Annotated** | `fn update(player: borrow Player, dt: Float)` | engine code, library authors |
| **L3 - Mut / Move explicit** | `fn consume(value: move Buffer)` | systems code, FFI |
| **L4 - Unsafe escape** | `unsafe { ptr = ... } end` | low-level / hardware / FFI |
| **L5 - Raw Rust** | `rblock { let p: *mut u8 = ... }` | when you need the actual Rust |

The compiler's job at L1 is to determine, from usage:

- is this value copied?
- is it moved?
- is it borrowed immutably?
- is it borrowed mutably?

If the compiler can prove the operation is safe, it generates the right `&T` / `&mut T` / move automatically. If the compiler **cannot** prove it, it asks the user, rather than silently picking the wrong semantics. That is the difference between "inferred" and "guessed."

Three explicit keywords bridge L1 and L4:

- `borrow T` → `&T`
- `mut T` → `&mut T` (read-write borrow)
- `move T` → takes ownership

These exist so an advanced programmer can take over without leaving the language.

---

## 9. The four-level memory model

For allocations and lifetimes:

| Level | Form | Lifetime |
|---|---|---|
| Stack | normal local `let x = ...` | scope of declaration |
| Heap | `Box.new(...)` from Rust | until dropped |
| Arena | `arena` block; `arena.create(...)` | until `arena.clear()` |
| Persistent | `static` / program lifetime | entire process |

Arenas are first-class because **games and simulations need them**. Frame arenas, level arenas, network-message arenas - all map onto Rust's `bumpalo` / `slotmap` / custom arena crates.

The `arena` block is only generated when used. A pure CLI tool that never asks for an arena has no arena code in the binary.

---

## 10. The interface / trait concept

```axol
Drawable = interface
    draw(self, renderer: Renderer)
end

Player : Drawable

Player.draw = fn(self, renderer)
    renderer.sprite(self.texture, self.position)
end
```

Underneath this is a Rust trait:

```rust
trait Drawable {
    fn draw(&self, renderer: &Renderer);
}

impl Drawable for Player {
    fn draw(&self, renderer: &Renderer) {
        renderer.sprite(&self.texture, self.position);
    }
}
```

The compiler does the translation. The Axolotl interface may also be implemented in `.rs` files, and Axolotl can consume an `impl Trait for ForeignType` defined in Rust directly.

---

## 11. The pattern matching concept

```axol
match state
    Idle => idle()
    Running(speed) => move(speed)
    Dead => die()
end
```

Algebraic data types from Rust, ergonomics from Lua. Exhaustive matching is enforced - the compiler refuses to compile a `match` that doesn't cover every variant.

---

## 12. The error handling concept

Three idiomatic forms, all compiling to Rust `Result` / `Option`:

| Form | What it does |
|---|---|
| `value = may_fail()?` | propagates the error up (early return) |
| `value = may_fail() ?? default` | provides a fallback if null/None |
| `try { ... } catch e { ... } end` | explicit error block |

Plus a Lua-style "multiple return" form for familiar code:

```axol
value, err = database.query(...)
if err then
    print(err)
end
```

The multiple-return form is desugared to a `Result` underneath. Both styles are first-class.

---

## 13. The concurrency concept

Axolotl inherits Rust's concurrency model - but the syntax stays high-level:

```axol
task = spawn async fn()
    response = await http.get(url)
    return response
end

result = await task
```

Underneath, this becomes ordinary Tokio / futures / `std::thread::spawn` code. No Axolotl scheduler exists.

The compiler enforces `Send` / `Sync` where required, but the programmer doesn't write the bounds.

Channels come from `std::sync::mpsc` or `tokio::sync::mpsc`, imported directly.

---

## 14. The compile-time / metaprogramming concept

Three layers, all useful:

1. **Constants** - `const MAX = 1024`; compile-time evaluable.
2. **Derives** - `@derive(Serialize, Clone, Debug, ...)` mapped to `#[derive(...)]`.
3. **Reflection (limited)** - `@reflect` for compile-time type metadata; still no runtime reflection.

Procedural macros from Rust crates are consumable directly. If a Rust crate ships a `#[derive(YourMacro)]`, Axolotl can apply it via `@derive(YourMacro)`.

---

## 15. The mixed `.axol` + `.rs` workspace concept

A project can have **both** languages in `src/`:

```text
my-project/
├── Bucket.jsonc
├── bucket.lock
├── Cargo.toml
│
├── src/
│   ├── main.axol
│   ├── server.axol
│   ├── database.axol
│   │
│   ├── crypto.rs        -- pure Rust
│   ├── protocol.rs      -- pure Rust
│   └── hardware.rs      -- pure Rust
│
└── pond/
```

Rules:

- Axolotl can `use "crypto"` and call into the Rust file's public API.
- Rust can `use crate::server` and consume the Axolotl-generated module.
- Cargo's CLI doesn't know `.axol` exists - `cargo build` only sees the `.rs` files. Bucket is what makes the mixed project work.
- The boundary is a **normal Rust ABI / type boundary**, not a foreign-function boundary.

This is the concept that makes adoption painless: a team can introduce Axolotl into a Rust project one file at a time.

---

## 16. The foreign-language block concept

Four escape hatches, each conditionally compiled:

| Block | What it does | Generated only when present |
|---|---|---|
| `cblock { ... }` | embeds raw C | yes - generates `build.rs`, `cc = "1"` config, linker flags |
| `cppblock { ... }` | embeds raw C++ | yes - same as above with `cpp` |
| `rblock { ... }` | embeds raw Rust | yes - appended into generated Rust |
| `pyblock { ... }` | embeds raw Python | yes - adds Python runtime + bindings |

If a project has no `cblock`, the build produces **no C toolchain configuration**, **no `cc` crate**, **no `build.rs` for C**. The same for the other blocks. The "what you didn't use doesn't exist" principle in action.

---

## 17. The dependency concept

Dependencies live in `Bucket.jsonc`:

```jsonc
{
    "name": "my-project",
    "version": "0.1.0",
    "language": { "edition": "2026" },
    "dependencies": {
        "wgpu": "^27",
        "serde": "^1",
        "tokio": "^1"
    }
}
```

Bucket resolves them in two layers:

```text
Bucket.jsonc
   ↓
Bucket resolver
   ↓
┌─────────────┐
│ Axol deps   │  (Axolotl packages, future)
│ Rust deps   │  (Cargo crates, day one)
│ Foreign deps│  (system libs, future)
└─────────────┘
   ↓
Cargo.toml (auto-generated)
   ↓
Cargo + crates.io
```

The user adds a dependency with `bucket add wgpu` and never touches TOML.

---

## 18. The Pond cache concept

A pnpm-style global content-addressed cache:

```text
~/.bucket/pond/
├── store/
│   ├── sources/        # crate source: serde@1.0.219/, tokio@1.47.1/, ...
│   └── artifacts/      # compiled rlibs keyed by content hash
├── objects/            # content-addressed blobs
├── index/              # which project needs which artifacts
└── cache/              # persistent compiler state

project/pond/
├── debug/              # per-project build outputs (linked, not copied)
├── release/            # fresh production artifacts
├── generated/          # generated Rust
├── native/             # C/C++ outputs
└── logs/               # structured build logs
```

The artifact key is comprehensive:

```text
ArtifactKey = source hash
            + dependency graph hash
            + feature set
            + rustc version
            + target triple
            + profile (debug / release)
            + relevant compiler flags
            + environment fingerprint
```

Same key → reuse. Different key → recompile. This is what makes 30 projects using the same `serde 1.0.219` not all duplicate the 800 MB.

`bucket build --release` does **not** inherit debug artifacts. Release gets a clean profile; debug gets aggressive reuse. Both share the same global store.

---

## 19. The build log concept

Every build produces two files in `pond/{profile}/logs/`:

1. **`.log`** - human-readable complete build transcript.
2. **`.jsonl`** - structured events for tooling, CI, IDEs.

Plus metadata:

```text
Git revision
Bucket version
Axolotl compiler version
Rust version
Cargo version
target triple
profile
features
dependency lock hash
environment fingerprint
build duration
cache statistics
```

`bucket logs` opens a TUI showing recent builds; `bucket logs --errors` extracts diagnostics; `bucket logs --latest` opens the most recent. Failed builds are never thrown away - `bucket explain errors` reopens them.

The fancy build TUI is **ephemeral UX**. The log is the **source of truth**.

---

## 20. The Bucket CLI concept

```bash
bucket new           # create a new project
bucket init          # adopt an existing folder
bucket add           # add a dependency
bucket remove        # remove one
bucket update        # refresh the lockfile
bucket upgrade       # migrate to a new edition
bucket tree          # dependency tree
bucket search        # search registry

bucket build         # build (debug, cached, incremental)
bucket check         # type-check only
bucket run           # build + run
bucket test          # run tests
bucket bench         # run benchmarks

bucket fmt           # run Shed
bucket lint          # run Neoten
bucket fix           # run Regrow

bucket doc           # run Ambystoma
bucket doc --open    # open in browser

bucket clean         # nuke project/pond
bucket cache clean   # gc the global pond
bucket doctor        # health check
bucket logs          # view build history

bucket publish       # publish to Eggbox
bucket package       # build a release artifact

bucket watch         # Larva / live-reload
bucket install       # install a binary
bucket uninstall     # remove it

bucket cargo ...     # forward to Cargo
bucket rust ...      # forward to rustc

bucket emit-rust     # dump the generated Rust for inspection
```

`bucket cargo <anything>` is a transparent passthrough. The user never loses the underlying Cargo / rustc surface.

---

## 21. The smart compiler concept

The compiler doesn't just produce diagnostics - it **helps**:

- recovers from parse errors and continues analysis
- collects all errors in one pass instead of one-at-a-time
- explains **Axolotl-level reasons** for failures, not raw rustc text
- maps generated-Rust spans back to Axolotl source spans
- suggests fixes the user can apply with one keystroke
- shows ownership as a visual diff (`- renderer.draw(player)` / `+ renderer.draw(&player)`)
- has a live TUI build dashboard with parallel activity, not `Compiling foo... Compiling bar...`
- supports `--plain` for CI and `--json` for tooling

It catches what it can **before** generating Rust, so most diagnostics are Axolotl's, not rustc's. Rust errors are the last mile, not the primary signal.

---

## 22. The toolchain concept (axolotl-biology names)

| Tool | Job | Real Axolotl biology |
|---|---|---|
| **`axolc`** | The compiler | the animal itself |
| **Bucket** | Project / build / dependency / package manager | the bucket that holds everything |
| **Gills** | LSP / IDE intelligence | axolotls breathe through external gills |
| **Neoten** | Linter / static analysis | axolotls stay juvenile (neoteny) |
| **Shed** | Formatter | shedding old skin |
| **Regrow** | Automatic fixes / refactoring | axolotls famously regenerate limbs |
| **Ambystoma** | Documentation generator | the axolotl's scientific genus |
| **Pond** | Build / output / cache directory | axolotls live in ponds |
| **Larva** | `bucket new` - project scaffolding | the larval form |
| **Molt** | Toolchain / project migration | shedding old code for new code |
| **Salamander** | Testing + benchmarking | axolotls are salamanders |
| **Eggbox** | Package publishing / registry | where the eggs go |

These names aren't decoration. They let the user say "Gills says..." or "Neoten flagged..." and everyone knows what's meant.

---

## 23. The "writes the Rust a Rust dev would write" concept

When the user runs `bucket emit-rust`, the output must look like code a Rust developer wrote:

```rust
let mut player = Player {
    position: Vec3::ZERO,
    health: 100,
};

impl Player {
    pub fn damage(&mut self, amount: i32) {
        self.health -= amount;
    }
}
```

Not:

```rust
let tmp_384 = axolotl_runtime::something(...);
```

This matters because:

1. Debugging - when something goes wrong, the user can read the generated Rust.
2. Trust - the user can audit the compiler.
3. Interop - the generated Rust is consumable by ordinary Rust.
4. Adoption - Rust developers will trust a tool that produces Rust they recognize.

This is what "idiomatic" means in "compiles to idiomatic Rust."

---

## 24. The "use the real Rust types" concept

When you need `Arc`, `Mutex`, `RefCell`, `HashMap`, `Vec`, `Future` - you import them from Rust:

```axol
use "std::sync::Arc"
use "std::sync::Mutex"
use "std::collections::HashMap"

counter = Arc.new(Mutex.new(0))
```

Axolotl doesn't ship parallel `AxolArc`, `AxolMutex`, `AxolRefCell`, `AxolVec`, `AxolHashMap`, `AxolFuture`. The rule is:

> *If Rust already provides it, Axolotl doesn't invent a parallel implementation unless there is a compelling language-level reason.*

That's the rule that keeps the runtime out of the binary.

---

## 25. The zero-overhead concept

"Zero overhead" has one precise meaning:

> **Axolotl introduces no inherent runtime overhead compared with equivalent idiomatic Rust.**

So if `player.health -= damage` lowers to `player.health -= damage;`, there is no reason for an Axolotl runtime call to exist. Likewise `for item in items do process(item) end` becomes an ordinary Rust iterator / loop. There is no:

```text
Axolotl VM
GC
dynamic dispatch
reflection
boxed everything
hidden runtime
```

unless the programmer explicitly requests a feature requiring it.

That's the "0 overhead" promise, made precisely.

---

## 26. The "where Axolotl doesn't reach, drop into Rust" concept

For exotic things - Rust-specific syntax, procedural macros with DSL arguments, complex trait plumbing - drop into `rblock`:

```axol
rblock {
    #[inline]
    pub fn fast_math(x: f32) -> f32 {
        x.sqrt()
    }
}
```

The block becomes part of the generated Rust crate directly. No FFI, no wrapping. Just Rust in the middle of Axolotl.

---

## 27. The configuration format concept

`Bucket.jsonc`, not `Bucket.toml`.

- JSONC = JSON with comments.
- Easier for most people to read and write than TOML.
- `bucket fmt` and `bucket lint` can format and validate it.
- The user can `// comment` their way through the manifest.

The manifest itself is **usage-driven**: it has the dependencies and config Bucket needs. The generated `Cargo.toml` is an internal artifact that Cargo sees, not the user.

---

## 28. The final concept: ecosystem, not language

Axolotl is not a language in isolation. It's:

```text
.axol source
   ↓
axolc (the compiler)
   ↓
idiomatic Rust
   ↓
bucket (the orchestrator)
   ↓
Cargo + rustc + LLVM
   ↓
native binary
```

with:

```text
Gills         (LSP / IDE)
Neoten        (linter)
Shed          (formatter)
Regrow        (auto-fix)
Ambystoma     (docs)
Salamander    (test/bench)
Larva         (scaffolding)
Molt          (migration)
Eggbox        (registry)

Pond          (cache + build dir)
Bucket.jsonc  (manifest)
bucket.lock   (lockfile)
```

The user lives in `bucket ...` commands, points their editor at Gills, fixes what Neoten flags, runs tests through Salamander, publishes to Eggbox, and **never touches Cargo unless they want to**.

That's the ecosystem concept, fully assembled.

---

# Part II - The Enjoyable Layer (High-Level Concepts)

The previous 28 concepts define the **spine** of Axolotl: what makes it safe, fast, and ecosystem-compatible. The next set of concepts define the **enjoyable layer**: what makes it a *new* language rather than a theme. Every concept here is sugar - it compiles to ordinary idiomatic Rust and has a verbose equivalent the user can drop down to.

Full design and rationale: `HighLevel.md`. Full reference (one section per feature): see Sections 71-91 of `PROGRAMMINGLANGUAGEBIBLE.md`. Cross-language IDE story: `LSP.md`.

---

## 29. The high-level philosophy

Three rules govern every feature in this part:

1. **Less code for the same meaning.** If two features express the same intent and one is shorter without losing clarity, the shorter one wins.
2. **Compile to the same Rust.** Every high-level feature lowers to the same idiomatic Rust as its verbose equivalent. Zero overhead.
3. **Optional everywhere.** Every high-level feature has a verbose equivalent. Beginners use the high-level forms; advanced users drop to verbose forms when clarity demands it.

The high-level features are not a separate "easy mode." They're sugar over the spine.

---

## 30. The four match replacements

`match` is a powerful but verbose construct. The user asked: *"Match is okay but still who wants to write too much code. Why is there no other solution that achieves the same thing as match but less code?"* There are four.

| Form | Verbosity | Best for |
|---|---|---|
| **`case`** | terse - `of X: body` | One-off dispatch with simple arms |
| **Variant-handler methods** | method-per-variant | Reused dispatch logic; open-closed style |
| **Dispatch tables** | Lua-style literal | Ad-hoc dispatch specific to one call site |
| **`match`** | full form | Complex patterns, guards, expression-position use |

All four compile to the same Rust `match`. All four are exhaustively checked. Pick the one that reads best. See `HighLevel.md` Section 2 and Bible Sections 71-74 for the full story.

---

## 31. The `let-else` concept

Linear early-return patterns, no nesting:

```axol
let Some(user) = find_user(id) else return default_user()
let Ok(data) = read_file(path) else return
let Some(name) = user?.name else "Anonymous"
```

The `else` body is a single expression. Returns from the enclosing function when the pattern fails. See `HighLevel.md` Section 3 and Bible Section 75.

---

## 32. The pipe concept

`x |> f` is `f(x)`. Data flows top-to-bottom instead of inside-out:

```axol
let result = data |> parse |> validate |> transform |> save
```

Compiles to a plain function call. Zero overhead. Reverse form `<|` available for first-argument styles. See `HighLevel.md` Section 4 and Bible Section 76.

---

## 33. The named-argument concept

Function calls can name their arguments in any order:

```axol
spawn_actor(name = "Hero", health = 200, team = "red")
```

Self-documenting at the call site. Compiler verifies required fields, no duplicates, valid names, type compatibility. See `HighLevel.md` Section 5 and Bible Section 77.

---

## 34. The default-value concept

Parameters can have defaults; callers can omit them:

```axol
fn create_window(title = "Untitled", width = 800, height = 600, resizable = true, fullscreen = false, vsync = true)
```

Compiler synthesizes a builder internally. Defaults evaluated at call time. Defaults can reference earlier parameters. See `HighLevel.md` Section 6 and Bible Section 78.

---

## 35. The spread concept

`...expr` expands in function calls, table literals, array literals, and struct literals:

```axol
let defaults = { color = "red", size = 10, weight = "normal" }
let custom = { ...defaults, size = 20 }

let base = Player { name = "Hero", health = 100, ... }
let harder = base{ health = 200, speed = 10.0 }   -- shorthand update
```

See `HighLevel.md` Section 7 and Bible Section 79.

---

## 36. The destructuring concept

A `let` can bind multiple variables by pulling fields out of structs, elements out of tuples, entries out of maps:

```axol
let { name, health } = player
let (x, y, z) = position
let { address: { city, country } } = user
```

Works in `let`, function parameters, and `for` loops. See `HighLevel.md` Section 8 and Bible Section 80.

---

## 37. The if-let-chain concept

Combine multiple `if let`s with `and` to form a single condition. Short-circuits on the first failure:

```axol
if let Some(user) = find(id) and let Some(name) = user.name and name ~= "" then
    print("hello, ${name}")
end
```

Removes the deep nesting of "if let Some(x) { if let Some(y) { ... } }" patterns. See `HighLevel.md` Section 9 and Bible Section 81.

---

## 38. The comprehension concept

Python-style syntax for building collections from iterators:

```axol
let alive = [e for e in enemies if e.health > 0]
let by_name = { e.name: e for e in entities }
let unique_tags = { e.tag for e in entities }
let pairs = [(a, b) for a in first for b in second]
let pages = [await page async for page in fetch_all_pages()]
```

Compiles to Rust iterator chains. See `HighLevel.md` Section 10 and Bible Section 82.

---

## 39. The method-cascading concept

Call multiple methods on the same value without repeating the receiver:

```axol
let result = player
    :update(dt)
    :take_damage(20)
    :heal(10)
```

The leading `:` makes the receiver implicit. Compiles to ordinary Rust method calls. See `HighLevel.md` Section 11 and Bible Section 83.

---

## 40. The string-literal concept

Four string forms for different needs:

| Form | Use |
|---|---|
| `"normal"` | Single-line, escape sequences |
| `"""multi-line"""` | Multi-line, auto-strip indentation |
| `r"raw"` | No escape sequences (regexes, paths) |
| `tag"prefix"` | Call a function with the string (HTML, SQL, Markdown) |

Plus `b"bytes"` for byte arrays and `${expr}` for interpolation. See `HighLevel.md` Section 12 and Bible Section 84.

---

## 41. The number-suffix concept

Numeric literals with human-readable suffixes:

```axol
let buffer_size = 4_KB
let timeout = 30_sec
let frame_budget = 16_ms
let discount = 0.5_pct
```

The compiler maps to ordinary integer / float / Duration constants. Users can define custom suffixes with `const unit NAME = expr`. Zero runtime cost. See `HighLevel.md` Section 13 and Bible Section 85.

---

## 42. The doc-comment concept

Triple-dash comments attach documentation to declarations:

```axol
--- Adds two numbers.
---
--- @param a the first number
--- @param b the second number
--- @return their sum
--- @example
---   add(1, 2)        -- 3
fn add(a: Int, b: Int) -> Int
    a + b
end
```

`Ambystoma` processes these. Gills shows them in hovers. Salamander can run examples as tests. Full Markdown supported. See `HighLevel.md` Section 14 and Bible Section 86.

---

## 43. The inline-test concept

Test code lives next to the code it tests:

```axol
fn add(a: Int, b: Int) -> Int
    a + b
end

test add
    assert add(1, 2) == 3
    assert add(0, 0) == 0
end
```

`Salamander` runs them. Gills surfaces them in the editor (green checkmarks, red Xs, "Run test" code lenses). Attributes include `@should_panic`, `@ignore`, `@timeout`. See `HighLevel.md` Section 15 and Bible Section 87.

---

## 44. The auto-derive concept

The compiler auto-implements `Copy`, `Clone`, `Debug`, `Default`, `Eq`, `PartialEq`, `Hash`, `Send`, `Sync` for types where all fields implement them. Opt out with `@no_auto_impl(Trait)`. See `HighLevel.md` Section 16 and Bible Section 88.

---

## 45. The quick-constructor concept

Every struct gets an auto-generated `Type:new` constructor. Fields with defaults are optional. Named arguments work. The user can override by defining their own `new`. See `HighLevel.md` Section 17 and Bible Section 89.

---

## 46. The struct-update concept

Create a new struct from an old one, overriding specific fields:

```axol
let boss = player{ health = 200, name = "Boss " .. player.name }
```

The shorthand `old{ field = value, ... }` is sugar for "copy all fields, then override the listed ones." See `HighLevel.md` Section 18 and Bible Section 90.

---

## 47. The LSP concept - Gills (axol-analyzer)

The user pointed at a real problem: *"default rust-analyzer will not work like if u put .rs alongside .axol Bucket, axol analyzer will know and send lsp diagnosis suggestions etc but rust analyzer doesn't know. if u try to import something that was writen in .axol it will complain."*

Gills is a **fork of rust-analyzer**, not a wrapper. It understands both `.axol` and `.rs` in the same project. Cross-language navigation jumps to the **Axolotl source** when the symbol was Axolotl-defined. The two-language symbol graph unifies the user's view. Diagnostics are remapped to Axolotl spans. Refactorings work across both languages.

Full architecture, capabilities, editor integrations, configuration, and future extensions: `LSP.md`.

---

## 48. The final concept (for real this time)

Axolotl is:

```text
Spine        (Concepts 1-28)
   + Rust semantics
   + Cargo ecosystem
   + No runtime
   + Smart compiler
   + Pond cache
   + Bucket toolchain

Enjoyable layer (Concepts 29-46)
   + Match replacements (4 forms)
   + Let-else, pipe, named args
   + Default values, spread
   + Destructuring, if-let chains
   + Comprehensions, method cascading
   + String literals, number suffixes
   + Doc comments, inline tests
   + Auto-derive, quick constructors, struct update

IDE layer (Concept 47)
   + Gills (axol-analyzer)
   + rust-analyzer fork
   + Cross-language understanding
   + Unified symbol graph
   + Span mapping
   + Code actions

Iteration layer (Concept 49)
   + axol-hot-runner (interpreter + JIT)
   + Hot reload on file save
   + Mixed .axol + .rs execution
   + Sub-200ms feedback loop
   + Elixir-grade DX
   + Bucket.jsonc `interpreted_dev_mode` + `scripts`
```

That's the language. Spine, enjoyable layer, IDE, iteration layer. The compiler is a frontend. The orchestrator is Bucket. The engine is Cargo. The runtime is whatever the user explicitly requires. The compiler is the product. The user is the priority.

---

## 49. The iteration layer - `axol-hot-runner`

The user said: *"nobody should ever spend hours compiling code to just test out"*. The iteration layer is the answer.

### The iteration principle

> **The user should never have to wait more than 200ms between "save my code" and "see my new code run."**

That's the bar. The 30-second compile loop is over.

### The two modes

`bucket run` is the dev command. It defaults to AOT build + run unless `interpreted_dev_mode: true` is in `Bucket.jsonc`, in which case it defaults to interpreter mode.

| Mode | Flag | Speed vs AOT | Best for |
|---|---|---|---|
| Interpreter | `--interpret` | 10-30% | Game dev, REPL, prototyping |
| JIT (Cranelift) | `--jit` | 70-95% | Servers, CLIs, anything CPU-bound |
| AOT (default) | `--release` | 100% | Production |

Both modes support:
- Mixed `.axol` + `.rs` execution through a unified symbol graph.
- Hot reload - change a function, save, see new behavior in ~50ms.
- Full `print`, `assert`, basic types, control flow, collections, async/await, tasks, channels.
- Error messages mapped back to Axolotl source spans.
- AOT fallback via `bucket build --release` when the user wants native performance.

### The `interpreted_dev_mode` field

```jsonc
{
    "interpreted_dev_mode": true
}
```

When `true`, `bucket run` defaults to interpreter mode. When `false` or absent, `bucket run` defaults to AOT. Per-call CLI flags (`--interpret`, `--jit`, `--release`) always win.

### The `scripts` field

The user said: *"in Bucket.json ni scripts (just like package.json in pnpm bun whatever u got you could make custom script just adding a key with value in scripts object same here)"*.

`scripts` is a string→string map. The user defines their own dev / build / test / release / etc. commands.

```jsonc
{
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
        "doctor":  "bucket doctor",
        "any_custom_script": "echo 'hi from a custom script'"
    }
}
```

Then `bucket run dev`, `bucket run test`, `bucket run any_custom_script` work the same way.

### The mixed-language execution

A single project can have any combination of `.axol` and `.rs` files. The interpreter loads all of them into a shared symbol graph and dispatches calls across the language boundary seamlessly. The boundary is a normal Rust ABI (because Axolotl compiles to Rust). No FFI, no marshalling, no serialization.

### The Elixir-grade DX goal

In Elixir / Erlang, the killer DX features are:
- No compile step
- Hot code reload with state preservation
- Lightweight processes
- Live REPL
- Pattern matching in function heads
- Pipe operator
- Protocol consolidation

Axolotl has the language features (`task`, `|>` for pipe, variant-handler methods for pattern matching, interfaces for protocols). `axol-hot-runner` provides the runtime layer. Together: Elixir-grade DX on top of Rust-grade performance.

### Full design

See `Interpreter.md` for the complete architecture, performance targets, file layout, limitations, and v1 acceptance criteria.

---

## 50. The "DX over compile-time" principle

Three iterations of the same idea:

1. The language should be enjoyable (see `HighLevel.md` - match replacements, let-else, pipe, named args, etc.).
2. The compiler should be smart (see `LSP.md` - Gills understands both languages, smart diagnostics, code actions).
3. The development loop should be fast (this concept - sub-200ms from save to new code running).

All three are about reducing the time between "I have an idea" and "I see it working." The first reduces typing. The second reduces confusion. The third reduces waiting.

Axolotl commits to all three. `axol-hot-runner` is the third pillar.
