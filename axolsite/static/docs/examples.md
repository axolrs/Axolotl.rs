# Examples

A collection of complete Axolotl examples. Each one is a small, self-contained
program that demonstrates a specific feature or pattern.

## Hello, World

```lua
function main()
    print("Hello, Axolotl!")
end
```

## Variables and Arithmetic

```lua
function main()
    let x = 5
    let y = 10
    var sum = x + y
    var product = x * y
    print("sum=" .. sum .. " product=" .. product)
end
```

## Functions

```lua
function add(a: Int, b: Int) -> Int
    return a + b
end

function main()
    print(add(3, 4))
end
```

## Multiple Returns

```lua
function divmod(a: Int, b: Int) -> (Int, Int)
    return a / b, a % b
end

function main()
    let q, r = divmod(17, 5)
    print("17 / 5 = " .. q .. " remainder " .. r)
end
```

## Structs

```lua
struct Point
    x: Int
    y: Int
end

function Point:magnitude() -> Float
    return (self.x * self.x + self.y * self.y).sqrt()
end

function main()
    let p = Point { x = 3, y = 4 }
    print("magnitude = " .. p:magnitude())
end
```

## Enums and Pattern Matching

```lua
enum Shape
    Circle(Float)
    Square(Float)
    Rectangle(Float, Float)
end

function area(s: Shape) -> Float
    match s
        case Circle(r) then
            return 3.14159 * r * r
        case Square(side) then
            return side * side
        case Rectangle(w, h) then
            return w * h
    end
end

function main()
    let shapes = [
        Shape.Circle(5.0),
        Shape.Square(4.0),
        Shape.Rectangle(3.0, 4.0),
    ]
    var total = 0.0
    for s in shapes do
        total = total + area(s)
    end
    print("total area = " .. total)
end
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

function main()
    let c = make_counter()
    print(c())    -- 1
    print(c())    -- 2
    print(c())    -- 3
end
```

## Vector Operations

```lua
function main()
    let nums = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
    var sum = 0
    for n in nums do
        sum = sum + n
    end
    print("sum = " .. sum)

    let evens = [for n in nums do if n % 2 == 0 then n end]
    print("evens = " .. evens.len())
end
```

## Error Handling

```lua
function parse_int(s: String) -> Result<Int, String>
    -- ...
    return Ok(42)
end

function main()
    let input = "42"
    match parse_int(input)
        case Ok(n) then
            print("parsed: " .. n)
        case Err(e) then
            print("error: " .. e)
    end
end
```

## Async

```lua
async function fetch(url: String) -> String
    -- ...
    return "hello"
end

async function main()
    let body = await fetch("https://example.com")
    print(body)
end
```

## Cargo Crate Integration

```lua
use "serde"

struct User
    name: String
    age: Int
end

function main()
    let user = User { name = "Alice", age = 30 }
    let json = serde.serialize(user)
    print(json)
end
```

## Foreign Block (C)

```lua
cblock {
    #include <stdio.h>
    void hello_from_c() { printf("hello from C\n"); }
}

function main()
    c.hello_from_c()
end
```

## Foreign Block (Rust)

```lua
rblock {
    use std::sync::Arc;
    fn make_arc() -> Arc<()> { Arc::new(()) }
}

function main()
    let arc = r.make_arc()
    print("arc created")
end
```

## Fibonacci (Tail-Recursive)

```lua
function fib(n: Int) -> Int
    var a = 0
    var b = 1
    var i = 0
    while i < n do
        let next = a + b
        a = b
        b = next
        i = i + 1
    end
    return a
end

function main()
    for i in 20 do
        print("fib(" .. i .. ") = " .. fib(i))
    end
end
```

## Concurrent Map

```lua
async function process(item: Int) -> Int
    return item * item
end

async function main()
    let inputs = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
    let tasks = [for i in inputs do spawn process(i) end]
    var total = 0
    for task in tasks do
        total = total + (await task)
    end
    print("total = " .. total)
end
```

## Implementing a Trait

```lua
trait Show
    function show(self) -> String
end

impl Show for Point
    function show(self) -> String
        return "(" .. self.x .. ", " .. self.y .. ")"
    end
end

function main()
    let p = Point { x = 3, y = 4 }
    print(p:show())
end
```

## Using the Pipe Operator

```lua
function parse(s: String) -> Int
    return s.to_int()
end

function validate(n: Int) -> Int
    if n < 0 then error("negative") end
    return n
end

function transform(n: Int) -> Int
    return n * 2
end

function main()
    let result = "42"
        |> parse
        |> validate
        |> transform
    print(result)
end
```

## What's Next

- Read the [Language Reference](/docs/language-reference) for the full
  syntax.
- Read the [Toolchain](/docs/toolchain) guide for the `bucket` command
  reference.
- Read the [Benchmarks](/benchmarks) page to see real Axolotl programs
  side by side with their Rust counterparts.
