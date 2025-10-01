use std::collections::HashMap;
use crate::parser::{symbol_table::SymbolError, Mutability, Type, Visibility};

#[derive(Debug, Clone)]
pub struct SymbolTable {
    variables: HashMap<String, VariableInfo>,
    functions: HashMap<String, FunctionInfo>, 
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct VariableInfo {
    pub var_type: Type,
    pub visibility: Option<Visibility>,
    pub mutability: Mutability,
    pub initialized: bool,
    pub line: usize,
    pub column: usize,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FunctionInfo {
    pub parameters: Vec<(String, Type)>,
    pub return_type: Option<Type>,
    pub visibility: Option<Visibility>,
    pub line: usize,
    pub column: usize,
}

impl VariableInfo {
    pub fn is_mutable(&self) -> bool {
        matches!(self.mutability, Mutability::Mut)
    }
}

#[allow(dead_code)]
impl SymbolTable {
    pub fn new() -> Self {
        SymbolTable {
            variables: HashMap::new(),
            functions: HashMap::new(),
        }
    }

    // ==================== Variabel ====================
    pub fn declare_variable(
        &mut self,
        name: String,
        var_type: Type,
        visibility: Option<Visibility>,
        mutability: Mutability,
        initialized: bool,
        line: usize,
        column: usize,
    ) -> Result<(), SymbolError> {
        if self.variables.contains_key(&name) {
            let existing = self.variables.get(&name).unwrap();
            return Err(SymbolError::AlreadyDeclared(name, existing.line, existing.column));
        }

        self.variables.insert(name, VariableInfo {
            var_type,
            visibility,
            mutability,
            initialized,
            line,
            column,
        });

        Ok(())
    }

    pub fn get_variable_type(&self, name: &str) -> Result<Type, SymbolError> {
        self.variables.get(name)
            .map(|info| info.var_type.clone())
            .ok_or_else(|| SymbolError::Undeclared(name.to_string()))
    }

    pub fn update_variable(&mut self, name: &str, new_type: Type) -> Result<(), SymbolError> {
        if let Some(var_info) = self.variables.get_mut(name) {
            if matches!(var_info.mutability, Mutability::Let) {
                return Err(SymbolError::ImmutableAssignment(name.to_string()));
            }
            var_info.var_type = new_type;
            var_info.initialized = true;
            Ok(())
        } else {
            Err(SymbolError::Undeclared(name.to_string()))
        }
    }

    // ==================== Fungsi ====================
    pub fn declare_function(
        &mut self,
        name: String,
        parameters: Vec<(String, Type)>,
        return_type: Option<Type>,
        visibility: Option<Visibility>,
        line: usize,
        column: usize,
    ) -> Result<(), SymbolError> {
        if self.functions.contains_key(&name) {
            let existing = self.functions.get(&name).unwrap();
            return Err(SymbolError::AlreadyDeclared(name, existing.line, existing.column));
        }

        self.functions.insert(name, FunctionInfo {
            parameters,
            return_type,
            visibility,
            line,
            column,
        });

        Ok(())
    }

    pub fn get_function_info(&self, name: &str) -> Result<&FunctionInfo, SymbolError> {
        self.functions.get(name)
            .ok_or_else(|| SymbolError::Undeclared(name.to_string()))
    }

    // ==================== Getter lainnya ====================
    pub fn variable_exists(&self, name: &str) -> bool {
        self.variables.contains_key(name)
    }

    pub fn function_exists(&self, name: &str) -> bool {
        self.functions.contains_key(name)
    }

    pub fn get_all_variables(&self) -> &HashMap<String, VariableInfo> {
        &self.variables
    }

    pub fn get_all_variables_owned(&self) -> HashMap<String, VariableInfo> {
        self.variables.clone()
    }

    pub fn variable_count(&self) -> usize {
        self.variables.len()
    }

    pub fn clear(&mut self) {
        self.variables.clear();
        self.functions.clear();
    }
}
