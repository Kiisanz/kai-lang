use crate::parser::expr::Literal;

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Int8, Int16, Int32, Int64,
    Uint8, Uint16, Uint32, Uint64,
    Float32, Float64,
    Bool, String, Byte, Rune,
    Custom(String),
    Optional(Box<Type>),
    Array(Box<Type>),
}

impl Type {
    pub fn from_type_name(name: &str) -> Option<Type> {
        match name {
            "int8" => Some(Type::Int8),
            "int16" => Some(Type::Int16),
            "int32" => Some(Type::Int32),
            "int64" => Some(Type::Int64),
            "uint8" => Some(Type::Uint8),
            "uint16" => Some(Type::Uint16),
            "uint32" => Some(Type::Uint32),
            "uint64" => Some(Type::Uint64),
            "float32" => Some(Type::Float32),
            "float64" => Some(Type::Float64),
            "bool" => Some(Type::Bool),
            "string" => Some(Type::String),
            "byte" => Some(Type::Byte),
            "rune" => Some(Type::Rune),
            _ => None,
        }
    }

    pub fn infer_from_literal(literal: &Literal) -> Type {
        match literal {
            Literal::Int(_) => Type::Int32,
            Literal::Float(_) => Type::Float64,
            Literal::String(_) => Type::String,
            Literal::Boolean(_) => Type::Bool,
        }
    }

    pub fn is_compatible(&self, other: &Type) -> bool {
        match (self, other) {
            (a, b) if a == b => true,
            (Type::Int8, Type::Int16 | Type::Int32 | Type::Int64) => true,
            (Type::Int16, Type::Int32 | Type::Int64) => true,
            (Type::Int32, Type::Int64) => true,
            (Type::Float32, Type::Float64) => true,
            (Type::Float32 | Type::Float64,
             Type::Int8 | Type::Int16 | Type::Int32 | Type::Int64) => true,
            _ => false,
        }
    }
}
