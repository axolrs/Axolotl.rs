// Owner: PascalElixir / axolrs (GitHub org)
// File: Auto-generated tests for keyword token kinds.

use axolc_core::tokenize;

#[test]
fn keyword_let() {
    let (tokens, _) = tokenize("let", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::token::TokenKind::KwLet);
}

#[test]
fn keyword_var() {
    let (tokens, _) = tokenize("var", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::token::TokenKind::KwVar);
}

#[test]
fn keyword_fn() {
    let (tokens, _) = tokenize("fn", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::token::TokenKind::KwFn);
}

#[test]
fn keyword_end() {
    let (tokens, _) = tokenize("end", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::token::TokenKind::KwEnd);
}

#[test]
fn keyword_if() {
    let (tokens, _) = tokenize("if", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::token::TokenKind::KwIf);
}

#[test]
fn keyword_then() {
    let (tokens, _) = tokenize("then", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::token::TokenKind::KwThen);
}

#[test]
fn keyword_else() {
    let (tokens, _) = tokenize("else", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::token::TokenKind::KwElse);
}

#[test]
fn keyword_elseif() {
    let (tokens, _) = tokenize("elseif", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::token::TokenKind::KwElseif);
}

#[test]
fn keyword_while() {
    let (tokens, _) = tokenize("while", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::token::TokenKind::KwWhile);
}

#[test]
fn keyword_do() {
    let (tokens, _) = tokenize("do", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::token::TokenKind::KwDo);
}

#[test]
fn keyword_repeat() {
    let (tokens, _) = tokenize("repeat", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::token::TokenKind::KwRepeat);
}

#[test]
fn keyword_until() {
    let (tokens, _) = tokenize("until", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::token::TokenKind::KwUntil);
}

#[test]
fn keyword_for() {
    let (tokens, _) = tokenize("for", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::token::TokenKind::KwFor);
}

#[test]
fn keyword_in() {
    let (tokens, _) = tokenize("in", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::token::TokenKind::KwIn);
}

#[test]
fn keyword_break() {
    let (tokens, _) = tokenize("break", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::token::TokenKind::KwBreak);
}

#[test]
fn keyword_continue() {
    let (tokens, _) = tokenize("continue", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::token::TokenKind::KwContinue);
}

#[test]
fn keyword_return() {
    let (tokens, _) = tokenize("return", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::token::TokenKind::KwReturn);
}

#[test]
fn keyword_match() {
    let (tokens, _) = tokenize("match", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::token::TokenKind::KwMatch);
}

#[test]
fn keyword_struct() {
    let (tokens, _) = tokenize("struct", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::token::TokenKind::KwStruct);
}

#[test]
fn keyword_enum() {
    let (tokens, _) = tokenize("enum", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::token::TokenKind::KwEnum);
}

#[test]
fn keyword_interface() {
    let (tokens, _) = tokenize("interface", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::token::TokenKind::KwInterface);
}

#[test]
fn keyword_use() {
    let (tokens, _) = tokenize("use", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::token::TokenKind::KwUse);
}

#[test]
fn keyword_pub() {
    let (tokens, _) = tokenize("pub", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::token::TokenKind::KwPub);
}

#[test]
fn keyword_const() {
    let (tokens, _) = tokenize("const", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::token::TokenKind::KwConst);
}

#[test]
fn keyword_true() {
    let (tokens, _) = tokenize("true", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::token::TokenKind::KwTrue);
}

#[test]
fn keyword_false() {
    let (tokens, _) = tokenize("false", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::token::TokenKind::KwFalse);
}

#[test]
fn keyword_null() {
    let (tokens, _) = tokenize("null", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::token::TokenKind::KwNull);
}

#[test]
fn keyword_nil() {
    let (tokens, _) = tokenize("nil", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::token::TokenKind::KwNil);
}

#[test]
fn keyword_and() {
    let (tokens, _) = tokenize("and", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::token::TokenKind::KwAnd);
}

#[test]
fn keyword_or() {
    let (tokens, _) = tokenize("or", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::token::TokenKind::KwOr);
}

#[test]
fn keyword_not() {
    let (tokens, _) = tokenize("not", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::token::TokenKind::KwNot);
}

#[test]
fn keyword_spawn() {
    let (tokens, _) = tokenize("spawn", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::token::TokenKind::KwSpawn);
}

#[test]
fn keyword_await() {
    let (tokens, _) = tokenize("await", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::token::TokenKind::KwAwait);
}

#[test]
fn keyword_async() {
    let (tokens, _) = tokenize("async", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::token::TokenKind::KwAsync);
}

#[test]
fn keyword_try() {
    let (tokens, _) = tokenize("try", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::token::TokenKind::KwTry);
}

#[test]
fn keyword_catch() {
    let (tokens, _) = tokenize("catch", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::token::TokenKind::KwCatch);
}

#[test]
fn keyword_unsafe() {
    let (tokens, _) = tokenize("unsafe", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::token::TokenKind::KwUnsafe);
}

#[test]
fn keyword_move() {
    let (tokens, _) = tokenize("move", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::token::TokenKind::KwMove);
}

#[test]
fn keyword_borrow() {
    let (tokens, _) = tokenize("borrow", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::token::TokenKind::KwBorrow);
}

#[test]
fn keyword_type() {
    let (tokens, _) = tokenize("type", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::token::TokenKind::KwType);
}

#[test]
fn keyword_as() {
    let (tokens, _) = tokenize("as", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::token::TokenKind::KwAs);
}

#[test]
fn keyword_is() {
    let (tokens, _) = tokenize("is", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::token::TokenKind::KwIs);
}

#[test]
fn keyword_extern() {
    let (tokens, _) = tokenize("extern", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::token::TokenKind::KwExtern);
}

#[test]
fn keyword_loop() {
    let (tokens, _) = tokenize("loop", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::token::TokenKind::KwLoop);
}

#[test]
fn keyword_case() {
    let (tokens, _) = tokenize("case", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::token::TokenKind::KwCase);
}

