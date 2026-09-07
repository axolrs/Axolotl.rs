// Owner: PascalElixir / axolrs (GitHub org)
// File: axolc-core library - the Axolotl compiler front-end: lexer, parser, HIR, ownership inference, type system, FFI, codegen, interpreter.

pub mod ast;
pub mod codegen;
pub mod diag;
pub mod ffi;
pub mod hir;
pub mod interp;
pub mod lexer;
pub mod manifest;
pub mod ownership;
pub mod parser;
pub mod span;
pub mod types;

/// Compile an Axolotl source string to idiomatic Rust.
pub fn compile_to_rust(src: &str, file_id: u32) -> (String, diag::Diagnostics) {
    codegen::compile_to_rust(src, file_id)
}

/// Interpret an Axolotl source string and return the output.
pub fn interpret(src: &str, file_id: u32) -> (String, diag::Diagnostics) {
    interp::interpret(src, file_id)
}

/// Parse an Axolotl source string into a Module.
pub fn parse(src: &str, file_id: u32) -> (ast::Module, diag::Diagnostics) {
    parser::parse(src, file_id)
}

/// Parse and lower an Axolotl source string into HIR (with ownership analysis attached).
pub fn lower(src: &str, file_id: u32) -> (hir::Module, diag::Diagnostics) {
    let (module, diags) = parser::parse(src, file_id);
    let lowered = hir::lower(&module);
    let mut all = diags;
    all.extend(lowered.diags.clone());
    (lowered, all)
}

/// Infer ownership for a source string, returning the mode table and diagnostics.
pub fn infer_ownership(src: &str, file_id: u32) -> (ownership::OwnershipTable, diag::Diagnostics) {
    let (module, _) = parser::parse(src, file_id);
    ownership::infer(&module)
}

/// Tokenize an Axolotl source string.
pub fn tokenize(src: &str, file_id: u32) -> (Vec<lexer::token::Token>, diag::Diagnostics) {
    lexer::tokenize(src, file_id)
}
