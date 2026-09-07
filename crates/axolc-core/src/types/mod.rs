// Owner: PascalElixir / axolrs (GitHub org)
// File: The Axolotl type system - type representation, inference, and checking.

use crate::ast::TypeAnnot;
use crate::ast::TypeAnnotKind;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fmt;

/// A semantic type after inference and checking.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Type {
    Int(IntWidth),
    UInt(IntWidth),
    Float(FloatWidth),
    Bool,
    String,
    StrRef,
    Char,
    Byte,
    Unit,
    Infer,
    Named(String, Vec<Type>),
    Optional(Box<Type>),
    Array(Box<Type>),
    Map(Box<Type>, Box<Type>),
    Set(Box<Type>),
    Tuple(Vec<Type>),
    Function(Vec<Type>, Box<Type>),
    Borrow(Box<Type>, bool),
    Move(Box<Type>),
    SelfType,
    Never,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum IntWidth { W8, W16, W32, W64, W128, Isize }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FloatWidth { W32, W64 }

impl Type {
    /// Return the Rust type name for this Axolotl type.
    pub fn to_rust(&self) -> String {
        match self {
            Type::Int(w) => match w {
                IntWidth::W8 => "i8".into(),
                IntWidth::W16 => "i16".into(),
                IntWidth::W32 => "i32".into(),
                IntWidth::W64 => "i64".into(),
                IntWidth::W128 => "i128".into(),
                IntWidth::Isize => "isize".into(),
            },
            Type::UInt(w) => match w {
                IntWidth::W8 => "u8".into(),
                IntWidth::W16 => "u16".into(),
                IntWidth::W32 => "u32".into(),
                IntWidth::W64 => "u64".into(),
                IntWidth::W128 => "u128".into(),
                IntWidth::Isize => "usize".into(),
            },
            Type::Float(w) => match w {
                FloatWidth::W32 => "f32".into(),
                FloatWidth::W64 => "f64".into(),
            },
            Type::Bool => "bool".into(),
            Type::String => "String".into(),
            Type::StrRef => "&'static str".into(),
            Type::Char => "char".into(),
            Type::Byte => "u8".into(),
            Type::Unit => "()".into(),
            Type::Infer => "_".into(),
            Type::Named(name, args) => {
                if args.is_empty() { name.clone() }
                else {
                    let inner = args.iter().map(|t| t.to_rust()).collect::<Vec<_>>().join(", ");
                    format!("{}<{}>", name, inner)
                }
            }
            Type::Optional(inner) => format!("Option<{}>", inner.to_rust()),
            Type::Array(inner) => format!("Vec<{}>", inner.to_rust()),
            Type::Map(k, v) => format!("HashMap<{}, {}>", k.to_rust(), v.to_rust()),
            Type::Set(inner) => format!("HashSet<{}>", inner.to_rust()),
            Type::Tuple(elems) => {
                if elems.is_empty() { "()".into() }
                else if elems.len() == 1 { format!("({},)", elems[0].to_rust()) }
                else {
                    let parts: Vec<_> = elems.iter().map(|t| t.to_rust()).collect();
                    format!("({})", parts.join(", "))
                }
            }
            Type::Function(params, ret) => {
                let p: Vec<_> = params.iter().map(|t| t.to_rust()).collect();
                let r = if matches!(**ret, Type::Infer) { "()".to_string() } else { ret.to_rust() };
                format!("Box<dyn FnMut({}) -> {}>", p.join(", "), r)
            }
            Type::Borrow(inner, is_mut) => {
                if *is_mut { format!("&mut {}", inner.to_rust()) }
                else { format!("&{}", inner.to_rust()) }
            }
            Type::Move(inner) => inner.to_rust(),
            Type::SelfType => "Self".into(),
            Type::Never => "!".into(),
        }
    }

    /// True when this type is an integer type.
    pub fn is_integer(&self) -> bool {
        matches!(self, Type::Int(_) | Type::UInt(_))
    }

    /// True when this type is a number (integer or float).
    pub fn is_number(&self) -> bool {
        matches!(self, Type::Int(_) | Type::UInt(_) | Type::Float(_))
    }

    /// Return the default Axolotl integer type (i64).
    pub fn default_int() -> Type { Type::Int(IntWidth::W64) }
    /// Return the default Axolotl float type (f64).
    pub fn default_float() -> Type { Type::Float(FloatWidth::W64) }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_rust())
    }
}

/// Convert a type annotation from the AST into a semantic type.
pub fn from_annot(annot: &TypeAnnot) -> Type {
    match &annot.kind {
        TypeAnnotKind::Primitive(tok) => match tok {
            crate::lexer::token::TokenKind::KwInt => Type::Int(IntWidth::W64),
            crate::lexer::token::TokenKind::KwUInt => Type::UInt(IntWidth::W64),
            crate::lexer::token::TokenKind::KwFloat | crate::lexer::token::TokenKind::KwDouble => Type::Float(FloatWidth::W64),
            crate::lexer::token::TokenKind::KwBool => Type::Bool,
            crate::lexer::token::TokenKind::KwString => Type::String,
            crate::lexer::token::TokenKind::KwStr => Type::StrRef,
            crate::lexer::token::TokenKind::KwChar => Type::Char,
            crate::lexer::token::TokenKind::KwByte => Type::Byte,
            crate::lexer::token::TokenKind::KwI8 => Type::Int(IntWidth::W8),
            crate::lexer::token::TokenKind::KwI16 => Type::Int(IntWidth::W16),
            crate::lexer::token::TokenKind::KwI32 => Type::Int(IntWidth::W32),
            crate::lexer::token::TokenKind::KwI64 => Type::Int(IntWidth::W64),
            crate::lexer::token::TokenKind::KwI128 => Type::Int(IntWidth::W128),
            crate::lexer::token::TokenKind::KwU8 => Type::UInt(IntWidth::W8),
            crate::lexer::token::TokenKind::KwU16 => Type::UInt(IntWidth::W16),
            crate::lexer::token::TokenKind::KwU32 => Type::UInt(IntWidth::W32),
            crate::lexer::token::TokenKind::KwU64 => Type::UInt(IntWidth::W64),
            crate::lexer::token::TokenKind::KwU128 => Type::UInt(IntWidth::W128),
            crate::lexer::token::TokenKind::KwF32 => Type::Float(FloatWidth::W32),
            crate::lexer::token::TokenKind::KwF64 => Type::Float(FloatWidth::W64),
            _ => Type::Infer,
        },
        TypeAnnotKind::Named(id, args) => {
            let args: Vec<Type> = args.iter().map(from_annot).collect();
            Type::Named(id.name.clone(), args)
        }
        TypeAnnotKind::Optional(inner) => Type::Optional(Box::new(from_annot(inner))),
        TypeAnnotKind::Array(inner) => Type::Array(Box::new(from_annot(inner))),
        TypeAnnotKind::Map(k, v) => Type::Map(Box::new(from_annot(k)), Box::new(from_annot(v))),
        TypeAnnotKind::Set(inner) => Type::Set(Box::new(from_annot(inner))),
        TypeAnnotKind::Tuple(elems) => Type::Tuple(elems.iter().map(from_annot).collect()),
        TypeAnnotKind::Function(params, ret) => {
            let p: Vec<Type> = params.iter().map(from_annot).collect();
            Type::Function(p, Box::new(from_annot(ret)))
        }
        TypeAnnotKind::Borrow(inner, is_mut) => Type::Borrow(Box::new(from_annot(inner)), *is_mut),
        TypeAnnotKind::Move(inner) => Type::Move(Box::new(from_annot(inner))),
        TypeAnnotKind::Infer => Type::Infer,
        TypeAnnotKind::SelfType => Type::SelfType,
    }
}

/// A symbol table mapping names to types for type inference.
#[derive(Debug, Clone, Default)]
pub struct SymbolTable {
    pub bindings: BTreeMap<String, Type>,
    pub parent: Option<Box<SymbolTable>>,
}

impl SymbolTable {
    /// Construct an empty symbol table.
    pub fn new() -> SymbolTable { SymbolTable::default() }

    /// Construct a child scope with the current scope as parent.
    pub fn child(parent: &SymbolTable) -> SymbolTable {
        SymbolTable { bindings: BTreeMap::new(), parent: Some(Box::new(parent.clone())) }
    }

    /// Insert a binding into the current scope.
    pub fn insert(&mut self, name: impl Into<String>, ty: Type) {
        self.bindings.insert(name.into(), ty);
    }

    /// Lookup a binding, walking up the scope chain.
    pub fn lookup(&self, name: &str) -> Option<&Type> {
        if let Some(t) = self.bindings.get(name) { return Some(t); }
        if let Some(p) = &self.parent { return p.lookup(name); }
        None
    }
}
