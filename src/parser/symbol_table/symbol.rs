use std::collections::HashMap;
use crate::parser::{ast::{Mutability, Visibility}, Type};

#[derive(Debug, Clone, PartialEq)]
pub struct Symbol {
    pub name: String,
    pub symbol_type: Type,
    pub visibility: Option<Visibility>,
    pub mutability: Mutability,
    pub is_initialized: bool,
    pub declaration_line: usize,
    pub declaration_column: usize,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Scope {
    pub symbols: HashMap<String, Symbol>,
    pub parent: Option<Box<Scope>>,
    pub scope_type: ScopeType,
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum ScopeType {
    Global,
    Function,
    Block,
}
