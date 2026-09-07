# PROGRAMMING LANGUAGE BIBLE - Axolotl

> **The complete Axolotl (`.axol`) language reference.** Every concept, every form, every example, in the same shape as the C++/Rust bible: **What it is** · **Why it's important** · **Basic syntax and usage** · **Key things to remember** · **Example Use Case**.

The file extension is `.axol`. The compiler is `axolc`. The orchestrator is `bucket`. There is **no Axolotl runtime, no VM, no GC, no interpreter, no scheduler, no object system** - programs compile to **idiomatic Rust** and ship as native binaries through Cargo + rustc + LLVM.

---

## Table of Contents

1. Hello World
2. Comments
3. Modules and `use`
4. Variables (`let` and `var`)
5. Primitive Types
6. Sized Integer / Float Types
7. Booleans
8. Strings
9. Type Inference
10. Type Annotations
11. Nullable Types (`T?` and `null`)
12. Print
13. Functions
14. Multiple Return Values
15. Closures
16. Structs
17. Methods (no `impl` keyword)
18. Tables and Record Literals
19. Enums
20. Pattern Matching
21. Interfaces (Traits)
22. Implementation Blocks
23. Generics
24. Constants and Compile-Time Evaluation
25. `if / elseif / else / end`
26. `while / do / end`
27. `repeat / until`
28. `for` (Numeric and Iterator)
29. `break` / `continue`
30. Arrays
31. Maps
32. Iterators
33. Ownership (Inferred)
34. Explicit Ownership (`borrow`, `mut`, `move`)
35. References and Borrows
36. Lifetimes (Inferred, Never Written)
37. Result and Error Handling (`!`, `?`, `??`, `try`/`catch`)
38. Option / Nullable Chaining (`?.` and `!.`)
39. Coroutines and Tasks
40. `spawn` and Threads
41. Channels
42. `async` / `await`
43. Send / Sync (Inferred)
44. Unsafe
45. Raw Pointers
46. Memory Layout (`@repr(C)` etc.)
47. Allocation Domains (Stack / Heap / Arena / Persistent)
48. RAII and Deterministic Destruction
49. FFI: `cblock`
50. FFI: `cppblock`
51. FFI: `rblock`
52. FFI: `pyblock`
53. `extern fn` (C Declarations)
54. Attributes (`@derive`, `@repr`, `@inline`, etc.)
55. Macros and Procedural Macros
56. Compile-Time Reflection (`@reflect`)
57. Error Recovery in the Parser
58. Source Maps and Span Mapping
59. Mixed `.axol` + `.rs` Projects
60. `bucket emit-rust` (Inspect Generated Rust)
61. The Smart Compiler (Gills / Neoten / Regrow)
62. The Pond Cache
63. The Bucket CLI
64. Bucket.jsonc Manifest
65. Bucket.lock Lockfile
66. Foreign Crates Just Work
67. The "Use the Real Rust Types" Rule
68. The "No Runtime" Invariant
69. The "What You Don't Use Doesn't Exist" Principle
70. The Zero-Overhead Promise
71. **Match Replacements** (the "less code than match" toolkit)
72. `case` Syntax (Terse Match)
73. Variant-Handler Methods (Match as Dispatch)
74. Dispatch Tables (Lua-Style, Statically Checked)
75. `let-else` (Early Return Patterns)
76. Pipe `|>` and Reverse Application `<|`
77. Named Arguments
78. Default Parameter Values
79. Spread Operator (`...`)
80. Destructuring Assignment
81. If-Let Chains
82. List / Map / Set Comprehensions
83. Method Cascading
84. Multi-line / Raw / Tagged Strings
85. Number-Suffix Literals (`4_KB`, `16_ms`, ...)
86. Documentation Comments (`---`)
87. Inline Tests and Examples
88. Auto-Derive Common Traits
89. Quick Constructors (`Type:new`)
90. Struct Update Syntax
91. The High-Level Surface Summary

> **Note on the high-level features (71-91).** The user asked: *"Why is there no other solution that achieves the same thing as match but less code?"* The answer is the family of features in this section. **All of them compile to ordinary idiomatic Rust** - they are sugar, not a separate "easy mode." Every feature has a verbose equivalent the user can drop down to when clarity demands it. The full design rationale is in `HighLevel.md`.

---

## 1. Hello World

* **What it is:** The minimal Axolotl program that prints a line of text and exits.
* **Why it's important:** Confirms your toolchain (`bucket`, `axolc`, Cargo, rustc) is wired correctly and produces a runnable native binary.
* **Basic syntax and usage:**
```axol
print("Hello, world!")
```
* **Key things to remember:** No `main` function is required for a single-file script. Bucket auto-generates a `fn main()` wrapper for entry-point files. The output is a native binary produced by rustc + LLVM, not a script interpreted by an Axolotl VM.
* **Example Use Case:** Sanity-checking a fresh `bucket new` project, or a CI smoke test for the toolchain.

---

## 2. Comments

* **What it is:** Non-executable text the programmer leaves for human readers.
* **Why it's important:** Documents intent, explains non-obvious decisions, helps future-you (or the next person) understand the code.
* **Basic syntax and usage:**
```axol
-- single-line comment

--[[
    multi-line
    block
    comment
]]
```
* **Key things to remember:** Axolotl uses Lua-style `--` (single) and `--[[ ... ]]` (block) comment syntax. Comments are stripped during parsing and have zero runtime cost - they don't appear in the generated Rust at all.
* **Example Use Case:** Annotating a tricky ownership rule, marking a TODO, or temporarily disabling a block of code.

---

## 3. Modules and `use`

* **What it is:** The mechanism for importing other modules, files, or Rust crates into the current file.
* **Why it's important:** Every non-trivial program is split across files and depends on external libraries. `use` is the way you say "I need this."
* **Basic syntax and usage:**
```axol
-- import a Rust crate by name
use "tokio"
use "serde"
use "wgpu"

-- import a local Axolotl module by path (no extension)
use "player"
use "world.physics"

-- import a specific symbol
use "std::sync::Arc"
use "std::collections::HashMap"
```
* **Key things to remember:** `use` is the unified import syntax - it covers local Axolotl modules, local Rust modules, and external Cargo crates. Bucket's resolver handles the difference; the user just writes `use "name"`. There is no `require`, no `mod`, no nested `pub use` chain - the model is flat and predictable.
* **Example Use Case:** Importing `wgpu` to render something, importing a project-local `player.axol` to use its `Player` struct, or importing `std::sync::Arc` for shared ownership.

---

## 4. Variables (`let` and `var`)

* **What it is:** A named binding to a value. `let` is immutable; `var` is mutable.
* **Why it's important:** Every program manipulates state. Axolotl makes mutability explicit so the compiler can reason about ownership.
* **Basic syntax and usage:**
```axol
let name = "Mohi"        -- immutable
let health = 100         -- immutable

var score = 0            -- mutable
score = score + 10
score += 5               -- compound assignment works
```
* **Key things to remember:** `let` is the default. `var` is opt-in. There is no `mut` keyword, no shadowing rules to memorize - when you want a new binding with the same name, just use `let` again. The compiler enforces that `let` bindings are not reassigned, which means it can place them in registers, on the stack, or in const contexts without runtime checks.
* **Example Use Case:** A player's `health` is mostly read, but it's decremented during gameplay - declare it `var` so damage can update it. A `name` is set once and never changes - `let` it.

---

## 5. Primitive Types

* **What it is:** The built-in scalar types every Axolotl program uses.
* **Why it's important:** The compiler needs concrete types to generate correct Rust and to reason about ownership, layout, and arithmetic.
* **Basic syntax and usage:**
```axol
let a: Int = 100              -- platform-default signed integer (i64 on 64-bit)
let b: UInt = 200             -- platform-default unsigned integer (u64 on 64-bit)
let c: Float = 3.14           -- platform-default float (f64)
let d: Double = 3.14159       -- explicit double precision (same as Float)
let e: Bool = true
let f: String = "hello"
let g: Char = 'x'             -- single Unicode scalar
let h: Byte = 0xFF            -- unsigned 8-bit
```
* **Key things to remember:** `Int` and `Float` are platform-default sized (so the same code is fast on every platform). When you need explicit widths, use the sized types below. The compiler maps these to Rust's `i64`, `u64`, `f64`, `bool`, `String`, `char`, `u8` respectively.
* **Example Use Case:** Any program that touches numbers, text, or flags. Counting frames, indexing into a buffer, parsing JSON, comparing two values.

---

## 6. Sized Integer / Float Types

* **What it is:** Explicit-width numeric types for when the platform default is wrong.
* **Why it's important:** Networking protocols, file formats, hardware registers, GPU buffers, and cryptographic algorithms all require exact bit widths. The default `Int` is not enough.
* **Basic syntax and usage:**
```axol
let small: I8   = -128
let medium: I16 = 32_000
let word: I32   = 2_000_000_000
let long: I64   = 9_000_000_000_000

let byte: U8    = 255
let word: U16   = 65_535
let dword: U32  = 4_000_000_000
let qword: U64  = 18_000_000_000_000_000_000

let single: F32 = 3.14
let double: F64 = 3.141592653589793
```
* **Key things to remember:** Sized types map directly to Rust's `i8`/`i16`/`i32`/`i64`/`u8`/`u16`/`u32`/`u64`/`f32`/`f64`. No implicit narrowing conversions - the compiler forces you to use an explicit cast. Underscores are allowed as digit separators for readability.
* **Example Use Case:** Reading a 4-byte little-endian integer from a binary file, packing a network packet, indexing a `Vec<U8>` of bytes, computing a hash, talking to a GPU buffer.

---

## 7. Booleans

* **What it is:** The two-valued logical type.
* **Why it's important:** Branching, comparison, predicates.
* **Basic syntax and usage:**
```axol
let alive: Bool = true
let visible: Bool = false

if alive and visible then
    print("draw the sprite")
end
```
* **Key things to remember:** `true` and `false` are lowercase. Logical operators are spelled `and` / `or` / `not` (not `&&`/`||`/`!`). The `not` operator can be prefix or, more idiomatically, prefixed as `not x`. Comparison returns `Bool`: `==`, `~=` (Lua-style "not equal"), `<`, `>`, `<=`, `>=`.
* **Example Use Case:** Game state checks, user input predicates, feature flags, validation.

---

## 8. Strings

* **What it is:** UTF-8 encoded text. `String` is owned and growable. `&str` is a borrowed view (slice).
* **Why it's important:** Almost every program processes text. Networking, file I/O, UI, logging, JSON, configuration - all strings.
* **Basic syntax and usage:**
```axol
let greeting: String = "hello, world"
let multiline: String = "line one\nline two\nline three"
let interpolated: String = "name = ${name}, age = ${age}"

-- string literal (borrowed view)
let slice: &str = "constant"
```
* **Key things to remember:** Strings are UTF-8, not ASCII. Use `${expr}` for interpolation (no `format!("{}", x)` ceremony). Indexing a string by `s[i]` returns a **byte** and panics if the index isn't a char boundary - prefer `s.chars()` for codepoint iteration. The compiler maps `String` to Rust's `String` and `&str` to `&str`.
* **Example Use Case:** Building log messages, parsing JSON, formatting user-visible text, serializing data.

---

## 9. Type Inference

* **What it is:** The compiler's ability to deduce the type of a binding or expression without an explicit annotation.
* **Why it's important:** Removes the most common source of C++/Java/Rust verbosity. Lets you write `let x = compute()` instead of `let x: Result<Int, Error> = compute()`.
* **Basic syntax and usage:**
```axol
let name = "Mohi"        -- inferred String
let age = 20             -- inferred Int
let alive = true         -- inferred Bool
let items = [1, 2, 3]    -- inferred Array<Int>

let player = Player {    -- inferred from constructor
    name = "Mohi"
    health = 100
}
```
* **Key things to remember:** Inference flows from usage and from initializers. It does **not** flow from later reassignment (because `let` bindings are immutable). If inference fails, the compiler points at the binding and asks for an explicit annotation - it does not silently guess.
* **Example Use Case:** Almost every binding in a real program. Inference is the reason Axolotl feels "Lua-like" while still being statically typed.

---

## 10. Type Annotations

* **What it is:** An explicit `: Type` after a binding, parameter, or return value.
* **Why it's important:** Public APIs need stable types. Generic functions sometimes need help. The compiler can ask you for one when inference can't decide.
* **Basic syntax and usage:**
```axol
let name: String = "Mohi"
let health: Int = 100

fn add(a: Int, b: Int) -> Int
    return a + b
end

fn find_user(id: Int) -> User?
    if database.has(id) then
        return Some(database.get(id))
    end
    return None
end
```
* **Key things to remember:** Annotations are **optional** when inference is unambiguous. The compiler will tell you when they're needed (`error: cannot infer type; annotate the binding`). Annotations also serve as documentation in public APIs.
* **Example Use Case:** A library's exported functions, anywhere a `T?` must be distinguished from a `T`, anywhere inference can't pick between two valid types.

---

## 11. Nullable Types (`T?` and `null`)

* **What it is:** A type that may or may not hold a value. `T?` is Axolotl syntax for Rust's `Option<T>`. `null` is its empty value.
* **Why it's important:** Absence is a real concept. Database lookups miss, parsers fail, user input is empty. The type system must let you represent "no value" without resorting to undefined behavior.
* **Basic syntax and usage:**
```axol
let name: String? = null        -- Option<String> = None
name = "Mohi"                   -- Some("Mohi")

let user: User? = find_user(id)

if user then
    print(user.name)
end

let safe = user?.name ?? "Anonymous"

let definitely = user!.name      -- assertion: user is not null
```
* **Key things to remember:** `null` is **not** a universal inhabitant of every type - it only exists in `T?` contexts. A `String` can never be null. A `String?` can. The compiler enforces this. `user?.name` produces another nullable (`String?`). `user!.name` asserts non-null and panics if wrong. `??` provides a fallback. Underneath, the compiler emits `Option<T>` and Rust's existing machinery - there is no nullable pointer, no null-deref bug.
* **Example Use Case:** Database row lookups, configuration values that may not be set, parser results, optional command-line flags, the return type of `Map.get`.

---

## 12. Print

* **What it is:** Output text to standard output.
* **Why it's important:** Debugging, logging, user-facing output.
* **Basic syntax and usage:**
```axol
print("Hello, world!")
print("name =", name, "age =", age)        -- multi-arg, space-separated
print("result = ${compute()}")              -- interpolation
```
* **Key things to remember:** `print` is variadic and accepts any value implementing `Display`. No `println!` / `print!` distinction - `print` always appends a newline (the Lua convention). For format-without-newline, use `io.write(...)`. The compiler maps `print(...)` to Rust's `println!` macro or `std::io::Write` depending on the call shape.
* **Example Use Case:** Logging, debugging, command-line tool output, simple progress reporting.

---

## 13. Functions

* **What it is:** A named, parameterized, reusable block of code.
* **Why it's important:** The fundamental unit of abstraction. Every non-trivial program is composed of functions.
* **Basic syntax and usage:**
```axol
fn add(a: Int, b: Int) -> Int
    return a + b
end

-- expression-bodied: the last expression is the return value (no `return` needed)
fn add(a: Int, b: Int) -> Int
    a + b
end

-- no return type means the function returns Unit
fn greet(name: String)
    print("hello, ${name}")
end
```
* **Key things to remember:** Functions use Lua-style `fn ... end` blocks. The last expression is the return value (Rust-style expression-oriented bodies) - but you can also use explicit `return` for early-exit. The compiler maps these to ordinary Rust `fn` items with full type inference. `fn` declarations are private to the file by default; prefix with `pub` to export.
* **Example Use Case:** Anything that benefits from naming. Math helpers, request handlers, game systems, parsers, validators.

---

## 14. Multiple Return Values

* **What it is:** A function can return more than one value. This is a Lua heritage - and a very natural fit for "result + error" patterns.
* **Why it's important:** Lets you write `value, err = may_fail()` instead of `value = may_fail().unwrap_or(default)`. Familiar to Lua, Go, and Python programmers.
* **Basic syntax and usage:**
```axol
fn divide(a: Float, b: Float) -> Float, Error
    if b == 0.0 then
        return 0.0, Error("division by zero")
    end
    return a / b, nil
end

value, err = divide(10.0, 3.0)
if err then
    print("failed:", err)
else
    print("result:", value)
end
```
* **Key things to remember:** Underneath, the compiler lowers this to a Rust tuple `(Float, Option<Error>)` or a `Result<Float, Error>` depending on the call site. Both styles are first-class; the multiple-return form is for Lua ergonomics, the `Result` form is for Rust-style exhaustiveness. You can mix and match.
* **Example Use Case:** Database queries, parsers, file I/O, anything that wants both a value and a status.

---

## 15. Closures

* **What it is:** An anonymous function value that can capture variables from its enclosing scope.
* **Why it's important:** Enables functional-style callbacks, iterator chains, deferred computation, and event handlers - without the ceremony of declaring a named function for every small operation.
* **Basic syntax and usage:**
```axol
let add = fn(a, b) a + b end
print(add(3, 4))

let numbers = [1, 2, 3, 4, 5]
let doubled = numbers.map(fn(n) n * 2 end)
print(doubled)

let counter = { count = 0 }
let bump = fn() counter.count += 1 end
bump()
bump()
print(counter.count)         -- 2
```
* **Key things to remember:** Captures are **inferred** - the compiler decides whether each captured variable is captured by reference, by mutable reference, or by move based on how the closure uses it. Closures implement the appropriate Rust `Fn`/`FnMut`/`FnOnce` trait automatically. The user never writes `move` on a closure unless they specifically need to force a move.
* **Example Use Case:** Callbacks for async I/O, predicates for filtering, comparators for sorting, event handlers in a UI toolkit, `map`/`filter`/`reduce` over collections.

---

## 16. Structs

* **What it is:** A product type - a fixed set of named fields, each with a type.
* **Why it's important:** The primary way to model a "thing with several properties." A `Player` has a `position`, a `velocity`, a `health`. A `Packet` has an `id`, a `length`, a `flags`. Structs are the building block of every domain model.
* **Basic syntax and usage:**
```axol
Player = struct
    position: Vec3
    velocity: Vec3
    health: Int
    name: String
end

-- construction
let player = Player {
    position = Vec3(0, 0, 0)
    velocity = Vec3(0, 0, 0)
    health = 100
    name = "Mohi"
}

-- field access
player.health -= 10
print(player.name)
```
* **Key things to remember:** Fields are `pub` by default within the module. The compiler maps `struct Player { ... }` to `pub struct Player { pub ... }` in Rust. The Lua-style table constructor `Player { name = ..., health = ... }` is statically typed - the compiler checks field names and types at compile time, even though the syntax looks like a Lua table. There is no `pub` keyword on individual fields in the common case.
* **Example Use Case:** Domain models (Player, Enemy, World, Packet, Request, User, Order), C-compatible data layouts, anything that is a "bag of named values."

---

## 17. Methods (no `impl` keyword)

* **What it is:** A function attached to a struct, called via `value:method()` or `value.method()`.
* **Why it's important:** Lets you group behavior with the data it operates on, without the boilerplate of an `impl` block.
* **Basic syntax and usage:**
```axol
Player = struct
    position: Vec3
    velocity: Vec3
    health: Int
end

-- method definition: NO `impl` block
Player.update = fn(self, dt: Float)
    self.position = self.position + self.velocity * dt
end

Player.damage = fn(self, amount: Int)
    self.health -= amount
end

-- call site
let p = Player { ... }
p:update(0.016)
p:damage(20)

-- or with `.` syntax
p.update(0.016)
p.damage(20)
```
* **Key things to remember:** Methods are just functions with `self` as the first parameter, attached to the type by name. The compiler infers whether `self` should be `&Self`, `&mut Self`, or `Self` based on what the method body does. You can call methods with either `:` (Lua-style, passes `self` automatically) or `.` (passes `self` explicitly). Both compile to ordinary Rust `impl` blocks.
* **Example Use Case:** Anything that wants OOP-style "do something to this thing" syntax. Game entity methods, builder methods, formatter methods, parser methods.

---

## 18. Tables and Record Literals

* **What it is:** Lua-style `{ key = value, ... }` table constructors, but statically typed.
* **Why it's important:** The most Lua-flavored feature in Axolotl. Lets you write data literals without declaring a struct first, while still getting compile-time type checking.
* **Basic syntax and usage:**
```axol
-- inferred record type
let config = {
    host = "localhost"
    port = 8080
    workers = 8
    tls = true
}

-- the compiler infers:
-- { host: String, port: Int, workers: Int, tls: Bool }

-- array literal
let numbers = [1, 2, 3, 4, 5]            -- Array<Int>

-- mixed array literal (inferred as Array<Any> or an enum)
let mixed = [1, "two", true]              -- use with care

-- nested
let nested = {
    name = "outer"
    inner = {
        x = 10
        y = 20
    }
}
```
* **Key things to remember:** `{ ... }` is a record literal - the compiler infers a struct type. `[ ... ]` is an array literal. `nil` is not allowed in record fields (use `T?` if absence is meaningful). The compiler can promote a record literal to a named `struct` if the same shape appears multiple times, but only if the user explicitly asks (`type Foo = { ... }`).
* **Example Use Case:** Configuration blocks, test fixtures, JSON-shaped data, quick data definitions in REPL-style code.

---

## 19. Enums

* **What it is:** A sum type - a value that is one of several variants. Each variant may carry data.
* **Why it's important:** Models "this is one of N things, and each thing has its own data." Rust's `enum` is the cleanest type-system feature in any mainstream language. Axolotl inherits it.
* **Basic syntax and usage:**
```axol
GameState = enum
    Menu
    Playing
    Paused
    GameOver(score: Int, reason: String)
end

-- construction
let s = GameOver(0, "out of time")

-- or with named fields
let s = GameOver {
    score = 0
    reason = "out of time"
}
```
* **Key things to remember:** Variants can be unit (`Menu`), tuple-like (`GameOver(Int, String)`), or struct-like (`GameOver { score, reason }`). Pattern matching on an enum is exhaustive - the compiler refuses to compile a `match` that misses a variant. The compiler maps `enum GameState { ... }` directly to Rust's `pub enum GameState { ... }`.
* **Example Use Case:** State machines, AST nodes, error types, message types, result types, any "one of several shapes" model.

---

## 20. Pattern Matching

* **What it is:** A control structure that deconstructs a value, binding names to its parts, and runs code based on the shape it matches.
* **Why it's important:** The natural way to handle enums, `Option`, `Result`, and any algebraic data type. Pattern matching is exhaustive - the compiler verifies every case is covered.
* **Basic syntax and usage:**
```axol
match state
    Menu => show_menu()
    Playing => tick_game()
    Paused => show_pause()
    GameOver(score, reason) =>
        print("final score: ${score}, reason: ${reason}")
end

-- with guards
match player
    p if p.health <= 0 => die()
    p if p.health < 25 => show_low_health_warning()
    p => p.update(dt)
end

-- destructuring tuples
match position
    (0, 0) => print("at origin")
    (x, 0) => print("on x-axis at ${x}")
    (0, y) => print("on y-axis at ${y}")
    (x, y) => print("at (${x}, ${y})")
end
```
* **Key things to remember:** `match` is exhaustive. If you forget a variant, the compiler tells you. The Lua-style `match ... end` block (not `match ... { ... }` braces) keeps the language consistent. Patterns can include literal values, variable bindings, tuples, struct destructuring, and guards (`if condition`). The compiler maps `match` to Rust's `match` expression.
* **Example Use Case:** State machines, error handling, AST traversal, message dispatch, parser combinators, any code that has to handle "one of N cases."

---

## 21. Interfaces (Traits)

* **What it is:** A named set of method signatures that a type can implement. Axolotl's equivalent of Rust's `trait`.
* **Why it's important:** The mechanism for polymorphism. Defines a contract - "any type that implements this interface can do these things." Lets you write generic code that works on any implementation.
* **Basic syntax and usage:**
```axol
Drawable = interface
    draw(self, renderer: Renderer)
end

Damageable = interface
    damage(self, amount: Int)
    health(self) -> Int
end

Storage = interface
    get(self, key: String) -> Value?
    set(self, key: String, value: Value)
end
```
* **Key things to remember:** Interfaces map to Rust traits. They can have default method bodies. They can have associated constants and types (via `@assoc type ... end` syntax). The compiler generates `pub trait Drawable { fn draw(&self, renderer: &Renderer); }` directly.
* **Example Use Case:** `Drawable`, `Damageable`, `Storage`, `Iterable`, `Hash`, `Display`, `Clone`, `Serialize` - anything that defines a contract multiple types will satisfy.

---

## 22. Implementation Blocks

* **What it is:** The mechanism for declaring "this type implements this interface" or "this type has these methods."
* **Why it's important:** Connects types to their behavior. Without it, interfaces are empty promises.
* **Basic syntax and usage:**
```axol
Player : Drawable
Player : Damageable

Player.draw = fn(self, renderer)
    renderer.sprite(self.texture, self.position)
end

Player.damage = fn(self, amount)
    self.health -= amount
end

Player.health = fn(self) -> Int
    return self.health
end

-- implementing for foreign types (when needed)
Vec3 : Drawable
Vec3.draw = fn(self, renderer)
    renderer.point(*self)
end
```
* **Key things to remember:** The `Type : Interface` form is the trait declaration. Methods can be attached to the type with `Type.method = fn(self, ...) ... end` (no `impl` block needed). The compiler maps these to ordinary Rust `impl Drawable for Player { ... }` blocks. Foreign types (e.g., types from a Rust crate) can also have methods attached this way.
* **Example Use Case:** Implementing standard interfaces (`Player : Damageable`), implementing your own interfaces, providing default methods, orphan-rule workarounds.

---

## 23. Generics

* **What it is:** A way to write code that works for many types without rewriting it for each one.
* **Why it's important:** The entire Rust standard library, the entire ecosystem, and most of your own code is generic. Without generics, you can't write a single `Array.sort()` that works for arrays of `Int`, `String`, and `Player`.
* **Basic syntax and usage:**
```axol
-- generic struct
Pair = struct<T>
    first: T
    second: T
end

-- generic function
identity = fn<T>(value: T) -> T
    return value
end

-- generic interface
Container = interface<T>
    get(self) -> T?
    put(self, value: T)
end

-- usage (type inferred)
let p = Pair { first = 1, second = 2 }
let s = Pair { first = "a", second = "b" }
let n = identity(42)
let s2 = identity("hello")
```
* **Key things to remember:** Generics are usually inferred. You don't write `Pair<Int>` at the call site unless inference fails. The compiler maps `fn identity<T>(value: T) -> T` to Rust's `fn identity<T>(value: T) -> T` directly. Bounds (`<T: Interface>`) can be added when the generic code requires them.
* **Example Use Case:** Collections (`Array<T>`, `Map<K, V>`), generic algorithms (`sort`, `find`, `map`), reusable utilities, polymorphic data structures.

---

## 24. Constants and Compile-Time Evaluation

* **What it is:** Named values that are computed at compile time and inlined wherever they're used.
* **Why it's important:** Magic numbers hurt readability. Configuration values, lookup tables, compile-time-computed arrays - all want a single source of truth.
* **Basic syntax and usage:**
```axol
const MAX_PLAYERS: Int = 16
const PI: Float = 3.141592653589793
const APP_NAME: String = "MyGame"

-- compile-time expression
const FRAME_BUFFER_SIZE: Int = 1920 * 1080 * 4

-- compile-time array
const POWERS_OF_TWO: Array<Int> = [1, 2, 4, 8, 16, 32, 64, 128]
```
* **Key things to remember:** Constants are evaluated at compile time and inlined. They have no runtime cost and no address. Functions that are pure (no I/O, no side effects, no heap allocation) can be marked `const` and called in constant contexts. The compiler maps these to Rust's `const` items.
* **Example Use Case:** Configuration constants, lookup tables, bit flags, compile-time-generated data, embedded firmware register addresses.

---

## 25. `if / elseif / else / end`

* **What it is:** Conditional execution. The most fundamental control-flow construct.
* **Why it's important:** Every program makes decisions.
* **Basic syntax and usage:**
```axol
if health <= 0 then
    die()
elseif health < 25 then
    show_low_health_warning()
else
    update_hud()
end

-- expression-style (returns a value)
let status = if health > 0 then "alive" else "dead" end
```
* **Key things to remember:** Lua-style: `if cond then ... elseif cond then ... else ... end`. The `then` is required. The `end` is required. `elseif` (one word) - not `else if`. `if/else/end` is also an expression, so it can be assigned to a `let`. The compiler maps to Rust's `if` expression.
* **Example Use Case:** Every conditional in every program.

---

## 26. `while / do / end`

* **What it is:** A loop that runs while a condition is true.
* **Why it's important:** The simplest loop. Unknown iteration count.
* **Basic syntax and usage:**
```axol
let i = 0
while i < 10 do
    print(i)
    i += 1
end
```
* **Key things to remember:** The condition is checked **before** each iteration. If false initially, the body never runs. `break` exits the loop; `continue` skips to the next iteration. The compiler maps to Rust's `while` loop. If you find yourself writing `while true { ... break ... }`, use `loop` instead.
* **Example Use Case:** Event loops, retry loops, "keep doing this until something happens" patterns.

---

## 27. `repeat / until`

* **What it is:** A post-test loop - the body runs at least once, then the condition is checked.
* **Why it's important:** "Do this, then check whether to keep doing it." Lua's `repeat/until` is more natural than `do/while` in C-style languages.
* **Basic syntax and usage:**
```axol
repeat
    line = read_line()
    process(line)
until line == ""
```
* **Key things to remember:** The body always runs at least once. `until cond` runs the body, then exits when `cond` is true (note: opposite sense from `while`). The compiler maps to a Rust `loop { ...; if cond { break; } }`. Useful for input validation, retry-until-success, and post-condition checks.
* **Example Use Case:** Menu loops ("do until user picks quit"), parser bootstrapping, retry with cleanup.

---

## 28. `for` (Numeric and Iterator)

* **What it is:** Two forms: a numeric `for` over a range, and a generic iterator `for` over a collection.
* **Why it's important:** The numeric `for` replaces C-style `for (i = 0; i < n; i++)` cleanly. The iterator `for` is the natural way to traverse arrays, maps, and any iterable.
* **Basic syntax and usage:**
```axol
-- numeric
for i = 1, 10 do
    print(i)
end

-- numeric with step
for i = 0, 1, 0.1 do
    print(i)
end

-- iterator
for enemy in enemies do
    enemy.update(dt)
end

-- iterator with key and value
for index, item in items do
    print("${index}: ${item}")
end
```
* **Key things to remember:** Numeric `for` is inclusive on both ends in Lua-style Axolotl - `for i = 1, 10` runs 1..=10. The iterator `for` is sugar for `into_iter()` on the collection, consuming it by default. If you want a non-consuming iteration, use `for x in collection.iter()` or `for x in &collection`. The compiler maps numeric `for` to a counted Rust loop and iterator `for` to a `for x in &vec` or `for (k, v) in map`.
* **Example Use Case:** Iterating game entities, looping frames, walking AST nodes, processing message queues.

---

## 29. `break` / `continue`

* **What it is:** Loop control - `break` exits the current loop entirely, `continue` skips to the next iteration.
* **Why it's important:** Most real loops need early exit (found the answer, fatal error) or skip-the-rest (skip invalid data).
* **Basic syntax and usage:**
```axol
for i = 1, 100 do
    if found(i) then
        break              -- exit the loop
    end
    if invalid(i) then
        continue           -- skip to the next iteration
    end
    process(i)
end
```
* **Key things to remember:** `break` and `continue` apply to the **innermost** enclosing loop. There is no labeled `break` in Axolotl - refactor into a function or `match` if you need finer control. The compiler maps to Rust's `break` and `continue` directly.
* **Example Use Case:** Search loops, error handling inside a loop, skipping filtered items.

---

## 30. Arrays

* **What it is:** A growable, heap-allocated sequence of elements of a single type.
* **Why it's important:** The most common collection. Lists of enemies, lists of messages, lists of vertices.
* **Basic syntax and usage:**
```axol
let enemies: Array<Enemy> = []
enemies:push(Enemy.spawn())
enemies:push(Enemy.spawn())

let n = enemies:len()
let first = enemies[0]
let last = enemies[n - 1]

-- iteration
for enemy in enemies do
    enemy:update(dt)
end

-- functional
let alive = enemies:filter(fn(e) e.health > 0 end)
let names = enemies:map(fn(e) e.name end)
```
* **Key things to remember:** `Array<T>` is Axolotl syntax for Rust's `Vec<T>`. Indexed access (`enemies[0]`) is bounds-checked in safe builds. Use the method-call syntax (`enemies:push(...)`, `enemies:len()`) for the Lua feel, or the function-call syntax (`enemies.push(...)`) for the Rust feel. Both are valid. The compiler generates ordinary Rust `Vec<T>` operations.
* **Example Use Case:** Lists of anything - game entities, queued messages, parsed tokens, file lines, network packets.

---

## 31. Maps

* **What it is:** A hash map from keys to values.
* **Why it's important:** The natural way to do "look up something by name." Symbol tables, configuration, JSON objects, caches.
* **Basic syntax and usage:**
```axol
let scores: Map<String, Int> = {}
scores:insert("mohi", 100)
scores:insert("alice", 85)

let mine = scores:get("mohi")           -- Int?
let other = scores:get("nobody")        -- nil

for name, score in scores do
    print("${name}: ${score}")
end
```
* **Key things to remember:** `Map<K, V>` is Axolotl syntax for Rust's `HashMap<K, V>`. The `:get` method returns `V?` (nullable), because the key may not exist. Use `:insert` to add, `:remove` to delete, `:contains` to check existence. The compiler generates ordinary Rust `HashMap` operations.
* **Example Use Case:** Configuration objects, lookup tables, caches, JSON object parsing, symbol tables in interpreters.

---

## 32. Iterators

* **What it is:** A lazy sequence of values produced by an underlying collection or generator. Composable: you can `map`, `filter`, `take`, `skip`, `collect` an iterator into another iterator.
* **Why it's important:** The foundation of functional-style collection processing. Zero-cost in Rust (and therefore in Axolotl).
* **Basic syntax and usage:**
```axol
let numbers = [1, 2, 3, 4, 5]

let result = numbers
    :iter()
    :map(fn(n) n * 2 end)
    :filter(fn(n) n > 4 end)
    :collect(Array<Int>)

print(result)            -- [6, 8, 10]

-- summing
let total = numbers:iter():sum()
```
* **Key things to remember:** Iterators are lazy - nothing is computed until you call a consumer (`:collect`, `:sum`, `:count`, `:fold`, etc.). The compiler maps Axolotl iterator chains to Rust iterator chains. All standard Rust iterator adapters are available. No overhead compared with handwritten loops.
* **Example Use Case:** Data pipelines, filtering events, transforming collections, computing aggregates.

---

## 33. Ownership (Inferred)

* **What it is:** The compiler's automatic determination of whether a value is copied, moved, or borrowed - based on how it's used.
* **Why it's important:** This is what lets Axolotl feel "Lua-like" while still being memory-safe. The programmer doesn't write `&` or `&mut` in 95% of cases.
* **Basic syntax and usage:**
```axol
fn damage(player, amount: Int)
    player.health -= amount       -- inferred &mut Player
end

fn describe(player) -> String
    return "player ${player.name}"  -- inferred &Player
end

fn consume(sword)
    inventory.add(sword)            -- move Sword
end

-- usage
let p = Player { health = 100, name = "Mohi" }
damage(p, 20)                      -- works: inferred &mut
let s = describe(p)                 -- works: inferred &
consume(p)                          -- works: inferred move (p can't be used after)
```
* **Key things to remember:** The compiler does the work. If it can't prove the operation is safe, it **asks you** rather than guessing. You can override the inference with `borrow T` / `mut T` / `move T` annotations. There are no lifetime parameters in user code. The compiler emits ordinary Rust borrows underneath.
* **Example Use Case:** Every function in every program. Ownership inference is the single biggest reason Axolotl is easier than Rust.

---

## 34. Explicit Ownership (`borrow`, `mut`, `move`)

* **What it is:** Explicit annotations that override the compiler's ownership inference.
* **Why it's important:** Some advanced cases (self-referential structs, FFI, complex lifetimes) need the programmer to take over.
* **Basic syntax and usage:**
```axol
fn inspect(value: borrow Data)
    print(value.field)
end

fn update(value: mut Data)
    value.field = new_value
end

fn consume(value: move Data)
    send_to_worker(value)
end
```
* **Key things to remember:** `borrow T` is `&T`. `mut T` is `&mut T` (read-write borrow). `move T` takes ownership. These are for advanced cases - most code should let inference work. The compiler still verifies that the explicit choice is safe; you can't bypass the borrow checker, you can only be more specific about what you want.
* **Example Use Case:** Engine code, library APIs, self-referential data structures, FFI, complex lifetime requirements.

---

## 35. References and Borrows

* **What it is:** A non-owning pointer to a value. The alternative to moving.
* **Why it's important:** Lets multiple parts of a program look at the same data without copying it.
* **Basic syntax and usage:**
```axol
fn longest(a: borrow String, b: borrow String) -> borrow String
    if a:len() > b:len() then
        return a
    end
    return b
end

let s1 = "hello"
let s2 = "world"
let result = longest(s1, s2)
print(result)              -- "hello" or "world"
```
* **Key things to remember:** Borrows are `borrow T` (read) and `mut T` (write). The compiler verifies that borrows don't outlive the owner - this is the lifetime inference that happens behind the scenes. The borrow checker's rules are unchanged; only the syntax is friendlier.
* **Example Use Case:** Function parameters (most common), return values, struct fields that point to other data, slices into buffers.

---

## 36. Lifetimes (Inferred, Never Written)

* **What it is:** The compiler's automatic tracking of how long a reference is valid.
* **Why it's important:** Without lifetime tracking, references could outlive the data they point to - use-after-free bugs.
* **Basic syntax and usage:** *(The user never writes lifetimes. They are inferred.)*
```axol
-- this works, lifetimes are inferred
fn longest(a: &String, b: &String) -> &String
    if a:len() > b:len() then a else b end
end

-- this would NOT work, and the compiler tells you why
fn dangling() -> &String
    let s = String.from("hello")
    return s                  -- error: returning reference to local variable
end
```
* **Key things to remember:** The compiler does the lifetime inference. You don't write `'a`, `'static`, or lifetime bounds. If the compiler can't figure out a valid lifetime, it asks you to refactor - usually by changing return types or using `move` semantics. This is the single biggest quality-of-life win over Rust.
* **Example Use Case:** Every function that takes or returns references. The compiler handles the bookkeeping.

---

## 37. Result and Error Handling (`!`, `?`, `??`, `try`/`catch`)

* **What it is:** A family of operators and blocks for working with fallible computations.
* **Why it's important:** Errors happen. The type system must let you handle them, not ignore them.
* **Basic syntax and usage:**
```axol
-- function declaration with error type
fn load(path: String) -> Texture ! IoError

-- propagation: `?` returns the error early
let tex = load("player.png")?

-- coalescing: `??` provides a fallback
let tex = load("missing.png") ?? default_texture()

-- try/catch block
try
    let tex = load("player.png")?
    renderer.draw(tex)
catch e: IoError
    print("failed to load: ${e}")
end

-- type annotation
fn read_file(path: String) -> String ! IoError
    return File.read_all_text(path)?
end
```
* **Key things to remember:** `T ! E` is the type signature for a function that returns `Result<T, E>` in Rust. `?` propagates the error. `??` provides a default for `Option` (or for `Result`, when you don't care which error). `try { ... } catch e: E { ... }` is an explicit error block. The compiler maps these to Rust's `Result` and `?` operator. The error type is checked statically - you can't catch an error that the function can't produce.
* **Example Use Case:** File I/O, network calls, JSON parsing, database queries, anywhere a function can fail in a way the caller should know about.

---

## 38. Option / Nullable Chaining (`?.` and `!.`)

* **What it is:** Two operators for working with `T?` values: `?.` for safe access, `!.` for asserted access.
* **Why it's important:** Lets you reach into a chain of nullable values without writing a dozen `if x then` checks.
* **Basic syntax and usage:**
```axol
let user: User? = find_user(id)
let city: String? = user?.address?.city

let city_name = user?.address?.city ?? "Unknown"

let guaranteed_city = user!.address!.city
```
* **Key things to remember:** `user?.address?.city` short-circuits to `None` if any link in the chain is null. The result is itself `String?`. `user!.address!.city` panics if any link is null. `?? "Unknown"` provides a default. The compiler maps `?.` to `Option::and_then` and `!.` to `Option::unwrap` (or to a checked access in debug builds).
* **Example Use Case:** Configuration that may not be set, optional fields in parsed data, deep field access on a value that may be null.

---

## 39. Coroutines and Tasks

* **What it is:** A function that can be paused and resumed - a lightweight concurrency primitive.
* **Why it's important:** Games need "do this for 3 seconds, then do that." Network code needs "wait for a response, then process it." Coroutines express this naturally.
* **Basic syntax and usage:**
```axol
task enemy_attack = fn()
    wait(1.0)
    player:damage(20)
    wait(2.0)
    enemy_attack()
end

task spawn_waves = fn()
    loop
        spawn_enemy()
        wait(3.0)
    end
end
```
* **Key things to remember:** Tasks compile to Rust futures / state machines under the hood. There is no Axolotl coroutine runtime - the underlying machinery is Tokio (or any other async runtime the user chooses). `wait(seconds)` is a built-in for sleeping. Tasks can call themselves recursively for periodic behavior.
* **Example Use Case:** Game enemy AI, animation timelines, periodic background jobs, "fire and forget" work.

---

## 40. `spawn` and Threads

* **What it is:** Run a function on a new OS thread, or in the background, and get a handle to wait on.
* **Why it's important:** Real programs need to do work in parallel. The compiler guarantees thread safety via Rust's `Send`/`Sync` rules.
* **Basic syntax and usage:**
```axol
let handle = spawn fn()
    for i = 0, 10 do
        print("background: ${i}")
    end
end

handle:join()                  -- wait for it to finish

-- with arguments
let handle = spawn fn(n: Int)
    for i = 0, n do
        compute(i)
    end
end

-- with a Rust thread pool
let handle = spawn::thread_pool(4) fn()
    heavy_computation()
end
```
* **Key things to remember:** `spawn` returns a `Task` or `JoinHandle` you can `await` or `join`. The compiler enforces `Send`/`Sync` automatically - if a captured variable can't be safely sent across threads, the compiler tells you. The runtime used (Tokio? Rayon? raw `std::thread`?) is whatever the user has imported.
* **Example Use Case:** Background I/O, parallel computation, server request handlers, anything that benefits from concurrency.

---

## 41. Channels

* **What it is:** A thread-safe queue for sending messages between tasks.
* **Why it's important:** The classic Go-style concurrency primitive. "Don't communicate by sharing memory; share memory by communicating."
* **Basic syntax and usage:**
```axol
let (tx, rx) = channel::<Message>()

spawn fn()
    tx:send(Message.new("hello"))
end

let msg = rx:recv()?
print(msg)
```
* **Key things to remember:** Channels come from Rust's `std::sync::mpsc` (multi-producer, single-consumer) or `tokio::sync::mpsc` (async). Axolotl's `channel<T>()` is a thin wrapper that picks the right one based on the async context. The compiler enforces that the types sent over the channel are `Send`.
* **Example Use Case:** Worker pools, message passing between threads, producer-consumer pipelines, event buses.

---

## 42. `async` / `await`

* **What it is:** Asynchronous function execution - a function that returns a future, and a way to wait on that future.
* **Why it's important:** Network servers, GUIs, and any program that handles many concurrent I/O-bound operations need async. Without it, every connection would need its own thread.
* **Basic syntax and usage:**
```axol
async fn fetch_url(url: String) -> String ! NetworkError
    let response = await http.get(url)
    return response:text()
end

main = async fn()
    let text = await fetch_url("https://example.com")
    print(text)
end
```
* **Key things to remember:** `async fn` is a function that returns a future. `await` suspends until the future completes. The compiler maps to Rust's `async fn` and `.await`. The async runtime is whatever the user has imported (Tokio, async-std, smol). Axolotl itself does not ship an async runtime.
* **Example Use Case:** Web servers, HTTP clients, database drivers, GUI event loops, any I/O-bound code.

---

## 43. Send / Sync (Inferred)

* **What it is:** Rust's marker traits for thread safety. `Send` means a value can be moved to another thread. `Sync` means a value can be shared (`&T`) across threads.
* **Why it's important:** Prevents data races at compile time.
* **Basic syntax and usage:** *(The programmer never writes these. They are inferred.)*
```axol
let counter = Arc.new(Mutex.new(0))

spawn fn()
    let mut lock = counter:lock()?
    *lock += 1                       -- Arc<Mutex<i32>> is Send + Sync, so this compiles
end
```
* **Key things to remember:** The compiler automatically verifies that values crossing thread boundaries are `Send`, and that values shared across threads are `Sync`. If you try to do something unsafe (e.g., send a `Rc<T>` to another thread), the compiler tells you - and tells you why, and suggests the fix (use `Arc<T>` instead). The user never writes `: Send` or `: Sync` bounds.
* **Example Use Case:** Every concurrent program. The compiler checks it for you.

---

## 44. Unsafe

* **What it is:** An opt-in escape hatch that disables some of the compiler's safety checks.
* **Why it's important:** Some operations are inherently unsafe (raw pointer dereference, FFI, inline assembly). The unsafe block marks them as the programmer's responsibility.
* **Basic syntax and usage:**
```axol
unsafe
    let ptr = alloc(4096) as *mut U8
    ptr[0] = 0xFF
    dealloc(ptr)
end

-- unsafe function declaration
unsafe fn read_register(address: UInt) -> UInt
    let ptr = address as *const UInt
    return *ptr
end
```
* **Key things to remember:** `unsafe` blocks do **not** disable the borrow checker or the type system - they only enable the five "unsafe superpowers": dereference raw pointers, call unsafe functions, access/modify mutable statics, implement unsafe traits, access fields of a `union`. The compiler tracks which code is unsafe and can warn about it. Use `unsafe` sparingly.
* **Example Use Case:** FFI, embedded programming, low-level data structure implementation, performance-critical code that the compiler can't verify.

---

## 45. Raw Pointers

* **What it is:** A pointer with no lifetime or aliasing guarantees - just an address.
* **Why it's important:** C interop, hardware access, custom allocators.
* **Basic syntax and usage:**
```axol
unsafe
    let ptr: *mut U8 = some_address as *mut U8
    ptr[0] = 0xFF                       -- direct memory write
    let value = ptr[0]                  -- direct memory read
end
```
* **Key things to remember:** Raw pointers can only be dereferenced inside `unsafe` blocks. They can be null. They can alias. They have no lifetime. The compiler trusts you, which is the point. Use them when you must; reach for safe alternatives (slices, `Vec`, references) when you can.
* **Example Use Case:** C FFI, hardware register access, custom memory allocators, writing a `Vec<T>`-like data structure from scratch.

---

## 46. Memory Layout (`@repr(C)` etc.)

* **What it is:** An attribute that forces a specific memory layout on a struct or enum.
* **Why it's important:** C interop, hardware interfaces, file formats, network protocols all require exact layouts.
* **Basic syntax and usage:**
```axol
@repr(C)
PacketHeader = struct
    id: U32
    length: U16
    flags: U16
end

@repr(C, packed)
TightStruct = struct
    a: U8
    b: U64
end

@repr(transparent)
Wrapper = struct
    inner: U64
end
```
* **Key things to remember:** `@repr(C)` matches the C ABI. `@repr(C, packed)` removes padding. `@repr(transparent)` means the struct has the same layout as its single field. The compiler maps these to Rust's `#[repr(...)]` attributes. Use them whenever you need to talk to non-Rust code.
* **Example Use Case:** Network packet headers, file format structures, hardware register maps, FFI types, OS-level structures.

---

## 47. Allocation Domains (Stack / Heap / Arena / Persistent)

* **What it is:** Different kinds of memory with different lifetimes. The compiler picks the right one for normal `let` bindings, but the programmer can request a specific one.
* **Why it's important:** Games, simulations, and high-performance servers need fine-grained control over when memory is allocated and freed.
* **Basic syntax and usage:**
```axol
-- normal `let` goes on the stack
let coord = Vec3(0, 0, 0)              -- stack-allocated

-- heap allocation (Box-like)
let owned = Box.new(BigStruct { ... }) -- heap-allocated, freed on drop

-- arena allocation: freed when the arena is cleared
let level_arena = arena()
let enemy = level_arena.create(Enemy.spawn())
let bullet = level_arena.create(Bullet.spawn())
-- ... gameplay ...
level_arena.clear()                    -- all enemies and bullets freed at once

-- persistent: lives for the entire program
let settings = persistent Settings.load()
```
* **Key things to remember:** `arena` blocks and `arena.create(...)` only generate arena code if used. The compiler uses `bumpalo` or a similar arena crate underneath. `persistent` maps to a `static` or `OnceLock`. The lifetime of arena-allocated objects is the arena's lifetime, not lexical scope. This is the pattern games and simulations use to avoid GC pause.
* **Example Use Case:** Game level loading, per-frame temporary data, simulation step scratch space, asset preloading.

---

## 48. RAII and Deterministic Destruction

* **What it is:** A value's destructor runs the moment its owner goes out of scope. No GC pause, no finalizer queue, no garbage collector at all.
* **Why it's important:** Predictable cleanup. File handles close, network sockets release, locks unlock, GPU memory frees - all at a known point in the code.
* **Basic syntax and usage:**
```axol
Resource = struct
    handle: GPUHandle
end

Resource.drop = fn(self)
    gpu.destroy(self.handle)
end

let res = Resource.new()
-- ... use res ...
-- res's drop runs here, at the end of the scope
```
* **Key things to remember:** `drop` is Axolotl's destructor method (mapped to Rust's `Drop`). It runs when the value goes out of scope, or when its owner does. There is no GC, so destructors run **immediately and predictably**. This is what makes Axolotl suitable for real-time systems, game engines, and any program where you need to know exactly when a resource is released.
* **Example Use Case:** File handles, GPU resources, network connections, locks, custom allocators, anything that holds a scarce resource.

---

## 49. FFI: `cblock`

* **What it is:** A block of raw C code embedded in an Axolotl file. The compiler automatically generates the C compilation and linking configuration when this block is present.
* **Why it's important:** Lets you drop into C when you need to - for an existing C library, a hot loop, inline assembly, or vendor SDK.
* **Basic syntax and usage:**
```axol
cblock
    #include <stdio.h>
    #include <stdlib.h>

    int native_add(int a, int b) {
        return a + b;
    }

    void* native_alloc(size_t size) {
        return malloc(size);
    }
end

-- call the C function from Axolotl
let result = native_add(3, 4)
print(result)                          -- 7
```
* **Key things to remember:** A project that contains **no `cblock`** generates **zero C build machinery**. No `cc` configuration, no `build.rs` for C, no linker flags, no C toolchain dependency. The moment you add a `cblock`, Bucket generates the necessary Cargo build configuration. The C functions are exposed as `extern fn` automatically.
* **Example Use Case:** Calling a C library (SDL, libpng, OpenSSL, system APIs), writing a hot loop in C, embedding vendor C code, inline assembly.

---

## 50. FFI: `cppblock`

* **What it is:** A block of raw C++ code. Same opt-in behavior as `cblock`.
* **Why it's important:** A lot of high-performance code is C++ - game engines, physics libraries, audio frameworks.
* **Basic syntax and usage:**
```axol
cppblock
    #include <iostream>
    #include <vector>

    void hello_cpp() {
        std::cout << "Hello from C++" << std::endl;
    }
end
```
* **Key things to remember:** Bucket generates the C++ toolchain configuration only when `cppblock` is present. Linking uses `cxx` or `autocxx` under the hood, depending on the user's needs. C++ name mangling is handled automatically.
* **Example Use Case:** Integrating a C++ engine (Unreal subsystems, custom physics), reusing a C++ library, calling into vendor C++ SDKs.

---

## 51. FFI: `rblock`

* **What it is:** A block of raw Rust code embedded in an Axolotl file.
* **Why it's important:** Lets you drop into Rust for things Axolotl's surface syntax can't express - exotic macros, lifetime gymnastics, experimental features.
* **Basic syntax and usage:**
```axol
rblock
    #[inline]
    pub fn fast_math(x: f32) -> f32 {
        x.sqrt()
    }

    pub fn exotic_macro(input: TokenStream) -> TokenStream {
        -- ... actual Rust code ...
    }
end
```
* **Key things to remember:** `rblock` becomes part of the generated Rust crate directly. No FFI, no wrapping - just Rust in the middle of Axolotl. Use it for procedural macros with DSL arguments, complex trait plumbing, or anything that would be awkward to express in Axolotl's surface.
* **Example Use Case:** Procedural macros, custom derives, lifetime-heavy code, experimental Rust features, low-level optimizations.

---

## 52. FFI: `pyblock`

* **What it is:** A block of raw Python code embedded in an Axolotl file.
* **Why it's important:** Lets you call into Python when you have a Python library you need - ML models, data science tools, scripting, prototyping.
* **Basic syntax and usage:**
```axol
pyblock
    def process_asset(path):
        # ... Python code ...
        return baked_data
end
```
* **Key things to remember:** Bucket generates the Python integration (via PyO3) **only when `pyblock` is present**. A project that never uses `pyblock` has no Python runtime dependency, no PyO3 linking, no `python3-sys` crate. Use it when you need it; ignore it when you don't.
* **Example Use Case:** Calling into NumPy / Pandas / TensorFlow / PyTorch, embedding a Python scripting layer, reusing existing Python tooling, ML model inference.

---

## 53. `extern fn` (C Declarations)

* **What it is:** Declare a function whose implementation is in a foreign library (typically C).
* **Why it's important:** The Axolotl equivalent of Rust's `extern "C" { fn ... }`. Tells the compiler "this function exists; trust me, link will find it."
* **Basic syntax and usage:**
```axol
extern fn SDL_Init(flags: U32) -> Int
extern fn SDL_CreateWindow(title: *const Char, w: Int, h: Int, flags: U32) -> *mut Void
```
* **Key things to remember:** Use raw pointer types (`*const T`, `*mut T`) for C strings and opaque handles. The compiler maps to Rust's `extern "C" { ... }` block. Linking is automatic when the library is in `Bucket.jsonc`'s system dependencies. If a project has no `extern fn` and no `cblock`, there's no C linker story at all.
* **Example Use Case:** Calling any C library - SDL, OpenGL, libuv, system APIs, vendor SDKs.

---

## 54. Attributes (`@derive`, `@repr`, `@inline`, etc.)

* **What it is:** Compile-time annotations on items (structs, functions, etc.) that change their behavior or generate code.
* **Why it's important:** Lets you opt into standard patterns (derives, layout, inlining) without writing boilerplate.
* **Basic syntax and usage:**
```axol
@derive(Debug, Clone, Serialize, Deserialize)
Player = struct
    id: U64
    name: String
    health: Int
end

@repr(C)
Packet = struct
    id: U32
    length: U16
end

@inline
fn fast_compute(x: Int) -> Int
    return x * 2 + 1
end
```
* **Key things to remember:** Attributes use `@` syntax (not Rust's `#[...]` brackets). The compiler maps `@derive(...)` to `#[derive(...)]`, `@repr(C)` to `#[repr(C)]`, etc. Attributes can be combined. Custom attributes (from user-defined proc macros) are supported.
* **Example Use Case:** Standard derives (`Clone`, `Debug`, `Serialize`), memory layout control, inlining hints, feature flags, custom derive macros.

---

## 55. Macros and Procedural Macros

* **What it is:** Compile-time code generation. A macro takes the AST as input and produces new code.
* **Why it's important:** Cuts boilerplate. `#[derive(Serialize)]` generates an entire `Serialize` implementation from a single attribute. Custom macros let you build domain-specific languages.
* **Basic syntax and usage:**
```axol
-- using a derive
@derive(Serialize, Deserialize)
User = struct
    id: U64
    name: String
end

-- calling a function-like macro
let html = html! {
    <div class="container">
        <h1>Hello, ${name}!</h1>
    </div>
}

-- declaring a macro (simple form)
macro_rules! log
    ($level:ident, $($args:tt)*) => {
        print("[${level}]", $($args)*)
    }
end

log!(INFO, "starting up")
log!(ERROR, "failed:", error_msg)
```
* **Key things to remember:** `macro_rules!` for simple syntactic macros, procedural macros for everything else. Procedural macros are imported from Rust crates - you can use any `proc-macro` crate on crates.io. The compiler maps the macro syntax to Rust's macro system.
* **Example Use Case:** Domain-specific languages, reducing boilerplate, compile-time validation, code generation.

---

## 56. Compile-Time Reflection (`@reflect`)

* **What it is:** A compile-time mechanism to inspect a type's fields, methods, and attributes - without needing a runtime reflection system.
* **Why it's important:** Serialization, deserialization, schema generation, debug printing, and ORMs all need to know "what fields does this type have?" Rust's solution is procedural macros; Axolotl adds a friendlier syntax on top.
* **Basic syntax and usage:**
```axol
@reflect
User = struct
    id: U64
    name: String
    email: String?
end

-- @reflect generates compile-time metadata:
-- User.fields = [("id", UInt64), ("name", String), ("email", Option<String>)]
-- User.names = ["id", "name", "email"]

-- you can use this in your own derive-like code
fn to_dict(value: Any) -> Map<String, Any>
    let t = reflect.type_of(value)
    let result = {}
    for name, type in t.fields do
        result[name] = reflect.get(value, name)
    end
    return result
end
```
* **Key things to remember:** `@reflect` is **compile-time only** - there is no runtime reflection. The generated metadata is available at compile time for proc-macros and codegen, and as a const for use in other code. This is the difference from Java/Python-style runtime reflection, which costs performance and breaks optimization.
* **Example Use Case:** Custom serializers, schema generation, debug printers, ORMs, GUI binding, scripting interfaces.

---

## 57. Error Recovery in the Parser

* **What it is:** When the parser hits a syntax error, it doesn't give up - it recovers and continues analyzing the rest of the file.
* **Why it's important:** Without recovery, you fix one error, recompile, see the next error, fix it, recompile... With recovery, you see all the errors at once.
* **Basic syntax and usage:** *(Not something the user writes - it's a compiler feature.)*
```axol
-- Imagine the user wrote this:
foo = whatever(
bar = 123

Player.update = fn()
    ...
end

-- The parser recovers, treats `foo = whatever(` as an unfinished expression,
-- keeps going, and reports BOTH the unfinished expression AND the missing
-- close-paren, in a single compilation pass.
```
* **Key things to remember:** The parser uses standard techniques (synchronization tokens, error nodes in the AST). The type checker and ownership analyzer also try to keep going after errors. The goal is to report every problem in your code in one build, not in N builds.
* **Example Use Case:** Every non-trivial program. Error recovery is a productivity multiplier.

---

## 58. Source Maps and Span Mapping

* **What it is:** A mapping from positions in the generated Rust source back to positions in the original Axolotl source.
* **Why it's important:** When rustc produces an error in the generated code, the user wants to see the line in their `.axol` file - not the line in the auto-generated `.rs`.
* **Basic syntax and usage:** *(Not something the user writes - it's a compiler feature.)*
```axol
-- If the generated Rust is:
--   1: fn main() { player.damage(20); }
--   2:                         ^^ cannot move out of `player`
--   3:
--   4: // generated from src/player.axol:48
--
-- The compiler shows:
--   src/player.axol:48
--     48 │ player:damage(20)
--        │         ^^^^^^^^^^ cannot move out of `player`
```
* **Key things to remember:** Span mapping is automatic. The compiler embeds `#line` directives in the generated Rust so rustc's own error messages also point at the right line. Bucket's diagnostic layer rewrites spans to the Axolotl source before display. The user never sees generated-Rust line numbers.
* **Example Use Case:** Every error in every program. The user experience depends on it.

---

## 59. Mixed `.axol` + `.rs` Projects

* **What it is:** A project that contains both Axolotl source files (`.axol`) and Rust source files (`.rs`) - both are first-class citizens.
* **Why it's important:** Lets teams adopt Axolotl incrementally. Existing Rust code keeps working; new features are written in Axolotl.
* **Basic syntax and usage:**
```text
my-project/
├── Bucket.jsonc
├── bucket.lock
├── Cargo.toml
├── src/
│   ├── main.axol
│   ├── server.axol
│   ├── database.axol
│   ├── crypto.rs        -- pure Rust
│   ├── protocol.rs      -- pure Rust
│   └── hardware.rs      -- pure Rust
└── pond/
```
```axol
-- in main.axol
use "crypto"
use "protocol"
use "hardware"

main = fn()
    hardware:initialize()
    let key = crypto:generate_key()
    let data = protocol:fetch(...)
    let encrypted = crypto:encrypt(data, key)
    protocol:send(encrypted)
end
```
```rust
// in crypto.rs
pub fn generate_key() -> [u8; 32] { ... }
pub fn encrypt(data: &[u8], key: &[u8; 32]) -> Vec<u8> { ... }
```
* **Key things to remember:** Cargo's CLI doesn't know `.axol` exists. Bucket does. The boundary between the two languages is a normal Rust ABI boundary, not a foreign-function interface. Both languages are compiled by the same `rustc` invocation (indirectly - Axolotl compiles to Rust, then both join the Cargo build). This is the most important adoption feature.
* **Example Use Case:** Introducing Axolotl into an existing Rust codebase. Sharing low-level crates between Rust and Axolotl teams. Mixing performance-critical Rust with high-level Axolotl application code.

---

## 60. `bucket emit-rust` (Inspect Generated Rust)

* **What it is:** A Bucket command that dumps the generated Rust source for one or more `.axol` files.
* **Why it's important:** Lets you verify the compiler is doing what you expect. Useful for debugging, for code review, for understanding the compiler, and for handing off generated code to a Rust team.
* **Basic syntax and usage:**
```bash
bucket emit-rust
# Writes the generated Rust to pond/generated/rust/

bucket emit-rust --pretty
# Pretty-prints the generated Rust

bucket emit-rust src/player.axol
# Emits only the generated Rust for player.axol

bucket emit-rust --diff
# Diffs the current generated Rust against the last build
```
* **Key things to remember:** The generated Rust is **ordinary Rust**. A Rust developer should be able to read it and think "yeah, a Rust developer could have written this." No `axolotl_runtime::something_34891(...)` call sites, no `Box<dyn Any>` everywhere, no `unsafe` unless your source required it. If the generated Rust looks ugly, that's a compiler bug - file it.
* **Example Use Case:** Debugging a surprising ownership error. Auditing what the compiler does. Reviewing generated code in a PR. Onboarding Rust developers who want to see what's happening underneath.

---

## 61. The Smart Compiler (Gills / Neoten / Regrow)

* **What it is:** A collection of compiler-driven tooling that doesn't just produce errors but actively helps the programmer.
* **Why it's important:** Most "language design" stops at syntax. The difference between Axolotl and a typical transpiler is that Axolotl treats the **engineering experience** as a first-class design concern.
* **Basic syntax and usage:**
```bash
# Gills: LSP / IDE intelligence
# (runs in the background, powers autocomplete, diagnostics, go-to-def)

# Neoten: linter
bucket lint
# warning[unused-import]: import 'Vec3' is never used
# warning[inefficient-collection]: use Array<Enemy> instead of Map<Int, Enemy>
# warning[unnecessary-type-annotation]: type can be inferred

# Regrow: automatic fixes
bucket fix
# Regrow analyzed 14 diagnostics.
# ✓ 8 fixes applied
# ✓ 3 imports repaired
# ✓ 2 type annotations removed
# ✓ 1 ownership rewrite suggested (interactive)
```
* **Key things to remember:** These tools share the compiler's semantic model. They understand ownership, types, traits, and the Axolotl ↔ Rust relationship. Neoten can find Axolotl-specific issues that Rust's Clippy can't. Regrow can suggest and apply fixes - including ownership rewrites ("did you mean to borrow this instead of move it?"). Gills makes all of this available in your editor via LSP.
* **Example Use Case:** Daily development. CI checks. Code review assistance. Onboarding new team members. Refactoring legacy Axolotl code.

---

## 62. The Pond Cache

* **What it is:** A pnpm-style global content-addressed cache for compiled crate artifacts.
* **Why it's important:** Cargo's `target/` directory can balloon to 20-60 GB per project because every project recompiles every dependency. The Pond cache reuses compiled artifacts across projects.
* **Basic syntax and usage:**
```bash
# Bucket manages the cache transparently
bucket build              # uses pond cache, fast incremental builds

bucket build --release    # fresh release artifacts, no debug reuse

bucket clean              # nuke project/pond/
bucket cache clean        # gc the global ~/.bucket/pond/
bucket cache gc           # automatic garbage collection

# Inspect cache
bucket cache info
# Global Pond: 12.4 GB, 847 artifacts
# Project pond: 240 MB
# Cache hit rate: 73%
```
* **Key things to remember:** The cache key is comprehensive (source hash + dep graph + features + rustc version + target + profile + flags). Same key → reuse. Different key → recompile. Debug profiles share; release profiles are isolated. The cache lives at `~/.bucket/pond/`; the project workspace is `project/pond/`.
* **Example Use Case:** Any project that depends on a lot of crates. CI systems. Multi-project monorepos. Reducing disk usage on developer machines.

---

## 63. The Bucket CLI

* **What it is:** The user-facing command-line tool for Axolotl projects. Replaces Cargo for most workflows.
* **Why it's important:** Most users live in `bucket ...` commands, not `cargo ...` commands. Bucket understands `.axol` files, manages the Pond cache, drives the smart compiler, and exposes the entire Axolotl toolchain.
* **Basic syntax and usage:**
```bash
bucket new my-project              # create a new project
bucket init                        # adopt an existing folder
bucket add wgpu                    # add a dependency
bucket remove wgpu                 # remove one
bucket update                      # refresh the lockfile
bucket upgrade                     # migrate to a new edition
bucket tree                        # dependency tree
bucket search "graphics"           # search registry

bucket build                       # build (debug, cached, incremental)
bucket build --release             # build (release, fresh artifacts)
bucket check                       # type-check only
bucket run                         # build + run
bucket test                        # run tests
bucket bench                       # run benchmarks

bucket fmt                         # run Shed
bucket lint                        # run Neoten
bucket fix                         # run Regrow

bucket doc                         # run Ambystoma
bucket doc --open                  # open in browser

bucket clean                       # nuke project/pond
bucket cache clean                 # gc the global pond
bucket doctor                      # health check
bucket logs                        # view build history
bucket logs --errors               # extract diagnostics
bucket logs --latest               # open the most recent

bucket publish                     # publish to Eggbox
bucket package                     # build a release artifact

bucket watch                       # Larva / live-reload
bucket install my-tool             # install a binary
bucket uninstall my-tool

bucket cargo ...                   # forward to Cargo
bucket rust ...                    # forward to rustc

bucket emit-rust                   # dump the generated Rust
```
* **Key things to remember:** `bucket cargo <anything>` is a transparent passthrough. The user never loses access to the underlying Cargo / rustc surface. Every Bucket command has `--plain` (CI-friendly text output) and `--json` (machine-readable) modes.
* **Example Use Case:** Every workflow. This is the entry point.

---

## 64. Bucket.jsonc Manifest

* **What it is:** The project manifest. JSONC (JSON with comments) instead of TOML.
* **Why it's important:** Easier for most people to read and write than TOML. Comment-friendly. The user shouldn't have to learn a new file format.
* **Basic syntax and usage:**
```jsonc
{
    // Project metadata
    "name": "my-project",
    "version": "0.1.0",

    // Axolotl language configuration
    "language": {
        "edition": "2026"
    },

    // Native Rust crates
    "dependencies": {
        "tokio": "^1",
        "serde": "^1",
        "wgpu": "^27"
    },

    // Optional Axolotl features
    "features": {
        "telemetry": true
    },

    // Build configuration
    "build": {
        "target": "x86_64-unknown-linux-gnu",
        "release": {
            "lto": "thin",
            "codegen-units": 1
        }
    }
}
```
* **Key things to remember:** Bucket reads `Bucket.jsonc` and generates the corresponding `Cargo.toml` for Cargo. The user rarely needs to look at the generated `Cargo.toml`. Bucket can `bucket fmt` the manifest for consistent formatting.
* **Example Use Case:** Configuring any project. Documenting dependencies. Setting build options. Enabling features.

---

## 65. Bucket.lock Lockfile

* **What it is:** A lockfile that records the exact versions and content hashes of every dependency in the resolved graph.
* **Why it's important:** Reproducible builds. If `serde 1.0.219` is in the lockfile, every build uses exactly that version.
* **Basic syntax and usage:**
```jsonc
{
    "version": 1,
    "dependencies": {
        "serde": {
            "version": "1.0.219",
            "source": "crates.io",
            "hash": "sha256:..."
        },
        "tokio": {
            "version": "1.47.1",
            "source": "crates.io",
            "hash": "sha256:..."
        }
    }
}
```
* **Key things to remember:** Commit `bucket.lock` to source control. `bucket update` refreshes the lockfile to the latest compatible versions. `bucket upgrade` migrates to a new major version. The lockfile records content hashes, not just version numbers - so a yanked or compromised version can't silently sneak in.
* **Example Use Case:** Reproducible CI builds. Team development. Production deployments. Security auditing.

---

## 66. Foreign Crates Just Work

* **What it is:** Importing a Rust crate (`wgpu`, `tokio`, `serde`, anything) and using its API from Axolotl without writing FFI bindings.
* **Why it's important:** The day-one library ecosystem. No "Luma std library" to build. No "Axolotl SDK" to maintain. Every Cargo crate is an Axolotl library.
* **Basic syntax and usage:**
```axol
use "tokio"
use "serde"
use "wgpu"
use "nalgebra"

-- generic
fn process<T: Serialize>(value: T) -> String
    return serde_json::to_string(value)?
end

-- async
async fn fetch(url: String) -> String
    let response = await reqwest::get(url)
    return response:text().await?
end
```
* **Key things to remember:** The compiler imports the crate's `.rmeta` metadata, understands its actual API, and generates the right Rust calls. Generics, traits, async, associated types, macros - all work. You don't write a parallel Axolotl wrapper for `wgpu`. You import `wgpu`.
* **Example Use Case:** Literally every Axolotl program that does anything. This is the feature that makes Axolotl viable on day one.

---

## 67. The "Use the Real Rust Types" Rule

* **What it is:** When Axolotl needs a type that Rust already provides (`Arc`, `Mutex`, `Vec`, `HashMap`, `Future`, ...), it imports the Rust type rather than shipping a parallel Axolotl type.
* **Why it's important:** Keeps the runtime out of the binary. Avoids "two of everything" duplication. Ensures the generated Rust is readable.
* **Basic syntax and usage:**
```axol
use "std::sync::Arc"
use "std::sync::Mutex"
use "std::collections::HashMap"

counter = Arc.new(Mutex.new(0))

data: HashMap<String, Int> = {}
data:insert("key", 42)
```
* **Key things to remember:** There is no `AxolArc`, `AxolMutex`, `AxolRefCell`, `AxolVec`, `AxolHashMap`, `AxolFuture`. The rule is: **if Rust already provides it, Axolotl doesn't reinvent it**. This is the rule that keeps the generated Rust readable and the binary small.
* **Example Use Case:** Concurrent counters, shared state, custom data structures, anything that needs the standard Rust toolbox.

---

## 68. The "No Runtime" Invariant

* **What it is:** There is no Axolotl runtime, period. Not a minimal one. Not an optional one. None.
* **Why it's important:** Makes the "0 overhead" claim honest. Makes the language suitable for real-time, embedded, and resource-constrained environments. Makes the generated Rust auditable.
* **Basic syntax and usage:** *(The user's source code is the contract.)*
```axol
-- This program has NO Axolotl runtime linked into it:
fn add(a: Int, b: Int) -> Int
    return a + b
end

-- It compiles to:
-- pub fn add(a: i64, b: i64) -> i64 { a + b }
-- That's it. No runtime, no prelude, no GC, nothing.
```
* **Key things to remember:** If you need an `Arc`, import Rust's `Arc`. If you need a `Mutex`, import Rust's `Mutex`. If you need an async runtime, import Tokio. If you need nothing, the compiler doesn't secretly inject something. The full list of things that don't exist: VM, interpreter, GC, scheduler, object system, heap, reflection runtime, mandatory standard runtime.
* **Example Use Case:** Every Axolotl program. This is the design constraint that defines the language.

---

## 69. The "What You Don't Use Doesn't Exist" Principle

* **What it is:** Every feature in Axolotl is opt-in at the build level. Unused features cost nothing.
* **Why it's important:** Keeps binaries small. Keeps build times fast. Keeps the toolchain simple.
* **Basic syntax and usage:**
```axol
-- A project with no foreign blocks: no C, no C++, no Python build machinery.
-- A project with no async: no Tokio pulled in.
-- A project with no arena: no arena code generated.
-- A project with no derive(Serialize): no serde pulled in.
```
* **Key things to remember:** This principle applies at every level. Foreign blocks (`cblock`/`cppblock`/`rblock`/`pyblock`) only generate build machinery when present. Dependencies are only pulled in when imported. Features are compile-time, not runtime. The cache links, not copies. The build script only exists when needed.
* **Example Use Case:** Every project. The user pays only for what they use.

---

## 70. The Zero-Overhead Promise

* **What it is:** Axolotl introduces no inherent runtime overhead compared with equivalent idiomatic Rust.
* **Why it's important:** The "Lua-like syntax, C-like performance" claim has to be real, not marketing.
* **Basic syntax and usage:** *(The compiler's output is the contract.)*
```axol
player.health -= damage
-- compiles to:
player.health -= damage;
-- exactly. No runtime call. No method dispatch. No boxing. No hidden indirection.
```
* **Key things to remember:** The Axolotl compiler is a **zero-cost abstraction**. If you write a simple loop, you get a simple loop in the generated Rust. If you write a complex iterator chain, you get the same iterator chain Rust's optimizer can handle. There is no Axolotl-specific overhead - no GC, no boxed values, no reflection, no hidden dispatch, no managed runtime. The runtime cost is whatever the generated Rust costs.
* **Example Use Case:** Every program. This is what makes "Lua-simple, Rust-fast" an honest pitch.

---

## The end of the bible

Every concept in this document was decided during the conversation. Every pivot, every reframe, every correction is captured here:

- The pivot from game-DSL to general-purpose language.
- The pivot from transpile-to-source to Cargo frontend.
- The pivot from "Rust with nicer syntax" to "Lua extended."
- The "no runtime" rule.
- The "use the real Rust types" rule.
- The "what you don't use doesn't exist" principle.
- The nullable types (`T?`).
- The inferred ownership.
- The four foreign blocks (`cblock`, `cppblock`, `rblock`, `pyblock`).
- The Bucket / Pond / Gills / Neoten / Shed / Regrow / Ambystoma / Pond / Larva / Molt / Salamander / Eggbox toolchain.
- The `Bucket.jsonc` manifest.
- The Pond global cache.
- The mixed `.axol` + `.rs` projects.
- The smart compiler.
- The Gills/axol-analyzer LSP story (see `LSP.md`).
- The high-level enjoyable features (see `HighLevel.md`).

If you can read this document end to end, you understand what Axolotl is. Welcome.

---

# Part II - The High-Level Surface

> **The enjoyable layer.** The features that make Axolotl feel like a *new* language rather than a theme. Every feature in this part is sugar over the spine - it compiles to ordinary idiomatic Rust. Every feature has a verbose equivalent the user can drop down to.

---

## 71. Match Replacements (the "less code than match" toolkit)

* **What it is:** A family of four constructs that achieve the same dispatch as `match` but with less code: `case` syntax, variant-handler methods, dispatch tables, and the original `match` for when you really need it.
* **Why it's important:** `match` is the right tool sometimes, but it's verbose for the common case. The user asked: *"Match is okay but still who wants to write too much code. Why is there no other solution that achieves the same thing as match but less code?"* The answer is this toolkit.
* **Basic syntax and usage:**
```axol
-- Form 1: 'case' syntax (terse match)
case state
of Menu: show_menu()
of Playing: tick_game()
of Paused: show_pause()
of GameOver(score, reason): print("${score}: ${reason}")
end

-- Form 2: variant-handler methods (match as dispatch)
GameState:on_menu = fn() show_menu() end
GameState:on_playing = fn() tick_game() end
GameState:on_paused = fn() show_pause() end
GameState:on_game_over = fn(score, reason)
    print("score: ${score}, reason: ${reason}")
end
state:handle()                  -- compiler generates a match

-- Form 3: dispatch tables (Lua-style, statically checked)
let handlers = {
    Menu: fn() show_menu() end
    Playing: fn() tick_game() end
    Paused: fn() show_pause() end
    GameOver: fn(score, reason) print("${score}: ${reason}") end
}
state:dispatch(handlers)

-- Form 4: original 'match' (when you really need it)
match state
    Menu => show_menu()
    Playing => tick_game()
    Paused => show_pause()
    GameOver(score, reason) => print("${score}: ${reason}")
end
```
* **Key things to remember:** All four forms compile to the same Rust `match`. All four are exhaustively checked by the compiler - adding a variant to the enum triggers a compile error in every form that doesn't handle it. Pick the form that reads best in your code. See Sections 72, 73, 74 for the detailed forms.
* **Example Use Case:** State machines, message dispatch, parser combinators, event handling - anywhere the same enum is matched in multiple places, variant-handler methods shine. For one-off dispatch, dispatch tables. For simple arms, `case`. For complex patterns with guards, `match`.

---

## 72. `case` Syntax (Terse Match)

* **What it is:** A shorter form of `match` with three syntactic reductions: `of` instead of `Pattern =>`, `:` instead of `=>`, and no braces around single-expression bodies.
* **Why it's important:** Reduces the visual noise of `match` while preserving all of its power. Reads naturally for simple arm lists.
* **Basic syntax and usage:**
```axol
case shape
of Circle(r): area_circle(r)
of Square(s): s * s
of Rect(w, h): w * h
end

-- with default
case shape
of Circle(r): area_circle(r)
of _: 0.0
end

-- with single-statement bodies
case event
of KeyDown(k): handle_key(k)
of MouseDown(p): handle_click(p)
of Resize(w, h): resize(w, h)
end
```
* **Key things to remember:** `case` is exactly `match` with sugar. Exhaustiveness is enforced. Guards are allowed (`of X if cond => ...`). Multi-statement bodies need `do ... end` inside the arm. The compiler maps to Rust's `match`.
* **Example Use Case:** State machines with simple per-variant logic, message dispatch with one or two lines per arm, parser token classification.

---

## 73. Variant-Handler Methods (Match as Dispatch)

* **What it is:** Instead of writing a `match`, you attach a method to the type for each variant. The method name follows the convention `on_<variant>` (or any name you choose). The call site becomes a single method call.
* **Why it's important:** When the same dispatch is used in many places, variant-handler methods let you define the logic once on the type. Open-closed principle applied to enums.
* **Basic syntax and usage:**
```axol
GameState = enum
    Menu
    Playing
    Paused
    GameOver(score: Int, reason: String)
end

-- attach handlers
GameState:on_menu = fn(self) show_menu() end
GameState:on_playing = fn(self) tick_game() end
GameState:on_paused = fn(self) show_pause() end
GameState:on_game_over = fn(self, score, reason)
    print("score: ${score}, reason: ${reason}")
end

-- call site
state:handle()    -- the compiler generates a match that dispatches to the right handler
state:handle_with_side_effects()  -- you can have multiple handler methods per type
```
* **Key things to remember:** The compiler verifies that every variant has a handler. If you add a new variant, the compiler points at every `:handle()` call site and the relevant type and says "add a handler for the new variant." The handlers can have any names - `on_X` is convention, not requirement. The compiler generates a normal Rust `match` underneath.
* **Example Use Case:** State machines reused across many functions, message dispatch, event handling for an entire game object, anything where the same enum is matched in many places.

---

## 74. Dispatch Tables (Lua-Style, Statically Checked)

* **What it is:** A table literal mapping each variant of an enum to a function. The compiler statically verifies that every variant has a handler in the table.
* **Why it's important:** Sometimes you don't want to commit to a method on the type - the dispatch is specific to one call site. Dispatch tables give you the Lua-style ad-hoc dispatch with full static type checking.
* **Basic syntax and usage:**
```axol
let handlers = {
    Menu: fn() show_menu() end
    Playing: fn() tick_game() end
    Paused: fn() show_pause() end
    GameOver: fn(score, reason) print("${score}: ${reason}") end
}

state:dispatch(handlers)
state:dispatch(handlers, with_side_effects = true)
```
* **Key things to remember:** The compiler statically verifies exhaustiveness. If a variant is added to the enum, the compiler points at every dispatch table and says "add a handler." Dispatch tables can be passed around, stored in data structures, and conditionally selected. The compiler generates a normal Rust `match` underneath.
* **Example Use Case:** Plugin systems, configurable dispatch logic, scenarios where the same enum is dispatched differently in different contexts (e.g., debug-mode handlers vs production-mode handlers).

---

## 75. `let-else` (Early Return Patterns)

* **What it is:** A pattern-matching form of `let` that propagates upward on failure. `let Some(x) = opt else return default()` binds `x` if `opt` is `Some(x)`, otherwise returns `default()` from the enclosing function.
* **Why it's important:** Removes the most common source of "pyramid of doom" in error-handling code. Replaces deeply-nested `if let` blocks with linear, top-down control flow.
* **Basic syntax and usage:**
```axol
fn load_config(path: String) -> Config ! IoError
    let text = read_file(path)?            -- propagate the error
    let json = parse_json(text)?           -- propagate the error
    let Some(name) = json.name else return Config.default()   -- early return
    let Ok(version) = json.version else return Config.default()
    Config { name, version }
end

-- with a value
fn user_name(id: Int) -> String
    let user = find_user(id) else return "Anonymous"
    let name = user.name else return "Anonymous"
    name
end

-- multi-pattern
let (Some(a), Some(b)) = (opt_a, opt_b) else return default()
```
* **Key things to remember:** `let-else` is the same as Rust's `let-else` but with the body of the `else` being a single expression (no braces needed for one-liners). The else body can return any value, which becomes the function's return when the pattern fails. Compiles to ordinary Rust `let-else`.
* **Example Use Case:** Any function that has multiple early-return conditions, any code that unwraps a chain of optional values, parser code that has multiple "expected X got Y" failure modes.

---

## 76. Pipe `|>` and Reverse Application `<|`

* **What it is:** `x |> f` means `f(x)`. `x |> f |> g` means `g(f(x))`. The pipe passes the left-hand value as the **last** argument to the right-hand function. Reverse application passes it as the **first**.
* **Why it's important:** Lets data flow top-to-bottom instead of inside-out. Makes data transformation pipelines read like a series of steps.
* **Basic syntax and usage:**
```axol
-- basic pipe
let result = data |> parse |> validate |> transform |> save

-- reverse application
let result = save <| transform <| validate <| parse <| data

-- with method calls
let result = data
    |> String.trim
    |> String.to_upper
    |> String.split(" ")

-- with closures
let result = numbers
    |> filter(fn(n) n > 0 end)
    |> map(fn(n) n * 2 end)
    |> sum()

-- as the first argument
let config = defaults
    |> merge(user_config)
    |> validate

-- as a partial application
let add_one = 1 |> (+)     -- not idiomatic; use lambdas
```
* **Key things to remember:** `|>` is right-associative; `x |> f |> g` is `g(f(x))`. The pipe compiles to a plain function call - zero overhead. Works with any callable: functions, methods, closures, partial applications. The compiler maps `a |> f` to `f(a)`.
* **Example Use Case:** Data transformation pipelines, builder-style method chains, configuration loading, parsing + validation, any place where a chain of function calls would otherwise be deeply nested.

---

## 77. Named Arguments

* **What it is:** Function calls can use `name = value` arguments, in any order, with any subset of optional parameters.
* **Why it's important:** Makes function calls self-documenting. Eliminates the "wait, was the third argument `team` or `position`?" problem.
* **Basic syntax and usage:**
```axol
fn spawn_actor(
    name: String,
    health: Int = 100,
    speed: Float = 5.0,
    position: Vec3 = Vec3(0, 0, 0),
    team: String = "neutral"
)

-- call sites
spawn_actor(name = "Hero")
spawn_actor(name = "Hero", health = 200)
spawn_actor(team = "red", name = "Villain", health = 50)
spawn_actor(name = "Boss", position = Vec3(100, 0, 0))
spawn_actor("Hero", health = 200)         -- positional first, then named
```
* **Key things to remember:** The compiler verifies that all required parameters are provided, no parameter is provided twice, all names are valid, and all values are type-compatible. Positional arguments must come before named ones. Method calls also support named arguments. The compiler maps to ordinary Rust function calls.
* **Example Use Case:** Any function with more than two parameters, especially when the parameters are similar types (multiple `Int`s or `Float`s), configuration-style functions, builders, anything where reading the call site matters.

---

## 78. Default Parameter Values

* **What it is:** Parameters can have default values. Callers can omit them.
* **Why it's important:** Eliminates the "one function with five overloads" pattern. Makes optional parameters natural instead of builder-ceremony.
* **Basic syntax and usage:**
```axol
fn create_window(
    title: String = "Untitled",
    width: Int = 800,
    height: Int = 600,
    resizable: Bool = true,
    fullscreen: Bool = false,
    vsync: Bool = true
)

-- calls
create_window()                                      -- all defaults
create_window("My Game")                             -- title only
create_window("My Game", fullscreen = true)          -- override just one
create_window(width = 1920, height = 1080)           -- named, in any order

-- defaults can reference earlier parameters
fn rect(x: Int, y: Int, width: Int = 100, height: Int = width)
    -- 'height' defaults to the same value as 'width'
end
```
* **Key things to remember:** Defaults are evaluated at call time, not at definition time. Defaults can reference earlier parameters. Defaults cannot reference later parameters, generic parameters, or `self`. The compiler synthesizes a builder internally; the generated Rust is the same as if you'd written the builder by hand.
* **Example Use Case:** Configuration functions, factory functions, optional parameters, anything with reasonable defaults that callers want to override sometimes.

---

## 79. Spread Operator (`...`)

* **What it is:** `...expr` in a function call, table literal, array literal, or struct literal expands `expr` into the surrounding context. For arrays: spread elements. For tables: spread key-value pairs. For structs: spread fields.
* **Why it's important:** Composable data construction. Lets you build new collections from existing ones without manual copying.
* **Basic syntax and usage:**
```axol
-- spread in function calls
let args = [1, 2, 3]
print(...args)                              -- prints 1 2 3 (space-separated, with newline)

let opts = { host = "localhost", port = 8080 }
connect(...opts)                            -- connect(host = "localhost", port = 8080)

-- spread in table literals
let defaults = { color = "red", size = 10, weight = "normal" }
let custom = { ...defaults, size = 20 }     -- color and weight from defaults, size overridden

-- spread in array literals
let first = [1, 2, 3]
let second = [4, 5, 6]
let all = [...first, ...second, 7, 8, 9]     -- [1, 2, 3, 4, 5, 6, 7, 8, 9]

-- spread in struct literals (the "update syntax")
let base = Player { name = "Hero", health = 100, speed = 5.0, position = Vec3(0, 0, 0) }
let harder = base{ health = 200, speed = 10.0 }   -- shorthand for Player { ...base, health = 200, speed = 10.0 }
```
* **Key things to remember:** Later keys override earlier ones in the spread chain. The compiler statically verifies type compatibility. Works in function calls (positional or named), array literals, table/map literals, and struct literals. The shorthand `base{ ... }` is sugar for "copy all fields, then override the listed ones."
* **Example Use Case:** Building configurations from defaults, merging user options, copying structs with modifications, combining collections.

---

## 80. Destructuring Assignment

* **What it is:** A `let` can bind multiple variables at once, pulling fields out of structs, elements out of tuples, or entries out of maps.
* **Why it's important:** Eliminates the `let x = obj.x; let y = obj.y;` boilerplate. Makes data decomposition a one-liner.
* **Basic syntax and usage:**
```axol
-- struct destructuring
let { name, health } = player
print(name, health)        -- two locals, types inferred

-- rename during destructuring
let { name: player_name, health: hp } = player

-- tuple destructuring
let (x, y, z) = position
let (head, ..tail) = list
let (first, second, ..rest) = items

-- nested destructuring
let { address: { city, country } } = user
print(city, country)

-- with default values
let { name, role = "guest" } = user    -- role defaults to "guest" if missing

-- in function parameters
fn describe({ name, health }: Player) -> String
    "Player ${name} with ${health} HP"
end

-- in for loops
for { name, score } in players do
    print("${name}: ${score}")
end
```
* **Key things to remember:** The compiler verifies the destructuring against the type. Renaming uses `field: new_name` syntax. Default values use the same `=` syntax as default parameters. Works in `let`, function parameters, and `for` loops. The compiler maps to ordinary Rust field access / tuple destructuring.
* **Example Use Case:** Pulling multiple fields out of a struct, pattern-matching in function parameters, iterating over collections of records.

---

## 81. If-Let Chains

* **What it is:** Combine multiple `if let`s with `and` to form a single condition. The chain short-circuits on the first failure.
* **Why it's important:** Removes the deep nesting of "if let Some(x) = ... { if let Some(y) = x.y { ... } }" patterns.
* **Basic syntax and usage:**
```axol
-- chained
if let Some(user) = find(id) and let Some(name) = user.name and name ~= "" then
    print("hello, ${name}")
end

-- with else
if let Some(user) = find(id) and user.active then
    use(user)
else
    print("user not found or inactive")
end

-- with else if chains
if let Some(user) = find_admin() then
    show_admin_panel()
else if let Some(user) = find_moderator() then
    show_mod_panel()
else if user then
    show_user_panel()
else
    show_login()
end
```
* **Key things to remember:** The chain short-circuits on the first failed `let` or false condition. The compiler desugars the chain into nested `if let` expressions with `&&`. No special runtime machinery; just a clearer syntax for the common pattern.
* **Example Use Case:** Optional chaining through several layers, validation chains, multi-step lookup patterns, any place where several optional values must all be present.

---

## 82. List / Map / Set Comprehensions

* **What it is:** Python-style syntax for building collections from iterators. `[expr for x in iter]` builds a list. `{key: value for ...}` builds a map. `{expr for ...}` builds a set.
* **Why it's important:** Replaces the imperative "create empty collection, for-loop, push" pattern with a single declarative line.
* **Basic syntax and usage:**
```axol
-- list comprehension
let alive = [e for e in enemies if e.health > 0]
let positions = [e.position for e in entities]
let names = [e.name for e in entities where e.name ~= ""]
let squares = [x * x for x = 0, 100]

-- map comprehension
let by_name = { e.name: e for e in entities }
let scores = { name: score for name, score in players }

-- set comprehension
let unique_tags = { e.tag for e in entities }

-- multiple iterators
let pairs = [(a, b) for a in first for b in second]
let grid = [(x, y) for x in 0..10 for y in 0..10]
let flat = [item for row in grid for item in row]

-- async comprehensions
let pages = [await page async for page in fetch_all_pages()]
```
* **Key things to remember:** Compiles to Rust iterator chains. The `if` and `where` clauses are both filters. Multiple iterators are nested (cartesian product). Async comprehensions use `async for` / `await`. The compiler maps to ordinary `iter().filter().map().collect()` patterns.
* **Example Use Case:** Filtering collections, transforming data, building lookup tables, extracting subsets, any place where a for-loop-with-push would otherwise appear.

---

## 83. Method Cascading

* **What it is:** Call multiple methods on the same value without repeating the receiver. The leading `:` on each method makes the receiver implicit.
* **Why it's important:** Makes builder-style code readable. The classic "fluent interface" without the repetition.
* **Basic syntax and usage:**
```axol
-- without cascading
let b1 = builder.set_name("Hero")
let b2 = b1.set_health(100)
let b3 = b2.set_position(Vec3(0, 0, 0))
let result = b3.build()

-- with cascading
let result = builder
    :set_name("Hero")
    :set_health(100)
    :set_position(Vec3(0, 0, 0))
    :build()

-- with pipe
let result = player
    :update(dt)
    :take_damage(20)
    |> check_death
    |> trigger_respawn

-- for fluent builders
let p = PlayerBuilder:default()
    :set_name("Hero")
    :set_health(200)
    :set_position(Vec3(10, 0, 0))
    :build()
```
* **Key things to remember:** Each `:` method in the chain receives the previous expression as `self`. Methods used in cascading should return `mutref Self` or `self` for chaining. The `-> self` shorthand in method declarations generates `-> &mut Self` underneath. Compiles to ordinary Rust method calls. The compiler maps `obj:m1():m2()` to `obj.m1().m2()` (but using the `mutref` returned by `m1`).
* **Example Use Case:** Builder pattern, configuration assembly, fluent interfaces, any sequence of state mutations on the same object.

---

## 84. Multi-line / Raw / Tagged Strings

* **What it is:** Three additional string literal forms. Multi-line strings (triple-quoted) preserve newlines and auto-strip indentation. Raw strings (prefix `r`) disable escape sequences. Tagged strings (prefix function name) call a function with the string.
* **Why it's important:** Removes the need for `+ "\n" +` concatenation in queries, SQL, HTML, JSON, regexes, and DSLs.
* **Basic syntax and usage:**
```axol
-- multi-line strings
let query = """
    SELECT *
    FROM users
    WHERE active = true
    ORDER BY created_at DESC
    """

-- raw strings
let regex = r"^\d+\.\d+$"
let path = r"C:\Users\Mohi\Documents\file.txt"
let json = r#"{"name": "Mohi", "age": 20}"#
let tricky = r##"contains a "quote" and a #hash"##    -- more #s for more embedded #s

-- byte strings
let bytes = b"\xDE\xAD\xBE\xEF"
let bytes_str = b"hello"

-- format strings (interpolation)
let msg = "Player ${player.name} has ${player.health} HP"
let table = """
    | ${name:>20} | ${health:>5} |
    """

-- tagged strings
let html = h"<div>${name}</div>"
let sql = s"SELECT * FROM ${table_name}"
let md = md"# ${title}\n\n${body}"

-- the prefix is a function call; any function that accepts String can be used
```
* **Key things to remember:** Multi-line strings auto-strip the leading indentation of the least-indented line. Raw strings use `#`s to handle embedded delimiters - more `#`s in the delimiter means more `#`s allowed in the content. Format specifiers follow Rust's `format!` syntax (`:>5`, `:<10`, `:.2`). Tagged strings are just function calls; the prefix is a function name. The compiler maps to ordinary `String` operations.
* **Example Use Case:** SQL queries, HTML / JSON / Markdown generation, regexes, file paths, DSLs, anything that needs embedded newlines or special characters.

---

## 85. Number-Suffix Literals (`4_KB`, `16_ms`, ...)

* **What it is:** Numeric literals can have a suffix that multiplies the value by a known constant. `4_KB` is `4096`. `16_ms` is a `Duration` of 16 milliseconds.
* **Why it's important:** Makes sizes and durations human-readable. The compiler still sees ordinary integers / floats.
* **Basic syntax and usage:**
```axol
let buffer_size = 4_KB              -- 4096 (UInt64)
let max_memory = 1_GB               -- 1,073,741,824
let frame_budget = 16_ms            -- 16 milliseconds (Duration)
let timeout = 30_sec                -- 30 seconds
let retry_after = 5_min             -- 5 minutes
let daily_limit = 1_day             -- 86,400 seconds
let discount = 0.5_pct              -- 0.005
let width = 100%                    -- 1.0

-- custom suffixes
const unit KIB = 1024
const unit MIB = 1024 * KIB
const unit GIB = 1024 * MIB

let mem = 8_GIB                     -- 8,589,934,592
```
* **Key things to remember:** Built-in suffixes cover bytes, bits, time, percent, and ratio. Users can define their own with `const unit NAME = expression`. The compiler maps to ordinary integer / float / Duration constants. Zero runtime cost - the suffix is purely a source-level convenience.
* **Example Use Case:** Buffer sizes, timeouts, network quotas, memory limits, anything where the human meaning of the number matters more than the raw value.

---

## 86. Documentation Comments (`---`)

* **What it is:** Triple-dash comments attach documentation to the next declaration. `Ambystoma` (the doc generator) processes them; Gills shows them in hovers; editors render them in IntelliSense.
* **Why it's important:** Makes documentation a first-class part of the source. The doc comment is the doc; you don't write it twice.
* **Basic syntax and usage:**
```axol
--- Adds two numbers together.
---
--- @param a the first number
--- @param b the second number
--- @return their sum
--- @example
---   add(1, 2)        -- 3
fn add(a: Int, b: Int) -> Int
    a + b
end

--- # Player Module
---
--- Contains the `Player` struct and all related types.
---
--- ## Quick start
---
--- ```axol
--- let p = Player.new()
--- p:update(dt)
--- ```

module player
    ...
end
```
* **Key things to remember:** Tags include `@param`, `@return`, `@throws`, `@example`, `@see`, `@deprecated`, `@since`, `@todo`, `@performance`, `@safety`, `@complexity`. Doc comments support full Markdown. Module-level docs come before the `module` declaration. Examples in doc comments can be marked runnable; `Salamander` runs them as tests.
* **Example Use Case:** Documenting any public API, library documentation, generating a docs site with `bucket doc`, hover help in editors.

---

## 87. Inline Tests and Examples

* **What it is:** Test code lives next to the code it tests, in `test` and `example` blocks. `Salamander` runs them; Gills surfaces them in the editor.
* **Why it's important:** Keeps tests close to the code they test. The user can see the test right next to the function. CI runs them with `bucket test`.
* **Basic syntax and usage:**
```axol
fn add(a: Int, b: Int) -> Int
    a + b
end

test add
    assert add(1, 2) == 3
    assert add(0, 0) == 0
    assert add(-5, 5) == 0
end

example add
    print(add(1, 2))        -- 3
end

-- with descriptive name
test "add returns correct sum for positive inputs"
    assert add(1, 2) == 3
    assert add(100, 200) == 300
end

-- with attributes
test add
    @should_panic
    assert panic_function() == 1

    @ignore("reason")
    assert slow_test() == 1

    @timeout(1000)            -- 1 second
    assert slow_test() == 1
end
```
* **Key things to remember:** Tests and examples are first-class - they're not in a separate file. Gills shows green checkmarks / red Xs inline. "Run test" / "Debug test" code lenses appear above each test. `@should_panic`, `@ignore`, `@timeout` provide common control. Examples in doc comments can also be runnable.
* **Example Use Case:** Unit tests for every function, regression tests, runnable documentation, CI integration via `bucket test`.

---

## 88. Auto-Derive Common Traits

* **What it is:** The compiler automatically implements `Copy`, `Clone`, `Debug`, `Default`, `Eq`, `PartialEq`, `Hash`, `Send`, `Sync` for types where all fields implement them.
* **Why it's important:** Eliminates the `@derive(Clone, Debug, ...)` ceremony for the common case. The user gets the right traits automatically.
* **Basic syntax and usage:**
```axol
-- no @derive needed
Point = struct
    x: Int
    y: Int
end

-- all of these just work
let p1 = Point { x = 1, y = 2 }
let p2 = p1                              -- copied (Copy was auto-derived)
let p3 = p1:clone()                      -- explicit clone
print(p1)                                -- Debug

-- opt out
@no_auto_impl(Clone)
SecretData = struct
    inner: Vec<U8>       -- Vec is Clone, but we don't want SecretData to be
end
```
* **Key things to remember:** Auto-derivation only happens when all fields implement the trait. `@no_auto_impl(TraitName)` opts out. `@derive(...)` still works for explicit control. Auto-derived traits are just as good as hand-written ones - the compiler synthesizes them at compile time. The user can also use `@derive` to add traits not in the auto-derive list.
* **Example Use Case:** POD types, configuration structs, value types - anywhere you'd want the standard set of traits without ceremony.

---

## 89. Quick Constructors (`Type:new`)

* **What it is:** The compiler auto-generates a `new` constructor for every struct, taking the struct's fields as parameters in declaration order.
* **Why it's important:** Eliminates the "should I write a `new` function or use the struct literal?" decision. The constructor is always there.
* **Basic syntax and usage:**
```axol
Player = struct
    name: String
    health: Int
    speed: Float
    position: Vec3
end

-- auto-generated: fn new(name: String, health: Int, speed: Float, position: Vec3) -> Player
let p = Player:new("Hero", 100, 5.0, Vec3(0, 0, 0))

-- with named args
let p = Player:new(name = "Hero", health = 200)

-- newtype (single-field struct)
UserId = struct
    inner: U64
end
let id = UserId(42)                      -- also valid
let id = UserId:new(42)

-- required fields without defaults are mandatory; defaulted ones are optional
```
* **Key things to remember:** Required fields without defaults must be provided. Fields with defaults are optional. Named arguments work. The constructor is auto-generated; if the user defines their own `new`, theirs takes precedence. The compiler maps to ordinary Rust struct construction.
* **Example Use Case:** Quick construction, builder-replacement (with cascading), testing, anywhere a struct literal would otherwise appear.

---

## 90. Struct Update Syntax

* **What it is:** Create a new struct from an old one with the shorthand `old{ field = new_value, ... }`. All unspecified fields are copied from `old`.
* **Why it's important:** Replaces the `Player { ...base, field = new }` boilerplate with a cleaner syntax. The type name is implicit.
* **Basic syntax and usage:**
```axol
let base = Player { name = "Hero", health = 100, speed = 5.0, position = Vec3(0, 0, 0) }

let boss = base{
    health = base.health * 5
    name = "Boss " .. base.name
}

-- equivalent to:
let boss = Player {
    ...base
    health = base.health * 5
    name = "Boss " .. base.name
}

-- combined with method cascading
let buffed = player
    :take_damage(-50)
    { health = 200 }
```
* **Key things to remember:** The shorthand `old{ ... }` is sugar for "copy all fields, then override the listed ones." The compiler verifies the result has all required fields. Compiles to ordinary Rust struct construction with explicit field assignments.
* **Example Use Case:** Modifying one or two fields of an existing struct, "evolve this enemy into a stronger one" patterns, builder-step output, test fixtures.

---

## 91. The High-Level Surface Summary

| Feature | Replaces | When to use |
|---|---|---|
| `case` | `match` with simple arms | One-off dispatch with one or two lines per arm |
| Variant-handler methods | `match` used in many places | Reused dispatch logic |
| Dispatch tables | `match` used in one place | Ad-hoc dispatch specific to a call site |
| `match` | (itself) | Complex patterns, guards, expression-position use |
| `let-else` | Nested `if let ... else { return ... }` | Early-return chains, optional unwrapping |
| `\|>` / `<\|` | Nested function calls | Data transformation pipelines |
| Named arguments | Positional args with magic numbers | Any function with >2 parameters, especially similar-typed ones |
| Default values | Builder pattern for optional params | Optional parameters, configuration functions |
| Spread | Array concatenation, manual field copying | Composing data, copying-with-overrides |
| Destructuring | Manual `let x = obj.x; let y = obj.y;` | Pulling multiple fields, pattern-matching in parameters |
| If-let chains | Nested `if let` | Multiple optional values, validation chains |
| Comprehensions | Imperative `for` + `push` | Building collections declaratively |
| Method cascading | Repeated `let x = x.method()` | Builders, fluent interfaces, state mutation chains |
| Multi-line / raw / tagged strings | `+ "\n" +` concatenation | Queries, HTML, JSON, regexes, DSLs |
| Number-suffix literals | `4096`, `16 * 1000 * 1000` | Sizes, durations, human-meaningful numbers |
| Doc comments | Separate doc files | Public API documentation |
| Inline tests | Separate test files | Unit tests, regression tests, runnable docs |
| Auto-derive traits | `@derive(Clone, Debug, ...)` for common cases | POD types, value types |
| Quick constructors | Manual `fn new(...)` | Quick struct construction |
| Struct update syntax | `Player { ...base, field = new }` | Modifying one or two fields |

Every feature compiles to ordinary idiomatic Rust. Every feature has a verbose equivalent. Every feature is opt-in. The high-level surface is the enjoyable layer.

**This is the answer to the user's question: "Why is there no other solution that achieves the same thing as match but less code?"** There are now four. Plus a dozen more like them. Pick the one that reads best.
