// Owner: PascalElixir / axolrs (GitHub org)
// File: crates/bucket/src/fmt.rs - the Shed formatter: canonical, idempotent, token-preserving .axol source formatting.

use crate::build::collect_axol_files;
use axolc_core::diag::Severity;
use axolc_core::lexer::token::{Token, TokenKind};
use axolc_core::span::Span;

/// Number of spaces per indentation level in Shed output.
const INDENT: &str = "    ";

/// A formatting failure with the source position where it was detected.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FmtError {
    pub message: String,
    pub span: Span,
}

/// One element of the comment-aware source stream: a real token or a comment.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Piece {
    Tok(Token),
    LineComment { text: String, start: u32 },
    BlockComment { text: String, start: u32 },
}

impl Piece {
    /// Return the source offset where this piece starts.
    pub fn start(&self) -> u32 {
        match self {
            Piece::Tok(t) => t.span.start,
            Piece::LineComment { start, .. } | Piece::BlockComment { start, .. } => *start,
        }
    }

    /// Return the token kind when this piece is a token, else None.
    pub fn kind(&self) -> Option<TokenKind> {
        match self {
            Piece::Tok(t) => Some(t.kind),
            _ => None,
        }
    }
}

/// Scan a source string into tokens plus comments, failing on lexer errors.
pub fn scan_pieces(src: &str, file_id: u32) -> Result<Vec<Piece>, FmtError> {
    let (tokens, diags) = axolc_core::tokenize(src, file_id);
    if let Some(d) = diags.items.iter().find(|d| d.severity == Severity::Error) {
        return Err(FmtError { message: d.message.clone(), span: d.span });
    }
    let mut pieces: Vec<Piece> = Vec::new();
    let mut cursor: u32 = 0;
    for tok in tokens {
        if tok.kind == TokenKind::Eof {
            break;
        }
        scan_gap(src, cursor, tok.span.start, &mut pieces);
        cursor = tok.span.end;
        pieces.push(Piece::Tok(tok));
    }
    scan_gap(src, cursor, src.len() as u32, &mut pieces);
    Ok(pieces)
}

/// Extract whitespace-separated comments from the byte range between two tokens.
fn scan_gap(src: &str, from: u32, to: u32, pieces: &mut Vec<Piece>) {
    let bytes = src.as_bytes();
    let mut i = from as usize;
    let end = to as usize;
    while i < end {
        let b = bytes[i];
        if b == b' ' || b == b'\t' || b == b'\r' || b == b'\n' {
            i += 1;
            continue;
        }
        if b == b'-' && i + 1 < end && bytes[i + 1] == b'-' {
            let start = i as u32;
            let rest = &src[i..end];
            if rest.starts_with("--[[") {
                let body_end = rest.find("]]").map(|p| i + p + 2).unwrap_or(end);
                let text = src[i..body_end].trim_end().to_string();
                pieces.push(Piece::BlockComment { text, start });
                i = body_end;
            } else {
                let line_end = rest.find('\n').map(|p| i + p).unwrap_or(end);
                let text = src[i..line_end].trim_end().to_string();
                pieces.push(Piece::LineComment { text, start });
                i = line_end;
            }
            continue;
        }
        i += 1;
    }
}

/// Format one Axolotl source string into the canonical Shed form.
pub fn format_source(src: &str) -> Result<String, FmtError> {
    let pieces = scan_pieces(src, 0)?;
    let mut r = Renderer::new(src, pieces);
    r.run();
    Ok(r.finish())
}

/// Format every .axol file in the project with Shed (the `bucket fmt` command).
pub fn fmt_cmd() {
    let dir = std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
    let files = collect_axol_files(&dir);
    if files.is_empty() {
        println!("Shed: no .axol files found");
        return;
    }
    let mut changed = 0;
    let mut failed = 0;
    for f in &files {
        let src = std::fs::read_to_string(f).unwrap_or_default();
        match format_source(&src) {
            Ok(out) => {
                if out != src && std::fs::write(f, &out).is_ok() {
                    changed += 1;
                }
            }
            Err(e) => {
                let (line, col) = line_col_of(&src, e.span.start as usize);
                eprintln!("{}:{}:{}: error: {}", f.display(), line, col, e.message);
                failed += 1;
            }
        }
    }
    println!("Shed: formatted {} files ({} changed)", files.len(), changed);
    if failed > 0 {
        std::process::exit(1);
    }
}

/// Compute the 1-based (line, char-column) of a byte offset.
pub fn line_col_of(src: &str, offset: usize) -> (usize, usize) {
    let mut offset = offset.min(src.len());
    while offset > 0 && !src.is_char_boundary(offset) {
        offset -= 1;
    }
    let mut line = 1usize;
    let mut line_start = 0usize;
    for (i, c) in src.char_indices() {
        if i >= offset {
            break;
        }
        if c == '\n' {
            line += 1;
            line_start = i + 1;
        }
    }
    let col = src[line_start..offset].chars().count() + 1;
    (line, col)
}

/// Compute the 1-based source line containing a byte offset.
fn line_of(src: &str, offset: usize) -> usize {
    line_col_of(src, offset).0
}

/// The Shed renderer: re-emits the token stream with canonical whitespace.
struct Renderer<'a> {
    src: &'a str,
    pieces: Vec<Piece>,
    pos: usize,
    lines: Vec<String>,
    cur: String,
    cur_indent: usize,
    pending_space: bool,
    last_char: Option<u8>,
    last_tok_end: Option<u32>,
}

impl<'a> Renderer<'a> {
    /// Construct a renderer over a scanned piece stream.
    fn new(src: &'a str, pieces: Vec<Piece>) -> Renderer<'a> {
        Renderer {
            src,
            pieces,
            pos: 0,
            lines: Vec::new(),
            cur: String::new(),
            cur_indent: 0,
            pending_space: false,
            last_char: None,
            last_tok_end: None,
        }
    }

    /// Render the whole module: comment-attached items separated by one blank line.
    fn run(&mut self) {
        let mut first = true;
        loop {
            if self.pos >= self.pieces.len() {
                break;
            }
            if !first {
                self.consume_trailing_comments();
                if self.tok_at(0).is_some() {
                    self.lines.push(String::new());
                }
            }
            first = false;
            self.render_comments_until_token();
            if self.tok_at(0).is_none() {
                break;
            }
            self.fmt_item();
        }
    }

    /// Glue same-line comments onto the previously emitted item's last line.
    fn consume_trailing_comments(&mut self) {
        while self.next_comment_is_trailing() {
            let p = self.pieces[self.pos].clone();
            self.pos += 1;
            let suffix = format!("  {}", comment_text(&p));
            if let Some(last) = self.lines.last_mut() {
                last.push_str(&suffix);
            }
        }
    }

    /// Report whether the next piece is a line comment on the last token's line.
    fn next_comment_is_trailing(&self) -> bool {
        match self.pieces.get(self.pos) {
            Some(Piece::LineComment { start, .. }) => self
                .last_tok_end
                .map_or(false, |e| line_of(self.src, e as usize) == line_of(self.src, *start as usize)),
            _ => false,
        }
    }

    /// Consume comment pieces in front of the next token, rendering each and preserving blank separators.
    fn render_comments_until_token(&mut self) {
        while let Some(p) = self.pieces.get(self.pos) {
            if matches!(p, Piece::Tok(_)) {
                break;
            }
            let p = p.clone();
            self.pos += 1;
            self.render_comment(&p);
            if self.gap_has_blank(&p, self.pieces.get(self.pos)) {
                self.flush();
                self.lines.push(String::new());
            }
        }
    }

    /// Report whether the source gap between two adjacent pieces contains a blank line.
    fn gap_has_blank(&self, prev: &Piece, next: Option<&Piece>) -> bool {
        match next {
            Some(n) => {
                let a = piece_end(prev);
                let b = n.start() as usize;
                a < b && b <= self.src.len() && self.src[a..b].matches('\n').count() >= 2
            }
            None => false,
        }
    }

    /// Render one comment: trailing glues to the previous line, leading gets its own.
    fn render_comment(&mut self, p: &Piece) {
        let text = comment_text(p);
        match p {
            Piece::BlockComment { start, .. } => {
                let trailing = self.last_tok_end.map_or(false, |e| {
                    line_of(self.src, e as usize) == line_of(self.src, *start as usize)
                });
                if trailing {
                    let suffix = format!("  {}", text);
                    if self.cur.is_empty() {
                        if let Some(last) = self.lines.last_mut() {
                            last.push_str(&suffix);
                        }
                    } else {
                        self.cur.push_str(&suffix);
                        self.flush();
                    }
                } else if self.cur.is_empty() {
                    self.cur.push_str(&text);
                    self.flush();
                } else {
                    self.push_head(&text);
                    self.mark_space();
                }
            }
            Piece::LineComment { start, .. } => {
                let trailing = self.last_tok_end.map_or(false, |e| {
                    line_of(self.src, e as usize) == line_of(self.src, *start as usize)
                });
                if trailing {
                    let suffix = format!("  {}", text);
                    if self.cur.is_empty() {
                        if let Some(last) = self.lines.last_mut() {
                            last.push_str(&suffix);
                        }
                    } else {
                        self.cur.push_str(&suffix);
                        self.flush();
                    }
                } else {
                    self.flush();
                    self.cur.push_str(&text);
                    self.flush();
                }
            }
            Piece::Tok(_) => {}
        }
    }

    /// Return the final output with exactly one trailing newline.
    fn finish(mut self) -> String {
        self.flush();
        while self.lines.last().map_or(false, |l| l.is_empty()) {
            self.lines.pop();
        }
        let mut out = String::new();
        for line in &self.lines {
            out.push_str(line);
            out.push('\n');
        }
        out
    }

    /// Flush the current line into the output lines.
    fn flush(&mut self) {
        let trimmed = self.cur.trim_end().to_string();
        if !trimmed.is_empty() {
            let mut line = String::new();
            for _ in 0..self.cur_indent {
                line.push_str(INDENT);
            }
            line.push_str(&trimmed);
            self.lines.push(line);
        }
        self.cur.clear();
        self.pending_space = false;
        self.last_char = None;
    }

    /// Begin a fresh output line at the given indent level.
    fn start_line(&mut self, indent: usize) {
        self.flush();
        self.cur_indent = indent;
    }

    /// Push raw text with optional preceding space, honoring the merge guard.
    fn push(&mut self, text: &str, space_before: bool) {
        if text.is_empty() {
            return;
        }
        let guard = needs_space_guard(self.last_char, text.as_bytes()[0]);
        let want_space = (self.pending_space || space_before || guard) && !self.cur.is_empty();
        if want_space {
            self.cur.push(' ');
        }
        self.cur.push_str(text);
        self.last_char = text.as_bytes().last().copied();
        self.pending_space = false;
    }

    /// Push raw text directly against the previous content.
    fn push_tight(&mut self, text: &str) {
        self.push(text, false);
    }

    /// Push raw text with one space before it.
    fn push_head(&mut self, text: &str) {
        self.push(text, true);
    }

    /// Push raw text spaced on both sides (operators, keywords).
    fn push_op(&mut self, text: &str) {
        self.push(text, true);
        self.pending_space = true;
    }

    /// Require one space before the next pushed text.
    fn mark_space(&mut self) {
        self.pending_space = true;
    }

    /// Consume a separator token (comma or semicolon) tightly, spacing what follows.
    fn separator(&mut self) {
        let t = self.bump();
        let text = self.raw(&t);
        self.cur.push_str(&text);
        self.last_char = text.as_bytes().last().copied();
        self.mark_space();
    }

    /// Push a closing bracket, swallowing any pending separator space.
    fn push_close(&mut self, text: &str) {
        self.pending_space = false;
        self.push(text, false);
    }

    /// Return the rendered text of a token from its source span.
    fn raw(&self, tok: &Token) -> String {
        if tok.kind == TokenKind::StrPart {
            let end = tok.span.end.saturating_sub(2).max(tok.span.start);
            return self.src[tok.span.start as usize..end as usize].to_string();
        }
        if tok.kind == TokenKind::InterpOpen {
            return "${".to_string();
        }
        if tok.kind == TokenKind::InterpClose {
            return "}".to_string();
        }
        self.src[tok.span.start as usize..tok.span.end as usize].to_string()
    }

    /// Consume and render the next token, rendering comments in front of it.
    fn bump(&mut self) -> Token {
        loop {
            match self.pieces.get(self.pos) {
                Some(Piece::Tok(t)) => {
                    let tok = t.clone();
                    self.pos += 1;
                    self.last_tok_end = Some(tok.span.end);
                    return tok;
                }
                Some(p) => {
                    let p = p.clone();
                    self.pos += 1;
                    self.render_comment(&p);
                }
                None => return self.eof_token(),
            }
        }
    }

    /// Fabricate an end-of-file token so malformed streams always terminate.
    fn eof_token(&self) -> Token {
        Token {
            kind: TokenKind::Eof,
            span: Span::new(self.src.len() as u32, self.src.len() as u32, 0),
            text: String::new(),
        }
    }

    /// Consume the leading keyword of a line, pushing it with a trailing space.
    fn lead_kw(&mut self, indent: usize) {
        self.start_line(indent);
        let t = self.bump();
        let text = self.raw(&t);
        self.push_tight(&text);
        self.mark_space();
    }

    /// Return the kind of the next real token without consuming anything.
    fn peek(&self) -> TokenKind {
        self.tok_at(0).unwrap_or(TokenKind::Eof)
    }

    /// Return the kind of the nth real token ahead, skipping comments.
    fn tok_at(&self, n: usize) -> Option<TokenKind> {
        let mut seen = 0;
        let mut i = self.pos;
        while i < self.pieces.len() {
            if let Piece::Tok(t) = &self.pieces[i] {
                if seen == n {
                    return Some(t.kind);
                }
                seen += 1;
            }
            i += 1;
        }
        None
    }

    /// Render one top-level item beginning at the current position.
    fn fmt_item(&mut self) {
        self.start_line(0);
        while self.peek() == TokenKind::At {
            self.fmt_attribute();
            self.start_line(0);
        }
        if self.peek() == TokenKind::KwPub {
            let t = self.bump();
            let text = self.raw(&t);
            self.push_tight(&text);
            self.mark_space();
        }
        if self.peek() == TokenKind::KwAsync && self.tok_at(1) == Some(TokenKind::KwFn) {
            let t = self.bump();
            let text = self.raw(&t);
            self.push_tight(&text);
            self.mark_space();
        }
        match self.peek() {
            TokenKind::KwUse => self.fmt_use(),
            TokenKind::KwFn => self.fmt_fn_head(),
            TokenKind::KwStruct => self.fmt_prefix_struct(),
            TokenKind::KwEnum => self.fmt_prefix_enum(),
            TokenKind::KwInterface => self.fmt_prefix_interface(),
            TokenKind::KwConst => self.fmt_const(),
            TokenKind::KwType => self.fmt_type_alias(),
            TokenKind::KwExtern => self.fmt_extern(),
            TokenKind::Ident => self.fmt_named_decl(),
            _ => self.fmt_garbage_line(),
        }
    }

    /// Render a `use a::b::c` item on one line.
    fn fmt_use(&mut self) {
        let t = self.bump();
        let text = self.raw(&t);
        self.push_tight(&text);
        self.mark_space();
        loop {
            match self.peek() {
                TokenKind::Ident | TokenKind::StrLit => {
                    let t = self.bump();
                    let text = self.raw(&t);
                    self.push_tight(&text);
                }
                TokenKind::DoubleColon => {
                    self.bump();
                    self.push_tight("::");
                }
                _ => break,
            }
        }
        self.flush();
    }

    /// Render `const NAME: T = value`.
    fn fmt_const(&mut self) {
        let t = self.bump();
        let text = self.raw(&t);
        self.push_tight(&text);
        self.mark_space();
        if self.peek() == TokenKind::Ident {
            let t = self.bump();
            let text = self.raw(&t);
            self.push_tight(&text);
        }
        if self.peek() == TokenKind::Colon {
            self.bump();
            self.push_tight(":");
            self.mark_space();
            self.fmt_type();
        }
        if self.peek() == TokenKind::Equal {
            self.bump();
            self.push_op("=");
            self.fmt_expr();
        }
        self.eat_trailing_semi();
        self.flush();
    }

    /// Render `type Name<A, B> = Type`.
    fn fmt_type_alias(&mut self) {
        let t = self.bump();
        let text = self.raw(&t);
        self.push_tight(&text);
        self.mark_space();
        if self.peek() == TokenKind::Ident {
            let t = self.bump();
            let text = self.raw(&t);
            self.push_tight(&text);
        }
        if self.peek() == TokenKind::Less {
            self.fmt_generics();
        }
        if self.peek() == TokenKind::Equal {
            self.bump();
            self.push_op("=");
            self.fmt_type();
        }
        self.flush();
    }

    /// Render `extern "ABI"` or bare `extern`.
    fn fmt_extern(&mut self) {
        let t = self.bump();
        let text = self.raw(&t);
        self.push_tight(&text);
        self.mark_space();
        if self.peek() == TokenKind::StrLit {
            let t = self.bump();
            let text = self.raw(&t);
            self.push_tight(&text);
        }
        self.flush();
    }

    /// Render an `@attr ...` line attached to the item that follows it.
    fn fmt_attribute(&mut self) {
        self.bump();
        self.push_tight("@");
        if self.peek() == TokenKind::Ident {
            let t = self.bump();
            let text = self.raw(&t);
            self.push_tight(&text);
        }
        if self.peek() == TokenKind::LParen {
            self.fmt_call_args();
            self.flush();
            return;
        }
        while let Some(k) = self.tok_at(0) {
            if matches!(
                k,
                TokenKind::KwFn
                    | TokenKind::KwStruct
                    | TokenKind::KwEnum
                    | TokenKind::KwInterface
                    | TokenKind::KwConst
                    | TokenKind::KwType
                    | TokenKind::KwUse
                    | TokenKind::KwExtern
                    | TokenKind::KwPub
                    | TokenKind::KwAsync
                    | TokenKind::At
                    | TokenKind::Eof
            ) {
                break;
            }
            let t = self.bump();
            let text = self.raw(&t);
            if !text.is_empty() {
                self.push_head(&text);
            }
        }
        self.flush();
    }

    /// Render `struct Name ... end` (the prefix form).
    fn fmt_prefix_struct(&mut self) {
        let t = self.bump();
        let text = self.raw(&t);
        self.push_tight(&text);
        self.mark_space();
        if self.peek() == TokenKind::Ident {
            let t = self.bump();
            let text = self.raw(&t);
            self.push_tight(&text);
        }
        self.fmt_struct_tail();
    }

    /// Render `Name = struct ... end` (the canonical struct form).
    fn fmt_struct(&mut self) {
        let t = self.bump();
        let text = self.raw(&t);
        self.push_tight(&text);
        self.bump();
        self.push_op("=");
        let t = self.bump();
        let text = self.raw(&t);
        self.push_tight(&text);
        self.fmt_struct_tail();
    }

    /// Render generics plus one-per-line fields plus `end`.
    fn fmt_struct_tail(&mut self) {
        if self.peek() == TokenKind::Less {
            self.fmt_generics();
        }
        self.flush();
        while !matches!(self.peek(), TokenKind::KwEnd | TokenKind::Eof) {
            if self.peek() == TokenKind::Comma {
                self.start_line(1);
                self.separator();
                self.flush();
                continue;
            }
            self.start_line(1);
            if self.peek() == TokenKind::KwPub {
                let t = self.bump();
                let text = self.raw(&t);
                self.push_tight(&text);
                self.mark_space();
            }
            if self.peek() == TokenKind::Ident {
                let t = self.bump();
                let text = self.raw(&t);
                self.push_tight(&text);
            }
            if self.peek() == TokenKind::Colon {
                self.bump();
                self.push_tight(":");
                self.mark_space();
                self.fmt_type();
            }
            if self.peek() == TokenKind::Equal {
                self.bump();
                self.push_op("=");
                self.fmt_expr();
            }
            if self.peek() == TokenKind::Comma {
                self.separator();
            }
            self.flush();
        }
        self.render_comments_until_token();
        if self.peek() == TokenKind::KwEnd {
            self.lead_kw(0);
            self.flush();
        }
    }

    /// Render `enum Name ... end` (the prefix form).
    fn fmt_prefix_enum(&mut self) {
        let t = self.bump();
        let text = self.raw(&t);
        self.push_tight(&text);
        self.mark_space();
        if self.peek() == TokenKind::Ident {
            let t = self.bump();
            let text = self.raw(&t);
            self.push_tight(&text);
        }
        self.fmt_enum_tail();
    }

    /// Render `Name = enum ... end`.
    fn fmt_enum(&mut self) {
        let t = self.bump();
        let text = self.raw(&t);
        self.push_tight(&text);
        self.bump();
        self.push_op("=");
        let t = self.bump();
        let text = self.raw(&t);
        self.push_tight(&text);
        self.fmt_enum_tail();
    }

    /// Render enum variants one per line plus `end`.
    fn fmt_enum_tail(&mut self) {
        if self.peek() == TokenKind::Less {
            self.fmt_generics();
        }
        self.flush();
        while !matches!(self.peek(), TokenKind::KwEnd | TokenKind::Eof) {
            if self.peek() == TokenKind::Comma {
                self.start_line(1);
                self.separator();
                self.flush();
                continue;
            }
            if self.peek() == TokenKind::Ident {
                self.start_line(1);
                let t = self.bump();
                let text = self.raw(&t);
                self.push_tight(&text);
                if self.peek() == TokenKind::LParen {
                    self.fmt_enum_variant_fields();
                }
                if self.peek() == TokenKind::Comma {
                    self.separator();
                }
                self.flush();
            } else {
                self.fmt_garbage_line();
            }
        }
        self.render_comments_until_token();
        if self.peek() == TokenKind::KwEnd {
            self.lead_kw(0);
            self.flush();
        }
    }

    /// Render the `(a: Int, b: String)` field list of an enum variant.
    fn fmt_enum_variant_fields(&mut self) {
        self.bump();
        self.push_tight("(");
        loop {
            match self.peek() {
                TokenKind::RParen | TokenKind::Eof => break,
                TokenKind::Comma => self.separator(),
                TokenKind::Ident if self.tok_at(1) == Some(TokenKind::Colon) => {
                    let t = self.bump();
                    let text = self.raw(&t);
                    self.push_tight(&text);
                    self.bump();
                    self.push_tight(":");
                    self.mark_space();
                    self.fmt_type();
                }
                _ => self.fmt_type(),
            }
        }
        if self.peek() == TokenKind::RParen {
            self.bump();
            self.push_close(")");
        }
    }

    /// Render `interface Name ... end` (the prefix form).
    fn fmt_prefix_interface(&mut self) {
        let t = self.bump();
        let text = self.raw(&t);
        self.push_tight(&text);
        self.mark_space();
        if self.peek() == TokenKind::Ident {
            let t = self.bump();
            let text = self.raw(&t);
            self.push_tight(&text);
        }
        self.fmt_interface_tail();
    }

    /// Render `Name = interface ... end`.
    fn fmt_interface(&mut self) {
        let t = self.bump();
        let text = self.raw(&t);
        self.push_tight(&text);
        self.bump();
        self.push_op("=");
        let t = self.bump();
        let text = self.raw(&t);
        self.push_tight(&text);
        self.fmt_interface_tail();
    }

    /// Render interface method signatures one per line plus `end`.
    fn fmt_interface_tail(&mut self) {
        if self.peek() == TokenKind::Less {
            self.fmt_generics();
        }
        self.flush();
        while !matches!(self.peek(), TokenKind::KwEnd | TokenKind::Eof) {
            if self.peek() == TokenKind::Ident {
                self.start_line(1);
                self.fmt_method_sig();
                self.flush();
            } else {
                self.fmt_garbage_line();
            }
        }
        self.render_comments_until_token();
        if self.peek() == TokenKind::KwEnd {
            self.lead_kw(0);
            self.flush();
        }
    }

    /// Render `name(params) -> ret` on the current line.
    fn fmt_method_sig(&mut self) {
        let t = self.bump();
        let text = self.raw(&t);
        self.push_tight(&text);
        if self.peek() == TokenKind::LParen {
            self.fmt_params();
        }
        if self.peek() == TokenKind::Arrow {
            self.bump();
            self.push_op("->");
            self.fmt_type();
        }
    }

    /// Render `Name: Trait` (impl marker) on one line.
    fn fmt_impl(&mut self) {
        let t = self.bump();
        let text = self.raw(&t);
        self.push_tight(&text);
        self.bump();
        self.push_tight(":");
        self.mark_space();
        if self.peek() == TokenKind::Ident {
            let t = self.bump();
            let text = self.raw(&t);
            self.push_tight(&text);
        }
        self.flush();
    }

    /// Render an identifier-led declaration: method, static method, struct, enum, interface, impl.
    fn fmt_named_decl(&mut self) {
        match self.tok_at(1) {
            Some(TokenKind::Colon) => self.fmt_impl(),
            Some(TokenKind::Dot) => self.fmt_method_decl(),
            Some(TokenKind::Equal) => match self.tok_at(2) {
                Some(TokenKind::KwFn) => self.fmt_static_method(),
                Some(TokenKind::KwStruct) => self.fmt_struct(),
                Some(TokenKind::KwEnum) => self.fmt_enum(),
                Some(TokenKind::KwInterface) => self.fmt_interface(),
                _ => self.fmt_garbage_line(),
            },
            _ => self.fmt_garbage_line(),
        }
    }

    /// Render `Recv.name = fn(params) -> ret body end` or `Recv.name(params) -> ret body end`.
    fn fmt_method_decl(&mut self) {
        let t = self.bump();
        let text = self.raw(&t);
        self.push_tight(&text);
        self.bump();
        self.push_tight(".");
        if self.peek() == TokenKind::Ident {
            let t = self.bump();
            let text = self.raw(&t);
            self.push_tight(&text);
        }
        if self.peek() == TokenKind::Equal {
            self.bump();
            self.push_op("=");
            if self.peek() == TokenKind::KwFn {
                let t = self.bump();
                let text = self.raw(&t);
                self.push_tight(&text);
            }
        }
        self.fmt_fn_body();
    }

    /// Render `Recv = fn name(params) -> ret body end`.
    fn fmt_static_method(&mut self) {
        let t = self.bump();
        let text = self.raw(&t);
        self.push_tight(&text);
        self.bump();
        self.push_op("=");
        let t = self.bump();
        let text = self.raw(&t);
        self.push_tight(&text);
        if self.peek() == TokenKind::Ident {
            self.mark_space();
            let t = self.bump();
            let text = self.raw(&t);
            self.push_tight(&text);
        }
        self.fmt_fn_body();
    }

    /// Render the `fn` keyword then the params, return type, and body.
    fn fmt_fn_head(&mut self) {
        let t = self.bump();
        let text = self.raw(&t);
        self.push_tight(&text);
        self.mark_space();
        if self.peek() == TokenKind::Ident {
            let t = self.bump();
            let text = self.raw(&t);
            self.push_tight(&text);
        }
        self.fmt_fn_body();
    }

    /// Render the params, return type, and body of a function or method.
    fn fmt_fn_body(&mut self) {
        if self.peek() == TokenKind::Less {
            self.fmt_generics();
        }
        if self.peek() == TokenKind::LParen {
            self.fmt_params();
        }
        if self.peek() == TokenKind::Arrow {
            self.bump();
            self.push_op("->");
            self.fmt_type();
        }
        if self.peek() == TokenKind::Semicolon {
            self.bump();
            self.cur.push(';');
            self.flush();
            return;
        }
        self.flush();
        self.fmt_construct_body(0);
    }

    /// Render statement lines at indent+1 and the closing `end` at indent.
    fn fmt_construct_body(&mut self, indent: usize) {
        self.fmt_block(indent + 1);
        self.render_comments_until_token();
        if self.peek() == TokenKind::KwEnd {
            self.lead_kw(indent);
            self.flush();
        }
    }

    /// Render a `<A, B>` generic parameter list.
    fn fmt_generics(&mut self) {
        self.bump();
        self.push_tight("<");
        loop {
            match self.peek() {
                TokenKind::Greater | TokenKind::Eof => break,
                TokenKind::Comma => self.separator(),
                TokenKind::Ident => {
                    let t = self.bump();
                    let text = self.raw(&t);
                    self.push_tight(&text);
                }
                _ => {
                    self.bump();
                }
            }
        }
        if self.peek() == TokenKind::Greater {
            self.bump();
            self.push_close(">");
        }
    }

    /// Render a parameter list `(a: T = default, ...rest)`.
    fn fmt_params(&mut self) {
        self.bump();
        self.push_tight("(");
        loop {
            match self.peek() {
                TokenKind::RParen | TokenKind::Eof => break,
                TokenKind::Comma => self.separator(),
                TokenKind::DotDotDot => {
                    self.bump();
                    self.push_tight("...");
                }
                TokenKind::Ident | TokenKind::KwSelfType => {
                    let t = self.bump();
                    let text = self.raw(&t);
                    self.push_tight(&text);
                    if self.peek() == TokenKind::Colon {
                        self.bump();
                        self.push_tight(":");
                        self.mark_space();
                        self.fmt_type();
                    }
                    if self.peek() == TokenKind::Equal {
                        self.bump();
                        self.push_op("=");
                        self.fmt_expr();
                    }
                }
                _ => {
                    let t = self.bump();
                    let text = self.raw(&t);
                    if !text.is_empty() {
                        self.push_head(&text);
                    }
                }
            }
        }
        if self.peek() == TokenKind::RParen {
            self.bump();
            self.push_close(")");
        }
    }

    /// Render statements until a block terminator, one per line.
    fn fmt_block(&mut self, indent: usize) {
        loop {
            match self.peek() {
                TokenKind::KwEnd
                | TokenKind::KwElse
                | TokenKind::KwElseif
                | TokenKind::KwUntil
                | TokenKind::Eof => return,
                _ => {}
            }
            self.fmt_stmt(indent);
            if self.tok_at(0).is_none() {
                return;
            }
        }
    }

    /// Render one statement at the given indent.
    fn fmt_stmt(&mut self, indent: usize) {
        match self.peek() {
            TokenKind::KwLet | TokenKind::KwVar => self.fmt_let(indent),
            TokenKind::KwIf => self.fmt_if(indent),
            TokenKind::KwWhile => self.fmt_while(indent),
            TokenKind::KwFor => self.fmt_for(indent),
            TokenKind::KwRepeat => self.fmt_repeat(indent),
            TokenKind::KwLoop => {
                self.lead_kw(indent);
                self.flush();
                self.fmt_construct_body(indent);
            }
            TokenKind::KwMatch => self.fmt_match(indent),
            TokenKind::KwBreak | TokenKind::KwContinue => {
                self.lead_kw(indent);
                self.eat_trailing_semi();
                self.flush();
            }
            TokenKind::KwReturn => self.fmt_return(indent),
            TokenKind::KwSpawn => {
                self.lead_kw(indent);
                self.fmt_expr();
                self.eat_trailing_semi();
                self.flush();
            }
            TokenKind::KwUnsafe => {
                self.lead_kw(indent);
                self.flush();
                self.fmt_construct_body(indent);
            }
            _ => self.fmt_expr_stmt(indent),
        }
    }

    /// Render `let`/`var name: T = value`.
    fn fmt_let(&mut self, indent: usize) {
        self.lead_kw(indent);
        if self.peek() == TokenKind::Ident {
            let t = self.bump();
            let text = self.raw(&t);
            self.push_tight(&text);
        }
        if self.peek() == TokenKind::Colon {
            self.bump();
            self.push_tight(":");
            self.mark_space();
            self.fmt_type();
        }
        if self.peek() == TokenKind::Equal {
            self.bump();
            self.push_op("=");
            self.fmt_expr();
        }
        self.eat_trailing_semi();
        self.flush();
    }

    /// Render an expression or assignment statement.
    fn fmt_expr_stmt(&mut self, indent: usize) {
        self.start_line(indent);
        self.fmt_expr();
        if self.is_assign_op(self.peek()) {
            let t = self.bump();
            let text = self.raw(&t);
            self.push_op(&text);
            self.fmt_expr();
        }
        self.eat_trailing_semi();
        self.flush();
    }

    /// Consume a trailing statement semicolon onto the current line.
    fn eat_trailing_semi(&mut self) {
        if self.peek() == TokenKind::Semicolon {
            let t = self.bump();
            let text = self.raw(&t);
            self.cur.push_str(&text);
        }
    }

    /// Render `if cond then ... elseif ... else ... end`.
    fn fmt_if(&mut self, indent: usize) {
        self.lead_kw(indent);
        self.fmt_expr();
        if self.peek() == TokenKind::KwThen {
            let t = self.bump();
            let text = self.raw(&t);
            self.push_op(&text);
        }
        self.flush();
        self.fmt_block(indent + 1);
        loop {
            self.render_comments_until_token();
            match self.peek() {
                TokenKind::KwElseif => {
                    self.lead_kw(indent);
                    self.fmt_expr();
                    if self.peek() == TokenKind::KwThen {
                        let t = self.bump();
                        let text = self.raw(&t);
                        self.push_op(&text);
                    }
                    self.flush();
                    self.fmt_block(indent + 1);
                }
                TokenKind::KwElse => {
                    self.lead_kw(indent);
                    self.flush();
                    self.fmt_block(indent + 1);
                    break;
                }
                _ => break,
            }
        }
        self.render_comments_until_token();
        if self.peek() == TokenKind::KwEnd {
            self.lead_kw(indent);
            self.flush();
        }
    }

    /// Render `while cond do ... end`.
    fn fmt_while(&mut self, indent: usize) {
        self.lead_kw(indent);
        self.fmt_expr();
        if self.peek() == TokenKind::KwDo {
            let t = self.bump();
            let text = self.raw(&t);
            self.push_op(&text);
        }
        self.flush();
        self.fmt_construct_body(indent);
    }

    /// Render `for var in iter do ... end`.
    fn fmt_for(&mut self, indent: usize) {
        self.lead_kw(indent);
        if self.peek() == TokenKind::Ident {
            let t = self.bump();
            let text = self.raw(&t);
            self.push_tight(&text);
        }
        if self.peek() == TokenKind::KwIn {
            self.bump();
            self.push_op("in");
        }
        self.fmt_expr();
        if self.peek() == TokenKind::KwDo {
            let t = self.bump();
            let text = self.raw(&t);
            self.push_op(&text);
        }
        self.flush();
        self.fmt_construct_body(indent);
    }

    /// Render `repeat ... until cond`.
    fn fmt_repeat(&mut self, indent: usize) {
        self.lead_kw(indent);
        self.flush();
        self.fmt_block(indent + 1);
        self.render_comments_until_token();
        if self.peek() == TokenKind::KwUntil {
            self.lead_kw(indent);
            self.fmt_expr();
            self.flush();
        }
    }

    /// Render `return` or `return expr`.
    fn fmt_return(&mut self, indent: usize) {
        self.lead_kw(indent);
        if !matches!(
            self.peek(),
            TokenKind::KwEnd
                | TokenKind::KwElse
                | TokenKind::KwElseif
                | TokenKind::KwUntil
                | TokenKind::Eof
                | TokenKind::KwLet
                | TokenKind::KwVar
                | TokenKind::KwIf
                | TokenKind::KwWhile
                | TokenKind::KwFor
                | TokenKind::KwLoop
                | TokenKind::KwMatch
                | TokenKind::KwRepeat
                | TokenKind::KwBreak
                | TokenKind::KwContinue
                | TokenKind::KwUnsafe
        ) {
            self.fmt_expr();
        }
        self.eat_trailing_semi();
        self.flush();
    }

    /// Render `match scrutinee` with one arm per line and `end`.
    fn fmt_match(&mut self, indent: usize) {
        self.lead_kw(indent);
        self.fmt_expr();
        self.flush();
        while !matches!(self.peek(), TokenKind::KwEnd | TokenKind::Eof) {
            self.fmt_match_arm(indent + 1);
        }
        self.render_comments_until_token();
        if self.peek() == TokenKind::KwEnd {
            self.lead_kw(indent);
            self.flush();
        }
    }

    /// Render one `pattern => body` match arm.
    fn fmt_match_arm(&mut self, indent: usize) {
        self.start_line(indent);
        self.fmt_pattern();
        if self.peek() == TokenKind::KwIf {
            let t = self.bump();
            let text = self.raw(&t);
            self.push_op(&text);
            self.fmt_expr();
        }
        if self.peek() == TokenKind::FatArrow {
            let t = self.bump();
            let text = self.raw(&t);
            self.push_op(&text);
        }
        if self.peek() == TokenKind::KwDo {
            let t = self.bump();
            let text = self.raw(&t);
            self.push_op(&text);
            self.flush();
            self.fmt_construct_body(indent);
            return;
        }
        if !matches!(self.peek(), TokenKind::KwEnd | TokenKind::Eof) {
            self.fmt_expr();
            if self.peek() == TokenKind::Comma {
                self.separator();
            }
        }
        self.flush();
    }

    /// Render a match pattern: alternatives, bindings, literals, destructuring.
    fn fmt_pattern(&mut self) {
        self.fmt_pattern_atom();
        while self.peek() == TokenKind::Pipe {
            let t = self.bump();
            let text = self.raw(&t);
            self.push_op(&text);
            self.fmt_pattern_atom();
        }
        if self.peek() == TokenKind::KwAs {
            let t = self.bump();
            let text = self.raw(&t);
            self.push_op(&text);
            if self.peek() == TokenKind::Ident {
                let t = self.bump();
                let text = self.raw(&t);
                self.push_tight(&text);
            }
        }
    }

    /// Render one pattern atom.
    fn fmt_pattern_atom(&mut self) {
        match self.peek() {
            TokenKind::Ident => {
                let t = self.bump();
                let text = self.raw(&t);
                self.push_tight(&text);
                if self.peek() == TokenKind::Dot {
                    self.bump();
                    self.push_tight(".");
                    if self.peek() == TokenKind::Ident {
                        let t = self.bump();
                        let text = self.raw(&t);
                        self.push_tight(&text);
                    }
                    if self.peek() == TokenKind::LParen {
                        self.fmt_pattern_subpats();
                    }
                } else if self.peek() == TokenKind::LBrace {
                    self.fmt_pattern_fields();
                } else if self.peek() == TokenKind::LParen {
                    self.fmt_pattern_subpats();
                }
            }
            TokenKind::LBracket => {
                self.bump();
                self.push_tight("[");
                self.fmt_pattern_list(TokenKind::RBracket);
                if self.peek() == TokenKind::RBracket {
                    self.bump();
                    self.push_close("]");
                }
            }
            TokenKind::LParen => {
                self.bump();
                self.push_tight("(");
                self.fmt_pattern_list(TokenKind::RParen);
                if self.peek() == TokenKind::RParen {
                    self.bump();
                    self.push_close(")");
                }
            }
            TokenKind::LBrace => {
                self.fmt_pattern_fields();
            }
            _ => {
                let t = self.bump();
                let text = self.raw(&t);
                if !text.is_empty() {
                    self.push_tight(&text);
                }
            }
        }
    }

    /// Render the `{ x = 1, y = 2 }` field list of a struct pattern.
    fn fmt_pattern_fields(&mut self) {
        self.bump();
        self.push_head("{");
        self.mark_space();
        loop {
            match self.peek() {
                TokenKind::RBrace | TokenKind::Eof => break,
                TokenKind::Comma => self.separator(),
                TokenKind::Ident => {
                    let t = self.bump();
                    let text = self.raw(&t);
                    self.push_tight(&text);
                    if matches!(self.peek(), TokenKind::Equal | TokenKind::Colon) {
                        let t = self.bump();
                        let text = self.raw(&t);
                        self.push_op(&text);
                        self.fmt_pattern();
                    }
                }
                _ => break,
            }
        }
        if self.peek() == TokenKind::RBrace {
            self.bump();
            self.push_head("}");
        }
    }

    /// Render a comma-separated sub-pattern list up to (not including) the closer.
    fn fmt_pattern_list(&mut self, closer: TokenKind) {
        loop {
            match self.peek() {
                k if k == closer || k == TokenKind::Eof => return,
                TokenKind::Comma => self.separator(),
                _ => self.fmt_pattern(),
            }
        }
    }

    /// Render an `(p1, p2)` list after an enum variant name.
    fn fmt_pattern_subpats(&mut self) {
        self.bump();
        self.push_tight("(");
        self.fmt_pattern_list(TokenKind::RParen);
        if self.peek() == TokenKind::RParen {
            self.bump();
            self.push_close(")");
        }
    }

    /// Report whether a token kind is an assignment operator.
    fn is_assign_op(&self, k: TokenKind) -> bool {
        matches!(
            k,
            TokenKind::Equal
                | TokenKind::PlusEqual
                | TokenKind::MinusEqual
                | TokenKind::StarEqual
                | TokenKind::SlashEqual
                | TokenKind::PercentEqual
                | TokenKind::AmpersandEqual
                | TokenKind::PipeEqual
                | TokenKind::CaretEqual
                | TokenKind::ShiftLeftEqual
                | TokenKind::ShiftRightEqual
        )
    }

    /// Render an expression at the current line position.
    fn fmt_expr(&mut self) {
        self.fmt_pipe_expr();
    }

    /// Render pipe expressions `a |> f` / `a <| f`.
    fn fmt_pipe_expr(&mut self) {
        self.fmt_assign_expr();
        if matches!(self.peek(), TokenKind::PipeForward | TokenKind::BackPipe) {
            let t = self.bump();
            let text = self.raw(&t);
            self.push_op(&text);
            self.fmt_pipe_expr();
        }
    }

    /// Render assignment expressions `a = b`.
    fn fmt_assign_expr(&mut self) {
        self.fmt_range_expr();
        if self.is_assign_op(self.peek()) {
            let t = self.bump();
            let text = self.raw(&t);
            self.push_op(&text);
            self.fmt_assign_expr();
        }
    }

    /// Render range expressions `a..b`, `a..=b`, `..b`.
    fn fmt_range_expr(&mut self) {
        if self.peek() == TokenKind::DotDot {
            self.bump();
            self.push_tight("..");
            if self.peek() == TokenKind::Equal {
                self.bump();
                self.push_tight("=");
            }
            if self.range_hi_starts() {
                self.fmt_or_expr();
            }
            return;
        }
        self.fmt_or_expr();
        if self.peek() == TokenKind::DotDot {
            self.bump();
            self.push_tight("..");
            if self.peek() == TokenKind::Equal {
                self.bump();
                self.push_tight("=");
            }
            if self.range_hi_starts() {
                self.fmt_or_expr();
            }
        }
    }

    /// Report whether the upcoming token can begin a range upper bound.
    fn range_hi_starts(&self) -> bool {
        !matches!(
            self.peek(),
            TokenKind::KwDo
                | TokenKind::KwEnd
                | TokenKind::KwThen
                | TokenKind::KwElse
                | TokenKind::KwElseif
                | TokenKind::KwUntil
                | TokenKind::RParen
                | TokenKind::RBracket
                | TokenKind::RBrace
                | TokenKind::Comma
                | TokenKind::Semicolon
                | TokenKind::Eof
        )
    }

    /// Render `or` / bitwise-or chains.
    fn fmt_or_expr(&mut self) {
        self.fmt_and_expr();
        while matches!(self.peek(), TokenKind::KwOr | TokenKind::Pipe) {
            let t = self.bump();
            let text = self.raw(&t);
            self.push_op(&text);
            self.fmt_and_expr();
        }
    }

    /// Render `and` / bitwise-and / bitwise-xor chains.
    fn fmt_and_expr(&mut self) {
        self.fmt_compare_expr();
        while matches!(
            self.peek(),
            TokenKind::KwAnd | TokenKind::Ampersand | TokenKind::Caret
        ) {
            let t = self.bump();
            let text = self.raw(&t);
            self.push_op(&text);
            self.fmt_compare_expr();
        }
    }

    /// Render comparison chains.
    fn fmt_compare_expr(&mut self) {
        self.fmt_add_expr();
        while is_compare_op(self.peek()) {
            let t = self.bump();
            let text = self.raw(&t);
            self.push_op(&text);
            self.fmt_add_expr();
        }
    }

    /// Render addition, subtraction, and shift chains.
    fn fmt_add_expr(&mut self) {
        self.fmt_mul_expr();
        while matches!(
            self.peek(),
            TokenKind::Plus | TokenKind::Minus | TokenKind::ShiftLeft | TokenKind::ShiftRight
        ) {
            let t = self.bump();
            let text = self.raw(&t);
            self.push_op(&text);
            self.fmt_mul_expr();
        }
    }

    /// Render multiplication, division, remainder chains.
    fn fmt_mul_expr(&mut self) {
        self.fmt_unary_expr();
        while matches!(self.peek(), TokenKind::Star | TokenKind::Slash | TokenKind::Percent) {
            let t = self.bump();
            let text = self.raw(&t);
            self.push_op(&text);
            self.fmt_unary_expr();
        }
    }

    /// Render unary prefix operators.
    fn fmt_unary_expr(&mut self) {
        match self.peek() {
            TokenKind::Minus | TokenKind::Bang | TokenKind::Tilde | TokenKind::Star => {
                let t = self.bump();
                let text = self.raw(&t);
                self.push_tight(&text);
                self.fmt_unary_expr();
            }
            TokenKind::KwNot => {
                let t = self.bump();
                let text = self.raw(&t);
                self.push_tight(&text);
                self.mark_space();
                self.fmt_unary_expr();
            }
            TokenKind::Ampersand => {
                self.bump();
                self.push_tight("&");
                if self.peek() == TokenKind::KwVar {
                    self.push_head("var");
                    self.bump();
                    self.mark_space();
                }
                self.fmt_unary_expr();
            }
            TokenKind::KwAwait
            | TokenKind::KwTry
            | TokenKind::KwSpawn
            | TokenKind::KwMove
            | TokenKind::KwBorrow => {
                let t = self.bump();
                let text = self.raw(&t);
                self.push_tight(&text);
                self.mark_space();
                self.fmt_unary_expr();
            }
            _ => self.fmt_postfix_expr(),
        }
    }

    /// Render a primary expression followed by postfix operations.
    fn fmt_postfix_expr(&mut self) {
        self.fmt_primary();
        loop {
            match self.peek() {
                TokenKind::Dot => {
                    self.bump();
                    self.push_tight(".");
                    if self.peek() == TokenKind::Ident {
                        let t = self.bump();
                        let text = self.raw(&t);
                        self.push_tight(&text);
                    }
                    if self.peek() == TokenKind::Colon {
                        self.bump();
                        self.push_tight(":");
                        if self.peek() == TokenKind::Ident {
                            let t = self.bump();
                            let text = self.raw(&t);
                            self.push_tight(&text);
                        }
                        if self.peek() == TokenKind::LParen {
                            self.fmt_call_args();
                        }
                    } else if self.peek() == TokenKind::LParen {
                        self.fmt_call_args();
                    } else if self.peek() == TokenKind::Bang {
                        self.bump();
                        self.push_tight("!");
                    }
                }
                TokenKind::Colon => {
                    self.bump();
                    self.push_tight(":");
                    if self.peek() == TokenKind::Ident {
                        let t = self.bump();
                        let text = self.raw(&t);
                        self.push_tight(&text);
                    }
                    if self.peek() == TokenKind::LParen {
                        self.fmt_call_args();
                    }
                }
                TokenKind::DoubleColon => {
                    self.bump();
                    self.push_tight("::");
                    if self.peek() == TokenKind::Ident {
                        let t = self.bump();
                        let text = self.raw(&t);
                        self.push_tight(&text);
                    }
                    if self.peek() == TokenKind::LParen {
                        self.fmt_call_args();
                    }
                }
                TokenKind::QuestionDot => {
                    self.bump();
                    self.push_tight("?.");
                    if self.peek() == TokenKind::Ident {
                        let t = self.bump();
                        let text = self.raw(&t);
                        self.push_tight(&text);
                    }
                }
                TokenKind::LParen => self.fmt_call_args(),
                TokenKind::LBracket => {
                    self.bump();
                    self.push_tight("[");
                    self.fmt_expr();
                    if self.peek() == TokenKind::RBracket {
                        self.bump();
                        self.push_close("]");
                    }
                }
                TokenKind::Bang => {
                    self.bump();
                    self.push_tight("!");
                }
                TokenKind::KwAs | TokenKind::KwIs => {
                    let t = self.bump();
                    let text = self.raw(&t);
                    self.push_op(&text);
                    self.fmt_type();
                }
                _ => return,
            }
        }
    }

    /// Render a call argument list including its parentheses.
    fn fmt_call_args(&mut self) {
        self.bump();
        self.push_tight("(");
        loop {
            match self.peek() {
                TokenKind::RParen | TokenKind::Eof => break,
                TokenKind::Comma => self.separator(),
                TokenKind::DotDotDot => {
                    self.bump();
                    self.push_tight("...");
                    self.fmt_expr();
                }
                TokenKind::Ident if self.tok_at(1) == Some(TokenKind::Equal) => {
                    let t = self.bump();
                    let text = self.raw(&t);
                    self.push_tight(&text);
                    self.bump();
                    self.push_op("=");
                    self.fmt_expr();
                }
                _ => self.fmt_expr(),
            }
        }
        if self.peek() == TokenKind::RParen {
            self.bump();
            self.push_close(")");
        }
    }

    /// Render one primary expression.
    fn fmt_primary(&mut self) {
        match self.peek() {
            TokenKind::IntLit
            | TokenKind::FloatLit
            | TokenKind::StrLit
            | TokenKind::CharLit
            | TokenKind::RawStrLit
            | TokenKind::KwTrue
            | TokenKind::KwFalse
            | TokenKind::KwNull
            | TokenKind::KwNil
            | TokenKind::KwSelfType => {
                let t = self.bump();
                let text = self.raw(&t);
                self.push_tight(&text);
            }
            TokenKind::StrPart | TokenKind::InterpOpen => self.fmt_interp_string(),
            TokenKind::LParen => self.fmt_paren_or_tuple(),
            TokenKind::LBracket => self.fmt_array(),
            TokenKind::LBrace => self.fmt_map(),
            TokenKind::KwFn => self.fmt_closure(),
            TokenKind::KwIf => self.fmt_if_expr(),
            TokenKind::KwMatch => self.fmt_match(self.cur_indent),
            TokenKind::Ident => {
                let t = self.bump();
                let text = self.raw(&t);
                self.push_tight(&text);
                if self.peek() == TokenKind::LBrace {
                    self.fmt_struct_lit_body();
                }
            }
            TokenKind::KwDo => {
                let t = self.bump();
                let text = self.raw(&t);
                self.push_tight(&text);
                self.flush();
                self.fmt_construct_body(self.cur_indent);
            }
            _ => {
                let t = self.bump();
                let text = self.raw(&t);
                if !text.is_empty() {
                    self.push_tight(&text);
                }
            }
        }
    }

    /// Render an interpolated string as a single unit, formatting inner expressions.
    fn fmt_interp_string(&mut self) {
        loop {
            match self.tok_at(0) {
                Some(TokenKind::StrPart) => {
                    let t = self.bump();
                    let text = self.raw(&t);
                    self.push_tight(&text);
                }
                Some(TokenKind::InterpOpen) => {
                    self.bump();
                    self.push_tight("${");
                    self.fmt_expr();
                    if self.peek() == TokenKind::InterpClose {
                        self.bump();
                        self.push_tight("}");
                    }
                }
                Some(TokenKind::StrLit) => {
                    let t = self.bump();
                    let text = self.raw(&t);
                    self.push_tight(&text);
                    return;
                }
                _ => return,
            }
        }
    }

    /// Render a parenthesized expression or tuple.
    fn fmt_paren_or_tuple(&mut self) {
        self.bump();
        self.push_tight("(");
        if self.peek() == TokenKind::RParen {
            self.bump();
            self.push_close(")");
            return;
        }
        self.fmt_expr();
        while self.peek() == TokenKind::Comma {
            self.separator();
            if self.peek() == TokenKind::RParen {
                break;
            }
            self.fmt_expr();
        }
        if self.peek() == TokenKind::RParen {
            self.bump();
            self.push_close(")");
        }
    }

    /// Render an array literal `[a, b]`.
    fn fmt_array(&mut self) {
        self.bump();
        self.push_tight("[");
        loop {
            match self.peek() {
                TokenKind::RBracket | TokenKind::Eof => break,
                TokenKind::Comma => self.separator(),
                _ => self.fmt_expr(),
            }
        }
        if self.peek() == TokenKind::RBracket {
            self.bump();
            self.push_close("]");
        }
    }

    /// Render a map literal `{ k = v }`.
    fn fmt_map(&mut self) {
        self.bump();
        if self.peek() == TokenKind::RBrace {
            self.bump();
            self.push_tight("{}");
            return;
        }
        self.push_head("{");
        self.mark_space();
        loop {
            match self.peek() {
                TokenKind::RBrace | TokenKind::Eof => break,
                TokenKind::Comma => self.separator(),
                _ => self.fmt_map_field(),
            }
        }
        if self.peek() == TokenKind::RBrace {
            self.bump();
            self.push_head("}");
        }
    }

    /// Render one `key = value` entry of a map or struct literal.
    fn fmt_map_field(&mut self) {
        match self.peek() {
            TokenKind::StrLit | TokenKind::Ident => {
                let t = self.bump();
                let text = self.raw(&t);
                self.push_tight(&text);
            }
            _ => {
                let t = self.bump();
                let text = self.raw(&t);
                if !text.is_empty() {
                    self.push_tight(&text);
                }
            }
        }
        if matches!(self.peek(), TokenKind::Equal | TokenKind::Colon) {
            let t = self.bump();
            let text = self.raw(&t);
            self.push_op(&text);
            self.fmt_expr();
        }
    }

    /// Render the `{ x = 1 }` body of a struct literal after its type name.
    fn fmt_struct_lit_body(&mut self) {
        self.bump();
        if self.peek() == TokenKind::RBrace {
            self.bump();
            self.push_head("{}");
            return;
        }
        self.push_head("{");
        self.mark_space();
        loop {
            match self.peek() {
                TokenKind::RBrace | TokenKind::Eof => break,
                TokenKind::Comma | TokenKind::Semicolon => self.separator(),
                _ => self.fmt_map_field(),
            }
        }
        if self.peek() == TokenKind::RBrace {
            self.bump();
            self.push_head("}");
        }
    }

    /// Render a closure literal `fn(params) -> ret body end` across lines.
    fn fmt_closure(&mut self) {
        let t = self.bump();
        let text = self.raw(&t);
        self.push_tight(&text);
        if self.peek() == TokenKind::Less {
            self.fmt_generics();
        }
        if self.peek() == TokenKind::LParen {
            self.fmt_params();
        }
        if self.peek() == TokenKind::Arrow {
            self.bump();
            self.push_op("->");
            self.fmt_type();
        }
        self.flush();
        self.fmt_construct_body(self.cur_indent);
    }

    /// Render an inline `if c then a else b end` expression.
    fn fmt_if_expr(&mut self) {
        let t = self.bump();
        let text = self.raw(&t);
        self.push_op(&text);
        self.fmt_expr();
        if self.peek() == TokenKind::KwThen {
            let t = self.bump();
            let text = self.raw(&t);
            self.push_op(&text);
        }
        self.fmt_inline_block();
        while self.peek() == TokenKind::KwElseif {
            let t = self.bump();
            let text = self.raw(&t);
            self.push_op(&text);
            self.fmt_expr();
            if self.peek() == TokenKind::KwThen {
                let t = self.bump();
                let text = self.raw(&t);
                self.push_op(&text);
            }
            self.fmt_inline_block();
        }
        if self.peek() == TokenKind::KwElse {
            let t = self.bump();
            let text = self.raw(&t);
            self.push_op(&text);
            self.fmt_inline_block();
        }
        if self.peek() == TokenKind::KwEnd {
            let t = self.bump();
            let text = self.raw(&t);
            self.push_op(&text);
        }
    }

    /// Render the statements and tail of a block without emitting line breaks.
    fn fmt_inline_block(&mut self) {
        loop {
            match self.peek() {
                TokenKind::KwEnd
                | TokenKind::KwElse
                | TokenKind::KwElseif
                | TokenKind::Eof => return,
                _ => {}
            }
            self.fmt_expr();
            if self.is_assign_op(self.peek()) {
                let t = self.bump();
                let text = self.raw(&t);
                self.push_op(&text);
                self.fmt_expr();
            }
            if self.peek() == TokenKind::Semicolon {
                let t = self.bump();
                let text = self.raw(&t);
                self.cur.push_str(&text);
            }
            self.mark_space();
        }
    }

    /// Render a type annotation.
    fn fmt_type(&mut self) {
        self.fmt_type_atom();
        if self.peek() == TokenKind::Question {
            self.bump();
            self.push_tight("?");
        }
    }

    /// Render one type atom.
    fn fmt_type_atom(&mut self) {
        match self.peek() {
            TokenKind::KwInt
            | TokenKind::KwUInt
            | TokenKind::KwFloat
            | TokenKind::KwDouble
            | TokenKind::KwBool
            | TokenKind::KwString
            | TokenKind::KwChar
            | TokenKind::KwByte
            | TokenKind::KwI8
            | TokenKind::KwI16
            | TokenKind::KwI32
            | TokenKind::KwI64
            | TokenKind::KwI128
            | TokenKind::KwU8
            | TokenKind::KwU16
            | TokenKind::KwU32
            | TokenKind::KwU64
            | TokenKind::KwU128
            | TokenKind::KwF32
            | TokenKind::KwF64
            | TokenKind::KwSelfType => {
                let t = self.bump();
                let text = self.raw(&t);
                self.push_tight(&text);
            }
            TokenKind::KwFn => {
                let t = self.bump();
                let text = self.raw(&t);
                self.push_tight(&text);
                if self.peek() == TokenKind::LParen {
                    self.fmt_fn_type_params();
                }
                if self.peek() == TokenKind::Arrow {
                    self.bump();
                    self.push_op("->");
                    self.fmt_type();
                }
            }
            TokenKind::LBracket => {
                self.bump();
                self.push_tight("[");
                self.fmt_type();
                if self.peek() == TokenKind::RBracket {
                    self.bump();
                    self.push_close("]");
                }
            }
            TokenKind::LParen => {
                self.bump();
                self.push_tight("(");
                loop {
                    match self.peek() {
                        TokenKind::RParen | TokenKind::Eof => break,
                        TokenKind::Comma => self.separator(),
                        _ => self.fmt_type(),
                    }
                }
                if self.peek() == TokenKind::RParen {
                    self.bump();
                    self.push_close(")");
                }
            }
            TokenKind::Ampersand => {
                self.bump();
                self.push_tight("&");
                if self.peek() == TokenKind::KwVar {
                    self.push_head("var");
                    self.bump();
                    self.mark_space();
                }
                self.fmt_type();
            }
            TokenKind::KwBorrow | TokenKind::KwMove => {
                let t = self.bump();
                let text = self.raw(&t);
                self.push_tight(&text);
                self.mark_space();
                self.fmt_type();
            }
            TokenKind::Ident => {
                let t = self.bump();
                let text = self.raw(&t);
                self.push_tight(&text);
                if self.peek() == TokenKind::Less {
                    self.bump();
                    self.push_tight("<");
                    loop {
                        match self.peek() {
                            TokenKind::Greater | TokenKind::Eof => break,
                            TokenKind::Comma => self.separator(),
                            _ => self.fmt_type(),
                        }
                    }
                    if self.peek() == TokenKind::Greater {
                        self.bump();
                        self.push_close(">");
                    }
                }
            }
            _ => {
                let t = self.bump();
                let text = self.raw(&t);
                if !text.is_empty() {
                    self.push_tight(&text);
                }
            }
        }
    }

    /// Render the `(A, B)` parameter-type list of an `fn` type.
    fn fmt_fn_type_params(&mut self) {
        self.bump();
        self.push_tight("(");
        loop {
            match self.peek() {
                TokenKind::RParen | TokenKind::Eof => break,
                TokenKind::Comma => self.separator(),
                _ => self.fmt_type(),
            }
        }
        if self.peek() == TokenKind::RParen {
            self.bump();
            self.push_close(")");
        }
    }

    /// Consume unrecognized tokens to the next statement or item boundary as one line.
    fn fmt_garbage_line(&mut self) {
        self.start_line(self.cur_indent);
        let mut depth = 0usize;
        let mut first = true;
        while let Some(k) = self.tok_at(0) {
            if !first && depth == 0 && is_stmt_boundary(k) {
                break;
            }
            first = false;
            match k {
                TokenKind::LParen | TokenKind::LBracket | TokenKind::LBrace => depth += 1,
                TokenKind::RParen | TokenKind::RBracket | TokenKind::RBrace => {
                    depth = depth.saturating_sub(1)
                }
                _ => {}
            }
            let t = self.bump();
            let text = self.raw(&t);
            if !text.is_empty() {
                self.push_head(&text);
            }
        }
        self.flush();
    }
}

/// Extract the rendered text of a comment piece.
fn comment_text(p: &Piece) -> String {
    match p {
        Piece::LineComment { text, .. } | Piece::BlockComment { text, .. } => text.clone(),
        _ => String::new(),
    }
}

/// Return the exclusive end offset of a comment piece's text.
fn piece_end(p: &Piece) -> usize {
    match p {
        Piece::LineComment { text, start, .. } | Piece::BlockComment { text, start, .. } => {
            (*start as usize) + text.len()
        }
        Piece::Tok(t) => t.span.end as usize,
    }
}

/// Report whether a token kind terminates statements or items.
fn is_stmt_boundary(k: TokenKind) -> bool {
    matches!(
        k,
        TokenKind::KwLet
            | TokenKind::KwVar
            | TokenKind::KwIf
            | TokenKind::KwWhile
            | TokenKind::KwFor
            | TokenKind::KwRepeat
            | TokenKind::KwLoop
            | TokenKind::KwMatch
            | TokenKind::KwBreak
            | TokenKind::KwContinue
            | TokenKind::KwReturn
            | TokenKind::KwSpawn
            | TokenKind::KwUnsafe
            | TokenKind::KwEnd
            | TokenKind::KwElse
            | TokenKind::KwElseif
            | TokenKind::KwUntil
            | TokenKind::KwFn
            | TokenKind::KwStruct
            | TokenKind::KwEnum
            | TokenKind::KwInterface
            | TokenKind::KwConst
            | TokenKind::KwType
            | TokenKind::KwUse
            | TokenKind::KwExtern
            | TokenKind::Eof
    )
}

/// Report whether a token kind is a comparison operator.
fn is_compare_op(k: TokenKind) -> bool {
    matches!(
        k,
        TokenKind::DoubleEqual
            | TokenKind::NotEqual
            | TokenKind::LuaNotEqual
            | TokenKind::Less
            | TokenKind::LessEqual
            | TokenKind::Greater
            | TokenKind::GreaterEqual
    )
}

/// Report whether two adjacent rendered tokens would re-lex differently, forcing a space.
fn needs_space_guard(prev: Option<u8>, next: u8) -> bool {
    let p = match prev {
        Some(c) => c,
        None => return false,
    };
    if is_word_byte(p) && is_word_byte(next) {
        return true;
    }
    matches!(
        (p, next),
        (b'-', b'-')
            | (b'-', b'>')
            | (b'-', b'=')
            | (b'=', b'=')
            | (b'=', b'>')
            | (b'!', b'=')
            | (b'!', b'!')
            | (b'~', b'=')
            | (b'<', b'=')
            | (b'<', b'<')
            | (b'<', b'|')
            | (b'>', b'=')
            | (b'>', b'>')
            | (b'?', b'?')
            | (b'?', b'.')
            | (b'|', b'>')
            | (b'|', b'=')
            | (b':', b':')
            | (b'.', b'.')
            | (b'+', b'=')
            | (b'*', b'=')
            | (b'/', b'=')
            | (b'%', b'=')
            | (b'&', b'=')
            | (b'^', b'=')
    )
}

/// Report whether a byte can appear inside an identifier or number token.
fn is_word_byte(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_'
}
