# Changelog

All notable changes to the AxolCode extension are documented in this file. The format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/) and the project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026-09-05

### Added

- Language server client that spawns `axol-analyzer` (Gills) over stdio and enables hover, completion, go-to-definition, references, rename, document symbols, and diagnostics for `.axol` files, with a `bucket gills --stdio` fallback when the server is not on `PATH`.
- `axol` language registration for `.axol` files with a TextMate grammar covering keywords, line (`--`) and block (`--[[ ]]`) comments, strings with `${...}` interpolation, char literals, numbers, and booleans.
- Bucket commands: `axolcode.run`, `axolcode.build`, `axolcode.test`, `axolcode.fmt`, `axolcode.lint`, `axolcode.fix`, and `axolcode.doc`, each run in a terminal rooted at the workspace folder.
- Interpreter commands: `axolcode.interpretedRun` (`bucket run --interpret`) and `axolcode.interpretedREPL` (`axol-hot-runner repl`).
- `axolcode.showGeneratedRust` which runs `bucket emit-rust` on the active document and opens the output in a read-only Rust document.
- `axolcode.buildStatus` which toggles the `Axolotl: <status>` status bar item; the item spins while commands run and reports readiness or failure.
- Configuration settings `axolcode.server.path`, `axolcode.server.fallbackCommand`, and `axolcode.run.saveOnRun`.
- Mocha unit tests for command construction, shell quoting, server resolution, the grammar JSON, and the extension manifest.

### Removed

- The placeholder `axolcode.helloWorld` command.
