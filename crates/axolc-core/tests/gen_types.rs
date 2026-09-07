// Owner: PascalElixir / axolrs (GitHub org)
// File: Auto-generated tests for primitive type token kinds.

#![allow(non_snake_case)]

use axolc_core::tokenize;

#[test]
fn type_Int() {
    let (tokens, _) = tokenize("Int", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::token::TokenKind::KwInt);
}

#[test]
fn type_UInt() {
    let (tokens, _) = tokenize("UInt", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::token::TokenKind::KwUInt);
}

#[test]
fn type_Float() {
    let (tokens, _) = tokenize("Float", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::token::TokenKind::KwFloat);
}

#[test]
fn type_Double() {
    let (tokens, _) = tokenize("Double", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::token::TokenKind::KwDouble);
}

#[test]
fn type_Bool() {
    let (tokens, _) = tokenize("Bool", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::token::TokenKind::KwBool);
}

#[test]
fn type_String() {
    let (tokens, _) = tokenize("String", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::token::TokenKind::KwString);
}

#[test]
fn type_Char() {
    let (tokens, _) = tokenize("Char", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::token::TokenKind::KwChar);
}

#[test]
fn type_Byte() {
    let (tokens, _) = tokenize("Byte", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::token::TokenKind::KwByte);
}

#[test]
fn type_I8() {
    let (tokens, _) = tokenize("I8", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::token::TokenKind::KwI8);
}

#[test]
fn type_I16() {
    let (tokens, _) = tokenize("I16", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::token::TokenKind::KwI16);
}

#[test]
fn type_I32() {
    let (tokens, _) = tokenize("I32", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::token::TokenKind::KwI32);
}

#[test]
fn type_I64() {
    let (tokens, _) = tokenize("I64", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::token::TokenKind::KwI64);
}

#[test]
fn type_I128() {
    let (tokens, _) = tokenize("I128", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::token::TokenKind::KwI128);
}

#[test]
fn type_U8() {
    let (tokens, _) = tokenize("U8", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::token::TokenKind::KwU8);
}

#[test]
fn type_U16() {
    let (tokens, _) = tokenize("U16", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::token::TokenKind::KwU16);
}

#[test]
fn type_U32() {
    let (tokens, _) = tokenize("U32", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::token::TokenKind::KwU32);
}

#[test]
fn type_U64() {
    let (tokens, _) = tokenize("U64", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::token::TokenKind::KwU64);
}

#[test]
fn type_U128() {
    let (tokens, _) = tokenize("U128", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::token::TokenKind::KwU128);
}

#[test]
fn type_F32() {
    let (tokens, _) = tokenize("F32", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::token::TokenKind::KwF32);
}

#[test]
fn type_F64() {
    let (tokens, _) = tokenize("F64", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::token::TokenKind::KwF64);
}

