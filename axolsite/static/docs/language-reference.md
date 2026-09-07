# Language Reference

This document is the complete reference for the Axolotl (`.axol`) programming
language. Every construct, every type, every operator - with examples.

Axolotl's syntax is Lua-flavored (`if/then/end`, `function ... end`, `:`
method calls, multiple returns) but the semantics are Rust-grade (static
typing, ownership, traits, generics, zero-cost abstractions, no GC). Axolotl
compiles to idiomatic Rust - the generated code is readable and uses the
Cargo ecosystem directly.

## File Extension

`.axol` - every Axolotl source file uses this extension.

## Top-Level Items

A source file is a sequence of top-level items: constants, functions,
structs, enums, interfaces, type aliases, modules, macros, and foreign
blocks.

### Constants

```lua
const WORKLOAD: Int = 200_000
const PI: Float = 3.14159
const GREETING = "hello, axolotl"
```

Constants are immutable and computed at compile time.

### Functions

```lua
function add(a: Int, b: Int) -> Int
    return a + b
end

function main()
    print(add(1, 2))
end
```

Functions can take typed parameters and return a single value. Multiple
returns are packed into a tuple:

```lua
function divmod(a: Int, b: Int) -> (Int, Int)
    return a / b, a % b
end
```

Functions may be nested. Nested functions close over their outer scope.

### Structs

```lua
struct Point
    x: Int
    y: Int
end

struct Rectangle
    origin: Point
    width: Int
    height: Int
end
```

Struct fields are private by default. The `pub` keyword makes a field visible
outside the module.

### Enums

```lua
enum Shape
    Circle(Float)
    Square(Float)
    Rectangle(Float, Float)
end
```

Enum variants can carry tuple data, like Rust's enums.

### Interfaces

```lua
interface Drawable
    function draw(self)
    function area(self) -> Float
end
```

Interfaces can have default method bodies. A type implements an interface
by providing all of its methods (either explicitly or via the default
body).

### Type Aliases

```lua
type UserId = Int
type Score = Float
```

### Modules

```lua
module geometry
    struct Point { x: Int, y: Int }

    function distance(a: Point, b: Point) -> Float
        let dx = a.x - b.x
        let dy = a.y - b.y
        return (dx * dx + dy * dy).sqrt()
    end
end
```

Modules group related items. They can be imported with `use`.

### Macros

```lua
macro assert_nonneg(x)
    if x < 0 then
        error("negative value: " .. x)
    end
end
```

Macros expand at compile time. They are hygienic.

### Foreign Blocks

Axolotl can embed foreign code via four block kinds:

```lua
cblock {
    #include <stdio.h>
    void hello() { printf("hello from C\n"); }
}

cppblock {
    #include <vector>
    std::vector<int> make_vec() { return {1, 2, 3}; }
}

rblock {
    use std::sync::Arc;
    fn make_arc() -> Arc<()> { Arc::new(()) }
}

pyblock {
    import sys
    print(sys.version)
}
```

Foreign blocks only generate build machinery when present. A project with
no `cblock` has no C toolchain configuration, no `cc` crate, no `build.rs`
for C, no linker flags.

## Types

| Type | Description |
|------|-------------|
| `Int` | 64-bit signed integer (`i64` in Rust) |
| `Float` | 64-bit floating point (`f64` in Rust) |
| `Bool` | Boolean (`true` / `false`) |
| `String` | Owned UTF-8 string (`String` in Rust) |
| `[T]` | Vector (`Vec<T>` in Rust) |
| `[T; N]` | Fixed-size array (`[T; N]` in Rust) |
| `T?` / `Option<T>` | Optional value |
| `Result<T, E>` | Success or error |
| `(T, U)` | Tuple |
| `&T` / `borrow T` | Shared reference |
| `mut T` | Mutable reference |
| `move T` | Move (consumes ownership) |
| `fn(A) -> B` | Function pointer |

## Variables

```lua
let x = 5             -- immutable
var y = 10            -- mutable
let z: Int = 15       -- typed
var w: Float = 2.5
```

`let` bindings are immutable. `var` bindings are mutable. Type annotations
are optional - Axolotl infers them.

## Operators

| Operator | Meaning |
|----------|---------|
| `+` `-` `*` `/` `%` | Arithmetic |
| `&` `|` `^` | Bitwise AND, OR, XOR |
| `<<` `>>` | Bitwise shift left, right |
| `~` | Bitwise NOT |
| `==` `!=` `<` `<=` `>` `>=` | Comparison |
| `and` `&&` | Logical AND |
| `or` `\|\|` | Logical OR |
| `not` `!` | Logical NOT |
| `..` `..=` | Range (exclusive, inclusive) |
| `\|>` | Pipe forward (apply function on right to left) |
| `<\|` | Pipe backward |

`^` is XOR - not exponentiation. Use `math.pow(base, exp)` for powers.

## Control Flow

### if / elseif / else / end

```lua
if x > 0 then
    print("positive")
elseif x < 0 then
    print("negative")
else
    print("zero")
end
```

### while / do / end

```lua
var i = 0
while i < 10 do
    print(i)
    i = i + 1
end
```

### for / in / do / end

```lua
for i in 10 do          -- 0..10 (exclusive)
    print(i)
end

for i in 1..=10 do      -- 1..=10 (inclusive)
    print(i)
end

for item in array do
    print(item)
end
```

### repeat / until

```lua
repeat
    x = x + 1
until x >= 100
```

### break and continue

```lua
for i in 100 do
    if i % 7 == 0 then
        break
    end
    if i % 2 == 0 then
        continue
    end
    print(i)
end
```

### match / case / end

```lua
match shape
    case Circle(r) then
        return 3.14159 * r * r
    case Square(s) then
        return s * s
    case Rectangle(w, h) then
        return w * h
    else
        return 0
end
```

`match` is exhaustive. The compiler rejects non-exhaustive matches.

### try / catch / end

```lua
try
    let result = parse_int(input)
catch e
    print("parse failed: " .. e.message)
end
```

### Error propagation

```lua
let value = parse_int(input)?      -- propagate on Err
let fallback = parse_int(input) ?? 0   -- default on Err
```

## Closures

```lua
function make_counter() -> fn() -> Int
    var count = 0
    return fn() -> Int
        count = count + 1
        return count
    end
end

local c = make_counter()
print(c())    -- 1
print(c())    -- 2
print(c())    -- 3
```

Closures capture their environment by reference. The compiler infers
whether the capture should be by-value or by-reference.

## Async

```lua
async function fetch(url: String) -> String
    -- ...
end

async function main()
    let body = await fetch("https://example.com")
    print(body)
end
```

`spawn` launches a task and returns a handle. `await` blocks on a task.

```lua
let task = spawn fetch("https://example.com")
let body = await task
```

## Pattern Matching

### Destructuring

```lua
let (x, y) = (1, 2)
let Point { x, y } = point
let [a, b, c] = array
```

### Variant Handlers

```lua
match shape
    Circle(r) then print("circle radius " .. r)
    Square(s) then print("square side " .. s)
end
```

### Dispatch Tables

```lua
local ops = {
    add = fn(a, b) return a + b end,
    sub = fn(a, b) return a - b end,
    mul = fn(a, b) return a * b end,
}

local result = ops[operation](x, y)
```

## Methods and Receivers

```lua
struct Counter
    count: Int
end

function Counter:increment()
    self.count = self.count + 1
end

function Counter:value() -> Int
    return self.count
end

local c = Counter { count = 0 }
c:increment()
c:increment()
print(c:value())     -- 2
```

`:` is the method call operator. `self` is the receiver. The compiler
infers whether `self` should be `borrow Self`, `mut Self`, or `move Self`
based on usage.

## Generics

```lua
function identity<T>(x: T) -> T
    return x
end

struct Stack<T>
    items: [T]
end

function Stack:push(item: T)
    self.items.push(item)
end
```

## Traits

```lua
trait Show
    function show(self) -> String
end

impl Show for Point
    function show(self) -> String
        return "(" .. self.x .. ", " .. self.y .. ")"
    end
end
```

Traits are explicit. The compiler resolves them at compile time and
monomorphizes the calls.

## Ownership

Axolotl infers ownership automatically. The three modes are:

- `borrow T` - shared reference (Rust `&T`)
- `mut T` - mutable reference (Rust `&mut T`)
- `move T` - owned (Rust `T`)

```lua
function foo(x: borrow Point) -> Int
    return x.x + x.y      -- read-only access
end

function bar(x: mut Point)
    x.x = 10              -- mutation allowed
end

function baz(x: move Point) -> Int
    return x.x + x.y      -- consumes x
end
```

The compiler emits the appropriate Rust lifetime annotations. The user
does not write lifetime annotations directly.

## FFI

Foreign blocks (`cblock`, `cppblock`, `rblock`, `pyblock`) embed foreign
code. The compiler generates the corresponding build machinery only when
the block is present.

```lua
cblock {
    int add(int a, int b) { return a + b; }
}

function main()
    let result = c.add(1, 2)
    print(result)
end
```

## Imports

```lua
use "serde"             -- Cargo crate
use "tokio::sync::Mutex"   -- specific path
use geometry.distance   -- local module
```

Cargo crates are imported by name. Local modules are imported by path.

## Comments

```lua
-- This is a line comment.

-- This is a multi-line comment.
-- It uses consecutive line comments.
```

Block comments are not supported; use consecutive `--` line comments
instead. Inline comments inside function bodies are not allowed (the CI
enforces this). The summary goes above the function signature, not inline.

## Example

A complete example - compute the n-th Fibonacci number:

```lua
function fib(n: Int) -> Int
    if n < 2 then
        return n
    end
    var a = 0
    var b = 1
    var i = 2
    while i <= n do
        let next = a + b
        a = b
        b = next
        i = i + 1
    end
    return b
end

function main()
    for i in 20 do
        print(i, fib(i))
    end
end
```

## What's Next

- Read the [Ownership](/docs/ownership) guide for the ownership model in
  depth.
- Read the [High-Level Surface](/docs/high-level) for the enjoyable
  ergonomics layer.
- Read the [Toolchain](/docs/toolchain) guide for the `bucket` command
  reference.
