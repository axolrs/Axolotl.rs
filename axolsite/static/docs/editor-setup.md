# Editor Setup

Axolotl ships three first-class editor integrations. All three talk to
Gills (the LSP) over the standard LSP protocol, and all three use the same
tree-sitter grammar for syntax highlighting.

| Editor | Plugin | Language |
|--------|--------|----------|
| Zed | `editors/zed-axolotl/` | Rust (WASM cdylib) |
| Neovim | `editors/nvim-axolotl/` | Lua |
| VS Code | `editors/axolcode/` | TypeScript + Webpack |

## Zed

The Zed extension is a Rust crate that compiles to a WASM cdylib. It is
not part of the main workspace because the WASM target is incompatible
with the rest of the workspace.

### Install

```bash
cd editors/zed-axolotl
cargo build --target wasm32-wasip2 --release
```

The compiled extension lands in `target/wasm32-wasip2/release/`. Copy it
to your Zed extensions directory.

### Configure

Zed reads the extension manifest from `extension.toml`. The extension
spawns `bucket gills --stdio` as a subprocess and forwards LSP.

### Features

- Syntax highlighting via the tree-sitter grammar.
- LSP integration (completion, hover, go-to-def, references, rename,
  diagnostics, code actions).
- Axolotl-specific commands (show inferred ownership, show generated
  Rust, apply Regrow fix).
- Pond cache status, current test status, build status.

## Neovim

The Neovim plugin is a Lua plugin. It configures `vim.lsp` to use Gills
and registers the tree-sitter parser.

### Install

Use your favorite Neovim plugin manager. With `lazy.nvim`:

```lua
{
    "PascalElixir/nvim-axolotl",
    config = function()
        require("axolotl").setup({
            cmd = { "axol-analyzer" },
            grammar_path = "/path/to/tree-sitter-axol/parser.so",
        })
    end,
}
```

### Commands

The plugin exposes the `:Axolotl` command namespace:

- `:Axolotl run` - invoke `bucket run`.
- `:Axolotl build` - invoke `bucket build --release`.
- `:Axolotl test` - invoke `bucket test`.
- `:Axolotl fmt` - invoke `bucket fmt`.
- `:Axolotl lint` - invoke `bucket lint`.
- `:Axolotl fix` - invoke `bucket fix`.
- `:Axolotl gills` - show the LSP status.
- `:Axolotl interpret` - drop into the interpreter REPL.
- `:Axolotl gill` - show the inferred type / ownership at the cursor
  (calls Gills' `textDocument/hover`).

### Defaults

The plugin ships with sensible defaults:

- `inlay_hints = { enabled = true, hide_guts = false }`
- `tree_sitter_highlight = { enabled = true }`
- `format_on_save = { enabled = true, tool = "Shed" }`
- `on_save = { run = false }` (opt-in to run-on-save)

A status line integration shows the build status (Gills computes it from
the project manifest).

## VS Code (AxolCode)

The VS Code extension is a TypeScript + Webpack extension. The package
name is `axolcode`, the display name is `AxolCode`, the engine is
`vscode ^1.136.0`.

### Install

```bash
cd editors/axolcode
pnpm install
pnpm run package
```

The packaged `.vsix` lands in `dist/`. Install it via the VS Code
extension installer.

### Commands

The extension exposes the `axolcode.*` command namespace:

- `axolcode.run` - invoke `bucket run`.
- `axolcode.build` - invoke `bucket build --release`.
- `axolcode.test` - invoke `bucket test`.
- `axolcode.fmt` - invoke `bucket fmt`.
- `axolcode.lint` - invoke `bucket lint`.
- `axolcode.fix` - invoke `bucket fix`.
- `axolcode.doc` - invoke `bucket doc`.
- `axolcode.interpretedRun` - invoke `bucket run` with
  `interpreted_dev_mode: true`.
- `axolcode.interpretedREPL` - drop into the interpreter REPL.
- `axolcode.buildStatus` - show the current build status.

### Features

The extension:

- Spawns `axol-analyzer` as a child process and forwards LSP via the
  standard `vscode-languageclient` library.
- Wires the tree-sitter grammar for syntax highlighting.
- Wires Gills' diagnostics to the editor's problems panel.
- Wires Gills' hover, completion, go-to-def, rename, references,
  signature help, inlay hints, code actions, semantic tokens, document
  symbols to the editor.
- Adds a status bar item showing the Bucket build state (synced via LSP
  `window/workDoneProgress/create`).
- Adds a sidebar view for the test results (synced via Gills' test
  discovery).

## Tree-sitter Grammar

All three editors use the same tree-sitter grammar for syntax
highlighting, code folding, and structure analysis. The grammar lives at
`tooling/tree-sitter-axol/`.

The grammar supports 8 language bindings: c, go, java, node, python,
rust, swift, zig. Each binding lets the grammar be used in that
language's editor ecosystem.

## What's Next

- Read the [Gills (LSP)](/docs/gills) guide for the LSP design.
- Read the [Toolchain](/docs/toolchain) guide for the `bucket` command
  reference.
- Read the [Getting Started](/docs/getting-started) guide to install
  Axolotl.
