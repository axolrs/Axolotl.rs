# Axolotl

**A high-level, all-purpose, memory-safe, zero-GC programming language that compiles to idiomatic Rust and ships as a native binary - with no runtime, no VM, no garbage collector, ever.**


## Summary

Axolotl takes Lua's surface - tables, `if/then/else/end`, `function ... end`, `:` method syntax - and pairs it with Rust's spine: ownership, traits, generics, async, algebraic data types, zero-cost abstractions, native compilation, and memory safety. The compiler (`axolc`) lowers Axolotl source to idiomatic Rust source. The orchestrator (`bucket`) drives Cargo, rustc, and LLVM underneath. The user gets Lua's "I can just write the damn thing" feeling with Rust's "this will run anywhere, fast, and safe" guarantees. Every Cargo crate ever published is, on day one, an Axolotl library. There is no Axolotl runtime, no VM, no GC, no interpreter in release builds, no scheduler. If you need `Arc`, you import Rust's `std::sync::Arc`. If you need nothing, the compiler doesn't secretly inject something.

```axol
Player = struct
    name: String
    hp: Int
end

Player.attack = fn(self, damage: Int)
    self.hp = self.hp - damage
    return self.hp
end

fn main()
    var hero = Player { name = "axol", hp = 100 }
    hero:attack(12)
    print("hero hp = ${hero.hp}")
end
```

## Install

From source (requires Rust 1.98+):

```sh
git clone --recurse-submodules git@github.com:axolrs/Axolotl.rs.git
cd Axolotl.rs
make install
```


## The 60-second tour

```sh
bucket new hello && cd hello      # scaffold a Axolotl Project
bucket run                      # interpret in dev mode
bucket run --watch              # edit + see output instantly
bucket build --release          # native binary via rustc; zero Axolotl runtime
```

- `bucket run` (with `interpreted_dev_mode: true` in `Bucket.jsonc`) runs the tree-walking interpreter for instant feedback; `--watch` hot-reloads with state preservation.
- `bucket build --release` lowers `.axol` to Rust and hands off to Cargo. The resulting binary links only system libraries - measured: zero interpreter, JIT, or analyzer symbols.
- `bucket gills --stdio` starts the language server (completions, hovers, diagnostics, rename) for the editor integrations.

## Status (0.1.0-moss)

- 42,800+ tests: 42,690 across 82 workspace suites (0 failures, 0 warnings), plus 38 VS Code-hosted extension tests, 8 wasm playground binding tests, and 92 tree-sitter corpus tests.
- All 10 build-your-own-x benchmark ports (Git, Database, Redis, Shell, Web Server, Text Editor, Programming Language, Regex Engine, Docker, CHIP-8 Emulator) within 5% of their hand-written Rust references, with byte-identical output.
- Real toolchain: Shed formatter, Neoten linter, Regrow auto-fixer, Ambystoma docs, Salamander bench, Eggbox packaging, Molt migration, and `bucket gills --stdio` spawning the Gills LSP.
- The in-browser playground at `axolsite/` `/play` runs axolc compiled to wasm: run programs, read diagnostics, and inspect generated Rust.
- Editor support: VS Code (`axolcode`), Zed (`zed-axolotl`), Neovim (`nvim-axolotl`), all sharing the tree-sitter grammar.

## Documentation

- [The language spec](axolotl-docs/PROGRAMMINGLANGUAGEBIBLE.md) - the 91-section source of truth This was somewhat AI generated doc which defines the language syntax and features.
- [Concepts](axolotl-docs/Concepts.md), [High-level design](axolotl-docs/HighLevel.md), [Architecture](axolotl-docs/ARCHITECTURE.md).
- [The interpreter and hot reload](axolotl-docs/Interpreter.md), [Gills the LSP](axolotl-docs/LSP.md).
- [Why Axolotl](axolotl-docs/ProblemAndSolutionAndWhyAxol.md), [Comparison with other languages](axolotl-docs/Comparison.md).
- [Contributing](CONTRIBUTING.md), [Code of Conduct](CODE_OF_CONDUCT.md).
- The website lives in [`axolsite/`](axolsite) (SvelteKit 5 + Tailwind 4): docs viewer, tutorial, benchmark pages.
- Benchmarks live in [`benchmarks/`](benchmarks): each port has an `axol/` port, a `rust/` reference, and a `bench/` harness that asserts the 5% bound.

## License

MIT OR Apache-2.0 (MIT preferred) - see [LICENSE](LICENSE). Free and open source.

## Owners

`PascalElixir` and the [axolrs](https://github.com/axolrs) GitHub organization - the only handles that appear in code, commits, docs, and public surfaces.

NOTE: its a fun project I did to see how good I could be in programming yes I do have some things I brought out needs attention but either way it was a good project put me through a lot of challanges and I did have fun working with it. maybe someday I will continue this and bring it to 1.x (stable release)
