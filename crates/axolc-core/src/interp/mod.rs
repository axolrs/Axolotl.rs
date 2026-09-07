// Owner: PascalElixir / axolrs (GitHub org)
// File: Tree-walking interpreter for Axolotl dev mode (sub-200ms iteration).

use crate::ast::*;
use crate::diag::{Diagnostic, Diagnostics};
use crate::span::Span;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

/// A struct value with its runtime type tag and field map.
#[derive(Debug, Clone)]
pub struct StructVal {
    pub ty: Rc<String>,
    pub fields: Rc<HashMap<String, Value>>,
}

/// A runtime value produced by the interpreter.
#[derive(Debug, Clone)]
pub enum Value {
    Unit,
    Int(i64),
    UInt(u64),
    Float(f64),
    Bool(bool),
    Str(Rc<String>),
    Char(char),
    Array(Rc<Vec<Value>>),
    Tuple(Rc<Vec<Value>>),
    Map(Rc<HashMap<String, Value>>),
    Struct(StructVal),
    EnumVariant { name: String, fields: Vec<Value> },
    Closure { params: Vec<Param>, body: Rc<Block>, env: Env },
    Null,
}

/// An environment of variable bindings.
#[derive(Debug, Clone, Default)]
pub struct Env {
    pub bindings: Rc<std::cell::RefCell<HashMap<String, Value>>>,
    pub parent: Option<Rc<Env>>,
}

impl Env {
    /// Construct an empty environment.
    pub fn new() -> Env {
        Env {
            bindings: Rc::new(std::cell::RefCell::new(HashMap::new())),
            parent: None,
        }
    }

    /// Construct a child environment.
    pub fn child(parent: &Rc<Env>) -> Env {
        Env {
            bindings: Rc::new(std::cell::RefCell::new(HashMap::new())),
            parent: Some(parent.clone()),
        }
    }

    /// Insert a binding into the current scope.
    pub fn insert(&self, name: impl Into<String>, value: Value) {
        self.bindings.borrow_mut().insert(name.into(), value);
    }

    /// Lookup a binding, walking up the scope chain.
    pub fn lookup(&self, name: &str) -> Option<Value> {
        if let Some(v) = self.bindings.borrow().get(name) {
            return Some(v.clone());
        }
        if let Some(p) = &self.parent {
            return p.lookup(name);
        }
        None
    }

    /// Update an existing binding or insert a new one.
    pub fn assign(&self, name: &str, value: Value) -> bool {
        if self.bindings.borrow().contains_key(name) {
            self.bindings.borrow_mut().insert(name.to_string(), value);
            return true;
        }
        if let Some(p) = &self.parent {
            return p.assign(name, value);
        }
        false
    }
}

/// An interpreter that walks the HIR.
pub struct Interpreter {
    env: Rc<Env>,
    diagnostics: Diagnostics,
    /// Global function table.
    functions: HashMap<String, FnItem>,
    /// Struct definitions keyed by type name.
    structs: HashMap<String, StructItem>,
    /// Method table keyed by `"Type.method"`.
    methods: HashMap<String, MethodItem>,
    /// Output buffer for `print` calls.
    pub output: String,
}

/// A control-flow signal raised during evaluation.
enum Signal {
    Normal(Value),
    Return(Value),
    Break,
    Continue,
}

/// Return the root variable name an lvalue writes through, if any.
fn root_binding_name(e: &Expr) -> Option<String> {
    match e {
        Expr::Ident(id) => Some(id.name.clone()),
        Expr::SelfExpr(_) => Some("self".to_string()),
        Expr::Field(recv, _, _) | Expr::Index(recv, _, _) => match recv.as_ref() {
            Expr::Ident(id) => Some(id.name.clone()),
            inner => root_binding_name(inner),
        },
        _ => None,
    }
}

/// Extract the static field key of a map/struct literal entry without evaluating it as a variable.
fn static_key_of(key: &Expr) -> String {
    match key {
        Expr::Ident(id) => id.name.clone(),
        Expr::StrLit(s, _) => s.clone(),
        Expr::IntLit(s, _) => s.replace('_', ""),
        _ => "".to_string(),
    }
}

impl Interpreter {
    /// Construct a fresh interpreter with an empty global scope.
    pub fn new() -> Interpreter {
        Interpreter {
            env: Rc::new(Env::new()),
            diagnostics: Diagnostics::new(),
            functions: HashMap::new(),
            structs: HashMap::new(),
            methods: HashMap::new(),
            output: String::new(),
        }
    }

    /// Consume the interpreter and return its diagnostics.
    pub fn into_diagnostics(self) -> Diagnostics {
        self.diagnostics
    }

    /// Register a module's items (functions, constants, structs, methods) into the interpreter's global scope.
    pub fn register_module(&mut self, module: &Module) {
        for item in &module.items {
            match item {
                Item::Fn(f) => {
                    self.functions.insert(f.name.name.clone(), f.clone());
                }
                Item::Const(c) => {
                    if let Ok(v) = self.eval_expr(&c.value, &Rc::new(Env::child(&self.env))) {
                        self.env.insert(c.name.name.clone(), v);
                    }
                }
                Item::Struct(s) => {
                    self.structs.insert(s.name.name.clone(), s.clone());
                }
                Item::Method(m) | Item::StaticMethod(m) => {
                    let key = format!("{}:{}", m.receiver.name, m.name.name);
                    self.methods.insert(key, m.clone());
                }
                _ => {}
            }
        }
    }

    /// Register a lowered HIR module and run its `main`.
    pub fn run_hir(&mut self, module: &crate::hir::Module) -> Result<Value, Diagnostics> {
        for item in &module.items {
            match item {
                crate::hir::Item::Fn(f) => {
                    self.functions.insert(f.name.name.clone(), f.clone());
                }
                crate::hir::Item::Const(c) => {
                    if let Ok(v) = self.eval_expr(&c.value, &Rc::new(Env::child(&self.env))) {
                        self.env.insert(c.name.name.clone(), v);
                    }
                }
                crate::hir::Item::Struct(s) => {
                    self.structs.insert(s.name.name.clone(), s.clone());
                }
                crate::hir::Item::Impl(imp) => {
                    for m in &imp.methods {
                        let key = format!("{}:{}", imp.ty_name.name, m.name.name);
                        self.methods.insert(key, m.clone());
                    }
                    for m in &imp.static_methods {
                        let key = format!("{}:{}", imp.ty_name.name, m.name.name);
                        self.methods.insert(key, m.clone());
                    }
                }
                _ => {}
            }
        }
        if let Some(main) = self.functions.get("main").cloned() {
            let env = Rc::new(Env::child(&self.env));
            match self.exec_block(&main.body, &env) {
                Ok(Signal::Normal(v)) | Ok(Signal::Return(v)) => return Ok(v),
                Ok(_) => return Ok(Value::Unit),
                Err(()) => {}
            }
        }
        if self.diagnostics.has_errors() {
            return Err(std::mem::take(&mut self.diagnostics));
        }
        Ok(Value::Unit)
    }

    /// Run the module by registering items then calling `main` if present.
    pub fn run(&mut self, module: &Module) -> Result<Value, Diagnostics> {
        self.register_module(module);
        if let Some(main) = self.functions.get("main").cloned() {
            let env = Rc::new(Env::child(&self.env));
            match self.exec_block(&main.body, &env) {
                Ok(Signal::Normal(v)) | Ok(Signal::Return(v)) => return Ok(v),
                Ok(_) => return Ok(Value::Unit),
                Err(()) => {}
            }
        }
        if self.diagnostics.has_errors() {
            return Err(std::mem::take(&mut self.diagnostics));
        }
        Ok(Value::Unit)
    }

    fn exec_block(&mut self, block: &Block, env: &Rc<Env>) -> Result<Signal, ()> {
        for s in &block.stmts {
            match self.exec_stmt(s, env)? {
                Signal::Normal(_) => {}
                other => return Ok(other),
            }
        }
        if let Some(tail) = &block.tail {
            let v = self.eval_expr(tail, env)?;
            Ok(Signal::Normal(v))
        } else {
            Ok(Signal::Normal(Value::Unit))
        }
    }

    fn exec_stmt(&mut self, s: &Stmt, env: &Rc<Env>) -> Result<Signal, ()> {
        match s {
            Stmt::Let { name, value, .. } | Stmt::Var { name, value, .. } => {
                let v = self.eval_expr(value, env)?;
                env.insert(name.name.clone(), v);
                Ok(Signal::Normal(Value::Unit))
            }
            Stmt::Assign { target, value, op, span } => {
                let v = self.eval_expr(value, env)?;
                if let Expr::Ident(id) = target {
                    if let Some(existing) = env.lookup(&id.name) {
                        let new_v = self.apply_assign(existing, v, *op);
                        env.assign(&id.name, new_v);
                    } else {
                        env.insert(id.name.clone(), v);
                    }
                } else if let Some(root) = root_binding_name(target) {
                    if let Some(existing) = env.lookup(&root) {
                        let updated = self.update_target(existing, target, v, *op, *span);
                        env.assign(&root, updated);
                    }
                }
                Ok(Signal::Normal(Value::Unit))
            }
            Stmt::Expr(e, _) => {
                let v = self.eval_expr(e, env)?;
                Ok(Signal::Normal(v))
            }
            Stmt::If { cond, then_body, elseifs, else_body, .. } => {
                let c = self.eval_expr(cond, env)?;
                if let Value::Bool(b) = c {
                    if b {
                        return self.exec_block(then_body, env);
                    }
                    for (ec, eb) in elseifs {
                        let cv = self.eval_expr(ec, env)?;
                        if let Value::Bool(true) = cv {
                            return self.exec_block(eb, env);
                        }
                    }
                    if let Some(eb) = else_body {
                        return self.exec_block(eb, env);
                    }
                }
                Ok(Signal::Normal(Value::Unit))
            }
            Stmt::While { cond, body, .. } => {
                loop {
                    let c = self.eval_expr(cond, env)?;
                    if let Value::Bool(false) = c { break; }
                    match self.exec_block(body, env)? {
                        Signal::Break => break,
                        Signal::Continue => continue,
                        Signal::Return(v) => return Ok(Signal::Return(v)),
                        _ => {}
                    }
                }
                Ok(Signal::Normal(Value::Unit))
            }
            Stmt::For { var, iter, body, .. } => {
                if let Expr::Range(lo, hi, inclusive, _) = iter {
                    let lo_v = lo
                        .as_ref()
                        .map(|l| self.eval_expr(l, env))
                        .transpose()?
                        .unwrap_or(Value::Int(0));
                    let hi_v = hi
                        .as_ref()
                        .map(|h| self.eval_expr(h, env))
                        .transpose()?;
                    let start = match lo_v { Value::Int(n) => n, _ => 0 };
                    let end = match hi_v {
                        Some(Value::Int(n)) => n,
                        _ => return Ok(Signal::Normal(Value::Unit)),
                    };
                    let exclusive_end = if *inclusive { end + 1 } else { end };
                    for i in start..exclusive_end {
                        let child = Rc::new(Env::child(env));
                        child.insert(var.name.clone(), Value::Int(i));
                        match self.exec_block(body, &child)? {
                            Signal::Break => break,
                            Signal::Continue => continue,
                            Signal::Return(v) => return Ok(Signal::Return(v)),
                            _ => {}
                        }
                    }
                    return Ok(Signal::Normal(Value::Unit));
                }
                let iter_v = self.eval_expr(iter, env)?;
                if let Value::Array(arr) = iter_v {
                    for item in arr.iter() {
                        let child = Rc::new(Env::child(env));
                        child.insert(var.name.clone(), item.clone());
                        match self.exec_block(body, &child)? {
                            Signal::Break => break,
                            Signal::Continue => continue,
                            Signal::Return(v) => return Ok(Signal::Return(v)),
                            _ => {}
                        }
                    }
                } else if let Value::Int(n) = iter_v {
                    for i in 0..n {
                        let child = Rc::new(Env::child(env));
                        child.insert(var.name.clone(), Value::Int(i));
                        match self.exec_block(body, &child)? {
                            Signal::Break => break,
                            Signal::Continue => continue,
                            Signal::Return(v) => return Ok(Signal::Return(v)),
                            _ => {}
                        }
                    }
                }
                Ok(Signal::Normal(Value::Unit))
            }
            Stmt::Loop { body, .. } => {
                loop {
                    match self.exec_block(body, env)? {
                        Signal::Break => break,
                        Signal::Continue => continue,
                        Signal::Return(v) => return Ok(Signal::Return(v)),
                        _ => {}
                    }
                }
                Ok(Signal::Normal(Value::Unit))
            }
            Stmt::Break(_) => Ok(Signal::Break),
            Stmt::Continue(_) => Ok(Signal::Continue),
            Stmt::Return(e, _) => {
                let v = if let Some(e) = e { self.eval_expr(e, env)? } else { Value::Unit };
                Ok(Signal::Return(v))
            }
            Stmt::Match { scrutinee, arms, .. } => {
                let v = self.eval_expr(scrutinee, env)?;
                for arm in arms {
                    if let Some(bindings) = self.match_pattern(&arm.pattern, &v, env) {
                        let child = Rc::new(Env::child(env));
                        for (n, bv) in bindings {
                            child.insert(n, bv);
                        }
                        if let Some(g) = &arm.guard {
                            let gv = self.eval_expr(g, &child)?;
                            if !matches!(gv, Value::Bool(true)) { continue; }
                        }
                        return self.exec_block(&arm.body, &child);
                    }
                }
                Ok(Signal::Normal(Value::Unit))
            }
            Stmt::Block(b, _) => self.exec_block(b, env),
            Stmt::Unsafe(b, _) => self.exec_block(b, env),
            Stmt::Spawn { .. } => Ok(Signal::Normal(Value::Unit)),
            Stmt::Repeat { body, cond, .. } => {
                loop {
                    match self.exec_block(body, env)? {
                        Signal::Break => break,
                        Signal::Continue => continue,
                        Signal::Return(v) => return Ok(Signal::Return(v)),
                        _ => {}
                    }
                    let c = self.eval_expr(cond, env)?;
                    if let Value::Bool(true) = c { break; }
                }
                Ok(Signal::Normal(Value::Unit))
            }
        }
    }

    /// Recursively update a field/index path on a cloned root value.
    fn update_target(&self, root: Value, target: &Expr, v: Value, op: AssignOp, _span: Span) -> Value {
        match target {
            Expr::Field(recv, name, _) => {
                if matches!(recv.as_ref(), Expr::Ident(_)) {
                    self.set_field(root, &name.name, |old| self.apply_assign(old, v.clone(), op))
                } else {
                    let inner = recv.as_ref().clone();
                    self.update_nested(root, &inner, |cur| {
                        self.set_field(cur, &name.name, |old| self.apply_assign(old, v.clone(), op))
                    })
                }
            }
            Expr::Index(recv, idx, _) => {
                let i = match idx.as_ref() {
                    Expr::IntLit(s, _) => s.replace('_', "").parse::<i64>().unwrap_or(0),
                    _ => return root,
                };
                if matches!(recv.as_ref(), Expr::Ident(_)) {
                    self.set_index(root, i, |old| self.apply_assign(old, v.clone(), op))
                } else {
                    let inner = recv.as_ref().clone();
                    self.update_nested(root, &inner, |cur| {
                        self.set_index(cur, i, |old| self.apply_assign(old, v.clone(), op))
                    })
                }
            }
            _ => root,
        }
    }

    /// Apply `f` to the child of `root` selected by `path`, rebuilding the value spine.
    fn update_nested(&self, root: Value, path: &Expr, f: impl Fn(Value) -> Value) -> Value {
        match path {
            Expr::Field(recv, name, _) => {
                let inner = self.get_field(&root, &name.name);
                let new_inner = self.update_nested(inner, recv, f);
                self.set_field(root, &name.name, |_| new_inner)
            }
            Expr::Index(recv, idx, _) => {
                let i = match idx.as_ref() {
                    Expr::IntLit(s, _) => s.replace('_', "").parse::<i64>().unwrap_or(0),
                    _ => 0,
                };
                let inner = self.get_index(&root, i);
                let new_inner = self.update_nested(inner, recv, f);
                self.set_index(root, i, |_| new_inner)
            }
            _ => f(root),
        }
    }

    /// Read one field from a struct or map value.
    fn get_field(&self, v: &Value, name: &str) -> Value {
        match v {
            Value::Struct(sv) => sv.fields.get(name).cloned().unwrap_or(Value::Unit),
            Value::Map(m) => m.get(name).cloned().unwrap_or(Value::Unit),
            _ => Value::Unit,
        }
    }

    /// Read one index from an array or tuple value.
    fn get_index(&self, v: &Value, i: i64) -> Value {
        match v {
            Value::Array(a) => a.get(i as usize).cloned().unwrap_or(Value::Unit),
            Value::Tuple(t) => t.get(i as usize).cloned().unwrap_or(Value::Unit),
            _ => Value::Unit,
        }
    }

    /// Write one field into a cloned struct or map value.
    fn set_field(&self, v: Value, name: &str, f: impl FnOnce(Value) -> Value) -> Value {
        match v {
            Value::Struct(sv) => {
                let mut fields = (*sv.fields).clone();
                let old = fields.get(name).cloned().unwrap_or(Value::Unit);
                fields.insert(name.to_string(), f(old));
                Value::Struct(StructVal { ty: sv.ty.clone(), fields: Rc::new(fields) })
            }
            Value::Map(m) => {
                let mut fields = (*m).clone();
                let old = fields.get(name).cloned().unwrap_or(Value::Unit);
                fields.insert(name.to_string(), f(old));
                Value::Map(Rc::new(fields))
            }
            other => other,
        }
    }

    /// Write one index into a cloned array or tuple value.
    fn set_index(&self, v: Value, i: i64, f: impl FnOnce(Value) -> Value) -> Value {
        match v {
            Value::Array(a) => {
                let mut items = (*a).clone();
                if i >= 0 && (i as usize) < items.len() {
                    let old = items[i as usize].clone();
                    items[i as usize] = f(old);
                }
                Value::Array(Rc::new(items))
            }
            Value::Tuple(t) => {
                let mut items = (*t).clone();
                if i >= 0 && (i as usize) < items.len() {
                    let old = items[i as usize].clone();
                    items[i as usize] = f(old);
                }
                Value::Tuple(Rc::new(items))
            }
            Value::Map(m) => {
                let mut fields = (*m).clone();
                let key = i.to_string();
                let old = fields.get(&key).cloned().unwrap_or(Value::Unit);
                fields.insert(key, f(old));
                Value::Map(Rc::new(fields))
            }
            other => other,
        }
    }

    fn apply_assign(&self, lhs: Value, rhs: Value, op: AssignOp) -> Value {
        match op {
            AssignOp::Assign => rhs,
            AssignOp::Add => arith(&lhs, &rhs, |a, b| a + b, |a, b| a + b, |a, b| a + b).unwrap_or(rhs),
            AssignOp::Sub => arith(&lhs, &rhs, |a, b| a - b, |a, b| a - b, |a, b| a - b).unwrap_or(rhs),
            AssignOp::Mul => arith(&lhs, &rhs, |a, b| a * b, |a, b| a * b, |a, b| a * b).unwrap_or(rhs),
            AssignOp::Div => arith(&lhs, &rhs, |a, b| a / b, |a, b| a / b, |a, b| a / b).unwrap_or(rhs),
            AssignOp::Rem => arith(&lhs, &rhs, |a, b| a % b, |a, b| a % b, |a, b| a % b).unwrap_or(rhs),
            _ => rhs,
        }
    }

    fn match_pattern(&self, p: &Pattern, v: &Value, _env: &Rc<Env>) -> Option<Vec<(String, Value)>> {
        let mut out = Vec::new();
        match p {
            Pattern::Wild(_) => Some(out),
            Pattern::Lit(lit, _) => {
                let lit_v = self.eval_const_expr(lit).ok()?;
                if value_eq(&lit_v, v) { Some(out) } else { None }
            }
            Pattern::Binding(id) => {
                out.push((id.name.clone(), v.clone()));
                Some(out)
            }
            Pattern::EnumVariant { variant, sub, .. } => {
                if let Value::EnumVariant { name, fields } = v {
                    if name != &variant.name { return None; }
                    if sub.len() != fields.len() { return None; }
                    for (p, v) in sub.iter().zip(fields.iter()) {
                        let mut bs = self.match_pattern(p, v, _env)?;
                        out.append(&mut bs);
                    }
                    Some(out)
                } else { None }
            }
            Pattern::Tuple(elems, _) => {
                if let Value::Tuple(arr) = v {
                    if elems.len() != arr.len() { return None; }
                    for (p, v) in elems.iter().zip(arr.iter()) {
                        let mut bs = self.match_pattern(p, v, _env)?;
                        out.append(&mut bs);
                    }
                    Some(out)
                } else { None }
            }
            Pattern::Array(elems, _) => {
                if let Value::Array(arr) = v {
                    if elems.len() != arr.len() { return None; }
                    for (p, v) in elems.iter().zip(arr.iter()) {
                        let mut bs = self.match_pattern(p, v, _env)?;
                        out.append(&mut bs);
                    }
                    Some(out)
                } else { None }
            }
            Pattern::Or(alts, _) => {
                for alt in alts {
                    if let Some(bs) = self.match_pattern(alt, v, _env) {
                        return Some(bs);
                    }
                }
                None
            }
            Pattern::Struct { fields, .. } => {
                if let Value::Struct(sv) = v {
                    for (n, p) in fields {
                        if let Some(fv) = sv.fields.get(&n.name) {
                            if let Some(mut bs) = self.match_pattern(p, fv, _env) {
                                out.append(&mut bs);
                            }
                        }
                    }
                    Some(out)
                } else { None }
            }
            Pattern::As(inner, name, _) => {
                let mut bs = self.match_pattern(inner, v, _env)?;
                bs.push((name.name.clone(), v.clone()));
                Some(bs)
            }
        }
    }

    fn eval_const_expr(&self, e: &Expr) -> Result<Value, ()> {
        match e {
            Expr::IntLit(s, _) => Ok(Value::Int(s.parse().unwrap_or(0))),
            Expr::FloatLit(s, _) => Ok(Value::Float(s.parse().unwrap_or(0.0))),
            Expr::StrLit(s, _) => Ok(Value::Str(Rc::new(s.clone()))),
            Expr::BoolLit(b, _) => Ok(Value::Bool(*b)),
            Expr::Null(_) => Ok(Value::Null),
            Expr::Unit(_) => Ok(Value::Unit),
            _ => Ok(Value::Unit),
        }
    }

    fn eval_expr(&mut self, e: &Expr, env: &Rc<Env>) -> Result<Value, ()> {
        match e {
            Expr::IntLit(s, _) => {
                let s2 = s.replace('_', "");
                if let Ok(n) = s2.parse::<i64>() { Ok(Value::Int(n)) }
                else if let Ok(n) = s2.parse::<u64>() { Ok(Value::UInt(n)) }
                else if let Ok(n) = s2.parse::<f64>() { Ok(Value::Float(n)) }
                else { Ok(Value::Int(0)) }
            }
            Expr::FloatLit(s, _) => Ok(Value::Float(s.parse().unwrap_or(0.0))),
            Expr::StrLit(s, _) => Ok(Value::Str(Rc::new(s.clone()))),
            Expr::RawStrLit(s, _) => Ok(Value::Str(Rc::new(s.clone()))),
            Expr::CharLit(s, _) => Ok(Value::Char(s.chars().next().unwrap_or('\0'))),
            Expr::BoolLit(b, _) => Ok(Value::Bool(*b)),
            Expr::Null(_) => Ok(Value::Null),
            Expr::Ident(id) => {
                if let Some(v) = env.lookup(&id.name) { Ok(v) }
                else if self.functions.contains_key(&id.name) {
                    Ok(Value::Str(Rc::new(id.name.clone())))
                } else {
                    self.diagnostics.push(Diagnostic::error(format!("undefined variable: {}", id.name), id.span));
                    Ok(Value::Unit)
                }
            }
            Expr::SelfExpr(_) => Ok(env.lookup("self").unwrap_or(Value::Unit)),
            Expr::InterpStr(parts, _) => {
                let mut out = String::new();
                for p in parts {
                    match p {
                        InterpStrPart::Text(t) => out.push_str(t),
                        InterpStrPart::Expr(e) => {
                            let v = self.eval_expr(e, env)?;
                            out.push_str(&format!("{}", ValueDisplay(&v)));
                        }
                    }
                }
                Ok(Value::Str(Rc::new(out)))
            }
            Expr::Array(elems, _) => {
                let mut out = Vec::with_capacity(elems.len());
                for e in elems { out.push(self.eval_expr(e, env)?); }
                Ok(Value::Array(Rc::new(out)))
            }
            Expr::Map(fields, _) => {
                let mut map = HashMap::new();
                for f in fields {
                    let key = static_key_of(&f.key);
                    let v = self.eval_expr(&f.value, env)?;
                    map.insert(key, v);
                }
                Ok(Value::Map(Rc::new(map)))
            }
            Expr::StructLit(ty, fields, _) => {
                let mut map = HashMap::new();
                for f in fields {
                    let key = static_key_of(&f.key);
                    let v = self.eval_expr(&f.value, env)?;
                    map.insert(key, v);
                }
                Ok(Value::Struct(StructVal { ty: Rc::new(ty.name.clone()), fields: Rc::new(map) }))
            }
            Expr::Tuple(elems, _) => {
                let mut out = Vec::with_capacity(elems.len());
                for e in elems { out.push(self.eval_expr(e, env)?); }
                Ok(Value::Tuple(Rc::new(out)))
            }
            Expr::Range(_, _, _, span) => {
                self.diagnostics.push(Diagnostic::warning(
                    "a range used outside a `for` loop evaluates to unit in dev mode",
                    *span,
                ));
                Ok(Value::Unit)
            }
            Expr::BinOp(l, op, r, _) => {
                let lv = self.eval_expr(l, env)?;
                let rv = self.eval_expr(r, env)?;
                Ok(self.bin_op(lv, *op, rv))
            }
            Expr::UnaryOp(op, e, _) => {
                let v = self.eval_expr(e, env)?;
                Ok(self.unary_op(*op, v))
            }
            Expr::Assign(target, op, val, _) => {
                let v = self.eval_expr(val, env)?;
                if let Expr::Ident(id) = target.as_ref() {
                    if let Some(existing) = env.lookup(&id.name) {
                        let new_v = self.apply_assign(existing, v.clone(), *op);
                        env.assign(&id.name, new_v);
                    }
                } else if let Some(root) = root_binding_name(target) {
                    if let Some(existing) = env.lookup(&root) {
                        let updated = self.update_target(existing, target, v.clone(), *op, Span::DUMMY);
                        env.assign(&root, updated);
                    }
                }
                Ok(v)
            }
            Expr::Call(callee, args, _) => self.eval_call(callee, args, env),
            Expr::MethodCall(recv, name, args, _, _) => self.eval_method_call(recv, name, args, env),
            Expr::Field(recv, name, _) => {
                let v = self.eval_expr(recv, env)?;
                if let Value::Struct(sv) = &v {
                    if let Some(f) = sv.fields.get(&name.name) { return Ok(f.clone()); }
                }
                if let Value::Map(m) = &v {
                    if let Some(f) = m.get(&name.name) { return Ok(f.clone()); }
                    return Ok(Value::Null);
                }
                Ok(Value::Unit)
            }
            Expr::Index(arr, idx, _) => {
                let av = self.eval_expr(arr, env)?;
                let iv = self.eval_expr(idx, env)?;
                match (av, iv) {
                    (Value::Array(arr), Value::Int(i)) => {
                        if i >= 0 && (i as usize) < arr.len() { Ok(arr[i as usize].clone()) }
                        else { Ok(Value::Unit) }
                    }
                    (Value::Tuple(arr), Value::Int(i)) => {
                        if i >= 0 && (i as usize) < arr.len() { Ok(arr[i as usize].clone()) }
                        else { Ok(Value::Unit) }
                    }
                    (Value::Str(s), Value::Int(i)) => {
                        if let Some(c) = s.chars().nth(i as usize) { Ok(Value::Char(c)) }
                        else { Ok(Value::Unit) }
                    }
                    (Value::Map(m), Value::Str(k)) => {
                        Ok(m.get(k.as_str()).cloned().unwrap_or(Value::Null))
                    }
                    _ => Ok(Value::Unit),
                }
            }
            Expr::Closure(params, _, body, _) => {
                Ok(Value::Closure { params: params.clone(), body: Rc::new((**body).clone()), env: env.as_ref().clone() })
            }
            Expr::IfExpr(cond, then_body, elseifs, else_body, _) => {
                let c = self.eval_expr(cond, env)?;
                if let Value::Bool(true) = c {
                    return self.exec_block(then_body, env).map(|s| match s { Signal::Normal(v) | Signal::Return(v) => v, _ => Value::Unit });
                }
                for (c, b) in elseifs {
                    let cv = self.eval_expr(c, env)?;
                    if let Value::Bool(true) = cv {
                        return self.exec_block(b, env).map(|s| match s { Signal::Normal(v) | Signal::Return(v) => v, _ => Value::Unit });
                    }
                }
                if let Some(eb) = else_body {
                    self.exec_block(eb, env).map(|s| match s { Signal::Normal(v) | Signal::Return(v) => v, _ => Value::Unit })
                } else { Ok(Value::Unit) }
            }
            Expr::MatchExpr(scrut, arms, _) => {
                let v = self.eval_expr(scrut, env)?;
                for arm in arms {
                    if let Some(bindings) = self.match_pattern(&arm.pattern, &v, env) {
                        let child = Rc::new(Env::child(env));
                        for (n, bv) in bindings {
                            child.insert(n, bv);
                        }
                        if let Some(g) = &arm.guard {
                            let gv = self.eval_expr(g, &child)?;
                            if !matches!(gv, Value::Bool(true)) { continue; }
                        }
                        return self.exec_block(&arm.body, &child).map(|s| match s { Signal::Normal(v) | Signal::Return(v) => v, _ => Value::Unit });
                    }
                }
                Ok(Value::Unit)
            }
            Expr::BlockExpr(b, _) => {
                self.exec_block(b, env).map(|s| match s { Signal::Normal(v) | Signal::Return(v) => v, _ => Value::Unit })
            }
            Expr::PipeForward(left, right, _) => {
                let lv = self.eval_expr(left, env)?;
                self.call_with_values(right, vec![lv], env)
            }
            Expr::BackPipe(_, _, _) => Ok(Value::Unit),
            Expr::Await(e, _) => self.eval_expr(e, env),
            Expr::Try(e, _) => self.eval_expr(e, env),
            Expr::Bang(e, _) => self.eval_expr(e, env),
            Expr::QuestionDot(_, _, _) => Ok(Value::Null),
            Expr::Cast(e, _, _) => self.eval_expr(e, env),
            Expr::As(e, _, _) => self.eval_expr(e, env),
            Expr::Spread(e, _) => self.eval_expr(e, env),
            Expr::Unit(_) => Ok(Value::Unit),
        }
    }

    fn eval_call(&mut self, callee: &Expr, args: &[CallArg], env: &Rc<Env>) -> Result<Value, ()> {
        let mut vals = Vec::with_capacity(args.len());
        for a in args {
            if let Some(e) = match a {
                CallArg::Positional(e) => Some(e),
                CallArg::Named(_, e) => Some(e),
                CallArg::Spread(e, _) => Some(e),
            } {
                vals.push(self.eval_expr(e, env)?);
            }
        }
        self.call_with_values(callee, vals, env)
    }

    /// Dispatch a call whose arguments are already evaluated values.
    fn call_with_values(&mut self, callee: &Expr, vals: Vec<Value>, env: &Rc<Env>) -> Result<Value, ()> {
        if let Expr::Ident(id) = callee {
            match id.name.as_str() {
                "print" | "println" => {
                    let mut out = String::new();
                    for (i, v) in vals.iter().enumerate() {
                        if i > 0 { out.push(' '); }
                        out.push_str(&format!("{}", ValueDisplay(v)));
                    }
                    out.push('\n');
                    self.output.push_str(&out);
                    return Ok(Value::Unit);
                }
                "len" => {
                    if let Some(v) = vals.first() {
                        return Ok(match v {
                            Value::Array(a) => Value::Int(a.len() as i64),
                            Value::Str(s) => Value::Int(s.chars().count() as i64),
                            Value::Map(m) => Value::Int(m.len() as i64),
                            _ => Value::Int(0),
                        });
                    }
                    return Ok(Value::Int(0));
                }
                _ => {}
            }
            if let Some(f) = self.functions.get(&id.name).cloned() {
                return self.exec_user_fn(&f, vals);
            }
            if let Some(s) = self.structs.get(&id.name).cloned() {
                let mut fields = HashMap::new();
                for (i, sf) in s.fields.iter().enumerate() {
                    if let Some(v) = vals.get(i) {
                        fields.insert(sf.name.name.clone(), v.clone());
                    }
                }
                return Ok(Value::Struct(StructVal {
                    ty: Rc::new(s.name.name.clone()),
                    fields: Rc::new(fields),
                }));
            }
        }
        let callee_v = self.eval_expr(callee, env)?;
        if let Value::Closure { params, body, env: captured_env } = callee_v {
            let captured = Rc::new(captured_env);
            let child = Rc::new(Env::child(&captured));
            for (i, p) in params.iter().enumerate() {
                if let Some(v) = vals.get(i) {
                    child.insert(p.name.name.clone(), v.clone());
                }
            }
            match self.exec_block(&body, &child)? {
                Signal::Return(v) => return Ok(v),
                Signal::Normal(v) => return Ok(v),
                _ => return Ok(Value::Unit),
            }
        }
        if let Expr::Closure(params, _, body, _) = callee {
            let child = Rc::new(Env::child(env));
            for (i, p) in params.iter().enumerate() {
                if let Some(v) = vals.get(i) {
                    child.insert(p.name.name.clone(), v.clone());
                }
            }
            match self.exec_block(body, &child)? {
                Signal::Return(v) => return Ok(v),
                Signal::Normal(v) => return Ok(v),
                _ => return Ok(Value::Unit),
            }
        }
        Ok(Value::Unit)
    }

    /// Execute a user function body with pre-bound argument values.
    fn exec_user_fn(&mut self, f: &FnItem, vals: Vec<Value>) -> Result<Value, ()> {
        let child = Rc::new(Env::child(&self.env));
        for (i, p) in f.params.iter().enumerate() {
            if let Some(v) = vals.get(i) {
                child.insert(p.name.name.clone(), v.clone());
            }
        }
        match self.exec_block(&f.body, &child)? {
            Signal::Return(v) => Ok(v),
            Signal::Normal(v) => Ok(v),
            _ => Ok(Value::Unit),
        }
    }

    fn eval_method_call(&mut self, recv: &Expr, name: &Ident, args: &[CallArg], env: &Rc<Env>) -> Result<Value, ()> {
        let rv = self.eval_expr(recv, env)?;
        let mut arg_vals = Vec::with_capacity(args.len());
        for a in args {
            if let CallArg::Positional(e) = a {
                arg_vals.push(self.eval_expr(e, env)?);
            }
        }
        match (&rv, name.name.as_str()) {
            (Value::Array(a), "push") => {
                let mut arr = (**a).clone();
                if let Some(v) = arg_vals.first() { arr.push(v.clone()); }
                let new_arr = Value::Array(Rc::new(arr));
                if let Some(root) = root_binding_name(recv) {
                    env.assign(&root, new_arr.clone());
                }
                Ok(new_arr)
            }
            (Value::Array(a), "len") => Ok(Value::Int(a.len() as i64)),
            (Value::Str(s), "len") => Ok(Value::Int(s.chars().count() as i64)),
            (Value::Str(s), "upper") => Ok(Value::Str(Rc::new(s.to_uppercase()))),
            (Value::Str(s), "lower") => Ok(Value::Str(Rc::new(s.to_lowercase()))),
            (Value::Str(s), "trim") => Ok(Value::Str(Rc::new(s.trim().to_string()))),
            (Value::Int(n), "abs") => Ok(Value::Int(n.abs())),
            (Value::Float(f), "abs") => Ok(Value::Float(f.abs())),
            _ => self.eval_user_method(rv, recv, name, arg_vals, env),
        }
    }

    /// Dispatch a user-defined method from the method table.
    fn eval_user_method(&mut self, recv: Value, recv_expr: &Expr, name: &Ident, args: Vec<Value>, env: &Rc<Env>) -> Result<Value, ()> {
        let ty_name = match &recv {
            Value::Struct(sv) => Some((*sv.ty).clone()),
            Value::Map(m) => self.structs.iter()
                .find(|(_, s)| s.fields.len() == m.len()
                    && s.fields.iter().all(|f| m.contains_key(&f.name.name)))
                .map(|(n, _)| n.clone()),
            _ => None,
        };
        let Some(ty) = ty_name else { return Ok(Value::Unit) };
        let key = format!("{}:{}", ty, name.name);
        let Some(method) = self.methods.get(&key).cloned() else {
            return Ok(Value::Unit);
        };
        let child = Rc::new(Env::child(&self.env));
        let has_self = method.params.first().is_some_and(|p| p.name.name == "self");
        let mut bindings: Vec<(String, Value)> = Vec::new();
        if has_self {
            bindings.push(("self".to_string(), recv));
        }
        for (i, p) in method.params.iter().enumerate().skip(if has_self { 1 } else { 0 }) {
            if let Some(v) = args.get(i - if has_self { 1 } else { 0 }) {
                bindings.push((p.name.name.clone(), v.clone()));
            }
        }
        for (n, v) in bindings {
            child.insert(n, v);
        }
        let writes_self = crate::ownership::method_writes_self(&method.body);
        let result = self.exec_block(&method.body, &child)?;
        if writes_self {
            if let Some(updated) = child.lookup("self") {
                if let Some(root) = root_binding_name(recv_expr) {
                    env.assign(&root, updated);
                }
            }
        }
        match result {
            Signal::Return(v) => Ok(v),
            Signal::Normal(v) => Ok(v),
            _ => Ok(Value::Unit),
        }
    }

    fn bin_op(&mut self, l: Value, op: BinOp, r: Value) -> Value {
        match op {
            BinOp::Add => arith(&l, &r, |a, b| a + b, |a, b| a + b, |a, b| a + b)
                .or_else(|| string_concat(&l, &r))
                .unwrap_or(Value::Unit),
            BinOp::Sub => arith(&l, &r, |a, b| a - b, |a, b| a - b, |a, b| a - b).unwrap_or(Value::Unit),
            BinOp::Mul => arith(&l, &r, |a, b| a * b, |a, b| a * b, |a, b| a * b).unwrap_or(Value::Unit),
            BinOp::Div => arith(&l, &r, |a, b| a / b, |a, b| a / b, |a, b| a / b).unwrap_or(Value::Unit),
            BinOp::Rem => arith(&l, &r, |a, b| a % b, |a, b| a % b, |a, b| a % b).unwrap_or(Value::Unit),
            BinOp::Eq => Value::Bool(value_eq(&l, &r)),
            BinOp::Ne => Value::Bool(!value_eq(&l, &r)),
            BinOp::Lt => Value::Bool(value_cmp(&l, &r).map_or(false, |o| o == std::cmp::Ordering::Less)),
            BinOp::Le => Value::Bool(value_cmp(&l, &r).map_or(false, |o| o != std::cmp::Ordering::Greater)),
            BinOp::Gt => Value::Bool(value_cmp(&l, &r).map_or(false, |o| o == std::cmp::Ordering::Greater)),
            BinOp::Ge => Value::Bool(value_cmp(&l, &r).map_or(false, |o| o != std::cmp::Ordering::Less)),
            BinOp::And | BinOp::LogicalAnd => match (l, r) {
                (Value::Bool(a), Value::Bool(b)) => Value::Bool(a && b),
                _ => Value::Bool(false),
            },
            BinOp::Or | BinOp::LogicalOr => match (l, r) {
                (Value::Bool(a), Value::Bool(b)) => Value::Bool(a || b),
                _ => Value::Bool(false),
            },
            BinOp::BitAnd => arith(&l, &r, |a, b| a & b, |a, b| a & b, |a, _b| a).unwrap_or(Value::Unit),
            BinOp::BitOr => arith(&l, &r, |a, b| a | b, |a, b| a | b, |a, _b| a).unwrap_or(Value::Unit),
            BinOp::BitXor => arith(&l, &r, |a, b| a ^ b, |a, b| a ^ b, |a, _b| a).unwrap_or(Value::Unit),
            BinOp::Shl => arith(&l, &r, |a, b| a.wrapping_shl(b as u32), |a, b| a.wrapping_shl(b as u32), |a, _b| a).unwrap_or(Value::Unit),
            BinOp::Shr => arith(&l, &r, |a, b| a.wrapping_shr(b as u32), |a, b| a.wrapping_shr(b as u32), |a, _b| a).unwrap_or(Value::Unit),
            BinOp::Concat => string_concat(&l, &r).unwrap_or(Value::Unit),
        }
    }

    fn unary_op(&self, op: UnaryOp, v: Value) -> Value {
        match op {
            UnaryOp::Neg => match v {
                Value::Int(n) => Value::Int(-n),
                Value::Float(f) => Value::Float(-f),
                _ => v,
            },
            UnaryOp::Not => match v {
                Value::Bool(b) => Value::Bool(!b),
                Value::Int(n) => Value::Int(!n),
                _ => v,
            },
            UnaryOp::BitNot => match v {
                Value::Int(n) => Value::Int(!n),
                _ => v,
            },
            _ => v,
        }
    }
}

impl Default for Interpreter {
    fn default() -> Self {
        Interpreter::new()
    }
}

fn arith(
    l: &Value,
    r: &Value,
    fi: fn(i64, i64) -> i64,
    fu: fn(u64, u64) -> u64,
    ff: fn(f64, f64) -> f64,
) -> Option<Value> {
    match (l, r) {
        (Value::Int(a), Value::Int(b)) => Some(Value::Int(fi(*a, *b))),
        (Value::UInt(a), Value::UInt(b)) => Some(Value::UInt(fu(*a, *b))),
        (Value::Float(a), Value::Float(b)) => Some(Value::Float(ff(*a, *b))),
        (Value::Int(a), Value::Float(b)) => Some(Value::Float(ff(*a as f64, *b))),
        (Value::Float(a), Value::Int(b)) => Some(Value::Float(ff(*a, *b as f64))),
        _ => None,
    }
}

fn string_concat(l: &Value, r: &Value) -> Option<Value> {
    let ls = match l {
        Value::Str(s) => s.to_string(),
        Value::Char(c) => c.to_string(),
        _ => return None,
    };
    let rs = match r {
        Value::Str(s) => s.to_string(),
        Value::Char(c) => c.to_string(),
        _ => return None,
    };
    Some(Value::Str(Rc::new(format!("{}{}", ls, rs))))
}

fn value_eq(a: &Value, b: &Value) -> bool {
    match (a, b) {
        (Value::Int(a), Value::Int(b)) => a == b,
        (Value::UInt(a), Value::UInt(b)) => a == b,
        (Value::Float(a), Value::Float(b)) => a == b,
        (Value::Bool(a), Value::Bool(b)) => a == b,
        (Value::Str(a), Value::Str(b)) => a == b,
        (Value::Char(a), Value::Char(b)) => a == b,
        (Value::Unit, Value::Unit) => true,
        (Value::Null, Value::Null) => true,
        _ => false,
    }
}

fn value_cmp(a: &Value, b: &Value) -> Option<std::cmp::Ordering> {
    match (a, b) {
        (Value::Int(a), Value::Int(b)) => Some(a.cmp(b)),
        (Value::UInt(a), Value::UInt(b)) => Some(a.cmp(b)),
        (Value::Float(a), Value::Float(b)) => Some(a.partial_cmp(b)?),
        (Value::Str(a), Value::Str(b)) => Some(a.cmp(b)),
        (Value::Char(a), Value::Char(b)) => Some(a.cmp(b)),
        _ => None,
    }
}

/// A helper for `format!`-style printing of values.
struct ValueDisplay<'a>(&'a Value);

impl<'a> fmt::Display for ValueDisplay<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0 {
            Value::Unit => write!(f, "()"),
            Value::Int(n) => write!(f, "{}", n),
            Value::UInt(n) => write!(f, "{}", n),
            Value::Float(n) => write!(f, "{}", n),
            Value::Bool(b) => write!(f, "{}", b),
            Value::Str(s) => write!(f, "{}", s),
            Value::Char(c) => write!(f, "{}", c),
            Value::Null => write!(f, "null"),
            Value::Array(arr) => {
                write!(f, "[")?;
                for (i, v) in arr.iter().enumerate() {
                    if i > 0 { write!(f, ", ")?; }
                    write!(f, "{}", ValueDisplay(v))?;
                }
                write!(f, "]")
            }
            Value::Tuple(arr) => {
                write!(f, "(")?;
                for (i, v) in arr.iter().enumerate() {
                    if i > 0 { write!(f, ", ")?; }
                    write!(f, "{}", ValueDisplay(v))?;
                }
                write!(f, ")")
            }
            Value::Map(m) => {
                write!(f, "{{")?;
                for (i, (k, v)) in m.iter().enumerate() {
                    if i > 0 { write!(f, ", ")?; }
                    write!(f, "{} = {}", k, ValueDisplay(v))?;
                }
                write!(f, "}}")
            }
            Value::Struct(sv) => {
                write!(f, "{} {{", sv.ty)?;
                for (i, (k, v)) in sv.fields.iter().enumerate() {
                    if i > 0 { write!(f, ", ")?; }
                    write!(f, "{} = {}", k, ValueDisplay(v))?;
                }
                write!(f, "}}")
            }
            Value::EnumVariant { name, fields } => {
                write!(f, "{}", name)?;
                if !fields.is_empty() {
                    write!(f, "(")?;
                    for (i, v) in fields.iter().enumerate() {
                        if i > 0 { write!(f, ", ")?; }
                        write!(f, "{}", ValueDisplay(v))?;
                    }
                    write!(f, ")")?;
                }
                Ok(())
            }
            Value::Closure { .. } => write!(f, "<closure>"),
        }
    }
}

/// Convenience function: interpret an Axolotl source string and return the output.
pub fn interpret(src: &str, file_id: u32) -> (String, Diagnostics) {
    let (module, parse_diags) = crate::parser::parse(src, file_id);
    let hir_module = crate::hir::lower(&module);
    let mut interp = Interpreter::new();
    let mut all = parse_diags;
    all.extend(hir_module.diags.clone());
    match interp.run_hir(&hir_module) {
        Ok(_) => (interp.output, all),
        Err(diags) => {
            all.extend(diags);
            (interp.output, all)
        }
    }
}
