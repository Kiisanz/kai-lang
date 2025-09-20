#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_tokens() {
        let mut lexer = Lexer::new("+ - * / % = == != < > <= >= && || ! ? :");
        let tokens = lexer.tokenize().unwrap();
        
        assert_eq!(tokens[0].token_type, TokenType::Plus);
        assert_eq!(tokens[1].token_type, TokenType::Minus);
        assert_eq!(tokens[2].token_type, TokenType::Star);
        assert_eq!(tokens[3].token_type, TokenType::Slash);
        assert_eq!(tokens[4].token_type, TokenType::Percent);
        assert_eq!(tokens[5].token_type, TokenType::Equal);
        assert_eq!(tokens[6].token_type, TokenType::EqualEqual);
        assert_eq!(tokens[7].token_type, TokenType::NotEqual);
        assert_eq!(tokens[8].token_type, TokenType::Less);
        assert_eq!(tokens[9].token_type, TokenType::Greater);
        assert_eq!(tokens[10].token_type, TokenType::LessEqual);
        assert_eq!(tokens[11].token_type, TokenType::GreaterEqual);
        assert_eq!(tokens[12].token_type, TokenType::And);
        assert_eq!(tokens[13].token_type, TokenType::Or);
        assert_eq!(tokens[14].token_type, TokenType::Not);
        assert_eq!(tokens[15].token_type, TokenType::Question);
        assert_eq!(tokens[16].token_type, TokenType::Colon);
    }

    #[test]
    fn test_keywords() {
        let mut lexer = Lexer::new("let mut const fn if else while for return");
        let tokens = lexer.tokenize().unwrap();
        
        assert_eq!(tokens[0].token_type, TokenType::Let);
        assert_eq!(tokens[1].token_type, TokenType::Mut);
        assert_eq!(tokens[2].token_type, TokenType::Const);
        assert_eq!(tokens[3].token_type, TokenType::Fn);
        assert_eq!(tokens[4].token_type, TokenType::If);
        assert_eq!(tokens[5].token_type, TokenType::Else);
        assert_eq!(tokens[6].token_type, TokenType::While);
        assert_eq!(tokens[7].token_type, TokenType::For);
        assert_eq!(tokens[8].token_type, TokenType::Return);
    }

    #[test]
    fn test_visibility_keywords() {
        let mut lexer = Lexer::new("public private protected");
        let tokens = lexer.tokenize().unwrap();
        
        assert_eq!(tokens[0].token_type, TokenType::Public);
        assert_eq!(tokens[1].token_type, TokenType::Private);
        assert_eq!(tokens[2].token_type, TokenType::Protected);
    }

    #[test]
    fn test_concurrency_keywords() {
        let mut lexer = Lexer::new("async sync par spawn await");
        let tokens = lexer.tokenize().unwrap();
        
        assert_eq!(tokens[0].token_type, TokenType::Async);
        assert_eq!(tokens[1].token_type, TokenType::Sync);
        assert_eq!(tokens[2].token_type, TokenType::Par);
        assert_eq!(tokens[3].token_type, TokenType::Spawn);
        assert_eq!(tokens[4].token_type, TokenType::Await);
    }

    #[test]
    fn test_numbers() {
        let mut lexer = Lexer::new("42 3.14 0 123.456");
        let tokens = lexer.tokenize().unwrap();
        
        assert_eq!(tokens[0].token_type, TokenType::IntLiteral(42));
        assert_eq!(tokens[1].token_type, TokenType::FloatLiteral(3.14));
        assert_eq!(tokens[2].token_type, TokenType::IntLiteral(0));
        assert_eq!(tokens[3].token_type, TokenType::FloatLiteral(123.456));
    }

    #[test]
    fn test_boolean_literals() {
        let mut lexer = Lexer::new("true false");
        let tokens = lexer.tokenize().unwrap();
        
        assert_eq!(tokens[0].token_type, TokenType::BooleanLiteral(true));
        assert_eq!(tokens[1].token_type, TokenType::BooleanLiteral(false));
    }

    #[test]
    fn test_strings() {
        let mut lexer = Lexer::new(r#""hello" "world\n" "test\"quote""#);
        let tokens = lexer.tokenize().unwrap();
        
        assert_eq!(tokens[0].token_type, TokenType::StringLiteral("hello".to_string()));
        assert_eq!(tokens[1].token_type, TokenType::StringLiteral("world\n".to_string()));
        assert_eq!(tokens[2].token_type, TokenType::StringLiteral("test\"quote".to_string()));
    }

    #[test]
    fn test_string_interpolation() {
        let mut lexer = Lexer::new(r#""Hello ${name}, you have ${count} messages""#);
        let tokens = lexer.tokenize().unwrap();
        
        if let TokenType::StringLiteral(content) = &tokens[0].token_type {
            assert!(content.contains("Hello ${name}"));
            assert!(content.contains("${count} messages"));
        } else {
            panic!("Expected StringLiteral token");
        }
    }

    #[test]
    fn test_unicode_escape() {
        let mut lexer = Lexer::new(r#""\u0041\u0042\u0043""#);
        let tokens = lexer.tokenize().unwrap();
        
        assert_eq!(tokens[0].token_type, TokenType::StringLiteral("ABC".to_string()));
    }

    #[test]
    fn test_identifiers() {
        let mut lexer = Lexer::new("variable_name _private CamelCase test123");
        let tokens = lexer.tokenize().unwrap();
        
        assert_eq!(tokens[0].token_type, TokenType::Identifier("variable_name".to_string()));
        assert_eq!(tokens[1].token_type, TokenType::Identifier("_private".to_string()));
        assert_eq!(tokens[2].token_type, TokenType::Identifier("CamelCase".to_string()));
        assert_eq!(tokens[3].token_type, TokenType::Identifier("test123".to_string()));
    }

    #[test]
    fn test_dsl_blocks() {
        let mut lexer = Lexer::new(r#"sql { SELECT * FROM users WHERE id = ${user_id} }"#);
        let tokens = lexer.tokenize().unwrap();
        
        match &tokens[0].token_type {
            TokenType::DSLContent { dsl_type, content } => {
                assert_eq!(dsl_type, "sql");
                assert!(content.contains("SELECT * FROM users"));
                assert!(content.contains("${user_id}"));
            }
            _ => panic!("Expected DSLContent token, got {:?}", tokens[0].token_type),
        }
    }

    #[test]
    fn test_nested_dsl_blocks() {
        let mut lexer = Lexer::new(r#"html { <div class="${className}">Hello {name}</div> }"#);
        let tokens = lexer.tokenize().unwrap();
        
        match &tokens[0].token_type {
            TokenType::DSLContent { dsl_type, content } => {
                assert_eq!(dsl_type, "html");
                assert!(content.contains("<div class="));
                assert!(content.contains("{name}"));
            }
            _ => panic!("Expected DSLContent token"),
        }
    }

    #[test]
    fn test_dsl_keyword_without_block() {
        let mut lexer = Lexer::new("sql");
        let tokens = lexer.tokenize().unwrap();
        
        assert_eq!(tokens[0].token_type, TokenType::DSL("sql".to_string()));
    }

    #[test]
    fn test_comments() {
        let code = r#"
            // Single line comment
            let x = 42; // Another comment
            /* Multi-line
               comment */
            let y = 10;
        "#;
        
        let mut lexer = Lexer::new(code);
        let tokens = lexer.tokenize().unwrap();
        
        assert_eq!(tokens[0].token_type, TokenType::Let);
        assert_eq!(tokens[1].token_type, TokenType::Identifier("x".to_string()));
        assert_eq!(tokens[2].token_type, TokenType::Equal);
        assert_eq!(tokens[3].token_type, TokenType::IntLiteral(42));
        assert_eq!(tokens[4].token_type, TokenType::Semicolon);
        assert_eq!(tokens[5].token_type, TokenType::Let);
        assert_eq!(tokens[6].token_type, TokenType::Identifier("y".to_string()));
    }

    #[test]
    fn test_simple_program() {
        let code = r#"
            public fn main() -> int32 {
                let x: int32 = 42;
                return x;
            }
        "#;
        
        let mut lexer = Lexer::new(code);
        let tokens = lexer.tokenize().unwrap();
        
        assert!(tokens.len() > 10);
        assert_eq!(tokens[0].token_type, TokenType::Public);
        assert_eq!(tokens[1].token_type, TokenType::Fn);
        assert_eq!(tokens[2].token_type, TokenType::Identifier("main".to_string()));
    }

    #[test]
    fn test_whitespace_handling() {
        let code = r#"fn main() {
            let x = 42;
            
            return x;
        }"#;
        
        let mut lexer = Lexer::new(code);
        let tokens = lexer.tokenize().unwrap();
        
        assert_eq!(tokens[0].token_type, TokenType::Fn);
        assert_eq!(tokens[1].token_type, TokenType::Identifier("main".to_string()));
        assert_eq!(tokens[2].token_type, TokenType::LeftParen);
        assert_eq!(tokens[3].token_type, TokenType::RightParen);
    }

    #[test]
    fn test_arrow_operator() {
        let mut lexer = Lexer::new("fn test() -> int32");
        let tokens = lexer.tokenize().unwrap();
        
        assert_eq!(tokens[0].token_type, TokenType::Fn);
        assert_eq!(tokens[1].token_type, TokenType::Identifier("test".to_string()));
        assert_eq!(tokens[2].token_type, TokenType::LeftParen);
        assert_eq!(tokens[3].token_type, TokenType::RightParen);
        assert_eq!(tokens[4].token_type, TokenType::Arrow);
        assert_eq!(tokens[5].token_type, TokenType::Identifier("int32".to_string()));
    }

    #[test]
    fn test_unterminated_string_error() {
        let mut lexer = Lexer::new(r#""unterminated string"#);
        let result = lexer.tokenize();
        
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(error.message.contains("Unterminated string"));
        assert_eq!(error.line, 1);
        assert_eq!(error.column, 1);
    }

    #[test]
    fn test_invalid_character_error() {
        let mut lexer = Lexer::new("let x = @");
        let result = lexer.tokenize();
        
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(error.message.contains("Unexpected character '@'"));
    }

    #[test]
    fn test_single_ampersand_error() {
        let mut lexer = Lexer::new("x & y");
        let result = lexer.tokenize();
        
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(error.message.contains("Did you mean '&&'?"));
    }

    #[test]
    fn test_single_pipe_error() {
        let mut lexer = Lexer::new("x | y");
        let result = lexer.tokenize();
        
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(error.message.contains("Did you mean '||'?"));
    }

    #[test]
    fn test_unterminated_multiline_comment_error() {
        let mut lexer = Lexer::new("/* unterminated comment");
        let result = lexer.tokenize();
        
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(error.message.contains("Unterminated multi-line comment"));
    }

    #[test]
    fn test_unterminated_dsl_block_error() {
        let mut lexer = Lexer::new("sql { SELECT * FROM users");
        let result = lexer.tokenize();
        
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(error.message.contains("Unterminated sql block"));
    }

    #[test]
    fn test_invalid_unicode_escape_error() {
        let mut lexer = Lexer::new(r#""\uXYZ""#);
        let result = lexer.tokenize();
        
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(error.message.contains("Invalid unicode escape"));
    }

    #[test]
    fn test_error_line_column_tracking() {
        let code = r#"
            let x = 42;
            let y = "unterminated
        "#;
        
        let mut lexer = Lexer::new(code);
        let result = lexer.tokenize();
        
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert_eq!(error.line, 3); 
        assert!(error.message.contains("Unterminated string"));
    }

    #[test]
    fn test_complex_program() {
        let code = r#"
            use std.collections.HashMap;

            public struct User {
                private id: int64;
                public name: string;
                
                public async fn validate() -> bool {
                    sql {
                        SELECT COUNT(*) FROM users 
                        WHERE name = ${self.name}
                    }
                    return true;
                }
            }

            private const MAX_USERS: int32 = 1000;

            public async fn main() -> int32 {
                let users: [User] = [];
                for user in users {
                    spawn user.validate();
                }
                return 0;
            }
        "#;
        
        let mut lexer = Lexer::new(code);
        let tokens = lexer.tokenize().unwrap();
        
        assert!(tokens.len() > 50);
        
        assert_eq!(tokens[0].token_type, TokenType::Use);
        
        let struct_pos = tokens.iter().position(|t| matches!(t.token_type, TokenType::Struct)).unwrap();
        assert_eq!(tokens[struct_pos - 1].token_type, TokenType::Public);
        
        let dsl_found = tokens.iter().any(|t| matches!(t.token_type, TokenType::DSLContent { .. }));
        assert!(dsl_found, "Should find DSL content");
    }
}