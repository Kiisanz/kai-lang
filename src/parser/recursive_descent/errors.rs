use crate::{lexer::token::TokenType, parser::symbol_table::SymbolError};
use super::core::RecursiveDescentParser;



#[derive(Debug, Clone)]
pub struct ParseError {
    pub message: String,
    pub line: usize,
    pub column: usize,
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Parse error at line {}, column {}: {}", 
               self.line, self.column, self.message)
    }
}

impl std::error::Error for ParseError {}

impl ParseError {
    pub fn new<M: Into<String>>(message: M, line: usize, column: usize) -> Self {
        Self {
            message: message.into(),
            line,
            column,
        }
    }
}


impl From<SymbolError> for ParseError {
    fn from(err: SymbolError) -> Self {
        ParseError {
            message: err.to_string(),
            line: 0,   
            column: 0, 
        }
    }
}

pub struct ErrorRecovery;

impl ErrorRecovery {
    pub fn synchronize(parser: &mut RecursiveDescentParser) {
        while !parser.is_at_end() {
            if let Ok(tok) = parser.current_token() {
                if matches!(tok.token_type, TokenType::Semicolon) {
                    let _ = parser.advance();
                    break;
                }
                if matches!(
                    tok.token_type,
                    TokenType::Let | TokenType::Mut | TokenType::Fn |
                    TokenType::Struct | TokenType::Public | TokenType::Private |
                    TokenType::Protected
                ) {
                    break;
                }
                if matches!(tok.token_type, TokenType::LeftBrace | TokenType::RightBrace) {
                    break;
                }
            }
            let _ = parser.advance();
        }
    }

    pub fn recover_from_expression_error(parser: &mut RecursiveDescentParser) {
        while !parser.is_at_end() {
            if let Ok(tok) = parser.current_token() {
                if matches!(
                    tok.token_type,
                    TokenType::Semicolon | TokenType::RightParen |
                    TokenType::RightBrace | TokenType::Comma
                ) {
                    break;
                }
            }
            let _ = parser.advance();
        }
    }
}
