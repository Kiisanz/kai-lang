// use crate::{lexer::{token::Token, TokenType}, parser::{ast::{ASTNode, Expr, Literal, Mutability, Program, Type, VarDecl, Visibility}, grammar::{NonTerminal, Production, Symbol, SymbolToken}, lalr::{Action, LALRState, LR1Item}}};
// use std::collections::{HashMap, HashSet};
// use super::super::symbol_table::SymbolTable;

// pub struct LALRParser {
//     tokens: Vec<Token>,
//     position: usize,
//     stack: Vec<usize>,
//     ast_stack: Vec<ASTNode>,
    
//     states: Vec<LALRState>,
//     action_table: Vec<HashMap<SymbolToken, Action>>,
//     goto_table: Vec<HashMap<NonTerminal, usize>>,
//     productions: Vec<Production>,
    
//     symbol_table: SymbolTable,
//     semantic_errors: Vec<String>,
// }

// impl LALRParser {
//     pub fn new(tokens: Vec<Token>) -> Self {
//         let mut parser = LALRParser {
//             tokens,
//             position: 0,
//             stack: vec![0],
//             ast_stack: Vec::new(),
//             states: Vec::new(),
//             action_table: Vec::new(),
//             goto_table: Vec::new(),
//             productions: Vec::new(),
//             symbol_table: SymbolTable::new(),
//             semantic_errors: Vec::new(),
//         };
        
//         parser.build_grammar();
//         parser.build_lalr_automaton();
//         parser
//     }
    
//     fn build_grammar(&mut self) {
//         self.productions = vec![
//             // 0: Start -> Program
//             Production {
//                 lhs: NonTerminal::Start,
//                 rhs: vec![Symbol::NonTerminal(NonTerminal::Program)],
//             },
            
//             // 1: Program -> VarDecl Program (recursive - multiple declarations)
//             Production {
//                 lhs: NonTerminal::Program,
//                 rhs: vec![
//                     Symbol::NonTerminal(NonTerminal::VarDecl),
//                     Symbol::NonTerminal(NonTerminal::Program),
//                 ],
//             },
            
//             // 2: Program -> VarDecl (base case - single declaration)
//             Production {
//                 lhs: NonTerminal::Program,
//                 rhs: vec![Symbol::NonTerminal(NonTerminal::VarDecl)],
//             },
            
//             // VarDecl productions (indices 3-6)
//             // 3: VarDecl -> Visibility Mutability Identifier : Type = Expr ;
//             Production {
//                 lhs: NonTerminal::VarDecl,
//                 rhs: vec![
//                     Symbol::NonTerminal(NonTerminal::Visibility),
//                     Symbol::NonTerminal(NonTerminal::Mutability),
//                     Symbol::Terminal(SymbolToken::Identifier),
//                     Symbol::Terminal(SymbolToken::Colon),
//                     Symbol::NonTerminal(NonTerminal::Type),
//                     Symbol::Terminal(SymbolToken::Equal),
//                     Symbol::NonTerminal(NonTerminal::Expr),
//                     Symbol::Terminal(SymbolToken::Semicolon),
//                 ],
//             },
            
//             // 4: VarDecl -> Mutability Identifier : Type = Expr ;
//             Production {
//                 lhs: NonTerminal::VarDecl,
//                 rhs: vec![
//                     Symbol::NonTerminal(NonTerminal::Mutability),
//                     Symbol::Terminal(SymbolToken::Identifier),
//                     Symbol::Terminal(SymbolToken::Colon),
//                     Symbol::NonTerminal(NonTerminal::Type),
//                     Symbol::Terminal(SymbolToken::Equal),
//                     Symbol::NonTerminal(NonTerminal::Expr),
//                     Symbol::Terminal(SymbolToken::Semicolon),
//                 ],
//             },
            
//             // 5: VarDecl -> Mutability Identifier = Expr ; (TYPE INFERENCE)
//             Production {
//                 lhs: NonTerminal::VarDecl,
//                 rhs: vec![
//                     Symbol::NonTerminal(NonTerminal::Mutability),
//                     Symbol::Terminal(SymbolToken::Identifier),
//                     Symbol::Terminal(SymbolToken::Equal),
//                     Symbol::NonTerminal(NonTerminal::Expr),
//                     Symbol::Terminal(SymbolToken::Semicolon),
//                 ],
//             },
            
//             // 6: VarDecl -> Mutability Identifier : Type ; (NO INITIALIZER)
//             Production {
//                 lhs: NonTerminal::VarDecl,
//                 rhs: vec![
//                     Symbol::NonTerminal(NonTerminal::Mutability),
//                     Symbol::Terminal(SymbolToken::Identifier),
//                     Symbol::Terminal(SymbolToken::Colon),
//                     Symbol::NonTerminal(NonTerminal::Type),
//                     Symbol::Terminal(SymbolToken::Semicolon),
//                 ],
//             },
            
//             // Visibility productions (7-9)
//             Production { lhs: NonTerminal::Visibility, rhs: vec![Symbol::Terminal(SymbolToken::Public)] },
//             Production { lhs: NonTerminal::Visibility, rhs: vec![Symbol::Terminal(SymbolToken::Private)] },
//             Production { lhs: NonTerminal::Visibility, rhs: vec![Symbol::Terminal(SymbolToken::Protected)] },
            
//             // Mutability productions (10-11)
//             Production { lhs: NonTerminal::Mutability, rhs: vec![Symbol::Terminal(SymbolToken::Let)] },
//             Production { lhs: NonTerminal::Mutability, rhs: vec![Symbol::Terminal(SymbolToken::Mut)] },
            
//             // Type productions (12-15) - basic types only
//             Production { lhs: NonTerminal::Type, rhs: vec![Symbol::Terminal(SymbolToken::TypeName("int32".to_string()))] },
//             Production { lhs: NonTerminal::Type, rhs: vec![Symbol::Terminal(SymbolToken::TypeName("string".to_string()))] },
//             Production { lhs: NonTerminal::Type, rhs: vec![Symbol::Terminal(SymbolToken::TypeName("bool".to_string()))] },
//             Production { lhs: NonTerminal::Type, rhs: vec![Symbol::Terminal(SymbolToken::TypeName("float64".to_string()))] },
            
//             // Expression & Literal productions (16-19)
//             Production { lhs: NonTerminal::Expr, rhs: vec![Symbol::NonTerminal(NonTerminal::Literal)] },
//             Production { lhs: NonTerminal::Literal, rhs: vec![Symbol::Terminal(SymbolToken::IntLiteral)] },
//             Production { lhs: NonTerminal::Literal, rhs: vec![Symbol::Terminal(SymbolToken::StringLiteral)] },
//             Production { lhs: NonTerminal::Literal, rhs: vec![Symbol::Terminal(SymbolToken::BooleanLiteral)] },
//         ];
//     }
    
//     fn build_lalr_automaton(&mut self) {
//         let initial_items = {
//             let mut items = HashSet::new();
//             items.insert(LR1Item {
//                 production_index: 0,
//                 position: 0,
//                 lookahead: SymbolToken::Eof,
//             });
//             items
//         };
        
//         self.states.push(LALRState {
//             items: self.closure(initial_items),
//             transitions: HashMap::new(),
//         });
        
//         let mut worklist = vec![0];
//         while let Some(state_index) = worklist.pop() {
//             let current_state = self.states[state_index].clone();
//             let mut transitions = HashMap::new();
//             let mut symbol_groups: HashMap<Symbol, HashSet<LR1Item>> = HashMap::new();
            
//             for item in &current_state.items {
//                 if item.position < self.productions[item.production_index].rhs.len() {
//                     let next_symbol = self.productions[item.production_index].rhs[item.position].clone();
//                     symbol_groups.entry(next_symbol).or_default().insert(LR1Item {
//                         production_index: item.production_index,
//                         position: item.position + 1,
//                         lookahead: item.lookahead.clone(),
//                     });
//                 }
//             }
            
//             for (symbol, items) in symbol_groups {
//                 let new_state = LALRState {
//                     items: self.closure(items),
//                     transitions: HashMap::new(),
//                 };
                
//                 let target_state = if let Some(existing_index) = self.find_existing_state(&new_state) {
//                     existing_index
//                 } else {
//                     let index = self.states.len();
//                     self.states.push(new_state);
//                     worklist.push(index);
//                     index
//                 };
                
//                 transitions.insert(symbol, target_state);
//             }
            
//             self.states[state_index].transitions = transitions;
//         }
//         let mut worklist = vec![0];
//     while let Some(state_index) = worklist.pop() {
//         let current_state = self.states[state_index].clone();
//         let mut transitions = HashMap::new();
//         let mut symbol_groups: HashMap<Symbol, HashSet<LR1Item>> = HashMap::new();
        
//         for (symbol, items) in symbol_groups {
//             let closure_items = self.closure(items);
//             let new_state = LALRState {
//                 items: closure_items,
//                 transitions: HashMap::new(),
//             };
            
//             // Gunakan method yang sudah ada
//             let target_state = if let Some(existing_index) = self.find_existing_state(&new_state) {
//                 existing_index
//             } else {
//                 let index = self.states.len();
//                 self.states.push(new_state);
//                 worklist.push(index);
//                 index
//             };
            
//             transitions.insert(symbol, target_state);
//         }
        
//         self.states[state_index].transitions = transitions;
//     }
//         self.build_parse_tables();

//         for (i, state) in self.states.iter().enumerate() {
//     println!("State {}: {:?}", i, state.transitions);
//     if i > 10 { break; }
// }
//     }
//     fn closure(&self, items: HashSet<LR1Item>) -> HashSet<LR1Item> {
//     let mut closure_set = items.clone();
//     let mut changed = true;
    
//     while changed {
//         changed = false;
//         let current_items = closure_set.clone();
        
//         for item in &current_items {
//             if item.position < self.productions[item.production_index].rhs.len() {
//                 if let Symbol::NonTerminal(nt) = &self.productions[item.production_index].rhs[item.position] {
//                     // Get proper lookaheads for this context
//                     let lookaheads = self.get_lookaheads_for_item(&item);
                    
//                     for (prod_index, production) in self.productions.iter().enumerate() {
//                         if production.lhs == *nt {
//                             for lookahead in &lookaheads {
//                                 let new_item = LR1Item {
//                                     production_index: prod_index,
//                                     position: 0,
//                                     lookahead: lookahead.clone(),
//                                 };
                                
//                                 if !closure_set.contains(&new_item) {
//                                     closure_set.insert(new_item);
//                                     changed = true;
//                                 }
//                             }
//                         }
//                     }
//                 }
//             }
//         }
//     }
    
//     closure_set
// }

// fn get_lookaheads_for_item(&self, item: &LR1Item) -> Vec<SymbolToken> {
//     let production = &self.productions[item.production_index];
    
//     // For VarDecl productions, we need both Equal and Semicolon as valid lookaheads
//     if production.lhs == NonTerminal::VarDecl {
//         vec![SymbolToken::Equal, SymbolToken::Semicolon, SymbolToken::Eof]
//     } else {
//         // For Type productions in VarDecl context
//         vec![SymbolToken::Equal, SymbolToken::Semicolon]
//     }
// }
    
//     fn find_existing_state(&self, new_state: &LALRState) -> Option<usize> {
//         for (index, existing_state) in self.states.iter().enumerate() {
//             let core1: HashSet<(usize, usize)> = new_state.items.iter()
//                 .map(|item| (item.production_index, item.position)).collect();
//             let core2: HashSet<(usize, usize)> = existing_state.items.iter()
//                 .map(|item| (item.production_index, item.position)).collect();
            
//             if core1 == core2 {
//                 return Some(index);
//             }
//         }
//         None
//     }
    
//    fn build_parse_tables(&mut self) {
//     self.action_table = vec![HashMap::new(); self.states.len()];
//     self.goto_table = vec![HashMap::new(); self.states.len()];
    
//     for (state_index, state) in self.states.iter().enumerate() {
//         // Deduplicate items by converting to set
//         let unique_items: HashSet<_> = state.items.iter().collect();
        
//         for item in unique_items {
//             let production = &self.productions[item.production_index];
            
//             if item.position < production.rhs.len() {
//                 match &production.rhs[item.position] {
//                     Symbol::Terminal(terminal) => {
//                         if let Some(&next_state) = state.transitions.get(&Symbol::Terminal(terminal.clone())) {
//                             // Skip conflict check temporarily
//                             self.action_table[state_index].insert(terminal.clone(), Action::Shift(next_state));
//                         }
//                     }
//                     Symbol::NonTerminal(nt) => {
//                         if let Some(&next_state) = state.transitions.get(&Symbol::NonTerminal(nt.clone())) {
//                             self.goto_table[state_index].insert(nt.clone(), next_state);
//                         }
//                     }
//                 }
//             } else {
//                 if item.production_index == 0 {
//                     self.action_table[state_index].insert(SymbolToken::Eof, Action::Accept);
//                 } else {
//                     self.action_table[state_index].insert(item.lookahead.clone(), Action::Reduce(item.production_index));
//                 }
//             }
//         }
//     }
// }
    
//     // MAIN PARSING METHOD - supports multiple declarations

//     pub fn parse(&mut self) -> Result<Program, String> {
//     self.reset_parser_state();
    
//     while self.position < self.tokens.len() {
//         let current_token = &self.tokens[self.position];
        
//         if matches!(current_token.token_type, TokenType::Eof) {
//             break;
//         }
        
//         let current_state = *self.stack.last().unwrap();
//         let lookup_symbol = self.get_lookup_symbol(&current_token.token_type);
//         let action = self.action_table[current_state].get(&lookup_symbol).unwrap_or(&Action::Error);
        
//         match action {
//             Action::Error => {
//         println!("DEBUG - State: {}, Token: {:?}, Available actions: {:?}", 
//                 current_state, current_token.token_type, 
//                 self.action_table[current_state]);
//         return Err(format!("Parse error..."));
//     }
//             Action::Shift(next_state) => {
//                 self.stack.push(*next_state);
//                 self.ast_stack.push(self.token_to_ast_node(current_token.clone()));
//                 self.position += 1;
//             }
//             Action::Reduce(prod_index) => {
            
//                 let (lhs, rhs_len) = {
//                     let production = &self.productions[*prod_index];
//                     (production.lhs.clone(), production.rhs.len())
//                 };
                
//                 let position = (current_token.line, current_token.column);
                
//                 let mut reduced_nodes = Vec::new();
//                 for _ in 0..rhs_len {
//                     self.stack.pop();
//                     if let Some(node) = self.ast_stack.pop() {
//                         reduced_nodes.push(node);
//                     }
//                 }
//                 reduced_nodes.reverse();
                
//                 let ast_node = self.reduce_to_ast_node(&lhs, reduced_nodes, *prod_index, position)?;
//                 self.ast_stack.push(ast_node);
                
//                 let current_state = *self.stack.last().unwrap();
//                 if let Some(&goto_state) = self.goto_table[current_state].get(&lhs) {
//                     self.stack.push(goto_state);
//                 } else {
//                     return Err(format!("No goto entry for state {} and {:?}", current_state, lhs));
//                 }
//             }
//             Action::Accept => {
//                 if let Some(ASTNode::Program(program)) = self.ast_stack.pop() {
//                     return Ok(program);
//                 } else {
//                     return Err("Invalid AST structure on accept".to_string());
//                 }
//             }
//             Action::Error => {
//                 return Err(format!("Parse error at line {}, column {}: unexpected token {:?}", 
//                                  current_token.line, current_token.column, current_token.token_type));
//             }
//         }
//     }
    
//     Err("Unexpected end of input".to_string())
// }
  
//     pub fn parse_single_var_decl(&mut self) -> Result<VarDecl, String> {
//         let program = self.parse()?;
//         program.declarations.into_iter().next()
//             .ok_or_else(|| "No variable declarations found".to_string())
//     }
    
//     fn reset_parser_state(&mut self) {
//         self.position = 0;
//         self.stack = vec![0];
//         self.ast_stack.clear();
//         self.semantic_errors.clear();
//         self.symbol_table = SymbolTable::new();
//     }
    
//     fn get_lookup_symbol(&self, token_type: &TokenType) -> SymbolToken {
//         match token_type {
//             TokenType::Public => SymbolToken::Public,
//             TokenType::Private => SymbolToken::Private,
//             TokenType::Protected => SymbolToken::Protected,
//             TokenType::Let => SymbolToken::Let,
//             TokenType::Mut => SymbolToken::Mut,
//             TokenType::Equal => SymbolToken::Equal,
//             TokenType::Semicolon => SymbolToken::Semicolon,
//             TokenType::Colon => SymbolToken::Colon,
//             TokenType::Identifier(name) => {
//                 if Type::from_type_name(name).is_some() {
//                     SymbolToken::TypeName(name.clone())
//                 } else {
//                     SymbolToken::Identifier
//                 }
//             }
//             TokenType::IntLiteral(_) => SymbolToken::IntLiteral,
//             TokenType::StringLiteral(_) => SymbolToken::StringLiteral,
//             TokenType::BooleanLiteral(_) => SymbolToken::BooleanLiteral,
//             TokenType::Eof => SymbolToken::Eof,
//             _ => SymbolToken::Identifier,
//         }
//     }
    
//     fn token_to_ast_node(&self, token: Token) -> ASTNode {
//         match token.token_type {
//             TokenType::Public => ASTNode::Visibility(Visibility::Public),
//             TokenType::Private => ASTNode::Visibility(Visibility::Private),
//             TokenType::Protected => ASTNode::Visibility(Visibility::Protected),
//             TokenType::Let => ASTNode::Mutability(Mutability::Let),
//             TokenType::Mut => ASTNode::Mutability(Mutability::Mut),
//             TokenType::Identifier(name) => {
//                 if let Some(t) = Type::from_type_name(&name) {
//                     ASTNode::Type(t)
//                 } else {
//                     ASTNode::Identifier(name)
//                 }
//             }
//             TokenType::IntLiteral(n) => ASTNode::Literal(Literal::Int(n)),
//             TokenType::StringLiteral(s) => ASTNode::Literal(Literal::String(s)),
//             TokenType::BooleanLiteral(b) => ASTNode::Literal(Literal::Boolean(b)),
//             _ => ASTNode::Identifier("unknown".to_string()),
//         }
//     }
    
//     fn reduce_to_ast_node(&mut self, lhs: &NonTerminal, nodes: Vec<ASTNode>, prod_index: usize, position: (usize, usize)) -> Result<ASTNode, String> {
//         match (lhs, prod_index) {
//             (NonTerminal::Start, 0) => {
//                 // Start -> Program
//                 Ok(nodes.into_iter().next().unwrap_or(ASTNode::Program(Program { declarations: vec![] })))
//             }
//             (NonTerminal::Program, 1) => {
//                 // Program -> VarDecl Program (recursive)
//                 match nodes.as_slice() {
//                     [ASTNode::VarDecl(var_decl), ASTNode::Program(program)] => {
//                         let mut new_program = program.clone();
//                         new_program.declarations.insert(0, var_decl.clone());
//                         Ok(ASTNode::Program(new_program))
//                     }
//                     _ => Err("Invalid Program -> VarDecl Program pattern".to_string()),
//                 }
//             }
//             (NonTerminal::Program, 2) => {
//                 // Program -> VarDecl (base case)
//                 if let Some(ASTNode::VarDecl(var_decl)) = nodes.first() {
//                     Ok(ASTNode::Program(Program { declarations: vec![var_decl.clone()] }))
//                 } else {
//                     Err("Invalid Program -> VarDecl".to_string())
//                 }
//             }
//             (NonTerminal::VarDecl, 3..=6) => {
//                 self.analyze_var_decl(nodes, prod_index, position)
//             }
//             (NonTerminal::Expr, 16) => {
//                 // Expr -> Literal
//                 if let Some(ASTNode::Literal(lit)) = nodes.first() {
//                     Ok(ASTNode::Expr(Expr::Literal(lit.clone())))
//                 } else {
//                     Err("Invalid Expr -> Literal".to_string())
//                 }
//             }
//             _ => {
//                 // For terminals, pass through first node
//                 Ok(nodes.into_iter().next().unwrap_or(ASTNode::Identifier("error".to_string())))
//             }
//         }
//     }
    
//     fn analyze_var_decl(&mut self, nodes: Vec<ASTNode>, prod_index: usize, position: (usize, usize)) -> Result<ASTNode, String> {
//         let mut visibility = None;
//         let mut mutability = Mutability::Let;
//         let mut name = String::new();
//         let mut declared_type = None;
//         let mut initializer = None;
//         let (line, column) = position;
        
//         // Extract components based on production pattern
//         match prod_index {
//             3 => {
//                 // VarDecl -> Visibility Mutability Identifier : Type = Expr ;
//                 if nodes.len() >= 7 {
//                     if let ASTNode::Visibility(v) = &nodes[0] { visibility = Some(v.clone()); }
//                     if let ASTNode::Mutability(m) = &nodes[1] { mutability = m.clone(); }
//                     if let ASTNode::Identifier(n) = &nodes[2] { name = n.clone(); }
//                     if let ASTNode::Type(t) = &nodes[4] { declared_type = Some(t.clone()); }
//                     if let ASTNode::Expr(e) = &nodes[6] { initializer = Some(e.clone()); }
//                 }
//             }
//             4 => {
//                 // VarDecl -> Mutability Identifier : Type = Expr ;
//                 if nodes.len() >= 6 {
//                     if let ASTNode::Mutability(m) = &nodes[0] { mutability = m.clone(); }
//                     if let ASTNode::Identifier(n) = &nodes[1] { name = n.clone(); }
//                     if let ASTNode::Type(t) = &nodes[3] { declared_type = Some(t.clone()); }
//                     if let ASTNode::Expr(e) = &nodes[5] { initializer = Some(e.clone()); }
//                 }
//             }
//             5 => {
//                 // VarDecl -> Mutability Identifier = Expr ; (TYPE INFERENCE)
//                 if nodes.len() >= 4 {
//                     if let ASTNode::Mutability(m) = &nodes[0] { mutability = m.clone(); }
//                     if let ASTNode::Identifier(n) = &nodes[1] { name = n.clone(); }
//                     if let ASTNode::Expr(e) = &nodes[3] { initializer = Some(e.clone()); }
//                 }
//             }
//             6 => {
//                 // VarDecl -> Mutability Identifier : Type ; (NO INITIALIZER)
//                 if nodes.len() >= 4 {
//                     if let ASTNode::Mutability(m) = &nodes[0] { mutability = m.clone(); }
//                     if let ASTNode::Identifier(n) = &nodes[1] { name = n.clone(); }
//                     if let ASTNode::Type(t) = &nodes[3] { declared_type = Some(t.clone()); }
//                 }
//             }
//             _ => return Err(format!("Unknown VarDecl production: {}", prod_index)),
//         }

//         // Type inference and validation with proper error positioning
//         let inferred_type = match (&declared_type, &initializer) {
//             (Some(declared), Some(init_expr)) => {
//                 let init_type = self.infer_expression_type(init_expr)?;
//                 if declared.is_compatible(&init_type) {
//                     Some(declared.clone())
//                 } else {
//                     return Err(format!(
//                         "Type mismatch at line {}, column {}: cannot assign {:?} to variable '{}' of type {:?}",
//                         line, column, init_type, name, declared
//                     ));
//                 }
//             }
//             (None, Some(init_expr)) => {
//                 Some(self.infer_expression_type(init_expr)?)
//             }
//             (Some(declared), None) => {
//                 Some(declared.clone())
//             }
//             (None, None) => {
//                 return Err(format!(
//                     "Variable '{}' at line {}, column {} requires either type annotation or initializer", 
//                     name, line, column
//                 ));
//             }
//         };

//         // Add to symbol table with proper error collection
//         let final_type = inferred_type.ok_or_else(|| "Could not determine variable type".to_string())?;
//         let has_initializer = initializer.is_some();

//         if let Err(symbol_error) = self.symbol_table.declare_variable(
//             name.clone(),
//             final_type.clone(),
//             visibility.clone(),
//             mutability.clone(),
//             has_initializer,
//             line,
//             column,
//         ) {
//             self.semantic_errors.push(format!("Line {}, Column {}: {}", line, column, symbol_error));
//         }

//         Ok(ASTNode::VarDecl(VarDecl {
//             visibility,
//             mutability,
//             name,
//             declared_type,
//             inferred_type: Some(final_type),
//             initializer,
//             line,
//             column,
//         }))
//     }
    
//     fn infer_expression_type(&self, expr: &Expr) -> Result<Type, String> {
//         match expr {
//             Expr::Literal(literal) => Ok(Type::infer_from_literal(literal)),
//             Expr::Identifier(name) => {
//                 match self.symbol_table.get_variable_type(name) {
//                     Ok(var_type) => Ok(var_type),
//                     Err(_) => Ok(Type::Int32), // Default for undeclared (semantic error handled elsewhere)
//                 }
//             }
//         }
//     }

//     pub fn get_semantic_errors(&self) -> &[String] {
//         &self.semantic_errors
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::lexer::Lexer;

//     fn parse_program_from_code(code: &str) -> Result<Program, String> {
//         let code_chars: Vec<char> = code.chars().collect();
//         let mut lexer = Lexer::new(&code_chars); // &Vec<char> deref jadi &[char]

//         let tokens = lexer.tokenize().map_err(|e| e.to_string())?;
//         let mut parser = LALRParser::new(tokens);
//         parser.parse()
//     }


//     fn parse_single_var_decl_from_code(code: &str) -> Result<VarDecl, String> {
//         let code_chars: Vec<char> = code.chars().collect();
//         let mut lexer = Lexer::new(&code_chars);
//         let tokens = lexer.tokenize().map_err(|e| e.to_string())?;
        
//         let mut parser = LALRParser::new(tokens);
//         parser.parse_single_var_decl()
//     }

//     #[test]
//     fn test_single_var_decl() {
//         let var_decl = parse_single_var_decl_from_code("let x: int32 = 42;").unwrap();
        
//         assert_eq!(var_decl.name, "x");
//         assert_eq!(var_decl.mutability, Mutability::Let);
//         assert_eq!(var_decl.declared_type, Some(Type::Int32));
//         assert!(matches!(var_decl.initializer, Some(Expr::Literal(Literal::Int(42)))));
//         // Position tracking test
//         assert_eq!(var_decl.line, 1);
//         assert_eq!(var_decl.column, 1);
//     }

//     #[test]
//     fn test_multiple_var_decls() {
//         let code = r#"
//             let x: int32 = 42;
//             mut y: string = "hello";
//             public let z = true;
//         "#;
        
//         let program = parse_program_from_code(code).unwrap();
        
//         // Test multiple declarations
//         assert_eq!(program.declarations.len(), 3);
//         assert_eq!(program.declarations[0].name, "x");
//         assert_eq!(program.declarations[1].name, "y");
//         assert_eq!(program.declarations[2].name, "z");
        
//         // Test mutability
//         assert_eq!(program.declarations[1].mutability, Mutability::Mut);
        
//         // Test visibility
//         assert_eq!(program.declarations[2].visibility, Some(Visibility::Public));
        
//         // Test type inference
//         assert_eq!(program.declarations[2].declared_type, None);
//         assert_eq!(program.declarations[2].inferred_type, Some(Type::Bool));
//     }

//     #[test]
//     fn test_type_inference() {
//         let var_decl = parse_single_var_decl_from_code("let name = \"Alice\";").unwrap();
        
//         assert_eq!(var_decl.name, "name");
//         assert_eq!(var_decl.declared_type, None);
//         assert_eq!(var_decl.inferred_type, Some(Type::String));
//         assert!(matches!(var_decl.initializer, Some(Expr::Literal(Literal::String(_)))));
//     }

//     #[test]
//     fn test_no_initializer() {
//         let var_decl = parse_single_var_decl_from_code("let age: int32;").unwrap();
        
//         assert_eq!(var_decl.name, "age");
//         assert_eq!(var_decl.declared_type, Some(Type::Int32));
//         assert_eq!(var_decl.initializer, None);
//     }

//     #[test]
//     fn test_visibility_modifiers() {
//         let var_decl = parse_single_var_decl_from_code("public let config: string = \"default\";").unwrap();
        
//         assert_eq!(var_decl.visibility, Some(Visibility::Public));
//         assert_eq!(var_decl.name, "config");
//         assert_eq!(var_decl.declared_type, Some(Type::String));
//     }

//     #[test]
//     fn test_type_mismatch_error() {
//         let result = parse_single_var_decl_from_code("let x: string = 42;");
//         assert!(result.is_err());
//         let error = result.unwrap_err();
//         assert!(error.contains("Type mismatch"));
//         assert!(error.contains("line 1, column 1"));
//     }

//     #[test]
//     fn test_no_type_no_init_error() {
//         let result = parse_single_var_decl_from_code("let x;");
//         assert!(result.is_err());
//         let error = result.unwrap_err();
//         assert!(error.contains("requires either type annotation or initializer"));
//         assert!(error.contains("line 1, column 1"));
//     }

//     #[test]
//     fn test_semantic_error_collection() {
//     let code = r#"
//         let x: int32 = 42;
//         let x: string = "duplicate";
//     "#;

//     // Lexing
//     let code_chars: Vec<char> = code.chars().collect();
//     let mut lexer = Lexer::new(&code_chars);
//     let tokens = lexer.tokenize().unwrap();

//     // Parsing
//     let mut parser = LALRParser::new(tokens);
//     let result = parser.parse();

//     // AST mungkin valid
//     assert!(result.is_ok());

//     // Semantic errors
//     let errors = parser.get_semantic_errors();
//     assert!(!errors.is_empty());
//     assert!(errors[0].contains("already declared"));
// }

//     #[test]
//     fn test_accurate_position_tracking() {
//         let code = r#"let x: int32 = 42;
// let y: string = "test";"#;
        
//         let program = parse_program_from_code(code).unwrap();
        
//         // First declaration should be on line 1
//         assert_eq!(program.declarations[0].line, 1);
//         assert_eq!(program.declarations[0].column, 1);
        
//         // Second declaration should be on line 2
//         assert_eq!(program.declarations[1].line, 2);
//         assert_eq!(program.declarations[1].column, 1);
//     }
// }

