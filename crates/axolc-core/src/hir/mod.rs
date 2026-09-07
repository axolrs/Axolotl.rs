// Owner: PascalElixir / axolrs (GitHub org)
// File: HIR - the high-level intermediate representation: lowered, desugared, impl-grouped form consumed by both codegen and the interpreter.

use crate::ast;
use crate::diag::Diagnostics;
use crate::ownership::OwnershipTable;
use crate::span::Span;

/// A HIR module: the desugared item list plus the ownership analysis result.
#[derive(Debug, Clone, Default)]
pub struct Module {
    pub items: Vec<Item>,
    pub ownership: OwnershipTable,
    /// Diagnostics produced during lowering (ownership inference and checks).
    pub diags: Diagnostics,
}

/// A HIR item - ast items with impl blocks grouping their methods.
#[derive(Debug, Clone, PartialEq)]
pub enum Item {
    Use(ast::UseItem),
    Fn(ast::FnItem),
    Struct(ast::StructItem),
    Enum(ast::EnumItem),
    Interface(ast::InterfaceItem),
    Const(ast::ConstItem),
    TypeAlias(ast::TypeAliasItem),
    Impl(ImplBlock),
    ExternBlock(ast::ExternBlock),
    Foreign(ast::ForeignBlock),
    ModuleDoc(String, Span),
}

/// A grouped impl block: one type (optionally implementing one interface) with its methods.
#[derive(Debug, Clone, PartialEq)]
pub struct ImplBlock {
    pub ty_name: ast::Ident,
    pub interface_name: Option<ast::Ident>,
    pub methods: Vec<ast::MethodItem>,
    pub static_methods: Vec<ast::MethodItem>,
    pub is_trait_impl: bool,
    pub span: Span,
}

/// Lower an AST module into HIR: desugars numeric `for`, `repeat/until`, simple pipes,
/// groups methods into impl blocks, and attaches the ownership analysis.
pub fn lower(module: &ast::Module) -> Module {
    let (ownership, diags) = crate::ownership::infer(module);
    let lowered = lower_with(module, ownership);
    Module { items: lowered.items, ownership: lowered.ownership, diags }
}

/// Lower an AST module into HIR with a pre-computed ownership table.
pub fn lower_with(module: &ast::Module, ownership: OwnershipTable) -> Module {
    let mut items: Vec<Item> = Vec::new();
    let mut pending_impls: Vec<ImplBlock> = Vec::new();

    for item in &module.items {
        match item {
            ast::Item::Method(m) => {
                let block = ensure_impl(&mut pending_impls, &m.receiver, None, m.span);
                let mut lowered = m.clone();
                lower_block_in_place(&mut lowered.body);
                block.methods.push(lowered);
            }
            ast::Item::StaticMethod(m) => {
                let block = ensure_impl(&mut pending_impls, &m.receiver, None, m.span);
                let mut lowered = m.clone();
                lower_block_in_place(&mut lowered.body);
                block.static_methods.push(lowered);
            }
            ast::Item::Impl(i) => {
                let block = ensure_impl(&mut pending_impls, &i.ty_name, i.interface_name.clone(), i.span);
                if i.interface_name.is_some() {
                    block.is_trait_impl = true;
                }
            }
            ast::Item::Fn(f) => {
                let mut f = f.clone();
                lower_block_in_place(&mut f.body);
                items.push(Item::Fn(f));
            }
            ast::Item::Use(u) => items.push(Item::Use(u.clone())),
            ast::Item::Struct(s) => items.push(Item::Struct(s.clone())),
            ast::Item::Enum(e) => items.push(Item::Enum(e.clone())),
            ast::Item::Interface(i) => items.push(Item::Interface(i.clone())),
            ast::Item::Const(c) => {
                let mut c = c.clone();
                c.value = lower_expr_clone(&c.value);
                items.push(Item::Const(c));
            }
            ast::Item::TypeAlias(t) => items.push(Item::TypeAlias(t.clone())),
            ast::Item::ExternBlock(e) => items.push(Item::ExternBlock(e.clone())),
            ast::Item::CBlock(b) | ast::Item::CppBlock(b) | ast::Item::RBlock(b)
            | ast::Item::PyBlock(b) => items.push(Item::Foreign(b.clone())),
            ast::Item::ModuleDoc(text, span) => {
                items.push(Item::ModuleDoc(text.clone(), *span))
            }
        }
    }

    for block in pending_impls {
        let mut lowered_block = block;
        for m in lowered_block.methods.iter_mut() {
            lower_block_in_place(&mut m.body);
        }
        for m in lowered_block.static_methods.iter_mut() {
            lower_block_in_place(&mut m.body);
        }
        items.push(Item::Impl(lowered_block));
    }

    Module { items, ownership, diags: Diagnostics::new() }
}

/// Find or create the pending impl block for a receiver type.
fn ensure_impl<'a>(
    pending: &'a mut Vec<ImplBlock>,
    ty_name: &ast::Ident,
    interface: Option<ast::Ident>,
    span: Span,
) -> &'a mut ImplBlock {
    let existing = pending.iter().position(|b| {
        b.ty_name.name == ty_name.name
            && b.interface_name.is_none()
            && interface.is_none()
    });
    let idx = existing.unwrap_or_else(|| {
        pending.push(ImplBlock {
            ty_name: ty_name.clone(),
            interface_name: interface.clone(),
            methods: Vec::new(),
            static_methods: Vec::new(),
            is_trait_impl: interface.is_some(),
            span,
        });
        pending.len() - 1
    });
    &mut pending[idx]
}

/// Desugar a block's statements and tail in place.
fn lower_block_in_place(block: &mut ast::Block) {
    let mut out: Vec<ast::Stmt> = Vec::new();
    for stmt in block.stmts.drain(..) {
        let lowered = lower_stmt(stmt);
        out.extend(lowered);
    }
    block.stmts = out;
    if let Some(tail) = &mut block.tail {
        *tail = Box::new(lower_expr_clone(tail));
    }
}

/// Lower one statement, possibly expanding it into several.
fn lower_stmt(stmt: ast::Stmt) -> Vec<ast::Stmt> {
    match stmt {
        ast::Stmt::For { var, iter, body, span } => {
            let lowered_iter = lower_iter_expr(iter);
            let mut body = body;
            lower_block_in_place(&mut body);
            vec![ast::Stmt::For { var, iter: lowered_iter, body, span }]
        }
        ast::Stmt::Repeat { body, cond, span } => {
            let mut expanded: Vec<ast::Stmt> = Vec::new();
            let mut body = body;
            lower_block_in_place(&mut body);
            let break_stmt = ast::Stmt::If {
                cond: lower_expr_clone(&cond),
                then_body: ast::Block {
                    stmts: vec![ast::Stmt::Break(span)],
                    tail: None,
                    span,
                },
                elseifs: Vec::new(),
                else_body: None,
                span,
            };
            expanded.extend(body.stmts);
            if let Some(tail) = body.tail {
                expanded.push(ast::Stmt::Expr((*tail).clone(), tail.span()));
            }
            expanded.push(break_stmt);
            vec![ast::Stmt::Loop {
                body: ast::Block { stmts: expanded, tail: None, span },
                span,
            }]
        }
        ast::Stmt::Let { name, ty, value, span } => {
            vec![ast::Stmt::Let { name, ty, value: lower_expr_clone(&value), span }]
        }
        ast::Stmt::Var { name, ty, value, span } => {
            vec![ast::Stmt::Var { name, ty, value: lower_expr_clone(&value), span }]
        }
        ast::Stmt::Assign { target, value, op, span } => {
            vec![ast::Stmt::Assign {
                target: lower_expr_clone(&target),
                value: lower_expr_clone(&value),
                op,
                span,
            }]
        }
        ast::Stmt::Expr(e, span) => vec![ast::Stmt::Expr(lower_expr_clone(&e), span)],
        ast::Stmt::If { cond, then_body, elseifs, else_body, span } => {
            let mut then_body = then_body;
            lower_block_in_place(&mut then_body);
            let mut elseifs_out = Vec::new();
            for (c, mut b) in elseifs {
                lower_block_in_place(&mut b);
                elseifs_out.push((lower_expr_clone(&c), b));
            }
            let mut else_body = else_body;
            if let Some(b) = &mut else_body {
                lower_block_in_place(b);
            }
            vec![ast::Stmt::If {
                cond: lower_expr_clone(&cond),
                then_body,
                elseifs: elseifs_out,
                else_body,
                span,
            }]
        }
        ast::Stmt::While { cond, body, span } => {
            let mut body = body;
            lower_block_in_place(&mut body);
            vec![ast::Stmt::While { cond: lower_expr_clone(&cond), body, span }]
        }
        ast::Stmt::Loop { body, span } => {
            let mut body = body;
            lower_block_in_place(&mut body);
            vec![ast::Stmt::Loop { body, span }]
        }
        ast::Stmt::Match { scrutinee, arms, span } => {
            let mut arms_out = Vec::new();
            for arm in arms {
                let mut body = arm.body;
                lower_block_in_place(&mut body);
                arms_out.push(ast::MatchArm {
                    pattern: arm.pattern,
                    guard: arm.guard.as_ref().map(lower_expr_clone),
                    body,
                    span: arm.span,
                });
            }
            vec![ast::Stmt::Match {
                scrutinee: lower_expr_clone(&scrutinee),
                arms: arms_out,
                span,
            }]
        }
        ast::Stmt::Block(mut b, span) => {
            lower_block_in_place(&mut b);
            vec![ast::Stmt::Block(b, span)]
        }
        ast::Stmt::Unsafe(mut b, span) => {
            lower_block_in_place(&mut b);
            vec![ast::Stmt::Unsafe(b, span)]
        }
        other => vec![other],
    }
}

/// Normalize the iterable of a `for` loop: a bare integer literal becomes an explicit range.
fn lower_iter_expr(iter: ast::Expr) -> ast::Expr {
    match iter {
        ast::Expr::IntLit(n, span) => ast::Expr::Range(
            Some(Box::new(ast::Expr::IntLit("0".to_string(), span))),
            Some(Box::new(ast::Expr::IntLit(n, span))),
            false,
            span,
        ),
        other => lower_expr_clone(&other),
    }
}

/// Lower a copy of one expression: simple pipes become direct calls.
fn lower_expr_clone(e: &ast::Expr) -> ast::Expr {
    match e {
        ast::Expr::PipeForward(left, right, span) => {
            let lowered_left = lower_expr_clone(left);
            match &**right {
                ast::Expr::Ident(f) => ast::Expr::Call(
                    Box::new(ast::Expr::Ident(f.clone())),
                    vec![ast::CallArg::Positional(lowered_left)],
                    *span,
                ),
                ast::Expr::Closure(params, ret, body, cspan) => {
                    let mut body = body.clone();
                    lower_block_in_place(&mut body);
                    ast::Expr::Call(
                        Box::new(ast::Expr::Closure(params.clone(), ret.clone(), body, *cspan)),
                        vec![ast::CallArg::Positional(lowered_left)],
                        *span,
                    )
                }
                _ => ast::Expr::PipeForward(
                    Box::new(lowered_left),
                    Box::new(lower_expr_clone(right)),
                    *span,
                ),
            }
        }
        ast::Expr::BinOp(l, op, r, span) => ast::Expr::BinOp(
            Box::new(lower_expr_clone(l)),
            *op,
            Box::new(lower_expr_clone(r)),
            *span,
        ),
        ast::Expr::UnaryOp(op, inner, span) => {
            ast::Expr::UnaryOp(*op, Box::new(lower_expr_clone(inner)), *span)
        }
        ast::Expr::Call(callee, args, span) => {
            let lowered_args = args
                .iter()
                .map(|a| match a {
                    ast::CallArg::Positional(x) => {
                        ast::CallArg::Positional(lower_expr_clone(x))
                    }
                    ast::CallArg::Named(n, x) => {
                        ast::CallArg::Named(n.clone(), lower_expr_clone(x))
                    }
                    ast::CallArg::Spread(x, sspan) => {
                        ast::CallArg::Spread(lower_expr_clone(x), *sspan)
                    }
                })
                .collect();
            ast::Expr::Call(Box::new(lower_expr_clone(callee)), lowered_args, *span)
        }
        ast::Expr::MethodCall(recv, name, args, colon, span) => {
            let lowered_args = args
                .iter()
                .map(|a| match a {
                    ast::CallArg::Positional(x) => {
                        ast::CallArg::Positional(lower_expr_clone(x))
                    }
                    ast::CallArg::Named(n, x) => {
                        ast::CallArg::Named(n.clone(), lower_expr_clone(x))
                    }
                    ast::CallArg::Spread(x, sspan) => {
                        ast::CallArg::Spread(lower_expr_clone(x), *sspan)
                    }
                })
                .collect();
            ast::Expr::MethodCall(
                Box::new(lower_expr_clone(recv)),
                name.clone(),
                lowered_args,
                *colon,
                *span,
            )
        }
        ast::Expr::Field(recv, name, span) => {
            ast::Expr::Field(Box::new(lower_expr_clone(recv)), name.clone(), *span)
        }
        ast::Expr::Index(recv, idx, span) => ast::Expr::Index(
            Box::new(lower_expr_clone(recv)),
            Box::new(lower_expr_clone(idx)),
            *span,
        ),
        ast::Expr::Array(elems, span) => ast::Expr::Array(
            elems.iter().map(lower_expr_clone).collect(),
            *span,
        ),
        ast::Expr::Tuple(elems, span) => ast::Expr::Tuple(
            elems.iter().map(lower_expr_clone).collect(),
            *span,
        ),
        ast::Expr::Map(fields, span) => ast::Expr::Map(
            fields
                .iter()
                .map(|f| ast::MapField {
                    key: lower_expr_clone(&f.key),
                    value: lower_expr_clone(&f.value),
                    span: f.span,
                })
                .collect(),
            *span,
        ),
        ast::Expr::StructLit(ty, fields, span) => ast::Expr::StructLit(
            ty.clone(),
            fields
                .iter()
                .map(|f| ast::MapField {
                    key: lower_expr_clone(&f.key),
                    value: lower_expr_clone(&f.value),
                    span: f.span,
                })
                .collect(),
            *span,
        ),
        ast::Expr::InterpStr(parts, span) => ast::Expr::InterpStr(
            parts
                .iter()
                .map(|p| match p {
                    ast::InterpStrPart::Text(t) => ast::InterpStrPart::Text(t.clone()),
                    ast::InterpStrPart::Expr(x) => {
                        ast::InterpStrPart::Expr(lower_expr_clone(x))
                    }
                })
                .collect(),
            *span,
        ),
        ast::Expr::Closure(params, ret, body, span) => {
            let mut body = body.clone();
            lower_block_in_place(&mut body);
            ast::Expr::Closure(params.clone(), ret.clone(), body, *span)
        }
        ast::Expr::IfExpr(cond, then_body, elseifs, else_body, span) => {
            let mut then_body = then_body.clone();
            lower_block_in_place(&mut then_body);
            let mut elseifs_out = Vec::new();
            for (c, b) in elseifs {
                let mut b = b.clone();
                lower_block_in_place(&mut b);
                elseifs_out.push((lower_expr_clone(c), b));
            }
            let mut else_body = else_body.clone();
            if let Some(b) = &mut else_body {
                lower_block_in_place(b);
            }
            ast::Expr::IfExpr(
                Box::new(lower_expr_clone(cond)),
                then_body,
                elseifs_out,
                else_body,
                *span,
            )
        }
        ast::Expr::MatchExpr(scrut, arms, span) => {
            let mut arms_out = Vec::new();
            for arm in arms {
                let mut body = arm.body.clone();
                lower_block_in_place(&mut body);
                arms_out.push(ast::MatchArm {
                    pattern: arm.pattern.clone(),
                    guard: arm.guard.as_ref().map(lower_expr_clone),
                    body,
                    span: arm.span,
                });
            }
            ast::Expr::MatchExpr(Box::new(lower_expr_clone(scrut)), arms_out, *span)
        }
        ast::Expr::BlockExpr(b, span) => {
            let mut b = b.clone();
            lower_block_in_place(&mut b);
            ast::Expr::BlockExpr(b, *span)
        }
        other => other.clone(),
    }
}
