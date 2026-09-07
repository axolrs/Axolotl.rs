# Comparison - Axolotl vs Everyone

This document positions **Axolotl** against the languages and language families that matter most: the two it inherits from (Lua and Rust), the one it borrows ergonomics from (TypeScript), and the modern systems languages that are trying to do similar things (Nim, Zig, Mojo, Carbon).

For each, the comparison covers: **what they are**, **what Axolotl takes**, **what Axolotl refuses to copy**, and **why Axolotl exists alongside them** rather than competing head-on.

---

## 1. Axolotl vs Lua

| | Lua 5.4 | Axolotl |
|---|---|---|
| **Type system** | Dynamic | Static, inferred, with optional explicit annotations |
| **Memory** | Garbage collected | Zero-GC, deterministic destruction |
| **Compilation** | Interpreted / JIT | Compiled to idiomatic Rust → native |
| **Ecosystem** | LuaRocks, small | crates.io (the entire Rust ecosystem) |
| **Concurrency** | Coroutines (yield/resume) | Tasks / spawn / async (compile to Rust futures) |
| **C interop** | Lua C API | cblock, cppblock, rblock, pyblock, or `use` a Rust crate |
| **Syntax** | `if/then/end`, `function ... end`, `:` method, tables | same surface, statically typed |
| **Tooling** | Minimal | Gills, Neoten, Shed, Regrow, Ambystoma, Salamander, Bucket |
| **Use case** | Embedded scripting, game scripting, config | All-purpose systems + applications + science + games |

### What Axolotl takes

- The entire **surface syntax**: `if/then/elseif/else/end`, `while/do/end`, `repeat/until`, `function ... end`, `:` method calls, `.` field access, `[...]` index, `{ ... }` table literals, multiple return values, `local` (renamed `let` to avoid the keyword), lexical scoping, first-class closures.
- The **method-call ergonomics** - `player:damage(20)` reads exactly like Lua.
- The **table-as-record** model, but **statically typed**. `{ name = "Mohi", age = 22 }` infers a struct type instead of being a runtime hash map.
- The **lightweight feel** - no mandatory type annotations, no `class` keyword, no `public/private` ceremony.

### What Axolotl refuses to copy

- **GC.** Lua's garbage collector is what makes it simple but is exactly what disqualifies it from real-time, embedded, and high-performance code. Axolotl has zero GC.
- **Runtime metatables.** Lua's `__index`, `__add`, `__newindex` etc. are powerful but require runtime dispatch. Axolotl resolves them statically (operator overloading compiles to Rust trait impls).
- **Dynamic typing by default.** Lua lets you assign `x = 10` then `x = "hello"`. Axolotl forbids this. There is an explicit `Any` type for the rare case where you actually want it.
- **`nil` as a universal inhabitant.** Axolotl replaces `nil` with explicit nullable types (`T?`). You can't assign `null` to a `String` - only to a `String?`.

### Why Axolotl exists

Lua programmers love Lua for one reason: **it doesn't get in the way**. The moment you need real type safety, real performance, real interop with the modern world, Lua stops being the answer. Axolotl is "Lua that grew up" - it kept the surface, but underneath it has Rust's spine. You can write the same program, but the result is a native binary, type-checked at compile time, with the entire Rust ecosystem available.

If you only ever needed a small scripting language for embedded use, Lua is still fine. If you want a serious general-purpose language that *feels* like Lua, Axolotl is the answer.

---

## 2. Axolotl vs Rust

| | Rust | Axolotl |
|---|---|---|
| **Surface syntax** | C-family; `fn`, `let`, `mut`, `impl`, `match`, `&T`/`&mut T`, lifetimes | Lua-family; `fn`, `let`/`var`, `function ... end`, no `impl`, no explicit borrows, no lifetimes |
| **Type system** | Static, inferred, traits, generics, lifetimes | Static, inferred, interfaces (= traits), generics, lifetimes **inferred** |
| **Memory model** | Ownership + borrow checker, explicit | Ownership + borrow checker, **inferred** |
| **Compilation** | rustc → LLVM | axolc → Rust source → rustc → LLVM |
| **Ecosystem** | crates.io (own) | crates.io (same, native) |
| **Use case** | All-purpose systems | All-purpose systems, but with Lua's surface |
| **Learning curve** | Steep (lifetimes, borrow checker, traits, async, macros) | Shallow (Lua-like surface, Rust semantics, optional explicitness) |
| **Generated code readability** | n/a (the source is the artifact) | reads like idiomatic Rust |
| **Mixed with Rust** | n/a | Yes - `.axol` and `.rs` live in the same project |

### What Axolotl takes

- **The entire semantic model.** Ownership, borrowing, move semantics, RAII, deterministic destruction, `Send`/`Sync`, async/await, traits, generics, algebraic data types, pattern matching, error handling, FFI, unsafe, raw pointers, attribute-driven code generation.
- **The type system** - all of it. `Option<T>`, `Result<T, E>`, traits, associated types, generic constraints, lifetimes. The compiler maps these directly to Rust.
- **The ecosystem** - crates.io. Axolotl doesn't ship a parallel ecosystem. It consumes the real one.
- **The performance profile** - zero-cost abstractions, monomorphization, LLVM optimization, all the things that make Rust fast.
- **The safety profile** - the borrow checker still runs. Axolotl just infers what Rust makes you write down.

### What Axolotl refuses to copy

- **The surface syntax.** `let mut x: &mut Player<'a> = ...` is correct Rust; it is not how Axolotl programmers write.
- **The borrow-checker errors as the user-facing experience.** Axolotl catches what it can **before** generating Rust and produces Axolotl-level diagnostics with suggestions. Rust errors are the last mile, not the primary signal.
- **The lifetime annotations.** Rust programmers write `'a`, `'static`, lifetime bounds. Axolotl programmers don't.
- **The `impl` blocks.** Methods are attached by name, not in a separate `impl` block.
- **The `Result<T, E>` ceremony.** Axolotl has `T!E` for declarations, `?` for propagation, `??` for fallback, `try/catch` blocks, and Lua-style multiple returns.
- **The build story.** Axolotl users live in `bucket ...`, not `cargo ...`. The Pond cache is global; the build is faster; the diagnostics are smarter.

### Why Axolotl exists

Rust is the right answer for *what it optimizes for*: maximum performance, maximum safety, maximum control. But it asks the programmer to be a systems engineer to get there. A game developer, a web developer, a data scientist, or a beginner doesn't want to fight the borrow checker to ship a feature. Axolotl is "Rust that hides the machinery." Same safety, same performance, same ecosystem - but the programmer writes Lua, not Rust.

If you want maximum control over every byte and every lifetime, Rust is still the answer. If you want Rust's guarantees with Lua's ergonomics, Axolotl is the answer.

**Crucially, Axolotl is not a replacement for Rust.** The Rust source files in your project still compile with `rustc`. The Rust crates you depend on are still Rust crates. Axolotl is a new frontend for the same Rust ecosystem.

---

## 3. Axolotl vs TypeScript

| | TypeScript | Axolotl |
|---|---|---|
| **Compilation** | To JavaScript | To Rust |
| **Runtime** | Node.js / browser / Deno | Native binary (no runtime) |
| **Type system** | Structural, gradual | Nominal, static, inferred |
| **Memory** | GC | Zero-GC, deterministic |
| **Ecosystem** | npm | crates.io |
| **Concurrency** | Promises, async/await | Tasks, spawn, async/await (Rust futures) |
| **Use case** | Web, server, tooling | All-purpose systems, applications, science |
| **Ecosystem interop** | 100% with JavaScript | 100% with Rust |

### What Axolotl takes

- **Gradual typing.** You can write `let x = 10` (inferred) or `let x: Int = 10` (annotated) or `let x: Int = compute()` (annotation when inference needs help). Same idea as TypeScript's `let x = 10` vs `let x: number = 10`.
- **Inference first.** The compiler figures out what it can. You only annotate when it can't.
- **Familiar shape for JS/TS users.** Many of the `:` syntax forms (variable annotations, function signatures, return types) feel natural to TypeScript developers.
- **Excellent error messages.** TypeScript is famous for this; Axolotl follows the same philosophy.

### What Axolotl refuses to copy

- **Garbage collection.** TypeScript is built on JavaScript, which is GC'd. Axolotl has no GC.
- **A runtime that ships in every binary.** TypeScript compiles to JS, which runs on a JS engine. Axolotl compiles to native code with no language-level runtime.
- **Structural typing only.** Axolotl uses nominal typing (matches Rust's model) so that two unrelated types with the same shape don't accidentally satisfy each other's contracts.

### Why Axolotl exists

TypeScript proved that developers want **gradual typing with excellent inference and a friendly feel**. Axolotl takes that ergonomic win and applies it to systems programming. You get TypeScript-style "the compiler figures it out" without giving up native performance, memory safety, or the entire Rust ecosystem.

If you're building for the web, TypeScript is the answer. If you want the same kind of developer experience but for native code, Axolotl is the answer.

---

## 4. Axolotl vs Nim

| | Nim | Axolotl |
|---|---|---|
| **Compilation** | To C, C++, JS, or native | To idiomatic Rust |
| **Memory** | ARC / ORC (ref counting, no GC by default) | Zero-GC, ownership, deterministic |
| **Ecosystem** | Nimble (small) | crates.io (huge) |
| **Syntax** | Python-like | Lua-like |
| **Backend** | Multiple (C/C++/JS) | Single (Rust) |
| **FFI** | C FFI | C / C++ / Rust / Python, automatic |
| **Tooling** | nimble, choosenim | bucket, Gills, Neoten, Shed, Regrow, etc. |
| **Use case** | All-purpose systems | All-purpose systems |

### What Axolotl takes

- The **"all-purpose" ambition** - Nim proved a single language can target systems, scripting, web, and scientific computing.
- The **arc/orc-style memory** without garbage collection (Nim uses reference counting; Axolotl uses Rust's ownership, which is more efficient and provably safe).
- The **commitment to a small, fast, single binary output**.

### What Axolotl refuses to copy

- **C as the compilation target.** Nim's C backend gives it portability but loses access to LLVM's modern optimizations and the entire Rust ecosystem.
- **Python-like syntax.** Axolotl is Lua-flavored, not Python-flavored. (This is a matter of taste; both have merit.)
- **A small ecosystem.** Axolotl's day-one ecosystem is crates.io - the largest crates registry in any language.

### Why Axolotl exists

Nim is an excellent language that got two things right (no GC, all-purpose) and two things wrong (small ecosystem, C backend). Axolotl is "Nim with Rust's ecosystem and Rust's backend." Same memory model philosophy (no GC), same all-purpose ambition - but the ecosystem is 100x larger and the backend is the most modern in the industry.

If you love Nim and its ecosystem is enough for you, stay with Nim. If you want the same philosophy with the Rust ecosystem underneath, Axolotl is the answer.

---

## 5. Axolotl vs Zig

| | Zig | Axolotl |
|---|---|---|
| **Compilation** | Self-hosted, native via LLVM | To Rust → native via LLVM |
| **Memory** | Manual (`alloc`/`free`), no GC | Inferred ownership, no GC |
| **Ecosystem** | Small, growing | crates.io (huge) |
| **Syntax** | C-like, minimal | Lua-like, minimal |
| **Comptime** | Yes, first-class | Yes (const fn, `@reflect`, macros) |
| **FFI** | C, first-class | C / C++ / Rust / Python, opt-in |
| **Tooling** | zig build, zig fmt, etc. | bucket, Gills, Neoten, Shed, Regrow, etc. |
| **Use case** | Systems, embedded, native libraries | All-purpose systems + applications + science + games |

### What Axolotl takes

- The **"manual, predictable" memory philosophy** - Zig programmers hate GC and want deterministic destruction. Same.
- The **"comptime" commitment** - Axolotl supports `const` evaluation, `@derive`, `@reflect`, and macros.
- The **"no hidden control flow" principle** - Axolotl's `no runtime` invariant is the same kind of commitment.
- The **"if you didn't use it, you don't pay for it"** philosophy.

### What Axolotl refuses to copy

- **Manual memory management.** Zig asks you to write `allocator: std.heap.GeneralPurposeAllocator(.{}) = .init;` and pass the allocator around. Axolotl's ownership inference handles allocation automatically.
- **A small ecosystem.** Zig's ecosystem is growing but small. Axolotl has crates.io from day one.
- **C-style syntax.** Axolotl is Lua-flavored.
- **No inheritance / no traits.** Zig's "no inheritance" is a feature, but it means writing a `Drawable` interface is awkward. Axolotl has interfaces (= traits).

### Why Axolotl exists

Zig is the right answer for *what it optimizes for*: maximum control, no hidden costs, predictable performance. But the manual allocator discipline and the small ecosystem are real costs. Axolotl is "Zig's philosophy with Rust's safety and Rust's ecosystem." You get the same deterministic, no-runtime feel - but the borrow checker handles memory for you, and you have 100,000+ crates to choose from.

If you love Zig and its ecosystem is enough for you, stay with Zig. If you want the same philosophy with memory safety and the Rust ecosystem, Axolotl is the answer.

---

## 6. Axolotl vs Mojo

| | Mojo | Axolotl |
|---|---|---|
| **Compilation** | To native via Modular's backend | To Rust → native via LLVM |
| **Memory** | ARC + ownership (still maturing) | Inferred ownership, mature |
| **Ecosystem** | Growing, AI-focused | crates.io (huge, general) |
| **Syntax** | Python-like | Lua-like |
| **Backend** | Custom (Modular) | Rust (rustc + LLVM) |
| **Goal** | Be a superset of Python for AI/ML | Be a high-level language for everything |
| **Stability** | Pre-1.0 | Pre-1.0 |
| **Tooling** | Mojo CLI | bucket, Gills, Neoten, Shed, Regrow, etc. |

### What Axolotl takes

- The **"AI/scientific computing is important"** awareness.
- The **"improve on Python"** ambition.
- The **"familiar syntax to attract an existing community"** strategy (Mojo targets Python users; Axolotl targets Lua + Rust + TypeScript users).
- The **"compile to fast native code"** promise.

### What Axolotl refuses to copy

- **A custom backend.** Mojo is building its own MLIR-based compiler infrastructure. That's a multi-decade project. Axolotl reuses rustc and LLVM.
- **Python as the source language.** Mojo's Python compatibility is a feature and a curse. Axolotl's Lua heritage is much smaller and easier to evolve.
- **AI as the primary identity.** Axolotl is general-purpose. AI is one workload among many.

### Why Axolotl exists

Mojo is going after the AI/ML community, which is a defensible niche but not the only niche. Axolotl targets *all* systems programming - game engines, databases, web servers, scientific computing, embedded, networking, the works. And Axolotl doesn't try to be Python-compatible; it inherits from Lua (smaller, cleaner) and Rust (safer, faster).

If you want a Python-compatible AI-first language, Mojo is the answer. If you want a Lua-flavored all-purpose systems language that consumes the entire Rust ecosystem, Axolotl is the answer.

---

## 7. Axolotl vs Carbon

| | Carbon | Axolotl |
|---|---|---|
| **Goal** | Successor to C++ | Successor to Rust (for ergonomics) |
| **Compilation** | To LLVM (native) | To Rust → LLVM (native) |
| **Memory** | Manual / unsafe | Inferred ownership |
| **Ecosystem** | C++ ecosystem (via interop) | crates.io |
| **Syntax** | C++-like, with improvements | Lua-like |
| **Status** | Experimental (Google) | Pre-implementation |
| **Tooling** | bazel, llvm | bucket, Gills, Neoten, Shed, Regrow, etc. |

### What Axolotl takes

- The **"be a successor to an existing language"** ambition (Carbon for C++, Axolotl for Rust-ergonomics).
- The **"interoperate with the existing ecosystem"** commitment (Carbon with C++; Axolotl with Rust).
- The **"modern language design"** philosophy (no legacy baggage).

### What Axolotl refuses to copy

- **C++-style syntax.** Carbon is consciously C++-compatible. Axolotl is consciously Lua-flavored.
- **A custom backend.** Carbon is building its own LLVM-based toolchain. Axolotl reuses rustc.
- **C++ as the interop target.** C++ is enormous and messy. Rust is a much cleaner interop target.

### Why Axolotl exists

Carbon and Axolotl are both "make systems programming nicer" projects, but they target different languages. Carbon wants to be a better C++; Axolotl wants to be a friendlier Rust. Carbon keeps C++ syntax; Axolotl uses Lua syntax. Carbon is built around C++ interop; Axolotl is built around Rust interop.

If you have a huge C++ codebase, Carbon is the answer. If you have a Rust codebase (or want to start with Rust's safety), Axolotl is the answer.

---

## 8. The matrix at a glance

| | Memory | GC | Native | Ecosystem | Syntax family | Interop with | Stable |
|---|---|---|---|---|---|---|---|
| **Lua** | manual + GC | yes | via LuaJIT | small | Lua | C | yes |
| **Rust** | ownership | no | yes | crates.io | C-family | C, FFI | yes |
| **TypeScript** | GC | yes | via JS engine | npm | C-family | JS | yes |
| **Nim** | ARC | no | yes | nimble | Python | C, FFI | yes |
| **Zig** | manual | no | yes | small | C | C | yes |
| **Mojo** | ARC (maturing) | no | yes | small | Python | Python | no |
| **Carbon** | manual / unsafe | no | yes | C++ | C++ | C++ | no |
| **Axolotl** | **inferred ownership** | **no** | **yes** | **crates.io** | **Lua** | **Rust, C, C++, Python** | **pre-1.0** |

---

## 9. What Axolotl is uniquely

Strip away the comparisons, and Axolotl is the only language that simultaneously is:

1. **Lua-flavored in syntax** - `if/then/end`, `function ... end`, `:` method, tables, multiple returns.
2. **Rust-semantic underneath** - ownership, traits, generics, async, no GC, zero-cost abstractions.
3. **Compiled to idiomatic Rust** - not a custom backend, not a custom optimizer. rustc and LLVM do the work.
4. **100% Cargo ecosystem compatible** - every crate on crates.io works on day one.
5. **Mixed-language native** - `.axol` and `.rs` are peers in the same project.
6. **Opt-in FFI** - `cblock`, `cppblock`, `rblock`, `pyblock` generate build machinery only when present.
7. **Zero overhead** - no Axolotl runtime ever.
8. **Smart-compiler driven** - Gills, Neoten, Regrow, Shed, Ambystoma make the engineering experience first-class.
9. **Global pnpm-style cache** - Pond reuses compiled artifacts across projects.

No other language has all nine. That's the positioning.
