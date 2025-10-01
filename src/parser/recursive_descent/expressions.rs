use super::core::RecursiveDescentParser;
use super::errors::ParseError;
use crate::lexer::token::TokenType;
use crate::parser::Expr;
use crate::parser::{BinaryOp, Literal, UnaryOp};

pub struct ExpressionParser;

impl ExpressionParser {
    pub fn parse_expression(parser: &mut RecursiveDescentParser) -> Result<Expr, ParseError> {
        Self::parse_assignment(parser)
    }

    fn parse_assignment(parser: &mut RecursiveDescentParser) -> Result<Expr, ParseError> {
        let expr = Self::parse_binary_expr(parser, 0)?;

        if parser.match_tokens(&[TokenType::Equal]) {
            let equals_token = parser.current_token()?.clone();
            parser.advance()?;
            if let Expr::Identifier(name) = expr {
                if let Err(err) = parser.semantic_analyzer.validate_assignment(
                    &name,
                    equals_token.line,
                    equals_token.column,
                ) {
                    return Err(err);
                }
                let value = Box::new(Self::parse_assignment(parser)?);
                return Ok(Expr::Assignment { name, value });
            } else {
                return Err(ParseError::new(
                    "Invalid assignment target",
                    equals_token.line,
                    equals_token.column,
                ));
            }
        }

        Ok(expr)
    }

    fn parse_binary_expr(
        parser: &mut RecursiveDescentParser,
        min_prec: u8,
    ) -> Result<Expr, ParseError> {
        let mut left = Self::parse_unary(parser)?;

        while let Ok(token) = parser.current_token() {
            if let Some((op, prec)) = Self::binary_precedence(&token.token_type) {
                if prec < min_prec {
                    break;
                }

                let _ = parser.advance()?;
                let right = Self::parse_binary_expr(parser, prec + 1)?;
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

    fn parse_unary(parser: &mut RecursiveDescentParser) -> Result<Expr, ParseError> {
        if let Ok(token) = parser.current_token() {
            if let Some(op) = UnaryOp::from_token(&token.token_type) {
                parser.advance()?;
                let expr = Self::parse_unary(parser)?;
                return Ok(Expr::Unary {
                    op,
                    expr: Box::new(expr),
                });
            } else if matches!(token.token_type, TokenType::LeftParen) {
                parser.advance()?;
                let expr = Self::parse_expression(parser)?;
                parser.consume(&TokenType::RightParen, "Expected ')' after expression")?;
                return Ok(Expr::Grouping(Box::new(expr)));
            }
        }
        Self::parse_primary(parser)
    }

    fn parse_primary(parser: &mut RecursiveDescentParser) -> Result<Expr, ParseError> {
        let token = parser.current_token()?;
        match &token.token_type {
            TokenType::IntLiteral(n) => {
                let val = *n;
                parser.advance()?;
                Ok(Expr::Literal(Literal::Int(val)))
            }
            TokenType::FloatLiteral(f) => {
                let val = *f;
                parser.advance()?;
                Ok(Expr::Literal(Literal::Float(val)))
            }
            TokenType::StringLiteral(s) => {
                let val = s.clone();
                parser.advance()?;
                Ok(Expr::Literal(Literal::String(val)))
            }
            TokenType::BooleanLiteral(b) => {
                let val = *b;
                parser.advance()?;
                Ok(Expr::Literal(Literal::Boolean(val)))
            }
            TokenType::Identifier(name) => {
                let id = name.clone();
                parser.advance()?;
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
}
