use crate::parser::Type;
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum SymbolError {
    AlreadyDeclared(String, usize, usize), // name, line, column
    Undeclared(String), // name
    ImmutableAssignment(String), // name 
    TypeMismatch(String, Type, Type), // name, expected, actual
}

impl std::fmt::Display for SymbolError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SymbolError::AlreadyDeclared(name, line, col) => {
                write!(f, "Variable '{}' already declared at line {}, column {}", name, line, col)
            }
            SymbolError::Undeclared(name) => {
                write!(f, "Undeclared variable '{}'", name)
            }
            SymbolError::ImmutableAssignment(name) => {
                write!(f, "Cannot assign to immutable variable '{}'", name)
            }
            SymbolError::TypeMismatch(name, expected, actual) => {
                write!(f, "Type mismatch for variable '{}': expected {:?}, got {:?}", 
                       name, expected, actual)
            }
        }
    }
}

impl std::error::Error for SymbolError {}