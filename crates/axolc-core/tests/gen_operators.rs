// Owner: PascalElixir / axolrs (GitHub org)
// File: Auto-generated tests for operator token kinds.

#![allow(non_snake_case)]

use axolc_core::tokenize;

#[test]
fn op_LParen() {
    let (tokens, _) = tokenize("(", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::token::TokenKind::LParen);
}

#[test]
fn op_RParen() {
    let (tokens, _) = tokenize(")", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::token::TokenKind::RParen);
}

#[test]
fn op_LBrace() {
    let (tokens, _) = tokenize("{", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::token::TokenKind::LBrace);
}

#[test]
fn op_RBrace() {
    let (tokens, _) = tokenize("}", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::token::TokenKind::RBrace);
}

#[test]
fn op_LBracket() {
    let (tokens, _) = tokenize("[", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::token::TokenKind::LBracket);
}

#[test]
fn op_RBracket() {
    let (tokens, _) = tokenize("]", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::token::TokenKind::RBracket);
}

#[test]
fn op_Comma() {
    let (tokens, _) = tokenize(",", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::token::TokenKind::Comma);
}

#[test]
fn op_Semicolon() {
    let (tokens, _) = tokenize(";", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::token::TokenKind::Semicolon);
}

#[test]
fn op_Colon() {
    let (tokens, _) = tokenize(":", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::token::TokenKind::Colon);
}

#[test]
fn op_DoubleColon() {
    let (tokens, _) = tokenize("::", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::token::TokenKind::DoubleColon);
}

#[test]
fn op_Dot() {
    let (tokens, _) = tokenize(".", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::token::TokenKind::Dot);
}

#[test]
fn op_DotDot() {
    let (tokens, _) = tokenize("..", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::token::TokenKind::DotDot);
}

#[test]
fn op_DotDotDot() {
    let (tokens, _) = tokenize("...", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::token::TokenKind::DotDotDot);
}

#[test]
fn op_Arrow() {
    let (tokens, _) = tokenize("->", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::token::TokenKind::Arrow);
}

#[test]
fn op_FatArrow() {
    let (tokens, _) = tokenize("=>", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::token::TokenKind::FatArrow);
}

#[test]
fn op_Question() {
    let (tokens, _) = tokenize("?", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::token::TokenKind::Question);
}

#[test]
fn op_DoubleQuestion() {
    let (tokens, _) = tokenize("??", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::token::TokenKind::DoubleQuestion);
}

#[test]
fn op_QuestionDot() {
    let (tokens, _) = tokenize("?.", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::token::TokenKind::QuestionDot);
}

#[test]
fn op_Bang() {
    let (tokens, _) = tokenize("!", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::token::TokenKind::Bang);
}

#[test]
fn op_DoubleBang() {
    let (tokens, _) = tokenize("!!", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::token::TokenKind::DoubleBang);
}

#[test]
fn op_At() {
    let (tokens, _) = tokenize("@", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::token::TokenKind::At);
}

#[test]
fn op_Hash() {
    let (tokens, _) = tokenize("#", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::token::TokenKind::Hash);
}

#[test]
fn op_Dollar() {
    let (tokens, _) = tokenize("$", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::token::TokenKind::Dollar);
}

#[test]
fn op_Ampersand() {
    let (tokens, _) = tokenize("&", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::token::TokenKind::Ampersand);
}

#[test]
fn op_Plus() {
    let (tokens, _) = tokenize("+", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::token::TokenKind::Plus);
}

#[test]
fn op_Minus() {
    let (tokens, _) = tokenize("-", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::token::TokenKind::Minus);
}

#[test]
fn op_Star() {
    let (tokens, _) = tokenize("*", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::token::TokenKind::Star);
}

#[test]
fn op_Slash() {
    let (tokens, _) = tokenize("/", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::token::TokenKind::Slash);
}

#[test]
fn op_Percent() {
    let (tokens, _) = tokenize("%", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::token::TokenKind::Percent);
}

#[test]
fn op_Caret() {
    let (tokens, _) = tokenize("^", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::token::TokenKind::Caret);
}

#[test]
fn op_Tilde() {
    let (tokens, _) = tokenize("~", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::token::TokenKind::Tilde);
}

#[test]
fn op_ShiftLeft() {
    let (tokens, _) = tokenize("<<", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::token::TokenKind::ShiftLeft);
}

#[test]
fn op_ShiftRight() {
    let (tokens, _) = tokenize(">>", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::token::TokenKind::ShiftRight);
}

#[test]
fn op_Equal() {
    let (tokens, _) = tokenize("=", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::token::TokenKind::Equal);
}

#[test]
fn op_DoubleEqual() {
    let (tokens, _) = tokenize("==", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::token::TokenKind::DoubleEqual);
}

#[test]
fn op_NotEqual() {
    let (tokens, _) = tokenize("!=", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::token::TokenKind::NotEqual);
}

#[test]
fn op_LuaNotEqual() {
    let (tokens, _) = tokenize("~=", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::token::TokenKind::LuaNotEqual);
}

#[test]
fn op_LessEqual() {
    let (tokens, _) = tokenize("<=", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::token::TokenKind::LessEqual);
}

#[test]
fn op_GreaterEqual() {
    let (tokens, _) = tokenize(">=", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::token::TokenKind::GreaterEqual);
}

#[test]
fn op_Less() {
    let (tokens, _) = tokenize("<", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::token::TokenKind::Less);
}

#[test]
fn op_Greater() {
    let (tokens, _) = tokenize(">", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::token::TokenKind::Greater);
}

#[test]
fn op_PlusEqual() {
    let (tokens, _) = tokenize("+=", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::token::TokenKind::PlusEqual);
}

#[test]
fn op_MinusEqual() {
    let (tokens, _) = tokenize("-=", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::token::TokenKind::MinusEqual);
}

#[test]
fn op_StarEqual() {
    let (tokens, _) = tokenize("*=", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::token::TokenKind::StarEqual);
}

#[test]
fn op_SlashEqual() {
    let (tokens, _) = tokenize("/=", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::token::TokenKind::SlashEqual);
}

#[test]
fn op_PercentEqual() {
    let (tokens, _) = tokenize("%=", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::token::TokenKind::PercentEqual);
}

#[test]
fn op_AmpersandEqual() {
    let (tokens, _) = tokenize("&=", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::token::TokenKind::AmpersandEqual);
}

#[test]
fn op_PipeEqual() {
    let (tokens, _) = tokenize("|=", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::token::TokenKind::PipeEqual);
}

#[test]
fn op_CaretEqual() {
    let (tokens, _) = tokenize("^=", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::token::TokenKind::CaretEqual);
}

#[test]
fn op_Pipe() {
    let (tokens, _) = tokenize("|", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::token::TokenKind::Pipe);
}

#[test]
fn op_PipeForward() {
    let (tokens, _) = tokenize("|>", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::token::TokenKind::PipeForward);
}

