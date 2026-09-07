# Ownership in Axolotl

Axolotl inherits Rust's ownership system. There is no GC, no reference
counting injected by the compiler, no hidden heap. Every value has exactly
one owner; when the owner goes out of scope, the value is dropped. The
compiler infers ownership automatically - the user rarely writes explicit
lifetime annotations.

## The Three Modes

Axolotl has three ownership modes for function parameters and return
values:

| Mode | Rust equivalent | Semantics |
|------|----------------|-----------|
| `borrow T` | `&T` | Shared reference. Read-only. Multiple borrows allowed. |
| `mut T` | `&mut T` | Mutable reference. Exclusive. |
| `move T` (or just `T`) | `T` | Owned. Consumes the value. |

## Examples

### Borrow

```lua
function length(p: borrow Point) -> Int
    return p.x + p.y      -- read-only access
end

function main()
    let p = Point { x = 1, y = 2 }
    print(length(p))      -- borrows p
    print(length(p))      -- still ok; p is not consumed
end
```

### Mutable borrow

```lua
function move_to(p: mut Point, x: Int, y: Int)
    p.x = x              -- mutation allowed
    p.y = y
end

function main()
    var p = Point { x = 0, y = 0 }
    move_to(p, 10, 20)
    print(p.x, p.y)      -- 10, 20
end
```

### Move

```lua
function consume(p: move Point) -> Int
    return p.x + p.y      -- p is consumed; caller can no longer use it
end

function main()
    let p = Point { x = 1, y = 2 }
    let total = consume(p)
    -- print(p.x)        -- ERROR: p has been moved
    print(total)
end
```

## Inference

The compiler infers the ownership mode from usage. If you only read from
the parameter, it's `borrow`. If you mutate, it's `mut`. If you store or
return it, it's `move`.

```lua
function distance(a: Point, b: Point) -> Float
    -- a and b are inferred as borrow Point because they are only read
    let dx = a.x - b.x
    let dy = a.y - b.y
    return (dx * dx + dy * dy).sqrt()
end
```

## The Borrow Checker

The Rust borrow checker still runs. Axolotl infers the ownership, but the
underlying Rust borrow checker verifies the generated code is safe. If the
inferred ownership is incorrect (e.g., you tried to mutate while something
else holds a borrow), the compiler rejects the program.

The user can write explicit ownership annotations to override the inference:

```lua
function process(data: mut Buffer)     -- force mutable borrow
function consume(data: move Buffer)    -- force move
function peek(data: borrow Buffer)     -- force shared borrow
```

## Moves and Clones

When a value is moved, the original binding is no longer usable. To make
a copy, use `.clone()`:

```lua
let original = make_big_vector()
let copy = original.clone()      -- explicit clone, allocates new memory
process(original)                -- moves original
-- print(original.len())         -- ERROR: original has been moved
print(copy.len())                -- ok
```

For types that implement `Copy` (integers, floats, booleans, etc.), the
value is copied automatically on assignment - no `.clone()` needed.

## Lifetimes

The compiler infers lifetimes automatically. The user does not write
lifetime annotations. If a lifetime conflict arises, the compiler reports
it as an Axolotl-level error with the `.axol` source location.

## Patterns

### Accumulator

```lua
function sum(items: borrow [Int]) -> Int
    var total = 0
    for item in items do
        total = total + item
    end
    return total
end
```

`items` is borrowed because we only read it. The function does not consume
the array.

### Builder

```lua
struct StringBuilder
    buffer: String
end

function StringBuilder:append(self: mut StringBuilder, s: borrow String)
    self.buffer = self.buffer .. s
end
```

`self` is `mut` because we mutate it. `s` is `borrow` because we only read
it (then clone it into the buffer via `..`).

### Pipeline

```lua
function process(input: move String) -> move String
    let stage1 = stage_one(input)     -- input moved into stage_one
    let stage2 = stage_two(stage1)    -- stage1 moved into stage_two
    return stage2                    -- stage2 moved to caller
end
```

Each stage consumes the previous stage's output. This is a "pipeline"
pattern; the compiler optimizes away the moves.

## What's Next

- Read the [Language Reference](/docs/language-reference) for the full
  syntax.
- Read the [High-Level Surface](/docs/high-level) for the enjoyable
  ergonomics.
- Read the [Comparison](/docs/comparison) to see how Axolotl compares to
  Rust, Lua, TypeScript, and others.
