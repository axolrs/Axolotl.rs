// Owner: PascalElixir / axolrs (GitHub org)
// File: Auto-generated grid tests.


#![allow(non_snake_case)]
#[test]
fn lex_kw_KwLet_0() {
    let (tokens, _) = axolc_core::lexer::tokenize("let", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::TokenKind::KwLet);
    assert_eq!(tokens[0].text, "let");
}
#[test]
fn lex_kw_KwVar_1() {
    let (tokens, _) = axolc_core::lexer::tokenize("var", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::TokenKind::KwVar);
    assert_eq!(tokens[0].text, "var");
}
#[test]
fn lex_kw_KwFn_2() {
    let (tokens, _) = axolc_core::lexer::tokenize("fn", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::TokenKind::KwFn);
    assert_eq!(tokens[0].text, "fn");
}
#[test]
fn lex_kw_KwEnd_3() {
    let (tokens, _) = axolc_core::lexer::tokenize("end", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::TokenKind::KwEnd);
    assert_eq!(tokens[0].text, "end");
}
#[test]
fn lex_kw_KwIf_4() {
    let (tokens, _) = axolc_core::lexer::tokenize("if", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::TokenKind::KwIf);
    assert_eq!(tokens[0].text, "if");
}
#[test]
fn lex_kw_KwThen_5() {
    let (tokens, _) = axolc_core::lexer::tokenize("then", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::TokenKind::KwThen);
    assert_eq!(tokens[0].text, "then");
}
#[test]
fn lex_kw_KwElse_6() {
    let (tokens, _) = axolc_core::lexer::tokenize("else", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::TokenKind::KwElse);
    assert_eq!(tokens[0].text, "else");
}
#[test]
fn lex_kw_KwElseif_7() {
    let (tokens, _) = axolc_core::lexer::tokenize("elseif", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::TokenKind::KwElseif);
    assert_eq!(tokens[0].text, "elseif");
}
#[test]
fn lex_kw_KwWhile_8() {
    let (tokens, _) = axolc_core::lexer::tokenize("while", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::TokenKind::KwWhile);
    assert_eq!(tokens[0].text, "while");
}
#[test]
fn lex_kw_KwDo_9() {
    let (tokens, _) = axolc_core::lexer::tokenize("do", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::TokenKind::KwDo);
    assert_eq!(tokens[0].text, "do");
}
#[test]
fn lex_kw_KwRepeat_10() {
    let (tokens, _) = axolc_core::lexer::tokenize("repeat", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::TokenKind::KwRepeat);
    assert_eq!(tokens[0].text, "repeat");
}
#[test]
fn lex_kw_KwUntil_11() {
    let (tokens, _) = axolc_core::lexer::tokenize("until", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::TokenKind::KwUntil);
    assert_eq!(tokens[0].text, "until");
}
#[test]
fn lex_kw_KwFor_12() {
    let (tokens, _) = axolc_core::lexer::tokenize("for", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::TokenKind::KwFor);
    assert_eq!(tokens[0].text, "for");
}
#[test]
fn lex_kw_KwIn_13() {
    let (tokens, _) = axolc_core::lexer::tokenize("in", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::TokenKind::KwIn);
    assert_eq!(tokens[0].text, "in");
}
#[test]
fn lex_kw_KwBreak_14() {
    let (tokens, _) = axolc_core::lexer::tokenize("break", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::TokenKind::KwBreak);
    assert_eq!(tokens[0].text, "break");
}
#[test]
fn lex_kw_KwContinue_15() {
    let (tokens, _) = axolc_core::lexer::tokenize("continue", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::TokenKind::KwContinue);
    assert_eq!(tokens[0].text, "continue");
}
#[test]
fn lex_kw_KwReturn_16() {
    let (tokens, _) = axolc_core::lexer::tokenize("return", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::TokenKind::KwReturn);
    assert_eq!(tokens[0].text, "return");
}
#[test]
fn lex_kw_KwMatch_17() {
    let (tokens, _) = axolc_core::lexer::tokenize("match", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::TokenKind::KwMatch);
    assert_eq!(tokens[0].text, "match");
}
#[test]
fn lex_kw_KwStruct_18() {
    let (tokens, _) = axolc_core::lexer::tokenize("struct", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::TokenKind::KwStruct);
    assert_eq!(tokens[0].text, "struct");
}
#[test]
fn lex_kw_KwEnum_19() {
    let (tokens, _) = axolc_core::lexer::tokenize("enum", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::TokenKind::KwEnum);
    assert_eq!(tokens[0].text, "enum");
}
#[test]
fn lex_kw_KwInterface_20() {
    let (tokens, _) = axolc_core::lexer::tokenize("interface", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::TokenKind::KwInterface);
    assert_eq!(tokens[0].text, "interface");
}
#[test]
fn lex_kw_KwUse_21() {
    let (tokens, _) = axolc_core::lexer::tokenize("use", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::TokenKind::KwUse);
    assert_eq!(tokens[0].text, "use");
}
#[test]
fn lex_kw_KwPub_22() {
    let (tokens, _) = axolc_core::lexer::tokenize("pub", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::TokenKind::KwPub);
    assert_eq!(tokens[0].text, "pub");
}
#[test]
fn lex_kw_KwConst_23() {
    let (tokens, _) = axolc_core::lexer::tokenize("const", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::TokenKind::KwConst);
    assert_eq!(tokens[0].text, "const");
}
#[test]
fn lex_kw_KwTrue_24() {
    let (tokens, _) = axolc_core::lexer::tokenize("true", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::TokenKind::KwTrue);
    assert_eq!(tokens[0].text, "true");
}
#[test]
fn lex_kw_KwFalse_25() {
    let (tokens, _) = axolc_core::lexer::tokenize("false", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::TokenKind::KwFalse);
    assert_eq!(tokens[0].text, "false");
}
#[test]
fn lex_kw_KwNull_26() {
    let (tokens, _) = axolc_core::lexer::tokenize("null", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::TokenKind::KwNull);
    assert_eq!(tokens[0].text, "null");
}
#[test]
fn lex_kw_KwNil_27() {
    let (tokens, _) = axolc_core::lexer::tokenize("nil", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::TokenKind::KwNil);
    assert_eq!(tokens[0].text, "nil");
}
#[test]
fn lex_kw_KwAnd_28() {
    let (tokens, _) = axolc_core::lexer::tokenize("and", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::TokenKind::KwAnd);
    assert_eq!(tokens[0].text, "and");
}
#[test]
fn lex_kw_KwOr_29() {
    let (tokens, _) = axolc_core::lexer::tokenize("or", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::TokenKind::KwOr);
    assert_eq!(tokens[0].text, "or");
}
#[test]
fn lex_kw_KwNot_30() {
    let (tokens, _) = axolc_core::lexer::tokenize("not", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::TokenKind::KwNot);
    assert_eq!(tokens[0].text, "not");
}
#[test]
fn lex_kw_KwSpawn_31() {
    let (tokens, _) = axolc_core::lexer::tokenize("spawn", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::TokenKind::KwSpawn);
    assert_eq!(tokens[0].text, "spawn");
}
#[test]
fn lex_kw_KwAwait_32() {
    let (tokens, _) = axolc_core::lexer::tokenize("await", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::TokenKind::KwAwait);
    assert_eq!(tokens[0].text, "await");
}
#[test]
fn lex_kw_KwAsync_33() {
    let (tokens, _) = axolc_core::lexer::tokenize("async", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::TokenKind::KwAsync);
    assert_eq!(tokens[0].text, "async");
}
#[test]
fn lex_kw_KwTry_34() {
    let (tokens, _) = axolc_core::lexer::tokenize("try", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::TokenKind::KwTry);
    assert_eq!(tokens[0].text, "try");
}
#[test]
fn lex_kw_KwCatch_35() {
    let (tokens, _) = axolc_core::lexer::tokenize("catch", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::TokenKind::KwCatch);
    assert_eq!(tokens[0].text, "catch");
}
#[test]
fn lex_kw_KwUnsafe_36() {
    let (tokens, _) = axolc_core::lexer::tokenize("unsafe", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::TokenKind::KwUnsafe);
    assert_eq!(tokens[0].text, "unsafe");
}
#[test]
fn lex_kw_KwMove_37() {
    let (tokens, _) = axolc_core::lexer::tokenize("move", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::TokenKind::KwMove);
    assert_eq!(tokens[0].text, "move");
}
#[test]
fn lex_kw_KwBorrow_38() {
    let (tokens, _) = axolc_core::lexer::tokenize("borrow", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::TokenKind::KwBorrow);
    assert_eq!(tokens[0].text, "borrow");
}
#[test]
fn lex_kw_KwType_39() {
    let (tokens, _) = axolc_core::lexer::tokenize("type", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::TokenKind::KwType);
    assert_eq!(tokens[0].text, "type");
}
#[test]
fn lex_kw_KwAs_40() {
    let (tokens, _) = axolc_core::lexer::tokenize("as", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::TokenKind::KwAs);
    assert_eq!(tokens[0].text, "as");
}
#[test]
fn lex_kw_KwIs_41() {
    let (tokens, _) = axolc_core::lexer::tokenize("is", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::TokenKind::KwIs);
    assert_eq!(tokens[0].text, "is");
}
#[test]
fn lex_kw_KwExtern_42() {
    let (tokens, _) = axolc_core::lexer::tokenize("extern", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::TokenKind::KwExtern);
    assert_eq!(tokens[0].text, "extern");
}
#[test]
fn lex_kw_KwLoop_43() {
    let (tokens, _) = axolc_core::lexer::tokenize("loop", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::TokenKind::KwLoop);
    assert_eq!(tokens[0].text, "loop");
}
#[test]
fn lex_kw_KwCase_44() {
    let (tokens, _) = axolc_core::lexer::tokenize("case", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::TokenKind::KwCase);
    assert_eq!(tokens[0].text, "case");
}
#[test]
fn lex_kw_KwInt_45() {
    let (tokens, _) = axolc_core::lexer::tokenize("Int", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::TokenKind::KwInt);
    assert_eq!(tokens[0].text, "Int");
}
#[test]
fn lex_kw_KwUInt_46() {
    let (tokens, _) = axolc_core::lexer::tokenize("UInt", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::TokenKind::KwUInt);
    assert_eq!(tokens[0].text, "UInt");
}
#[test]
fn lex_kw_KwFloat_47() {
    let (tokens, _) = axolc_core::lexer::tokenize("Float", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::TokenKind::KwFloat);
    assert_eq!(tokens[0].text, "Float");
}
#[test]
fn lex_kw_KwDouble_48() {
    let (tokens, _) = axolc_core::lexer::tokenize("Double", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::TokenKind::KwDouble);
    assert_eq!(tokens[0].text, "Double");
}
#[test]
fn lex_kw_KwBool_49() {
    let (tokens, _) = axolc_core::lexer::tokenize("Bool", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::TokenKind::KwBool);
    assert_eq!(tokens[0].text, "Bool");
}
#[test]
fn lex_kw_KwString_50() {
    let (tokens, _) = axolc_core::lexer::tokenize("String", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::TokenKind::KwString);
    assert_eq!(tokens[0].text, "String");
}
#[test]
fn lex_kw_KwChar_51() {
    let (tokens, _) = axolc_core::lexer::tokenize("Char", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::TokenKind::KwChar);
    assert_eq!(tokens[0].text, "Char");
}
#[test]
fn lex_kw_KwByte_52() {
    let (tokens, _) = axolc_core::lexer::tokenize("Byte", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::TokenKind::KwByte);
    assert_eq!(tokens[0].text, "Byte");
}
#[test]
fn lex_kw_KwI8_53() {
    let (tokens, _) = axolc_core::lexer::tokenize("I8", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::TokenKind::KwI8);
    assert_eq!(tokens[0].text, "I8");
}
#[test]
fn lex_kw_KwI16_54() {
    let (tokens, _) = axolc_core::lexer::tokenize("I16", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::TokenKind::KwI16);
    assert_eq!(tokens[0].text, "I16");
}
#[test]
fn lex_kw_KwI32_55() {
    let (tokens, _) = axolc_core::lexer::tokenize("I32", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::TokenKind::KwI32);
    assert_eq!(tokens[0].text, "I32");
}
#[test]
fn lex_kw_KwI64_56() {
    let (tokens, _) = axolc_core::lexer::tokenize("I64", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::TokenKind::KwI64);
    assert_eq!(tokens[0].text, "I64");
}
#[test]
fn lex_kw_KwI128_57() {
    let (tokens, _) = axolc_core::lexer::tokenize("I128", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::TokenKind::KwI128);
    assert_eq!(tokens[0].text, "I128");
}
#[test]
fn lex_kw_KwU8_58() {
    let (tokens, _) = axolc_core::lexer::tokenize("U8", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::TokenKind::KwU8);
    assert_eq!(tokens[0].text, "U8");
}
#[test]
fn lex_kw_KwU16_59() {
    let (tokens, _) = axolc_core::lexer::tokenize("U16", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::TokenKind::KwU16);
    assert_eq!(tokens[0].text, "U16");
}
#[test]
fn lex_kw_KwU32_60() {
    let (tokens, _) = axolc_core::lexer::tokenize("U32", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::TokenKind::KwU32);
    assert_eq!(tokens[0].text, "U32");
}
#[test]
fn lex_kw_KwU64_61() {
    let (tokens, _) = axolc_core::lexer::tokenize("U64", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::TokenKind::KwU64);
    assert_eq!(tokens[0].text, "U64");
}
#[test]
fn lex_kw_KwU128_62() {
    let (tokens, _) = axolc_core::lexer::tokenize("U128", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::TokenKind::KwU128);
    assert_eq!(tokens[0].text, "U128");
}
#[test]
fn lex_kw_KwF32_63() {
    let (tokens, _) = axolc_core::lexer::tokenize("F32", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::TokenKind::KwF32);
    assert_eq!(tokens[0].text, "F32");
}
#[test]
fn lex_kw_KwF64_64() {
    let (tokens, _) = axolc_core::lexer::tokenize("F64", 0);
    assert_eq!(tokens[0].kind, axolc_core::lexer::TokenKind::KwF64);
    assert_eq!(tokens[0].text, "F64");
}
