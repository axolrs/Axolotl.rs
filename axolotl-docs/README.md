# Axolotl

> **A high-level, all-purpose, memory-safe, zero-GC programming language with the smartest compiler you've ever used, that compiles to idiomatic Rust and ships as a native binary - with no runtime, no VM, no garbage collector, ever.**

`Source ext`: `.axol`
`Compiler`: `axolc`
`Project tool`: `bucket`
`Build dir`: `pond/`
`Config`: `Bucket.jsonc`
`Lockfile`: `bucket.lock`
`Backend`: Rust (rustc + LLVM) via Cargo
`Ecosystem`: crates.io (the entire Rust ecosystem, day one)

---

## The pitch in one paragraph

Axolotl is a new language that takes Lua's surface - tables, `if/then/else/end`, `function ... end`, `:` method syntax, multiple returns - and pairs it with Rust's spine - ownership, traits, generics, async, algebraic data types, zero-cost abstractions, native compilation, memory safety. The compiler (`axolc`) lowers Axolotl source to idiomatic Rust source. The orchestrator (`bucket`) drives Cargo, rustc, and LLVM underneath. The user gets Lua's "I can just write the damn thing" feeling with Rust's "this will run anywhere, fast, and safe" guarantees. Every Cargo crate ever published is, on day one, an Axolotl library. There is no Axolotl runtime, no VM, no GC, no interpreter, no scheduler, no object system. If you need `Arc`, you import Rust's `std::sync::Arc`. If you need a `Vec`, you import Rust's `Vec`. If you need nothing, the compiler doesn't secretly inject something.

---

## Quickstart

```bash
# Install Bucket + axolc
curl -sSf https://axolotl.rs/install | sh

# Create a new project
bucket new my-project
cd my-project

# Add a dependency
bucket add tokio
bucket add serde

# Write some Axolotl
cat > src/main.axol <<'EOF'
use "tokio"

main = async fn()
    print("Hello, Axolotl!")
    await async_sleep(1_sec)
    print("Goodbye, Axolotl!")
end

async fn async_sleep(ms: Int)
    timer = tokio.time.sleep(tokio.time.Duration.from_millis(ms))
    await timer
end
EOF

# Build and run
bucket run
```

The output is a single native binary. No Axolotl runtime linked in. No Lua VM. No GC. The Axolotl source compiled to idiomatic Rust; the Rust compiled with rustc; rustc and the linker produced the binary.

---

## Iteration without the wait - `axol-hot-runner` (Interpreter + JIT)

Nobody should ever spend hours compiling to test something. The `axol-hot-runner` crate (already in the workspace) is Axolotl's interpreter + JIT. It reads both `.axol` and `.rs` files, executes them in-process, hot-reloads on save, and keeps game / server state alive across edits.

Two modes:

- **Interpreter** - `bucket run --interpret`. Pure tree-walking. ~50-200ms per change. Best for game dev, REPL-style exploration, prototyping.
- **JIT** - `bucket run --jit`. Cranelift-based, native codegen, hot functions compiled on the fly. ~70-95% of AOT speed. Best for servers, CLIs, anything CPU-bound.

Opt in via `Bucket.jsonc`:

```jsonc
{
    "interpreted_dev_mode": true,
    "scripts": {
        "dev":     "bucket run --watch",
        "play":    "bucket run",
        "release": "bucket build --release",
        "test":    "bucket test",
        "bench":   "bucket bench"
    }
}
```

Then `bucket run dev` for hot-reload iteration, `bucket run release` for AOT production. Goal: sub-200ms from save to new code in effect. **Full design:** `Interpreter.md`.

---

## The enjoyable layer

Axolotl is Rust's spine with a **Lua-flavored, high-level surface** that doesn't make you write verbose code. Highlights:

### Match has four forms

```axol
-- 1. case (terse)
case state
of Menu: show_menu()
of Playing: tick_game()
of GameOver(s, r): print("${s}: ${r}")
end

-- 2. variant-handler methods (match as dispatch)
GameState:on_menu = fn() show_menu() end
GameState:on_playing = fn() tick_game() end
state:handle()

-- 3. dispatch tables
let handlers = { Menu: fn() show_menu() end, Playing: fn() tick_game() end }
state:dispatch(handlers)

-- 4. match (when you really need it)
match state
    Menu => show_menu()
    Playing => tick_game()
    GameOver(score, reason) => print("${score}: ${reason}")
end
```

### Let-else, pipe, named args, defaults

```axol
-- let-else: linear early-return
let Some(user) = find_user(id) else return default_user()

-- pipe: data flows top-to-bottom
let result = data |> parse |> validate |> save

-- named args: self-documenting
spawn_actor(name = "Hero", health = 200, team = "red")

-- defaults: optional params without builders
fn create_window(title = "Untitled", width = 800, height = 600, resizable = true)
```

### Comprehensions, cascading, destructuring

```axol
-- comprehensions
let alive = [e for e in enemies if e.health > 0]

-- method cascading (no repeated `let x = x.method()`)
let result = player :update(dt) :take_damage(20) :heal(10)

-- destructuring
let { name, health } = player
let (x, y, z) = position
```

### Strings, numbers, docs, tests

```axol
-- multi-line strings auto-strip indent
let query = """
    SELECT * FROM users WHERE active = true
    """

-- number suffixes
let buffer = 4_KB
let timeout = 30_sec

--- Adds two numbers.
--- @example
---   add(1, 2)        -- 3
fn add(a: Int, b: Int) -> Int = a + b

test add
    assert add(1, 2) == 3
end
```

**The full list, design rationale, and verbose equivalents** are in `HighLevel.md` and Sections 71-91 of `PROGRAMMINGLANGUAGEBIBLE.md`. Every feature compiles to ordinary idiomatic Rust. Zero overhead.

---

## The IDE layer - Gills (axol-analyzer)

Default rust-analyzer doesn't understand `.axol`. In a mixed project, it can't see symbols defined in Axolotl, can't navigate across languages, can't remap diagnostics. **Gills** fixes this - it's a **fork of rust-analyzer** that:

- Understands both `.axol` and `.rs` in the same project.
- Jumps to the Axolotl source when a symbol is Axolotl-defined (not to `pond/generated/rust/...`).
- Surfaces Axolotl-level diagnostics (from `axolc` and `Neoten`) with full source mapping.
- Provides cross-language refactoring: rename, extract, inline, signature changes work across both languages.
- Ships inlay hints for types, parameters, lifetimes, and borrow kinds.
- Speaks standard LSP - works with VS Code, Zed, Helix, Neovim, IntelliJ, Sublime, Lapce, anything.

**Full architecture, capabilities, configuration, and editor integrations:** `LSP.md`.

---

## A complete program (game-style)

```axol
use "wgpu"
use "winit"
use "glam"

Player = struct
    position: Vec3
    velocity: Vec3
    health: Int
end

Player.update = fn(self, dt: Float)
    self.position = self.position + self.velocity * dt
end

Player.damage = fn(self, amount: Int)
    self.health -= amount
end

main = fn()
    window = Window.new("My Game", 1280, 720)
    renderer = Renderer.new(window)

    player = Player {
        position = Vec3(0, 0, 0)
        velocity = Vec3(1, 0, 0)
        health = 100
    }

    while window.is_open()
        player.update(delta)
        renderer.draw(player)
    end
end
```

The same program, expanded, can call into any Rust crate. Axolotl doesn't ship a parallel game engine, parallel graphics API, or parallel math library. It calls `wgpu`, `winit`, and `glam` directly.

---

## Mixed `.axol` + `.rs` projects

A project can have both:

```text
my-game/
├── Bucket.jsonc
├── bucket.lock
├── Cargo.toml
├── src/
│   ├── main.axol
│   ├── player.axol
│   ├── enemy.axol
│   ├── physics.rs      -- pure Rust
│   └── renderer.rs     -- pure Rust
└── pond/
```

Axolotl can call into the Rust files:

```axol
use "physics"
use "renderer"

main = fn()
    world = physics.World.new()
    renderer.init()

    while true
        world.step(delta)
        renderer.draw(world)
    end
end
```

Rust can call into the Axolotl files:

```rust
use crate::player::Player;

fn spawn_player() -> Player {
    Player::new()
}
```

Cargo's CLI doesn't know `.axol` exists. Bucket's CLI does. The boundary between the two languages is a normal Rust ABI boundary.

---

## The "magic blocks"

Foreign-language escape hatches that are **opt-in**:

```axol
cblock {
    #include <stdio.h>
    int native_add(int a, int b) {
        return a + b;
    }
}

cppblock {
    #include <iostream>
    void hello_cpp() {
        std::cout << "Hello" << std::endl;
    }
}

rblock {
    #[inline]
    pub fn fast_math(x: f32) -> f32 {
        x.sqrt()
    }
}

pyblock {
    def process_asset(path):
        return load_and_bake(path)
}
```

If your project contains **none of these blocks**, the compiler generates **zero foreign build machinery**. No `cc` configuration. No `build.rs` for C. No Python runtime. No linker flags. Nothing.

If your project contains a `cblock`, Bucket automatically generates the C toolchain configuration. Same for the others. The "what you didn't use, doesn't exist" rule, applied to FFI.

---

## Nullable types (the `T?` syntax)

Axolotl adds ergonomic nullable types on top of Rust's `Option<T>`:

```axol
name: String? = null       -- Option<String> = None
name = "Mohi"              -- Some("Mohi")

user = find_user(id)?       -- propagate the error up
user?.name                  -- optional chaining (produces String?)
user?.name ?? "Anonymous"   -- null coalescing
user!.name                  -- assert non-null
```

Underneath, the compiler emits `Option<T>` and Rust's existing machinery. There is no nullable pointer. There is no null-deref class of bug. The syntax is ergonomic; the semantics are Rust's.

---

## The toolchain

| Tool | Job | Why this name |
|---|---|---|
| **`axolc`** | The Axolotl compiler | the animal |
| **Bucket** | Project / build / dependency / package manager | the bucket that holds everything |
| **Gills** | LSP / IDE intelligence | axolotls breathe through external gills |
| **Neoten** | Linter and static analysis | axolotls stay juvenile (neoteny) |
| **Shed** | Formatter | shedding old skin |
| **Regrow** | Automatic fixes and refactors | axolotls famously regenerate limbs |
| **Ambystoma** | Documentation generator | the axolotl's scientific genus |
| **Pond** | Build / output / cache directory | axolotls live in ponds |
| **Larva** | `bucket new` - project scaffolding | the larval form |
| **Molt** | Toolchain / project migration | shedding old code for new code |
| **Salamander** | Testing and benchmarking | axolotls are salamanders |
| **Eggbox** | Package publishing and registry | where the eggs go |

These are not cute names. They are a cohesive developer ecosystem where you can live in `bucket ...` commands and never type `cargo` unless you want to.

---

## Bucket CLI

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
bucket build --release  # build (release, fresh artifacts)
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
bucket cache gc      # automatic garbage collection
bucket doctor        # health check
bucket logs          # view build history
bucket logs --errors # extract diagnostics
bucket logs --latest # open the most recent

bucket publish       # publish to Eggbox
bucket package       # build a release artifact

bucket watch         # Larva / live-reload
bucket install       # install a binary
bucket uninstall     # remove it

bucket cargo ...     # forward to Cargo
bucket rust ...      # forward to rustc

bucket emit-rust     # dump the generated Rust for inspection
```

---

## The Pond cache

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
├── incremental/        # incremental compilation state
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

Same key → reuse. Different key → recompile. That's how 30 projects using the same `serde 1.0.219` don't each duplicate 800 MB of compilation output.

`bucket build --release` does **not** inherit debug artifacts. Release gets a clean profile; debug gets aggressive reuse. Both share the same global store.

---

## Smart compiler

The compiler doesn't just produce diagnostics - it **helps**:

- recovers from parse errors and continues analysis
- collects all errors in one pass instead of one-at-a-time
- explains **Axolotl-level reasons** for failures, not raw rustc text
- maps generated-Rust spans back to Axolotl source spans
- suggests fixes the user can apply with one keystroke
- shows ownership as a visual diff
- has a live TUI build dashboard with parallel activity

For example:

```text
╭─ AXOLOTL DIAGNOSTIC ────────────────────────────────────────────╮
│                                                                │
│  3 errors prevent Axolotl → Rust compilation                   │
│  7 warnings                                                    │
│                                                                │
│  [1/3] E1024                                                   │
│                                                                │
│  Cannot move `player` while it is borrowed.                    │
│                                                                │
│  src/player.axol                                                │
│  48 │ renderer.draw(player)                                    │
│     │              └── value moved here                        │
│                                                                │
│  Borrow originated here:                                       │
│  42 │ let view = player.transform()                            │
│     │            └── borrow remains active                     │
│                                                                │
│  Why?                                                          │
│  `view` still references data owned by `player`.               │
│                                                                │
│  Suggested fix:                                                │
│  → finish using `view` before moving `player`                  │
│                                                                │
│  [r] apply suggestion  [n] next  [q] quit                      │
╰────────────────────────────────────────────────────────────────╯
```

That is **much more useful** than just making error messages longer.

---

## Use the real Rust types

When you need an `Arc`, a `Mutex`, a `RefCell`, a `HashMap`, a `Vec`, a `Future` - import them from Rust:

```axol
use "std::sync::Arc"
use "std::sync::Mutex"
use "std::collections::HashMap"
use "tokio"
use "serde"
use "nalgebra"

counter = Arc.new(Mutex.new(0))

task = spawn fn()
    lock = counter.lock()?
    *lock += 1
end
```

Axolotl doesn't ship parallel `AxolArc`, `AxolMutex`, `AxolVec`, `AxolFuture`, `AxolHashMap`. The rule is:

> *If Rust already provides it, Axolotl doesn't invent a parallel implementation unless there is a compelling language-level reason.*

That's the rule that keeps the runtime out of the binary.

---

## Use cases

Axolotl is **all-purpose**. It can write:

- **Applications** - web backends, CLIs, desktop, mobile, tools
- **Systems** - OS components, drivers, embedded, networking, drivers
- **Science** - simulation, numerical, aerospace, robotics, HPC
- **Games and game engines** - one workload among many
- **Compilers and developer tooling** - including Axolotl itself
- **Infrastructure** - distributed systems, databases, cryptography, servers

If Rust can reasonably be used for it, Axolotl should be able to target the same space.

---

## Documentation map

This project ships with nine documents that together describe Axolotl from idea to spec:

| Document | What it covers |
|---|---|
| `IDEA.md` | The seed, the pivots, the corrections, the final identity |
| `Concepts.md` | Every language, semantic, ecosystem, and high-level concept |
| `ARCHITECTURE.md` | Five candidate architectures, the chosen one, why the others were rejected |
| `README.md` | This file - pitch, quickstart, toolchain, examples |
| `PROGRAMMINGLANGUAGEBIBLE.md` | Full language reference (91 sections), in ccpprustbible style |
| `Comparison.md` | Axolotl vs Lua, Rust, TypeScript, Nim, Zig, Mojo, Carbon |
| `ProblemAndSolutionAndWhyAxol.md` | Every problem raised in the conversation, the solution, why Axolotl is the answer |
| `HighLevel.md` | The enjoyable layer - match replacements, let-else, pipe, named args, comprehensions, etc. |
| `LSP.md` | Gills (axol-analyzer) - the rust-analyzer fork that understands `.axol` + `.rs` together |
| `Interpreter.md` | axol-hot-runner - the interpreter + JIT, hot reload, Elixir-grade DX |
| `PROMPT.md` | The master engineering brief - the prompt AI agents read to work on this project |

Start with `IDEA.md` if you want the story. Start with `PROGRAMMINGLANGUAGEBIBLE.md` if you want the spec. Start with `ARCHITECTURE.md` if you want to understand the system. Start with `HighLevel.md` if you want to see what makes the language *enjoyable*. Start with `LSP.md` if you want to understand the IDE story for mixed projects.

---

## Why "Axolotl"

The axolotl is a salamander famous for:

- **regenerating lost limbs** → Regrow, the auto-fix tool
- **staying in its larval form for life** (neoteny) → Neoten, the linter
- **breathing through external gills** → Gills, the LSP
- **living in ponds** → Pond, the build/cache directory
- **being a salamander** → Salamander, the test/bench tool
- **its scientific genus *Ambystoma*** → Ambystoma, the doc generator
- **laying eggs** → Eggbox, the registry
- **being a larva before adulthood** → Larva, the scaffolder
- **molting** → Molt, the migrator
- **the ecosystem around the animal** → Bucket, the orchestrator

The toolchain is the ecosystem that wraps the language. The axolotl regenerates code; the gills breathe intelligence; the pond holds the artifacts; the bucket orchestrates the work; the larva scaffolds the project; the molt migrates old code to new; the salamander tests it; the eggbox ships it.

That's why every tool is named after the animal.

---

## License

Axolotl is dual-licensed under MIT and Apache 2.0, the same as Rust.

---

## Contributing

See `CONTRIBUTING.md`. The project is built almost entirely in Rust (the language), so contributors should be comfortable with `axolc`, `bucket`, and the existing Cargo ecosystem.

---

## Status

Axolotl is in **design / pre-implementation**. The architecture is settled. The language surface is settled. The toolchain is named. The Pond cache model is settled. The `Bucket.jsonc` manifest format is settled.

**The current scaffold (every file):** `Cargo.toml` (workspace; edition 2024; license MIT OR Apache-2.0; six crate members), six placeholder Rust crates (`axolc`, `axolc-core`, `axol-hot-runner`, `axol-analyzer`, `bucket`, `zed-axolotl` - each is `println!("Hello, world!")`), three editor integrations (`zed-axolotl` Rust WASM, `nvim-axolotl` Lua, **`axolcode` TypeScript + Webpack + vscode-test**), the tree-sitter grammar (`tooling/tree-sitter-axol/` with bindings for 8 languages), the SvelteKit 5 website (`axolsite/` with shadcn-svelte + magic UI + Tailwind 4), and seven empty GitHub Actions workflows.

What's left is the actual implementation: the lexer, the parser, the type checker, the ownership inferencer, the diagnostics engine, the Rust code generator, the metadata importer, the Bucket orchestrator, the Gills LSP, the Neoten linter, the Shed formatter, the Regrow auto-fixer, the Ambystoma doc generator, the Pond cache, the Larva scaffolder, the Molt migrator, the Salamander test runner, the Eggbox registry, the interpreter + JIT (`axol-hot-runner`), the three editor integrations, the 10 build-your-own-x ports, the 20,000+ tests.

Join us.
