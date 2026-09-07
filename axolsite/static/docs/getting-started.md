# Getting Started with Axolotl

Welcome to Axolotl - a high-level, all-purpose, memory-safe, zero-GC programming
language that compiles to idiomatic Rust and ships as a native binary. Axolotl
borrows Lua's friendly surface syntax, inherits Rust's ownership and type
system, and uses the entire Cargo ecosystem as its standard library.

This guide walks you through installing the toolchain, creating your first
Axolotl project, and running it in both interpreted (sub-200ms) and compiled
(native binary) modes.

## Install Bucket

The `bucket` tool is the project orchestrator. It sits above Cargo and manages
Axolotl projects the same way Cargo manages Rust projects.

```bash
curl -sSf https://axolotl.rs/install | sh
```

This installs four binaries on your PATH:

- `bucket` - project / build / dependency orchestrator
- `axolc` - the Axolotl → Rust compiler
- `axol-analyzer` - Gills, the LSP for mixed `.axol` and `.rs` files
- `axol-hot-runner` - the interpreter + JIT used by `bucket run`

Verify the install:

```bash
bucket --version
axolc --version
axol-analyzer --version
```

All four should report `0.1.0-moss` (or newer).

## Create a Project

```bash
bucket new hello
cd hello
```

The new project has this layout:

```
hello/
├── Bucket.jsonc
├── src/
│   └── main.axol
└── pond/                # created on first build
```

`Bucket.jsonc` is the project manifest. It looks like:

```jsonc
{
    "name": "hello",
    "version": "0.1.0",
    "language": { "edition": "2026" },
    "interpreted_dev_mode": true,
    "scripts": {
        "dev":     "bucket run --watch",
        "release": "bucket build --release",
        "test":    "bucket test"
    },
    "dependencies": {}
}
```

The default `src/main.axol`:

```lua
function main()
    print("Hello, Axolotl!")
end
```

## Run It (Dev Mode)

```bash
bucket run
```

This uses `axol-hot-runner` - the interpreter - for sub-200ms dev iteration.
You should see:

```
Hello, Axolotl!
```

Edit `src/main.axol`, save, and `bucket run --watch` re-runs in well under a
second. This is the inner loop you will live in.

## Build a Native Binary

```bash
bucket build --release
```

This compiles `.axol` to idiomatic Rust, then hands the Rust to Cargo +
`rustc` + LLVM. The result is a native binary in `pond/release/`:

```bash
./pond/release/hello
```

The binary has **no Axolotl runtime** linked in. There is no interpreter,
no JIT, no GC, no scheduler, no hidden heap. The binary is just LLVM-optimized
native code, exactly as if you had written the Rust by hand.

## Add a Cargo Dependency

Axolotl uses Cargo crates directly. To add `serde` and `tokio`:

```bash
bucket add serde tokio
```

This updates `Bucket.jsonc`:

```jsonc
"dependencies": {
    "serde": "^1",
    "tokio": "^1"
}
```

And regenerates the underlying `Cargo.toml`. Now in your `.axol` file:

```lua
use "serde"

struct Point { x: Int, y: Int }

function main()
    let p = Point { x: 1, y: 2 }
    let json = serde.serialize(p)
    print(json)
end
```

## Run the Tests

```bash
bucket test
```

This runs every `#[test]` function in your project plus all of the upstream
Cargo tests for your dependencies.

## What's Next

- Read the [Language Reference](/docs/language-reference) for the full syntax.
- Read the [Ownership](/docs/ownership) guide to understand how Axolotl's
  ownership system maps to Rust's.
- Read the [Toolchain](/docs/toolchain) guide for the full `bucket` command
  reference.
- Read the [Interpreter](/docs/interpreter) guide to learn how the
  dev-mode hot runner works.

Welcome to Axolotl.
