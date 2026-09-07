// Owner: PascalElixir / axolrs (GitHub org)
// File: Ownership inference - tracks moves, borrows, and writes through the AST; produces .axol-level diagnostics and per-function parameter modes.

use crate::ast::{self, CallArg, Expr, Item, Stmt, TypeAnnot, TypeAnnotKind};
use crate::diag::{Diagnostic, Diagnostics};
use crate::span::Span;
use std::collections::HashMap;

/// How a parameter is passed into a function.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ParamMode {
    /// Read-only reference: `borrow T` (or inferred read-only).
    Borrow,
    /// Read-write reference: `mut T` (or inferred write-through).
    BorrowMut,
    /// Takes ownership: `move T` (or inferred move-out).
    Move,
    /// Plain by-value (Copy semantics - primitives): the default.
    Value,
}

/// The inferred usage class of a parameter inside a function body.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Usage {
    None,
    Read,
    Write,
    MoveOut,
}

/// A move of a value into a `move`-taking function, recorded for later checks.
#[derive(Debug, Clone)]
pub struct MoveSite {
    pub name: String,
    pub callee: String,
    pub ident: Span,
    pub call: Span,
}

/// The result of ownership inference for one module.
#[derive(Debug, Clone, Default)]
pub struct OwnershipTable {
    /// Per function name: ordered list of (param name, resolved mode).
    pub fn_params: HashMap<String, Vec<(String, ParamMode)>>,
    /// Advisory inferred modes before explicit override, surfaced by Gills as hints.
    pub inferred_hints: HashMap<String, Vec<(String, ParamMode)>>,
    /// Every recorded move site, tagged with the enclosing function name.
    pub moves: Vec<(String, MoveSite)>,
}

impl OwnershipTable {
    /// Return the resolved mode for a function parameter, if known.
    pub fn param_mode(&self, fn_name: &str, param: &str) -> Option<ParamMode> {
        self.fn_params
            .get(fn_name)?
            .iter()
            .find(|(n, _)| n == param)
            .map(|(_, m)| *m)
    }

    /// Return all resolved modes for one function.
    pub fn modes_for(&self, fn_name: &str) -> Vec<(String, ParamMode)> {
        self.fn_params.get(fn_name).cloned().unwrap_or_default()
    }

    /// Return the advisory hints for one function.
    pub fn hints_for(&self, fn_name: &str) -> Vec<(String, ParamMode)> {
        self.inferred_hints.get(fn_name).cloned().unwrap_or_default()
    }
}

/// True when the annotation is an explicit borrow (`borrow T` / `&T` / `mut T`).
fn explicit_borrow(annot: &TypeAnnot) -> Option<bool> {
    match &annot.kind {
        TypeAnnotKind::Borrow(_, is_mut) => Some(*is_mut),
        _ => None,
    }
}

/// True when the annotation is an explicit move (`move T`).
fn explicit_move(annot: &TypeAnnot) -> bool {
    matches!(annot.kind, TypeAnnotKind::Move(_))
}

/// True when a type annotation denotes a primitive (Copy) type.
fn is_copy_annot(annot: &Option<TypeAnnot>) -> bool {
    annot.as_ref().is_some_and(|a| matches!(a.kind, TypeAnnotKind::Primitive(_)))
}

/// Infer ownership for a module: parameter modes, annotation verification,
/// and static move/borrow checking with .axol-level diagnostics.
pub fn infer(module: &ast::Module) -> (OwnershipTable, Diagnostics) {
    let mut table = OwnershipTable::default();
    let mut diags = Diagnostics::new();

    let functions = collect_functions(module);
    let move_fns = move_taking_functions(&functions);

    for (name, f) in &functions {
        let mut usage: Vec<(String, Usage, Option<TypeAnnot>)> = f
            .params
            .iter()
            .map(|p| (p.name.name.clone(), Usage::None, p.ty.clone()))
            .collect();
        let mut fn_moves: Vec<(String, MoveSite)> = Vec::new();
        classify_block(&f.body, &mut usage, &mut fn_moves, &move_fns, name);
        table.moves.extend(fn_moves);

        let mut resolved: Vec<(String, ParamMode)> = Vec::new();
        let mut hints: Vec<(String, ParamMode)> = Vec::new();
        for (pname, usage, annot) in &usage {
            let inferred = match usage {
                Usage::None | Usage::Read => ParamMode::Borrow,
                Usage::Write => ParamMode::BorrowMut,
                Usage::MoveOut => ParamMode::Move,
            };
            hints.push((pname.clone(), inferred));

            let mode = match (annot, usage) {
                (Some(a), _) if explicit_move(a) => ParamMode::Move,
                (Some(a), _) if let Some(is_mut) = explicit_borrow(a) => {
                    check_borrow_param(a, is_mut, *usage, &mut diags);
                    if is_mut { ParamMode::BorrowMut } else { ParamMode::Borrow }
                }
                (_, Usage::None) | (_, Usage::Read) => {
                    if is_copy_annot(annot) { ParamMode::Value } else { ParamMode::Borrow }
                }
                (_, Usage::Write) => {
                    if is_copy_annot(annot) { ParamMode::Value } else { ParamMode::BorrowMut }
                }
                (_, Usage::MoveOut) => ParamMode::Move,
            };
            resolved.push((pname.clone(), mode));
        }
        table.fn_params.insert(name.clone(), resolved);
        table.inferred_hints.insert(name.clone(), hints);
        check_returned_borrows(&f.ret, &f.body, &mut diags);
    }

    collect_closure_move_sites(&functions, &mut table.moves);
    check_use_after_move(&table.moves, &functions, &mut diags);
    check_move_out_of_borrow(&functions, &move_fns, &mut diags);
    (table, diags)
}

/// Map of function name to FnItem for every top-level function.
fn collect_functions(module: &ast::Module) -> HashMap<String, ast::FnItem> {
    let mut map = HashMap::new();
    for item in &module.items {
        if let Item::Fn(f) = item {
            map.insert(f.name.name.clone(), f.clone());
        }
    }
    map
}

/// Map of function name to indices of parameters declared with `move T`.
fn move_taking_functions(functions: &HashMap<String, ast::FnItem>) -> HashMap<String, Vec<usize>> {
    let mut map = HashMap::new();
    for (name, f) in functions {
        let idxs: Vec<usize> = f
            .params
            .iter()
            .enumerate()
            .filter(|(_, p)| p.ty.as_ref().is_some_and(explicit_move))
            .map(|(i, _)| i)
            .collect();
        if !idxs.is_empty() {
            map.insert(name.clone(), idxs);
        }
    }
    map
}

/// Fold one usage classification into the per-param usage list.
fn record(usage: &mut [(String, Usage, Option<TypeAnnot>)], name: &str, u: Usage) {
    for (n, slot, _) in usage.iter_mut() {
        if n == name {
            *slot = merge(*slot, u);
            return;
        }
    }
}

/// Merge two usage classes into the strongest one.
fn merge(a: Usage, b: Usage) -> Usage {
    use Usage::*;
    match (a, b) {
        (MoveOut, _) | (_, MoveOut) => MoveOut,
        (Write, _) | (_, Write) => Write,
        (Read, _) | (_, Read) => Read,
        (None, None) => None,
    }
}

/// Walk a block, recording parameter usage and moves.
fn classify_block(
    block: &ast::Block,
    usage: &mut Vec<(String, Usage, Option<TypeAnnot>)>,
    moves: &mut Vec<(String, MoveSite)>,
    move_fns: &HashMap<String, Vec<usize>>,
    current_fn: &str,
) {
    for stmt in &block.stmts {
        classify_stmt(stmt, usage, moves, move_fns, current_fn);
    }
    if let Some(tail) = &block.tail {
        classify_expr(tail, usage, moves, move_fns, current_fn);
    }
}

/// Walk one statement for ownership effects.
fn classify_stmt(
    stmt: &Stmt,
    usage: &mut Vec<(String, Usage, Option<TypeAnnot>)>,
    moves: &mut Vec<(String, MoveSite)>,
    move_fns: &HashMap<String, Vec<usize>>,
    current_fn: &str,
) {
    match stmt {
        Stmt::Let { value, .. } | Stmt::Var { value, .. } => {
            classify_expr(value, usage, moves, move_fns, current_fn);
        }
        Stmt::Assign { target, value, .. } => {
            if let Some(root) = root_ident(target) {
                record(usage, &root, Usage::Write);
            }
            classify_expr(value, usage, moves, move_fns, current_fn);
        }
        Stmt::Expr(e, _) => classify_expr(e, usage, moves, move_fns, current_fn),
        Stmt::If { cond, then_body, elseifs, else_body, .. } => {
            classify_expr(cond, usage, moves, move_fns, current_fn);
            classify_block(then_body, usage, moves, move_fns, current_fn);
            for (c, b) in elseifs {
                classify_expr(c, usage, moves, move_fns, current_fn);
                classify_block(b, usage, moves, move_fns, current_fn);
            }
            if let Some(b) = else_body {
                classify_block(b, usage, moves, move_fns, current_fn);
            }
        }
        Stmt::While { cond, body, .. } => {
            classify_expr(cond, usage, moves, move_fns, current_fn);
            classify_block(body, usage, moves, move_fns, current_fn);
        }
        Stmt::Repeat { body, cond, .. } => {
            classify_block(body, usage, moves, move_fns, current_fn);
            classify_expr(cond, usage, moves, move_fns, current_fn);
        }
        Stmt::For { var, iter, body, .. } => {
            classify_expr(iter, usage, moves, move_fns, current_fn);
            record(usage, &var.name, Usage::Read);
            classify_block(body, usage, moves, move_fns, current_fn);
        }
        Stmt::Loop { body, .. } => classify_block(body, usage, moves, move_fns, current_fn),
        Stmt::Break(_) | Stmt::Continue(_) => {}
        Stmt::Return(e, _) => {
            if let Some(e) = e {
                if let Some(root) = root_ident(e) {
                    record(usage, &root, Usage::MoveOut);
                }
                classify_expr(e, usage, moves, move_fns, current_fn);
            }
        }
        Stmt::Match { scrutinee, arms, .. } => {
            classify_expr(scrutinee, usage, moves, move_fns, current_fn);
            for arm in arms {
                classify_block(&arm.body, usage, moves, move_fns, current_fn);
                if let Some(g) = &arm.guard {
                    classify_expr(g, usage, moves, move_fns, current_fn);
                }
            }
        }
        Stmt::Block(b, _) => classify_block(b, usage, moves, move_fns, current_fn),
        Stmt::Unsafe(b, _) => classify_block(b, usage, moves, move_fns, current_fn),
        Stmt::Spawn { call, .. } => classify_expr(call, usage, moves, move_fns, current_fn),
    }
}

/// Walk one expression for ownership effects, recording moves into `move`-taking calls.
fn classify_expr(
    e: &Expr,
    usage: &mut Vec<(String, Usage, Option<TypeAnnot>)>,
    moves: &mut Vec<(String, MoveSite)>,
    move_fns: &HashMap<String, Vec<usize>>,
    current_fn: &str,
) {
    match e {
        Expr::Ident(id) => record(usage, &id.name, Usage::Read),
        Expr::Call(callee, args, call_span) => {
            if let Expr::Ident(fid) = &**callee {
                if let Some(idxs) = move_fns.get(&fid.name) {
                    for &i in idxs {
                        if let Some(CallArg::Positional(Expr::Ident(v)) | CallArg::Named(_, Expr::Ident(v))) =
                            args.get(i)
                        {
                            record(usage, &v.name, Usage::MoveOut);
                            moves.push((
                                current_fn.to_string(),
                                MoveSite {
                                    name: v.name.clone(),
                                    callee: fid.name.clone(),
                                    ident: v.span,
                                    call: *call_span,
                                },
                            ));
                        }
                    }
                }
            }
            classify_expr(callee, usage, moves, move_fns, current_fn);
            for a in args {
                if let Some(inner) = arg_expr(a) {
                    classify_expr(inner, usage, moves, move_fns, current_fn);
                }
            }
        }
        Expr::MethodCall(recv, _, args, _, _) => {
            classify_expr(recv, usage, moves, move_fns, current_fn);
            for a in args {
                if let Some(inner) = arg_expr(a) {
                    classify_expr(inner, usage, moves, move_fns, current_fn);
                }
            }
        }
        Expr::BinOp(l, _, r, _) => {
            classify_expr(l, usage, moves, move_fns, current_fn);
            classify_expr(r, usage, moves, move_fns, current_fn);
        }
        Expr::UnaryOp(_, inner, _) | Expr::Spread(inner, _) => {
            classify_expr(inner, usage, moves, move_fns, current_fn);
        }
        Expr::Assign(t, _, v, _) => {
            if let Some(root) = root_ident(t) {
                record(usage, &root, Usage::Write);
            }
            classify_expr(v, usage, moves, move_fns, current_fn);
        }
        Expr::Field(recv, _, _)
        | Expr::Index(recv, _, _)
        | Expr::Await(recv, _)
        | Expr::Try(recv, _)
        | Expr::Bang(recv, _)
        | Expr::QuestionDot(recv, _, _)
        | Expr::Cast(recv, _, _)
        | Expr::As(recv, _, _) => classify_expr(recv, usage, moves, move_fns, current_fn),
        Expr::Array(elems, _) | Expr::Tuple(elems, _) => {
            for el in elems {
                classify_expr(el, usage, moves, move_fns, current_fn);
            }
        }
        Expr::Map(fields, _) => {
            for f in fields {
                classify_expr(&f.key, usage, moves, move_fns, current_fn);
                classify_expr(&f.value, usage, moves, move_fns, current_fn);
            }
        }
        Expr::InterpStr(parts, _) => {
            for p in parts {
                if let ast::InterpStrPart::Expr(inner) = p {
                    classify_expr(inner, usage, moves, move_fns, current_fn);
                }
            }
        }
        Expr::Closure(_, _, body, _) => {
            let mut inner = usage.to_vec();
            classify_block(body, &mut inner, moves, move_fns, current_fn);
            for (n, u, _) in inner {
                record(usage, &n, u);
            }
        }
        Expr::IfExpr(c, t, ei, el, _) => {
            classify_expr(c, usage, moves, move_fns, current_fn);
            classify_block(t, usage, moves, move_fns, current_fn);
            for (c2, b) in ei {
                classify_expr(c2, usage, moves, move_fns, current_fn);
                classify_block(b, usage, moves, move_fns, current_fn);
            }
            if let Some(b) = el {
                classify_block(b, usage, moves, move_fns, current_fn);
            }
        }
        Expr::MatchExpr(scrut, arms, _) => {
            classify_expr(scrut, usage, moves, move_fns, current_fn);
            for arm in arms {
                classify_block(&arm.body, usage, moves, move_fns, current_fn);
            }
        }
        Expr::BlockExpr(b, _) => classify_block(b, usage, moves, move_fns, current_fn),
        Expr::PipeForward(l, r, _) | Expr::BackPipe(l, r, _) => {
            classify_expr(l, usage, moves, move_fns, current_fn);
            classify_expr(r, usage, moves, move_fns, current_fn);
        }
        Expr::Range(lo, hi, _, _) => {
            if let Some(l) = lo {
                classify_expr(l, usage, moves, move_fns, current_fn);
            }
            if let Some(h) = hi {
                classify_expr(h, usage, moves, move_fns, current_fn);
            }
        }
        _ => {}
    }
}

/// Extract the expression of a call argument.
fn arg_expr(a: &CallArg) -> Option<&Expr> {
    match a {
        CallArg::Positional(e) => Some(e),
        CallArg::Named(_, e) => Some(e),
        CallArg::Spread(e, _) => Some(e),
    }
}

/// Return the root identifier of an lvalue expression, if any.
fn root_ident(e: &Expr) -> Option<String> {
    match e {
        Expr::Ident(id) => Some(id.name.clone()),
        Expr::Field(recv, _, _) | Expr::Index(recv, _, _) => root_ident(recv),
        _ => None,
    }
}

/// Reject writes through an explicit `borrow T` parameter.
fn check_borrow_param(annot: &TypeAnnot, is_mut: bool, usage: Usage, diags: &mut Diagnostics) {
    if !is_mut && usage == Usage::Write {
        diags.push(
            Diagnostic::error(
                "parameter is written through but declared as `borrow`",
                annot.span,
            )
            .with_code("E0104")
            .with_note("use `mut T` to allow writes through the reference".to_string()),
        );
    }
}

/// Report E0103 when a local declared inside the function is returned by borrow.
fn check_returned_borrows(ret: &Option<TypeAnnot>, body: &ast::Block, diags: &mut Diagnostics) {
    let Some(ret) = ret else { return };
    if !matches!(ret.kind, TypeAnnotKind::Borrow(_, _)) {
        return;
    }
    let local_names: Vec<String> = body
        .stmts
        .iter()
        .filter_map(|s| match s {
            Stmt::Let { name, .. } | Stmt::Var { name, .. } => Some(name.name.clone()),
            _ => None,
        })
        .collect();
    for stmt in &body.stmts {
        if let Stmt::Return(Some(Expr::Ident(id)), span) = stmt {
            if local_names.contains(&id.name) {
                diags.push(
                    Diagnostic::error(
                        format!("cannot return reference to local variable `{}`", id.name),
                        *span,
                    )
                    .with_code("E0103")
                    .with_note(
                        "the local is destroyed when the function returns; return the value by move instead"
                            .to_string(),
                    ),
                );
            }
        }
    }
}

/// Report E0101 when a moved local is used after its move site.
fn check_use_after_move(
    moves: &[(String, MoveSite)],
    functions: &HashMap<String, ast::FnItem>,
    diags: &mut Diagnostics,
) {
    for (fname, f) in functions {
        let fn_moves: Vec<&MoveSite> =
            moves.iter().filter(|(n, _)| n == fname).map(|(_, m)| m).collect();
        if fn_moves.is_empty() {
            continue;
        }
        for stmt in &f.body.stmts {
            for m in &fn_moves {
                let mut sites: Vec<Span> = Vec::new();
                stmt_use_sites(stmt, &m.name, &mut sites);
                for site in sites {
                    if site.start > m.call.end {
                        diags.push(
                            Diagnostic::error(
                                format!("use of moved value `{}`", m.name),
                                site,
                            )
                            .with_code("E0101")
                            .with_note(format!(
                                "value moved into `{}` earlier in this function",
                                m.callee
                            )),
                        );
                    }
                }
            }
        }
        if let Some(tail) = &f.body.tail {
            for m in &fn_moves {
                let mut sites: Vec<Span> = Vec::new();
                expr_use_sites(tail, &m.name, &mut sites);
                for site in sites {
                    if site.start > m.call.end {
                        diags.push(
                            Diagnostic::error(
                                format!("use of moved value `{}`", m.name),
                                site,
                            )
                            .with_code("E0101")
                            .with_note(format!(
                                "value moved into `{}` earlier in this function",
                                m.callee
                            )),
                        );
                    }
                }
            }
        }
    }
}

/// Report E0102 when a `borrow`/`mut` parameter is passed by value into a `move`-taking function.
fn check_move_out_of_borrow(
    functions: &HashMap<String, ast::FnItem>,
    move_fns: &HashMap<String, Vec<usize>>,
    diags: &mut Diagnostics,
) {
    for (fname, f) in functions {
        let _ = fname;
        let mut call_sites: Vec<(&Expr, &Vec<CallArg>)> = Vec::new();
        for stmt in &f.body.stmts {
            if let Stmt::Expr(Expr::Call(callee, args, _), _) = stmt {
                call_sites.push((callee, args));
            }
        }
        if let Some(tail) = &f.body.tail {
            if let Expr::Call(callee, args, _) = &**tail {
                call_sites.push((callee, args));
            }
        }
        for (callee, args) in call_sites {
            let Expr::Ident(callee_id) = callee else { continue };
            let Some(idxs) = move_fns.get(&callee_id.name) else { continue };
            for &i in idxs {
                if let Some(CallArg::Positional(Expr::Ident(v)) | CallArg::Named(_, Expr::Ident(v))) =
                    args.get(i)
                {
                    let is_borrowed = f.params.iter().any(|p| {
                        p.name.name == v.name
                            && p.ty.as_ref().is_some_and(|t| {
                                matches!(t.kind, TypeAnnotKind::Borrow(_, _))
                            })
                    });
                    if is_borrowed {
                        diags.push(
                            Diagnostic::error(
                                format!("cannot move `{}` out of borrowed content", v.name),
                                v.span,
                            )
                            .with_code("E0102")
                            .with_note(format!(
                                "`{}` is only borrowed here; `move` requires ownership",
                                v.name
                            )),
                        );
                    }
                }
            }
        }
    }
}

/// True when the statement reads the given identifier anywhere in it.
/// Collect the spans of every identifier reference to `name` inside a statement.
fn stmt_use_sites(stmt: &Stmt, name: &str, out: &mut Vec<Span>) {
    match stmt {
        Stmt::Let { value, .. } | Stmt::Var { value, .. } => expr_use_sites(value, name, out),
        Stmt::Assign { target, value, .. } => {
            expr_use_sites(target, name, out);
            expr_use_sites(value, name, out);
        }
        Stmt::Expr(e, _) => expr_use_sites(e, name, out),
        Stmt::If { cond, then_body, elseifs, else_body, .. } => {
            expr_use_sites(cond, name, out);
            block_use_sites(then_body, name, out);
            for (c, b) in elseifs {
                expr_use_sites(c, name, out);
                block_use_sites(b, name, out);
            }
            if let Some(b) = else_body {
                block_use_sites(b, name, out);
            }
        }
        Stmt::While { cond, body, .. } => {
            expr_use_sites(cond, name, out);
            block_use_sites(body, name, out);
        }
        Stmt::Repeat { body, cond, .. } => {
            block_use_sites(body, name, out);
            expr_use_sites(cond, name, out);
        }
        Stmt::For { iter, body, .. } => {
            expr_use_sites(iter, name, out);
            block_use_sites(body, name, out);
        }
        Stmt::Loop { body, .. } => block_use_sites(body, name, out),
        Stmt::Return(e, _) => {
            if let Some(e) = e {
                expr_use_sites(e, name, out);
            }
        }
        Stmt::Match { scrutinee, arms, .. } => {
            expr_use_sites(scrutinee, name, out);
            for a in arms {
                block_use_sites(&a.body, name, out);
                if let Some(g) = &a.guard {
                    expr_use_sites(g, name, out);
                }
            }
        }
        Stmt::Block(b, _) | Stmt::Unsafe(b, _) => block_use_sites(b, name, out),
        Stmt::Spawn { call, .. } => expr_use_sites(call, name, out),
        Stmt::Break(_) | Stmt::Continue(_) => {}
    }
}

/// Collect the spans of every identifier reference to `name` inside a block.
fn block_use_sites(block: &ast::Block, name: &str, out: &mut Vec<Span>) {
    for stmt in &block.stmts {
        stmt_use_sites(stmt, name, out);
    }
    if let Some(tail) = &block.tail {
        expr_use_sites(tail, name, out);
    }
}

/// Collect the spans of every identifier reference to `name` inside an expression.
fn expr_use_sites(e: &Expr, name: &str, out: &mut Vec<Span>) {
    match e {
        Expr::Ident(id) => {
            if id.name == name {
                out.push(id.span);
            }
        }
        Expr::BinOp(l, _, r, _) => {
            expr_use_sites(l, name, out);
            expr_use_sites(r, name, out);
        }
        Expr::UnaryOp(_, inner, _) | Expr::Spread(inner, _) => expr_use_sites(inner, name, out),
        Expr::Call(c, args, _) => {
            expr_use_sites(c, name, out);
            for a in args {
                if let Some(x) = arg_expr(a) {
                    expr_use_sites(x, name, out);
                }
            }
        }
        Expr::MethodCall(recv, _, args, _, _) => {
            expr_use_sites(recv, name, out);
            for a in args {
                if let Some(x) = arg_expr(a) {
                    expr_use_sites(x, name, out);
                }
            }
        }
        Expr::Field(recv, _, _)
        | Expr::Index(recv, _, _)
        | Expr::Await(recv, _)
        | Expr::Try(recv, _)
        | Expr::Bang(recv, _)
        | Expr::QuestionDot(recv, _, _)
        | Expr::Cast(recv, _, _)
        | Expr::As(recv, _, _) => expr_use_sites(recv, name, out),
        Expr::Array(elems, _) | Expr::Tuple(elems, _) => {
            for el in elems {
                expr_use_sites(el, name, out);
            }
        }
        Expr::Map(fields, _) => {
            for f in fields {
                expr_use_sites(&f.key, name, out);
                expr_use_sites(&f.value, name, out);
            }
        }
        Expr::InterpStr(parts, _) => {
            for p in parts {
                if let ast::InterpStrPart::Expr(inner) = p {
                    expr_use_sites(inner, name, out);
                }
            }
        }
        Expr::Range(lo, hi, _, _) => {
            if let Some(l) = lo {
                expr_use_sites(l, name, out);
            }
            if let Some(h) = hi {
                expr_use_sites(h, name, out);
            }
        }
        _ => {}
    }
}


/// True when a method body assigns through `self` (drives `&mut self` codegen).
pub fn method_writes_self(body: &ast::Block) -> bool {
    block_writes_self(body)
}

/// Check every statement of a block for writes through `self`.
fn block_writes_self(block: &ast::Block) -> bool {
    block.stmts.iter().any(stmt_writes_self)
        || block.tail.as_ref().is_some_and(|t| expr_writes_self(t))
}

/// Check one statement for writes through `self`.
fn stmt_writes_self(stmt: &Stmt) -> bool {
    match stmt {
        Stmt::Assign { target, .. } => target_writes_self(target),
        Stmt::Expr(e, _) => expr_writes_self(e),
        Stmt::If { then_body, elseifs, else_body, .. } => {
            block_writes_self(then_body)
                || elseifs.iter().any(|(_, b)| block_writes_self(b))
                || else_body.as_ref().is_some_and(block_writes_self)
        }
        Stmt::While { body, .. }
        | Stmt::Repeat { body, .. }
        | Stmt::For { body, .. }
        | Stmt::Loop { body, .. }
        | Stmt::Block(body, _)
        | Stmt::Unsafe(body, _) => block_writes_self(body),
        Stmt::Match { arms, .. } => arms.iter().any(|a| block_writes_self(&a.body)),
        _ => false,
    }
}

/// Check an expression for writes through `self`.
fn expr_writes_self(e: &Expr) -> bool {
    match e {
        Expr::Assign(target, _, _, _) => target_writes_self(target),
        Expr::IfExpr(_, t, ei, el, _) => {
            block_writes_self(t)
                || ei.iter().any(|(_, b)| block_writes_self(b))
                || el.as_ref().is_some_and(|b| block_writes_self(b))
        }
        Expr::MatchExpr(_, arms, _) => arms.iter().any(|a| block_writes_self(&a.body)),
        Expr::BlockExpr(b, _) => block_writes_self(b),
        _ => false,
    }
}

/// True when an assignment target roots at `self`.
fn target_writes_self(target: &Expr) -> bool {
    match target {
        Expr::SelfExpr(_) => true,
        Expr::Field(recv, _, _) | Expr::Index(recv, _, _) => target_writes_self(recv),
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Explicit borrow annotation resolves to Borrow mode.
    #[test]
    fn borrow_param_gets_borrow_mode() {
        let (table, diags) = infer(&crate::parser::parse("fn f(x: borrow Int) print(x) end", 0).0);
        assert_eq!(table.param_mode("f", "x"), Some(ParamMode::Borrow));
        assert!(!diags.has_errors());
    }

    /// Explicit mut annotation resolves to BorrowMut mode.
    #[test]
    fn mut_param_gets_borrow_mut_mode() {
        let (table, _) = infer(&crate::parser::parse("fn f(x: mut Int) x = x + 1 end", 0).0);
        assert_eq!(table.param_mode("f", "x"), Some(ParamMode::BorrowMut));
    }

    /// Explicit move annotation resolves to Move mode.
    #[test]
    fn move_param_gets_move_mode() {
        let (table, _) = infer(&crate::parser::parse("fn f(x: move Int) g(x) end", 0).0);
        assert_eq!(table.param_mode("f", "x"), Some(ParamMode::Move));
    }

    /// Unannotated primitive params resolve to plain Value mode.
    #[test]
    fn primitive_param_gets_value_mode() {
        let (table, _) = infer(&crate::parser::parse("fn f(x: Int) print(x) end", 0).0);
        assert_eq!(table.param_mode("f", "x"), Some(ParamMode::Value));
    }

    /// Writing through a read-only borrow is an E0104 error.
    #[test]
    fn write_through_borrow_is_error() {
        let (_, diags) = infer(&crate::parser::parse("fn f(x: borrow Int) x = 1 end", 0).0);
        assert!(diags.items.iter().any(|d| d.code.as_deref() == Some("E0104")));
    }

    /// Returning a local by borrow is an E0103 error.
    #[test]
    fn dangling_return_is_error() {
        let (_, diags) =
            infer(&crate::parser::parse("fn f() -> borrow String let s = \"x\" return s end", 0).0);
        assert!(diags.items.iter().any(|d| d.code.as_deref() == Some("E0103")));
    }

    /// Using a local after moving it into a `move` function is an E0101 error.
    #[test]
    fn use_after_move_is_error() {
        let src = "fn take(x: move Int) print(x) end\nfn main() let a = 1\ntake(a)\nprint(a) end";
        let (_, diags) = infer(&crate::parser::parse(src, 0).0);
        assert!(
            diags.items.iter().any(|d| d.code.as_deref() == Some("E0101")),
            "expected E0101, got: {:?}",
            diags.items
        );
    }

    /// Moving a borrowed parameter out is an E0102 error.
    #[test]
    fn move_out_of_borrow_is_error() {
        let src = "fn take(x: move Int) print(x) end\nfn f(y: borrow Int) take(y) end";
        let (_, diags) = infer(&crate::parser::parse(src, 0).0);
        assert!(
            diags.items.iter().any(|d| d.code.as_deref() == Some("E0102")),
            "expected E0102, got: {:?}",
            diags.items
        );
    }

    /// Read-only params get a Borrow hint in the advisory table.
    #[test]
    fn read_only_param_gets_borrow_hint() {
        let (table, _) = infer(&crate::parser::parse("fn f(x: Int) print(x) end", 0).0);
        assert_eq!(table.hints_for("f").first().map(|(_, m)| *m), Some(ParamMode::Borrow));
    }

    /// Written-through params get a BorrowMut hint in the advisory table.
    #[test]
    fn write_param_gets_borrow_mut_hint() {
        let (table, _) = infer(&crate::parser::parse("fn f(x: Int) x = 2 end", 0).0);
        assert_eq!(table.hints_for("f").first().map(|(_, m)| *m), Some(ParamMode::BorrowMut));
    }
}

/// Model non-Copy closure captures as moves: a local moved into a closure
/// literal cannot be used afterwards, matching the `move` closure codegen and
/// the rustc backstop on the generated Rust.
fn collect_closure_move_sites(
    functions: &HashMap<String, ast::FnItem>,
    moves: &mut Vec<(String, MoveSite)>,
) {
    for (fname, f) in functions {
        let noncopy = noncopy_locals(&f.body);
        if noncopy.is_empty() {
            continue;
        }
        let mut sites: Vec<(String, MoveSite)> = Vec::new();
        collect_block_captures(&f.body, &noncopy, fname, &mut sites);
        moves.extend(sites);
    }
}

/// Names of locals initialized with non-Copy values (strings, collections,
/// structs), propagated through ident-to-ident copies in source order.
fn noncopy_locals(block: &ast::Block) -> std::collections::HashSet<String> {
    let mut noncopy = std::collections::HashSet::new();
    let mut copy = std::collections::HashSet::new();
    collect_block_kinds(block, &mut noncopy, &mut copy);
    noncopy
}

/// Record the Copy/non-Copy kind of every let/var binding in a block, in order.
fn collect_block_kinds(
    block: &ast::Block,
    noncopy: &mut std::collections::HashSet<String>,
    copy: &mut std::collections::HashSet<String>,
) {
    for stmt in &block.stmts {
        collect_stmt_kinds(stmt, noncopy, copy);
    }
}

/// Record kinds from one statement, recursing into nested blocks.
fn collect_stmt_kinds(
    stmt: &Stmt,
    noncopy: &mut std::collections::HashSet<String>,
    copy: &mut std::collections::HashSet<String>,
) {
    match stmt {
        Stmt::Let { name, value, .. } | Stmt::Var { name, value, .. } => {
            collect_expr_kinds(value, noncopy, copy);
            match init_kind(value, noncopy, copy) {
                Some(true) => {
                    noncopy.insert(name.name.clone());
                }
                Some(false) => {
                    copy.insert(name.name.clone());
                }
                None => {}
            }
        }
        Stmt::Assign { value, .. } => collect_expr_kinds(value, noncopy, copy),
        Stmt::Expr(e, _) => collect_expr_kinds(e, noncopy, copy),
        Stmt::If { then_body, elseifs, else_body, .. } => {
            collect_block_kinds(then_body, noncopy, copy);
            for (_, b) in elseifs {
                collect_block_kinds(b, noncopy, copy);
            }
            if let Some(b) = else_body {
                collect_block_kinds(b, noncopy, copy);
            }
        }
        Stmt::While { body, .. } | Stmt::Repeat { body, .. } | Stmt::Loop { body, .. } => {
            collect_block_kinds(body, noncopy, copy);
        }
        Stmt::For { body, .. } => collect_block_kinds(body, noncopy, copy),
        Stmt::Return(e, _) => {
            if let Some(e) = e {
                collect_expr_kinds(e, noncopy, copy);
            }
        }
        Stmt::Match { arms, .. } => {
            for arm in arms {
                collect_block_kinds(&arm.body, noncopy, copy);
            }
        }
        Stmt::Block(b, _) | Stmt::Unsafe(b, _) => collect_block_kinds(b, noncopy, copy),
        Stmt::Spawn { call, .. } => collect_expr_kinds(call, noncopy, copy),
        _ => {}
    }
}

/// Record kinds from expressions containing blocks.
fn collect_expr_kinds(
    e: &Expr,
    noncopy: &mut std::collections::HashSet<String>,
    copy: &mut std::collections::HashSet<String>,
) {
    match e {
        Expr::Closure(_, _, body, _) => collect_block_kinds(body, noncopy, copy),
        Expr::IfExpr(_, t, ei, el, _) => {
            collect_block_kinds(t, noncopy, copy);
            for (_, b) in ei {
                collect_block_kinds(b, noncopy, copy);
            }
            if let Some(b) = el {
                collect_block_kinds(b, noncopy, copy);
            }
        }
        Expr::MatchExpr(_, arms, _) => {
            for arm in arms {
                collect_block_kinds(&arm.body, noncopy, copy);
            }
        }
        Expr::BlockExpr(b, _) => collect_block_kinds(b, noncopy, copy),
        _ => {}
    }
}

/// Classify an initializer: Some(true) = non-Copy, Some(false) = Copy, None = unknown.
fn init_kind(
    value: &Expr,
    noncopy: &std::collections::HashSet<String>,
    copy: &std::collections::HashSet<String>,
) -> Option<bool> {
    match value {
        Expr::StrLit(..) | Expr::RawStrLit(..) | Expr::InterpStr(..) | Expr::Array(..)
        | Expr::Map(..) | Expr::Tuple(..) | Expr::StructLit(..) => Some(true),
        Expr::IntLit(..) | Expr::FloatLit(..) | Expr::BoolLit(..) | Expr::CharLit(..)
        | Expr::Unit(..) => Some(false),
        Expr::Ident(id) => {
            if noncopy.contains(&id.name) {
                Some(true)
            } else if copy.contains(&id.name) {
                Some(false)
            } else {
                None
            }
        }
        _ => None,
    }
}

/// Find closure literals in a block and record their non-Copy captures as moves.
fn collect_block_captures(
    block: &ast::Block,
    noncopy: &std::collections::HashSet<String>,
    fname: &str,
    out: &mut Vec<(String, MoveSite)>,
) {
    for stmt in &block.stmts {
        collect_stmt_captures(stmt, noncopy, fname, out);
    }
    if let Some(t) = &block.tail {
        collect_expr_captures(t, noncopy, fname, out);
    }
}

/// Find closure literals in one statement.
fn collect_stmt_captures(
    stmt: &Stmt,
    noncopy: &std::collections::HashSet<String>,
    fname: &str,
    out: &mut Vec<(String, MoveSite)>,
) {
    match stmt {
        Stmt::Let { value, .. } | Stmt::Var { value, .. } => {
            collect_expr_captures(value, noncopy, fname, out)
        }
        Stmt::Assign { target, value, .. } => {
            collect_expr_captures(target, noncopy, fname, out);
            collect_expr_captures(value, noncopy, fname, out);
        }
        Stmt::Expr(e, _) => collect_expr_captures(e, noncopy, fname, out),
        Stmt::If { cond, then_body, elseifs, else_body, .. } => {
            collect_expr_captures(cond, noncopy, fname, out);
            collect_block_captures(then_body, noncopy, fname, out);
            for (c, b) in elseifs {
                collect_expr_captures(c, noncopy, fname, out);
                collect_block_captures(b, noncopy, fname, out);
            }
            if let Some(b) = else_body {
                collect_block_captures(b, noncopy, fname, out);
            }
        }
        Stmt::While { cond, body, .. } => {
            collect_expr_captures(cond, noncopy, fname, out);
            collect_block_captures(body, noncopy, fname, out);
        }
        Stmt::Repeat { body, cond, .. } => {
            collect_block_captures(body, noncopy, fname, out);
            collect_expr_captures(cond, noncopy, fname, out);
        }
        Stmt::For { iter, body, .. } => {
            collect_expr_captures(iter, noncopy, fname, out);
            collect_block_captures(body, noncopy, fname, out);
        }
        Stmt::Loop { body, .. } => collect_block_captures(body, noncopy, fname, out),
        Stmt::Return(e, _) => {
            if let Some(e) = e {
                collect_expr_captures(e, noncopy, fname, out);
            }
        }
        Stmt::Match { scrutinee, arms, .. } => {
            collect_expr_captures(scrutinee, noncopy, fname, out);
            for arm in arms {
                collect_block_captures(&arm.body, noncopy, fname, out);
            }
        }
        Stmt::Block(b, _) | Stmt::Unsafe(b, _) => collect_block_captures(b, noncopy, fname, out),
        Stmt::Spawn { call, .. } => collect_expr_captures(call, noncopy, fname, out),
        _ => {}
    }
}

/// Find closure literals in one expression; a closure that references a
/// non-Copy outer local records a move site at the closure's span.
fn collect_expr_captures(
    e: &Expr,
    noncopy: &std::collections::HashSet<String>,
    fname: &str,
    out: &mut Vec<(String, MoveSite)>,
) {
    match e {
        Expr::Closure(params, _, body, cspan) => {
            let mut bound: Vec<String> = params.iter().map(|p| p.name.name.clone()).collect();
            collect_binding_names(body, &mut bound);
            let mut refs: Vec<(String, Span)> = Vec::new();
            collect_ref_sites(body, &mut refs);
            for (n, span) in refs {
                if noncopy.contains(&n) && !bound.contains(&n) {
                    out.push((
                        fname.to_string(),
                        MoveSite {
                            name: n.clone(),
                            callee: "closure".to_string(),
                            ident: span,
                            call: *cspan,
                        },
                    ));
                }
            }
            collect_block_captures(body, noncopy, fname, out);
        }
        Expr::BinOp(l, _, r, _) => {
            collect_expr_captures(l, noncopy, fname, out);
            collect_expr_captures(r, noncopy, fname, out);
        }
        Expr::UnaryOp(_, inner, _) | Expr::Spread(inner, _) => {
            collect_expr_captures(inner, noncopy, fname, out)
        }
        Expr::Call(c, args, _) => {
            collect_expr_captures(c, noncopy, fname, out);
            for a in args {
                if let Some(x) = arg_expr(a) {
                    collect_expr_captures(x, noncopy, fname, out);
                }
            }
        }
        Expr::MethodCall(recv, _, args, _, _) => {
            collect_expr_captures(recv, noncopy, fname, out);
            for a in args {
                if let Some(x) = arg_expr(a) {
                    collect_expr_captures(x, noncopy, fname, out);
                }
            }
        }
        Expr::Field(recv, _, _)
        | Expr::Index(recv, _, _)
        | Expr::Await(recv, _)
        | Expr::Try(recv, _)
        | Expr::Bang(recv, _)
        | Expr::QuestionDot(recv, _, _)
        | Expr::Cast(recv, _, _)
        | Expr::As(recv, _, _) => collect_expr_captures(recv, noncopy, fname, out),
        Expr::Array(elems, _) | Expr::Tuple(elems, _) => {
            for el in elems {
                collect_expr_captures(el, noncopy, fname, out);
            }
        }
        Expr::Map(fields, _) => {
            for f in fields {
                collect_expr_captures(&f.key, noncopy, fname, out);
                collect_expr_captures(&f.value, noncopy, fname, out);
            }
        }
        Expr::StructLit(_, fields, _) => {
            for f in fields {
                collect_expr_captures(&f.value, noncopy, fname, out);
            }
        }
        Expr::InterpStr(parts, _) => {
            for p in parts {
                if let ast::InterpStrPart::Expr(inner) = p {
                    collect_expr_captures(inner, noncopy, fname, out);
                }
            }
        }
        Expr::Range(lo, hi, _, _) => {
            if let Some(l) = lo {
                collect_expr_captures(l, noncopy, fname, out);
            }
            if let Some(h) = hi {
                collect_expr_captures(h, noncopy, fname, out);
            }
        }
        Expr::IfExpr(c, t, ei, el, _) => {
            collect_expr_captures(c, noncopy, fname, out);
            collect_block_captures(t, noncopy, fname, out);
            for (c2, b) in ei {
                collect_expr_captures(c2, noncopy, fname, out);
                collect_block_captures(b, noncopy, fname, out);
            }
            if let Some(b) = el {
                collect_block_captures(b, noncopy, fname, out);
            }
        }
        Expr::MatchExpr(scrut, arms, _) => {
            collect_expr_captures(scrut, noncopy, fname, out);
            for arm in arms {
                collect_block_captures(&arm.body, noncopy, fname, out);
            }
        }
        Expr::BlockExpr(b, _) => collect_block_captures(b, noncopy, fname, out),
        Expr::PipeForward(l, r, _) | Expr::BackPipe(l, r, _) => {
            collect_expr_captures(l, noncopy, fname, out);
            collect_expr_captures(r, noncopy, fname, out);
        }
        _ => {}
    }
}

/// Collect names bound by let/var/for and patterns inside a block.
fn collect_binding_names(block: &ast::Block, out: &mut Vec<String>) {
    for stmt in &block.stmts {
        match stmt {
            Stmt::Let { name, .. } | Stmt::Var { name, .. } => out.push(name.name.clone()),
            Stmt::For { var, .. } => out.push(var.name.clone()),
            Stmt::If { then_body, elseifs, else_body, .. } => {
                collect_binding_names(then_body, out);
                for (_, b) in elseifs {
                    collect_binding_names(b, out);
                }
                if let Some(b) = else_body {
                    collect_binding_names(b, out);
                }
            }
            Stmt::While { body, .. } | Stmt::Repeat { body, .. } | Stmt::Loop { body, .. } => {
                collect_binding_names(body, out)
            }
            Stmt::Match { arms, .. } => {
                for arm in arms {
                    collect_pattern_names(&arm.pattern, out);
                    collect_binding_names(&arm.body, out);
                }
            }
            Stmt::Block(b, _) | Stmt::Unsafe(b, _) => collect_binding_names(b, out),
            _ => {}
        }
    }
    let _ = block.tail.as_deref().map(|t| collect_expr_binding_names(t, out));
}

/// Collect names bound inside expression-level blocks.
fn collect_expr_binding_names(e: &Expr, out: &mut Vec<String>) {
    match e {
        Expr::Closure(params, _, body, _) => {
            out.extend(params.iter().map(|p| p.name.name.clone()));
            collect_binding_names(body, out);
        }
        Expr::IfExpr(_, t, ei, el, _) => {
            collect_binding_names(t, out);
            for (_, b) in ei {
                collect_binding_names(b, out);
            }
            if let Some(b) = el {
                collect_binding_names(b, out);
            }
        }
        Expr::MatchExpr(_, arms, _) => {
            for arm in arms {
                collect_pattern_names(&arm.pattern, out);
                collect_binding_names(&arm.body, out);
            }
        }
        Expr::BlockExpr(b, _) => collect_binding_names(b, out),
        _ => {}
    }
}

/// Collect names bound by a match pattern.
fn collect_pattern_names(p: &ast::Pattern, out: &mut Vec<String>) {
    match p {
        ast::Pattern::Binding(id) => out.push(id.name.clone()),
        ast::Pattern::EnumVariant { sub, .. } => {
            for s in sub {
                collect_pattern_names(s, out);
            }
        }
        ast::Pattern::Struct { fields, .. } => {
            for (_, sub) in fields {
                collect_pattern_names(sub, out);
            }
        }
        ast::Pattern::Tuple(elems, _) | ast::Pattern::Array(elems, _) | ast::Pattern::Or(elems, _) => {
            for el in elems {
                collect_pattern_names(el, out);
            }
        }
        ast::Pattern::As(inner, id, _) => {
            out.push(id.name.clone());
            collect_pattern_names(inner, out);
        }
        _ => {}
    }
}

/// Collect every identifier reference (name, span) in a block, skipping
/// method names, field names, and struct-literal keys.
fn collect_ref_sites(block: &ast::Block, out: &mut Vec<(String, Span)>) {
    for stmt in &block.stmts {
        match stmt {
            Stmt::Let { value, .. } | Stmt::Var { value, .. } => {
                collect_expr_refs(value, out)
            }
            Stmt::Assign { target, value, .. } => {
                collect_expr_refs(target, out);
                collect_expr_refs(value, out);
            }
            Stmt::Expr(e, _) => collect_expr_refs(e, out),
            Stmt::If { cond, then_body, elseifs, else_body, .. } => {
                collect_expr_refs(cond, out);
                collect_ref_sites(then_body, out);
                for (c, b) in elseifs {
                    collect_expr_refs(c, out);
                    collect_ref_sites(b, out);
                }
                if let Some(b) = else_body {
                    collect_ref_sites(b, out);
                }
            }
            Stmt::While { cond, body, .. } => {
                collect_expr_refs(cond, out);
                collect_ref_sites(body, out);
            }
            Stmt::Repeat { body, cond, .. } => {
                collect_ref_sites(body, out);
                collect_expr_refs(cond, out);
            }
            Stmt::For { iter, body, .. } => {
                collect_expr_refs(iter, out);
                collect_ref_sites(body, out);
            }
            Stmt::Loop { body, .. } => collect_ref_sites(body, out),
            Stmt::Return(e, _) => {
                if let Some(e) = e {
                    collect_expr_refs(e, out);
                }
            }
            Stmt::Match { scrutinee, arms, .. } => {
                collect_expr_refs(scrutinee, out);
                for arm in arms {
                    if let Some(g) = &arm.guard {
                        collect_expr_refs(g, out);
                    }
                    collect_ref_sites(&arm.body, out);
                }
            }
            Stmt::Block(b, _) | Stmt::Unsafe(b, _) => collect_ref_sites(b, out),
            Stmt::Spawn { call, .. } => collect_expr_refs(call, out),
            _ => {}
        }
    }
    if let Some(t) = &block.tail {
        collect_expr_refs(t, out);
    }
}

/// Collect identifier references in one expression.
fn collect_expr_refs(e: &Expr, out: &mut Vec<(String, Span)>) {
    match e {
        Expr::Ident(id) => out.push((id.name.clone(), id.span)),
        Expr::BinOp(l, _, r, _) => {
            collect_expr_refs(l, out);
            collect_expr_refs(r, out);
        }
        Expr::UnaryOp(_, inner, _) | Expr::Spread(inner, _) => collect_expr_refs(inner, out),
        Expr::Call(c, args, _) => {
            collect_expr_refs(c, out);
            for a in args {
                if let Some(x) = arg_expr(a) {
                    collect_expr_refs(x, out);
                }
            }
        }
        Expr::MethodCall(recv, _, args, _, _) => {
            collect_expr_refs(recv, out);
            for a in args {
                if let Some(x) = arg_expr(a) {
                    collect_expr_refs(x, out);
                }
            }
        }
        Expr::Field(recv, _, _)
        | Expr::Index(recv, _, _)
        | Expr::Await(recv, _)
        | Expr::Try(recv, _)
        | Expr::Bang(recv, _)
        | Expr::QuestionDot(recv, _, _)
        | Expr::Cast(recv, _, _)
        | Expr::As(recv, _, _) => collect_expr_refs(recv, out),
        Expr::Array(elems, _) | Expr::Tuple(elems, _) => {
            for el in elems {
                collect_expr_refs(el, out);
            }
        }
        Expr::Map(fields, _) => {
            for f in fields {
                collect_expr_refs(&f.key, out);
                collect_expr_refs(&f.value, out);
            }
        }
        Expr::StructLit(_, fields, _) => {
            for f in fields {
                collect_expr_refs(&f.value, out);
            }
        }
        Expr::InterpStr(parts, _) => {
            for p in parts {
                if let ast::InterpStrPart::Expr(inner) = p {
                    collect_expr_refs(inner, out);
                }
            }
        }
        Expr::Range(lo, hi, _, _) => {
            if let Some(l) = lo {
                collect_expr_refs(l, out);
            }
            if let Some(h) = hi {
                collect_expr_refs(h, out);
            }
        }
        Expr::Closure(params, _, body, _) => {
            let _ = params;
            collect_ref_sites(body, out);
        }
        Expr::IfExpr(c, t, ei, el, _) => {
            collect_expr_refs(c, out);
            collect_ref_sites(t, out);
            for (c2, b) in ei {
                collect_expr_refs(c2, out);
                collect_ref_sites(b, out);
            }
            if let Some(b) = el {
                collect_ref_sites(b, out);
            }
        }
        Expr::MatchExpr(scrut, arms, _) => {
            collect_expr_refs(scrut, out);
            for arm in arms {
                if let Some(g) = &arm.guard {
                    collect_expr_refs(g, out);
                }
                collect_ref_sites(&arm.body, out);
            }
        }
        Expr::BlockExpr(b, _) => collect_ref_sites(b, out),
        Expr::PipeForward(l, r, _) | Expr::BackPipe(l, r, _) => {
            collect_expr_refs(l, out);
            collect_expr_refs(r, out);
        }
        _ => {}
    }
}
