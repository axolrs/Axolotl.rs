# The High-Level Surface

Axolotl's spine is Rust's semantics (ownership, traits, generics,
zero-cost abstractions, no GC). The surface is Lua's syntax (`if/then/end`,
`function ... end`, `:` method calls). On top of that, Axolotl adds a third
layer: **enjoyable ergonomics** - the small, opinionated, programmer-friendly
features that turn "I can write this in 20 lines" into "I can write this
in 5."

This document is the reference for that third layer.

## Match Replacements

The `match` construct in Axolotl is more powerful than Rust's. It can
also replace `if let`, `while let`, `map_or_else`, `unwrap_or_else`, and
several other Rust idioms.

```lua
match result
    Ok(value) then process(value)
    Err(e) then log_error(e)
end
```

The compiler desugars this into the most efficient Rust - typically an
`if let` chain - without generating a closure or allocating.

## let-else

```lua
let Some(value) = parse(input)
else
    return Err("parse failed")
end
```

If the pattern does not match, the `else` branch runs. It must diverge
(`return`, `break`, `continue`, `error()`). The bound variable is
available after the `else` block.

## Pipe Forward

```lua
let result = input
    |> parse
    |> validate
    |> transform
    |> serialize
```

This desugars to `serialize(transform(validate(parse(input))))`. The
compiler inlines the chain; there is no overhead vs. the nested form.

## Pipe Backward

```lua
let result = output <| transform(input)
```

This desugars to `transform(input, output)` - useful for "fill in the last
argument" patterns like `Vec::push`.

## Named Arguments

```lua
function greet(name: String, greeting: String = "Hello", punctuation: String = "!")
    print(greeting .. ", " .. name .. punctuation)
end

greet("Alice")
greet("Bob", greeting: "Hi")
greet("Carol", punctuation: "?")
greet("Dave", greeting: "Hey", punctuation: ".")
```

Named arguments can appear in any order after the positional ones. They
must have default values.

## Comprehensions

```lua
let squares = [for i in 10 do i * i end]
let evens = [for i in 100 do if i % 2 == 0 then i end]
let pairs = { for i in 10 do i: i * i end }
```

List comprehensions, conditional comprehensions, and map comprehensions
all desugar to `for` loops with a `push` at the end.

## Destructuring

```lua
let (x, y) = (1, 2)
let Point { x, y } = point
let [a, b, c] = array
let (a, b, ...rest) = tuple     -- rest is a slice of the remaining
```

Destructuring works on tuples, structs, arrays, and slices. The `...rest`
pattern binds the remaining elements to a slice.

## String Interpolation

```lua
let name = "Alice"
let age = 30
print("${name} is ${age} years old")
```

String interpolation desugars to `format!("{} is {} years old", name, age)`
in Rust. The compiler performs the same lifetime inference.

## Type Inference

```lua
let x = 5                  -- Int
let y = 2.5                -- Float
let z = [1, 2, 3]          -- [Int]
let f = fn(x) return x * 2 end  -- fn(Int) -> Int
```

Type annotations are optional. The compiler infers them from context.

## Auto-derive

```lua
#[derive(Show, Eq)]
struct Point
    x: Int
    y: Int
end
```

The `#[derive(...)]` attribute auto-implements the listed traits. Available
traits: `Show`, `Eq`, `Ord`, `Hash`, `Default`, `Clone`, `Debug`.

## Struct Update

```lua
let p1 = Point { x = 1, y = 2 }
let p2 = Point { x = 10, ..p1 }   -- p2 = Point { x = 10, y = 2 }
```

The `..p1` syntax copies the fields not explicitly set from `p1`.

## Variant Handlers

```lua
match shape
    Circle(r) then return 3.14159 * r * r
    Square(s) then return s * s
    Rectangle(w, h) then return w * h
end
```

Each variant handler is a single expression. The compiler generates the
most efficient dispatch (typically a jump table).

## Dispatch Tables

```lua
local ops = {
    add = fn(a, b) return a + b end,
    sub = fn(a, b) return a - b end,
    mul = fn(a, b) return a * b end,
}

let result = ops[operation](x, y)
```

Dispatch tables are first-class. The compiler inlines the lookup when the
key is known at compile time.

## Error Propagation

```lua
let value = parse_int(input)?       -- propagates Err
let fallback = parse_int(input) ?? 0   -- default on Err
```

`?` is the propagation operator - it returns the inner value on `Ok` and
early-returns the `Err` on `Err`. `??` is the default operator - it
returns the inner value on `Ok` and the right-hand side on `Err`.

## try / catch

```lua
try
    let value = risky_operation()
    print(value)
catch e
    print("caught: " .. e.message)
end
```

`try` blocks catch errors. The `catch` block binds the error to a
variable.

## Number Suffixes

```lua
let x = 5i8
let y = 5i16
let z = 5i32
let w = 5i64           -- same as just `5`
let a = 5u8
let b = 5u32
let c = 5.0f32
let d = 5.0f64         -- same as just `5.0`
```

Number suffixes pin the type. The default integer is `Int` (i64); the
default float is `Float` (f64).

## Doc Comments

```lua
--| Computes the natural log of x with a domain check.
function ln(x: Float) -> Float
    if x <= 0 then
        error("ln domain error: ${x}")
    end
    return math.ln(x)
end
```

Doc comments start with `--|`. They are attached to the following item.
`bucket doc` (Ambystoma) collects them and generates API documentation.

## Inline Tests

```lua
function fib(n: Int) -> Int
    --| test: fib(0)
    --| expected: 0
    --| test: fib(10)
    --| expected: 55
    if n < 2 then return n end
    return fib(n - 1) + fib(n - 2)
end
```

Inline tests live next to the code they test. `bucket test` collects them
and runs them with the rest of the test suite.

## What's Next

- Read the [Language Reference](/docs/language-reference) for the full
  syntax.
- Read the [Ownership](/docs/ownership) guide for the ownership model.
- Read the [Examples](/docs/examples) page for complete worked examples.
