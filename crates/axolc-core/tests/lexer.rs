// Owner: PascalElixir / axolrs (GitHub org)
// File: Lexer tests - verify every keyword, operator, and literal kind.

use axolc_core::lexer::{tokenize, TokenKind};
use axolc_testkit::token_count;

#[test]
fn lexes_empty_source() {
    let (tokens, diags) = tokenize("", 0);
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].kind, TokenKind::Eof);
    assert!(!diags.has_errors());
}

#[test]
fn lexes_single_line_comment() {
    let (tokens, _) = tokenize("-- this is a comment\n", 0);
    assert_eq!(tokens.last().unwrap().kind, TokenKind::Eof);
    assert_eq!(tokens.len(), 1);
}

#[test]
fn lexes_block_comment() {
    let (tokens, _) = tokenize("--[[ block comment ]]\n", 0);
    assert_eq!(tokens.last().unwrap().kind, TokenKind::Eof);
}

#[test]
fn lexes_let_keyword() {
    let (tokens, _) = tokenize("let x = 1", 0);
    assert_eq!(tokens[0].kind, TokenKind::KwLet);
}

#[test]
fn lexes_var_keyword() {
    let (tokens, _) = tokenize("var x = 1", 0);
    assert_eq!(tokens[0].kind, TokenKind::KwVar);
}

#[test]
fn lexes_fn_keyword() {
    let (tokens, _) = tokenize("fn f() end", 0);
    assert_eq!(tokens[0].kind, TokenKind::KwFn);
}

#[test]
fn lexes_end_keyword() {
    let (tokens, _) = tokenize("end", 0);
    assert_eq!(tokens[0].kind, TokenKind::KwEnd);
}

#[test]
fn lexes_if_then_else() {
    let (tokens, _) = tokenize("if true then else end", 0);
    assert_eq!(tokens[0].kind, TokenKind::KwIf);
    assert_eq!(tokens[1].kind, TokenKind::KwTrue);
    assert_eq!(tokens[2].kind, TokenKind::KwThen);
    assert_eq!(tokens[3].kind, TokenKind::KwElse);
    assert_eq!(tokens[4].kind, TokenKind::KwEnd);
}

#[test]
fn lexes_while_do() {
    let (tokens, _) = tokenize("while true do end", 0);
    assert_eq!(tokens[0].kind, TokenKind::KwWhile);
    assert_eq!(tokens[2].kind, TokenKind::KwDo);
}

#[test]
fn lexes_for_in() {
    let (tokens, _) = tokenize("for x in [1,2,3] do end", 0);
    assert_eq!(tokens[0].kind, TokenKind::KwFor);
    assert_eq!(tokens[2].kind, TokenKind::KwIn);
}

#[test]
fn lexes_match() {
    let (tokens, _) = tokenize("match x end", 0);
    assert_eq!(tokens[0].kind, TokenKind::KwMatch);
}

#[test]
fn lexes_struct() {
    let (tokens, _) = tokenize("Foo = struct end", 0);
    assert_eq!(tokens[2].kind, TokenKind::KwStruct);
}

#[test]
fn lexes_enum() {
    let (tokens, _) = tokenize("Color = enum Red end", 0);
    assert_eq!(tokens[2].kind, TokenKind::KwEnum);
}

#[test]
fn lexes_interface() {
    let (tokens, _) = tokenize("Drawable = interface end", 0);
    assert_eq!(tokens[2].kind, TokenKind::KwInterface);
}

#[test]
fn lexes_use() {
    let (tokens, _) = tokenize("use \"tokio\"", 0);
    assert_eq!(tokens[0].kind, TokenKind::KwUse);
}

#[test]
fn lexes_pub() {
    let (tokens, _) = tokenize("pub fn f() end", 0);
    assert_eq!(tokens[0].kind, TokenKind::KwPub);
}

#[test]
fn lexes_const() {
    let (tokens, _) = tokenize("const MAX = 100", 0);
    assert_eq!(tokens[0].kind, TokenKind::KwConst);
}

#[test]
fn lexes_true_false_null() {
    let (tokens, _) = tokenize("true false null", 0);
    assert_eq!(tokens[0].kind, TokenKind::KwTrue);
    assert_eq!(tokens[1].kind, TokenKind::KwFalse);
    assert_eq!(tokens[2].kind, TokenKind::KwNull);
}

#[test]
fn lexes_and_or_not() {
    let (tokens, _) = tokenize("and or not", 0);
    assert_eq!(tokens[0].kind, TokenKind::KwAnd);
    assert_eq!(tokens[1].kind, TokenKind::KwOr);
    assert_eq!(tokens[2].kind, TokenKind::KwNot);
}

#[test]
fn lexes_spawn_await_async() {
    let (tokens, _) = tokenize("spawn await async", 0);
    assert_eq!(tokens[0].kind, TokenKind::KwSpawn);
    assert_eq!(tokens[1].kind, TokenKind::KwAwait);
    assert_eq!(tokens[2].kind, TokenKind::KwAsync);
}

#[test]
fn lexes_try_catch_unsafe() {
    let (tokens, _) = tokenize("try catch unsafe", 0);
    assert_eq!(tokens[0].kind, TokenKind::KwTry);
    assert_eq!(tokens[1].kind, TokenKind::KwCatch);
    assert_eq!(tokens[2].kind, TokenKind::KwUnsafe);
}

#[test]
fn lexes_move_borrow() {
    let (tokens, _) = tokenize("move borrow", 0);
    assert_eq!(tokens[0].kind, TokenKind::KwMove);
    assert_eq!(tokens[1].kind, TokenKind::KwBorrow);
}

#[test]
fn lexes_type_as_is() {
    let (tokens, _) = tokenize("type as is", 0);
    assert_eq!(tokens[0].kind, TokenKind::KwType);
    assert_eq!(tokens[1].kind, TokenKind::KwAs);
    assert_eq!(tokens[2].kind, TokenKind::KwIs);
}

#[test]
fn lexes_extern() {
    let (tokens, _) = tokenize("extern \"C\"", 0);
    assert_eq!(tokens[0].kind, TokenKind::KwExtern);
}

#[test]
fn lexes_break_continue_return() {
    let (tokens, _) = tokenize("break continue return", 0);
    assert_eq!(tokens[0].kind, TokenKind::KwBreak);
    assert_eq!(tokens[1].kind, TokenKind::KwContinue);
    assert_eq!(tokens[2].kind, TokenKind::KwReturn);
}

#[test]
fn lexes_repeat_until() {
    let (tokens, _) = tokenize("repeat until false", 0);
    assert_eq!(tokens[0].kind, TokenKind::KwRepeat);
    assert_eq!(tokens[1].kind, TokenKind::KwUntil);
}

#[test]
fn lexes_loop() {
    let (tokens, _) = tokenize("loop end", 0);
    assert_eq!(tokens[0].kind, TokenKind::KwLoop);
}

#[test]
fn lexes_primitive_types() {
    let (tokens, _) = tokenize("Int UInt Float Bool String Char Byte", 0);
    assert_eq!(tokens[0].kind, TokenKind::KwInt);
    assert_eq!(tokens[1].kind, TokenKind::KwUInt);
    assert_eq!(tokens[2].kind, TokenKind::KwFloat);
    assert_eq!(tokens[3].kind, TokenKind::KwBool);
    assert_eq!(tokens[4].kind, TokenKind::KwString);
    assert_eq!(tokens[5].kind, TokenKind::KwChar);
    assert_eq!(tokens[6].kind, TokenKind::KwByte);
}

#[test]
fn lexes_sized_int_types() {
    let (tokens, _) = tokenize("I8 I16 I32 I64 I128", 0);
    assert_eq!(tokens[0].kind, TokenKind::KwI8);
    assert_eq!(tokens[1].kind, TokenKind::KwI16);
    assert_eq!(tokens[2].kind, TokenKind::KwI32);
    assert_eq!(tokens[3].kind, TokenKind::KwI64);
    assert_eq!(tokens[4].kind, TokenKind::KwI128);
}

#[test]
fn lexes_sized_uint_types() {
    let (tokens, _) = tokenize("U8 U16 U32 U64 U128", 0);
    assert_eq!(tokens[0].kind, TokenKind::KwU8);
    assert_eq!(tokens[1].kind, TokenKind::KwU16);
    assert_eq!(tokens[2].kind, TokenKind::KwU32);
    assert_eq!(tokens[3].kind, TokenKind::KwU64);
    assert_eq!(tokens[4].kind, TokenKind::KwU128);
}

#[test]
fn lexes_sized_float_types() {
    let (tokens, _) = tokenize("F32 F64", 0);
    assert_eq!(tokens[0].kind, TokenKind::KwF32);
    assert_eq!(tokens[1].kind, TokenKind::KwF64);
}

#[test]
fn lexes_self() {
    let (tokens, _) = tokenize("self Self", 0);
    assert_eq!(tokens[0].kind, TokenKind::KwSelfType);
    assert_eq!(tokens[1].kind, TokenKind::KwSelfType);
}

#[test]
fn lexes_punctuation() {
    let (tokens, _) = tokenize("() {} [] , ; :", 0);
    assert_eq!(tokens[0].kind, TokenKind::LParen);
    assert_eq!(tokens[1].kind, TokenKind::RParen);
    assert_eq!(tokens[2].kind, TokenKind::LBrace);
    assert_eq!(tokens[3].kind, TokenKind::RBrace);
    assert_eq!(tokens[4].kind, TokenKind::LBracket);
    assert_eq!(tokens[5].kind, TokenKind::RBracket);
    assert_eq!(tokens[6].kind, TokenKind::Comma);
    assert_eq!(tokens[7].kind, TokenKind::Semicolon);
    assert_eq!(tokens[8].kind, TokenKind::Colon);
}

#[test]
fn lexes_dot_operators() {
    let (tokens, _) = tokenize(". .. ...", 0);
    assert_eq!(tokens[0].kind, TokenKind::Dot);
    assert_eq!(tokens[1].kind, TokenKind::DotDot);
    assert_eq!(tokens[2].kind, TokenKind::DotDotDot);
}

#[test]
fn lexes_arithmetic_operators() {
    let (tokens, _) = tokenize("+ - * / %", 0);
    assert_eq!(tokens[0].kind, TokenKind::Plus);
    assert_eq!(tokens[1].kind, TokenKind::Minus);
    assert_eq!(tokens[2].kind, TokenKind::Star);
    assert_eq!(tokens[3].kind, TokenKind::Slash);
    assert_eq!(tokens[4].kind, TokenKind::Percent);
}

#[test]
fn lexes_compound_assignment() {
    let (tokens, _) = tokenize("+= -= *= /= %= &= |= ^=", 0);
    assert_eq!(tokens[0].kind, TokenKind::PlusEqual);
    assert_eq!(tokens[1].kind, TokenKind::MinusEqual);
    assert_eq!(tokens[2].kind, TokenKind::StarEqual);
    assert_eq!(tokens[3].kind, TokenKind::SlashEqual);
    assert_eq!(tokens[4].kind, TokenKind::PercentEqual);
    assert_eq!(tokens[5].kind, TokenKind::AmpersandEqual);
    assert_eq!(tokens[6].kind, TokenKind::PipeEqual);
    assert_eq!(tokens[7].kind, TokenKind::CaretEqual);
}

#[test]
fn lexes_comparison_operators() {
    let (tokens, _) = tokenize("== != ~= < <= > >=", 0);
    assert_eq!(tokens[0].kind, TokenKind::DoubleEqual);
    assert_eq!(tokens[1].kind, TokenKind::NotEqual);
    assert_eq!(tokens[2].kind, TokenKind::LuaNotEqual);
    assert_eq!(tokens[3].kind, TokenKind::Less);
    assert_eq!(tokens[4].kind, TokenKind::LessEqual);
    assert_eq!(tokens[5].kind, TokenKind::Greater);
    assert_eq!(tokens[6].kind, TokenKind::GreaterEqual);
}

#[test]
fn lexes_assignment() {
    let (tokens, _) = tokenize("=", 0);
    assert_eq!(tokens[0].kind, TokenKind::Equal);
}

#[test]
fn lexes_arrows() {
    let (tokens, _) = tokenize("-> => |> <|", 0);
    assert_eq!(tokens[0].kind, TokenKind::Arrow);
    assert_eq!(tokens[1].kind, TokenKind::FatArrow);
    assert_eq!(tokens[2].kind, TokenKind::PipeForward);
    assert_eq!(tokens[3].kind, TokenKind::BackPipe);
}

#[test]
fn lexes_question_marks() {
    let (tokens, _) = tokenize("? ?? ?.", 0);
    assert_eq!(tokens[0].kind, TokenKind::Question);
    assert_eq!(tokens[1].kind, TokenKind::DoubleQuestion);
    assert_eq!(tokens[2].kind, TokenKind::QuestionDot);
}

#[test]
fn lexes_bangs() {
    let (tokens, _) = tokenize("! !!", 0);
    assert_eq!(tokens[0].kind, TokenKind::Bang);
    assert_eq!(tokens[1].kind, TokenKind::DoubleBang);
}

#[test]
fn lexes_at_hash_dollar() {
    let (tokens, _) = tokenize("@ # $", 0);
    assert_eq!(tokens[0].kind, TokenKind::At);
    assert_eq!(tokens[1].kind, TokenKind::Hash);
    assert_eq!(tokens[2].kind, TokenKind::Dollar);
}

#[test]
fn lexes_amp_caret_tilde() {
    let (tokens, _) = tokenize("& ^ ~", 0);
    assert_eq!(tokens[0].kind, TokenKind::Ampersand);
    assert_eq!(tokens[1].kind, TokenKind::Caret);
    assert_eq!(tokens[2].kind, TokenKind::Tilde);
}

#[test]
fn lexes_shifts() {
    let (tokens, _) = tokenize("<< >> <<= >>=", 0);
    assert_eq!(tokens[0].kind, TokenKind::ShiftLeft);
    assert_eq!(tokens[1].kind, TokenKind::ShiftRight);
    assert_eq!(tokens[2].kind, TokenKind::ShiftLeftEqual);
    assert_eq!(tokens[3].kind, TokenKind::ShiftRightEqual);
}

#[test]
fn lexes_double_colon() {
    let (tokens, _) = tokenize("::", 0);
    assert_eq!(tokens[0].kind, TokenKind::DoubleColon);
}

#[test]
fn lexes_pipe() {
    let (tokens, _) = tokenize("|", 0);
    assert_eq!(tokens[0].kind, TokenKind::Pipe);
}

#[test]
fn lexes_integer_literal() {
    let (tokens, _) = tokenize("42", 0);
    assert_eq!(tokens[0].kind, TokenKind::IntLit);
    assert_eq!(tokens[0].text, "42");
}

#[test]
fn lexes_underscored_integer() {
    let (tokens, _) = tokenize("1_000_000", 0);
    assert_eq!(tokens[0].kind, TokenKind::IntLit);
    assert_eq!(tokens[0].text, "1_000_000");
}

#[test]
fn lexes_hex_integer() {
    let (tokens, _) = tokenize("0xFF", 0);
    assert_eq!(tokens[0].kind, TokenKind::IntLit);
}

#[test]
fn lexes_float_literal() {
    let (tokens, _) = tokenize("3.14", 0);
    assert_eq!(tokens[0].kind, TokenKind::FloatLit);
    assert_eq!(tokens[0].text, "3.14");
}

#[test]
fn lexes_scientific_float() {
    let (tokens, _) = tokenize("1.5e10", 0);
    assert_eq!(tokens[0].kind, TokenKind::FloatLit);
}

#[test]
fn lexes_negative_exponent_float() {
    let (tokens, _) = tokenize("1.5e-10", 0);
    assert_eq!(tokens[0].kind, TokenKind::FloatLit);
}

#[test]
fn lexes_string_literal() {
    let (tokens, _) = tokenize("\"hello\"", 0);
    assert_eq!(tokens[0].kind, TokenKind::StrLit);
    assert_eq!(tokens[0].text, "hello");
}

#[test]
fn lexes_escaped_string() {
    let (tokens, _) = tokenize("\"hello\\nworld\"", 0);
    assert_eq!(tokens[0].kind, TokenKind::StrLit);
}

#[test]
fn lexes_char_literal() {
    let (tokens, _) = tokenize("'x'", 0);
    assert_eq!(tokens[0].kind, TokenKind::CharLit);
    assert_eq!(tokens[0].text, "x");
}

#[test]
fn lexes_raw_string_literal() {
    let (tokens, _) = tokenize("`raw`", 0);
    assert_eq!(tokens[0].kind, TokenKind::RawStrLit);
    assert_eq!(tokens[0].text, "raw");
}

#[test]
fn lexes_number_suffix() {
    let (tokens, _) = tokenize("4_KB", 0);
    assert_eq!(tokens[0].kind, TokenKind::IntLit);
    assert_eq!(tokens[0].text, "4_KB");
}

#[test]
fn lexes_identifier() {
    let (tokens, _) = tokenize("foo_bar", 0);
    assert_eq!(tokens[0].kind, TokenKind::Ident);
    assert_eq!(tokens[0].text, "foo_bar");
}

#[test]
fn lexes_pascal_identifier() {
    let (tokens, _) = tokenize("Player", 0);
    assert_eq!(tokens[0].kind, TokenKind::Ident);
    assert_eq!(tokens[0].text, "Player");
}

#[test]
fn lexes_interpolation_string() {
    let (tokens, _) = tokenize("\"name = ${x}\"", 0);
    assert_eq!(tokens[0].kind, TokenKind::StrPart);
    assert_eq!(tokens[0].text, "name = ");
    assert_eq!(tokens[1].kind, TokenKind::InterpOpen);
    assert_eq!(tokens[2].kind, TokenKind::Ident);
    assert_eq!(tokens[2].text, "x");
    assert_eq!(tokens[3].kind, TokenKind::InterpClose);
    assert_eq!(tokens[4].kind, TokenKind::StrLit);
}

#[test]
fn lexes_interpolation_with_nested_braces() {
    let (tokens, _) = tokenize("\"${Point{x: 1}.y}\"", 0);
    let kinds: Vec<TokenKind> = tokens.iter().map(|t| t.kind).collect();
    let pos_open = kinds.iter().position(|k| *k == TokenKind::InterpOpen).unwrap();
    let pos_close = kinds.iter().position(|k| *k == TokenKind::InterpClose).unwrap();
    assert_eq!(tokens[pos_open].kind, TokenKind::InterpOpen);
    assert!(kinds[..pos_close].contains(&TokenKind::LBrace));
    assert!(kinds[..pos_close].contains(&TokenKind::RBrace));
}

#[test]
fn lexes_nested_string_inside_interpolation() {
    let (tokens, diags) = tokenize("\"a ${\"n\"} b\"", 0);
    assert!(!diags.has_errors());
    let kinds: Vec<TokenKind> = tokens.iter().map(|t| t.kind).collect();
    assert!(kinds.contains(&TokenKind::StrPart));
    assert!(kinds.contains(&TokenKind::InterpClose));
}

#[test]
fn errors_on_unterminated_interpolation() {
    let (tokens, diags) = tokenize("\"open ${x", 0);
    assert!(diags.has_errors());
    assert_eq!(*kinds_last(&tokens), TokenKind::Eof);
}

/// Return the kind of the final token in the stream.
fn kinds_last(tokens: &[axolc_core::lexer::Token]) -> &TokenKind {
    &tokens[tokens.len() - 1].kind
}

#[test]
fn token_count_helper_works() {
    assert!(token_count("let x = 1") > 0);
}

#[test]
fn lexes_multi_line_block_comment() {
    let src = "--[[\nline one\nline two\n]]\nlet x = 1";
    let (tokens, _) = tokenize(src, 0);
    assert_eq!(tokens[0].kind, TokenKind::KwLet);
}

#[test]
fn lexes_complex_program() {
    let src = r#"
fn main()
    let x = 42
    let s = "hello"
    if x > 40 then
        print(s)
    end
end
"#;
    let (tokens, diags) = tokenize(src, 0);
    assert!(!diags.has_errors());
    assert!(tokens.len() > 10);
}

#[test]
fn lexes_unicode_string() {
    let (tokens, _) = tokenize("\"cafél\"", 0);
    assert_eq!(tokens[0].kind, TokenKind::StrLit);
}

#[test]
fn lexes_empty_string() {
    let (tokens, _) = tokenize("\"\"", 0);
    assert_eq!(tokens[0].kind, TokenKind::StrLit);
    assert_eq!(tokens[0].text, "");
}

#[test]
fn lexes_string_with_special_escapes() {
    let (tokens, _) = tokenize("\"\\t\\r\\n\\\\\\\"\"", 0);
    assert_eq!(tokens[0].kind, TokenKind::StrLit);
}
