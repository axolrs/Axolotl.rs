// Owner: PascalElixir / axolrs (GitHub org)
// File: Scope - real name resolution for Gills: a scope tree with shadowing, locals, params, fields, methods and variants built from the AST.

use std::collections::HashMap;

use axolc_core::ast::{
    self, CallArg, Expr, InterpStrPart, Item, Pattern, Stmt, TypeAnnot, TypeAnnotKind,
};
use axolc_core::span::Span;

/// The kind of a resolved declaration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DefKind {
    /// A `let`/`var` local, loop variable or pattern binding.
    Let,
    /// A function, method or closure parameter (including `self`).
    Param,
    /// A struct field.
    Field,
    /// A method attached to a receiver type or declared in an interface.
    Method,
    /// A free function.
    Fn,
    /// A struct type.
    Struct,
    /// An enum type.
    Enum,
    /// An interface type.
    Interface,
    /// A named constant.
    Const,
    /// An enum variant.
    Variant,
}

impl DefKind {
    /// Return true when the binding is declared at module level and is therefore shared across open documents.
    pub fn is_module_level(self) -> bool {
        matches!(
            self,
            DefKind::Fn
                | DefKind::Struct
                | DefKind::Enum
                | DefKind::Interface
                | DefKind::Const
                | DefKind::Method
        )
    }
}

/// The enclosing declaration a binding lives in, used for hover prose.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Container {
    /// Module level.
    Module,
    /// Inside the named free function.
    Function(String),
    /// Inside the method `Receiver.name`.
    Method { receiver: String, name: String },
    /// Inside a closure expression.
    Closure,
}

impl Container {
    /// Render the container name for hover prose (`main`, `Player.attack`, `a closure`).
    pub fn describe(&self) -> String {
        match self {
            Container::Module => "the module".to_string(),
            Container::Function(name) => name.clone(),
            Container::Method { receiver, name } => format!("{receiver}.{name}"),
            Container::Closure => "a closure".to_string(),
        }
    }
}

/// One declared binding: kind, declaration span and resolution metadata.
#[derive(Debug, Clone)]
pub struct Def {
    /// What kind of declaration this is.
    pub kind: DefKind,
    /// The declared name.
    pub name: String,
    /// Span of the declaring identifier token.
    pub span: Span,
    /// Owning struct, enum, interface or receiver name for fields, methods and variants.
    pub owner: Option<String>,
    /// The base type text for locals and params, or the return type for functions and methods.
    pub ty: Option<String>,
    /// True when declared with `var` rather than `let`.
    pub is_var: bool,
    /// Where the binding is declared, for hover prose.
    pub container: Container,
    /// True for the implicit `self` parameter of a method.
    pub self_param: bool,
    /// Offset from which the binding is visible to uses in its scope chain.
    pub visible_from: u32,
}

/// Index of a binding in the defs table.
pub type BindingId = usize;

/// One scope in the tree: an ordered list of bindings declared inside it.
#[derive(Debug, Clone)]
struct Scope {
    parent: Option<usize>,
    bindings: Vec<BindingId>,
}

/// How the receiver of a field access or method call was classified at walk time.
#[derive(Debug, Clone)]
enum Recv {
    /// The receiver was an identifier resolving to a module item (struct or enum).
    Item(BindingId),
    /// The receiver's type name was inferred.
    Ty(String),
    /// The receiver's type could not be determined.
    Unknown,
}

/// One recorded name position: either a plain use or a type-directed position.
#[derive(Debug, Clone)]
enum Site {
    /// A value-namespace identifier use inside a scope.
    Name { span: Span, scope: usize, name: String },
    /// A type-namespace identifier use resolved against module items only.
    Type { span: Span, name: String },
    /// The method name of a method call, with its classified receiver.
    Method { span: Span, name: String, recv: Recv },
    /// The field or variant name of a field access, with its classified receiver.
    Field { span: Span, name: String, recv: Recv },
    /// A field key of a struct literal of the named type.
    StructKey { span: Span, name: String, ty: String },
    /// A variant name in an `Enum.Variant` pattern of the named enum.
    VariantKey { span: Span, name: String, ty: String },
    /// A field key of a struct pattern of the named type.
    PatternKey { span: Span, name: String, ty: String },
    /// A `self` expression resolving to the enclosing method's self parameter.
    SelfUse { span: Span, binding: BindingId },
}

impl Site {
    /// Return the identifier span this site was recorded for.
    fn span(&self) -> Span {
        match self {
            Site::Name { span, .. }
            | Site::Type { span, .. }
            | Site::Method { span, .. }
            | Site::Field { span, .. }
            | Site::StructKey { span, .. }
            | Site::VariantKey { span, .. }
            | Site::PatternKey { span, .. }
            | Site::SelfUse { span, .. } => *span,
        }
    }

    /// Return the plain name when the site is a name or type use eligible for cross-document fallback.
    fn plain_name(&self) -> Option<&str> {
        match self {
            Site::Name { name, .. } | Site::Type { name, .. } => Some(name),
            _ => None,
        }
    }
}

/// The completed name-resolution index for one document.
#[derive(Debug, Clone)]
pub struct ScopeIndex {
    defs: Vec<Def>,
    scopes: Vec<Scope>,
    sites: Vec<Site>,
    resolved: Vec<Option<BindingId>>,
}

impl ScopeIndex {
    /// Build the scope index for one parsed module.
    pub fn build(module: &ast::Module, src: &str) -> ScopeIndex {
        let mut builder = Builder::new(src);
        builder.declare_items(module);
        builder.walk_items(module);
        builder.finish()
    }

    /// Return the declaration of a binding.
    pub fn def(&self, id: BindingId) -> &Def {
        &self.defs[id]
    }

    /// Resolve the binding for an identifier with exactly this span, preferring declarations over uses.
    pub fn resolve_span(&self, span: Span) -> Option<BindingId> {
        for (id, def) in self.defs.iter().enumerate() {
            if def.span == span {
                return Some(id);
            }
        }
        for (idx, site) in self.sites.iter().enumerate() {
            if site.span() == span {
                return self.resolved[idx];
            }
        }
        for (id, def) in self.defs.iter().enumerate() {
            if span_contains(def.span, span) {
                return Some(id);
            }
        }
        for (idx, site) in self.sites.iter().enumerate() {
            if span_contains(site.span(), span) {
                return self.resolved[idx];
            }
        }
        None
    }

    /// Return all resolved use spans of one binding, in source order.
    pub fn binding_uses(&self, id: BindingId) -> Vec<Span> {
        let mut spans: Vec<Span> = self
            .sites
            .iter()
            .zip(self.resolved.iter())
            .filter(|(_, r)| **r == Some(id))
            .map(|(s, _)| s.span())
            .collect();
        spans.sort_by_key(|s| s.start);
        spans
    }

    /// Return the name at the given span when it is an unresolved plain name or type use.
    pub fn unresolved_name_at(&self, span: Span) -> Option<String> {
        for (idx, site) in self.sites.iter().enumerate() {
            if site.span() == span && self.resolved[idx].is_none() {
                return site.plain_name().map(str::to_string);
            }
        }
        None
    }

    /// Return spans of unresolved plain uses of the given name, in source order.
    pub fn unresolved_name_uses(&self, name: &str) -> Vec<Span> {
        let mut spans: Vec<Span> = self
            .sites
            .iter()
            .zip(self.resolved.iter())
            .filter(|(s, r)| r.is_none() && s.plain_name() == Some(name))
            .map(|(s, _)| s.span())
            .collect();
        spans.sort_by_key(|s| s.start);
        spans
    }

    /// Return the module-level binding declared with the given name, if any.
    pub fn module_decl(&self, name: &str) -> Option<BindingId> {
        for id in &self.scopes[0].bindings {
            if self.defs[*id].name == name {
                return Some(*id);
            }
        }
        self.defs.iter().position(|d| d.kind == DefKind::Method && d.name == name)
    }

    /// Return the resolved use spans of the module-level binding declared with the given name.
    pub fn binding_uses_of(&self, name: &str) -> Vec<Span> {
        match self.module_decl(name) {
            Some(id) => self.binding_uses(id),
            None => Vec::new(),
        }
    }
}

/// Return true when outer covers inner.
fn span_contains(outer: Span, inner: Span) -> bool {
    outer.start <= inner.start && inner.end <= outer.end
}

/// The walking builder that constructs the scope tree while recording use sites and inferring types.
struct Builder<'a> {
    src: &'a str,
    defs: Vec<Def>,
    scopes: Vec<Scope>,
    sites: Vec<Site>,
    structs: HashMap<String, Vec<BindingId>>,
    enums: HashMap<String, Vec<BindingId>>,
    methods: HashMap<String, Vec<BindingId>>,
    types: Vec<HashMap<String, String>>,
    scope_stack: Vec<usize>,
    self_binding: Vec<Option<BindingId>>,
    container: Container,
}

impl<'a> Builder<'a> {
    /// Create an empty builder over one document body.
    fn new(src: &'a str) -> Builder<'a> {
        Builder {
            src,
            defs: Vec::new(),
            scopes: vec![Scope { parent: None, bindings: Vec::new() }],
            sites: Vec::new(),
            structs: HashMap::new(),
            enums: HashMap::new(),
            methods: HashMap::new(),
            types: vec![HashMap::new()],
            scope_stack: vec![0],
            self_binding: Vec::new(),
            container: Container::Module,
        }
    }

    /// Declare all module-level items and their fields, variants and method signatures.
    fn declare_items(&mut self, module: &ast::Module) {
        for item in &module.items {
            match item {
                Item::Fn(f) => {
                    let def = Def {
                        kind: DefKind::Fn,
                        name: f.name.name.clone(),
                        span: f.name.span,
                        owner: None,
                        ty: f.ret.as_ref().map(|r| self.base_text(r)),
                        is_var: false,
                        container: Container::Function(f.name.name.clone()),
                        self_param: false,
                        visible_from: 0,
                    };
                    self.declare(def);
                }
                Item::Method(m) | Item::StaticMethod(m) => {
                    let def = Def {
                        kind: DefKind::Method,
                        name: m.name.name.clone(),
                        span: m.name.span,
                        owner: Some(m.receiver.name.clone()),
                        ty: m.ret.as_ref().map(|r| self.base_text(r)),
                        is_var: false,
                        container: Container::Method {
                            receiver: m.receiver.name.clone(),
                            name: m.name.name.clone(),
                        },
                        self_param: false,
                        visible_from: 0,
                    };
                    let id = self.declare_unscoped(def);
                    self.methods.entry(m.receiver.name.clone()).or_default().push(id);
                }
                Item::Struct(s) => {
                    let def = Def {
                        kind: DefKind::Struct,
                        name: s.name.name.clone(),
                        span: s.name.span,
                        owner: None,
                        ty: Some(s.name.name.clone()),
                        is_var: false,
                        container: Container::Module,
                        self_param: false,
                        visible_from: 0,
                    };
                    self.declare(def);
                    for field in &s.fields {
                        let def = Def {
                            kind: DefKind::Field,
                            name: field.name.name.clone(),
                            span: field.name.span,
                            owner: Some(s.name.name.clone()),
                            ty: Some(self.base_text(&field.ty)),
                            is_var: false,
                            container: Container::Module,
                            self_param: false,
                            visible_from: 0,
                        };
                        let id = self.declare_unscoped(def);
                        self.structs.entry(s.name.name.clone()).or_default().push(id);
                    }
                }
                Item::Enum(e) => {
                    let def = Def {
                        kind: DefKind::Enum,
                        name: e.name.name.clone(),
                        span: e.name.span,
                        owner: None,
                        ty: Some(e.name.name.clone()),
                        is_var: false,
                        container: Container::Module,
                        self_param: false,
                        visible_from: 0,
                    };
                    self.declare(def);
                    for variant in &e.variants {
                        let def = Def {
                            kind: DefKind::Variant,
                            name: variant.name.name.clone(),
                            span: variant.name.span,
                            owner: Some(e.name.name.clone()),
                            ty: Some(e.name.name.clone()),
                            is_var: false,
                            container: Container::Module,
                            self_param: false,
                            visible_from: 0,
                        };
                        let id = self.declare_unscoped(def);
                        self.enums.entry(e.name.name.clone()).or_default().push(id);
                    }
                }
                Item::Interface(i) => {
                    let def = Def {
                        kind: DefKind::Interface,
                        name: i.name.name.clone(),
                        span: i.name.span,
                        owner: None,
                        ty: Some(i.name.name.clone()),
                        is_var: false,
                        container: Container::Module,
                        self_param: false,
                        visible_from: 0,
                    };
                    self.declare(def);
                    for sig in &i.methods {
                        let def = Def {
                            kind: DefKind::Method,
                            name: sig.name.name.clone(),
                            span: sig.name.span,
                            owner: Some(i.name.name.clone()),
                            ty: sig.ret.as_ref().map(|r| self.base_text(r)),
                            is_var: false,
                            container: Container::Method {
                                receiver: i.name.name.clone(),
                                name: sig.name.name.clone(),
                            },
                            self_param: false,
                            visible_from: 0,
                        };
                        let id = self.declare_unscoped(def);
                        self.methods.entry(i.name.name.clone()).or_default().push(id);
                    }
                }
                Item::Const(c) => {
                    let def = Def {
                        kind: DefKind::Const,
                        name: c.name.name.clone(),
                        span: c.name.span,
                        owner: None,
                        ty: c.ty.as_ref().map(|t| self.base_text(t)),
                        is_var: false,
                        container: Container::Module,
                        self_param: false,
                        visible_from: 0,
                    };
                    self.declare(def);
                }
                _ => {}
            }
        }
    }

    /// Walk all item bodies recording use sites, bindings and types.
    fn walk_items(&mut self, module: &ast::Module) {
        for item in &module.items {
            match item {
                Item::Fn(f) => self.walk_fn(f),
                Item::Method(m) | Item::StaticMethod(m) => self.walk_method(m),
                Item::Struct(s) => {
                    for field in &s.fields {
                        self.walk_type(&field.ty);
                        if let Some(default) = &field.default {
                            self.walk_expr(default);
                        }
                    }
                }
                Item::Enum(e) => {
                    for variant in &e.variants {
                        for field in &variant.fields {
                            self.walk_type(&field.ty);
                        }
                    }
                }
                Item::Interface(i) => {
                    for sig in &i.methods {
                        self.walk_method_sig(&i.name.name, sig);
                    }
                }
                Item::Const(c) => {
                    if let Some(ty) = &c.ty {
                        self.walk_type(ty);
                    }
                    self.walk_expr(&c.value);
                }
                Item::Impl(imp) => {
                    self.use_type(&imp.ty_name);
                    if let Some(name) = &imp.interface_name {
                        self.use_type(name);
                    }
                }
                _ => {}
            }
        }
    }

    /// Walk one free function: parameter scope then body block scope.
    fn walk_fn(&mut self, f: &ast::FnItem) {
        let saved = self.set_container(Container::Function(f.name.name.clone()));
        self.push_scope();
        for param in &f.params {
            if let Some(ty) = &param.ty {
                self.walk_type(ty);
            }
            self.declare_param(&param.name.name, param.name.span, param.ty.as_ref(), f.span.start);
            if let Some(default) = &param.default {
                self.walk_expr(default);
            }
        }
        if let Some(ret) = &f.ret {
            self.walk_type(ret);
        }
        self.walk_block(&f.body);
        self.pop_scope();
        self.restore_container(saved);
    }

    /// Walk one method: receiver use, parameter scope with `self`, then body block scope.
    fn walk_method(&mut self, m: &ast::MethodItem) {
        self.use_name(&m.receiver);
        let saved = self.set_container(Container::Method {
            receiver: m.receiver.name.clone(),
            name: m.name.name.clone(),
        });
        self.push_scope();
        let mut self_id = None;
        for param in &m.params {
            if let Some(ty) = &param.ty {
                self.walk_type(ty);
            }
            let is_self = param.name.name == "self";
            let id = self.declare_param(
                &param.name.name,
                param.name.span,
                param.ty.as_ref(),
                m.span.start,
            );
            if is_self {
                self_id = Some(id);
                self.defs[id].ty = Some(m.receiver.name.clone());
                self.defs[id].self_param = true;
                if let Some(map) = self.types.last_mut() {
                    map.insert("self".to_string(), m.receiver.name.clone());
                }
            }
            if let Some(default) = &param.default {
                self.walk_expr(default);
            }
        }
        if let Some(ret) = &m.ret {
            self.walk_type(ret);
        }
        self.self_binding.push(self_id);
        self.walk_block(&m.body);
        self.self_binding.pop();
        self.pop_scope();
        self.restore_container(saved);
    }

    /// Walk one interface method signature and its optional default body.
    fn walk_method_sig(&mut self, interface: &str, sig: &ast::MethodSig) {
        let saved = self.set_container(Container::Method {
            receiver: interface.to_string(),
            name: sig.name.name.clone(),
        });
        self.push_scope();
        let mut self_id = None;
        for param in &sig.params {
            if let Some(ty) = &param.ty {
                self.walk_type(ty);
            }
            let is_self = param.name.name == "self";
            let id = self.declare_param(
                &param.name.name,
                param.name.span,
                param.ty.as_ref(),
                sig.span.start,
            );
            if is_self {
                self_id = Some(id);
                self.defs[id].ty = Some(interface.to_string());
                self.defs[id].self_param = true;
                if let Some(map) = self.types.last_mut() {
                    map.insert("self".to_string(), interface.to_string());
                }
            }
        }
        if let Some(ret) = &sig.ret {
            self.walk_type(ret);
        }
        if let Some(body) = &sig.default_body {
            self.self_binding.push(self_id);
            self.walk_block(body);
            self.self_binding.pop();
        }
        self.pop_scope();
        self.restore_container(saved);
    }

    /// Walk one closure expression: parameter scope then body block scope.
    fn walk_closure(
        &mut self,
        params: &[axolc_core::ast::Param],
        ret: &Option<TypeAnnot>,
        body: &axolc_core::ast::Block,
    ) {
        let saved = self.set_container(Container::Closure);
        self.push_scope();
        for param in params {
            if let Some(ty) = &param.ty {
                self.walk_type(ty);
            }
            self.declare_param(&param.name.name, param.name.span, param.ty.as_ref(), body.span.start);
            if let Some(default) = &param.default {
                self.walk_expr(default);
            }
        }
        if let Some(ret) = ret {
            self.walk_type(ret);
        }
        self.walk_block(body);
        self.pop_scope();
        self.restore_container(saved);
    }

    /// Walk one block in a fresh child scope.
    fn walk_block(&mut self, block: &axolc_core::ast::Block) {
        self.push_scope();
        for stmt in &block.stmts {
            self.walk_stmt(stmt);
        }
        if let Some(tail) = &block.tail {
            self.walk_expr(tail);
        }
        self.pop_scope();
    }

    /// Walk one statement, entering child scopes for blocks, branches and arms.
    fn walk_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Let { name, ty, value, span } | Stmt::Var { name, ty, value, span } => {
                if let Some(ty) = ty {
                    self.walk_type(ty);
                }
                self.walk_expr(value);
                let inferred = ty
                    .as_ref()
                    .map(|t| self.base_text(t))
                    .or_else(|| self.expr_ty(value));
                let def = Def {
                    kind: DefKind::Let,
                    name: name.name.clone(),
                    span: name.span,
                    owner: None,
                    ty: inferred,
                    is_var: matches!(stmt, Stmt::Var { .. }),
                    container: self.container.clone(),
                    self_param: false,
                    visible_from: span.end,
                };
                self.declare(def);
            }
            Stmt::Assign { target, value, .. } => {
                self.walk_expr(target);
                self.walk_expr(value);
            }
            Stmt::Expr(expr, _) => self.walk_expr(expr),
            Stmt::If { cond, then_body, elseifs, else_body, .. } => {
                self.walk_expr(cond);
                self.walk_block(then_body);
                for (c, b) in elseifs {
                    self.walk_expr(c);
                    self.walk_block(b);
                }
                if let Some(b) = else_body {
                    self.walk_block(b);
                }
            }
            Stmt::While { cond, body, .. } => {
                self.walk_expr(cond);
                self.walk_block(body);
            }
            Stmt::Repeat { body, cond, .. } => {
                self.walk_block(body);
                self.walk_expr(cond);
            }
            Stmt::For { var, iter, body, .. } => {
                self.walk_expr(iter);
                self.push_scope();
                let def = Def {
                    kind: DefKind::Let,
                    name: var.name.clone(),
                    span: var.span,
                    owner: None,
                    ty: None,
                    is_var: false,
                    container: self.container.clone(),
                    self_param: false,
                    visible_from: iter.span().end,
                };
                self.declare(def);
                self.walk_block(body);
                self.pop_scope();
            }
            Stmt::Loop { body, .. } => self.walk_block(body),
            Stmt::Break(_) | Stmt::Continue(_) => {}
            Stmt::Return(expr, _) => {
                if let Some(e) = expr {
                    self.walk_expr(e);
                }
            }
            Stmt::Match { scrutinee, arms, .. } => {
                self.walk_expr(scrutinee);
                for arm in arms {
                    self.push_scope();
                    self.walk_pattern(&arm.pattern, arm.span.start);
                    if let Some(guard) = &arm.guard {
                        self.walk_expr(guard);
                    }
                    self.walk_block(&arm.body);
                    self.pop_scope();
                }
            }
            Stmt::Block(block, _) => self.walk_block(block),
            Stmt::Unsafe(block, _) => self.walk_block(block),
            Stmt::Spawn { call, .. } => self.walk_expr(call),
        }
    }

    /// Walk one expression recording plain uses, type-directed sites and nested scopes.
    fn walk_expr(&mut self, expr: &Expr) {
        match expr {
            Expr::Ident(id) => self.use_name(id),
            Expr::SelfExpr(span) => {
                if let Some(Some(binding)) = self.self_binding.last() {
                    let binding = *binding;
                    self.sites.push(Site::SelfUse { span: *span, binding });
                }
            }
            Expr::InterpStr(parts, _) => {
                for part in parts {
                    if let InterpStrPart::Expr(e) = part {
                        self.walk_expr(e);
                    }
                }
            }
            Expr::Array(elems, _) | Expr::Tuple(elems, _) => {
                for e in elems {
                    self.walk_expr(e);
                }
            }
            Expr::Map(fields, _) => {
                for field in fields {
                    self.walk_expr(&field.key);
                    self.walk_expr(&field.value);
                }
            }
            Expr::StructLit(ty, fields, _) => {
                self.use_name(ty);
                for field in fields {
                    if let Expr::Ident(key) = &field.key {
                        self.sites.push(Site::StructKey {
                            span: key.span,
                            name: key.name.clone(),
                            ty: ty.name.clone(),
                        });
                    }
                    self.walk_expr(&field.value);
                }
            }
            Expr::Range(start, end, _, _) => {
                if let Some(e) = start {
                    self.walk_expr(e);
                }
                if let Some(e) = end {
                    self.walk_expr(e);
                }
            }
            Expr::BinOp(left, _, right, _) => {
                self.walk_expr(left);
                self.walk_expr(right);
            }
            Expr::UnaryOp(_, inner, _) => self.walk_expr(inner),
            Expr::Assign(target, _, value, _) => {
                self.walk_expr(target);
                self.walk_expr(value);
            }
            Expr::Call(callee, args, _) => {
                self.walk_expr(callee);
                for arg in args {
                    self.walk_call_arg(arg);
                }
            }
            Expr::MethodCall(recv, name, args, _, _) => {
                let info = self.recv_info(recv);
                self.walk_expr(recv);
                self.sites.push(Site::Method {
                    span: name.span,
                    name: name.name.clone(),
                    recv: info,
                });
                for arg in args {
                    self.walk_call_arg(arg);
                }
            }
            Expr::Field(recv, name, _) | Expr::QuestionDot(recv, name, _) => {
                let info = self.recv_info(recv);
                self.walk_expr(recv);
                self.sites.push(Site::Field {
                    span: name.span,
                    name: name.name.clone(),
                    recv: info,
                });
            }
            Expr::Index(recv, idx, _) => {
                self.walk_expr(recv);
                self.walk_expr(idx);
            }
            Expr::Closure(params, ret, body, _) => self.walk_closure(params, ret, body),
            Expr::IfExpr(cond, then_body, elseifs, else_body, _) => {
                self.walk_expr(cond);
                self.walk_block(then_body);
                for (c, b) in elseifs {
                    self.walk_expr(c);
                    self.walk_block(b);
                }
                if let Some(b) = else_body {
                    self.walk_block(b);
                }
            }
            Expr::MatchExpr(scrutinee, arms, _) => {
                self.walk_expr(scrutinee);
                for arm in arms {
                    self.push_scope();
                    self.walk_pattern(&arm.pattern, arm.span.start);
                    if let Some(guard) = &arm.guard {
                        self.walk_expr(guard);
                    }
                    self.walk_block(&arm.body);
                    self.pop_scope();
                }
            }
            Expr::BlockExpr(block, _) => self.walk_block(block),
            Expr::PipeForward(left, right, _) | Expr::BackPipe(left, right, _) => {
                self.walk_expr(left);
                self.walk_expr(right);
            }
            Expr::Await(inner, _)
            | Expr::Try(inner, _)
            | Expr::Bang(inner, _)
            | Expr::Spread(inner, _) => self.walk_expr(inner),
            Expr::Cast(inner, ty, _) | Expr::As(inner, ty, _) => {
                self.walk_expr(inner);
                self.walk_type(ty);
            }
            Expr::IntLit(..)
            | Expr::FloatLit(..)
            | Expr::StrLit(..)
            | Expr::CharLit(..)
            | Expr::RawStrLit(..)
            | Expr::BoolLit(..)
            | Expr::Null(_)
            | Expr::Unit(_) => {}
        }
    }

    /// Walk one call argument, skipping named-argument labels.
    fn walk_call_arg(&mut self, arg: &CallArg) {
        match arg {
            CallArg::Positional(e) => self.walk_expr(e),
            CallArg::Named(_, e) => self.walk_expr(e),
            CallArg::Spread(e, _) => self.walk_expr(e),
        }
    }

    /// Walk one pattern, declaring bindings and recording type-directed keys.
    fn walk_pattern(&mut self, pattern: &Pattern, visible_from: u32) {
        match pattern {
            Pattern::Wild(_) | Pattern::Lit(..) => {}
            Pattern::Binding(id) => {
                let def = Def {
                    kind: DefKind::Let,
                    name: id.name.clone(),
                    span: id.span,
                    owner: None,
                    ty: None,
                    is_var: false,
                    container: self.container.clone(),
                    self_param: false,
                    visible_from,
                };
                self.declare(def);
            }
            Pattern::EnumVariant { ty, variant, sub, .. } => {
                self.use_type(ty);
                self.sites.push(Site::VariantKey {
                    span: variant.span,
                    name: variant.name.clone(),
                    ty: ty.name.clone(),
                });
                for s in sub {
                    self.walk_pattern(s, visible_from);
                }
            }
            Pattern::Struct { ty, fields, .. } => {
                self.use_type(ty);
                for (fname, sub) in fields {
                    self.sites.push(Site::PatternKey {
                        span: fname.span,
                        name: fname.name.clone(),
                        ty: ty.name.clone(),
                    });
                    self.walk_pattern(sub, visible_from);
                }
            }
            Pattern::Tuple(elems, _) | Pattern::Array(elems, _) => {
                for e in elems {
                    self.walk_pattern(e, visible_from);
                }
            }
            Pattern::Or(alts, _) => {
                for alt in alts {
                    self.walk_pattern(alt, visible_from);
                }
            }
            Pattern::As(inner, id, _) => {
                self.walk_pattern(inner, visible_from);
                let def = Def {
                    kind: DefKind::Let,
                    name: id.name.clone(),
                    span: id.span,
                    owner: None,
                    ty: None,
                    is_var: false,
                    container: self.container.clone(),
                    self_param: false,
                    visible_from,
                };
                self.declare(def);
            }
        }
    }

    /// Walk one type annotation, recording named-type uses.
    fn walk_type(&mut self, ty: &TypeAnnot) {
        match &ty.kind {
            TypeAnnotKind::Named(id, args) => {
                self.use_type(id);
                for arg in args {
                    self.walk_type(arg);
                }
            }
            TypeAnnotKind::Optional(inner)
            | TypeAnnotKind::Array(inner)
            | TypeAnnotKind::Borrow(inner, _)
            | TypeAnnotKind::Move(inner) => self.walk_type(inner),
            TypeAnnotKind::Map(key, value) => {
                self.walk_type(key);
                self.walk_type(value);
            }
            TypeAnnotKind::Set(inner) => self.walk_type(inner),
            TypeAnnotKind::Tuple(parts) => {
                for part in parts {
                    self.walk_type(part);
                }
            }
            TypeAnnotKind::Function(params, ret) => {
                for param in params {
                    self.walk_type(param);
                }
                self.walk_type(ret);
            }
            TypeAnnotKind::Primitive(_) | TypeAnnotKind::SelfType | TypeAnnotKind::Infer => {}
        }
    }

    /// Record a plain value-namespace use of an identifier in the current scope.
    fn use_name(&mut self, id: &axolc_core::ast::Ident) {
        let scope = *self.scope_stack.last().expect("scope stack");
        self.sites.push(Site::Name {
            span: id.span,
            scope,
            name: id.name.clone(),
        });
    }

    /// Record a type-namespace use of an identifier.
    fn use_type(&mut self, id: &axolc_core::ast::Ident) {
        self.sites.push(Site::Type { span: id.span, name: id.name.clone() });
    }

    /// Push a fresh child scope.
    fn push_scope(&mut self) {
        let parent = *self.scope_stack.last().expect("scope stack");
        let id = self.scopes.len();
        self.scopes.push(Scope { parent: Some(parent), bindings: Vec::new() });
        self.scope_stack.push(id);
        self.types.push(HashMap::new());
    }

    /// Pop the innermost scope.
    fn pop_scope(&mut self) {
        self.scope_stack.pop();
        self.types.pop();
    }

    /// Declare a binding in the current scope and record its type for inference.
    fn declare(&mut self, def: Def) -> BindingId {
        let id = self.defs.len();
        let ty = def.ty.clone();
        let name = def.name.clone();
        self.defs.push(def);
        let scope = *self.scope_stack.last().expect("scope stack");
        self.scopes[scope].bindings.push(id);
        if let Some(ty) = ty {
            if let Some(map) = self.types.last_mut() {
                map.insert(name, ty);
            }
        }
        id
    }

    /// Declare a binding that is not visible to plain name lookup (fields, variants, methods).
    fn declare_unscoped(&mut self, def: Def) -> BindingId {
        let id = self.defs.len();
        self.defs.push(def);
        id
    }

    /// Declare one parameter binding in the current scope.
    fn declare_param(
        &mut self,
        name: &str,
        span: Span,
        ty: Option<&TypeAnnot>,
        visible_from: u32,
    ) -> BindingId {
        let def = Def {
            kind: DefKind::Param,
            name: name.to_string(),
            span,
            owner: None,
            ty: ty.map(|t| self.base_text(t)),
            is_var: false,
            container: self.container.clone(),
            self_param: false,
            visible_from,
        };
        self.declare(def)
    }

    /// Set the current container, returning the previous one.
    fn set_container(&mut self, container: Container) -> Container {
        let saved = self.container.clone();
        self.container = container;
        saved
    }

    /// Restore a previously saved container.
    fn restore_container(&mut self, saved: Container) {
        self.container = saved;
    }

    /// Slice the document at a span, clamped and boundary-checked.
    fn slice(&self, span: Span) -> String {
        let start = (span.start as usize).min(self.src.len());
        let end = (span.end as usize).min(self.src.len()).max(start);
        if self.src.is_char_boundary(start) && self.src.is_char_boundary(end) {
            self.src[start..end].to_string()
        } else {
            "_".to_string()
        }
    }

    /// Return the base type text of an annotation with borrow and move wrappers unwrapped.
    fn base_text(&self, ty: &TypeAnnot) -> String {
        match &ty.kind {
            TypeAnnotKind::Borrow(inner, _) | TypeAnnotKind::Move(inner) => {
                self.slice(inner.span)
            }
            _ => self.slice(ty.span),
        }
    }

    /// Resolve a name in the currently visible scope chain (read-only walk-time lookup).
    fn resolve_name_current(&self, name: &str, offset: u32) -> Option<BindingId> {
        for scope_id in self.scope_stack.iter().rev() {
            let found = self.scopes[*scope_id]
                .bindings
                .iter()
                .rev()
                .find(|id| {
                    let def = &self.defs[**id];
                    def.name == name && def.visible_from <= offset
                })
                .copied();
            if found.is_some() {
                return found;
            }
        }
        None
    }

    /// Classify the receiver of a field access or method call at walk time.
    fn recv_info(&self, expr: &Expr) -> Recv {
        match expr {
            Expr::Ident(id) => match self.resolve_name_current(&id.name, id.span.start) {
                Some(binding) => match self.defs[binding].kind {
                    DefKind::Struct | DefKind::Enum | DefKind::Interface => Recv::Item(binding),
                    _ => match &self.defs[binding].ty {
                        Some(ty) => Recv::Ty(ty.clone()),
                        None => Recv::Unknown,
                    },
                },
                None => Recv::Unknown,
            },
            Expr::SelfExpr(_) => match self.self_binding.last() {
                Some(Some(binding)) => match &self.defs[*binding].ty {
                    Some(ty) => Recv::Ty(ty.clone()),
                    None => Recv::Unknown,
                },
                _ => Recv::Unknown,
            },
            _ => match self.expr_ty(expr) {
                Some(ty) => Recv::Ty(ty),
                None => Recv::Unknown,
            },
        }
    }

    /// Infer the type name of an expression from annotations, initializers and signatures.
    fn expr_ty(&self, expr: &Expr) -> Option<String> {
        match expr {
            Expr::Ident(id) => {
                let binding = self.resolve_name_current(&id.name, id.span.start)?;
                let def = &self.defs[binding];
                match def.kind {
                    DefKind::Struct | DefKind::Enum | DefKind::Interface => Some(def.name.clone()),
                    _ => def.ty.clone(),
                }
            }
            Expr::SelfExpr(_) => match self.self_binding.last() {
                Some(Some(binding)) => self.defs[*binding].ty.clone(),
                _ => None,
            },
            Expr::StructLit(ty, _, _) => Some(ty.name.clone()),
            Expr::Field(recv, name, _) | Expr::QuestionDot(recv, name, _) => {
                self.field_ty(recv, name)
            }
            Expr::MethodCall(recv, name, _, _, _) => {
                let info = self.recv_info(recv);
                match info {
                    Recv::Item(binding) => {
                        let owner = self.defs[binding].name.clone();
                        self.method_ret(&owner, &name.name)
                    }
                    Recv::Ty(ty) => self.method_ret(&ty, &name.name),
                    Recv::Unknown => None,
                }
            }
            Expr::Call(callee, _, _) => {
                if let Expr::Ident(id) = &**callee {
                    let binding = self.resolve_name_current(&id.name, id.span.start)?;
                    let def = &self.defs[binding];
                    if def.kind == DefKind::Fn {
                        return def.ty.clone();
                    }
                }
                None
            }
            Expr::Cast(_, ty, _) | Expr::As(_, ty, _) => Some(self.base_text(ty)),
            Expr::BlockExpr(block, _) => self.block_ty(block),
            Expr::IfExpr(_, then_body, _, _, _) => self.block_ty(then_body),
            Expr::MatchExpr(_, arms, _) => arms.first().and_then(|arm| self.block_ty(&arm.body)),
            Expr::InterpStr(..) | Expr::StrLit(..) => Some("String".to_string()),
            Expr::IntLit(..) => Some("Int".to_string()),
            Expr::FloatLit(..) => Some("Float".to_string()),
            Expr::BoolLit(..) => Some("Bool".to_string()),
            Expr::CharLit(..) => Some("Char".to_string()),
            Expr::BinOp(left, _, right, _) => self.expr_ty(left).or_else(|| self.expr_ty(right)),
            Expr::UnaryOp(_, inner, _) => self.expr_ty(inner),
            Expr::Assign(target, _, _, _) => self.expr_ty(target),
            _ => None,
        }
    }

    /// Return the type of a named field on a classified receiver.
    fn field_ty(&self, recv: &Expr, name: &axolc_core::ast::Ident) -> Option<String> {
        let info = self.recv_info(recv);
        match info {
            Recv::Item(binding) => {
                let owner = self.defs[binding].name.clone();
                if self.defs[binding].kind == DefKind::Enum {
                    return Some(owner);
                }
                self.structs
                    .get(&owner)?
                    .iter()
                    .copied()
                    .find(|id| self.defs[*id].name == name.name)
                    .and_then(|id| self.defs[id].ty.clone())
            }
            Recv::Ty(ty) => {
                if self.enums.contains_key(&ty) {
                    return Some(ty);
                }
                self.structs
                    .get(&ty)?
                    .iter()
                    .copied()
                    .find(|id| self.defs[*id].name == name.name)
                    .and_then(|id| self.defs[id].ty.clone())
            }
            Recv::Unknown => None,
        }
    }

    /// Return the declared return type of a method on one owner type.
    fn method_ret(&self, owner: &str, name: &str) -> Option<String> {
        self.methods
            .get(owner)?
            .iter()
            .copied()
            .find(|id| self.defs[*id].name == name)
            .and_then(|id| self.defs[id].ty.clone())
    }

    /// Return the type of a block's value expression.
    fn block_ty(&self, block: &axolc_core::ast::Block) -> Option<String> {
        block.tail.as_ref().and_then(|tail| self.expr_ty(tail))
    }

    /// Finish the walk, resolve every recorded site and produce the index.
    fn finish(self) -> ScopeIndex {
        let mut resolved = Vec::with_capacity(self.sites.len());
        for site in &self.sites {
            let binding = self.resolve_site(site);
            resolved.push(binding);
        }
        ScopeIndex {
            defs: self.defs,
            scopes: self.scopes,
            sites: self.sites,
            resolved,
        }
    }

    /// Resolve one recorded site against the completed scope tree.
    fn resolve_site(&self, site: &Site) -> Option<BindingId> {
        match site {
            Site::Name { span, scope, name } => {
                let mut current = Some(*scope);
                while let Some(sid) = current {
                    let found = self.scopes[sid]
                        .bindings
                        .iter()
                        .rev()
                        .find(|id| {
                            let def = &self.defs[**id];
                            def.name == *name && def.visible_from <= span.start
                        })
                        .copied();
                    if found.is_some() {
                        return found;
                    }
                    current = self.scopes[sid].parent;
                }
                None
            }
            Site::Type { span, name } => self.scopes[0]
                .bindings
                .iter()
                .rev()
                .find(|id| {
                    let def = &self.defs[**id];
                    def.name == *name && def.visible_from <= span.start
                })
                .copied(),
            Site::Method { name, recv, .. } => self.resolve_method_site(name, recv),
            Site::Field { name, recv, .. } => self.resolve_field_site(name, recv),
            Site::StructKey { name, ty, .. } => {
                if self.structs.contains_key(ty) {
                    self.find_field(ty, name)
                } else {
                    self.unique_field(name)
                }
            }
            Site::VariantKey { name, ty, .. } => {
                if self.enums.contains_key(ty) {
                    self.find_variant(ty, name)
                } else {
                    self.unique_variant(name)
                }
            }
            Site::PatternKey { name, ty, .. } => {
                if self.structs.contains_key(ty) {
                    self.find_field(ty, name)
                } else {
                    self.unique_field(name)
                }
            }
            Site::SelfUse { binding, .. } => Some(*binding),
        }
    }

    /// Resolve a method call site through its receiver, with a unique-name fallback for unknown receivers.
    fn resolve_method_site(&self, name: &str, recv: &Recv) -> Option<BindingId> {
        match recv {
            Recv::Item(binding) => {
                let owner = self.defs[*binding].name.clone();
                self.find_method(&owner, name)
            }
            Recv::Ty(ty) => self.find_method(ty, name),
            Recv::Unknown => self.unique_method(name),
        }
    }

    /// Resolve a field access site through its receiver, with unique-name fallbacks for unknown receivers.
    fn resolve_field_site(&self, name: &str, recv: &Recv) -> Option<BindingId> {
        match recv {
            Recv::Item(binding) => match self.defs[*binding].kind {
                DefKind::Enum => self.find_variant(&self.defs[*binding].name, name),
                DefKind::Struct | DefKind::Interface => {
                    self.find_field(&self.defs[*binding].name, name)
                }
                _ => None,
            },
            Recv::Ty(ty) => self
                .find_field(ty, name)
                .or_else(|| self.find_variant(ty, name)),
            Recv::Unknown => self
                .unique_field(name)
                .or_else(|| self.unique_variant(name)),
        }
    }

    /// Find the field binding of one struct.
    fn find_field(&self, struct_name: &str, field: &str) -> Option<BindingId> {
        self.structs.get(struct_name)?.iter().copied().find(|id| {
            let def = &self.defs[*id];
            def.kind == DefKind::Field && def.name == field
        })
    }

    /// Find the variant binding of one enum.
    fn find_variant(&self, enum_name: &str, variant: &str) -> Option<BindingId> {
        self.enums.get(enum_name)?.iter().copied().find(|id| {
            let def = &self.defs[*id];
            def.kind == DefKind::Variant && def.name == variant
        })
    }

    /// Find a method binding by owner and name.
    fn find_method(&self, owner: &str, name: &str) -> Option<BindingId> {
        self.methods.get(owner)?.iter().copied().find(|id| {
            let def = &self.defs[*id];
            def.kind == DefKind::Method && def.name == name
        })
    }

    /// Return the field binding when exactly one struct declares a field with this name.
    fn unique_field(&self, name: &str) -> Option<BindingId> {
        let matches: Vec<BindingId> = self
            .structs
            .values()
            .flat_map(|ids| ids.iter().copied())
            .filter(|id| self.defs[*id].name == name)
            .collect();
        if matches.len() == 1 {
            Some(matches[0])
        } else {
            None
        }
    }

    /// Return the variant binding when exactly one enum declares a variant with this name.
    fn unique_variant(&self, name: &str) -> Option<BindingId> {
        let matches: Vec<BindingId> = self
            .enums
            .values()
            .flat_map(|ids| ids.iter().copied())
            .filter(|id| self.defs[*id].name == name)
            .collect();
        if matches.len() == 1 {
            Some(matches[0])
        } else {
            None
        }
    }

    /// Return the method binding when exactly one owner declares a method with this name.
    fn unique_method(&self, name: &str) -> Option<BindingId> {
        let matches: Vec<BindingId> = self
            .methods
            .values()
            .flat_map(|ids| ids.iter().copied())
            .filter(|id| self.defs[*id].name == name)
            .collect();
        if matches.len() == 1 {
            Some(matches[0])
        } else {
            None
        }
    }
}
