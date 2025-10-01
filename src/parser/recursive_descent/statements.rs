use crate::lexer::token::TokenType;
use crate::parser::recursive_descent::{ExpressionParser, TypeParser};
use crate::parser::{ast::*, Expr};
use super::errors::ParseError;
use crate::parser::{Type};
use super::core::RecursiveDescentParser;

pub struct StatementParser;

impl StatementParser {
    pub fn parse_declaration(parser: &mut RecursiveDescentParser) -> Result<ASTNode, ParseError> {
        let var_decl = Self::parse_var_decl(parser)?;
        Ok(ASTNode::VarDecl(var_decl))
    }

    pub fn parse_expression(parser: &mut RecursiveDescentParser) -> Result<Expr, ParseError> {
        ExpressionParser::parse_expression(parser)
    }

    pub fn parse_var_decl(parser: &mut RecursiveDescentParser) -> Result<VarDecl, ParseError> {
        let start_token = parser.current_token()?.clone();

        // 1. Parse visibility
        let visibility = Self::parse_visibility(parser)?;

        // 2. Parse mutability
        let mut_token = parser.consume_one_of(
            &[TokenType::Let, TokenType::Mut],
            "Expected 'let' or 'mut'",
        )?;
        let mutability = match mut_token.token_type {
            TokenType::Let => Mutability::Let,
            TokenType::Mut => Mutability::Mut,
            _ => unreachable!(),
        };

        // 3. Parse name
        let name_tok = parser.consume_identifier("Expected variable name")?;
        let name = name_tok.lexeme.clone();

        // 4. Optional type
        let declared_type = if parser.match_tokens(&[TokenType::Colon]) {
            parser.advance()?;
            Some(TypeParser::parse_type(parser)?)
        } else {
            None
        };

        // 5. Optional initializer
        let initializer = if parser.match_tokens(&[TokenType::Equal]) {
            parser.advance()?;
            Some(ExpressionParser::parse_expression(parser)?)
        } else {
            None
        };

        // 6. Semicolon
        parser.consume(&TokenType::Semicolon, "Expected ';' after variable declaration")?;

        // 7. Semantic analysis
        let var_decl = parser.semantic_analyzer.analyze_var_declaration(
            visibility,
            mutability,
            name,
            declared_type,
            initializer,
            start_token.line,
            start_token.column,
        )?;

        // 8. Update symbol table
        // Fixed: Access the inferred_type field properly
        if let Some(ref inferred_type) = var_decl.inferred_type {
            if let Err(err) = parser.semantic_analyzer.get_symbol_table_mut().declare_variable(
                var_decl.name.clone(),
                inferred_type.clone(),
                var_decl.visibility.clone(),
                var_decl.mutability.clone(),
                var_decl.initializer.is_some(),
                start_token.line,
                start_token.column,
            ) {
                parser.add_semantic_error(err.to_string());
            }
        }


        Ok(var_decl)
    }

    fn parse_visibility(parser: &mut RecursiveDescentParser) -> Result<Option<Visibility>, ParseError> {
        if parser.match_tokens(&[TokenType::Public, TokenType::Private, TokenType::Protected]) {
            match parser.advance()?.token_type {
                TokenType::Public => Ok(Some(Visibility::Public)),
                TokenType::Protected => Ok(Some(Visibility::Protected)),
                TokenType::Private => Ok(Some(Visibility::Private)),
                _ => Ok(None),
            }
        } else {
            Ok(None)
        }
    }

    pub fn parse_function_decl(parser: &mut RecursiveDescentParser) -> Result<FnDecl, ParseError>{
        // 1. Visibility (optional)
        let visibility = if parser.match_tokens(&[
            TokenType::Public,
            TokenType::Private,
            TokenType::Protected,
        ]) {
            let vis_token = parser.advance()?;
            Some(match vis_token.token_type {
                TokenType::Public => Visibility::Public,
                TokenType::Private => Visibility::Private,
                TokenType::Protected => Visibility::Protected,
                _ => unreachable!(),
            })
        } else {
            None
        };

        // 2. 'fn' keyword
        parser.consume(&TokenType::Fn, "Expected 'fn' keyword")?;

        // 3. Function name
        let name_token = parser.consume_identifier("Expected function name")?;
        let name = if let TokenType::Identifier(n) = name_token.token_type {
            n
        } else {
            unreachable!()
        };

        // 4. Parameter list
        parser.consume(&TokenType::LeftParen, "Expected '(' before parameters")?;
        let mut parameters = Vec::new();
        while !parser.match_tokens(&[TokenType::RightParen]) {
            // parameter name
            let param_token = parser.consume_identifier("Expected parameter name")?;
            let param_name = if let TokenType::Identifier(n) = param_token.token_type {
                n
            } else { unreachable!() };

            // colon
            parser.consume(&TokenType::Colon, "Expected ':' after parameter name")?;

            // parameter type
            let type_token = parser.consume_identifier("Expected parameter type")?;
           let param_type = Type::from_type_name(&type_token.lexeme)
           .ok_or_else(|| ParseError::new("Invalid parameter type", type_token.line, type_token.column))?;


            parameters.push(Parameter {
                name: param_name,
                param_type,
                line: param_token.line,
                column: param_token.column,
            });

            // comma or end
            if parser.match_tokens(&[TokenType::Comma]) {
                let _ = parser.advance()?;
            } else {
                break;
            }
        }
        parser.consume(&TokenType::RightParen, "Expected ')' after parameters")?;

        // 5. Optional return type
        let return_type = if parser.match_tokens(&[TokenType::Arrow]) {
            let _ = parser.advance()?; // consume '->'
            let ret_token = parser.consume_identifier("Expected return type")?;
            Some(Type::from_type_name(&ret_token.lexeme)
                .ok_or_else(|| ParseError::new("Invalid return type", ret_token.line, ret_token.column))?)
        } else {
            None
        };

        // 6. Function body
        parser.consume(&TokenType::LeftBrace, "Expected '{' to start function body")?;
        let mut body_exprs = Vec::new();

        while !parser.match_tokens(&[TokenType::RightBrace]) && !parser.is_at_end() {
            // Ini sementara parse statement sebagai Expr
            let expr = super::statements::StatementParser::parse_expression(parser)?;
            body_exprs.push(expr);
        }

        parser.consume(&TokenType::RightBrace, "Expected '}' to close function body")?;

        let body = Expr::Block(body_exprs);

        Ok(FnDecl {
            visibility,
            name,
            parameters,
            return_type,
            body,
            line: name_token.line,
            column: name_token.column,
        })
    }
}