use crate::lexer::token::{TokenType};
use crate::parser::{Type};
use crate::parser::recursive_descent::errors::ParseError;
use super::core::RecursiveDescentParser;

pub struct TypeParser;


impl TypeParser {
    pub fn parse_type(parser: &mut RecursiveDescentParser) -> Result<Type, ParseError> {
        let token = parser.current_token()?;
        match &token.token_type {
            TokenType::Identifier(name) => {
                let type_name = name.clone();
                parser.advance()?;
                
                let base_type = if let Some(primitive) = Type::from_type_name(&type_name) {
                    primitive
                } else {
                    Type::Custom(type_name)
                };

                // Handle optional types (Type?)
                if parser.match_tokens(&[TokenType::Question]) {
                    parser.advance()?;
                    Ok(Type::Optional(Box::new(base_type)))
                } else {
                    Ok(base_type)
                }
            }
            _ => Err(ParseError::new(
                &format!("Expected type, found {:?}", token.token_type),
                token.line,
                token.column,
            )),
        }
    }

    pub fn parse_function_type(parser: &mut RecursiveDescentParser) -> Result<Type, ParseError> {
        // For future implementation of function types
        // fn(param_types...) -> return_type
        todo!("Function type parsing not implemented yet")
    }
}
