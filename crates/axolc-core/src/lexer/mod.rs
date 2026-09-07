// Owner: PascalElixir / axolrs (GitHub org)
// File: The Axolotl lexer - turns source text into a token stream.

pub mod token;

pub use token::{Token, TokenKind};

use crate::diag::{Diagnostic, Diagnostics};
use crate::span::Span;

/// The lexer reads characters from a source string and produces tokens.
pub struct Lexer<'a> {
    src: &'a str,
    bytes: &'a [u8],
    pos: usize,
    file_id: u32,
    diagnostics: Diagnostics,
    /// Tokens produced by a multi-token lex step, emitted before further scanning.
    pending: Vec<Token>,
    /// The quote character when currently inside an interpolated string.
    in_string: Option<u8>,
    /// Number of open `${` interpolations whose expression is being lexed.
    interp_depth: u32,
    /// Nested brace depth inside a string interpolation expression.
    interp_brace: u32,
}

impl<'a> Lexer<'a> {
    /// Construct a lexer over `src` with the given file id.
    pub fn new(src: &'a str, file_id: u32) -> Lexer<'a> {
        Lexer {
            src,
            bytes: src.as_bytes(),
            pos: 0,
            file_id,
            diagnostics: Diagnostics::new(),
            pending: Vec::new(),
            in_string: None,
            interp_depth: 0,
            interp_brace: 0,
        }
    }

    /// Consume the lexer and return the diagnostics accumulated during tokenization.
    pub fn into_diagnostics(self) -> Diagnostics {
        self.diagnostics
    }

    /// Tokenize the entire source, returning the token stream and diagnostics.
    pub fn tokenize(mut self) -> (Vec<Token>, Diagnostics) {
        let mut tokens = Vec::new();
        loop {
            if let Some(t) = self.pending.pop() {
                tokens.push(t);
                continue;
            }
            if self.in_string.is_some() {
                let start = self.pos;
                let t = self.lex_string_chunk(start);
                tokens.push(t);
                continue;
            }
            self.skip_trivia();
            if self.pos >= self.bytes.len() {
                if self.interp_depth > 0 {
                    self.diagnostics.push(Diagnostic::error(
                        "unterminated string literal",
                        Span::new(self.pos as u32, self.pos as u32, self.file_id),
                    ));
                }
                tokens.push(Token {
                    kind: TokenKind::Eof,
                    span: Span::new(self.pos as u32, self.pos as u32, self.file_id),
                    text: String::new(),
                });
                break;
            }
            let token = self.next_token();
            tokens.push(token);
        }
        let diags = std::mem::take(&mut self.diagnostics);
        (tokens, diags)
    }

    fn next_token(&mut self) -> Token {
        let start = self.pos;
        let c = self.bytes[self.pos];
        let kind = match c {
            b'(' => { self.pos += 1; TokenKind::LParen }
            b')' => { self.pos += 1; TokenKind::RParen }
            b'{' => {
                self.pos += 1;
                if self.interp_depth > 0 {
                    self.interp_brace += 1;
                }
                TokenKind::LBrace
            }
            b'}' => {
                self.pos += 1;
                if self.interp_depth > 0 && self.interp_brace == 0 {
                    self.interp_depth -= 1;
                    self.in_string = Some(b'"');
                    TokenKind::InterpClose
                } else {
                    if self.interp_depth > 0 {
                        self.interp_brace = self.interp_brace.saturating_sub(1);
                    }
                    TokenKind::RBrace
                }
            }
            b'[' => { self.pos += 1; TokenKind::LBracket }
            b']' => { self.pos += 1; TokenKind::RBracket }
            b',' => { self.pos += 1; TokenKind::Comma }
            b';' => { self.pos += 1; TokenKind::Semicolon }
            b'@' => { self.pos += 1; TokenKind::At }
            b'#' => { self.pos += 1; TokenKind::Hash }
            b'$' => { self.pos += 1; TokenKind::Dollar }
            b'~' => {
                self.pos += 1;
                if self.peek() == Some(b'=') { self.pos += 1; TokenKind::LuaNotEqual }
                else { TokenKind::Tilde }
            }
            b'?' => {
                self.pos += 1;
                match self.peek() {
                    Some(b'?') => { self.pos += 1; TokenKind::DoubleQuestion }
                    Some(b'.') => { self.pos += 1; TokenKind::QuestionDot }
                    _ => TokenKind::Question
                }
            }
            b'!' => {
                self.pos += 1;
                match self.peek() {
                    Some(b'!') => { self.pos += 1; TokenKind::DoubleBang }
                    Some(b'=') => { self.pos += 1; TokenKind::NotEqual }
                    _ => TokenKind::Bang
                }
            }
            b':' => {
                self.pos += 1;
                if self.peek() == Some(b':') { self.pos += 1; TokenKind::DoubleColon }
                else { TokenKind::Colon }
            }
            b'.' => {
                self.pos += 1;
                if self.peek() == Some(b'.') {
                    self.pos += 1;
                    if self.peek() == Some(b'.') { self.pos += 1; TokenKind::DotDotDot }
                    else { TokenKind::DotDot }
                } else {
                    TokenKind::Dot
                }
            }
            b'+' => {
                self.pos += 1;
                if self.peek() == Some(b'=') { self.pos += 1; TokenKind::PlusEqual }
                else { TokenKind::Plus }
            }
            b'-' => {
                self.pos += 1;
                if self.peek() == Some(b'>') { self.pos += 1; TokenKind::Arrow }
                else if self.peek() == Some(b'=') { self.pos += 1; TokenKind::MinusEqual }
                else { TokenKind::Minus }
            }
            b'*' => {
                self.pos += 1;
                if self.peek() == Some(b'=') { self.pos += 1; TokenKind::StarEqual }
                else { TokenKind::Star }
            }
            b'/' => {
                self.pos += 1;
                if self.peek() == Some(b'=') { self.pos += 1; TokenKind::SlashEqual }
                else { TokenKind::Slash }
            }
            b'%' => {
                self.pos += 1;
                if self.peek() == Some(b'=') { self.pos += 1; TokenKind::PercentEqual }
                else { TokenKind::Percent }
            }
            b'^' => {
                self.pos += 1;
                if self.peek() == Some(b'=') { self.pos += 1; TokenKind::CaretEqual }
                else { TokenKind::Caret }
            }
            b'&' => {
                self.pos += 1;
                if self.peek() == Some(b'=') { self.pos += 1; TokenKind::AmpersandEqual }
                else { TokenKind::Ampersand }
            }
            b'|' => {
                self.pos += 1;
                match self.peek() {
                    Some(b'=') => { self.pos += 1; TokenKind::PipeEqual }
                    Some(b'>') => { self.pos += 1; TokenKind::PipeForward }
                    _ => TokenKind::Pipe
                }
            }
            b'<' => {
                self.pos += 1;
                match self.peek() {
                    Some(b'=') => { self.pos += 1; TokenKind::LessEqual }
                    Some(b'<') => {
                        self.pos += 1;
                        if self.peek() == Some(b'=') { self.pos += 1; TokenKind::ShiftLeftEqual }
                        else { TokenKind::ShiftLeft }
                    }
                    Some(b'|') => { self.pos += 1; TokenKind::BackPipe }
                    _ => TokenKind::Less,
                }
            }
            b'>' => {
                self.pos += 1;
                match self.peek() {
                    Some(b'=') => { self.pos += 1; TokenKind::GreaterEqual }
                    Some(b'>') => {
                        self.pos += 1;
                        if self.peek() == Some(b'=') { self.pos += 1; TokenKind::ShiftRightEqual }
                        else { TokenKind::ShiftRight }
                    }
                    _ => TokenKind::Greater,
                }
            }
            b'=' => {
                self.pos += 1;
                if self.peek() == Some(b'=') { self.pos += 1; TokenKind::DoubleEqual }
                else if self.peek() == Some(b'>') { self.pos += 1; TokenKind::FatArrow }
                else { TokenKind::Equal }
            }
            b'"' => {
                self.pos += 1;
                self.in_string = Some(b'"');
                return self.lex_string_chunk(start);
            }
            b'\'' => return self.lex_char(start),
            b'`' => return self.lex_raw_string(start),
            b'0'..=b'9' => return self.lex_number(start),
            b'a'..=b'z' | b'A'..=b'Z' | b'_' => return self.lex_ident(start),
            other => {
                self.diagnostics.push(Diagnostic::error(
                    format!("unexpected character {:?}", other as char),
                    Span::new(start as u32, start as u32 + 1, self.file_id),
                ));
                self.pos += 1;
                TokenKind::Ident
            }
        };
        let end = self.pos;
        let text = self.src[start..end].to_string();
        Token { kind, span: Span::new(start as u32, end as u32, self.file_id), text }
    }

    fn lex_number(&mut self, start: usize) -> Token {
        while let Some(c) = self.peek() {
            match c {
                b'0'..=b'9' | b'_' => self.pos += 1,
                _ => break,
            }
        }
        let mut is_float = false;
        if self.peek() == Some(b'.') && self.peek2().map_or(false, |c| c.is_ascii_digit()) {
            is_float = true;
            self.pos += 1;
            while let Some(c) = self.peek() {
                match c {
                    b'0'..=b'9' | b'_' => self.pos += 1,
                    _ => break,
                }
            }
        }
        if matches!(self.peek(), Some(b'e') | Some(b'E')) {
            is_float = true;
            self.pos += 1;
            if matches!(self.peek(), Some(b'+') | Some(b'-')) { self.pos += 1; }
            while let Some(c) = self.peek() {
                match c {
                    b'0'..=b'9' | b'_' => self.pos += 1,
                    _ => break,
                }
            }
        }
        // Suffixes (number units like 4_KB, 16_ms, or rust-style 1u32)
        while let Some(c) = self.peek() {
            match c {
                b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'_' => self.pos += 1,
                _ => break,
            }
        }
        let end = self.pos;
        let text = self.src[start..end].to_string();
        let kind = if is_float { TokenKind::FloatLit } else { TokenKind::IntLit };
        Token { kind, span: Span::new(start as u32, end as u32, self.file_id), text }
    }

    /// Lex one chunk of a (possibly interpolated) string: a literal run ending at
    /// the closing quote (StrLit) or at an interpolation opening `${` (StrPart).
    fn lex_string_chunk(&mut self, start: usize) -> Token {
        let quote = self.in_string.unwrap_or(b'"');
        let mut out = String::new();
        while self.pos < self.bytes.len() {
            let c = self.bytes[self.pos];
            if c == b'$' && self.bytes.get(self.pos + 1).copied() == Some(b'{') {
                self.pos += 2;
                self.in_string = None;
                self.interp_depth += 1;
                self.interp_brace = 0;
                let open_span = Span::new((self.pos - 2) as u32, self.pos as u32, self.file_id);
                self.pending.push(Token {
                    kind: TokenKind::InterpOpen,
                    span: open_span,
                    text: String::new(),
                });
                let span = Span::new(start as u32, self.pos as u32, self.file_id);
                return Token { kind: TokenKind::StrPart, span, text: out };
            }
            if c == quote {
                self.pos += 1;
                self.in_string = None;
                let span = Span::new(start as u32, self.pos as u32, self.file_id);
                return Token { kind: TokenKind::StrLit, span, text: out };
            }
            if c == b'\\' {
                self.pos += 1;
                if let Some(esc) = self.peek() {
                    self.pos += 1;
                    match esc {
                        b'n' => out.push('\n'),
                        b'r' => out.push('\r'),
                        b't' => out.push('\t'),
                        b'\\' => out.push('\\'),
                        b'0' => out.push('\0'),
                        b'\'' => out.push('\''),
                        b'"' => out.push('"'),
                        b'x' => {
                            let mut val: u32 = 0;
                            for _ in 0..2 {
                                if let Some(h) = self.peek().and_then(|c| (c as char).to_digit(16)) {
                                    val = val * 16 + h;
                                    self.pos += 1;
                                }
                            }
                            if let Some(ch) = char::from_u32(val) { out.push(ch); }
                        }
                        b'u' => {
                            if self.peek() == Some(b'{') {
                                self.pos += 1;
                                let mut val: u32 = 0;
                                while let Some(h) = self.peek().and_then(|c| (c as char).to_digit(16)) {
                                    val = val * 16 + h;
                                    self.pos += 1;
                                }
                                if self.peek() == Some(b'}') { self.pos += 1; }
                                if let Some(ch) = char::from_u32(val) { out.push(ch); }
                            }
                        }
                        _ => out.push(esc as char),
                    }
                }
            } else {
                let ch_len = char_len_at(self.bytes, self.pos);
                out.push_str(&self.src[self.pos..self.pos + ch_len]);
                self.pos += ch_len;
            }
        }
        self.in_string = None;
        self.diagnostics.push(Diagnostic::error(
            "unterminated string literal",
            Span::new(start as u32, self.pos as u32, self.file_id),
        ));
        let span = Span::new(start as u32, self.pos as u32, self.file_id);
        Token { kind: TokenKind::StrLit, span, text: out }
    }

    fn lex_char(&mut self, start: usize) -> Token {
        self.pos += 1;
        let mut out = String::new();
        if self.pos < self.bytes.len() {
            let c = self.bytes[self.pos];
            if c == b'\\' {
                self.pos += 1;
                if let Some(esc) = self.peek() {
                    self.pos += 1;
                    match esc {
                        b'n' => out.push('\n'),
                        b'r' => out.push('\r'),
                        b't' => out.push('\t'),
                        b'\\' => out.push('\\'),
                        b'0' => out.push('\0'),
                        b'\'' => out.push('\''),
                        b'"' => out.push('"'),
                        _ => out.push(esc as char),
                    }
                }
            } else {
                let ch_len = char_len_at(self.bytes, self.pos);
                out.push_str(&self.src[self.pos..self.pos + ch_len]);
                self.pos += ch_len;
            }
        }
        if self.peek() == Some(b'\'') { self.pos += 1; }
        let span = Span::new(start as u32, self.pos as u32, self.file_id);
        Token { kind: TokenKind::CharLit, span, text: out }
    }

    fn lex_raw_string(&mut self, start: usize) -> Token {
        self.pos += 1;
        let inner_start = self.pos;
        while self.pos < self.bytes.len() && self.peek() != Some(b'`') {
            self.pos += 1;
        }
        let text = self.src[inner_start..self.pos].to_string();
        if self.peek() == Some(b'`') { self.pos += 1; }
        let span = Span::new(start as u32, self.pos as u32, self.file_id);
        Token { kind: TokenKind::RawStrLit, span, text }
    }

    fn lex_ident(&mut self, start: usize) -> Token {
        while let Some(c) = self.peek() {
            match c {
                b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'_' => self.pos += 1,
                _ => break,
            }
        }
        let end = self.pos;
        let text = &self.src[start..end];
        let kind = match text {
            "let" => TokenKind::KwLet,
            "var" => TokenKind::KwVar,
            "fn" => TokenKind::KwFn,
            "end" => TokenKind::KwEnd,
            "if" => TokenKind::KwIf,
            "then" => TokenKind::KwThen,
            "elseif" => TokenKind::KwElseif,
            "else" => TokenKind::KwElse,
            "while" => TokenKind::KwWhile,
            "do" => TokenKind::KwDo,
            "repeat" => TokenKind::KwRepeat,
            "until" => TokenKind::KwUntil,
            "for" => TokenKind::KwFor,
            "in" => TokenKind::KwIn,
            "break" => TokenKind::KwBreak,
            "continue" => TokenKind::KwContinue,
            "return" => TokenKind::KwReturn,
            "match" => TokenKind::KwMatch,
            "case" => TokenKind::KwCase,
            "struct" => TokenKind::KwStruct,
            "enum" => TokenKind::KwEnum,
            "interface" => TokenKind::KwInterface,
            "use" => TokenKind::KwUse,
            "pub" => TokenKind::KwPub,
            "const" => TokenKind::KwConst,
            "true" => TokenKind::KwTrue,
            "false" => TokenKind::KwFalse,
            "null" => TokenKind::KwNull,
            "and" => TokenKind::KwAnd,
            "or" => TokenKind::KwOr,
            "not" => TokenKind::KwNot,
            "spawn" => TokenKind::KwSpawn,
            "await" => TokenKind::KwAwait,
            "async" => TokenKind::KwAsync,
            "try" => TokenKind::KwTry,
            "catch" => TokenKind::KwCatch,
            "unsafe" => TokenKind::KwUnsafe,
            "move" => TokenKind::KwMove,
            "borrow" => TokenKind::KwBorrow,
            "type" => TokenKind::KwType,
            "where" => TokenKind::KwWhere,
            "as" => TokenKind::KwAs,
            "is" => TokenKind::KwIs,
            "extern" => TokenKind::KwExtern,
            "nil" => TokenKind::KwNil,
            "loop" => TokenKind::KwLoop,
            "Int" => TokenKind::KwInt,
            "UInt" => TokenKind::KwUInt,
            "Float" => TokenKind::KwFloat,
            "Double" => TokenKind::KwDouble,
            "Bool" => TokenKind::KwBool,
            "String" => TokenKind::KwString,
            "Char" => TokenKind::KwChar,
            "Byte" => TokenKind::KwByte,
            "I8" => TokenKind::KwI8,
            "I16" => TokenKind::KwI16,
            "I32" => TokenKind::KwI32,
            "I64" => TokenKind::KwI64,
            "I128" => TokenKind::KwI128,
            "U8" => TokenKind::KwU8,
            "U16" => TokenKind::KwU16,
            "U32" => TokenKind::KwU32,
            "U64" => TokenKind::KwU64,
            "U128" => TokenKind::KwU128,
            "F32" => TokenKind::KwF32,
            "F64" => TokenKind::KwF64,
            "self" | "Self" => TokenKind::KwSelfType,
            _ => TokenKind::Ident,
        };
        let text = text.to_string();
        Token { kind, span: Span::new(start as u32, end as u32, self.file_id), text }
    }

    fn skip_trivia(&mut self) {
        while self.pos < self.bytes.len() {
            let c = self.bytes[self.pos];
            match c {
                b' ' | b'\t' | b'\r' | b'\n' => { self.pos += 1; }
                b'-' if self.bytes.get(self.pos + 1) == Some(&b'-') => {
                    self.pos += 2;
                    if self.peek() == Some(b'[') && self.peek2() == Some(b'[') {
                        self.pos += 2;
                        self.skip_block_comment();
                    } else {
                        while self.pos < self.bytes.len() && self.bytes[self.pos] != b'\n' {
                            self.pos += 1;
                        }
                    }
                }
                _ => break,
            }
        }
    }

    fn skip_block_comment(&mut self) {
        while self.pos + 1 < self.bytes.len() {
            if self.bytes[self.pos] == b']' && self.bytes[self.pos + 1] == b']' {
                self.pos += 2;
                return;
            }
            self.pos += 1;
        }
        self.pos = self.bytes.len();
    }

    fn peek(&self) -> Option<u8> {
        self.bytes.get(self.pos).copied()
    }
    fn peek2(&self) -> Option<u8> {
        self.bytes.get(self.pos + 1).copied()
    }
}

/// Compute the UTF-8 byte length of the character starting at the given offset.
fn char_len_at(bytes: &[u8], pos: usize) -> usize {
    if pos >= bytes.len() { return 0; }
    let b = bytes[pos];
    if b < 0x80 { 1 }
    else if b < 0xC0 { 1 }
    else if b < 0xE0 { 2 }
    else if b < 0xF0 { 3 }
    else { 4 }
}

/// Convenience function: tokenize a source string with a default file id.
pub fn tokenize(src: &str, file_id: u32) -> (Vec<Token>, Diagnostics) {
    Lexer::new(src, file_id).tokenize()
}
