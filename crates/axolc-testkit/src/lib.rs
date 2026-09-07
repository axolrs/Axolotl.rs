// Owner: PascalElixir / axolrs (GitHub org)
// File: axolc-testkit - shared test helpers for the Axolotl workspace.

pub use axolc_core::*;

/// Helper for parsing an Axolotl source snippet in a test.
pub fn parse(src: &str) -> (axolc_core::ast::Module, axolc_core::diag::Diagnostics) {
    axolc_core::parse(src, 0)
}

/// Helper for type-checking a parsed module (currently a passthrough).
pub fn check(_module: &axolc_core::ast::Module) -> Result<(), Vec<String>> {
    Ok(())
}

/// Helper for lowering to Rust and returning the generated source.
pub fn emit_rust(src: &str) -> String {
    let (rust, _diags) = axolc_core::compile_to_rust(src, 0);
    rust
}

/// Helper for running the interpreter on a snippet.
pub fn interpret(src: &str) -> (String, axolc_core::diag::Diagnostics) {
    axolc_core::interpret(src, 0)
}

/// Helper that returns true if the source parses without errors.
pub fn parses_clean(src: &str) -> bool {
    let (_, diags) = parse(src);
    !diags.has_errors()
}

/// Helper that returns true if the source compiles to Rust without errors.
pub fn compiles_clean(src: &str) -> bool {
    let (_, diags) = axolc_core::compile_to_rust(src, 0);
    !diags.has_errors()
}

/// Helper that returns the interpreter output for a source string.
pub fn interpret_output(src: &str) -> String {
    let (out, _diags) = interpret(src);
    out
}

/// Helper that returns the diagnostic count for a source string.
pub fn diagnostic_count(src: &str) -> usize {
    let (_, diags) = parse(src);
    diags.items.len()
}

/// Helper that returns the token count for a source string.
pub fn token_count(src: &str) -> usize {
    let (tokens, _) = axolc_core::tokenize(src, 0);
    tokens.len()
}

/// Helper that returns true if the source contains a specific substring in the generated Rust.
pub fn rust_contains(src: &str, needle: &str) -> bool {
    let rust = emit_rust(src);
    rust.contains(needle)
}
