#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum NonTerminal {
    Start,
    Program,   
    VarDecl,
    Visibility,
    Mutability,
    Type,
    Expr,
    Literal,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Symbol {
    Terminal(SymbolToken),
    NonTerminal(NonTerminal),
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SymbolToken {
    // Keywords
    Public, Private, Protected,
    Let, Mut, Const,
    
    // Operators
    Equal, Semicolon, Colon,
    
    // Literals
    IntLiteral,
    FloatLiteral,
    StringLiteral,
    BooleanLiteral,
    
    // Identifiers
    Identifier,
    TypeName(String),
    
    // Special
    Eof,
}

#[derive(Debug, Clone)]
pub struct Production {
    pub lhs: NonTerminal,
    pub rhs: Vec<Symbol>,
}