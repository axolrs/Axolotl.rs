// Owner: PascalElixir / axolrs (GitHub org)
// File: Cranelift-based JIT for a statically verifiable Axolotl subset with per-function interpreter fallback.

use axolc_core::ast::{
    AssignOp, BinOp, Block, CallArg, Expr, FnItem, Ident, Item, Module, Param, Stmt, TypeAnnotKind,
    UnaryOp,
};
use axolc_core::diag::{Diagnostics, Severity};
use axolc_core::interp::{Interpreter, Value};
use axolc_core::lexer::TokenKind;
use axolc_core::span::Span;
use cranelift_codegen::ir::condcodes::IntCC;
use cranelift_codegen::ir::{
    types, AbiParam, Block as IrBlock, InstBuilder, Signature, UserFuncName, Value as IrValue,
};
use cranelift_frontend::{FunctionBuilder, FunctionBuilderContext, Variable};
use cranelift_jit::{JITBuilder, JITModule};
use cranelift_module::{default_libcall_names, FuncId, Linkage, Module as _};
use std::cell::RefCell;
use std::collections::HashMap;

/// The inferred static type of a JIT-compatible expression.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JType {
    Int,
    Bool,
    Unit,
}

/// The ABI return shape of a JIT-compiled function.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RetSig {
    None,
    Int,
    Bool,
}

/// How a module-level function executes under the JIT plan.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FnMode {
    Jit,
    Trampoline,
    Interpreted,
}

/// One function's execution decision from the JIT planner.
#[derive(Debug, Clone)]
pub struct FnDecision {
    pub name: String,
    pub mode: FnMode,
    pub reason: Option<String>,
}

/// The per-function execution plan for a module, plus whether main runs natively.
#[derive(Debug, Clone)]
pub struct JitPlan {
    pub decisions: Vec<FnDecision>,
    pub main_jit: bool,
}

impl JitPlan {
    /// Return the decision recorded for a named function, if present.
    pub fn decision(&self, name: &str) -> Option<&FnDecision> {
        self.decisions.iter().find(|d| d.name == name)
    }
}

/// A resolvable top-level constant: a literal Int or Bool.
#[derive(Debug, Clone, Copy)]
enum ConstVal {
    Int(i64),
    Bool(bool),
    Unsupported,
}

/// Per-function analysis: execution mode, fallback reason, and ABI return shape.
struct FnAnalysis {
    item: FnItem,
    mode: FnMode,
    reason: Option<String>,
    ret: RetSig,
}

/// The module-wide analysis: the function table plus main's executability.
struct Analysis {
    fns: HashMap<String, FnAnalysis>,
    consts: HashMap<String, ConstVal>,
    main_jit: bool,
}

/// Run a program through the JIT with per-function interpreter fallback and return captured output.
pub fn run(src: &str) -> Result<String, Vec<String>> {
    run_traced(src, false)
}

/// Run a program through the JIT, optionally printing per-function decisions to stderr.
pub fn run_traced(src: &str, trace: bool) -> Result<String, Vec<String>> {
    let (module, diags) = axolc_core::parse(src, 0);
    if diags.has_errors() {
        return Err(render_errors(&diags));
    }
    let analysis = analyze(&module);
    if trace {
        trace_plan(&analysis);
    }
    if !analysis.main_jit {
        if trace {
            eprintln!("jit: main is not JIT-compatible - running the whole program through the interpreter");
        }
        return interp_run(src);
    }
    if trace {
        eprintln!("jit: executing `main` as native code");
    }
    jit_execute(&module, &analysis)
}

/// Compute the per-function JIT plan for a source string without executing anything.
pub fn plan(src: &str) -> Result<JitPlan, Vec<String>> {
    let (module, diags) = axolc_core::parse(src, 0);
    if diags.has_errors() {
        return Err(render_errors(&diags));
    }
    let analysis = analyze(&module);
    let mut decisions: Vec<FnDecision> = analysis
        .fns
        .iter()
        .map(|(name, fa)| FnDecision {
            name: name.clone(),
            mode: fa.mode,
            reason: fa.reason.clone(),
        })
        .collect();
    decisions.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(JitPlan {
        decisions,
        main_jit: analysis.main_jit,
    })
}

/// Run a program through the tree-walking interpreter and shape the result.
fn interp_run(src: &str) -> Result<String, Vec<String>> {
    let (out, diags) = axolc_core::interpret(src, 0);
    if diags.has_errors() {
        return Err(render_errors(&diags));
    }
    Ok(out)
}

/// Render error-severity diagnostics as printable strings.
fn render_errors(diags: &Diagnostics) -> Vec<String> {
    diags
        .items
        .iter()
        .filter(|d| d.severity == Severity::Error)
        .map(|d| format!("{}", d))
        .collect()
}

/// Print the per-function execution plan to stderr.
fn trace_plan(analysis: &Analysis) {
    let mut names: Vec<&String> = analysis.fns.keys().collect();
    names.sort();
    for name in names {
        let fa = &analysis.fns[name];
        let reason = fa.reason.as_deref().unwrap_or("unsupported construct");
        match fa.mode {
            FnMode::Jit => eprintln!("jit: `{}` - compiled to native code", name),
            FnMode::Trampoline => {
                eprintln!("jit: `{}` - interpreter fallback via trampoline ({})", name, reason)
            }
            FnMode::Interpreted => {
                eprintln!("jit: `{}` - interpreter fallback: {}", name, reason)
            }
        }
    }
}

/// Collect the module's function and constant tables.
fn collect_symbols(module: &Module) -> (HashMap<String, FnItem>, HashMap<String, ConstVal>) {
    let mut fns = HashMap::new();
    let mut consts = HashMap::new();
    for item in &module.items {
        match item {
            Item::Fn(f) => {
                fns.insert(f.name.name.clone(), f.clone());
            }
            Item::Const(c) => {
                let v = match &c.value {
                    Expr::IntLit(s, _) => s
                        .replace('_', "")
                        .parse::<i64>()
                        .map(ConstVal::Int)
                        .unwrap_or(ConstVal::Unsupported),
                    Expr::BoolLit(b, _) => ConstVal::Bool(*b),
                    _ => ConstVal::Unsupported,
                };
                consts.insert(c.name.name.clone(), v);
            }
            _ => {}
        }
    }
    (fns, consts)
}

/// Run the fixpoint compatibility analysis over the module's functions.
fn analyze(module: &Module) -> Analysis {
    let (fns, consts) = collect_symbols(module);
    let mut modes: HashMap<String, FnMode> = fns.keys().map(|k| (k.clone(), FnMode::Jit)).collect();
    let mut reasons: HashMap<String, String> = HashMap::new();
    let mut rets: HashMap<String, RetSig> = HashMap::new();
    loop {
        let mut changed = false;
        let mut names: Vec<String> = fns.keys().cloned().collect();
        names.sort();
        for name in names {
            if modes[&name] != FnMode::Jit {
                continue;
            }
            let item = fns[&name].clone();
            match check_function(&item, &fns, &consts, &modes) {
                Ok(ret) => {
                    rets.insert(name, ret);
                }
                Err(reason) => {
                    let mode = if trampolinable(&item) {
                        FnMode::Trampoline
                    } else {
                        FnMode::Interpreted
                    };
                    modes.insert(name.clone(), mode);
                    reasons.insert(name, reason);
                    changed = true;
                }
            }
        }
        if !changed {
            break;
        }
    }
    let main_jit = modes.get("main") == Some(&FnMode::Jit);
    let mut analyzed = HashMap::new();
    for (name, item) in fns {
        let mode = modes.remove(&name).unwrap_or(FnMode::Interpreted);
        let reason = reasons.remove(&name);
        let ret = match mode {
            FnMode::Jit => rets.get(&name).copied().unwrap_or(RetSig::None),
            _ => match declared_ret(&item) {
                JType::Int => RetSig::Int,
                JType::Bool => RetSig::Bool,
                JType::Unit => RetSig::None,
            },
        };
        analyzed.insert(name, FnAnalysis { item, mode, reason, ret });
    }
    Analysis { fns: analyzed, consts, main_jit }
}

/// Return true when an incompatible function can still be called from JIT code via a host trampoline.
fn trampolinable(f: &FnItem) -> bool {
    if f.name.name == "main" {
        return false;
    }
    if f.is_extern || f.is_async || !f.generics.is_empty() {
        return false;
    }
    if f.params.len() > 4 {
        return false;
    }
    if !matches!(declared_ret(f), JType::Int | JType::Bool) {
        return false;
    }
    f.params.iter().all(|p| p.default.is_none() && param_jtype(p).is_ok())
}

/// Type-check one function for JIT compatibility, returning its ABI return shape or a reason.
fn check_function(
    f: &FnItem,
    fns: &HashMap<String, FnItem>,
    consts: &HashMap<String, ConstVal>,
    modes: &HashMap<String, FnMode>,
) -> Result<RetSig, String> {
    if f.is_extern {
        return Err("extern function".into());
    }
    if f.is_async {
        return Err("async function".into());
    }
    if !f.generics.is_empty() {
        return Err("generic function".into());
    }
    for p in &f.params {
        param_jtype(p).map_err(|_| format!("parameter `{}` has an unsupported type", p.name.name))?;
        if p.default.is_some() {
            return Err(format!("default value on parameter `{}`", p.name.name));
        }
    }
    if f.name.name == "main" && !f.params.is_empty() {
        return Err("main takes parameters".into());
    }
    let mut param_scope = HashMap::new();
    for p in &f.params {
        param_scope.insert(p.name.name.clone(), param_jtype(p)?);
    }
    let declared = declared_ret(f);
    let mut returns: Vec<JType> = Vec::new();
    let mut ctx = CheckCtx {
        fns,
        consts,
        modes,
        scopes: vec![param_scope],
        loop_depth: 0,
        returns: &mut returns,
    };
    ctx.check_block(&f.body, true)?;
    unify_returns(declared, &returns, &f.body)
}

/// Resolve a parameter annotation into a JIT type.
fn param_jtype(p: &Param) -> Result<JType, String> {
    match p.ty.as_ref().map(|t| &t.kind) {
        Some(TypeAnnotKind::Primitive(TokenKind::KwInt)) => Ok(JType::Int),
        Some(TypeAnnotKind::Primitive(TokenKind::KwBool)) => Ok(JType::Bool),
        _ => Err("unsupported parameter type".into()),
    }
}

/// Resolve the declared return annotation into a JIT type (Unit when absent or unsupported).
fn declared_ret(f: &FnItem) -> JType {
    match f.ret.as_ref().map(|t| &t.kind) {
        Some(TypeAnnotKind::Primitive(TokenKind::KwInt)) => JType::Int,
        Some(TypeAnnotKind::Primitive(TokenKind::KwBool)) => JType::Bool,
        _ => JType::Unit,
    }
}

/// Unify the collected return types and body tail into the function's ABI return shape.
fn unify_returns(declared: JType, returns: &[JType], body: &Block) -> Result<RetSig, String> {
    match declared {
        JType::Int => {
            if returns.iter().any(|t| *t != JType::Int) {
                return Err("returns a non-Int value".into());
            }
            if !definitely_returns(body) {
                return Err("can fall off the end without returning an Int".into());
            }
            Ok(RetSig::Int)
        }
        JType::Bool => {
            if returns.iter().any(|t| *t != JType::Bool) {
                return Err("returns a non-Bool value".into());
            }
            if !definitely_returns(body) {
                return Err("can fall off the end without returning a Bool".into());
            }
            Ok(RetSig::Bool)
        }
        JType::Unit => {
            if returns.iter().all(|t| *t == JType::Unit) {
                return Ok(RetSig::None);
            }
            if returns.iter().all(|t| *t == JType::Int) && definitely_returns(body) {
                return Ok(RetSig::Int);
            }
            if returns.iter().all(|t| *t == JType::Bool) && definitely_returns(body) {
                return Ok(RetSig::Bool);
            }
            Err("returns inconsistent value types".into())
        }
    }
}

/// Over-approximate whether every path through a block produces a return value.
fn definitely_returns(block: &Block) -> bool {
    if block.tail.is_some() {
        return true;
    }
    match block.stmts.last() {
        Some(Stmt::Return(_, _)) => true,
        Some(Stmt::If { then_body, elseifs, else_body, .. }) => {
            let Some(else_block) = else_body else {
                return false;
            };
            let mut all = definitely_returns(then_body) && definitely_returns(else_block);
            for (_, b) in elseifs {
                all = all && definitely_returns(b);
            }
            all
        }
        _ => false,
    }
}

/// Mutable checking state for one function body.
struct CheckCtx<'a> {
    fns: &'a HashMap<String, FnItem>,
    consts: &'a HashMap<String, ConstVal>,
    modes: &'a HashMap<String, FnMode>,
    scopes: Vec<HashMap<String, JType>>,
    loop_depth: usize,
    returns: &'a mut Vec<JType>,
}

impl<'a> CheckCtx<'a> {
    /// Declare a variable in the innermost scope.
    fn declare(&mut self, name: &str, ty: JType) {
        self.scopes.last_mut().unwrap().insert(name.to_string(), ty);
    }

    /// Look up a variable walking outward through the scope stack.
    fn lookup(&self, name: &str) -> Option<JType> {
        for scope in self.scopes.iter().rev() {
            if let Some(t) = scope.get(name) {
                return Some(*t);
            }
        }
        None
    }

    /// Check a block: every statement, plus the tail when it counts as a return value.
    fn check_block(&mut self, block: &Block, count_tail: bool) -> Result<(), String> {
        self.scopes.push(HashMap::new());
        for s in &block.stmts {
            self.check_stmt(s)?;
        }
        if let Some(t) = &block.tail {
            let ty = self.infer_expr(t)?;
            if count_tail {
                self.returns.push(ty);
            }
        }
        self.scopes.pop();
        Ok(())
    }

    /// Check one statement for compatibility and type consistency.
    fn check_stmt(&mut self, s: &Stmt) -> Result<(), String> {
        match s {
            Stmt::Let { name, value, .. } | Stmt::Var { name, value, .. } => {
                let ty = self.infer_expr(value)?;
                self.declare(&name.name, ty);
                Ok(())
            }
            Stmt::Assign { target, value, op, .. } => {
                let Expr::Ident(id) = target else {
                    return Err("assignment to a non-variable target".into());
                };
                let current = self
                    .lookup(&id.name)
                    .ok_or_else(|| format!("assignment to undeclared variable `{}`", id.name))?;
                let vty = self.infer_expr(value)?;
                let ok = match op {
                    AssignOp::Assign => current == vty,
                    AssignOp::Add | AssignOp::Sub | AssignOp::Mul | AssignOp::Div | AssignOp::Rem => {
                        current == JType::Int && vty == JType::Int
                    }
                    _ => false,
                };
                if ok {
                    Ok(())
                } else {
                    Err(format!("unsupported assignment to `{}`", id.name))
                }
            }
            Stmt::Expr(e, _) => {
                self.infer_expr(e)?;
                Ok(())
            }
            Stmt::If { cond, then_body, elseifs, else_body, .. } => {
                if self.infer_expr(cond)? != JType::Bool {
                    return Err("non-Bool `if` condition".into());
                }
                self.check_block(then_body, false)?;
                for (c, b) in elseifs {
                    if self.infer_expr(c)? != JType::Bool {
                        return Err("non-Bool `elseif` condition".into());
                    }
                    self.check_block(b, false)?;
                }
                if let Some(b) = else_body {
                    self.check_block(b, false)?;
                }
                Ok(())
            }
            Stmt::While { cond, body, .. } => {
                if self.infer_expr(cond)? != JType::Bool {
                    return Err("non-Bool `while` condition".into());
                }
                self.loop_depth += 1;
                let r = self.check_block(body, false);
                self.loop_depth -= 1;
                r
            }
            Stmt::Break(_) if self.loop_depth > 0 => Ok(()),
            Stmt::Continue(_) if self.loop_depth > 0 => Ok(()),
            Stmt::Return(e, _) => {
                let ty = match e {
                    Some(x) => self.infer_expr(x)?,
                    None => JType::Unit,
                };
                self.returns.push(ty);
                Ok(())
            }
            Stmt::Block(b, _) => self.check_block(b, false),
            _ => Err(stmt_reason(s)),
        }
    }

    /// Infer the static type of an expression or explain why it is unsupported.
    fn infer_expr(&mut self, e: &Expr) -> Result<JType, String> {
        match e {
            Expr::IntLit(s, _) => {
                if s.replace('_', "").parse::<i64>().is_ok() {
                    Ok(JType::Int)
                } else {
                    Err(format!("integer literal `{}` outside the i64 range", s))
                }
            }
            Expr::BoolLit(_, _) => Ok(JType::Bool),
            Expr::Ident(id) => {
                if let Some(t) = self.lookup(&id.name) {
                    return Ok(t);
                }
                match self.consts.get(&id.name) {
                    Some(ConstVal::Int(_)) => Ok(JType::Int),
                    Some(ConstVal::Bool(_)) => Ok(JType::Bool),
                    Some(ConstVal::Unsupported) => {
                        Err(format!("constant `{}` has a non-literal value", id.name))
                    }
                    None => {
                        if self.fns.contains_key(&id.name)
                            || id.name == "print"
                            || id.name == "println"
                        {
                            Err(format!("function name `{}` used as a value", id.name))
                        } else {
                            Err(format!("unknown identifier `{}`", id.name))
                        }
                    }
                }
            }
            Expr::BinOp(l, op, r, _) => {
                let lt = self.infer_expr(l)?;
                let rt = self.infer_expr(r)?;
                match op {
                    BinOp::Add | BinOp::Sub | BinOp::Mul | BinOp::Div | BinOp::Rem => {
                        if lt == JType::Int && rt == JType::Int {
                            Ok(JType::Int)
                        } else {
                            Err("non-integer arithmetic".into())
                        }
                    }
                    BinOp::Eq | BinOp::Ne => {
                        if (lt == JType::Int && rt == JType::Int)
                            || (lt == JType::Bool && rt == JType::Bool)
                        {
                            Ok(JType::Bool)
                        } else {
                            Err("comparison of mismatched types".into())
                        }
                    }
                    BinOp::Lt | BinOp::Le | BinOp::Gt | BinOp::Ge => {
                        if lt == JType::Int && rt == JType::Int {
                            Ok(JType::Bool)
                        } else {
                            Err("non-integer comparison".into())
                        }
                    }
                    BinOp::And | BinOp::Or | BinOp::LogicalAnd | BinOp::LogicalOr => {
                        if lt == JType::Bool && rt == JType::Bool {
                            Ok(JType::Bool)
                        } else {
                            Err("non-boolean logic".into())
                        }
                    }
                    _ => Err(format!("unsupported operator `{:?}`", op)),
                }
            }
            Expr::UnaryOp(UnaryOp::Neg, x, _) => {
                if self.infer_expr(x)? == JType::Int {
                    Ok(JType::Int)
                } else {
                    Err("negation of a non-integer".into())
                }
            }
            Expr::UnaryOp(UnaryOp::Not, x, _) => {
                let t = self.infer_expr(x)?;
                if t == JType::Int || t == JType::Bool {
                    Ok(t)
                } else {
                    Err("logical not of a non-boolean".into())
                }
            }
            Expr::UnaryOp(_, _, _) => Err("unsupported unary operator".into()),
            Expr::Call(callee, args, _) => self.infer_call(callee, args),
            _ => Err(expr_reason(e)),
        }
    }

    /// Infer the type of a call expression and validate its arguments.
    fn infer_call(&mut self, callee: &Expr, args: &[CallArg]) -> Result<JType, String> {
        let Expr::Ident(id) = callee else {
            return Err("call of a non-function expression".into());
        };
        let mut arg_tys = Vec::with_capacity(args.len());
        for a in args {
            let CallArg::Positional(e) = a else {
                return Err("named or spread call argument".into());
            };
            arg_tys.push(self.infer_expr(e)?);
        }
        if id.name == "print" || id.name == "println" {
            for t in &arg_tys {
                if !matches!(t, JType::Int | JType::Bool | JType::Unit) {
                    return Err("print of a non-integer, non-boolean value".into());
                }
            }
            return Ok(JType::Unit);
        }
        let Some(f) = self.fns.get(&id.name) else {
            return Err(format!("call to unknown function `{}`", id.name));
        };
        if f.params.len() != arg_tys.len() {
            return Err(format!("call to `{}` with the wrong number of arguments", id.name));
        }
        for (p, t) in f.params.iter().zip(&arg_tys) {
            if param_jtype(p)? != *t {
                return Err(format!("argument type mismatch in a call to `{}`", id.name));
            }
        }
        match self.modes.get(&id.name).copied().unwrap_or(FnMode::Interpreted) {
            FnMode::Jit | FnMode::Trampoline => match declared_ret(f) {
                JType::Int => Ok(JType::Int),
                JType::Bool => Ok(JType::Bool),
                JType::Unit => Err(format!(
                    "call to `{}` in a value position with no declared return type",
                    id.name
                )),
            },
            FnMode::Interpreted => Err(format!("call to non-JIT-compatible function `{}`", id.name)),
        }
    }
}

/// Describe an unsupported statement kind for fallback logging.
fn stmt_reason(s: &Stmt) -> String {
    match s {
        Stmt::Match { .. } => "match statement".into(),
        Stmt::For { .. } => "for loop".into(),
        Stmt::Repeat { .. } => "repeat loop".into(),
        Stmt::Loop { .. } => "loop statement".into(),
        Stmt::Spawn { .. } => "spawn statement".into(),
        Stmt::Unsafe(_, _) => "unsafe block".into(),
        _ => "unsupported statement".into(),
    }
}

/// Describe an unsupported expression kind for fallback logging.
fn expr_reason(e: &Expr) -> String {
    match e {
        Expr::StrLit(_, _) | Expr::RawStrLit(_, _) | Expr::InterpStr(_, _) => "string value".into(),
        Expr::FloatLit(_, _) => "float value".into(),
        Expr::CharLit(_, _) => "char value".into(),
        Expr::Null(_) => "null value".into(),
        Expr::Array(_, _) => "array literal".into(),
        Expr::Map(_, _) => "map literal".into(),
        Expr::Tuple(_, _) => "tuple literal".into(),
        Expr::StructLit(_, _, _) => "struct literal".into(),
        Expr::Closure(_, _, _, _) => "closure".into(),
        Expr::MethodCall(_, _, _, _, _) => "method call".into(),
        Expr::Field(_, _, _) => "field access".into(),
        Expr::Index(_, _, _) => "index access".into(),
        Expr::MatchExpr(_, _, _) => "match expression".into(),
        Expr::IfExpr(_, _, _, _, _) => "if expression".into(),
        Expr::Range(_, _, _, _) => "range expression".into(),
        _ => "unsupported expression".into(),
    }
}

/// Host runtime function declarations reachable from JIT-compiled code.
struct HostDecls {
    print_int: FuncId,
    print_bool: FuncId,
    print_unit: FuncId,
    print_sep: FuncId,
    print_nl: FuncId,
    calls: [FuncId; 5],
}

thread_local! {
    static OUT: RefCell<String> = const { RefCell::new(String::new()) };
    static RT_ERRORS: RefCell<Vec<String>> = const { RefCell::new(Vec::new()) };
    static TRAMPOLINE_INTERP: RefCell<Option<Interpreter>> = const { RefCell::new(None) };
    static TRAMPOLINE_FNS: RefCell<Vec<FnItem>> = const { RefCell::new(Vec::new()) };
    static TRAMPOLINE_ORIG: RefCell<Option<Module>> = const { RefCell::new(None) };
}

/// Append text to the thread-local JIT output buffer.
fn push_out(s: &str) {
    OUT.with(|o| o.borrow_mut().push_str(s));
}

/// Record a runtime error raised inside a fallback trampoline.
fn push_err(msg: String) {
    RT_ERRORS.with(|e| e.borrow_mut().push(msg));
}

/// Host print function: render one integer.
unsafe extern "C" fn axol_jit_print_int(v: i64) {
    push_out(&v.to_string());
}

/// Host print function: render one boolean.
unsafe extern "C" fn axol_jit_print_bool(v: i64) {
    push_out(if v != 0 { "true" } else { "false" });
}

/// Host print function: render the unit value.
unsafe extern "C" fn axol_jit_print_unit() {
    push_out("()");
}

/// Host print function: render the multi-argument separator.
unsafe extern "C" fn axol_jit_print_sep() {
    push_out(" ");
}

/// Host print function: render the trailing newline.
unsafe extern "C" fn axol_jit_print_nl() {
    push_out("\n");
}

/// Host trampoline: re-enter the interpreter for a zero-argument fallback function.
unsafe extern "C" fn axol_jit_call0(fidx: i64) -> i64 {
    run_trampoline(fidx, Vec::new())
}

/// Host trampoline: re-enter the interpreter for a one-argument fallback function.
unsafe extern "C" fn axol_jit_call1(fidx: i64, a0: i64) -> i64 {
    run_trampoline(fidx, vec![a0])
}

/// Host trampoline: re-enter the interpreter for a two-argument fallback function.
unsafe extern "C" fn axol_jit_call2(fidx: i64, a0: i64, a1: i64) -> i64 {
    run_trampoline(fidx, vec![a0, a1])
}

/// Host trampoline: re-enter the interpreter for a three-argument fallback function.
unsafe extern "C" fn axol_jit_call3(fidx: i64, a0: i64, a1: i64, a2: i64) -> i64 {
    run_trampoline(fidx, vec![a0, a1, a2])
}

/// Host trampoline: re-enter the interpreter for a four-argument fallback function.
unsafe extern "C" fn axol_jit_call4(fidx: i64, a0: i64, a1: i64, a2: i64, a3: i64) -> i64 {
    run_trampoline(fidx, vec![a0, a1, a2, a3])
}

/// Execute a fallback function inside the persistent sub-interpreter and marshal the result.
fn run_trampoline(fidx: i64, args: Vec<i64>) -> i64 {
    let item = TRAMPOLINE_FNS.with(|t| t.borrow().get(fidx as usize).cloned());
    let Some(item) = item else {
        push_err(format!("trampoline function index {} is unknown", fidx));
        return 0;
    };
    let mut interp = TRAMPOLINE_INTERP.with(|t| t.borrow_mut().take());
    if interp.is_none() {
        let mut fresh = Interpreter::new();
        let orig = TRAMPOLINE_ORIG.with(|t| t.borrow().clone());
        if let Some(orig) = orig {
            fresh.register_module(&orig);
        }
        interp = Some(fresh);
    }
    let mut interp = interp.unwrap();
    let call = build_trampoline_call(&item, &args);
    let synthetic = Module {
        items: vec![Item::Fn(synthetic_main(call))],
        span: Span::DUMMY,
    };
    let result = interp.run(&synthetic);
    push_out(&interp.output);
    interp.output.clear();
    let value = match result {
        Ok(v) => v,
        Err(diags) => {
            for d in render_errors(&diags) {
                push_err(d);
            }
            Value::Unit
        }
    };
    TRAMPOLINE_INTERP.with(|t| *t.borrow_mut() = Some(interp));
    value_to_i64(value)
}

/// Build the synthetic main function that performs one trampoline call.
fn synthetic_main(call: Expr) -> FnItem {
    FnItem {
        name: Ident { name: "main".to_string(), span: Span::DUMMY },
        generics: Vec::new(),
        params: Vec::new(),
        ret: None,
        body: Block {
            stmts: vec![Stmt::Return(Some(call), Span::DUMMY)],
            tail: None,
            span: Span::DUMMY,
        },
        is_pub: false,
        is_async: false,
        is_extern: false,
        span: Span::DUMMY,
    }
}

/// Build the call expression that re-enters a fallback function with marshaled arguments.
fn build_trampoline_call(item: &FnItem, args: &[i64]) -> Expr {
    let callee = Expr::Ident(Ident { name: item.name.name.clone(), span: Span::DUMMY });
    let call_args = item
        .params
        .iter()
        .zip(args.iter().copied())
        .map(|(p, a)| CallArg::Positional(trampoline_literal(p, a)))
        .collect();
    Expr::Call(Box::new(callee), call_args, Span::DUMMY)
}

/// Build the literal expression for one marshaled trampoline argument.
fn trampoline_literal(p: &Param, a: i64) -> Expr {
    if param_jtype(p) == Ok(JType::Bool) {
        Expr::BoolLit(a != 0, Span::DUMMY)
    } else {
        Expr::IntLit(a.to_string(), Span::DUMMY)
    }
}

/// Marshal an interpreter value back into the i64 trampoline ABI.
fn value_to_i64(v: Value) -> i64 {
    match v {
        Value::Int(n) => n,
        Value::Bool(b) => b as i64,
        _ => {
            push_err("fallback function returned a non-integer value".to_string());
            0
        }
    }
}

/// Reset the thread-local JIT runtime state and register the fallback function table.
fn reset_runtime(orig: Module, tramp_fns: Vec<FnItem>) {
    OUT.with(|o| o.borrow_mut().clear());
    RT_ERRORS.with(|e| e.borrow_mut().clear());
    TRAMPOLINE_INTERP.with(|t| *t.borrow_mut() = None);
    TRAMPOLINE_FNS.with(|t| *t.borrow_mut() = tramp_fns);
    TRAMPOLINE_ORIG.with(|t| *t.borrow_mut() = Some(orig));
}

/// Declare all host runtime functions as JIT imports.
fn declare_host(jit: &mut JITModule) -> Result<HostDecls, String> {
    let print_int = declare_import(jit, "axol_jit_print_int", 1, false)?;
    let print_bool = declare_import(jit, "axol_jit_print_bool", 1, false)?;
    let print_unit = declare_import(jit, "axol_jit_print_unit", 0, false)?;
    let print_sep = declare_import(jit, "axol_jit_print_sep", 0, false)?;
    let print_nl = declare_import(jit, "axol_jit_print_nl", 0, false)?;
    let mut call_ids: Vec<FuncId> = Vec::with_capacity(5);
    for i in 0..5 {
        call_ids.push(declare_import(jit, &format!("axol_jit_call{}", i), i + 1, true)?);
    }
    Ok(HostDecls {
        print_int,
        print_bool,
        print_unit,
        print_sep,
        print_nl,
        calls: [call_ids[0], call_ids[1], call_ids[2], call_ids[3], call_ids[4]],
    })
}

/// Declare one host import with an all-i64 signature.
fn declare_import(jit: &mut JITModule, name: &str, params: usize, returns: bool) -> Result<FuncId, String> {
    let mut sig = jit.make_signature();
    for _ in 0..params {
        sig.params.push(AbiParam::new(types::I64));
    }
    if returns {
        sig.returns.push(AbiParam::new(types::I64));
    }
    jit.declare_function(name, Linkage::Import, &sig).map_err(|e| e.to_string())
}

/// Fill an all-i64 ABI signature for an analyzed function.
fn fill_signature(sig: &mut Signature, f: &FnItem, ret: RetSig) {
    for _ in &f.params {
        sig.params.push(AbiParam::new(types::I64));
    }
    if ret != RetSig::None {
        sig.returns.push(AbiParam::new(types::I64));
    }
}

/// Build the JIT module, compile every planned function, and run main natively.
fn jit_execute(module: &Module, analysis: &Analysis) -> Result<String, Vec<String>> {
    let mut jit_names: Vec<String> = analysis
        .fns
        .iter()
        .filter(|(_, fa)| fa.mode == FnMode::Jit)
        .map(|(n, _)| n.clone())
        .collect();
    jit_names.sort();
    let mut tramp_names: Vec<String> = analysis
        .fns
        .iter()
        .filter(|(_, fa)| fa.mode == FnMode::Trampoline)
        .map(|(n, _)| n.clone())
        .collect();
    tramp_names.sort();
    let tramp_fns: Vec<FnItem> = tramp_names.iter().map(|n| analysis.fns[n].item.clone()).collect();
    let ast_fns: HashMap<String, FnItem> = analysis
        .fns
        .iter()
        .map(|(n, fa)| (n.clone(), fa.item.clone()))
        .collect();
    reset_runtime(module.clone(), tramp_fns);

    let mut builder = JITBuilder::new(default_libcall_names())
        .map_err(|e| vec![format!("jit backend init failed: {}", e)])?;
    builder.symbol("axol_jit_print_int", axol_jit_print_int as *const u8);
    builder.symbol("axol_jit_print_bool", axol_jit_print_bool as *const u8);
    builder.symbol("axol_jit_print_unit", axol_jit_print_unit as *const u8);
    builder.symbol("axol_jit_print_sep", axol_jit_print_sep as *const u8);
    builder.symbol("axol_jit_print_nl", axol_jit_print_nl as *const u8);
    builder.symbol("axol_jit_call0", axol_jit_call0 as *const u8);
    builder.symbol("axol_jit_call1", axol_jit_call1 as *const u8);
    builder.symbol("axol_jit_call2", axol_jit_call2 as *const u8);
    builder.symbol("axol_jit_call3", axol_jit_call3 as *const u8);
    builder.symbol("axol_jit_call4", axol_jit_call4 as *const u8);
    let mut jit = JITModule::new(builder);

    let host = declare_host(&mut jit).map_err(|e| vec![format!("jit host registration failed: {}", e)])?;
    let tramp: HashMap<String, usize> = tramp_names.iter().enumerate().map(|(i, n)| (n.clone(), i)).collect();

    let mut fn_ids: HashMap<String, (FuncId, RetSig)> = HashMap::new();
    for name in &jit_names {
        let fa = &analysis.fns[name];
        let mut sig = jit.make_signature();
        fill_signature(&mut sig, &fa.item, fa.ret);
        let fid = jit
            .declare_function(name, Linkage::Export, &sig)
            .map_err(|e| vec![format!("jit declaration of `{}` failed: {}", name, e)])?;
        fn_ids.insert(name.clone(), (fid, fa.ret));
    }
    for name in &jit_names {
        let fa = &analysis.fns[name];
        let (fid, _) = fn_ids[name];
        compile_function(&mut jit, &fa.item, fa.ret, fid, &fn_ids, &host, &tramp, &ast_fns, &analysis.consts)
            .map_err(|e| vec![format!("jit compilation of `{}` failed: {}", name, e)])?;
    }
    jit.finalize_definitions()
        .map_err(|e| vec![format!("jit finalization failed: {}", e)])?;
    let (main_id, main_ret) = fn_ids
        .get("main")
        .copied()
        .ok_or_else(|| vec!["main was not compiled".to_string()])?;
    let ptr = jit.get_finalized_function(main_id);
    unsafe { call_main(ptr, main_ret) };
    let errors = RT_ERRORS.with(|e| std::mem::take(&mut *e.borrow_mut()));
    if !errors.is_empty() {
        return Err(errors);
    }
    Ok(OUT.with(|o| std::mem::take(&mut *o.borrow_mut())))
}

/// Invoke the compiled main pointer with the signature shape it was built with.
unsafe fn call_main(ptr: *const u8, ret: RetSig) {
    match ret {
        RetSig::None => {
            let f: unsafe extern "C" fn() = unsafe { std::mem::transmute(ptr) };
            unsafe { f() };
        }
        _ => {
            let f: unsafe extern "C" fn() -> i64 = unsafe { std::mem::transmute(ptr) };
            unsafe { f() };
        }
    }
}

/// Compile one analyzed function into native code inside the JIT module.
fn compile_function(
    jit: &mut JITModule,
    item: &FnItem,
    ret: RetSig,
    fid: FuncId,
    fn_ids: &HashMap<String, (FuncId, RetSig)>,
    host: &HostDecls,
    tramp: &HashMap<String, usize>,
    ast_fns: &HashMap<String, FnItem>,
    consts: &HashMap<String, ConstVal>,
) -> Result<(), String> {
    let mut ctx = jit.make_context();
    fill_signature(&mut ctx.func.signature, item, ret);
    ctx.func.name = UserFuncName::user(0, fid.as_u32());
    let frontend = jit.isa().frontend_config();
    let mut fbc = FunctionBuilderContext::new();
    let mut b = FunctionBuilder::new(&mut ctx.func, &mut fbc);
    let entry = b.create_block();
    b.append_block_params_for_function_params(entry);
    b.switch_to_block(entry);
    b.seal_block(entry);
    let params: Vec<IrValue> = b.block_params(entry).to_vec();
    let mut cg = Codegen::new(jit, fn_ids, host, tramp, ast_fns, consts);
    for (p, v) in item.params.iter().zip(params) {
        let ty = param_jtype(p)?;
        cg.bind(&mut b, &p.name.name, ty, v);
    }
    let result = cg.body(&mut b, item, ret);
    drop(cg);
    result?;
    b.finalize(frontend);
    jit.define_function(fid, &mut ctx).map_err(|e| e.to_string())?;
    Ok(())
}

/// A compiled expression: a cranelift value tagged with its static type.
enum JitVal {
    Int(IrValue),
    Bool(IrValue),
    Unit,
}

impl JitVal {
    /// Return the static type of this value.
    fn ty(&self) -> JType {
        match self {
            JitVal::Int(_) => JType::Int,
            JitVal::Bool(_) => JType::Bool,
            JitVal::Unit => JType::Unit,
        }
    }

    /// Return the raw IR value, or None for the unit value.
    fn raw(&self) -> Option<IrValue> {
        match self {
            JitVal::Int(v) | JitVal::Bool(v) => Some(*v),
            JitVal::Unit => None,
        }
    }
}

/// Per-function code generation state over the cranelift IR builder.
struct Codegen<'a> {
    jit: &'a mut JITModule,
    fns: &'a HashMap<String, FnItem>,
    fn_ids: &'a HashMap<String, (FuncId, RetSig)>,
    host: &'a HostDecls,
    tramp: &'a HashMap<String, usize>,
    consts: &'a HashMap<String, ConstVal>,
    vars: HashMap<String, (Variable, JType)>,
    loops: Vec<(IrBlock, IrBlock)>,
    terminated: bool,
}

impl<'a> Codegen<'a> {
    /// Construct the code generator for one function.
    fn new(
        jit: &'a mut JITModule,
        fn_ids: &'a HashMap<String, (FuncId, RetSig)>,
        host: &'a HostDecls,
        tramp: &'a HashMap<String, usize>,
        fns: &'a HashMap<String, FnItem>,
        consts: &'a HashMap<String, ConstVal>,
    ) -> Codegen<'a> {
        Codegen {
            jit,
            fns,
            fn_ids,
            host,
            tramp,
            consts,
            vars: HashMap::new(),
            loops: Vec::new(),
            terminated: false,
        }
    }

    /// Emit a fresh unreachable block when the current block is already terminated.
    fn ensure_open(&mut self, b: &mut FunctionBuilder) {
        if self.terminated {
            let dead = b.create_block();
            b.switch_to_block(dead);
            b.seal_block(dead);
            self.terminated = false;
        }
    }

    /// Compile the function body statements, tail, and implicit return.
    fn body(&mut self, b: &mut FunctionBuilder, item: &FnItem, ret: RetSig) -> Result<(), String> {
        for s in &item.body.stmts {
            self.stmt(b, s)?;
        }
        if let Some(t) = &item.body.tail {
            if !self.terminated {
                let v = self.expr(b, t)?;
                match v {
                    JitVal::Int(x) | JitVal::Bool(x) => {
                        b.ins().return_(&[x]);
                    }
                    JitVal::Unit => {
                        b.ins().return_(&[]);
                    }
                }
                self.terminated = true;
            }
        }
        if !self.terminated {
            match ret {
                RetSig::None => {
                    b.ins().return_(&[]);
                }
                _ => {
                    let z = b.ins().iconst(types::I64, 0);
                    b.ins().return_(&[z]);
                }
            }
            self.terminated = true;
        }
        Ok(())
    }

    /// Compile a nested block's statements and discarded tail expression.
    fn block(&mut self, b: &mut FunctionBuilder, blk: &Block) -> Result<(), String> {
        for s in &blk.stmts {
            self.stmt(b, s)?;
        }
        if let Some(t) = &blk.tail {
            self.expr(b, t)?;
        }
        Ok(())
    }

    /// Compile one statement into the current block.
    fn stmt(&mut self, b: &mut FunctionBuilder, s: &Stmt) -> Result<(), String> {
        match s {
            Stmt::Let { name, value, .. } | Stmt::Var { name, value, .. } => {
                self.ensure_open(b);
                let v = self.expr(b, value)?;
                let material = self.materialize(b, &v);
                self.bind(b, &name.name, v.ty(), material);
                Ok(())
            }
            Stmt::Assign { target, value, op, .. } => {
                self.ensure_open(b);
                let Expr::Ident(id) = target else {
                    return Err("assignment to a non-variable target".into());
                };
                let rhs = self.expr(b, value)?;
                let (var, _) = self
                    .vars
                    .get(&id.name)
                    .copied()
                    .ok_or_else(|| format!("assignment to undeclared variable `{}`", id.name))?;
                let rhs_val = self.materialize(b, &rhs);
                match op {
                    AssignOp::Assign => {
                        b.def_var(var, rhs_val);
                    }
                    AssignOp::Add | AssignOp::Sub | AssignOp::Mul | AssignOp::Div | AssignOp::Rem => {
                        let cur = b.use_var(var);
                        let next = match op {
                            AssignOp::Add => b.ins().iadd(cur, rhs_val),
                            AssignOp::Sub => b.ins().isub(cur, rhs_val),
                            AssignOp::Mul => b.ins().imul(cur, rhs_val),
                            AssignOp::Div => b.ins().sdiv(cur, rhs_val),
                            _ => b.ins().srem(cur, rhs_val),
                        };
                        b.def_var(var, next);
                    }
                    _ => return Err("unsupported assignment operator".into()),
                }
                Ok(())
            }
            Stmt::Expr(e, _) => {
                self.ensure_open(b);
                self.expr(b, e)?;
                Ok(())
            }
            Stmt::If { cond, then_body, elseifs, else_body, .. } => {
                self.ensure_open(b);
                let end = b.create_block();
                self.branch(b, cond, then_body, elseifs, else_body, end)?;
                b.seal_block(end);
                b.switch_to_block(end);
                self.terminated = false;
                Ok(())
            }
            Stmt::While { cond, body, .. } => {
                self.ensure_open(b);
                let header = b.create_block();
                let body_block = b.create_block();
                let exit = b.create_block();
                b.ins().jump(header, &[]);
                self.terminated = true;
                b.switch_to_block(header);
                self.terminated = false;
                let c = self.expr(b, cond)?;
                let raw = c.raw().ok_or("non-boolean `while` condition")?;
                b.ins().brif(raw, body_block, &[], exit, &[]);
                self.terminated = true;
                b.switch_to_block(body_block);
                self.terminated = false;
                self.loops.push((header, exit));
                let r = self.block(b, body);
                if !self.terminated {
                    b.ins().jump(header, &[]);
                    self.terminated = true;
                }
                self.loops.pop();
                r?;
                b.seal_block(body_block);
                b.seal_block(header);
                b.seal_block(exit);
                b.switch_to_block(exit);
                self.terminated = false;
                Ok(())
            }
            Stmt::Break(_) => {
                self.ensure_open(b);
                let (_, exit) = *self.loops.last().ok_or("`break` outside a loop")?;
                b.ins().jump(exit, &[]);
                self.terminated = true;
                Ok(())
            }
            Stmt::Continue(_) => {
                self.ensure_open(b);
                let (header, _) = *self.loops.last().ok_or("`continue` outside a loop")?;
                b.ins().jump(header, &[]);
                self.terminated = true;
                Ok(())
            }
            Stmt::Return(e, _) => {
                self.ensure_open(b);
                match e {
                    Some(x) => {
                        let v = self.expr(b, x)?;
                        match v {
                            JitVal::Int(a) | JitVal::Bool(a) => {
                                b.ins().return_(&[a]);
                            }
                            JitVal::Unit => {
                                b.ins().return_(&[]);
                            }
                        }
                    }
                    None => {
                        b.ins().return_(&[]);
                    }
                }
                self.terminated = true;
                Ok(())
            }
            Stmt::Block(blk, _) => self.block(b, blk),
            _ => Err(stmt_reason(s)),
        }
    }

    /// Compile an if/elseif/else chain into branches converging on one end block.
    fn branch(
        &mut self,
        b: &mut FunctionBuilder,
        cond: &Expr,
        then_body: &Block,
        elseifs: &[(Expr, Block)],
        else_body: &Option<Block>,
        end: IrBlock,
    ) -> Result<(), String> {
        let c = self.expr(b, cond)?;
        let raw = c.raw().ok_or("non-boolean `if` condition")?;
        let then_block = b.create_block();
        let cont = b.create_block();
        b.ins().brif(raw, then_block, &[], cont, &[]);
        self.terminated = true;
        b.switch_to_block(then_block);
        self.terminated = false;
        b.seal_block(then_block);
        self.block(b, then_body)?;
        if !self.terminated {
            b.ins().jump(end, &[]);
            self.terminated = true;
        }
        b.switch_to_block(cont);
        self.terminated = false;
        b.seal_block(cont);
        if let Some(((ec, eb), rest)) = elseifs.split_first() {
            self.branch(b, ec, eb, rest, else_body, end)
        } else if let Some(eb) = else_body {
            self.block(b, eb)?;
            if !self.terminated {
                b.ins().jump(end, &[]);
                self.terminated = true;
            }
            Ok(())
        } else {
            b.ins().jump(end, &[]);
            self.terminated = true;
            Ok(())
        }
    }

    /// Compile an expression and return its value tagged with its static type.
    fn expr(&mut self, b: &mut FunctionBuilder, e: &Expr) -> Result<JitVal, String> {
        match e {
            Expr::IntLit(s, _) => {
                let n: i64 = s
                    .replace('_', "")
                    .parse()
                    .map_err(|_| format!("integer literal `{}` outside the i64 range", s))?;
                Ok(JitVal::Int(b.ins().iconst(types::I64, n)))
            }
            Expr::BoolLit(v, _) => Ok(JitVal::Bool(b.ins().iconst(types::I64, *v as i64))),
            Expr::Ident(id) => {
                if let Some((var, ty)) = self.vars.get(&id.name).copied() {
                    let v = b.use_var(var);
                    return Ok(match ty {
                        JType::Int => JitVal::Int(v),
                        JType::Bool => JitVal::Bool(v),
                        JType::Unit => JitVal::Unit,
                    });
                }
                match self.consts.get(&id.name) {
                    Some(ConstVal::Int(n)) => Ok(JitVal::Int(b.ins().iconst(types::I64, *n))),
                    Some(ConstVal::Bool(v)) => {
                        Ok(JitVal::Bool(b.ins().iconst(types::I64, *v as i64)))
                    }
                    _ => Err(format!("unknown identifier `{}`", id.name)),
                }
            }
            Expr::BinOp(l, op, r, _) => {
                let lv = self.expr(b, l)?;
                let rv = self.expr(b, r)?;
                let a = lv.raw().ok_or("unit operand in arithmetic")?;
                let c = rv.raw().ok_or("unit operand in arithmetic")?;
                match op {
                    BinOp::Add => Ok(JitVal::Int(b.ins().iadd(a, c))),
                    BinOp::Sub => Ok(JitVal::Int(b.ins().isub(a, c))),
                    BinOp::Mul => Ok(JitVal::Int(b.ins().imul(a, c))),
                    BinOp::Div => Ok(JitVal::Int(b.ins().sdiv(a, c))),
                    BinOp::Rem => Ok(JitVal::Int(b.ins().srem(a, c))),
                    BinOp::Eq => {
                        let r = b.ins().icmp(IntCC::Equal, a, c);
                        Ok(JitVal::Bool(self.widen(b, r)))
                    }
                    BinOp::Ne => {
                        let r = b.ins().icmp(IntCC::NotEqual, a, c);
                        Ok(JitVal::Bool(self.widen(b, r)))
                    }
                    BinOp::Lt => {
                        let r = b.ins().icmp(IntCC::SignedLessThan, a, c);
                        Ok(JitVal::Bool(self.widen(b, r)))
                    }
                    BinOp::Le => {
                        let r = b.ins().icmp(IntCC::SignedLessThanOrEqual, a, c);
                        Ok(JitVal::Bool(self.widen(b, r)))
                    }
                    BinOp::Gt => {
                        let r = b.ins().icmp(IntCC::SignedGreaterThan, a, c);
                        Ok(JitVal::Bool(self.widen(b, r)))
                    }
                    BinOp::Ge => {
                        let r = b.ins().icmp(IntCC::SignedGreaterThanOrEqual, a, c);
                        Ok(JitVal::Bool(self.widen(b, r)))
                    }
                    BinOp::And | BinOp::LogicalAnd => Ok(JitVal::Bool(b.ins().band(a, c))),
                    BinOp::Or | BinOp::LogicalOr => Ok(JitVal::Bool(b.ins().bor(a, c))),
                    _ => Err(format!("unsupported operator `{:?}`", op)),
                }
            }
            Expr::UnaryOp(UnaryOp::Neg, x, _) => {
                let v = self.expr(b, x)?;
                let a = v.raw().ok_or("negation of a unit value")?;
                Ok(JitVal::Int(b.ins().ineg(a)))
            }
            Expr::UnaryOp(UnaryOp::Not, x, _) => {
                let v = self.expr(b, x)?;
                let a = v.raw().ok_or("logical not of a unit value")?;
                let mask = match v.ty() {
                    JType::Int => b.ins().iconst(types::I64, -1),
                    _ => b.ins().iconst(types::I64, 1),
                };
                let out = b.ins().bxor(a, mask);
                Ok(match v.ty() {
                    JType::Int => JitVal::Int(out),
                    _ => JitVal::Bool(out),
                })
            }
            Expr::UnaryOp(_, _, _) => Err("unsupported unary operator".into()),
            Expr::Call(callee, args, _) => self.call(b, callee, args),
            _ => Err(expr_reason(e)),
        }
    }

    /// Normalize an I8 comparison result into the I64 boolean representation.
    fn widen(&mut self, b: &mut FunctionBuilder, v: IrValue) -> IrValue {
        if b.func.dfg.value_type(v) == types::I8 {
            b.ins().uextend(types::I64, v)
        } else {
            v
        }
    }

    /// Materialize a unit value as a zero so it can be bound to a variable slot.
    fn materialize(&mut self, b: &mut FunctionBuilder, v: &JitVal) -> IrValue {
        v.raw().unwrap_or_else(|| b.ins().iconst(types::I64, 0))
    }

    /// Bind a variable, declaring its cranelift slot on first use.
    fn bind(&mut self, b: &mut FunctionBuilder, name: &str, ty: JType, val: IrValue) {
        if let Some((var, _)) = self.vars.get(name).copied() {
            b.def_var(var, val);
        } else {
            let var = b.declare_var(types::I64);
            b.def_var(var, val);
            self.vars.insert(name.to_string(), (var, ty));
        }
    }

    /// Emit a call to a zero-argument host function.
    fn host_call0(&mut self, b: &mut FunctionBuilder, fid: FuncId) -> Result<(), String> {
        let fr = self.jit.declare_func_in_func(fid, b.func);
        b.ins().call(fr, &[]);
        Ok(())
    }

    /// Emit a call to a one-argument host function.
    fn host_call1(&mut self, b: &mut FunctionBuilder, fid: FuncId, arg: IrValue) -> Result<(), String> {
        let fr = self.jit.declare_func_in_func(fid, b.func);
        b.ins().call(fr, &[arg]);
        Ok(())
    }

    /// Compile a call to print, another compiled function, or a trampoline.
    fn call(&mut self, b: &mut FunctionBuilder, callee: &Expr, args: &[CallArg]) -> Result<JitVal, String> {
        let Expr::Ident(id) = callee else {
            return Err("call of a non-function expression".into());
        };
        let mut vals = Vec::with_capacity(args.len());
        for a in args {
            let CallArg::Positional(e) = a else {
                return Err("named or spread call argument".into());
            };
            vals.push(self.expr(b, e)?);
        }
        if id.name == "print" || id.name == "println" {
            for (i, v) in vals.iter().enumerate() {
                if i > 0 {
                    self.host_call0(b, self.host.print_sep)?;
                }
                match v {
                    JitVal::Int(x) => {
                        self.host_call1(b, self.host.print_int, *x)?;
                    }
                    JitVal::Bool(x) => {
                        self.host_call1(b, self.host.print_bool, *x)?;
                    }
                    JitVal::Unit => {
                        self.host_call0(b, self.host.print_unit)?;
                    }
                }
            }
            self.host_call0(b, self.host.print_nl)?;
            return Ok(JitVal::Unit);
        }
        if let Some(idx) = self.tramp.get(&id.name) {
            let f = self
                .fns
                .get(&id.name)
                .ok_or_else(|| format!("trampoline target `{}` is unknown", id.name))?;
            let host_id = self.host.calls[vals.len().min(4)];
            let fr = self.jit.declare_func_in_func(host_id, b.func);
            let mut cargs = vec![b.ins().iconst(types::I64, *idx as i64)];
            for v in &vals {
                cargs.push(v.raw().ok_or("unit value passed as a call argument")?);
            }
            let inst = b.ins().call(fr, &cargs);
            match declared_ret(f) {
                JType::Int => Ok(JitVal::Int(b.inst_results(inst)[0])),
                JType::Bool => Ok(JitVal::Bool(b.inst_results(inst)[0])),
                JType::Unit => Ok(JitVal::Unit),
            }
        } else {
            let Some((fid, fret)) = self.fn_ids.get(&id.name).copied() else {
                return Err(format!("call to non-compiled function `{}`", id.name));
            };
            let fr = self.jit.declare_func_in_func(fid, b.func);
            let mut cargs = Vec::with_capacity(vals.len());
            for v in &vals {
                cargs.push(v.raw().ok_or("unit value passed as a call argument")?);
            }
            let inst = b.ins().call(fr, &cargs);
            match fret {
                RetSig::None => Ok(JitVal::Unit),
                RetSig::Int => Ok(JitVal::Int(b.inst_results(inst)[0])),
                RetSig::Bool => Ok(JitVal::Bool(b.inst_results(inst)[0])),
            }
        }
    }
}
