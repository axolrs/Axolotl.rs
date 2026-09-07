// Owner: PascalElixir / axolrs (GitHub org)
// File: crates/bucket/src/lint.rs - the Neoten linter: static analysis over .axol sources.

use crate::build::collect_axol_files;
use crate::fmt::FmtError;
use crate::fmt::line_col_of;
use axolc_core::ast;
use axolc_core::diag::Severity;
use axolc_core::lexer::token::{Token, TokenKind};
use axolc_core::span::Span;
use std::collections::HashSet;

/// Warning-level finding: reported to the user, never fails the build.
pub const N0101: &str = "N0101";
/// Warning-level finding code for unused fn parameters.
pub const N0102: &str = "N0102";
/// Warning-level finding code for unused struct fields.
pub const N0103: &str = "N0103";
/// Warning-level finding code for unreachable code after `return`.
pub const N0104: &str = "N0104";
/// Warning-level finding code for self-assignments.
pub const N0105: &str = "N0105";
/// Warning-level finding code for empty block bodies.
pub const N0106: &str = "N0106";
/// Fallback code for surfaced front-end errors that carry no code of their own.
pub const E0000: &str = "E0000";

/// The severity of a lint finding as rendered in the report.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LintLevel {
    Warning,
    Error,
}

/// One lint finding: a code, a severity, a source span, and a message.
#[derive(Debug, Clone, PartialEq)]
pub struct Finding {
    pub level: LintLevel,
    pub code: String,
    pub span: Span,
    pub message: String,
}

/// One unreachable statement after a `return`, with its side-effect classification.
#[derive(Debug, Clone, PartialEq)]
pub struct UnreachableStmt {
    pub span: Span,
    pub pure: bool,
}

/// Construct a warning-level finding.
fn warn(code: &str, span: Span, message: String) -> Finding {
    Finding { level: LintLevel::Warning, code: code.to_string(), span, message }
}

/// Construct an error-level finding.
fn error(code: &str, span: Span, message: String) -> Finding {
    Finding { level: LintLevel::Error, code: code.to_string(), span, message }
}

/// Report whether a binding name is conventionally allowed to be unused.
fn is_intentionally_unused(name: &str) -> bool {
    name.starts_with('_')
}

/// Report whether a token kind can serve as an identifier-like name.
fn is_identish(k: TokenKind) -> bool {
    matches!(
        k,
        TokenKind::Ident
            | TokenKind::KwSelfType
            | TokenKind::KwInt
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
            | TokenKind::KwStr
    )
}

/// One declared binding tracked for usage inside a scope.
struct Binding {
    name: String,
    span: Span,
    is_param: bool,
    used: bool,
}

/// The AST-walking analyzer for the N0101, N0102, N0103, N0105, and N0106 checks.
struct Analyzer {
    findings: Vec<Finding>,
    field_reads: HashSet<String>,
    scopes: Vec<Vec<Binding>>,
    struct_fields: Vec<(String, Span, String)>,
}

impl Analyzer {
    /// Construct an empty analyzer.
    fn new() -> Analyzer {
        Analyzer { findings: Vec::new(), field_reads: HashSet::new(), scopes: Vec::new(), struct_fields: Vec::new() }
    }

    /// Push a fresh scope onto the scope stack.
    fn push_scope(&mut self) {
        self.scopes.push(Vec::new());
    }

    /// Pop the innermost scope, reporting its unused bindings.
    fn pop_scope(&mut self) {
        let scope = self.scopes.pop().unwrap_or_default();
        for b in scope {
            if b.used || b.name == "self" || is_intentionally_unused(&b.name) {
                continue;
            }
            if b.is_param {
                self.findings.push(warn(
                    N0102,
                    b.span,
                    format!("unused fn parameter `{}` (never read in the body)", b.name),
                ));
            } else {
                self.findings.push(warn(
                    N0101,
                    b.span,
                    format!("unused local `{}` (never read after declaration)", b.name),
                ));
            }
        }
    }

    /// Register a new binding in the innermost scope.
    fn declare(&mut self, name: &ast::Ident, is_param: bool) {
        let b = Binding {
            name: name.name.clone(),
            span: name.span,
            is_param,
            used: false,
        };
        if let Some(top) = self.scopes.last_mut() {
            top.push(b);
        }
    }

    /// Mark the innermost visible binding with this name as used.
    fn mark_used(&mut self, name: &str) {
        for scope in self.scopes.iter_mut().rev() {
            for b in scope.iter_mut().rev() {
                if b.name == name {
                    b.used = true;
                    return;
                }
            }
        }
    }

    /// Record a struct field read by field name for the N0103 check.
    fn read_field(&mut self, name: &str) {
        self.field_reads.insert(name.to_string());
    }

    /// Report unused struct fields after the whole module has been walked.
    fn report_unused_fields(&mut self) {
        let fields = std::mem::take(&mut self.struct_fields);
        for (_, span, name) in fields {
            if !self.field_reads.contains(&name) {
                self.findings.push(warn(
                    N0103,
                    span,
                    format!("unused struct field `{}` (never read in the module)", name),
                ));
            }
        }
    }

    /// Walk every item of a module.
    fn walk_module(&mut self, module: &ast::Module) {
        for item in &module.items {
            self.walk_item(item);
        }
    }

    /// Walk one top-level item.
    fn walk_item(&mut self, item: &ast::Item) {
        match item {
            ast::Item::Fn(f) => self.walk_fn(&f.name, &f.params, &f.body),
            ast::Item::Method(m) | ast::Item::StaticMethod(m) => {
                self.walk_fn(&m.name, &m.params, &m.body)
            }
            ast::Item::Interface(i) => {
                for sig in &i.methods {
                    if let Some(body) = &sig.default_body {
                        self.walk_fn(&sig.name, &sig.params, body);
                    }
                }
            }
            ast::Item::Struct(s) => {
                for field in &s.fields {
                    self.struct_fields
                        .push((s.name.name.clone(), field.name.span, field.name.name.clone()));
                    if let Some(d) = &field.default {
                        self.walk_expr(d);
                    }
                }
            }
            ast::Item::Enum(_) => {}
            ast::Item::Const(c) => self.walk_expr(&c.value),
            ast::Item::Use(..)
            | ast::Item::TypeAlias(..)
            | ast::Item::Impl(..)
            | ast::Item::ExternBlock(..)
            | ast::Item::CBlock(..)
            | ast::Item::CppBlock(..)
            | ast::Item::RBlock(..)
            | ast::Item::PyBlock(..)
            | ast::Item::ModuleDoc(..) => {}
        }
    }

    /// Walk a function-shaped item: parameters plus body in one shared scope.
    fn walk_fn(&mut self, _name: &ast::Ident, params: &[ast::Param], body: &ast::Block) {
        self.push_scope();
        for p in params {
            self.declare(&p.name, true);
        }
        for p in params {
            if let Some(d) = &p.default {
                self.walk_expr(d);
            }
        }
        self.walk_block(body);
        self.pop_scope();
    }

    /// Walk a block's statements and tail, checking for an empty body.
    fn walk_block(&mut self, block: &ast::Block) {
        if block.stmts.is_empty() && block.tail.is_none() {
            self.findings.push(warn(N0106, block.span, "empty block body".to_string()));
        }
        for stmt in &block.stmts {
            self.walk_stmt(stmt);
        }
        if let Some(tail) = &block.tail {
            self.walk_expr(tail);
        }
    }

    /// Walk one statement.
    fn walk_stmt(&mut self, stmt: &ast::Stmt) {
        match stmt {
            ast::Stmt::Let { name, value, .. } | ast::Stmt::Var { name, value, .. } => {
                let closure_valued = matches!(value, ast::Expr::Closure(..));
                self.walk_expr(value);
                self.declare(name, false);
                if closure_valued {
                    self.mark_used(&name.name);
                }
            }
            ast::Stmt::Assign { target, value, op, span } => {
                self.check_self_assign(target, value, *op, *span);
                self.walk_expr(value);
                let compound = !matches!(op, ast::AssignOp::Assign);
                self.walk_assign_target(target, compound);
            }
            ast::Stmt::Expr(e, _) => self.walk_expr(e),
            ast::Stmt::If { cond, then_body, elseifs, else_body, .. } => {
                self.walk_expr(cond);
                self.push_scope();
                self.walk_block(then_body);
                self.pop_scope();
                for (c, b) in elseifs {
                    self.walk_expr(c);
                    self.push_scope();
                    self.walk_block(b);
                    self.pop_scope();
                }
                if let Some(b) = else_body {
                    self.push_scope();
                    self.walk_block(b);
                    self.pop_scope();
                }
            }
            ast::Stmt::While { cond, body, .. } => {
                self.walk_expr(cond);
                self.push_scope();
                self.walk_block(body);
                self.pop_scope();
            }
            ast::Stmt::Repeat { body, cond, .. } => {
                self.push_scope();
                self.walk_block(body);
                self.walk_expr(cond);
                self.pop_scope();
            }
            ast::Stmt::For { var, iter, body, .. } => {
                self.walk_expr(iter);
                self.push_scope();
                self.declare(var, false);
                self.walk_block(body);
                self.pop_scope();
            }
            ast::Stmt::Loop { body, .. } => {
                self.push_scope();
                self.walk_block(body);
                self.pop_scope();
            }
            ast::Stmt::Break(_) | ast::Stmt::Continue(_) => {}
            ast::Stmt::Return(Some(e), _) => self.walk_expr(e),
            ast::Stmt::Return(None, _) => {}
            ast::Stmt::Match { scrutinee, arms, .. } => {
                self.walk_expr(scrutinee);
                for arm in arms {
                    self.push_scope();
                    self.walk_pattern(&arm.pattern);
                    if let Some(g) = &arm.guard {
                        self.walk_expr(g);
                    }
                    self.walk_block(&arm.body);
                    self.pop_scope();
                }
            }
            ast::Stmt::Block(b, _) => {
                self.push_scope();
                self.walk_block(b);
                self.pop_scope();
            }
            ast::Stmt::Unsafe(b, _) => {
                self.push_scope();
                self.walk_block(b);
                self.pop_scope();
            }
            ast::Stmt::Spawn { call, .. } => self.walk_expr(call),
        }
    }

    /// Report an N0105 finding when a plain assignment copies an identifier onto itself.
    fn check_self_assign(&mut self, target: &ast::Expr, value: &ast::Expr, op: ast::AssignOp, span: Span) {
        if !matches!(op, ast::AssignOp::Assign) {
            return;
        }
        if let (ast::Expr::Ident(t), ast::Expr::Ident(v)) = (target, value) {
            if t.name == v.name {
                let full = span.union(t.span).union(v.span);
                self.findings.push(warn(
                    N0105,
                    full,
                    format!("self-assignment `{} = {}` has no effect", t.name, v.name),
                ));
            }
        }
    }

    /// Walk an assignment target, treating the written field as a non-read.
    fn walk_assign_target(&mut self, target: &ast::Expr, reads_root: bool) {
        match target {
            ast::Expr::Ident(id) => {
                if reads_root {
                    self.mark_used(&id.name);
                }
            }
            ast::Expr::Field(base, _, _) => self.walk_expr(base),
            other => self.walk_expr(other),
        }
    }

    /// Walk one expression.
    fn walk_expr(&mut self, e: &ast::Expr) {
        match e {
            ast::Expr::Ident(id) => self.mark_used(&id.name),
            ast::Expr::SelfExpr(_) => self.mark_used("self"),
            ast::Expr::IntLit(..)
            | ast::Expr::FloatLit(..)
            | ast::Expr::StrLit(..)
            | ast::Expr::CharLit(..)
            | ast::Expr::RawStrLit(..)
            | ast::Expr::BoolLit(..)
            | ast::Expr::Null(_)
            | ast::Expr::Unit(_) => {}
            ast::Expr::InterpStr(parts, _) => {
                for part in parts {
                    if let ast::InterpStrPart::Expr(inner) = part {
                        self.walk_expr(inner);
                    }
                }
            }
            ast::Expr::Array(es, _) | ast::Expr::Tuple(es, _) => {
                for inner in es {
                    self.walk_expr(inner);
                }
            }
            ast::Expr::Map(fs, _) | ast::Expr::StructLit(_, fs, _) => {
                for f in fs {
                    self.walk_map_key(&f.key);
                    self.walk_expr(&f.value);
                }
            }
            ast::Expr::Range(lo, hi, _, _) => {
                if let Some(l) = lo {
                    self.walk_expr(l);
                }
                if let Some(h) = hi {
                    self.walk_expr(h);
                }
            }
            ast::Expr::BinOp(a, _, b, _) => {
                self.walk_expr(a);
                self.walk_expr(b);
            }
            ast::Expr::UnaryOp(_, inner, _) => self.walk_expr(inner),
            ast::Expr::Assign(target, op, value, span) => {
                self.check_self_assign(target, value, *op, *span);
                self.walk_expr(value);
                let compound = !matches!(op, ast::AssignOp::Assign);
                self.walk_assign_target(target, compound);
            }
            ast::Expr::Call(f, args, _) => {
                self.walk_expr(f);
                self.walk_args(args);
            }
            ast::Expr::MethodCall(recv, _, args, _, _) => {
                self.walk_expr(recv);
                self.walk_args(args);
            }
            ast::Expr::Field(base, name, _) => {
                self.walk_expr(base);
                self.read_field(&name.name);
            }
            ast::Expr::Index(base, idx, _) => {
                self.walk_expr(base);
                self.walk_expr(idx);
            }
            ast::Expr::Closure(params, _, body, _) => {
                self.push_scope();
                for p in params {
                    self.declare(&p.name, true);
                }
                for p in params {
                    if let Some(d) = &p.default {
                        self.walk_expr(d);
                    }
                }
                self.walk_block(body);
                self.pop_scope();
            }
            ast::Expr::IfExpr(cond, then_body, elseifs, else_body, _) => {
                self.walk_expr(cond);
                self.push_scope();
                self.walk_block(then_body);
                self.pop_scope();
                for (c, b) in elseifs {
                    self.walk_expr(c);
                    self.push_scope();
                    self.walk_block(b);
                    self.pop_scope();
                }
                if let Some(b) = else_body {
                    self.push_scope();
                    self.walk_block(b);
                    self.pop_scope();
                }
            }
            ast::Expr::MatchExpr(scrutinee, arms, _) => {
                self.walk_expr(scrutinee);
                for arm in arms {
                    self.push_scope();
                    self.walk_pattern(&arm.pattern);
                    if let Some(g) = &arm.guard {
                        self.walk_expr(g);
                    }
                    self.walk_block(&arm.body);
                    self.pop_scope();
                }
            }
            ast::Expr::BlockExpr(b, _) => {
                self.push_scope();
                self.walk_block(b);
                self.pop_scope();
            }
            ast::Expr::PipeForward(a, b, _) | ast::Expr::BackPipe(a, b, _) => {
                self.walk_expr(a);
                self.walk_expr(b);
            }
            ast::Expr::Await(inner, _)
            | ast::Expr::Try(inner, _)
            | ast::Expr::Bang(inner, _)
            | ast::Expr::Spread(inner, _) => self.walk_expr(inner),
            ast::Expr::QuestionDot(base, name, _) => {
                self.walk_expr(base);
                self.read_field(&name.name);
            }
            ast::Expr::Cast(inner, _, _) | ast::Expr::As(inner, _, _) => self.walk_expr(inner),
        }
    }

    /// Walk a map or struct-literal key, which is a name rather than a read.
    fn walk_map_key(&mut self, key: &ast::Expr) {
        match key {
            ast::Expr::Ident(_) | ast::Expr::StrLit(..) => {}
            other => self.walk_expr(other),
        }
    }

    /// Walk a call argument list, skipping named-argument labels.
    fn walk_args(&mut self, args: &[ast::CallArg]) {
        for arg in args {
            match arg {
                ast::CallArg::Positional(e) => self.walk_expr(e),
                ast::CallArg::Named(_, e) => self.walk_expr(e),
                ast::CallArg::Spread(e, _) => self.walk_expr(e),
            }
        }
    }

    /// Walk a match pattern, registering the bindings it introduces.
    fn walk_pattern(&mut self, p: &ast::Pattern) {
        match p {
            ast::Pattern::Wild(_) => {}
            ast::Pattern::Lit(e, _) => self.walk_expr(e),
            ast::Pattern::Binding(id) => self.declare(id, false),
            ast::Pattern::EnumVariant { sub, .. } => {
                for s in sub {
                    self.walk_pattern(s);
                }
            }
            ast::Pattern::Struct { fields, .. } => {
                for (fname, sub) in fields {
                    self.read_field(&fname.name);
                    self.walk_pattern(sub);
                }
            }
            ast::Pattern::Tuple(ps, _) | ast::Pattern::Array(ps, _) | ast::Pattern::Or(ps, _) => {
                for s in ps {
                    self.walk_pattern(s);
                }
            }
            ast::Pattern::As(inner, id, _) => {
                self.walk_pattern(inner);
                self.declare(id, false);
            }
        }
    }
}

/// Run the AST-based Neoten checks (N0101, N0102, N0103, N0105, N0106) over a parsed module.
pub fn analyze(module: &ast::Module) -> Vec<Finding> {
    let mut a = Analyzer::new();
    a.walk_module(module);
    a.report_unused_fields();
    a.findings
}

/// Report whether a `fn` token at index i is a function type rather than a closure or declaration.
fn is_fn_type(toks: &[&Token], i: usize) -> bool {
    if i > 0 {
        match toks[i - 1].kind {
            TokenKind::Colon | TokenKind::Less | TokenKind::Arrow => return true,
            _ => {}
        }
    }
    if toks.get(i + 1).map_or(true, |t| t.kind != TokenKind::LParen) {
        return false;
    }
    let mut j = match_bracket(toks, i + 1);
    if toks.get(j).map_or(false, |t| t.kind == TokenKind::Arrow) {
        j += 1;
        j = consume_type_idx(toks, j);
    }
    matches!(
        toks.get(j).map_or(TokenKind::Eof, |t| t.kind),
        TokenKind::Equal
            | TokenKind::Comma
            | TokenKind::Greater
            | TokenKind::RParen
            | TokenKind::RBracket
            | TokenKind::RBrace
            | TokenKind::Semicolon
            | TokenKind::Eof
    )
}

/// Return the index just past the bracket pair starting at index i.
fn match_bracket(toks: &[&Token], i: usize) -> usize {
    let mut depth = 1usize;
    let mut j = i + 1;
    while j < toks.len() {
        match toks[j].kind {
            TokenKind::LParen | TokenKind::LBracket | TokenKind::LBrace => depth += 1,
            TokenKind::RParen | TokenKind::RBracket | TokenKind::RBrace => {
                depth -= 1;
                if depth == 0 {
                    return j + 1;
                }
            }
            _ => {}
        }
        j += 1;
    }
    toks.len()
}

/// Skip one type annotation starting at index i, returning the index just past it.
fn consume_type_idx(toks: &[&Token], i: usize) -> usize {
    let k = toks.get(i).map_or(TokenKind::Eof, |t| t.kind);
    let j = match k {
        TokenKind::Ampersand => {
            let mut j = i + 1;
            if toks.get(j).map_or(false, |t| t.kind == TokenKind::KwVar) {
                j += 1;
            }
            consume_type_idx(toks, j)
        }
        TokenKind::KwBorrow | TokenKind::KwMove => consume_type_idx(toks, i + 1),
        TokenKind::LParen | TokenKind::LBracket => match_bracket(toks, i),
        TokenKind::KwFn => {
            let mut j = i + 1;
            if toks.get(j).map_or(false, |t| t.kind == TokenKind::LParen) {
                j = match_bracket(toks, j);
            }
            if toks.get(j).map_or(false, |t| t.kind == TokenKind::Arrow) {
                j = consume_type_idx(toks, j + 1);
            }
            j
        }
        TokenKind::Ident => {
            let mut j = i + 1;
            if toks.get(j).map_or(false, |t| t.kind == TokenKind::Less) {
                let mut depth = 1usize;
                j += 1;
                while j < toks.len() && depth > 0 {
                    match toks[j].kind {
                        TokenKind::Less => depth += 1,
                        TokenKind::Greater => depth -= 1,
                        _ => {}
                    }
                    j += 1;
                }
            }
            j
        }
        _ => i + 1,
    };
    if toks.get(j).map_or(false, |t| t.kind == TokenKind::Question) {
        return j + 1;
    }
    j
}

/// Report whether a token kind continues an expression as a binary operator.
fn is_binary_op(k: TokenKind) -> bool {
    matches!(
        k,
        TokenKind::Plus
            | TokenKind::Minus
            | TokenKind::Star
            | TokenKind::Slash
            | TokenKind::Percent
            | TokenKind::Ampersand
            | TokenKind::Pipe
            | TokenKind::Caret
            | TokenKind::ShiftLeft
            | TokenKind::ShiftRight
            | TokenKind::DoubleEqual
            | TokenKind::NotEqual
            | TokenKind::LuaNotEqual
            | TokenKind::Less
            | TokenKind::LessEqual
            | TokenKind::Greater
            | TokenKind::GreaterEqual
            | TokenKind::KwAnd
            | TokenKind::KwOr
            | TokenKind::DotDot
            | TokenKind::PipeForward
            | TokenKind::BackPipe
            | TokenKind::DoubleQuestion
    )
}

/// Report whether a token kind is a compound or plain assignment operator.
fn is_assign_op(k: TokenKind) -> bool {
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

/// Consume one unary-prefixed atom, returning the index just past it.
fn consume_unary(toks: &[&Token], i: usize, nl: &[bool]) -> usize {
    let k = toks.get(i).map_or(TokenKind::Eof, |t| t.kind);
    match k {
        TokenKind::Minus | TokenKind::Bang | TokenKind::Tilde | TokenKind::KwNot => {
            consume_unary(toks, i + 1, nl)
        }
        TokenKind::Ampersand => {
            if toks.get(i + 1).map_or(false, |t| t.kind == TokenKind::KwVar) {
                consume_unary(toks, i + 2, nl)
            } else {
                consume_unary(toks, i + 1, nl)
            }
        }
        TokenKind::KwBorrow | TokenKind::KwMove | TokenKind::KwTry | TokenKind::KwAwait | TokenKind::KwSpawn => {
            consume_unary(toks, i + 1, nl)
        }
        _ => consume_atom(toks, i, nl),
    }
}

/// Consume one expression atom, returning the index just past it.
fn consume_atom(toks: &[&Token], i: usize, nl: &[bool]) -> usize {
    let k = toks.get(i).map_or(TokenKind::Eof, |t| t.kind);
    match k {
        TokenKind::IntLit
        | TokenKind::FloatLit
        | TokenKind::StrLit
        | TokenKind::CharLit
        | TokenKind::RawStrLit
        | TokenKind::BytesLit
        | TokenKind::KwTrue
        | TokenKind::KwFalse
        | TokenKind::KwNull
        | TokenKind::KwNil
        | TokenKind::KwSelfType
        | TokenKind::Ident
        | TokenKind::InterpOpen
        | TokenKind::StrPart => {
            if k == TokenKind::Ident
                && toks.get(i + 1).map_or(false, |t| t.kind == TokenKind::LBrace)
            {
                return match_bracket(toks, i + 1);
            }
            if k == TokenKind::InterpOpen || k == TokenKind::StrPart {
                return consume_interp_string(toks, i);
            }
            i + 1
        }
        TokenKind::LParen | TokenKind::LBracket | TokenKind::LBrace => match_bracket(toks, i),
        TokenKind::KwFn => consume_closure(toks, i),
        TokenKind::KwIf => consume_if_chain(toks, i + 1, nl),
        TokenKind::KwMatch => {
            let j = consume_expr(toks, i + 1, nl);
            scan_block_end(toks, j) + 1
        }
        TokenKind::KwDo => scan_block_end(toks, i + 1) + 1,
        _ => i + 1,
    }
}

/// Consume an interpolated string's parts, returning the index just past it.
fn consume_interp_string(toks: &[&Token], i: usize) -> usize {
    let mut j = i;
    loop {
        match toks.get(j).map_or(TokenKind::Eof, |t| t.kind) {
            TokenKind::StrPart => j += 1,
            TokenKind::InterpOpen => {
                j += 1;
                while j < toks.len() && toks[j].kind != TokenKind::InterpClose {
                    j += 1;
                }
                if j < toks.len() {
                    j += 1;
                }
            }
            TokenKind::StrLit => return j + 1,
            _ => return j,
        }
    }
}

/// Consume a closure literal `fn(params) -> T body end`, returning the index just past its end.
fn consume_closure(toks: &[&Token], i: usize) -> usize {
    let mut j = i + 1;
    if toks.get(j).map_or(false, |t| t.kind == TokenKind::LParen) {
        j = match_bracket(toks, j);
    }
    if toks.get(j).map_or(false, |t| t.kind == TokenKind::Arrow) {
        j = consume_type_idx(toks, j + 1);
    }
    scan_block_end(toks, j) + 1
}

/// Consume `cond then body (elseif cond then body)* (else body)? end` from the cond onward.
fn consume_if_chain(toks: &[&Token], i: usize, nl: &[bool]) -> usize {
    let mut j = consume_expr(toks, i, nl);
    if toks.get(j).map_or(false, |t| t.kind == TokenKind::KwThen) {
        j = scan_block_end(toks, j + 1);
    }
    loop {
        match toks.get(j).map_or(TokenKind::Eof, |t| t.kind) {
            TokenKind::KwElseif => {
                j = consume_expr(toks, j + 1, nl);
                if toks.get(j).map_or(false, |t| t.kind == TokenKind::KwThen) {
                    j = scan_block_end(toks, j + 1);
                }
            }
            TokenKind::KwElse => return scan_block_end(toks, j + 1) + 1,
            _ => return j + 1,
        }
    }
}

/// Consume one full expression starting at index i, returning the index just past it.
fn consume_expr(toks: &[&Token], i: usize, nl: &[bool]) -> usize {
    let mut i = consume_unary(toks, i, nl);
    loop {
        let k = toks.get(i).map_or(TokenKind::Eof, |t| t.kind);
        if k == TokenKind::DotDot {
            i += 1;
            if toks.get(i).map_or(false, |t| t.kind == TokenKind::Equal) {
                i += 1;
            }
            i = consume_unary(toks, i, nl);
            continue;
        }
        if is_binary_op(k) {
            i = consume_unary(toks, i + 1, nl);
            continue;
        }
        if is_assign_op(k) {
            i = consume_expr(toks, i + 1, nl);
            continue;
        }
        let starts_new_line = nl.get(i).copied().unwrap_or(true);
        match k {
            TokenKind::Dot | TokenKind::Colon | TokenKind::DoubleColon | TokenKind::QuestionDot => {
                if starts_new_line {
                    return i;
                }
                if i + 1 < toks.len() && is_identish(toks[i + 1].kind) {
                    i += 2;
                    continue;
                }
                return i;
            }
            TokenKind::LParen | TokenKind::LBracket | TokenKind::LBrace => {
                if starts_new_line {
                    return i;
                }
                i = match_bracket(toks, i);
            }
            TokenKind::Bang | TokenKind::Question => {
                if starts_new_line {
                    return i;
                }
                i += 1;
            }
            TokenKind::KwAs | TokenKind::KwIs => i = consume_type_idx(toks, i + 1),
            _ => return i,
        }
    }
}

/// Scan a block body starting at depth 1, returning the index of its closing token.
fn scan_block_end(toks: &[&Token], i: usize) -> usize {
    let mut depth = 1usize;
    let mut j = i;
    while j < toks.len() {
        match toks[j].kind {
            TokenKind::KwThen
            | TokenKind::KwDo
            | TokenKind::KwLoop
            | TokenKind::KwRepeat
            | TokenKind::KwUnsafe
            | TokenKind::KwMatch
            | TokenKind::KwStruct
            | TokenKind::KwEnum
            | TokenKind::KwInterface => depth += 1,
            TokenKind::KwEnd | TokenKind::KwUntil | TokenKind::KwElseif => {
                depth -= 1;
                if depth == 0 {
                    return j;
                }
            }
            TokenKind::KwElse => {
                if depth == 1 {
                    return j;
                }
            }
            TokenKind::KwFn => {
                if !is_fn_type(toks, j) {
                    depth += 1;
                }
            }
            _ => {}
        }
        j += 1;
    }
    toks.len()
}

/// Report whether a statement keyword opens a block tracked by the main scanner.
fn pushes_block(k: TokenKind) -> bool {
    matches!(
        k,
        TokenKind::KwThen
            | TokenKind::KwDo
            | TokenKind::KwLoop
            | TokenKind::KwRepeat
            | TokenKind::KwUnsafe
            | TokenKind::KwMatch
            | TokenKind::KwStruct
            | TokenKind::KwEnum
            | TokenKind::KwInterface
    )
}

/// Report whether a token kind is a statement-leading keyword.
fn is_stmt_keyword(k: TokenKind) -> bool {
    matches!(
        k,
        TokenKind::KwLet
            | TokenKind::KwVar
            | TokenKind::KwIf
            | TokenKind::KwWhile
            | TokenKind::KwFor
            | TokenKind::KwLoop
            | TokenKind::KwMatch
            | TokenKind::KwRepeat
            | TokenKind::KwBreak
            | TokenKind::KwContinue
            | TokenKind::KwReturn
            | TokenKind::KwSpawn
            | TokenKind::KwUnsafe
    )
}

/// Report whether the token after a `return` keyword can begin its value expression.
fn return_value_starts(k: TokenKind) -> bool {
    !matches!(
        k,
        TokenKind::KwEnd
            | TokenKind::KwElse
            | TokenKind::KwElseif
            | TokenKind::KwUntil
            | TokenKind::Eof
            | TokenKind::Semicolon
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
    )
}

/// Report whether an identifier token opens a foreign FFI block.
fn is_foreign_block(toks: &[&Token], i: usize) -> bool {
    let text = toks.get(i).map_or("", |t| t.text.as_str());
    matches!(text, "cblock" | "cppblock" | "rblock" | "pyblock")
        && toks.get(i + 1).map_or(false, |t| t.kind == TokenKind::LBrace)
}

/// One tracked block during the unreachable-code scan.
struct BlockCtx {
    returned_from: Option<usize>,
    suppress_regions: bool,
}

/// Compute the unreachable-statement spans of a source string (the N0104 check).
pub fn scan_unreachable_stmts(src: &str, file_id: u32) -> Result<Vec<UnreachableStmt>, FmtError> {
    let (tokens, diags) = axolc_core::tokenize(src, file_id);
    if let Some(d) = diags.items.iter().find(|d| d.severity == Severity::Error) {
        return Err(FmtError { message: d.message.clone(), span: d.span });
    }
    let toks: Vec<&Token> = tokens
        .iter()
        .filter(|t| t.kind != TokenKind::Eof && t.kind != TokenKind::DocComment)
        .collect();
    let nl = compute_line_starts(src, &toks);
    let mut stack: Vec<BlockCtx> = Vec::new();
    let mut regions: Vec<(usize, usize)> = Vec::new();
    let mut i = 0usize;
    while i < toks.len() {
        let k = toks[i].kind;
        if k == TokenKind::KwFn && !is_fn_type(&toks, i) {
            stack.push(BlockCtx { returned_from: None, suppress_regions: false });
            i += 1;
            continue;
        }
        if pushes_block(k) {
            let suppress = matches!(
                k,
                TokenKind::KwMatch | TokenKind::KwStruct | TokenKind::KwEnum | TokenKind::KwInterface
            );
            stack.push(BlockCtx { returned_from: None, suppress_regions: suppress });
            i += 1;
            continue;
        }
        if k == TokenKind::KwElseif {
            if let Some(ctx) = stack.pop() {
                if let Some(rs) = ctx.returned_from {
                    regions.push((rs, i));
                }
            }
            i += 1;
            continue;
        }
        if k == TokenKind::KwElse {
            if let Some(ctx) = stack.pop() {
                if let Some(rs) = ctx.returned_from {
                    regions.push((rs, i));
                }
            }
            stack.push(BlockCtx { returned_from: None, suppress_regions: false });
            i += 1;
            continue;
        }
        if k == TokenKind::KwEnd || k == TokenKind::KwUntil {
            if let Some(ctx) = stack.pop() {
                if let Some(rs) = ctx.returned_from {
                    regions.push((rs, i));
                }
            }
            i += 1;
            continue;
        }
        if k == TokenKind::KwReturn {
            let next = toks.get(i + 1).map_or(TokenKind::Eof, |t| t.kind);
            let next_starts_line = gap_has_newline(src, &toks, i, i + 1);
            let after = if return_value_continues(next, next_starts_line) {
                consume_expr(&toks, i + 1, &nl)
            } else {
                i + 1
            };
            if let Some(top) = stack.last_mut() {
                if top.returned_from.is_none() && !top.suppress_regions {
                    top.returned_from = Some(after);
                }
            }
            i += 1;
            continue;
        }
        if k == TokenKind::Ident && is_foreign_block(&toks, i) {
            i = match_bracket(&toks, i + 1);
            continue;
        }
        i += 1;
    }
    let mut out = Vec::new();
    for (rs, re) in regions {
        if rs < re {
            out.extend(chunk_region(&toks, rs, re, file_id, &nl));
        }
    }
    Ok(out)
}

/// Report whether the source between two adjacent tokens contains a line break.
fn gap_has_newline(src: &str, toks: &[&Token], a: usize, b: usize) -> bool {
    match (toks.get(a), toks.get(b)) {
        (Some(x), Some(y)) => src[x.span.end as usize..y.span.start as usize].contains('\n'),
        _ => true,
    }
}

/// Mark which token indices begin a new source line.
fn compute_line_starts(src: &str, toks: &[&Token]) -> Vec<bool> {
    let mut out = Vec::with_capacity(toks.len());
    let mut prev_end: Option<u32> = None;
    for t in toks {
        let starts = match prev_end {
            Some(pe) => {
                let a = pe as usize;
                let b = t.span.start as usize;
                if a < b && b <= src.len() {
                    src[a..b].contains('\n')
                } else {
                    false
                }
            }
            None => true,
        };
        out.push(starts);
        prev_end = Some(t.span.end.max(t.span.start));
    }
    out
}

/// Split one unreachable region into per-statement chunks.
fn chunk_region(toks: &[&Token], rs: usize, re: usize, file_id: u32, nl: &[bool]) -> Vec<UnreachableStmt> {
    let mut out = Vec::new();
    let mut i = rs;
    while i < re {
        if toks[i].kind == TokenKind::Semicolon {
            i += 1;
            continue;
        }
        let start = i;
        i = consume_stmt(toks, i, re, nl);
        if i < re && toks[i].kind == TokenKind::Semicolon {
            i += 1;
        }
        if i <= start {
            i = start + 1;
        }
        let end = i.min(re);
        let span = Span::new(toks[start].span.start, toks[end - 1].span.end, file_id);
        out.push(UnreachableStmt { span, pure: chunk_is_pure(toks, start, end) });
    }
    out
}

/// Consume one statement starting at index i, bounded by the region end.
fn consume_stmt(toks: &[&Token], i: usize, limit: usize, nl: &[bool]) -> usize {
    let k = toks.get(i).map_or(TokenKind::Eof, |t| t.kind);
    match k {
        TokenKind::KwLet | TokenKind::KwVar => {
            let mut j = i + 1;
            if j < limit && is_identish(toks[j].kind) {
                j += 1;
            }
            if j < limit && toks[j].kind == TokenKind::Colon {
                j = consume_type_idx(toks, j + 1);
            }
            if j < limit && toks[j].kind == TokenKind::Equal {
                j = consume_expr(toks, j + 1, nl);
            }
            j.min(limit)
        }
        TokenKind::KwIf => consume_if_chain(toks, i + 1, nl),
        TokenKind::KwWhile => {
            let j = consume_expr(toks, i + 1, nl);
            if toks.get(j).map_or(false, |t| t.kind == TokenKind::KwDo) {
                (scan_block_end(toks, j + 1) + 1).min(limit)
            } else {
                (scan_block_end(toks, j) + 1).min(limit)
            }
        }
        TokenKind::KwFor => {
            let mut j = i + 1;
            if j < limit && is_identish(toks[j].kind) {
                j += 1;
            }
            if j < limit && toks[j].kind == TokenKind::KwIn {
                j = consume_expr(toks, j + 1, nl);
            }
            if j < limit && toks[j].kind == TokenKind::KwDo {
                j = scan_block_end(toks, j + 1) + 1;
            } else if j < limit {
                j = scan_block_end(toks, j) + 1;
            }
            j.min(limit)
        }
        TokenKind::KwLoop | TokenKind::KwUnsafe => (scan_block_end(toks, i + 1) + 1).min(limit),
        TokenKind::KwRepeat => {
            let j = (scan_block_end(toks, i + 1) + 1).min(limit);
            consume_expr(toks, j, nl).min(limit)
        }
        TokenKind::KwMatch => {
            let j = consume_expr(toks, i + 1, nl);
            (scan_block_end(toks, j) + 1).min(limit)
        }
        TokenKind::KwBreak | TokenKind::KwContinue => (i + 1).min(limit),
        TokenKind::KwReturn => {
            let next = toks.get(i + 1).map_or(TokenKind::Eof, |t| t.kind);
            let next_starts_line = nl.get(i + 1).copied().unwrap_or(true);
            if return_value_continues(next, next_starts_line) {
                consume_expr(toks, i + 1, nl).min(limit)
            } else {
                (i + 1).min(limit)
            }
        }
        TokenKind::KwSpawn => consume_expr(toks, i + 1, nl).min(limit),
        _ => consume_expr(toks, i, nl).min(limit),
    }
}

/// Report whether a `return` value continues onto the next line via a leading operator.
fn return_value_continues(next: TokenKind, next_starts_line: bool) -> bool {
    if !return_value_starts(next) {
        return false;
    }
    if !next_starts_line {
        return true;
    }
    matches!(
        next,
        TokenKind::Minus
            | TokenKind::Bang
            | TokenKind::Tilde
            | TokenKind::KwNot
            | TokenKind::Ampersand
            | TokenKind::KwMove
            | TokenKind::KwBorrow
    )
}

/// Report whether an unreachable statement chunk is side-effect free.
fn chunk_is_pure(toks: &[&Token], start: usize, end: usize) -> bool {
    let k = toks[start].kind;
    if matches!(k, TokenKind::KwLet | TokenKind::KwVar) {
        let mut j = start + 1;
        if j < end && is_identish(toks[j].kind) {
            j += 1;
        }
        if j < end && toks[j].kind == TokenKind::Colon {
            j = consume_type_idx(toks, j + 1);
        }
        if j < end && toks[j].kind == TokenKind::Equal {
            j += 1;
        }
        return expr_is_pure(toks, j, end);
    }
    if is_stmt_keyword(k) {
        return false;
    }
    expr_is_pure(toks, start, end)
}

/// Report whether a token range, as an expression, is free of side effects.
fn expr_is_pure(toks: &[&Token], start: usize, end: usize) -> bool {
    let mut depth = 0usize;
    let mut prev = TokenKind::Eof;
    let mut i = start;
    while i < end {
        let k = toks[i].kind;
        match k {
            TokenKind::LParen | TokenKind::LBracket | TokenKind::LBrace => {
                if depth == 0
                    && k == TokenKind::LParen
                    && matches!(
                        prev,
                        TokenKind::Ident
                            | TokenKind::KwSelfType
                            | TokenKind::RParen
                            | TokenKind::RBracket
                            | TokenKind::InterpClose
                    )
                {
                    return false;
                }
                depth += 1;
            }
            TokenKind::RParen | TokenKind::RBracket | TokenKind::RBrace | TokenKind::InterpClose => {
                depth = depth.saturating_sub(1);
            }
            TokenKind::InterpOpen => return false,
            TokenKind::Colon | TokenKind::QuestionDot => return false,
            TokenKind::KwSpawn | TokenKind::KwAwait | TokenKind::KwTry | TokenKind::KwMove | TokenKind::KwBorrow | TokenKind::KwFn => {
                return false;
            }
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
            | TokenKind::ShiftRightEqual => {
                if depth == 0 {
                    return false;
                }
            }
            TokenKind::Bang => {
                if depth == 0
                    && matches!(
                        prev,
                        TokenKind::Ident
                            | TokenKind::KwSelfType
                            | TokenKind::RParen
                            | TokenKind::RBracket
                            | TokenKind::IntLit
                            | TokenKind::FloatLit
                            | TokenKind::StrLit
                            | TokenKind::CharLit
                            | TokenKind::RawStrLit
                    )
                {
                    return false;
                }
            }
            _ => {}
        }
        prev = k;
        i += 1;
    }
    depth == 0
}

/// Compute every Neoten finding for one source string: warnings plus surfaced front-end errors.
pub fn lint_source(src: &str, file_id: u32) -> Vec<Finding> {
    let mut out: Vec<Finding> = Vec::new();
    match scan_unreachable_stmts(src, file_id) {
        Ok(stmts) => {
            for u in stmts {
                out.push(warn(N0104, u.span, "unreachable code after `return`".to_string()));
            }
        }
        Err(e) => {
            out.push(error(E0000, e.span, e.message));
            return out;
        }
    }
    let (module, pdiags) = axolc_core::parse(src, file_id);
    for d in &pdiags.items {
        if d.severity == Severity::Error {
            let code = d.code.clone().unwrap_or_else(|| E0000.to_string());
            out.push(error(&code, d.span, d.message.clone()));
        }
    }
    if !pdiags.has_errors() {
        let (_, odiags) = axolc_core::ownership::infer(&module);
        for d in &odiags.items {
            if d.severity == Severity::Error {
                let code = d.code.clone().unwrap_or_else(|| E0000.to_string());
                out.push(error(&code, d.span, d.message.clone()));
            }
        }
        out.extend(analyze(&module));
    }
    out.sort_by_key(|f| (f.span.start, f.span.end));
    out
}

/// Render one finding as the canonical `file:line:col: level[code]: message` line.
pub fn render_finding(path: &std::path::Path, src: &str, finding: &Finding) -> String {
    let (line, col) = line_col_of(src, finding.span.start as usize);
    let level = match finding.level {
        LintLevel::Warning => "warning",
        LintLevel::Error => "error",
    };
    format!(
        "{}:{}:{}: {}[{}]: {}",
        path.display(),
        line,
        col,
        level,
        finding.code,
        finding.message
    )
}

/// Lint the project with Neoten (the `bucket lint` command).
pub fn lint_cmd() {
    let dir = std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
    let files = collect_axol_files(&dir);
    if files.is_empty() {
        println!("Neoten: no .axol files found");
        return;
    }
    let mut warnings = 0usize;
    let mut errors = 0usize;
    for (idx, f) in files.iter().enumerate() {
        let src = match std::fs::read_to_string(f) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Neoten: cannot read {}: {}", f.display(), e);
                errors += 1;
                continue;
            }
        };
        let findings = lint_source(&src, idx as u32);
        for finding in &findings {
            println!("{}", render_finding(f, &src, finding));
            match finding.level {
                LintLevel::Warning => warnings += 1,
                LintLevel::Error => errors += 1,
            }
        }
    }
    println!("Neoten: {} warnings, {} errors", warnings, errors);
    if errors > 0 {
        std::process::exit(1);
    }
}
