# AxolCode - Axolotl language support for VS Code

AxolCode brings first-class [Axolotl](https://github.com/axolrs/axolotl) support to Visual Studio Code. It launches the **Gills** language server (`axol-analyzer`) to power editor intelligence, ships a TextMate grammar for `.axol` files, and wraps the `bucket` build tool with one-click commands.

## Features

- **Language server integration** - spawns `axol-analyzer` over stdio using `vscode-languageclient` and wires hover, completion, go-to-definition, find references, rename, document symbols, and live diagnostics to the editor once the server is connected.
- **Syntax highlighting** - a TextMate grammar (`source.axol`) covering keywords, `--` line comments and `--[[ ]]` block comments, strings with `${...}` interpolation, char literals, numbers, booleans, types, and operators.
- **Language configuration** - bracket matching, auto-closing pairs, block-comment toggling, and `end`/`else`/`until`-aware indentation for `.axol` files.
- **Bucket commands** - run, build, test, format, lint, fix, and document generation from the command palette, each in a dedicated terminal rooted at your workspace folder.
- **Interpreter and REPL** - run the interpreter with `bucket run --interpret`, or drop into the `axol-hot-runner` REPL.
- **Generated Rust preview** - shows the Rust that `bucket emit-rust` produces for the active `.axol` file in a read-only editor.
- **Status bar** - an `Axolotl: <status>` item that spins while a command runs, turns ready after the language server initializes, and turns red when a command fails.

## Requirements

- An Axolotl toolchain on your `PATH`: the `bucket` build tool, the `axol-analyzer` language server, and optionally the `axol-hot-runner` interpreter.
- If `axol-analyzer` is not on `PATH`, AxolCode falls back to `bucket gills --stdio` (configurable below).

## Install

From the repository:

```sh
git clone https://github.com/axolrs/axolotl
cd axolotl/editors/axolcode
pnpm install
pnpm run compile
```

Then open the `editors/axolcode` folder in VS Code and press F5 to launch the Extension Development Host, or package a `.vsix`:

```sh
pnpm run package
code --install-extension axolcode-0.1.0.vsix
```

## Commands

Open the Command Palette (`Ctrl+Shift+P` / `Cmd+Shift+P`) and run:

| Command | Action |
| --- | --- |
| `Axolotl: Run Project (bucket run)` | Runs `bucket run` in a terminal. |
| `Axolotl: Build Project (Release)` | Runs `bucket build --release`. |
| `Axolotl: Run Tests (bucket test)` | Runs `bucket test`. |
| `Axolotl: Format Project (bucket fmt)` | Runs `bucket fmt`. |
| `Axolotl: Lint Project (bucket lint)` | Runs `bucket lint`. |
| `Axolotl: Auto-Fix Project (bucket fix)` | Runs `bucket fix`. |
| `Axolotl: Build Documentation (bucket doc)` | Runs `bucket doc`. |
| `Axolotl: Run Interpreted (bucket run --interpret)` | Runs the interpreter instead of the compiler. |
| `Axolotl: Open REPL (axol-hot-runner repl)` | Starts the `axol-hot-runner` REPL. |
| `Axolotl: Show Generated Rust for Active File` | Runs `bucket emit-rust` on the active `.axol` file and opens the output read-only. |
| `Axolotl: Toggle Build Status Item` | Shows or hides the `Axolotl: <status>` status bar item. |

All commands that invoke `bucket` or `axol-hot-runner` require a workspace folder to be open; otherwise AxolCode shows an error message.

## Configuration

| Setting | Type | Default | Description |
| --- | --- | --- | --- |
| `axolcode.server.path` | `string` | `axol-analyzer` | Command that starts the Gills language server. May include quoted arguments; without arguments the server speaks LSP over stdio. |
| `axolcode.server.fallbackCommand` | `string` | `bucket gills --stdio` | Command used when `axolcode.server.path` cannot be found on `PATH`. |
| `axolcode.run.saveOnRun` | `boolean` | `false` | Save all editors before running the `Axolotl: Run Project` command. |

## Development

```sh
pnpm install          # restore dependencies
pnpm run compile      # webpack bundle into dist/extension.js
pnpm run watch        # rebuild on change
pnpm run lint         # eslint over src/
pnpm run compile-tests && pnpm test  # compile and run the mocha suite
```

The unit tests cover command argument construction, shell quoting, server resolution and fallback, the TextMate grammar JSON, and the extension manifest.

## License

MIT OR Apache-2.0, at your option - the same dual license as the rest of the Axolotl project.

## Owner

PascalElixir / axolrs (GitHub org)
