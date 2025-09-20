#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;

    fn parse_program_from_code(code: &str) -> Result<ASTNode, ParseError> {
        let code_chars: Vec<char> = code.chars().collect();
        let mut lexer = Lexer::new(&code_chars);
        let tokens = lexer.tokenize().map_err(|e| ParseError {
            message: e.message,
            line: e.line,
            column: e.column,
        })?;

        let mut parser = RecursiveDescentParser::new(tokens);
        parser.parse_program()
    }

    fn parse_single_var_decl_from_code(code: &str) -> Result<VarDecl, ParseError> {
        let code_chars: Vec<char> = code.chars().collect();
        let mut lexer = Lexer::new(&code_chars);
        let tokens = lexer.tokenize().map_err(|e| ParseError {
            message: e.message,
            line: e.line,
            column: e.column,
        })?;

        let mut parser = RecursiveDescentParser::new(tokens);
        parser.parse_single_var_decl()
    }

    #[test]
    fn test_simple_var_decl() {
        let var_decl = parse_single_var_decl_from_code("let x: int32 = 42;").unwrap();

        assert_eq!(var_decl.name, "x");
        assert_eq!(var_decl.mutability, Mutability::Let);
        assert_eq!(var_decl.declared_type, Some(Type::Int32));
        assert!(matches!(
            var_decl.initializer,
            Some(Expr::Literal(Literal::Int(42)))
        ));
        assert_eq!(var_decl.line, 1);
        assert_eq!(var_decl.column, 1);
    }

    #[test]
    fn test_mut_var_decl() {
        let var_decl = parse_single_var_decl_from_code("mut count: int32 = 0;").unwrap();

        assert_eq!(var_decl.name, "count");
        assert_eq!(var_decl.mutability, Mutability::Mut);
        assert_eq!(var_decl.declared_type, Some(Type::Int32));
    }

    #[test]
    fn test_type_inference() {
        let var_decl = parse_single_var_decl_from_code("let name = \"Alice\";").unwrap();

        assert_eq!(var_decl.name, "name");
        assert_eq!(var_decl.declared_type, None);
        assert_eq!(var_decl.inferred_type, Some(Type::String));
    }

    #[test]
    fn test_no_initializer() {
        let var_decl = parse_single_var_decl_from_code("let age: int32;").unwrap();

        assert_eq!(var_decl.name, "age");
        assert_eq!(var_decl.declared_type, Some(Type::Int32));
        assert_eq!(var_decl.initializer, None);
    }

    #[test]
    fn test_public_var_decl() {
        let var_decl =
            parse_single_var_decl_from_code("public let config: string = \"default\";").unwrap();

        assert_eq!(var_decl.visibility, Some(Visibility::Public));
        assert_eq!(var_decl.name, "config");
        assert_eq!(var_decl.declared_type, Some(Type::String));
    }

    #[test]
    fn test_optional_type() {
        let var_decl = parse_single_var_decl_from_code("let age: int32?;").unwrap();

        assert_eq!(
            var_decl.declared_type,
            Some(Type::Optional(Box::new(Type::Int32)))
        );
        assert_eq!(var_decl.initializer, None);
    }

    #[test]
    fn test_multiple_declarations() {
        let code = r#"
        let x: int32 = 42;
        mut y: string = "hello";
        public let z = true;
    "#;

        let program_node = parse_program_from_code(code).unwrap();

        let declarations = match program_node {
            ASTNode::Program(prog) => prog.declarations,
            _ => panic!("Expected Program node"),
        };

        assert_eq!(declarations.len(), 3);

        // var decl #1
        match &declarations[0] {
            ASTNode::VarDecl(decl) => assert_eq!(decl.name, "x"),
            _ => panic!("Expected VarDecl"),
        }

        // var decl #2
        match &declarations[1] {
            ASTNode::VarDecl(decl) => {
                assert_eq!(decl.name, "y");
                assert_eq!(decl.mutability, Mutability::Mut);
            }
            _ => panic!("Expected VarDecl"),
        }

        //var decl #3
        match &declarations[2] {
            ASTNode::VarDecl(decl) => {
                assert_eq!(decl.name, "z");
                assert_eq!(decl.visibility, Some(Visibility::Public));
                assert_eq!(decl.inferred_type, Some(Type::Bool));
            }
            _ => panic!("Expected VarDecl"),
        }
    }

    #[test]
    fn test_type_mismatch_error() {
        let result = parse_single_var_decl_from_code("let x: string = 42;");
        assert!(result.is_err());
        assert!(result.unwrap_err().message.contains("Type mismatch"));
    }

    #[test]
    fn test_no_type_no_init_error() {
        let result = parse_single_var_decl_from_code("let x;");
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .message
            .contains("must have either a type annotation or an initializer"));
    }
}
