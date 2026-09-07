# Gills - The LSP

Gills (the `axol-analyzer`) is Axolotl's language server. It is a fork of
the rust-analyzer architecture, extended to understand both `.axol` and
`.rs` files in the same project. It provides cross-language navigation,
completion, hover, diagnostics, and rename.

The name fits: axolotls breathe through external gills, and **Gills is
how the editor breathes the project** - autocomplete, hover,
go-to-definition, references, rename, refactor, inline diagnostics, code
actions, type info, and intelligent suggestions, all unified across both
languages.

## What Gills Does

Gills implements the Language Server Protocol (LSP), so any editor that
supports LSP can use it:

- **Completion** - context-aware suggestions, including cross-language
  (e.g., suggesting Rust crate methods when you type `use "serde"`).
- **Hover** - show the inferred type and ownership of any expression.
- **Go to definition** - works across `.axol` and `.rs` files.
- **References** - find all uses of a symbol, in both languages.
- **Rename** - refactor a symbol across the entire project.
- **Code actions** - apply Regrow's suggested fixes inline.
- **Inlay hints** - show inferred types, ownership, and lifetimes inline.
- **Diagnostics** - show errors and warnings at the `.axol` level (not
  the generated-Rust level).
- **Semantic tokens** - syntax highlighting based on semantic info, not
  just syntactic.
- **Document symbols** - outline view of the current file.
- **Workspace symbols** - search for symbols across the project.

## Cross-Language Navigation

In a mixed `.axol` + `.rs` project, Gills maintains a unified symbol
graph. Go-to-definition from `.axol` jumps to `.rs` if the symbol is
defined in Rust. Hover on a Rust function call inside `.axol` shows the
Rust function's signature and doc comment.

This works because Gills builds two parallel symbol graphs (one for
`.axol`, one for `.rs`) and links them at the `use` boundaries.

## Span Mapping

When the underlying Rust borrow checker rejects code, the error is
reported in terms of the generated Rust source. Gills maps this span back
to the `.axol` source, so the user sees the error in their `.axol` file
at the correct line and column.

The span map is built during compilation. Each `.axol` AST node carries
the span of the generated Rust code it produced. When the Rust compiler
reports an error at a generated-Rust span, Gills looks up the
corresponding `.axol` span.

## How to Use Gills

### From the CLI

```bash
bucket gills --stdio
```

This runs Gills over stdio. Editors connect to it via the standard LSP
stdio transport.

### From Zed

The Zed extension (`editors/zed-axolotl/`) spawns Gills as a subprocess
and forwards LSP. See the [Editor Setup](/docs/editor-setup) guide.

### From Neovim

The Neovim plugin (`editors/nvim-axolotl/`) configures `vim.lsp` to use
Gills. See the [Editor Setup](/docs/editor-setup) guide.

### From VS Code

The VS Code extension (`editors/axolcode/`) spawns Gills as a child
process and forwards LSP via the `vscode-languageclient` library. See the
[Editor Setup](/docs/editor-setup) guide.

## Configuration

Gills is configured via the `lsp` field in `Bucket.jsonc`:

```jsonc
{
    "lsp": {
        "trace": "messages",
        "inlay_hints": {
            "enabled": true,
            "hide_guts": false,
            "show_ownership": true,
            "show_lifetimes": false
        },
        "completion": {
            "enabled": true,
            "auto_import": true,
            "snippet_support": true
        },
        "diagnostics": {
            "enabled": true,
            "show_warnings": true
        }
    }
}
```

## Scope Resolution

Gills builds a scope tree for each `.axol` file. The scope tree tracks:

- Variable bindings (let, var, function parameters)
- Closures (captured bindings)
- Method self-inference (borrow vs. mut vs. move)
- Shadowing (a later binding shadows an earlier one with the same name)

Definition lookup walks the scope tree from the cursor outward. References
lookup walks the entire project.

## Inferred Type / Ownership Display

Hover on any expression to see its inferred type and ownership:

```
let p = Point { x = 1, y = 2 }
         ^^^^^^^^^^^^^^^^^^^^^^^
         Point (owned, moves on use)
```

Hover on a function parameter to see its inferred ownership:

```
function distance(a: Point, b: Point) -> Float
                 ^^^^^
                 borrow Point (read-only)
```

## Code Actions

Gills provides code actions (the "lightbulb" in VS Code):

- **Apply Regrow fix** - apply the auto-fixer's suggestion.
- **Extract function** - extract the selected code into a new function.
- **Inline variable** - replace a variable with its value.
- **Convert to comprehension** - convert a for-loop with a push into a
  list comprehension.
- **Add missing match arms** - complete a non-exhaustive match.
- **Generate impl** - generate a trait implementation skeleton.

## What's Next

- Read the [Toolchain](/docs/toolchain) guide for the `bucket` command
  reference.
- Read the [Editor Setup](/docs/editor-setup) guide for your specific
  editor.
- Read the [Architecture](/docs/architecture) guide for the system design.
