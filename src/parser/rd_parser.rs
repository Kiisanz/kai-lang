use crate::lexer::token::Token;
use crate::lexer::TokenType;
use crate::parser::{ASTNode, BinaryOp, Expr, Literal, Program, Type, UnaryOp, VarDecl};
use super::ast::*;
use super::symbol_table::{SymbolTable, SymbolError};

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

pub struct RecursiveDescentParser {
    tokens: Vec<Token>,
    position: usize,
    symbol_table: SymbolTable,
    semantic_errors: Vec<String>,
}

#[allow(dead_code)]
impl RecursiveDescentParser {
    pub fn new(tokens: Vec<Token>) -> Self {
        RecursiveDescentParser {
            tokens,
            position: 0,
            symbol_table: SymbolTable::new(),
            semantic_errors: Vec::new(),
        }
    }

    //core helpers
    fn current_token(&self) -> Result<&Token, ParseError> {
        self.tokens.get(self.position).ok_or_else(|| {
            let last = self.tokens.last();
            ParseError::new(
                "Unexpected end of input",
                last.map(|t| t.line).unwrap_or(1),
                last.map(|t| t.column).unwrap_or(1),
            )
        })
    }

    fn peek_token(&self) -> Option<&Token> {
        self.tokens.get(self.position + 1)
    }

    fn advance(&mut self) -> Result<Token, ParseError> {
        let token = self.current_token()?.clone();
        self.position += 1;
        Ok(token)
    }

    fn consume(&mut self, expected: &TokenType, message: &str) -> Result<Token, ParseError> {
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

    fn consume_one_of(
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

    fn consume_identifier(&mut self, message: &str) -> Result<Token, ParseError> {
        let token = self.current_token()?.clone();
        if matches!(token.token_type, TokenType::Identifier(_)) {
            self.advance()?;
            Ok(token)
        } else {
            Err(ParseError::new(message, token.line, token.column))
        }
    }

    fn matches_token(&self, actual: &TokenType, expected: &TokenType) -> bool {
        match (actual, expected) {
            (TokenType::Identifier(_), TokenType::Identifier(_)) => true,
            (TokenType::IntLiteral(_), TokenType::IntLiteral(_)) => true,
            (TokenType::StringLiteral(_), TokenType::StringLiteral(_)) => true,
            (TokenType::BooleanLiteral(_), TokenType::BooleanLiteral(_)) => true,
            (TokenType::FloatLiteral(_), TokenType::FloatLiteral(_)) => true,
            (a, b) => std::mem::discriminant(a) == std::mem::discriminant(b),
        }
    }

    fn match_tokens(&self, token_types: &[TokenType]) -> bool {
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

    fn synchronize(&mut self) {
        while !self.is_at_end() {
            if let Ok(tok) = self.current_token() {
                if matches!(tok.token_type, TokenType::Semicolon) {
                    let _ = self.advance();
                    break;
                }
                if matches!(
                    tok.token_type,
                    TokenType::Let | TokenType::Mut | TokenType::Fn | TokenType::Struct
                ) {
                    break;
                }
            }
            let _ = self.advance();
        }
    }

    // main entry point
    pub fn parse_program(&mut self) -> Result<ASTNode, ParseError> {
        let mut declarations = Vec::new();

        while !self.is_at_end() {
            if let Ok(token) = self.current_token() {
                if matches!(token.token_type, TokenType::Eof) {
                    break;
                }
            }

            match self.parse_var_decl() {
                Ok(decl) => declarations.push(ASTNode::VarDecl(decl)),
                Err(err) => {
                    self.semantic_errors
                        .push(format!("Parse error: {}", err.message));
                    self.synchronize();
                }
            }
        }

        Ok(ASTNode::Program(Program { declarations }))
    }

    // var decl parser
    pub fn parse_var_decl(&mut self) -> Result<VarDecl, ParseError> {
        let start_token = self.current_token()?.clone();

        // 1. visibility
        let visibility =
            if self.match_tokens(&[TokenType::Public, TokenType::Private, TokenType::Protected]) {
                match self.advance()?.token_type {
                    TokenType::Public => Some(Visibility::Public),
                    TokenType::Protected => Some(Visibility::Protected),
                    TokenType::Private => Some(Visibility::Private),
                    _ => None,
                }
            } else {
                None
            };

        // 2. mutability
        let mut_token =
            self.consume_one_of(&[TokenType::Let, TokenType::Mut], "Expected 'let' or 'mut'")?;
        let mutability = match mut_token.token_type {
            TokenType::Let => Mutability::Let,
            TokenType::Mut => Mutability::Mut,
            _ => unreachable!(),
        };

        // 3. name
        let name_tok = self.consume_identifier("Expected variable name")?;
        let name = name_tok.lexeme.clone();

        // 4. type (optional)
        let declared_type = if self.match_tokens(&[TokenType::Colon]) {
            self.advance()?;
            Some(self.parse_type()?)
        } else {
            None
        };

        // 5. initializer (optional)
        let initializer = if self.match_tokens(&[TokenType::Equal]) {
            self.advance()?;
            Some(self.parse_expression()?)
        } else {
            None
        };

        // 6. semicolon
        self.consume(
            &TokenType::Semicolon,
            "Expected ';' after variable declaration",
        )?;

        // 7. semantic = inference + type checking
        let inferred_type = match (&declared_type, &initializer) {
            (None, None) => {
                return Err(ParseError::new(
                    "Variable must have type annotation or initializer",
                    start_token.line,
                    start_token.column,
                ));
            }
            (Some(t), None) => Some(t.clone()),
            (None, Some(init)) => match self.infer_expression_type(init) {
                Ok(inferred) => Some(inferred),
                Err(err) => return Err(err),
            },
            (Some(t), Some(init)) => match self.infer_expression_type(init) {
                Ok(inferred) if inferred == *t => Some(t.clone()),
                Ok(inferred) => {
                    return Err(ParseError::new(
                        &format!("Type mismatch: declared {:?}, but got {:?}", t, inferred),
                        start_token.line,
                        start_token.column,
                    ));
                }
                Err(err) => return Err(err),
            },
        };

        // 8. symbol table insert
        if let Some(t) = inferred_type.clone() {
            if let Err(err) = self.symbol_table.declare_variable(
                name.clone(),
                t,
                visibility.clone(),
                mutability.clone(),
                initializer.is_some(),
                start_token.line,
                start_token.column,
            ) {
                self.semantic_errors.push(err.to_string());
            }
        }

        Ok(VarDecl {
            visibility,
            mutability,
            name,
            declared_type,
            inferred_type,
            initializer,
            line: start_token.line,
            column: start_token.column,
        })
    }

    // type parser
    fn parse_type(&mut self) -> Result<Type, ParseError> {
        let token = self.current_token()?;
        match &token.token_type {
            TokenType::Identifier(name) => {
                let type_name = name.clone();
                self.advance()?;
                if let Some(primitive) = Type::from_type_name(&type_name) {
                    if self.match_tokens(&[TokenType::Question]) {
                        self.advance()?;
                        Ok(Type::Optional(Box::new(primitive)))
                    } else {
                        Ok(primitive)
                    }
                } else {
                    Ok(Type::Custom(type_name))
                }
            }
            _ => Err(ParseError::new(
                &format!("Expected type, found {:?}", token.token_type),
                token.line,
                token.column,
            )),
        }
    }

    // expr parser
    fn parse_expression(&mut self) -> Result<Expr, ParseError> {
        self.parse_binary_expr(0)
    }

    fn parse_binary_expr(&mut self, min_prec: u8) -> Result<Expr, ParseError> {
        let mut left = self.parse_unary()?;

        while let Ok(token) = self.current_token() {
            if let Some((op, prec)) = Self::binary_precedence(&token.token_type) {
                if prec < min_prec {
                    break;
                }

                let _ = self.advance()?;
                let right = self.parse_binary_expr(prec + 1)?;
                left = Expr::Binary {
                    op,
                    left: Box::new(left),
                    right: Box::new(right),
                };
            } else {
                break;
            }
        }

        Ok(left)
    }

    fn parse_unary(&mut self) -> Result<Expr, ParseError> {
        if let Ok(token) = self.current_token() {
            if let Some(op) = UnaryOp::from_token(&token.token_type) {
                self.advance()?;
                let expr = self.parse_unary()?;
                return Ok(Expr::Unary {
                    op,
                    expr: Box::new(expr),
                });
            } else if matches!(token.token_type, TokenType::LeftParen) {
                self.advance()?;
                let expr = self.parse_expression()?;
                self.consume(&TokenType::RightParen, "Expected ')' after expression")?;
                return Ok(Expr::Grouping(Box::new(expr)));
            }
        }
        // if not unary, parse primary expression
        self.parse_primary()
    }

    fn parse_primary(&mut self) -> Result<Expr, ParseError> {
        let token = self.current_token()?;
        match &token.token_type {
            TokenType::IntLiteral(n) => {
                let val = *n;
                self.advance()?;
                Ok(Expr::Literal(Literal::Int(val)))
            }
            TokenType::FloatLiteral(f) => {
                let val = *f;
                self.advance()?;
                Ok(Expr::Literal(Literal::Float(val)))
            }
            TokenType::StringLiteral(s) => {
                let val = s.clone();
                self.advance()?;
                Ok(Expr::Literal(Literal::String(val)))
            }
            TokenType::BooleanLiteral(b) => {
                let val = *b;
                self.advance()?;
                Ok(Expr::Literal(Literal::Boolean(val)))
            }
            TokenType::Identifier(name) => {
                let id = name.clone();
                self.advance()?;
                Ok(Expr::Identifier(id))
            }
            _ => Err(ParseError::new(
                &format!("Expected expression, found {:?}", token.token_type),
                token.line,
                token.column,
            )),
        }
    }

    fn binary_precedence(token: &TokenType) -> Option<(BinaryOp, u8)> {
        match token {
            TokenType::Star | TokenType::Slash | TokenType::Percent => Some((
                match token {
                    TokenType::Star => BinaryOp::Mul,
                    TokenType::Slash => BinaryOp::Div,
                    TokenType::Percent => BinaryOp::Mod,
                    _ => unreachable!(),
                },
                20,
            )),
            TokenType::Plus | TokenType::Minus => Some((
                match token {
                    TokenType::Plus => BinaryOp::Add,
                    TokenType::Minus => BinaryOp::Sub,
                    _ => unreachable!(),
                },
                10,
            )),
            TokenType::EqualEqual
            | TokenType::NotEqual
            | TokenType::Less
            | TokenType::LessEqual
            | TokenType::Greater
            | TokenType::GreaterEqual => Some((
                match token {
                    TokenType::EqualEqual => BinaryOp::Equal,
                    TokenType::NotEqual => BinaryOp::NotEqual,
                    TokenType::Less => BinaryOp::Less,
                    TokenType::LessEqual => BinaryOp::LessEqual,
                    TokenType::Greater => BinaryOp::Greater,
                    TokenType::GreaterEqual => BinaryOp::GreaterEqual,
                    _ => unreachable!(),
                },
                5,
            )),
            TokenType::And => Some((BinaryOp::And, 3)),
            TokenType::Or => Some((BinaryOp::Or, 2)),
            _ => None,
        }
    }

    // type inference
    fn infer_expression_type(&self, expr: &Expr) -> Result<Type, ParseError> {
        match expr {
            Expr::Literal(lit) => Ok(Type::infer_from_literal(lit)),

            Expr::Identifier(name) => self
                .symbol_table
                .get_variable_type(name)
                .map_err(|err| ParseError::new(&err.to_string(), 0, 0)),

            Expr::Unary { expr, .. } => self.infer_expression_type(expr),

            Expr::Binary { left, right, .. } => {
                let l = self.infer_expression_type(left)?;
                let r = self.infer_expression_type(right)?;

                if l.is_compatible(&r) {
                    Ok(l)
                } else {
                    Err(ParseError::new("Type mismatch in binary expression", 0, 0))
                }
            }

            Expr::Assignment { value, .. } => self.infer_expression_type(value),

            Expr::Grouping(inner) => self.infer_expression_type(inner),
        }
    }

    // utils
    pub fn get_semantic_errors(&self) -> &[String] {
        &self.semantic_errors
    }

    pub fn reset(&mut self) {
        self.position = 0;
        self.symbol_table = SymbolTable::new();
        self.semantic_errors.clear();
    }

    pub fn parse_single_var_decl(&mut self) -> Result<VarDecl, ParseError> {
        self.parse_var_decl()
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

    pub fn get_symbol_table(&self) -> &SymbolTable {
        &self.symbol_table
    }

    pub fn set_symbol_table(&mut self, table: SymbolTable) {
        self.symbol_table = table;
    }
}
