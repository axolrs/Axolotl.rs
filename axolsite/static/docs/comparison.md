# Comparison - Axolotl vs Everyone

This document positions Axolotl against the languages and language families
that matter most: the two it inherits from (Lua and Rust), the one it
borrows ergonomics from (TypeScript), and the modern systems languages that
are trying to do similar things (Nim, Zig, Mojo, Carbon).

For each, the comparison covers: **what they are**, **what Axolotl takes**,
**what Axolotl refuses to copy**, and **why Axolotl exists alongside them**
rather than competing head-on.

## Lua - the syntax ancestor

### What it is

Lua is a small, fast, embeddable scripting language. It has a clean
table-based data model, first-class functions, and a friendly `if/then/end`
syntax. It is dynamically typed and garbage-collected.

### What Axolotl takes

- The `if/then/end`, `function ... end`, `for ... do ... end`, `while ... do ... end`
  syntax.
- The `:` method-call operator.
- Multiple return values.
- Tables as the universal data structure (in Axolotl's case: structs).
- The general "feel" of writing Lua: short, friendly, no ceremony.

### What Axolotl refuses to copy

- The GC. Lua has a garbage collector; Axolotl does not.
- The dynamic typing. Lua is dynamically typed; Axolotl is statically typed.
- The metatable system. Lua's metatables allow runtime metaprogramming;
  Axolotl's traits are compile-time only.
- The interpreter runtime. Lua runs in a VM; Axolotl compiles to native code.

### Why Axolotl exists alongside Lua

Lua is a scripting language. Axolotl is a systems language. They share a
syntax family but solve different problems. Lua is the right choice for
embedding in a game engine or a config file; Axolotl is the right choice
for writing the game engine itself.

## Rust - the semantics ancestor

### What it is

Rust is a systems programming language with a strong type system, an
ownership model that eliminates data races, and a rich ecosystem (Cargo,
crates.io, rust-analyzer). It is the modern successor to C++.

### What Axolotl takes

- The ownership system. `borrow T`, `mut T`, `move T` map directly to
  `&T`, `&mut T`, `T`.
- The trait system. Axolotl's interfaces are Rust's traits.
- The generics system. Axolotl's `<T>` is Rust's `<T>`.
- The error handling. `Result<T, E>` is shared.
- The async model. `async function`, `await`, `spawn` map to Rust's.
- The Cargo ecosystem. Every crates.io crate is an Axolotl library.
- The LSP architecture (rust-analyzer). Gills is a fork of the
  rust-analyzer concepts.

### What Axolotl refuses to copy

- The verbose syntax. Rust's `fn foo(x: &i32) -> i32 { *x + 1 }` becomes
  Axolotl's `function foo(x: borrow Int) -> Int return x + 1 end`.
- The lifetime annotations. Axolotl infers them.
- The `mut` keyword on every binding. Axolotl uses `let` (immutable) and
  `var` (mutable); the default is immutable, the same as Rust, but the
  syntax is shorter.
- The trait-import ceremony. Axolotl brings traits into scope
  automatically when their methods are used.

### Why Axolotl exists alongside Rust

Rust is the right tool for hard systems programming where the borrow
checker is the killer feature. Axolotl is the right tool for application
programming where you want Rust's safety but with a friendlier surface.
Axolotl compiles to Rust, so the safety is identical.

## TypeScript - the ergonomics cousin

### What it is

TypeScript is JavaScript with types. It borrows type syntax from C# and
ergonomics from JavaScript. It is the most widely-used typed language for
web development.

### What Axolotl takes

- The type inference. TypeScript infers types aggressively; so does Axolotl.
- The optional parameters and default values.
- The structural typing for object literals (Axolotl's struct literals
  are structurally typed).
- The string interpolation syntax `${...}`.

### What Axolotl refuses to copy

- The GC. TypeScript has a GC; Axolotl does not.
- The dynamic dispatch. TypeScript uses dynamic dispatch heavily;
  Axolotl monomorphizes.
- The undefined/null dichotomy. Axolotl has `Option<T>`; no `null` and
  no `undefined`.

### Why Axolotl exists alongside TypeScript

TypeScript is the right tool for web frontend development. Axolotl is
the right tool for systems programming where you want TypeScript-like
ergonomics but Rust-grade safety and performance.

## Nim - the elegant systems language

### What it is

Nim is a statically typed systems language with a Python-like syntax.
It compiles to C, C++, or JavaScript. It has a GC by default (though
memory modes can disable it).

### What Axolotl takes

- The general "high-level surface, low-level semantics" philosophy.
- The macro system. Axolotl's macros are inspired by Nim's.

### What Axolotl refuses to copy

- The GC. Nim has one by default; Axolotl does not.
- The compilation to C. Axolotl compiles to Rust, which compiles to
  LLVM IR.
- The Python-like syntax. Axolotl's syntax is Lua-like.

### Why Axolotl exists alongside Nim

Nim is a strong language with a small ecosystem. Axolotl is positioned
to leverage the much larger Rust/Cargo ecosystem, while offering similar
ergonomic advantages.

## Zig - the C successor

### What it is

Zig is a systems programming language designed as a modern C. It has no
hidden control flow, no hidden memory allocation, and a powerful compile-time
meta-programming system.

### What Axolotl takes

- The compile-time meta-programming philosophy. Axolotl's macros and
  `const` evaluation are inspired by Zig's `comptime`.
- The "no hidden control flow" principle. Axolotl's error propagation is
  explicit (`?`).

### What Axolotl refuses to copy

- The lack of a borrow checker. Zig does not have one; Axolotl inherits
  Rust's.
- The lack of traits. Zig uses duck typing; Axolotl uses explicit traits.
- The syntax. Zig's syntax is C-like; Axolotl's is Lua-like.

### Why Axolotl exists alongside Zig

Zig is the right tool for maximum control over memory layout and code
generation. Axolotl is the right tool for application programming where
you want safety and ergonomics without sacrificing performance.

## Mojo - the AI language

### What it is

Mojo is a programming language for AI developers that is a superset of
Python. It compiles to native code via MLIR.

### What Axolotl takes

- The "Python-like ergonomics, native performance" philosophy.
- The focus on numeric computing (Mojo's `SIMD` types inspired some of
  Axolotl's number suffixes).

### What Axolotl refuses to copy

- The Python syntax. Axolotl's syntax is Lua-like.
- The MLIR backend. Axolotl uses LLVM via Rust.
- The Python ecosystem coupling. Axolotl uses the Cargo ecosystem.

### Why Axolotl exists alongside Mojo

Mojo is the right tool for AI developers who want to stay in Python.
Axolotl is the right tool for systems programmers who want a friendlier
language but with Rust-grade safety.

## Carbon - the C++ successor

### What it is

Carbon is a programming language designed as a successor to C++. It
has bidirectional interoperability with C++ and a modern type system.

### What Axolotl takes

- The "successor language" positioning. Axolotl is positioned as a
  friendlier successor to Rust, the same way Carbon is positioned as a
  successor to C++.
- The performance-first design.

### What Axolotl refuses to copy

- The C++ interop. Axolotl interops with Rust, not C++.
- The syntax. Carbon's syntax is C++-like; Axolotl's is Lua-like.

### Why Axolotl exists alongside Carbon

Carbon is the right tool for organizations with large C++ codebases.
Axolotl is the right tool for organizations that want Rust-grade safety
with friendlier ergonomics.

## The Positioning

Axolotl exists in a specific niche:

- **For Lua developers** who want a systems language with the same syntax.
- **For Rust developers** who want a friendlier surface without giving up
  the safety or the ecosystem.
- **For TypeScript developers** who want to write systems code with
  similar ergonomics.
- **For Nim/Zig/Mojo/Carbon developers** who want the Cargo ecosystem
  and the rust-analyzer LSP architecture.

Axolotl is **not** a competitor to any of these languages. It is a
frontend to the Rust ecosystem with a different surface syntax and an
interpreter for fast dev iteration.

## What's Next

- Read the [Language Reference](/docs/language-reference) for the full
  syntax.
- Read the [Architecture](/docs/architecture) guide for the system
  design.
- Read the [Getting Started](/docs/getting-started) guide to install
  Axolotl and write your first program.
