// Owner: PascalElixir / axolrs (GitHub org)
// File: Recursive-descent parser for the Axolotl language.

use crate::ast::*;
use crate::diag::{Diagnostic, Diagnostics};
use crate::lexer::token::{Token, TokenKind};
use crate::span::Span;

/// Parser state machine over a token stream.
pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
    diagnostics: Diagnostics,
}

impl Parser {
    /// Construct a parser over a token stream with the given file id.
    pub fn new(tokens: Vec<Token>, _file_id: u32) -> Parser {
        Parser {
            tokens,
            pos: 0,
            diagnostics: Diagnostics::new(),
        }
    }

    /// Consume the parser and return the diagnostics.
    pub fn into_diagnostics(self) -> Diagnostics {
        self.diagnostics
    }

    /// Parse a complete module from the token stream.
    pub fn parse_module(mut self) -> (Module, Diagnostics) {
        let mut items = Vec::new();
        let start_span = self.peek().span;
        while !self.at_end() {
            if let Some(item) = self.parse_item() {
                items.push(item);
            } else {
                self.advance();
            }
        }
        let end_span = self.last_span();
        let span = start_span.union(end_span);
        let diags = std::mem::take(&mut self.diagnostics);
        (Module { items, span }, diags)
    }

    fn parse_item(&mut self) -> Option<Item> {
        let start_span = self.peek().span;
        let mut is_pub = false;
        if self.peek_kw(TokenKind::KwPub) {
            is_pub = true;
            self.advance();
        }
        match self.peek().kind {
            TokenKind::KwUse => self.parse_use(start_span),
            TokenKind::KwFn => self.parse_fn(is_pub, start_span),
            TokenKind::KwStruct => self.parse_struct(is_pub, start_span),
            TokenKind::KwEnum => self.parse_enum(is_pub, start_span),
            TokenKind::KwInterface => self.parse_interface(is_pub, start_span),
            TokenKind::KwConst => self.parse_const(is_pub, start_span),
            TokenKind::KwType => self.parse_type_alias(is_pub, start_span),
            TokenKind::KwExtern => self.parse_extern(),
            TokenKind::KwAsync => {
                self.advance();
                if self.peek_kw(TokenKind::KwFn) {
                    self.parse_fn_async(is_pub, start_span)
                } else {
                    self.error("expected `fn` after `async`", self.peek().span);
                    None
                }
            }
            TokenKind::Ident => self.parse_named_decl(is_pub, start_span),
            TokenKind::At => self.parse_attribute(),
            TokenKind::DocComment => {
                let text = self.advance().text.clone();
                Some(Item::ModuleDoc(text, start_span))
            }
            TokenKind::Eof => None,
            _ => {
                let span = self.peek().span;
                self.error(format!("unexpected token {:?} at top level", self.peek().kind), span);
                self.advance();
                None
            }
        }
    }

    fn parse_named_decl(&mut self, is_pub: bool, start: Span) -> Option<Item> {
        let ty_name = self.parse_ident()?;
        if self.peek().kind == TokenKind::Colon {
            self.advance();
            let interface_name = if self.peek().kind == TokenKind::Ident {
                Some(self.parse_ident()?)
            } else { None };
            let span = start.union(self.last_span());
            return Some(Item::Impl(ImplItem { ty_name, interface_name, span }));
        }
        if self.peek().kind == TokenKind::Dot {
            self.advance();
            let name = self.parse_ident()?;
            if self.peek().kind == TokenKind::Equal {
                self.advance();
                self.expect(TokenKind::KwFn);
                self.expect(TokenKind::LParen);
                let params = self.parse_params();
                self.expect(TokenKind::RParen);
                let ret = if self.peek().kind == TokenKind::Arrow {
                    self.advance();
                    Some(self.parse_type())
                } else { None };
                let body = self.parse_block();
                let span = start.union(self.last_span());
                return Some(Item::Method(MethodItem {
                    receiver: ty_name, name, params, ret, body, is_async: false, span,
                }));
            }
            self.expect(TokenKind::LParen);
            let params = self.parse_params();
            self.expect(TokenKind::RParen);
            let ret = if self.peek().kind == TokenKind::Arrow {
                self.advance();
                Some(self.parse_type())
            } else { None };
            let body = self.parse_block();
            let span = start.union(self.last_span());
            return Some(Item::Method(MethodItem {
                receiver: ty_name, name, params, ret, body, is_async: false, span,
            }));
        }
        if self.peek().kind == TokenKind::Equal {
            self.advance();
            match self.peek().kind {
                TokenKind::KwFn => {
                    self.advance();
                    let name = match self.parse_ident() {
                        Some(id) => id,
                        None => {
                            self.error("expected method name after `fn`", self.peek().span);
                            return None;
                        }
                    };
                    self.expect(TokenKind::LParen);
                    let params = self.parse_params();
                    self.expect(TokenKind::RParen);
                    let ret = if self.peek().kind == TokenKind::Arrow {
                        self.advance();
                        Some(self.parse_type())
                    } else { None };
                    let body = self.parse_block();
                    let span = start.union(self.last_span());
                    return Some(Item::StaticMethod(MethodItem {
                        receiver: ty_name, name, params, ret, body, is_async: false, span,
                    }));
                }
                TokenKind::KwStruct => {
                    return self.parse_struct_after_name(is_pub, start, ty_name);
                }
                TokenKind::KwEnum => {
                    return self.parse_enum_after_name(is_pub, start, ty_name);
                }
                TokenKind::KwInterface => {
                    return self.parse_interface_after_name(is_pub, start, ty_name);
                }
                _ => {
                    self.error("expected `fn`, `struct`, `enum`, or `interface` after `=`", self.peek().span);
                }
            }
            return None;
        }
        self.error("expected `:` (impl), `.` (method), or `= fn/struct/enum/interface`", self.peek().span);
        None
    }

    fn parse_struct_after_name(&mut self, is_pub: bool, start: Span, name: Ident) -> Option<Item> {
        self.expect(TokenKind::KwStruct);
        let generics = self.parse_generic_params();
        let mut fields = Vec::new();
        while !self.at_end() && !self.peek_kw(TokenKind::KwEnd) {
            let fstart = self.peek().span;
            let mut is_pub_field = false;
            if self.peek_kw(TokenKind::KwPub) { is_pub_field = true; self.advance(); }
            if let Some(fname) = self.parse_ident() {
                self.expect(TokenKind::Colon);
                let ty = self.parse_type();
                let default = if self.peek().kind == TokenKind::Equal {
                    self.advance();
                    Some(self.parse_expr())
                } else { None };
                let span = fstart.union(self.last_span());
                fields.push(StructField { name: fname, ty, default, is_pub: is_pub_field, span });
            }
            if self.peek().kind == TokenKind::Comma { self.advance(); }
            else if self.peek().kind == TokenKind::Ident || self.peek_kw(TokenKind::KwPub) { continue; }
            else { break; }
        }
        self.expect(TokenKind::KwEnd);
        let span = start.union(self.last_span());
        Some(Item::Struct(StructItem { name, generics, fields, is_pub, span }))
    }

    fn parse_struct(&mut self, is_pub: bool, start: Span) -> Option<Item> {
        self.expect(TokenKind::KwStruct);
        let name = self.parse_ident()?;
        self.parse_struct_after_name(is_pub, start, name)
    }

    fn parse_enum_after_name(&mut self, is_pub: bool, start: Span, name: Ident) -> Option<Item> {
        self.expect(TokenKind::KwEnum);
        let generics = self.parse_generic_params();
        let mut variants = Vec::new();
        while !self.at_end() && !self.peek_kw(TokenKind::KwEnd) {
            if let Some(v) = self.parse_enum_variant() {
                variants.push(v);
            } else {
                self.advance();
            }
        }
        self.expect(TokenKind::KwEnd);
        let span = start.union(self.last_span());
        Some(Item::Enum(EnumItem { name, generics, variants, is_pub, span }))
    }

    fn parse_enum(&mut self, is_pub: bool, start: Span) -> Option<Item> {
        self.expect(TokenKind::KwEnum);
        let name = self.parse_ident()?;
        self.parse_enum_after_name(is_pub, start, name)
    }

    fn parse_interface_after_name(&mut self, is_pub: bool, start: Span, name: Ident) -> Option<Item> {
        self.expect(TokenKind::KwInterface);
        let generics = self.parse_generic_params();
        let mut methods = Vec::new();
        while !self.at_end() && !self.peek_kw(TokenKind::KwEnd) {
            if let Some(m) = self.parse_method_sig() {
                methods.push(m);
            } else {
                self.advance();
            }
        }
        self.expect(TokenKind::KwEnd);
        let span = start.union(self.last_span());
        Some(Item::Interface(InterfaceItem { name, generics, methods, is_pub, span }))
    }

    fn parse_interface(&mut self, is_pub: bool, start: Span) -> Option<Item> {
        self.expect(TokenKind::KwInterface);
        let name = self.parse_ident()?;
        self.parse_interface_after_name(is_pub, start, name)
    }

    fn parse_use(&mut self, start: Span) -> Option<Item> {
        self.expect(TokenKind::KwUse);
        let mut path = String::new();
        let mut first = true;
        while let Some(t) = self.peek_ident_or_str() {
            if !first { path.push_str("::"); }
            path.push_str(&t);
            first = false;
            self.advance();
            if self.peek().kind == TokenKind::DoubleColon { self.advance(); }
            else { break; }
        }
        let span = start.union(self.last_span());
        Some(Item::Use(UseItem { path, span }))
    }

    fn parse_fn(&mut self, is_pub: bool, start: Span) -> Option<Item> {
        self.parse_fn_inner(is_pub, false, start)
    }

    fn parse_fn_async(&mut self, is_pub: bool, start: Span) -> Option<Item> {
        self.parse_fn_inner(is_pub, true, start)
    }

    fn parse_fn_inner(&mut self, is_pub: bool, is_async: bool, start: Span) -> Option<Item> {
        self.expect(TokenKind::KwFn);
        let name = self.parse_ident()?;
        let generics = self.parse_generic_params();
        self.expect(TokenKind::LParen);
        let params = self.parse_params();
        self.expect(TokenKind::RParen);
        let ret = if self.peek().kind == TokenKind::Arrow {
            self.advance();
            Some(self.parse_type())
        } else {
            None
        };
        let is_extern = if self.peek().kind == TokenKind::Semicolon {
            self.advance();
            true
        } else {
            false
        };
        if is_extern {
            let span = start.union(self.last_span());
            return Some(Item::Fn(FnItem {
                name, generics, params, ret,
                body: empty_block(span),
                is_pub, is_async, is_extern, span,
            }));
        }
        let body = self.parse_block();
        let span = start.union(self.last_span());
        Some(Item::Fn(FnItem {
            name, generics, params, ret,
            body, is_pub, is_async, is_extern, span,
        }))
    }

    fn parse_enum_variant(&mut self) -> Option<EnumVariant> {
        let start = self.peek().span;
        let name = self.parse_ident()?;
        let mut fields = Vec::new();
        if self.peek().kind == TokenKind::LParen {
            self.advance();
            loop {
                if self.peek().kind == TokenKind::RParen { break; }
                let fname = if self.peek().kind == TokenKind::Ident {
                    let id = self.parse_ident()?;
                    if self.peek().kind == TokenKind::Colon {
                        self.advance();
                        Some(id)
                    } else {
                        // positional
                        self.tokens.insert(self.pos, Token { kind: TokenKind::Colon, span: id.span, text: ":".into() });
                        None
                    }
                } else { None };
                let ty = self.parse_type();
                let fspan = ty.span;
                fields.push(EnumVariantField { name: fname, ty, span: fspan });
                if self.peek().kind == TokenKind::Comma { self.advance(); }
                else { break; }
            }
            self.expect(TokenKind::RParen);
        }
        let span = start.union(self.last_span());
        Some(EnumVariant { name, fields, span })
    }

    fn parse_method_sig(&mut self) -> Option<MethodSig> {
        let start = self.peek().span;
        let name = self.parse_ident()?;
        self.expect(TokenKind::LParen);
        let params = self.parse_params();
        self.expect(TokenKind::RParen);
        let ret = if self.peek().kind == TokenKind::Arrow {
            self.advance();
            Some(self.parse_type())
        } else { None };
        let default_body = if self.peek().kind == TokenKind::KwEnd || self.peek().kind == TokenKind::Ident {
            None
        } else {
            Some(self.parse_block())
        };
        let span = start.union(self.last_span());
        Some(MethodSig { name, params, ret, default_body, span })
    }

    fn parse_const(&mut self, is_pub: bool, start: Span) -> Option<Item> {
        self.expect(TokenKind::KwConst);
        let name = self.parse_ident()?;
        let ty = if self.peek().kind == TokenKind::Colon {
            self.advance();
            Some(self.parse_type())
        } else { None };
        self.expect(TokenKind::Equal);
        let value = self.parse_expr();
        let span = start.union(self.last_span());
        Some(Item::Const(ConstItem { name, ty, value, is_pub, span }))
    }

    fn parse_type_alias(&mut self, is_pub: bool, start: Span) -> Option<Item> {
        self.expect(TokenKind::KwType);
        let name = self.parse_ident()?;
        let generics = self.parse_generic_params();
        self.expect(TokenKind::Equal);
        let ty = self.parse_type();
        let span = start.union(self.last_span());
        Some(Item::TypeAlias(TypeAliasItem { name, generics, ty, is_pub, span }))
    }

    fn parse_extern(&mut self) -> Option<Item> {
        let start = self.peek().span;
        self.expect(TokenKind::KwExtern);
        let abi = if let Some(text) = self.peek_string() {
            self.advance();
            text
        } else { String::from("C") };
        let span = start.union(self.last_span());
        Some(Item::ExternBlock(ExternBlock { name: abi.clone(), abi, span }))
    }

    fn parse_attribute(&mut self) -> Option<Item> {
        let start = self.peek().span;
        self.expect(TokenKind::At);
        let name = self.parse_ident()?;
        // For now, we discard the attribute body and treat it as a module doc item.
        while !self.at_end() && !matches!(self.peek().kind,
            TokenKind::KwFn | TokenKind::KwStruct | TokenKind::KwEnum |
            TokenKind::KwInterface | TokenKind::KwConst | TokenKind::KwType |
            TokenKind::KwUse | TokenKind::KwExtern | TokenKind::Eof) {
            self.advance();
        }
        let span = start.union(self.last_span());
        Some(Item::ModuleDoc(format!("@{}", name.name), span))
    }

    fn parse_generic_params(&mut self) -> Vec<Ident> {
        let mut out = Vec::new();
        if self.peek().kind == TokenKind::Less {
            self.advance();
            loop {
                if self.peek().kind == TokenKind::Greater { self.advance(); break; }
                if let Some(id) = self.parse_ident() {
                    out.push(id);
                    if self.peek().kind == TokenKind::Comma { self.advance(); }
                } else { break; }
            }
        }
        out
    }

    fn parse_params(&mut self) -> Vec<Param> {
        let mut out = Vec::new();
        loop {
            if self.peek().kind == TokenKind::RParen { break; }
            let start = self.peek().span;
            let spread = if self.peek().kind == TokenKind::DotDotDot {
                self.advance();
                true
            } else { false };
            let name = match self.parse_ident() {
                Some(id) => id,
                None => break,
            };
            let ty = if self.peek().kind == TokenKind::Colon {
                self.advance();
                Some(self.parse_type())
            } else { None };
            let default = if self.peek().kind == TokenKind::Equal {
                self.advance();
                Some(self.parse_expr())
            } else { None };
            let span = start.union(self.last_span());
            out.push(Param { name, ty, default, span });
            if spread { break; }
            if self.peek().kind == TokenKind::Comma { self.advance(); }
            else { break; }
        }
        out
    }

    fn parse_block(&mut self) -> Block {
        let start = self.peek().span;
        // Block can be either an explicit `do ... end` or just a sequence until `end`
        if self.peek().kind == TokenKind::KwDo { self.advance(); }
        let mut stmts = Vec::new();
        let mut tail = None;
        while !self.at_end() && !self.peek_kw(TokenKind::KwEnd)
            && !self.peek_kw(TokenKind::KwElse)
            && !self.peek_kw(TokenKind::KwElseif)
            && !self.peek_kw(TokenKind::KwUntil) {
            if self.peek().kind == TokenKind::KwReturn {
                let rs = self.peek().span;
                self.advance();
                if self.is_expr_start() && !self.peek_kw(TokenKind::KwEnd) && !self.peek_kw(TokenKind::KwElse) {
                    let e = self.parse_expr();
                    let es = e.span();
                    stmts.push(Stmt::Return(Some(e), rs.union(es)));
                    break;
                } else {
                    stmts.push(Stmt::Return(None, rs));
                    break;
                }
            }
            if self.peek_kw(TokenKind::KwEnd) || self.peek_kw(TokenKind::KwElse) || self.peek_kw(TokenKind::KwElseif) || self.peek_kw(TokenKind::KwUntil) { break; }
            if self.is_stmt_start() {
                if let Some(s) = self.parse_stmt() {
                    stmts.push(s);
                }
            } else {
                let e = self.parse_expr();
                let es = e.span();
                if self.peek().kind == TokenKind::Equal
                    || (self.peek().kind as u32) >= (TokenKind::PlusEqual as u32)
                       && (self.peek().kind as u32) <= (TokenKind::ShiftRightEqual as u32) {
                    let op = self.assign_op(self.peek().kind);
                    self.advance();
                    let val = self.parse_expr();
                    let s = Stmt::Assign {
                        target: e,
                        value: val,
                        op,
                        span: es,
                    };
                    stmts.push(s);
                } else if self.peek().kind == TokenKind::Semicolon {
                    self.advance();
                    stmts.push(Stmt::Expr(e, es));
                } else if self.peek_kw(TokenKind::KwEnd) {
                    tail = Some(Box::new(e));
                    break;
                } else {
                    stmts.push(Stmt::Expr(e, es));
                }
            }
        }
        // Only consume `end` - `else`, `elseif`, and `until` are handled by their enclosing construct.
        if self.peek_kw(TokenKind::KwEnd) { self.advance(); }
        let span = start.union(self.last_span());
        Block { stmts, tail, span }
    }

    fn assign_op(&self, k: TokenKind) -> AssignOp {
        match k {
            TokenKind::Equal => AssignOp::Assign,
            TokenKind::PlusEqual => AssignOp::Add,
            TokenKind::MinusEqual => AssignOp::Sub,
            TokenKind::StarEqual => AssignOp::Mul,
            TokenKind::SlashEqual => AssignOp::Div,
            TokenKind::PercentEqual => AssignOp::Rem,
            TokenKind::AmpersandEqual => AssignOp::And,
            TokenKind::PipeEqual => AssignOp::Or,
            TokenKind::CaretEqual => AssignOp::Xor,
            TokenKind::ShiftLeftEqual => AssignOp::Shl,
            TokenKind::ShiftRightEqual => AssignOp::Shr,
            _ => AssignOp::Assign,
        }
    }

    fn is_stmt_start(&self) -> bool {
        matches!(self.peek().kind,
            TokenKind::KwLet | TokenKind::KwVar |
            TokenKind::KwIf | TokenKind::KwWhile | TokenKind::KwRepeat |
            TokenKind::KwFor | TokenKind::KwLoop | TokenKind::KwMatch |
            TokenKind::KwBreak | TokenKind::KwContinue | TokenKind::KwReturn |
            TokenKind::KwSpawn | TokenKind::KwUnsafe
        )
    }

    fn is_expr_start(&self) -> bool {
        !matches!(self.peek().kind, TokenKind::KwEnd | TokenKind::Eof)
    }

    fn parse_stmt(&mut self) -> Option<Stmt> {
        let start = self.peek().span;
        match self.peek().kind {
            TokenKind::KwLet => {
                self.advance();
                let name = self.parse_ident()?;
                let ty = if self.peek().kind == TokenKind::Colon {
                    self.advance();
                    Some(self.parse_type())
                } else { None };
                self.expect(TokenKind::Equal);
                let value = self.parse_expr();
                let span = start.union(self.last_span());
                Some(Stmt::Let { name, ty, value, span })
            }
            TokenKind::KwVar => {
                self.advance();
                let name = self.parse_ident()?;
                let ty = if self.peek().kind == TokenKind::Colon {
                    self.advance();
                    Some(self.parse_type())
                } else { None };
                self.expect(TokenKind::Equal);
                let value = self.parse_expr();
                let span = start.union(self.last_span());
                Some(Stmt::Var { name, ty, value, span })
            }
            TokenKind::KwIf => {
                self.advance();
                let cond = self.parse_expr();
                self.expect(TokenKind::KwThen);
                let then_body = self.parse_block();
                let mut elseifs = Vec::new();
                let mut else_body = None;
                while self.peek_kw(TokenKind::KwElseif) {
                    self.advance();
                    let c = self.parse_expr();
                    self.expect(TokenKind::KwThen);
                    let b = self.parse_block();
                    elseifs.push((c, b));
                }
                if self.peek_kw(TokenKind::KwElse) {
                    self.advance();
                    else_body = Some(self.parse_block());
                }
                let span = start.union(self.last_span());
                Some(Stmt::If { cond, then_body, elseifs, else_body, span })
            }
            TokenKind::KwWhile => {
                self.advance();
                let cond = self.parse_expr();
                self.expect(TokenKind::KwDo);
                let body = self.parse_block();
                let span = start.union(self.last_span());
                Some(Stmt::While { cond, body, span })
            }
            TokenKind::KwRepeat => {
                self.advance();
                let body = self.parse_block();
                self.expect(TokenKind::KwUntil);
                let cond = self.parse_expr();
                let span = start.union(self.last_span());
                Some(Stmt::Repeat { body, cond, span })
            }
            TokenKind::KwFor => {
                self.advance();
                let var = self.parse_ident()?;
                self.expect(TokenKind::KwIn);
                let iter = self.parse_expr();
                self.expect(TokenKind::KwDo);
                let body = self.parse_block();
                let span = start.union(self.last_span());
                Some(Stmt::For { var, iter, body, span })
            }
            TokenKind::KwLoop => {
                self.advance();
                let body = self.parse_block();
                let span = start.union(self.last_span());
                Some(Stmt::Loop { body, span })
            }
            TokenKind::KwBreak => {
                let s = self.peek().span;
                self.advance();
                Some(Stmt::Break(s))
            }
            TokenKind::KwContinue => {
                let s = self.peek().span;
                self.advance();
                Some(Stmt::Continue(s))
            }
            TokenKind::KwMatch => {
                self.advance();
                let scrutinee = self.parse_expr();
                let mut arms = Vec::new();
                while !self.at_end() && !self.peek_kw(TokenKind::KwEnd) {
                    if let Some(arm) = self.parse_match_arm() {
                        arms.push(arm);
                    } else {
                        self.advance();
                    }
                }
                self.expect(TokenKind::KwEnd);
                let span = start.union(self.last_span());
                Some(Stmt::Match { scrutinee, arms, span })
            }
            TokenKind::KwSpawn => {
                self.advance();
                let call = self.parse_expr();
                let span = start.union(self.last_span());
                Some(Stmt::Spawn { call, span })
            }
            TokenKind::KwUnsafe => {
                self.advance();
                let body = self.parse_block();
                let span = start.union(self.last_span());
                Some(Stmt::Unsafe(body, span))
            }
            _ => {
                let e = self.parse_expr();
                let es = e.span();
                Some(Stmt::Expr(e, es))
            }
        }
    }

    fn parse_match_arm(&mut self) -> Option<MatchArm> {
        let start = self.peek().span;
        let pattern = self.parse_pattern();
        let guard = if self.peek_kw(TokenKind::KwIf) {
            self.advance();
            Some(self.parse_expr())
        } else { None };
        self.expect(TokenKind::FatArrow);
        let body = self.parse_match_arm_body(start);
        let span = start.union(self.last_span());
        Some(MatchArm { pattern, guard, body, span })
    }

    fn parse_match_arm_body(&mut self, start: Span) -> Block {
        // A match arm body can be either a block (statements + tail) or a single expression.
        if self.peek().kind == TokenKind::KwDo {
            return self.parse_block();
        }
        let e = self.parse_expr();
        let es = e.span();
        let stmt = Stmt::Expr(e, es);
        let span = start.union(self.last_span());
        Block { stmts: vec![stmt], tail: None, span }
    }

    fn parse_pattern(&mut self) -> Pattern {
        let p = self.parse_pattern_atom();
        if self.peek().kind == TokenKind::Pipe {
            let span = p.span();
            let mut alts = vec![p];
            while self.peek().kind == TokenKind::Pipe {
                self.advance();
                alts.push(self.parse_pattern_atom());
            }
            return Pattern::Or(alts, span);
        }
        if self.peek_kw(TokenKind::KwAs) {
            self.advance();
            let name = self.parse_ident();
            if let Some(id) = name {
                let s = p.span();
                return Pattern::As(Box::new(p), id, s);
            }
        }
        p
    }

    fn parse_pattern_atom(&mut self) -> Pattern {
        let start = self.peek().span;
        match self.peek().kind {
            TokenKind::Ident => {
                let id = self.parse_ident().unwrap();
                if self.peek().kind == TokenKind::Dot {
                    self.advance();
                    let variant = self.parse_ident().unwrap();
                    let mut subs = Vec::new();
                    if self.peek().kind == TokenKind::LParen {
                        self.advance();
                        loop {
                            if self.peek().kind == TokenKind::RParen { break; }
                            subs.push(self.parse_pattern());
                            if self.peek().kind == TokenKind::Comma { self.advance(); }
                            else { break; }
                        }
                        self.expect(TokenKind::RParen);
                    }
                    let span = start.union(self.last_span());
                    return Pattern::EnumVariant { ty: id, variant, sub: subs, span };
                }
                Pattern::Binding(id)
            }
            TokenKind::LBracket => {
                self.advance();
                let mut elems = Vec::new();
                loop {
                    if self.peek().kind == TokenKind::RBracket { break; }
                    elems.push(self.parse_pattern());
                    if self.peek().kind == TokenKind::Comma { self.advance(); }
                    else { break; }
                }
                self.expect(TokenKind::RBracket);
                Pattern::Array(elems, start.union(self.last_span()))
            }
            TokenKind::LParen => {
                self.advance();
                let mut elems = Vec::new();
                loop {
                    if self.peek().kind == TokenKind::RParen { break; }
                    elems.push(self.parse_pattern());
                    if self.peek().kind == TokenKind::Comma { self.advance(); }
                    else { break; }
                }
                self.expect(TokenKind::RParen);
                Pattern::Tuple(elems, start.union(self.last_span()))
            }
            TokenKind::LBrace => {
                self.advance();
                let ty = self.parse_ident().unwrap();
                let mut fields = Vec::new();
                loop {
                    if self.peek().kind == TokenKind::RBrace { break; }
                    let fname = self.parse_ident().unwrap();
                    let pat = if self.peek().kind == TokenKind::Equal {
                        self.advance();
                        self.parse_pattern()
                    } else {
                        Pattern::Binding(fname.clone())
                    };
                    fields.push((fname, pat));
                    if self.peek().kind == TokenKind::Comma { self.advance(); }
                    else { break; }
                }
                self.expect(TokenKind::RBrace);
                Pattern::Struct { ty, fields, span: start.union(self.last_span()) }
            }
            TokenKind::IntLit | TokenKind::FloatLit | TokenKind::StrLit | TokenKind::CharLit | TokenKind::KwTrue | TokenKind::KwFalse | TokenKind::KwNull => {
                let e = self.parse_literal_expr();
                Pattern::Lit(e, start)
            }
            _ => {
                self.advance();
                Pattern::Wild(start)
            }
        }
    }

    fn parse_literal_expr(&mut self) -> Expr {
        let start = self.peek().span;
        if matches!(self.peek().kind, TokenKind::StrPart | TokenKind::InterpOpen) {
            return self.parse_interp_string(start);
        }
        let t = self.peek().clone();
        self.advance();
        match t.kind {
            TokenKind::IntLit => Expr::IntLit(t.text, t.span),
            TokenKind::FloatLit => Expr::FloatLit(t.text, t.span),
            TokenKind::StrLit => Expr::StrLit(t.text, t.span),
            TokenKind::RawStrLit => Expr::RawStrLit(t.text, t.span),
            TokenKind::CharLit => Expr::CharLit(t.text, t.span),
            TokenKind::KwTrue => Expr::BoolLit(true, t.span),
            TokenKind::KwFalse => Expr::BoolLit(false, t.span),
            TokenKind::KwNull | TokenKind::KwNil => Expr::Null(t.span),
            _ => Expr::Unit(t.span),
        }
    }

    /// Parse an interpolated string from its token sequence into InterpStr parts.
    fn parse_interp_string(&mut self, start: Span) -> Expr {
        let mut parts = Vec::new();
        loop {
            match self.peek().kind {
                TokenKind::StrPart => {
                    let t = self.advance().clone();
                    if !t.text.is_empty() {
                        parts.push(InterpStrPart::Text(t.text));
                    }
                }
                TokenKind::InterpOpen => {
                    self.advance();
                    let e = self.parse_expr();
                    self.expect(TokenKind::InterpClose);
                    parts.push(InterpStrPart::Expr(e));
                }
                TokenKind::StrLit => {
                    let t = self.advance().clone();
                    if !t.text.is_empty() {
                        parts.push(InterpStrPart::Text(t.text));
                    }
                    break;
                }
                _ => break,
            }
        }
        let span = start.union(self.last_span());
        Expr::InterpStr(parts, span)
    }

    fn parse_type(&mut self) -> TypeAnnot {
        let start = self.peek().span;
        let kind = self.parse_type_atom();
        let mut t = TypeAnnot { kind, span: start.union(self.last_span()) };
        if self.peek().kind == TokenKind::Question {
            self.advance();
            let s = t.span;
            t = TypeAnnot { kind: TypeAnnotKind::Optional(Box::new(t)), span: s };
        }
        t
    }

    fn parse_type_atom(&mut self) -> TypeAnnotKind {
        match self.peek().kind {
            TokenKind::KwInt => { self.advance(); TypeAnnotKind::Primitive(TokenKind::KwInt) }
            TokenKind::KwUInt => { self.advance(); TypeAnnotKind::Primitive(TokenKind::KwUInt) }
            TokenKind::KwFloat | TokenKind::KwDouble => { self.advance(); TypeAnnotKind::Primitive(TokenKind::KwFloat) }
            TokenKind::KwBool => { self.advance(); TypeAnnotKind::Primitive(TokenKind::KwBool) }
            TokenKind::KwString => { self.advance(); TypeAnnotKind::Primitive(TokenKind::KwString) }
            TokenKind::KwChar => { self.advance(); TypeAnnotKind::Primitive(TokenKind::KwChar) }
            TokenKind::KwByte => { self.advance(); TypeAnnotKind::Primitive(TokenKind::KwByte) }
            TokenKind::KwI8 => { self.advance(); TypeAnnotKind::Primitive(TokenKind::KwI8) }
            TokenKind::KwI16 => { self.advance(); TypeAnnotKind::Primitive(TokenKind::KwI16) }
            TokenKind::KwI32 => { self.advance(); TypeAnnotKind::Primitive(TokenKind::KwI32) }
            TokenKind::KwI64 => { self.advance(); TypeAnnotKind::Primitive(TokenKind::KwI64) }
            TokenKind::KwI128 => { self.advance(); TypeAnnotKind::Primitive(TokenKind::KwI128) }
            TokenKind::KwU8 => { self.advance(); TypeAnnotKind::Primitive(TokenKind::KwU8) }
            TokenKind::KwU16 => { self.advance(); TypeAnnotKind::Primitive(TokenKind::KwU16) }
            TokenKind::KwU32 => { self.advance(); TypeAnnotKind::Primitive(TokenKind::KwU32) }
            TokenKind::KwU64 => { self.advance(); TypeAnnotKind::Primitive(TokenKind::KwU64) }
            TokenKind::KwU128 => { self.advance(); TypeAnnotKind::Primitive(TokenKind::KwU128) }
            TokenKind::KwF32 => { self.advance(); TypeAnnotKind::Primitive(TokenKind::KwF32) }
            TokenKind::KwF64 => { self.advance(); TypeAnnotKind::Primitive(TokenKind::KwF64) }
            TokenKind::KwSelfType => { self.advance(); TypeAnnotKind::SelfType }
            TokenKind::KwFn => {
                self.advance();
                self.expect(TokenKind::LParen);
                let mut params = Vec::new();
                loop {
                    if self.peek().kind == TokenKind::RParen { break; }
                    params.push(self.parse_type());
                    if self.peek().kind == TokenKind::Comma { self.advance(); }
                    else { break; }
                }
                self.expect(TokenKind::RParen);
                let ret = if self.peek().kind == TokenKind::Arrow {
                    self.advance();
                    self.parse_type()
                } else {
                    TypeAnnot { kind: TypeAnnotKind::Infer, span: self.last_span() }
                };
                TypeAnnotKind::Function(params, Box::new(ret))
            }
            TokenKind::LBracket => {
                self.advance();
                let inner = self.parse_type();
                self.expect(TokenKind::RBracket);
                TypeAnnotKind::Array(Box::new(inner))
            }
            TokenKind::LParen => {
                self.advance();
                let mut elems = Vec::new();
                loop {
                    if self.peek().kind == TokenKind::RParen { break; }
                    elems.push(self.parse_type());
                    if self.peek().kind == TokenKind::Comma { self.advance(); }
                    else { break; }
                }
                self.expect(TokenKind::RParen);
                TypeAnnotKind::Tuple(elems)
            }
            TokenKind::Ampersand => {
                self.advance();
                let is_mut = if self.peek_kw(TokenKind::KwVar) { self.advance(); true } else { false };
                let inner = self.parse_type();
                TypeAnnotKind::Borrow(Box::new(inner), is_mut)
            }
            TokenKind::KwBorrow => {
                self.advance();
                let is_mut = if self.peek_kw(TokenKind::KwVar) { self.advance(); true } else { false };
                let inner = self.parse_type();
                TypeAnnotKind::Borrow(Box::new(inner), is_mut)
            }
            TokenKind::KwMove => {
                self.advance();
                let inner = self.parse_type();
                TypeAnnotKind::Move(Box::new(inner))
            }
            TokenKind::Ident => {
                let id = self.parse_ident().unwrap();
                let mut args = Vec::new();
                if self.peek().kind == TokenKind::Less {
                    self.advance();
                    loop {
                        if self.peek().kind == TokenKind::Greater { break; }
                        args.push(self.parse_type());
                        if self.peek().kind == TokenKind::Comma { self.advance(); }
                        else { break; }
                    }
                    self.expect(TokenKind::Greater);
                }
                TypeAnnotKind::Named(id, args)
            }
            _ => {
                self.error("expected type", self.peek().span);
                TypeAnnotKind::Infer
            }
        }
    }

    fn parse_expr(&mut self) -> Expr {
        self.parse_pipe_expr()
    }

    fn parse_pipe_expr(&mut self) -> Expr {
        let left = self.parse_assign_expr();
        if self.peek().kind == TokenKind::PipeForward {
            let start = left.span();
            self.advance();
            let right = self.parse_pipe_expr();
            let rspan = right.span();
            return Expr::PipeForward(Box::new(left), Box::new(right), start.union(rspan));
        }
        if self.peek().kind == TokenKind::BackPipe {
            let start = left.span();
            self.advance();
            let right = self.parse_pipe_expr();
            let rspan = right.span();
            return Expr::BackPipe(Box::new(right), Box::new(left), start.union(rspan));
        }
        left
    }

    fn parse_assign_expr(&mut self) -> Expr {
        let left = self.parse_range_expr();
        let span = left.span();
        if matches!(self.peek().kind, TokenKind::Equal | TokenKind::PlusEqual | TokenKind::MinusEqual |
            TokenKind::StarEqual | TokenKind::SlashEqual | TokenKind::PercentEqual |
            TokenKind::AmpersandEqual | TokenKind::PipeEqual | TokenKind::CaretEqual |
            TokenKind::ShiftLeftEqual | TokenKind::ShiftRightEqual) {
            let op = self.assign_op(self.peek().kind);
            self.advance();
            let right = self.parse_assign_expr();
            let rspan = right.span();
            return Expr::Assign(Box::new(left), op, Box::new(right), span.union(rspan));
        }
        left
    }

    /// Parse a range expression `a..b` / `a..=b` / `..b` / `a..` above the logical layer.
    fn parse_range_expr(&mut self) -> Expr {
        if self.peek().kind == TokenKind::DotDot {
            let start = self.peek().span;
            self.advance();
            let inclusive = if self.peek().kind == TokenKind::Equal {
                self.advance();
                true
            } else {
                false
            };
            let hi = self.parse_or_expr();
            let hspan = hi.span();
            return Expr::Range(None, Some(Box::new(hi)), inclusive, start.union(hspan));
        }
        let lo = self.parse_or_expr();
        if self.peek().kind == TokenKind::DotDot {
            let lspan = lo.span();
            self.advance();
            let inclusive = if self.peek().kind == TokenKind::Equal {
                self.advance();
                true
            } else {
                false
            };
            if self.range_hi_starts() {
                let hi = self.parse_or_expr();
                let hspan = hi.span();
                return Expr::Range(Some(Box::new(lo)), Some(Box::new(hi)), inclusive, lspan.union(hspan));
            }
            return Expr::Range(Some(Box::new(lo)), None, inclusive, lspan);
        }
        lo
    }

    /// True when the next token can begin the upper bound of a range.
    fn range_hi_starts(&self) -> bool {
        !matches!(
            self.peek().kind,
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

    fn parse_or_expr(&mut self) -> Expr {
        let mut left = self.parse_and_expr();
        loop {
            let op = match self.peek().kind {
                TokenKind::KwOr => BinOp::LogicalOr,
                TokenKind::Pipe => BinOp::BitOr,
                _ => break,
            };
            let span = left.span();
            self.advance();
            let right = self.parse_and_expr();
            let rspan = right.span();
            left = Expr::BinOp(Box::new(left), op, Box::new(right), span.union(rspan));
        }
        left
    }

    fn parse_and_expr(&mut self) -> Expr {
        let mut left = self.parse_compare_expr();
        loop {
            let op = match self.peek().kind {
                TokenKind::KwAnd => BinOp::LogicalAnd,
                TokenKind::Ampersand => BinOp::BitAnd,
                _ => break,
            };
            let span = left.span();
            self.advance();
            let right = self.parse_compare_expr();
            let rspan = right.span();
            left = Expr::BinOp(Box::new(left), op, Box::new(right), span.union(rspan));
        }
        left
    }

    fn parse_compare_expr(&mut self) -> Expr {
        let mut left = self.parse_add_expr();
        loop {
            let op = match self.peek().kind {
                TokenKind::DoubleEqual => BinOp::Eq,
                TokenKind::NotEqual | TokenKind::LuaNotEqual => BinOp::Ne,
                TokenKind::Less => BinOp::Lt,
                TokenKind::LessEqual => BinOp::Le,
                TokenKind::Greater => BinOp::Gt,
                TokenKind::GreaterEqual => BinOp::Ge,
                _ => break,
            };
            let span = left.span();
            self.advance();
            let right = self.parse_add_expr();
            let rspan = right.span();
            left = Expr::BinOp(Box::new(left), op, Box::new(right), span.union(rspan));
        }
        left
    }

    fn parse_add_expr(&mut self) -> Expr {
        let mut left = self.parse_mul_expr();
        while matches!(self.peek().kind, TokenKind::Plus | TokenKind::Minus) {
            let op = if self.peek().kind == TokenKind::Plus { BinOp::Add } else { BinOp::Sub };
            let span = left.span();
            self.advance();
            let right = self.parse_mul_expr();
            let rspan = right.span();
            left = Expr::BinOp(Box::new(left), op, Box::new(right), span.union(rspan));
        }
        left
    }

    fn parse_mul_expr(&mut self) -> Expr {
        let mut left = self.parse_unary_expr();
        while matches!(self.peek().kind, TokenKind::Star | TokenKind::Slash | TokenKind::Percent) {
            let op = match self.peek().kind {
                TokenKind::Star => BinOp::Mul,
                TokenKind::Slash => BinOp::Div,
                TokenKind::Percent => BinOp::Rem,
                _ => unreachable!(),
            };
            let span = left.span();
            self.advance();
            let right = self.parse_unary_expr();
            let rspan = right.span();
            left = Expr::BinOp(Box::new(left), op, Box::new(right), span.union(rspan));
        }
        left
    }

    fn parse_unary_expr(&mut self) -> Expr {
        let start = self.peek().span;
        match self.peek().kind {
            TokenKind::Minus => {
                self.advance();
                let e = self.parse_unary_expr();
                let s = e.span();
                Expr::UnaryOp(UnaryOp::Neg, Box::new(e), start.union(s))
            }
            TokenKind::KwNot | TokenKind::Bang => {
                self.advance();
                let e = self.parse_unary_expr();
                let s = e.span();
                Expr::UnaryOp(UnaryOp::Not, Box::new(e), start.union(s))
            }
            TokenKind::Tilde => {
                self.advance();
                let e = self.parse_unary_expr();
                let s = e.span();
                Expr::UnaryOp(UnaryOp::BitNot, Box::new(e), start.union(s))
            }
            TokenKind::Ampersand => {
                self.advance();
                let is_mut = if self.peek_kw(TokenKind::KwVar) { self.advance(); true } else { false };
                let e = self.parse_unary_expr();
                let s = e.span();
                let op = if is_mut { UnaryOp::MutRef } else { UnaryOp::Ref };
                Expr::UnaryOp(op, Box::new(e), start.union(s))
            }
            TokenKind::Star => {
                self.advance();
                let e = self.parse_unary_expr();
                let s = e.span();
                Expr::UnaryOp(UnaryOp::Deref, Box::new(e), start.union(s))
            }
            TokenKind::KwAwait => {
                self.advance();
                let e = self.parse_unary_expr();
                let s = e.span();
                Expr::Await(Box::new(e), start.union(s))
            }
            TokenKind::KwTry => {
                self.advance();
                let e = self.parse_unary_expr();
                let s = e.span();
                Expr::Try(Box::new(e), start.union(s))
            }
            TokenKind::KwSpawn => {
                self.advance();
                let e = self.parse_unary_expr();
                Expr::Call(Box::new(Expr::Ident(Ident { name: "spawn".into(), span: start })), vec![CallArg::Positional(e)], start)
            }
            TokenKind::KwMove => {
                self.advance();
                let e = self.parse_unary_expr();
                let s = e.span();
                Expr::UnaryOp(UnaryOp::Deref, Box::new(e), start.union(s))
            }
            _ => self.parse_postfix_expr(),
        }
    }

    fn parse_postfix_expr(&mut self) -> Expr {
        let mut e = self.parse_primary();
        loop {
            match self.peek().kind {
                TokenKind::Dot => {
                    self.advance();
                    let field_start = self.peek().span;
                    let field = self.parse_ident().unwrap_or(Ident { name: String::new(), span: field_start });
                    let span = e.span().union(field.span);
                    if self.peek().kind == TokenKind::LParen {
                        self.advance();
                        let args = self.parse_call_args();
                        self.expect(TokenKind::RParen);
                        let s = e.span().union(self.last_span());
                        e = Expr::MethodCall(Box::new(e), field, args, false, s);
                    } else if self.peek().kind == TokenKind::Colon {
                        self.advance();
                        self.expect(TokenKind::LParen);
                        let args = self.parse_call_args();
                        self.expect(TokenKind::RParen);
                        let s = e.span().union(self.last_span());
                        e = Expr::MethodCall(Box::new(e), field, args, true, s);
                    } else if self.peek().kind == TokenKind::Bang {
                        self.advance();
                        let span = e.span().union(self.last_span());
                        e = Expr::Bang(Box::new(e), span);
                    } else {
                        e = Expr::Field(Box::new(e), field, span);
                    }
                }
                TokenKind::Colon => {
                    self.advance();
                    let field_start = self.peek().span;
                    let field = self.parse_ident().unwrap_or(Ident { name: String::new(), span: field_start });
                    self.expect(TokenKind::LParen);
                    let args = self.parse_call_args();
                    self.expect(TokenKind::RParen);
                    let s = e.span().union(self.last_span());
                    e = Expr::MethodCall(Box::new(e), field, args, true, s);
                }
                TokenKind::QuestionDot => {
                    self.advance();
                    let field_start = self.peek().span;
                    let field = self.parse_ident().unwrap_or(Ident { name: String::new(), span: field_start });
                    let span = e.span().union(field.span);
                    e = Expr::QuestionDot(Box::new(e), field, span);
                }
                TokenKind::LParen => {
                    self.advance();
                    let args = self.parse_call_args();
                    self.expect(TokenKind::RParen);
                    let span = e.span().union(self.last_span());
                    e = Expr::Call(Box::new(e), args, span);
                }
                TokenKind::LBracket => {
                    self.advance();
                    let idx = self.parse_expr();
                    self.expect(TokenKind::RBracket);
                    let span = e.span().union(self.last_span());
                    e = Expr::Index(Box::new(e), Box::new(idx), span);
                }
                TokenKind::Bang => {
                    self.advance();
                    let span = e.span().union(self.last_span());
                    e = Expr::Bang(Box::new(e), span);
                }
                TokenKind::KwAs => {
                    self.advance();
                    let t = self.parse_type();
                    let span = e.span().union(t.span);
                    e = Expr::As(Box::new(e), t, span);
                }
                _ => break,
            }
        }
        e
    }

    fn parse_call_args(&mut self) -> Vec<CallArg> {
        let mut out = Vec::new();
        loop {
            if self.peek().kind == TokenKind::RParen { break; }
            if self.peek().kind == TokenKind::DotDotDot {
                let s = self.peek().span;
                self.advance();
                let e = self.parse_expr();
                out.push(CallArg::Spread(e, s));
            } else if self.peek().kind == TokenKind::Ident
                && self.tokens.get(self.pos + 1).map_or(false, |t| t.kind == TokenKind::Equal) {
                let name = self.parse_ident().unwrap();
                self.advance();
                let val = self.parse_expr();
                out.push(CallArg::Named(name, val));
            } else {
                let e = self.parse_expr();
                out.push(CallArg::Positional(e));
            }
            if self.peek().kind == TokenKind::Comma { self.advance(); }
            else { break; }
        }
        out
    }

    fn parse_primary(&mut self) -> Expr {
        let start = self.peek().span;
        match self.peek().kind {
            TokenKind::IntLit | TokenKind::FloatLit | TokenKind::StrLit | TokenKind::CharLit |
            TokenKind::KwTrue | TokenKind::KwFalse | TokenKind::KwNull | TokenKind::KwNil | TokenKind::RawStrLit |
            TokenKind::StrPart | TokenKind::InterpOpen => {
                self.parse_literal_expr()
            }
            TokenKind::LParen => {
                self.advance();
                if self.peek().kind == TokenKind::RParen {
                    self.advance();
                    return Expr::Unit(start);
                }
                let e = self.parse_expr();
                if self.peek().kind == TokenKind::Comma {
                    let mut elems = vec![e];
                    while self.peek().kind == TokenKind::Comma {
                        self.advance();
                        if self.peek().kind == TokenKind::RParen { break; }
                        elems.push(self.parse_expr());
                    }
                    self.expect(TokenKind::RParen);
                    let span = start.union(self.last_span());
                    return Expr::Tuple(elems, span);
                }
                self.expect(TokenKind::RParen);
                e
            }
            TokenKind::LBracket => {
                self.advance();
                let mut elems = Vec::new();
                loop {
                    if self.peek().kind == TokenKind::RBracket { break; }
                    elems.push(self.parse_expr());
                    if self.peek().kind == TokenKind::Comma { self.advance(); }
                    else { break; }
                }
                self.expect(TokenKind::RBracket);
                let span = start.union(self.last_span());
                Expr::Array(elems, span)
            }
            TokenKind::LBrace => {
                self.advance();
                let mut fields = Vec::new();
                loop {
                    if self.peek().kind == TokenKind::RBrace { break; }
                    let ks = self.peek().span;
                    let key = if self.peek().kind == TokenKind::StrLit {
                        let t = self.advance().clone();
                        Expr::StrLit(t.text, t.span)
                    } else if let Some(id) = self.parse_ident() {
                        Expr::Ident(id)
                    } else {
                        self.advance();
                        Expr::Unit(ks)
                    };
                    if self.peek().kind == TokenKind::Equal {
                        self.advance();
                        let value = self.parse_expr();
                        let span = key.span().union(value.span());
                        fields.push(MapField { key, value, span });
                    }
                    if self.peek().kind == TokenKind::Comma { self.advance(); }
                    else { break; }
                }
                self.expect(TokenKind::RBrace);
                let span = start.union(self.last_span());
                Expr::Map(fields, span)
            }
            TokenKind::KwFn => {
                self.advance();
                let generics = self.parse_generic_params();
                let _ = generics;
                self.expect(TokenKind::LParen);
                let params = self.parse_params();
                self.expect(TokenKind::RParen);
                let ret = if self.peek().kind == TokenKind::Arrow {
                    self.advance();
                    Some(self.parse_type())
                } else { None };
                let body = self.parse_block();
                let span = start.union(self.last_span());
                Expr::Closure(params, ret, Box::new(body), span)
            }
            TokenKind::KwIf => {
                self.advance();
                let cond = self.parse_expr();
                self.expect(TokenKind::KwThen);
                let then_body = self.parse_block();
                let mut elseifs = Vec::new();
                let mut else_body = None;
                while self.peek_kw(TokenKind::KwElseif) {
                    self.advance();
                    let c = self.parse_expr();
                    self.expect(TokenKind::KwThen);
                    let b = self.parse_block();
                    elseifs.push((c, b));
                }
                if self.peek_kw(TokenKind::KwElse) {
                    self.advance();
                    else_body = Some(self.parse_block());
                }
                let span = start.union(self.last_span());
                Expr::IfExpr(Box::new(cond), then_body, elseifs, else_body, span)
            }
            TokenKind::KwMatch => {
                self.advance();
                let scrutinee = self.parse_expr();
                let mut arms = Vec::new();
                while !self.at_end() && !self.peek_kw(TokenKind::KwEnd) {
                    if let Some(arm) = self.parse_match_arm() {
                        arms.push(arm);
                    } else { break; }
                }
                self.expect(TokenKind::KwEnd);
                let span = start.union(self.last_span());
                Expr::MatchExpr(Box::new(scrutinee), arms, span)
            }
            TokenKind::KwSelfType => {
                let t = self.advance().clone();
                if t.text == "self" {
                    Expr::SelfExpr(t.span)
                } else {
                    Expr::Ident(Ident { name: t.text, span: t.span })
                }
            }
            TokenKind::Ident => {
                let id = self.parse_ident().unwrap();
                if self.peek().kind == TokenKind::LBrace {
                    self.advance();
                    let fields = self.parse_struct_lit_fields();
                    let span = start.union(self.last_span());
                    return Expr::StructLit(id, fields, span);
                }
                Expr::Ident(id)
            }
            TokenKind::KwDo => {
                let body = self.parse_block();
                let span = start.union(self.last_span());
                Expr::BlockExpr(body, span)
            }
            _ => {
                self.error(format!("unexpected token {:?} in expression", self.peek().kind), self.peek().span);
                self.advance();
                Expr::Unit(start)
            }
        }
    }

    /// Parse the `field = expr, ...` list of a struct literal (assumes `{` consumed).
    fn parse_struct_lit_fields(&mut self) -> Vec<MapField> {
        let mut fields = Vec::new();
        loop {
            if self.peek().kind == TokenKind::RBrace { break; }
            if self.at_end() {
                self.error("expected `}` to close struct literal", self.peek().span);
                break;
            }
            let key = match self.parse_ident() {
                Some(id) => Expr::Ident(id),
                None => {
                    self.error("expected field name in struct literal", self.peek().span);
                    self.advance();
                    break;
                }
            };
            if self.peek().kind == TokenKind::Equal || self.peek().kind == TokenKind::Colon {
                self.advance();
                let value = self.parse_expr();
                let span = key.span().union(value.span());
                fields.push(MapField { key, value, span });
            } else {
                self.error("expected `=` after struct literal field", self.peek().span);
                break;
            }
            if self.peek().kind == TokenKind::Comma || self.peek().kind == TokenKind::Semicolon {
                self.advance();
            } else {
                break;
            }
        }
        self.expect(TokenKind::RBrace);
        fields
    }

    fn parse_ident(&mut self) -> Option<Ident> {
        let t = self.peek().clone();
        if t.kind == TokenKind::Ident || t.kind == TokenKind::KwSelfType {
            self.advance();
            return Some(Ident { name: t.text, span: t.span });
        }
        // Primitive type names can also be used as identifiers in patterns
        match t.kind {
            TokenKind::KwInt | TokenKind::KwUInt | TokenKind::KwFloat | TokenKind::KwDouble |
            TokenKind::KwBool | TokenKind::KwString | TokenKind::KwChar | TokenKind::KwByte |
            TokenKind::KwI8 | TokenKind::KwI16 | TokenKind::KwI32 | TokenKind::KwI64 | TokenKind::KwI128 |
            TokenKind::KwU8 | TokenKind::KwU16 | TokenKind::KwU32 | TokenKind::KwU64 | TokenKind::KwU128 |
            TokenKind::KwF32 | TokenKind::KwF64 | TokenKind::KwStr => {
                self.advance();
                Some(Ident { name: t.text, span: t.span })
            }
            _ => {
                self.error(format!("expected identifier, got {:?}", t.kind), t.span);
                None
            }
        }
    }

    fn peek_ident_or_str(&mut self) -> Option<String> {
        let t = self.peek().clone();
        if t.kind == TokenKind::Ident {
            Some(t.text)
        } else if t.kind == TokenKind::StrLit {
            Some(t.text)
        } else {
            None
        }
    }

    fn peek_string(&self) -> Option<String> {
        if self.peek().kind == TokenKind::StrLit { Some(self.peek().text.clone()) } else { None }
    }

    fn expect(&mut self, kind: TokenKind) {
        if self.peek().kind == kind {
            self.advance();
        } else {
            self.error(format!("expected {:?}, got {:?}", kind.describe(), self.peek().kind.describe()), self.peek().span);
        }
    }

    fn error(&mut self, msg: impl Into<String>, span: Span) {
        self.diagnostics.push(Diagnostic::error(msg, span));
    }

    fn peek(&self) -> &Token {
        self.tokens.get(self.pos).unwrap_or_else(|| {
            static LAST_TOKEN: std::sync::LazyLock<Token> = std::sync::LazyLock::new(|| Token {
                kind: TokenKind::Eof,
                span: Span::DUMMY,
                text: String::new(),
            });
            &LAST_TOKEN
        })
    }

    fn peek_kw(&self, kind: TokenKind) -> bool {
        self.peek().kind == kind
    }

    fn advance(&mut self) -> Token {
        let t = self.tokens.get(self.pos).cloned().unwrap_or(LAST_TOKEN.clone());
        if self.pos < self.tokens.len() { self.pos += 1; }
        t
    }

    fn last_span(&self) -> Span {
        if self.pos == 0 { Span::DUMMY } else { self.tokens[self.pos - 1].span }
    }

    fn at_end(&self) -> bool {
        self.peek().kind == TokenKind::Eof
    }
}

const LAST_TOKEN: Token = Token { kind: TokenKind::Eof, span: Span::DUMMY, text: String::new() };

fn empty_block(span: Span) -> Block {
    Block { stmts: Vec::new(), tail: None, span }
}

trait PatternSpan {
    fn span(&self) -> Span;
}

impl PatternSpan for Pattern {
    fn span(&self) -> Span {
        match self {
            Pattern::Wild(s) => *s,
            Pattern::Lit(_, s) => *s,
            Pattern::Binding(i) => i.span,
            Pattern::EnumVariant { span, .. } => *span,
            Pattern::Struct { span, .. } => *span,
            Pattern::Tuple(_, s) => *s,
            Pattern::Array(_, s) => *s,
            Pattern::Or(_, s) => *s,
            Pattern::As(_, _, s) => *s,
        }
    }
}

/// Convenience function: parse a source string into a Module.
pub fn parse(src: &str, file_id: u32) -> (Module, Diagnostics) {
    let (tokens, lex_diags) = crate::lexer::tokenize(src, file_id);
    let (module, parse_diags) = Parser::new(tokens, file_id).parse_module();
    let mut diags = lex_diags;
    diags.extend(parse_diags);
    (module, diags)
}
