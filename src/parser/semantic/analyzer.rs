use crate::parser::{
    BinaryOp, Expr, FnDecl, Mutability, Parameter, Type, UnaryOp, VarDecl, Visibility,
};
use crate::parser::recursive_descent::errors::ParseError;
use crate::parser::symbol_table::{SymbolError, SymbolTable};

pub struct SemanticAnalyzer<'a> {
    symbol_table: &'a mut SymbolTable,
}

impl<'a> SemanticAnalyzer<'a> {
    pub fn new(symbol_table: &'a mut SymbolTable) -> Self {
        Self { symbol_table }
    }

    pub fn reset(&mut self) {
        self.symbol_table.clear();
    }

    // ===========================
    // Variable Declaration
    // ===========================
    pub fn analyze_var_declaration(
        &mut self,
        visibility: Option<Visibility>,
        mutability: Mutability,
        name: String,
        declared_type: Option<Type>,
        initializer: Option<Expr>,
        line: usize,
        column: usize,
    ) -> Result<VarDecl, ParseError> {
        let inferred_type = match (declared_type.as_ref(), &initializer) {
            (None, None) => {
                return Err(ParseError::new(
                    "Variable must have type annotation or initializer",
                    line,
                    column,
                ))
            }
            (Some(t), None) => Some(t.clone()),
            (None, Some(expr)) => Some(self.infer_expression_type(expr, line, column)?),
            (Some(t), Some(expr)) => {
                let init_type = self.infer_expression_type(expr, line, column)?;
                if self.types_compatible(t, &init_type) {
                    Some(t.clone())
                } else {
                    return Err(ParseError::new(
                        &format!("Type mismatch: declared {:?}, but got {:?}", t, init_type),
                        line,
                        column,
                    ));
                }
            }
        };

        // Masukkan ke symbol table
        self.symbol_table
            .declare_variable(
                name.clone(),
                inferred_type.clone().unwrap_or(Type::Unknown),
                visibility,
                mutability,
                initializer.is_some(),
                line,
                column,
            )
            .map_err(|e| ParseError::new(&e.to_string(), line, column))?;

        Ok(VarDecl {
            visibility,
            mutability,
            name,
            declared_type,
            inferred_type,
            initializer,
            line,
            column,
        })
    }

    // ===========================
    // Function Declaration
    // ===========================
    pub fn analyze_func_declaration(
    &mut self,
    visibility: Option<Visibility>,
    name: String,
    params: Vec<Parameter>,
    return_type: Option<Type>,
    body: Option<Expr>, // bisa Expr::Block atau stmts
    line: usize,
    column: usize,
) -> Result<FnDecl, ParseError> {
    // Cek duplicate
    if self.symbol_table.function_exists(&name) {
        return Err(ParseError::new(
            &format!("Function '{}' already declared", name),
            line,
            column,
        ));
    }

    // Masukkan function ke symbol table dulu supaya rekursi bisa
    let param_pairs: Vec<(String, Type)> = params.iter()
        .map(|p| (p.name.clone(), p.param_type.clone()))
        .collect();
    self.symbol_table.declare_function(
        name.clone(),
        param_pairs,
        return_type.clone(),
        visibility.clone(),
        line,
        column,
    )?;

    // Analisis parameter types
    for param in &params {
        if param.param_type.clone() == Type::Unknown {
            return Err(ParseError::new(
                &format!("Parameter '{}' must have type annotation", param.name),
                line,
                column,
            ));
        }
    }

    // Analisis body jika ada
    if let Some(body_expr) = &body {
        let body_type = self.infer_expression_type(body_expr, line, column)?;
        if let Some(ret_type) = &return_type {
            if !self.types_compatible(ret_type, &body_type) {
                return Err(ParseError::new(
                    &format!("Return type mismatch: expected {:?}, got {:?}", ret_type, body_type),
                    line,
                    column,
                ));
            }
        }
    }

    Ok(FnDecl {
        visibility,
        name,
        parameters: params,
        return_type,
        body, // tetap disimpan sebagai Option<Expr>
        line,
        column,
    })
}


    // ===========================
    // Expression Type Inference
    // ===========================
    pub fn infer_expression_type(
        &self,
        expr: &Expr,
        line: usize,
        column: usize,
    ) -> Result<Type, ParseError> {
        match expr {
            Expr::Literal(lit) => Ok(Type::infer_from_literal(lit)),

            Expr::Identifier(name) => self
                .symbol_table
                .get_variable_type(name)
                .map_err(|err| ParseError::new(&err.to_string(), line, column)),

            Expr::Unary { expr, op } => {
                let expr_type = self.infer_expression_type(expr, line, column)?;
                match op {
                    UnaryOp::Negate | UnaryOp::Positive => {
                        if expr_type.is_numeric() {
                            Ok(expr_type)
                        } else {
                            Err(ParseError::new("Unary +/- requires numeric type", line, column))
                        }
                    }
                    UnaryOp::Not => {
                        if matches!(expr_type, Type::Bool) {
                            Ok(expr_type)
                        } else {
                            Err(ParseError::new("Unary ! requires boolean type", line, column))
                        }
                    }
                }
            }

            Expr::Binary { left, right, op } => {
                let left_type = self.infer_expression_type(left, line, column)?;
                let right_type = self.infer_expression_type(right, line, column)?;
                self.validate_binary_operation(&left_type, &right_type, op, line, column)?;

                match op {
                    BinaryOp::Add | BinaryOp::Sub | BinaryOp::Mul | BinaryOp::Div | BinaryOp::Mod => {
                        Ok(self.promote_numeric_types(&left_type, &right_type))
                    }
                    BinaryOp::Equal
                    | BinaryOp::NotEqual
                    | BinaryOp::Less
                    | BinaryOp::LessEqual
                    | BinaryOp::Greater
                    | BinaryOp::GreaterEqual
                    | BinaryOp::And
                    | BinaryOp::Or => Ok(Type::Bool),
                }
            }

            Expr::Assignment { name, value } => {
                self.validate_assignment(name, line, column)?;
                self.infer_expression_type(value, line, column)
            }

            Expr::Grouping(inner) => self.infer_expression_type(inner, line, column),

            Expr::Call { func_name, args } => {
                let func_info = self
                    .symbol_table
                    .get_function_info(func_name)
                    .map_err(|_| ParseError::new(&format!("Undefined function '{}'", func_name), line, column))?;

                if args.len() != func_info.parameters.len() {
                    return Err(ParseError::new(
                        &format!("Function '{}' expects {} args, got {}", func_name, func_info.parameters.len(), args.len()),
                        line,
                        column,
                    ));
                }

                for ((arg_expr, (param_name, param_type))) in args.iter().zip(func_info.parameters.iter()) {
                    let arg_type = self.infer_expression_type(arg_expr, line, column)?;
                    if !self.types_compatible(&arg_type, param_type) {
                        return Err(ParseError::new(
                            &format!("Argument '{}' expects type {:?}, got {:?}", param_name, param_type, arg_type),
                            line,
                            column,
                        ));
                    }
                }

                Ok(func_info.return_type.clone().unwrap_or(Type::Unknown))
            }
        }
    }

    // ===========================
    // Assignment Validation
    // ===========================
    pub fn validate_assignment(&self, name: &str, line: usize, column: usize) -> Result<(), ParseError> {
        match self.symbol_table.get_variable_type(name) {
            Ok(_) => {
                let symbol = self.symbol_table.get_all_variables().get(name).unwrap();
                if !symbol.is_mutable() {
                    return Err(ParseError::new(&format!("Cannot assign to immutable variable '{}'", name), line, column));
                }
            }
            Err(_) => {
                return Err(ParseError::new(&format!("Undefined variable '{}'", name), line, column));
            }
        }
        Ok(())
    }

    // ===========================
    // Binary Operation Validation
    // ===========================
    fn validate_binary_operation(
        &self,
        left: &Type,
        right: &Type,
        op: &BinaryOp,
        line: usize,
        column: usize,
    ) -> Result<(), ParseError> {
        match op {
            BinaryOp::Add | BinaryOp::Sub | BinaryOp::Mul | BinaryOp::Div | BinaryOp::Mod => {
                if !left.is_numeric() || !right.is_numeric() {
                    return Err(ParseError::new("Arithmetic operations require numeric types", line, column));
                }
            }
            BinaryOp::And | BinaryOp::Or => {
                if !matches!(left, Type::Bool) || !matches!(right, Type::Bool) {
                    return Err(ParseError::new("Logical operations require boolean types", line, column));
                }
            }
            BinaryOp::Equal | BinaryOp::NotEqual => {
                if !self.types_compatible(left, right) {
                    return Err(ParseError::new("Equality comparison requires compatible types", line, column));
                }
            }
            BinaryOp::Less | BinaryOp::LessEqual | BinaryOp::Greater | BinaryOp::GreaterEqual => {
                if !left.is_numeric() || !right.is_numeric() {
                    return Err(ParseError::new("Comparison operations require numeric types", line, column));
                }
            }
        }
        Ok(())
    }

    fn promote_numeric_types(&self, left: &Type, right: &Type) -> Type {
        use Type::*;
        let priority = |t: &Type| match t {
            Float64 => 4,
            Float32 => 3,
            Int64 | Uint64 => 2,
            Int32 | Uint32 => 1,
            Int16 | Uint16 | Int8 | Uint8 => 0,
            _ => -1,
        };
        if priority(left) >= priority(right) { left.clone() } else { right.clone() }
    }

    fn types_compatible(&self, t1: &Type, t2: &Type) -> bool {
        match (t1, t2) {
            (a, b) if a == b => true,
            (Type::Optional(inner1), t2) => self.types_compatible(inner1, t2),
            (t1, Type::Optional(inner2)) => self.types_compatible(t1, inner2),
            (a, b) if a.is_numeric() && b.is_numeric() => true, // numeric promotion
            _ => false,
        }
    }

    pub fn get_symbol_table_mut(&mut self) -> &mut SymbolTable {
        &mut self.symbol_table
    }
}
