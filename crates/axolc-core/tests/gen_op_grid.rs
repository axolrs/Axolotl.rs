// Owner: PascalElixir / axolrs (GitHub org)
// File: Auto-generated grid tests.


#![allow(non_snake_case)]
#[test]
fn lex_op_LParen_0() {
    let (tokens, _) = axolc_core::lexer::tokenize("(", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::TokenKind::LParen);
}
#[test]
fn lex_op_RParen_1() {
    let (tokens, _) = axolc_core::lexer::tokenize(")", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::TokenKind::RParen);
}
#[test]
fn lex_op_LBrace_2() {
    let (tokens, _) = axolc_core::lexer::tokenize("{", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::TokenKind::LBrace);
}
#[test]
fn lex_op_RBrace_3() {
    let (tokens, _) = axolc_core::lexer::tokenize("}", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::TokenKind::RBrace);
}
#[test]
fn lex_op_LBracket_4() {
    let (tokens, _) = axolc_core::lexer::tokenize("[", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::TokenKind::LBracket);
}
#[test]
fn lex_op_RBracket_5() {
    let (tokens, _) = axolc_core::lexer::tokenize("]", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::TokenKind::RBracket);
}
#[test]
fn lex_op_Comma_6() {
    let (tokens, _) = axolc_core::lexer::tokenize(",", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::TokenKind::Comma);
}
#[test]
fn lex_op_Semicolon_7() {
    let (tokens, _) = axolc_core::lexer::tokenize(";", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::TokenKind::Semicolon);
}
#[test]
fn lex_op_Colon_8() {
    let (tokens, _) = axolc_core::lexer::tokenize(":", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::TokenKind::Colon);
}
#[test]
fn lex_op_DoubleColon_9() {
    let (tokens, _) = axolc_core::lexer::tokenize("::", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::TokenKind::DoubleColon);
}
#[test]
fn lex_op_Dot_10() {
    let (tokens, _) = axolc_core::lexer::tokenize(".", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::TokenKind::Dot);
}
#[test]
fn lex_op_DotDot_11() {
    let (tokens, _) = axolc_core::lexer::tokenize("..", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::TokenKind::DotDot);
}
#[test]
fn lex_op_DotDotDot_12() {
    let (tokens, _) = axolc_core::lexer::tokenize("...", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::TokenKind::DotDotDot);
}
#[test]
fn lex_op_Arrow_13() {
    let (tokens, _) = axolc_core::lexer::tokenize("->", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::TokenKind::Arrow);
}
#[test]
fn lex_op_FatArrow_14() {
    let (tokens, _) = axolc_core::lexer::tokenize("=>", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::TokenKind::FatArrow);
}
#[test]
fn lex_op_Question_15() {
    let (tokens, _) = axolc_core::lexer::tokenize("?", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::TokenKind::Question);
}
#[test]
fn lex_op_DoubleQuestion_16() {
    let (tokens, _) = axolc_core::lexer::tokenize("??", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::TokenKind::DoubleQuestion);
}
#[test]
fn lex_op_QuestionDot_17() {
    let (tokens, _) = axolc_core::lexer::tokenize("?.", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::TokenKind::QuestionDot);
}
#[test]
fn lex_op_Bang_18() {
    let (tokens, _) = axolc_core::lexer::tokenize("!", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::TokenKind::Bang);
}
#[test]
fn lex_op_DoubleBang_19() {
    let (tokens, _) = axolc_core::lexer::tokenize("!!", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::TokenKind::DoubleBang);
}
#[test]
fn lex_op_At_20() {
    let (tokens, _) = axolc_core::lexer::tokenize("@", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::TokenKind::At);
}
#[test]
fn lex_op_Hash_21() {
    let (tokens, _) = axolc_core::lexer::tokenize("#", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::TokenKind::Hash);
}
#[test]
fn lex_op_Dollar_22() {
    let (tokens, _) = axolc_core::lexer::tokenize("$", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::TokenKind::Dollar);
}
#[test]
fn lex_op_Ampersand_23() {
    let (tokens, _) = axolc_core::lexer::tokenize("&", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::TokenKind::Ampersand);
}
#[test]
fn lex_op_Plus_24() {
    let (tokens, _) = axolc_core::lexer::tokenize("+", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::TokenKind::Plus);
}
#[test]
fn lex_op_Minus_25() {
    let (tokens, _) = axolc_core::lexer::tokenize("-", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::TokenKind::Minus);
}
#[test]
fn lex_op_Star_26() {
    let (tokens, _) = axolc_core::lexer::tokenize("*", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::TokenKind::Star);
}
#[test]
fn lex_op_Slash_27() {
    let (tokens, _) = axolc_core::lexer::tokenize("/", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::TokenKind::Slash);
}
#[test]
fn lex_op_Percent_28() {
    let (tokens, _) = axolc_core::lexer::tokenize("%", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::TokenKind::Percent);
}
#[test]
fn lex_op_Caret_29() {
    let (tokens, _) = axolc_core::lexer::tokenize("^", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::TokenKind::Caret);
}
#[test]
fn lex_op_Tilde_30() {
    let (tokens, _) = axolc_core::lexer::tokenize("~", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::TokenKind::Tilde);
}
#[test]
fn lex_op_ShiftLeft_31() {
    let (tokens, _) = axolc_core::lexer::tokenize("<<", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::TokenKind::ShiftLeft);
}
#[test]
fn lex_op_ShiftRight_32() {
    let (tokens, _) = axolc_core::lexer::tokenize(">>", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::TokenKind::ShiftRight);
}
#[test]
fn lex_op_Equal_33() {
    let (tokens, _) = axolc_core::lexer::tokenize("=", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::TokenKind::Equal);
}
#[test]
fn lex_op_DoubleEqual_34() {
    let (tokens, _) = axolc_core::lexer::tokenize("==", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::TokenKind::DoubleEqual);
}
#[test]
fn lex_op_NotEqual_35() {
    let (tokens, _) = axolc_core::lexer::tokenize("!=", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::TokenKind::NotEqual);
}
#[test]
fn lex_op_LuaNotEqual_36() {
    let (tokens, _) = axolc_core::lexer::tokenize("~=", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::TokenKind::LuaNotEqual);
}
#[test]
fn lex_op_LessEqual_37() {
    let (tokens, _) = axolc_core::lexer::tokenize("<=", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::TokenKind::LessEqual);
}
#[test]
fn lex_op_GreaterEqual_38() {
    let (tokens, _) = axolc_core::lexer::tokenize(">=", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::TokenKind::GreaterEqual);
}
#[test]
fn lex_op_Less_39() {
    let (tokens, _) = axolc_core::lexer::tokenize("<", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::TokenKind::Less);
}
#[test]
fn lex_op_Greater_40() {
    let (tokens, _) = axolc_core::lexer::tokenize(">", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::TokenKind::Greater);
}
#[test]
fn lex_op_PlusEqual_41() {
    let (tokens, _) = axolc_core::lexer::tokenize("+=", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::TokenKind::PlusEqual);
}
#[test]
fn lex_op_MinusEqual_42() {
    let (tokens, _) = axolc_core::lexer::tokenize("-=", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::TokenKind::MinusEqual);
}
#[test]
fn lex_op_StarEqual_43() {
    let (tokens, _) = axolc_core::lexer::tokenize("*=", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::TokenKind::StarEqual);
}
#[test]
fn lex_op_SlashEqual_44() {
    let (tokens, _) = axolc_core::lexer::tokenize("/=", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::TokenKind::SlashEqual);
}
#[test]
fn lex_op_PercentEqual_45() {
    let (tokens, _) = axolc_core::lexer::tokenize("%=", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::TokenKind::PercentEqual);
}
#[test]
fn lex_op_AmpersandEqual_46() {
    let (tokens, _) = axolc_core::lexer::tokenize("&=", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::TokenKind::AmpersandEqual);
}
#[test]
fn lex_op_PipeEqual_47() {
    let (tokens, _) = axolc_core::lexer::tokenize("|=", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::TokenKind::PipeEqual);
}
#[test]
fn lex_op_CaretEqual_48() {
    let (tokens, _) = axolc_core::lexer::tokenize("^=", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::TokenKind::CaretEqual);
}
#[test]
fn lex_op_Pipe_49() {
    let (tokens, _) = axolc_core::lexer::tokenize("|", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::TokenKind::Pipe);
}
#[test]
fn lex_op_PipeForward_50() {
    let (tokens, _) = axolc_core::lexer::tokenize("|>", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::TokenKind::PipeForward);
}
