# HighLevel - The Enjoyable Layer

> **Axolotl's high-level features.** The set of language constructs that make the language *enjoyable* - fewer keystrokes for the same meaning, more direct expression of intent, less ceremony around the things programmers actually do all day.

The spine of Axolotl is Rust's semantics: ownership, traits, generics, zero-cost abstractions, no GC. The **surface** is Lua's: `if/then/end`, `function ... end`, tables, `:` methods, multiple returns. This document adds the third layer on top: **enjoyable ergonomics** - the small, opinionated, programmer-friendly features that turn "I can write this in 20 lines" into "I can write this in 5."

These are the features the user was asking for when they said:

> *"Match is okay but still who wants to write too much code. Why is there no other solution that achieves the same thing as match but less code?"*

That question is the seed of this entire document.

---

## Table of Contents

1. The high-level philosophy
2. Match replacements (the original ask)
3. `let-else` for early return
4. Pipe and reverse-application
5. Named arguments
6. Default parameter values
7. Spread operator
8. Destructuring assignment
9. If-let chains
10. List / map / set comprehensions
11. Method cascading
12. Multi-line and raw strings
13. Number-suffix literals
14. Documentation comments
15. Inline tests and examples
16. Auto-implement common traits
17. Quick constructors
18. Struct update syntax
19. Pattern matching on `Result` / `Option` with the `?` form
20. The "ergonomic surface" summary

---

## 1. The high-level philosophy

Three rules govern every feature in this document:

1. **Less code for the same meaning.** If two features express the same intent and one is shorter without losing clarity, the shorter one wins.
2. **Compile to the same Rust.** Every high-level feature lowers to the same idiomatic Rust as its verbose equivalent. Zero overhead. The compiler just saves you keystrokes.
3. **Optional everywhere.** Every high-level feature has a verbose equivalent. Beginners use the high-level forms; advanced users can drop to the verbose forms when clarity demands it.

The high-level features are not a separate "easy mode." They're sugar over the spine.

---

## 2. Match replacements (the original ask)

The user asked: *"Why is there no other solution that achieves the same thing as match but less code?"*

There are four. Each is right for a different context. **Pick the one that reads best in your code.**

### 2a. `case` syntax - terse match

```axol
case state
of Menu: show_menu()
of Playing: tick_game()
of Paused: show_pause()
of GameOver(score, reason): print("${score}: ${reason}")
end
```

`case` is `match` with three syntactic reductions:

1. No `=>` between the pattern and the body - use `:` instead.
2. No braces around the body.
3. `of` instead of `Pattern =>` for each arm.

Compiles to exactly the same Rust `match`. The compiler still enforces exhaustiveness. If you forget a variant, you get the same error as with `match`.

### 2b. Variant-handler methods - match becomes dispatch

This is the most powerful match replacement. Instead of writing a `match`, you attach a method to the type for each variant:

```axol
GameState = enum
    Menu
    Playing
    Paused
    GameOver(score: Int, reason: String)
end

-- attach handlers by method name
GameState:on_menu = fn() show_menu() end
GameState:on_playing = fn() tick_game() end
GameState:on_paused = fn() show_pause() end
GameState:on_game_over = fn(score, reason)
    print("score: ${score}, reason: ${reason}")
end

-- call site
state:handle()    -- the compiler generates a match that dispatches to the right handler
```

The compiler verifies that every variant has a handler. The user's call site is a single method call. There's no `match` block in sight.

**This is the form to reach for when "the same match is being written in many places" - the open-closed principle, applied to enums.**

The compiler generates a normal Rust `match` underneath. The handler methods are stored on the type's `impl` block.

### 2c. Dispatch tables - Lua-style, statically checked

For ad-hoc dispatch (you don't want to commit to a method on the type), use a dispatch table:

```axol
let handlers = {
    Menu: fn() show_menu() end
    Playing: fn() tick_game() end
    Paused: fn() show_pause() end
    GameOver: fn(score, reason) print("${score}: ${reason}") end
}

state:dispatch(handlers)
```

The compiler statically verifies that every variant of the enum has a handler in the table. If a variant is added to the enum, the compiler points at the dispatch table and says "you forgot a handler."

This is the form to reach for when the dispatch is **specific to one call site**.

### 2d. `match` itself - when you actually want it

Sometimes you really do want a `match`. Use it.

```axol
match request
    Get(path) => handle_get(path)
    Post(path, body) => handle_post(path, body)
    Delete(path) => handle_delete(path)
    _ => not_found()
end
```

`match` is the right form when:

- You need guards (`if condition`).
- You need to bind multiple variables and the body is complex.
- You want the match in the middle of an expression.

### 2e. Choosing the right form

| Form | Use when |
|---|---|
| `match` | You need guards, complex bindings, or you want the full power. |
| `case` | You want a terse match with no ceremony. |
| Variant-handler methods | The same dispatch is used in many places. |
| Dispatch table | The dispatch is specific to one call site. |

All four compile to the same Rust. Pick the one that reads best.

---

## 3. `let-else` for early return

The standard Rust pattern:

```rust
let Some(user) = find_user(id) else {
    return User::default();
};
```

In Axolotl, the same pattern becomes:

```axol
let Some(user) = find_user(id) else return default_user()
let Ok(data) = read_file(path) else return
let Some(name) = user?.name else "Anonymous"
```

Three reductions:

1. `else return` instead of `else { return ...; }` - the body of the else is a single expression.
2. `else "Anonymous"` - the else body can be a value, which becomes the binding's value when the pattern fails.
3. `else return` without a value means "early return from the enclosing function with no value" (i.e., `return None` in a function returning `T?`).

The compiler lowers this to the same Rust `let-else` construct.

### When to use

- Anywhere a function has multiple early-return conditions.
- Anywhere you want to bind a value or propagate upward.
- Anywhere the alternative is a deeply nested `if let Some(x) = ... { ... } else { ... }`.

---

## 4. Pipe and reverse-application

### 4a. Pipe `|>`

Pass the left-hand value as the **last** argument to the right-hand function:

```axol
let result = data
    |> parse
    |> validate
    |> transform
    |> save
```

Equivalent to:

```axol
let result = save(transform(validate(parse(data))))
```

The pipe is right-associative. Each step is on its own line, making data transformation pipelines read top-to-bottom.

### 4b. Reverse application `<|`

Pass the left-hand value as the **first** argument:

```axol
let result = save <| transform <| validate <| parse <| data
```

Same as the pipe but for languages / styles that prefer it.

### 4c. Both compile to ordinary function calls

```axol
data |> parse
-- becomes:
parse(data)

data |> parse |> validate
-- becomes:
validate(parse(data))
```

Zero overhead. The pipe is purely a syntactic convenience.

### 4d. Use cases

- Data transformation pipelines.
- Builder-style method chains (combined with method cascading - see Section 11).
- Configuration: `config |> load |> apply_defaults |> validate`.
- Anywhere a chain of function calls would otherwise be deeply nested.

---

## 5. Named arguments

Function calls can use named arguments, in any order, with any subset of optional parameters:

```axol
-- definition
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
```

The compiler verifies that:

- All required parameters are provided.
- No parameter is provided twice.
- All provided names are valid parameter names.
- All provided values are type-compatible.

Named arguments can be freely mixed with positional arguments, but **all positional arguments must come first**:

```axol
spawn_actor("Hero", health = 200)    -- OK
spawn_actor(health = 200, "Hero")    -- error: positional after named
```

### Why this matters

Compare:

```axol
-- positional (which is team? which is position?)
spawn_actor("Hero", 100, 5.0, Vec3(0, 0, 0), "red")

-- named (crystal clear)
spawn_actor(name = "Hero", health = 100, team = "red")
```

The named form is impossible to misread. The compiler errors on the positional form when the user names a parameter - *"do you want positional or named?"* - but accepts the all-positional form for backward compatibility.

### Method calls too

Methods support named arguments:

```axol
player:configure(
    health = 200,
    speed = 10.0,
    invulnerable = true
)
```

---

## 6. Default parameter values

Parameters can have default values:

```axol
fn create_window(
    title: String = "Untitled",
    width: Int = 800,
    height: Int = 600,
    resizable: Bool = true,
    fullscreen: Bool = false,
    vsync: Bool = true
)

-- call with any subset
create_window()                                     -- all defaults
create_window("My Game")                            -- title only
create_window("My Game", fullscreen = true)         -- override just one
```

The compiler synthesizes a builder internally; the generated Rust is the same as if you'd written the builder by hand.

### Default values can reference earlier parameters

```axol
fn rect(x: Int, y: Int, width: Int = 100, height: Int = width)
    -- 'height' defaults to the same value as 'width'
end
```

### Default values are evaluated at call time

Each call evaluates the defaults fresh. If the default depends on external state, that state is read at call time, not at definition time.

### What doesn't work as a default

- Parameters declared after the current one (forward references).
- Generic parameters.
- `self` / `this` (default values for `self` are always the receiver).

---

## 7. Spread operator

### 7a. Spread in function calls

```axol
let args = [1, 2, 3]
print(...args)              -- prints 1 2 3 (space-separated, with newline)

let opts = { host = "localhost", port = 8080 }
connect(...opts)            -- equivalent to connect(host = "localhost", port = 8080)
```

The spread works for both positional and named arguments. The compiler verifies that the spread is type-compatible with the receiving parameter list.

### 7b. Spread in table literals

```axol
let defaults = { color = "red", size = 10, weight = "normal" }
let custom = { ...defaults, size = 20 }     -- color and weight from defaults, size overridden

-- result: { color = "red", size = 20, weight = "normal" }
```

Later keys override earlier ones. The compiler infers the type from the union of all keys.

### 7c. Spread in array literals

```axol
let first = [1, 2, 3]
let second = [4, 5, 6]
let all = [...first, ...second, 7, 8, 9]     -- [1, 2, 3, 4, 5, 6, 7, 8, 9]
```

### 7d. Spread in struct literals

```axol
Player = struct
    name: String
    health: Int
    speed: Float
    position: Vec3
end

let base = Player { name = "Hero", health = 100, speed = 5.0, position = Vec3(0, 0, 0) }
let harder = base{ health = 200, speed = 10.0 }   -- shorthand update

-- equivalent to:
let harder = Player { ...base, health = 200, speed = 10.0 }
```

The shorthand form `base{ ... }` is sugar for "copy all fields, then override the listed ones."

---

## 8. Destructuring assignment

### 8a. Struct destructuring

```axol
let { name, health } = player
print(name, health)        -- two locals, types inferred

-- rename during destructuring
let { name: player_name, health: hp } = player
```

### 8b. Tuple destructuring

```axol
let (x, y, z) = position
let (head, ..tail) = list
let (first, second, ..rest) = items
```

### 8c. Nested destructuring

```axol
let { address: { city, country } } = user
print(city, country)
```

### 8d. With default values

```axol
let { name, role = "guest" } = user    -- role defaults to "guest" if missing or null
```

### 8e. In function parameters

```axol
fn describe({ name, health }: Player) -> String
    "Player ${name} with ${health} HP"
end

fn distance((x1, y1): (Float, Float), (x2, y2): (Float, Float)) -> Float
    sqrt((x2 - x1)^2 + (y2 - y1)^2)
end
```

### 8f. In `for` loops

```axol
for { name, score } in players do
    print("${name}: ${score}")
end
```

---

## 9. If-let chains

Combine multiple `if let`s without nesting:

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
```

The chain short-circuits on the first failure. If any `let` fails or any guard is false, the chain evaluates to false.

### With `else if` chains

```axol
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

### Compile to

The compiler desugars the chain into nested `if let` expressions with `&&`. No special runtime machinery; just a clearer syntax for the common pattern.

---

## 10. List / map / set comprehensions

Python-style comprehensions for building collections:

### 10a. List comprehensions

```axol
let alive = [e for e in enemies if e.health > 0]
let positions = [e.position for e in entities]
let names = [e.name for e in entities where e.name ~= ""]
let squares = [x * x for x = 0, 100]
```

The `[expr for x in iter if cond]` form is a single line of declarative code. The compiler lowers it to a Rust iterator chain.

### 10b. Map comprehensions

```axol
let by_name = { e.name: e for e in entities }
let scores = { name: score for name, score in players }
```

The `{ key: value for ... }` form builds a map.

### 10c. Set comprehensions

```axol
let unique_tags = { e.tag for e in entities }
```

The `{ expr for ... }` form (without `: value`) builds a set.

### 10d. With multiple iterators

```axol
-- nested
let pairs = [(a, b) for a in first for b in second]

-- cartesian product
let grid = [(x, y) for x in 0..10 for y in 0..10]

-- flat
let flat = [item for row in grid for item in row]
```

### 10e. Async comprehensions

```axol
let pages = [await page async for page in fetch_all_pages()]
```

Each `await` happens sequentially. Use `async for` in regular `for` loops for the same effect:

```axol
async for event in event_stream do
    process(event)
end
```

---

## 11. Method cascading

Call multiple methods on the same value without repeating the receiver:

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
```

The leading `:` on each method makes the receiver implicit - the previous expression in the chain is passed as `self`. Compiles to the same Rust as the verbose form.

### With `|>`

You can combine cascading with the pipe for explicit ordering:

```axol
let result = player
    :update(dt)
    :take_damage(20)
    |> check_death
    |> trigger_respawn
```

### Fluent builders

Cascading is purpose-built for the builder pattern. Define:

```axol
@derive(Default)
PlayerBuilder = struct
    name: String = ""
    health: Int = 100
    speed: Float = 5.0
    position: Vec3 = Vec3(0, 0, 0)
end

PlayerBuilder:set_name = fn(self, name: String) -> mutref Self
    self.name = name
    self
end

PlayerBuilder:set_health = fn(self, health: Int) -> mutref Self
    self.health = health
    self
end

PlayerBuilder:build = fn(self) -> Player
    Player { ...self }
end

-- usage
let p = PlayerBuilder:default()
    :set_name("Hero")
    :set_health(200)
    :set_position(Vec3(10, 0, 0))
    :build()
```

`@derive(Default)` generates a `default()` constructor that uses the field default values. `set_*` methods return `mutref Self` so they can be chained. `build()` returns the final type.

### The return-`self` shorthand

For methods that return `self` for chaining, Axolotl provides a `-> self` shorthand:

```axol
PlayerBuilder:set_name = fn(self, name: String) -> self   -- returns *self, not &mut Self
    self.name = name
end
```

The compiler emits `&mut Self` underneath; the return type is just a nicer notation.

---

## 12. Multi-line and raw strings

### 12a. Multi-line strings

Triple-quoted strings preserve newlines and indentation:

```axol
let query = """
    SELECT *
    FROM users
    WHERE active = true
    ORDER BY created_at DESC
    """

print(query)
```

The leading indentation is automatically stripped based on the minimum indent of non-empty lines. The user can write the string naturally without worrying about leading whitespace.

### 12b. Raw strings

Prefix with `r` to disable escape sequences:

```axol
let regex = r"^\d+\.\d+$"
let path = r"C:\Users\Mohi\Documents\file.txt"
let json = r#"{"name": "Mohi", "age": 20}"#
```

For strings that contain the delimiter, use `r#""..."#`. For more `#`s, use more `#`s:

```axol
let tricky = r##"contains a "quote" and a #hash"##
```

### 12c. Byte strings

Prefix with `b` for byte arrays:

```axol
let bytes = b"\xDE\xAD\xBE\xEF"
let bytes_str = b"hello"
```

### 12d. Format strings

Strings with `${expr}` interpolate any value that implements `Display`:

```axol
let msg = "Player ${player.name} has ${player.health} HP"
let table = """
    | ${name:>20} | ${health:>5} |
    """
```

Format specifiers follow Rust's `format!` syntax (`:>5` for right-align, `:<10` for left-align, `:.2` for two decimal places, etc.).

### 12e. Tagged strings

For DSLs, use tagged strings:

```axol
let html = h"<div>${name}</div>"
let sql = s"SELECT * FROM ${table_name}"
let md = md"# ${title}\n\n${body}"

-- the prefix is a function call; the function gets the string as input
```

The prefix is a regular function that takes a string. Any function that accepts `String` can be used as a tag.

---

## 13. Number-suffix literals

For human-readable sizes and durations:

```axol
let buffer_size = 4_KB              -- 4096
let max_memory = 1_GB               -- 1,073,741,824
let frame_budget = 16_ms            -- 16 milliseconds
let timeout = 30_sec                -- 30 seconds
let retry_after = 5_min             -- 5 minutes
let daily_limit = 1_day             -- 86,400 seconds
let discount = 0.5_pct              -- 0.005
let width = 100%                    -- 1.0
```

### Available suffixes

| Category | Suffixes | Type | Multiplier |
|---|---|---|---|
| Bytes | `_B`, `_KB`, `_MB`, `_GB`, `_TB`, `_PB` | `UInt64` | 1, 1024, 1024², 1024³, ... |
| Bits | `_b`, `_Kb`, `_Mb`, `_Gb` | `UInt64` | 1, 1024, 1024², 1024³ |
| Time | `_ns`, `_us`, `_ms`, `_sec`, `_min`, `_hour`, `_day`, `_week` | `Duration` | as named |
| Percent | `_pct` | `Float` | / 100 |
| Ratio | `%` | `Float` | / 100 |

### Custom suffixes

Users can define their own:

```axol
const unit KIB = 1024
const unit MIB = 1024 * KIB
const unit GIB = 1024 * MIB

let mem = 8_GIB                     -- 8,589,934,592
```

### They compile to ordinary constants

```axol
4_KB
-- becomes:
4096_u64
```

Zero runtime cost. Just more readable source.

---

## 14. Documentation comments

Triple-dash comments attach documentation to the next declaration:

```axol
--- Adds two numbers together.
---
--- @param a the first number
--- @param b the second number
--- @return their sum
fn add(a: Int, b: Int) -> Int
    a + b
end
```

`Ambystoma` (the documentation generator) processes these. Gills shows them in hovers. Editors render them in the IntelliSense popup.

### Tags

| Tag | Meaning |
|---|---|
| `@param name description` | Document a parameter |
| `@return description` / `@returns` | Document the return value |
| `@throws ErrorType` | Document an error case |
| `@example` | Start an example block |
| `@see Symbol` | Cross-reference |
| `@deprecated message` | Mark as deprecated |
| `@since version` | Mark as introduced in a version |
| `@todo message` | Mark as needing work |
| `@performance note` | Mark with a performance note |
| `@safety` | Mark a function as requiring unsafe (Rust convention) |
| `@complexity O(n)` | Document complexity |

### Example block

```axol
--- Greets the user.
---
--- @param name the user's name
--- @example
---   greet("Mohi")        -- "Hello, Mohi!"
fn greet(name: String)
    print("Hello, ${name}!")
end
```

### Markdown in docs

Doc comments support full Markdown:

```axol
--- # Process
---
--- This function processes the input in three stages:
---
--- 1. **Parse** - convert the input to an AST
--- 2. **Validate** - verify the AST against the schema
--- 3. **Transform** - apply the user's transformation
---
--- > Note: This function is hot. Avoid allocations.
fn process(input: String) -> Result<Output, Error>
    ...
end
```

### Module-level docs

```axol
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

---

## 15. Inline tests and examples

Test code lives next to the code it tests:

```axol
fn add(a: Int, b: Int) -> Int
    a + b
end

-- inline test
test add
    assert add(1, 2) == 3
    assert add(0, 0) == 0
    assert add(-5, 5) == 0
    assert add(100, 200) == 300
end

-- inline example (also runnable as a test)
example add
    print(add(1, 2))        -- 3
end
```

### Test syntax

```axol
test "descriptive test name"           -- optional name
    setup: -> State
        return State.new()
    end

    teardown: (state: State) -> ()
        state:cleanup()
    end

    test1: (state: State) -> ()
        assert add(state.x, state.y) == 3
    end

    test2: (state: State) -> ()
        assert_eq(add(1, 2), 3)
    end
end
```

Or simpler:

```axol
test "add returns correct sum"
    assert add(1, 2) == 3
    assert add(100, 200) == 300
end
```

`Salamander` runs these as part of `bucket test`. Gills surfaces inline test status in the editor (green checkmarks, red Xs, "Run test" code lenses).

### Test attributes

```axol
test add
    @should_panic
    assert panic_function() == 1

    @ignore("reason")
    assert slow_test() == 1

    @timeout(1000)            -- 1 second
    assert slow_test() == 1
end
```

### Doc tests

Any example in a doc comment can be marked as a runnable test:

```axol
--- Multiplies two numbers.
---
--- @example
---   assert multiply(3, 4) == 12
fn multiply(a: Int, b: Int) -> Int
    a * b
end
```

`Salamander` runs these. The output is included in the generated documentation.

---

## 16. Auto-implement common traits

The compiler automatically implements certain traits for types that look like they should have them:

### Auto-derived by default

| Trait | Auto-when |
|---|---|
| `Copy` | Type contains only `Copy` fields |
| `Clone` | Type contains only `Clone` fields |
| `Debug` | Type contains only `Debug` fields |
| `Default` | All fields have a `Default` (or are primitives) |
| `Eq`, `PartialEq` | All fields implement these |
| `Hash` | All fields implement `Hash` |
| `Send`, `Sync` | All fields implement these |

For example:

```axol
Point = struct
    x: Int
    y: Int
end

-- no @derive needed
let p1 = Point { x = 1, y = 2 }
let p2 = p1                              -- copied, not moved
let p3 = p1:clone()                      -- explicit clone also works
print(p1)                                -- works (Debug)
```

The compiler synthesizes the trait implementations. If the user wants to be explicit, they can use `@derive(...)` to opt in manually.

### Opt out

```axol
@no_auto_impl(Clone)     -- don't auto-implement Clone
SecretData = struct
    inner: Vec<U8>       -- Vec is Clone, but we don't want SecretData to be
end
```

---

## 17. Quick constructors

For any struct, the compiler generates a constructor named `new` from the struct's fields:

```axol
Player = struct
    name: String
    health: Int
    speed: Float
    position: Vec3
end

-- auto-generated:
-- fn new(name: String, health: Int, speed: Float, position: Vec3) -> Player
--     Player { name, health, speed, position }
-- end

-- usage
let p = Player:new("Hero", 100, 5.0, Vec3(0, 0, 0))
```

### Named-argument constructor

```axol
let p = Player:new(name = "Hero", health = 200)
```

Required fields without defaults are mandatory; fields with defaults are optional.

### Tuple / newtype constructors

```axol
UserId = struct
    inner: U64
end

-- auto-generated: UserId(inner: U64) -> UserId
let id = UserId(42)
```

### Struct literal vs `new`

Both work. `new` is clearer when the call is at the top of a function and named args are used. The struct literal is clearer when the call is in the middle of a larger expression.

---

## 18. Struct update syntax

Create a new struct from an old one, overriding specific fields:

```axol
let boss = player{
    health = player.health * 5
    name = "Boss " .. player.name
}
```

Equivalent to:

```axol
let boss = Player {
    ...player
    health = player.health * 5
    name = "Boss " .. player.name
}
```

The shorthand `player{ ... }` is "copy all fields, then override the listed ones." Saves typing the type name and the unmentioned fields.

### With method cascading

```axol
let buffed = player
    :take_damage(-50)              -- adds 50 HP
    :upgrade_speed(2.0)
    { health = 200 }               -- override health
```

(Yes, this combines method cascading and struct update. Useful for game logic.)

---

## 19. Pattern matching on `Result` / `Option` with the `?` form

The standard `?` operator is fine, but for one-liner handling, Axolotl supports a `?`-with-fallback form:

```axol
let name = user?.name ?? "Anonymous"     -- optional chaining + null coalescing

-- tuple destructuring from a Result-returning call
let Ok(data) = load() else handle_error
```

For the common case of "either a value or an error," there's a destructuring `let`:

```axol
let Ok(data) = load("file.txt")
-- if load returns Ok(data), this binds 'data'
-- if load returns Err(e), this returns from the function with the error

-- with else
let Ok(data) = load("file.txt") else return default_data()
```

This is the same as `let Ok(data) = load("file.txt")?` but with a more flexible `else` clause.

---

## 20. The "ergonomic surface" summary

Here is the complete high-level toolkit, at a glance:

| Feature | Replaces |
|---|---|
| `case` | `match` with simple arms |
| Variant-handler methods | `match` used in many places |
| Dispatch tables | `match` used in one place |
| `let-else` | nested `if let Some(...) else { return ... }` |
| `\|>` / `<\|` | nested function calls |
| Named arguments | positional arguments with magic numbers |
| Default values | builder pattern for optional params |
| Spread | array concatenation, manual field copying |
| Destructuring | `match` or manual `let x = obj.x; let y = obj.y;` |
| If-let chains | nested `if let` |
| Comprehensions | imperative `for` + `push` |
| Method cascading | repeated `let x = x.method()` |
| Multi-line / raw strings | `+` concatenation with `\n` |
| Number-suffix literals | `4096`, `16 * 1000 * 1000` |
| Documentation comments | separate doc files |
| Inline tests | separate test files |
| Auto-derived traits | `@derive(Clone, Debug, ...)` for common cases |
| Quick constructors | manual `fn new(...) -> Self` |
| Struct update syntax | `Player { ...old, field = new }` |
| Result destructuring | verbose `match result { Ok(x) => ..., Err(e) => ... }` |

Every feature compiles to ordinary idiomatic Rust. Every feature has a verbose equivalent. Every feature is opt-in.

That's the enjoyable layer.

---

## Why this matters

A language that compiles to Rust but **doesn't** make the high-level surface feel Lua-like and TypeScript-friendly will end up feeling like Rust with extra steps. That's not a new language - that's a theme.

The features in this document are what make Axolotl feel like a **different language**, not just a different syntax. They are the layer between "I can write this" and "I want to write this."

The user said it: *"Match is okay but still who wants to write too much code. Why is there no other solution that achieves the same thing as match but less code."*

There are now four solutions, plus a dozen other ergonomic features that follow the same philosophy. Pick the one that reads best. The compiler does the rest.
