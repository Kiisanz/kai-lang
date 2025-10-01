use crate::parser::ast::*;
use crate::lexer::token::{Token, TokenType};
use crate::parser::symbol_table::SymbolTable;
use crate::parser::semantic::analyzer::SemanticAnalyzer;
use super::statements::StatementParser;
use super::errors::{ParseError, ErrorRecovery};

pub struct RecursiveDescentParser<'a> {
    tokens: Vec<Token>,
    position: usize,
    semantic_errors: Vec<String>,
    pub semantic_analyzer: SemanticAnalyzer<'a>,
}

impl<'a> RecursiveDescentParser<'a> {
    pub fn new(tokens: Vec<Token>, symbol_table: &'a mut SymbolTable) -> Self {
        RecursiveDescentParser {
            tokens,
            position: 0,
            semantic_errors: Vec::new(),
            semantic_analyzer: SemanticAnalyzer::new(symbol_table),
        }
    }

    // Core token navigation
    pub fn current_token(&self) -> Result<&Token, ParseError> {
        self.tokens.get(self.position).ok_or_else(|| {
            let last = self.tokens.last();
            ParseError::new(
                "Unexpected end of input",
                last.map(|t| t.line).unwrap_or(1),
                last.map(|t| t.column).unwrap_or(1),
            )
        })
    }

    pub fn peek_token(&self) -> Option<&Token> {
        self.tokens.get(self.position + 1)
    }

    pub fn advance(&mut self) -> Result<&Token, ParseError> {
    if self.position >= self.tokens.len() {
        return Err(ParseError::new("Unexpected end of input", 0, 0));
    }
    self.position += 1;
    Ok(&self.tokens[self.position - 1])
}


    pub fn consume(&mut self, expected: &TokenType, message: &str) -> Result<Token, ParseError> {
        let token = self.current_token()?.clone();
        if self.matches_token(&token.token_type, expected) {
            self.advance()?;
            Ok(token)
        } else {
            Err(ParseError::new(
                &format!("{} (found {:?})", message, token.token_type),
                token.line,
                token.column,
            ))
        }
    }

    pub fn consume_one_of(
        &mut self,
        options: &[TokenType],
        message: &str,
    ) -> Result<Token, ParseError> {
        let token = self.current_token()?.clone();
        for opt in options {
            if self.matches_token(&token.token_type, opt) {
                self.advance()?;
                return Ok(token);
            }
        }
        Err(ParseError::new(
            &format!("{} (found {:?})", message, token.token_type),
            token.line,
            token.column,
        ))
    }

    pub fn consume_identifier(&mut self, message: &str) -> Result<Token, ParseError> {
        let token = self.current_token()?.clone();
        if matches!(token.token_type, TokenType::Identifier(_)) {
            self.advance()?;
            Ok(token)
        } else {
            Err(ParseError::new(message, token.line, token.column))
        }
    }

   pub fn matches_token(&self, actual: &TokenType, expected: &TokenType) -> bool {
    match (actual, expected) {
        (TokenType::Identifier(_), TokenType::Identifier(_)) => true,
        (TokenType::IntLiteral(_), TokenType::IntLiteral(_)) => true,
        (TokenType::FloatLiteral(_), TokenType::FloatLiteral(_)) => true,
        (TokenType::StringLiteral(_), TokenType::StringLiteral(_)) => true,
        (TokenType::BooleanLiteral(_), TokenType::BooleanLiteral(_)) => true,
        (TokenType::Eof, TokenType::Eof) => true,
        (a, b) => std::mem::discriminant(a) == std::mem::discriminant(b),
    }
}


    pub fn match_tokens(&self, token_types: &[TokenType]) -> bool {
        if let Ok(current) = self.current_token() {
            token_types
                .iter()
                .any(|t| self.matches_token(&current.token_type, t))
        } else {
            false
        }
    }

    pub fn is_at_end(&self) -> bool {
        self.position >= self.tokens.len()
            || matches!(self.current_token(), Ok(token) if matches!(token.token_type, TokenType::Eof))
    }

    // Main parsing entry point
    pub fn parse_program(&mut self) -> Result<ASTNode, ParseError> {
        let mut declarations = Vec::new();

        while !self.is_at_end() {
            if let Ok(token) = self.current_token() {
                if matches!(token.token_type, TokenType::Eof) {
                    break;
                }
            }
            match StatementParser::parse_declaration(self) {
                Ok(decl) => declarations.push(decl),
                Err(err) => {
                    self.semantic_errors.push(format!("Parse error: {}", err.message));
                    ErrorRecovery::synchronize(self);
                }
            }
        }

        Ok(ASTNode::Program(Program { declarations }))
    }

    // Utility
    pub fn get_semantic_errors(&self) -> &[String] {
        &self.semantic_errors
    }

    pub fn add_semantic_error(&mut self, error: String) {
        self.semantic_errors.push(error);
    }

    pub fn reset(&mut self) {
        self.position = 0;
        self.semantic_errors.clear();
        self.semantic_analyzer.reset();
    }

    pub fn current_position(&self) -> usize {
        self.position
    }

    pub fn remaining_tokens(&self) -> usize {
        self.tokens.len().saturating_sub(self.position)
    }

    pub fn push_tokens(&mut self, new_tokens: Vec<Token>) {
        self.tokens.extend(new_tokens);
    }

}
