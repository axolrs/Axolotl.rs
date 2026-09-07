// Owner: PascalElixir / axolrs (GitHub org)
// File: AST node definitions for the Axolotl parser.

use crate::lexer::token::TokenKind;
use crate::span::Span;
use serde::{Deserialize, Serialize};

/// Identifier with span.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Ident {
    pub name: String,
    pub span: Span,
}

/// Top-level module item.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Item {
    Use(UseItem),
    Fn(FnItem),
    Struct(StructItem),
    Enum(EnumItem),
    Interface(InterfaceItem),
    Const(ConstItem),
    TypeAlias(TypeAliasItem),
    Impl(ImplItem),
    ExternBlock(ExternBlock),
    Method(MethodItem),
    StaticMethod(MethodItem),
    CBlock(ForeignBlock),
    CppBlock(ForeignBlock),
    RBlock(ForeignBlock),
    PyBlock(ForeignBlock),
    ModuleDoc(String, Span),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UseItem {
    pub path: String,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FnItem {
    pub name: Ident,
    pub generics: Vec<Ident>,
    pub params: Vec<Param>,
    pub ret: Option<TypeAnnot>,
    pub body: Block,
    pub is_pub: bool,
    pub is_async: bool,
    pub is_extern: bool,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Param {
    pub name: Ident,
    pub ty: Option<TypeAnnot>,
    pub default: Option<Expr>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StructItem {
    pub name: Ident,
    pub generics: Vec<Ident>,
    pub fields: Vec<StructField>,
    pub is_pub: bool,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StructField {
    pub name: Ident,
    pub ty: TypeAnnot,
    pub default: Option<Expr>,
    pub is_pub: bool,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EnumItem {
    pub name: Ident,
    pub generics: Vec<Ident>,
    pub variants: Vec<EnumVariant>,
    pub is_pub: bool,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EnumVariant {
    pub name: Ident,
    pub fields: Vec<EnumVariantField>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EnumVariantField {
    pub name: Option<Ident>,
    pub ty: TypeAnnot,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InterfaceItem {
    pub name: Ident,
    pub generics: Vec<Ident>,
    pub methods: Vec<MethodSig>,
    pub is_pub: bool,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MethodSig {
    pub name: Ident,
    pub params: Vec<Param>,
    pub ret: Option<TypeAnnot>,
    pub default_body: Option<Block>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConstItem {
    pub name: Ident,
    pub ty: Option<TypeAnnot>,
    pub value: Expr,
    pub is_pub: bool,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TypeAliasItem {
    pub name: Ident,
    pub generics: Vec<Ident>,
    pub ty: TypeAnnot,
    pub is_pub: bool,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImplItem {
    pub ty_name: Ident,
    pub interface_name: Option<Ident>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MethodItem {
    pub receiver: Ident,
    pub name: Ident,
    pub params: Vec<Param>,
    pub ret: Option<TypeAnnot>,
    pub body: Block,
    pub is_async: bool,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExternBlock {
    pub name: String,
    pub abi: String,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ForeignBlock {
    pub kind: ForeignKind,
    pub body: String,
    pub span: Span,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ForeignKind {
    C,
    Cpp,
    Rust,
    Python,
}

/// A type annotation in source.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TypeAnnot {
    pub kind: TypeAnnotKind,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TypeAnnotKind {
    Named(Ident, Vec<TypeAnnot>),
    Primitive(TokenKind),
    Optional(Box<TypeAnnot>),
    Array(Box<TypeAnnot>),
    Map(Box<TypeAnnot>, Box<TypeAnnot>),
    Set(Box<TypeAnnot>),
    Tuple(Vec<TypeAnnot>),
    Function(Vec<TypeAnnot>, Box<TypeAnnot>),
    Borrow(Box<TypeAnnot>, bool),
    Move(Box<TypeAnnot>),
    Infer,
    SelfType,
}

/// A statement.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Stmt {
    Let { name: Ident, ty: Option<TypeAnnot>, value: Expr, span: Span },
    Var { name: Ident, ty: Option<TypeAnnot>, value: Expr, span: Span },
    Assign { target: Expr, value: Expr, op: AssignOp, span: Span },
    Expr(Expr, Span),
    If { cond: Expr, then_body: Block, elseifs: Vec<(Expr, Block)>, else_body: Option<Block>, span: Span },
    While { cond: Expr, body: Block, span: Span },
    Repeat { body: Block, cond: Expr, span: Span },
    For { var: Ident, iter: Expr, body: Block, span: Span },
    Loop { body: Block, span: Span },
    Break(Span),
    Continue(Span),
    Return(Option<Expr>, Span),
    Match { scrutinee: Expr, arms: Vec<MatchArm>, span: Span },
    Block(Block, Span),
    Unsafe(Block, Span),
    Spawn { call: Expr, span: Span },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AssignOp {
    Assign,
    Add, Sub, Mul, Div, Rem,
    And, Or, Xor, Shl, Shr,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MatchArm {
    pub pattern: Pattern,
    pub guard: Option<Expr>,
    pub body: Block,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Pattern {
    Wild(Span),
    Lit(Expr, Span),
    Binding(Ident),
    EnumVariant { ty: Ident, variant: Ident, sub: Vec<Pattern>, span: Span },
    Struct { ty: Ident, fields: Vec<(Ident, Pattern)>, span: Span },
    Tuple(Vec<Pattern>, Span),
    Array(Vec<Pattern>, Span),
    Or(Vec<Pattern>, Span),
    As(Box<Pattern>, Ident, Span),
}

/// A block is a sequence of statements with an optional trailing expression.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Block {
    pub stmts: Vec<Stmt>,
    pub tail: Option<Box<Expr>>,
    pub span: Span,
}

/// An expression.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Expr {
    IntLit(String, Span),
    FloatLit(String, Span),
    StrLit(String, Span),
    CharLit(String, Span),
    RawStrLit(String, Span),
    BoolLit(bool, Span),
    Null(Span),
    Ident(Ident),
    SelfExpr(Span),
    InterpStr(Vec<InterpStrPart>, Span),
    Array(Vec<Expr>, Span),
    Map(Vec<MapField>, Span),
    Tuple(Vec<Expr>, Span),
    /// A struct literal construction: `Point { x = 1, y = 2 }`.
    StructLit(Ident, Vec<MapField>, Span),
    /// A range expression `a..b` / `a..=b`.
    Range(Option<Box<Expr>>, Option<Box<Expr>>, bool, Span),
    BinOp(Box<Expr>, BinOp, Box<Expr>, Span),
    UnaryOp(UnaryOp, Box<Expr>, Span),
    Assign(Box<Expr>, AssignOp, Box<Expr>, Span),
    Call(Box<Expr>, Vec<CallArg>, Span),
    MethodCall(Box<Expr>, Ident, Vec<CallArg>, bool, Span),
    Field(Box<Expr>, Ident, Span),
    Index(Box<Expr>, Box<Expr>, Span),
    Closure(Vec<Param>, Option<TypeAnnot>, Box<Block>, Span),
    IfExpr(Box<Expr>, Block, Vec<(Expr, Block)>, Option<Block>, Span),
    MatchExpr(Box<Expr>, Vec<MatchArm>, Span),
    BlockExpr(Block, Span),
    PipeForward(Box<Expr>, Box<Expr>, Span),
    BackPipe(Box<Expr>, Box<Expr>, Span),
    Await(Box<Expr>, Span),
    Try(Box<Expr>, Span),
    Bang(Box<Expr>, Span),
    QuestionDot(Box<Expr>, Ident, Span),
    Cast(Box<Expr>, TypeAnnot, Span),
    As(Box<Expr>, TypeAnnot, Span),
    Spread(Box<Expr>, Span),
    Unit(Span),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum InterpStrPart {
    Text(String),
    Expr(Expr),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MapField {
    pub key: Expr,
    pub value: Expr,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CallArg {
    Positional(Expr),
    Named(Ident, Expr),
    Spread(Expr, Span),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BinOp {
    Add, Sub, Mul, Div, Rem,
    Eq, Ne, Lt, Le, Gt, Ge,
    And, Or, LogicalAnd, LogicalOr,
    BitAnd, BitOr, BitXor, Shl, Shr,
    Concat,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum UnaryOp {
    Neg, Not, BitNot, Ref, MutRef, Deref,
}

/// A complete parsed module.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Module {
    pub items: Vec<Item>,
    pub span: Span,
}

/// Return the source span of an expression node.
impl Expr {
    pub fn span(&self) -> Span {
        *match self {
            Expr::IntLit(_, s) => s,
            Expr::FloatLit(_, s) => s,
            Expr::StrLit(_, s) => s,
            Expr::CharLit(_, s) => s,
            Expr::RawStrLit(_, s) => s,
            Expr::BoolLit(_, s) => s,
            Expr::Null(s) => s,
            Expr::Ident(i) => &i.span,
            Expr::SelfExpr(s) => s,
            Expr::InterpStr(_, s) => s,
            Expr::Array(_, s) => s,
            Expr::Map(_, s) => s,
            Expr::Tuple(_, s) => s,
            Expr::Range(_, _, _, s) => s,
            Expr::BinOp(_, _, _, s) => s,
            Expr::UnaryOp(_, _, s) => s,
            Expr::Assign(_, _, _, s) => s,
            Expr::Call(_, _, s) => s,
            Expr::MethodCall(_, _, _, _, s) => s,
            Expr::Field(_, _, s) => s,
            Expr::StructLit(_, _, s) => s,
            Expr::Index(_, _, s) => s,
            Expr::Closure(_, _, _, s) => s,
            Expr::IfExpr(_, _, _, _, s) => s,
            Expr::MatchExpr(_, _, s) => s,
            Expr::BlockExpr(_, s) => s,
            Expr::PipeForward(_, _, s) => s,
            Expr::BackPipe(_, _, s) => s,
            Expr::Await(_, s) => s,
            Expr::Try(_, s) => s,
            Expr::Bang(_, s) => s,
            Expr::QuestionDot(_, _, s) => s,
            Expr::Cast(_, _, s) => s,
            Expr::As(_, _, s) => s,
            Expr::Spread(_, s) => s,
            Expr::Unit(s) => s,
        }
    }
}
