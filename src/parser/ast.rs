use crate::parser::Expr;

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub enum ASTNode {
    Program(Program),
    VarDecl(VarDecl),
    FnDecl(FnDecl),
    Visibility(Visibility),
    Mutability(Mutability),
    Type(super::types::Type),
    Expr(super::expr::Expr),
    Literal(super::expr::Literal),
    Identifier(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub declarations: Vec<ASTNode>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct VarDecl {
    pub visibility: Option<Visibility>,
    pub mutability: Mutability,
    pub name: String,
    pub declared_type: Option<super::types::Type>,
    pub inferred_type: Option<super::types::Type>,
    pub initializer: Option<super::expr::Expr>,
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FnDecl {
    pub visibility: Option<Visibility>,
    pub name: String,
    pub parameters: Vec<Parameter>,
    pub return_type: Option<super::types::Type>,
    pub body: Option<Expr>,
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Parameter {
    pub name: String,
    pub param_type: super::types::Type,
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Visibility {
    Public,
    Private,
    Protected,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Mutability {
    Let,
    Mut,
}


