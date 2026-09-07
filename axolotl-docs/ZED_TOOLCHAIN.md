# Zed Toolchain Setup for Axolotl

> **The Zed extension for Axolotl is `editors/zed-axolotl/`.** It is a Rust crate that compiles to WASM and integrates with Zed's extension API. This document is the toolchain setup guide - the missing piece the user identified in the project checklist.

---

## 1. Prerequisites

Before building the extension:

```bash
# 1. Install the latest stable Rust toolchain
rustup update stable
rustc --version
cargo --version

# 2. Install the WASM target (required for the Zed extension)
rustup target add wasm32-wasip2

# 3. Install the latest Zed release
# (see https://zed.dev/docs for installation instructions)
```

The Zed extension is `crate-type = ["cdylib"]` and is built for `wasm32-wasip2`. It is **not** part of the Cargo workspace because the WASM target is incompatible with the rest of the workspace's `cdylib` + `rlib` build configuration.

## 2. The `editors/zed-axolotl/` layout

```
editors/zed-axolotl/
├── Cargo.toml             # crate-type = ["cdylib"], depends on zed_extension_api
├── extension.toml         # the Zed extension manifest
├── README.md
├── src/
│   └── lib.rs             # the extension entrypoint
└── tests/
    └── lsp_integration.rs # tests for the LSP integration
```

### 2.1 `Cargo.toml`

```toml
[package]
name = "zed-axolotl"
version.workspace = true
edition.workspace = true
license.workspace = true

[dependencies]
zed_extension_api = "0.7.0"

[lib]
crate-type = ["cdylib"]
```

### 2.2 `extension.toml`

```toml
id = "axolotl"
name = "Axolotl"
description = "Axolotl language support for Zed."
version = "0.1.0"
schema_version = 1
authors = ["PascalElixir <axolrs>"]
repository = "https://github.com/axolrs/Axolotl.rs"
```

### 2.3 `src/lib.rs`

```rust
// Owner: PascalElixir / axolrs (GitHub org)
// File: Axolotl Zed extension entrypoint - spawns Gills and forwards LSP requests.

use zed_extension_api::{self as zed, LanguageServerId, Result};

struct AxolotlExtension;

impl zed::Extension for AxolotlExtension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        match language_server_id.as_ref() {
            "axol-analyzer" => Ok(zed::Command {
                command: zed::Path::local("bucket"),
                args: vec!["gills".to_string(), "--stdio".to_string()],
                env: Default::default(),
            }),
            _ => Err(format!("Unknown language server: {}", language_server_id.as_ref()).into()),
        }
    }
}

zed::register_extension!(AxolotlExtension);
```

## 3. Building the extension

```bash
cd editors/zed-axolotl
cargo build --target wasm32-wasip2 --release
```

The output is `target/wasm32-wasip2/release/zed_axolotl.wasm`. That is the file Zed loads.

## 4. Installing the extension (development)

For local development, the extension can be loaded from the build output:

```bash
zed --install-extension ./editors/zed-axolotl/target/wasm32-wasip2/release/zed_axolotl.wasm
```

Or, for active development, symlink the extension into Zed's extension directory:

```bash
# Linux / macOS
ln -s "$(pwd)/editors/zed-axolotl" ~/.local/share/zed/extensions/axolotl

# Or via the Zed UI: Cmd+Shift+P (Linux/Windows) or Cmd+Shift+P (macOS) > "zed: install dev extension"
```

After installation, open a `.axol` file in Zed. Gills should launch, completions should appear, diagnostics should be visible.

## 5. The LSP integration

The extension talks to `axol-analyzer` (Gills) over the standard LSP protocol. The extension is a thin wrapper:

- Spawns `bucket gills --stdio` as a subprocess.
- Forwards LSP `initialize`, `initialized`, `textDocument/*`, `workspace/*` requests.
- Renders diagnostics, completions, hovers, code actions, inlay hints.
- Provides Axolotl-specific commands via the LSP `Window/_axolotl/customRequest` channel.

The actual LSP server logic lives in `crates/axol-analyzer/`, not in the extension. The extension is just a transport adapter.

## 6. The tree-sitter grammar

`tooling/tree-sitter-axol/` is the tree-sitter grammar for Axolotl. Zed uses it for syntax highlighting, code folding, and indent computation. To regenerate the parser:

```bash
cd tooling/tree-sitter-axol
# Follow the tree-sitter docs: https://tree-sitter.github.io/tree-sitter/
tree-sitter generate
tree-sitter test
```

The generated `parser.c` is committed. The Rust binding is in `bindings/rust/`.

## 7. The Neovim plugin

`editors/nvim-axolotl/` is a Lua plugin. It uses `vim.lsp.start` to spawn Gills, registers Axolotl filetypes via `ftdetect/`, provides syntax highlighting via a tree-sitter query, and exposes a few `:Axolotl` commands for Bucket integration.

The plugin is not a Zed extension; it lives in its own crate/directory because Neovim's plugin API is Lua, not Rust.

## 8. The CI workflow for the extension

The `.github/workflows/ci.yaml` workflow builds the extension on every push:

```yaml
- name: Build Zed extension
  run: |
    rustup target add wasm32-wasip2
    cd editors/zed-axolotl
    cargo build --target wasm32-wasip2 --release
    cargo test
```

The `.github/workflows/release.yaml` workflow packages the extension for the Zed extension registry.

## 9. Common issues and fixes

| Issue | Fix |
|---|---|
| `error: linker 'wasm-ld' not found` | `rustup target add wasm32-wasip2` (re-run; the linker is part of the target) |
| `bucket: command not found` from Zed | Make sure `bucket` is on `PATH`; install it via `curl -sSf https://axolotl.rs/install \| sh` |
| Gills doesn't launch | Check `bucket gills status`. The Gills binary should be on `PATH`. |
| No completions in `.axol` files | Make sure the file is recognized as Axolotl (Zed may default to plaintext; check the language selector in the status bar) |
| Tree-sitter parse errors | Run `tree-sitter test` in `tooling/tree-sitter-axol/`. The grammar may have been updated; rebuild the parser. |

## 10. The toolchain versions

When this document is read (2026), the latest stable versions are:

- **Rust toolchain:** whatever `rustup update stable` returns today.
- **WASM target:** `wasm32-wasip2`. This is the Zed-compatible target. Older `wasm32-unknown-unknown` does not work with Zed.
- **Zed:** 0.140+ (whatever the latest stable release is). The `zed_extension_api` crate version is `0.7.0` or later.
- **Tree-sitter:** 0.22+.

The CI installs the current stable. The lockfile pins the exact versions.

---

The toolchain setup is now complete. The Zed extension builds, the Neovim plugin installs, and the CI packages both for distribution. With this in place, contributors can edit `.axol` files in either editor and get full IDE support.
