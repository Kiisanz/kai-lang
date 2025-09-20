mod lexer;
mod parser;

use lexer::{Lexer, TokenType};
use std::env;
use std::fs;
use std::process;
use parser::RecursiveDescentParser;

use crate::parser::ASTNode;
use crate::parser::Mutability;
use crate::parser::SymbolTable;
use crate::parser::Visibility;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage: {} <command> [file]", args[0]);
        eprintln!("Commands:");
        eprintln!("  lex <file>     - Tokenize a Flux file");
        eprintln!("  parse <file>   - Parse a Flux file into AST");
        eprintln!("  repl           - Start interactive REPL");
        eprintln!("  test           - Run quick tests");
        process::exit(1);
    }

    match args[1].as_str() {
        "lex" => {
            if args.len() < 3 {
                eprintln!("Usage: {} lex <file>", args[0]);
                process::exit(1);
            }
            tokenize_file(&args[2]);
        }
        "parse" => {
            if args.len() < 3 {
                eprintln!("Usage: {} parse <file>", args[0]);
                process::exit(1);
            }
            parse_file(&args[2]);
        }
        "repl" => start_repl(),
        "test" => run_quick_tests(),
        _ => {
            eprintln!("Unknown command: {}", args[1]);
            process::exit(1);
        }
    }
}

fn tokenize_file(filename: &str) {
    let contents = fs::read_to_string(filename)
        .unwrap_or_else(|err| { 
            eprintln!("Error reading file '{}': {}", filename, err); 
            process::exit(1); 
        });

    let code_chars: Vec<char> = contents.chars().collect();
    let mut lexer = Lexer::new(&code_chars);
    
    match lexer.tokenize() {
        Ok(tokens) => {
            println!("✅ Tokenization successful! Found {} tokens:\n", tokens.len());
            for (i, token) in tokens.iter().enumerate() {
                if matches!(token.token_type, TokenType::Eof) {
                    println!("{:3}: {:?}", i, token.token_type);
                } else {
                    println!("{:3}: {:?} '{}' @ {}:{}", 
                        i, token.token_type, token.lexeme, token.line, token.column
                    );
                }
            }
        }
        Err(err) => {
            eprintln!("❌ Lexer error: {}", err);
            process::exit(1);
        }
    }
}

fn parse_file(filename: &str) {
    let contents = fs::read_to_string(filename)
        .unwrap_or_else(|err| { 
            eprintln!("Error reading file '{}': {}", filename, err); 
            process::exit(1); 
        });

    // Lexing
    let code_chars: Vec<char> = contents.chars().collect();
    let mut lexer = Lexer::new(&code_chars);
    let tokens = match lexer.tokenize() {
        Ok(t) => t,
        Err(err) => {
            eprintln!("❌ Lexer error: {}", err);
            process::exit(1);
        }
    };

    // Parsing with RD parser
    let mut parser = RecursiveDescentParser::new(tokens);
    match parser.parse_program() {
    Ok(ast_node) => {
        if let ASTNode::Program(program) = ast_node {
            println!("✅ Parsing successful! Program has {} variable declarations.", program.declarations.len());
            
            // Display parsed variables
            if !program.declarations.is_empty() {
    println!("\nVariable Declarations:");
    for (i, node) in program.declarations.iter().enumerate() {
        if let ASTNode::VarDecl(decl) = node {
            let vis_str = match &decl.visibility {
                Some(v) => format!("{:?} ", v).to_lowercase(),
                None => "".to_string(),
            };
            let mut_str = match decl.mutability {
                parser::ast::Mutability::Let => "let",
                parser::ast::Mutability::Mut => "mut",
            };
            let type_str = match &decl.inferred_type {
                Some(t) => format!("{:?}", t),
                None => "unknown".to_string(),
            };
            let init_str = if decl.initializer.is_some() { " (initialized)" } else { "" };

            println!(
                "  {}. {}{} {} : {}{}",
                i + 1, vis_str, mut_str, decl.name, type_str, init_str
            );
        } else {
            println!("  ⚠️  Found non-variable declaration in program");
        }
    }
}

            
            // Display semantic errors if any
            let errors = parser.get_semantic_errors();
            if !errors.is_empty() {
                println!("\n⚠️  Semantic warnings:");
                for e in errors {
                    println!("  - {}", e);
                }
            }
        } else {
            eprintln!("❌ Expected ASTNode::Program, got something else");
            process::exit(1);
        }
    }
    Err(err) => {
        eprintln!("❌ Parser error: {}", err);
        process::exit(1);
    }
}

}

fn start_repl() {
    use std::io::{self, Write};
    println!("🚀 Flux REPL (Persistent Symbol Table)");
    println!("Commands: 'symbols', 'clear', 'exit'");
    
    let mut persistent_symbol_table = SymbolTable::new();
    let mut line_number = 1;
    
    loop {
        print!("flux:{}> ", line_number);
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() { 
            break; 
        }
        
        let input = input.trim();
        if input.is_empty() { 
            continue; 
        }
        
        // Handle REPL commands
        match input {
            "exit" => {
                println!("Goodbye! 👋");
                break;
            }
            "symbols" => {
                print_symbol_table(&persistent_symbol_table);
                line_number += 1;
                continue;
            }
            "clear" => {
                persistent_symbol_table.clear();
                println!("🗑️ Symbol table cleared");
                line_number = 1;
                continue;
            }
            _ => {}
        }
        
        // Lex current line
        let code_chars: Vec<char> = input.chars().collect();
        let mut lexer = Lexer::new(&code_chars);
        let tokens = match lexer.tokenize() {
            Ok(t) => t,
            Err(err) => { 
                eprintln!("  ❌ Lexer error: {}", err); 
                line_number += 1; 
                continue; 
            }
        };
        
        // Create fresh parser with persistent symbol table
        let mut parser = RecursiveDescentParser::new(tokens);
        parser.set_symbol_table(persistent_symbol_table.clone());
        
        // Parse single declaration
        match parser.parse_single_var_decl() {
            Ok(var_decl) => {
                let visibility_str = var_decl.visibility
                    .map(|v| format!("{:?} ", v).to_lowercase())
                    .unwrap_or_default();
                
                let mutability_str = match var_decl.mutability { 
                    Mutability::Let => "let", 
                    Mutability::Mut => "mut" 
                };
                
                let type_str = var_decl.inferred_type
                    .as_ref()
                    .map(|t| format!("{:?}", t))
                    .unwrap_or_else(|| "Unknown".to_string());
                
                println!("  ✅ Parsed: {}{} {} : {}",
                    visibility_str,
                    mutability_str,
                    var_decl.name,
                    type_str
                );
                
                // Update persistent symbol table
                persistent_symbol_table = parser.get_symbol_table().clone();
                
                // Show warnings if any
                let errors = parser.get_semantic_errors();
                if !errors.is_empty() {
                    for error in errors {
                        eprintln!("  ⚠️ Warning: {}", error);
                    }
                }
            }
            Err(err) => {
                eprintln!("  ❌ Parse error: {}", err);
            }
        }
        
        line_number += 1;
    }
}


fn print_symbol_table(symbol_table: &SymbolTable) {
    let vars = symbol_table.get_all_variables();
    
    if vars.is_empty() {
        println!("📋 Symbol table is empty");
        return;
    }
    
    println!("📋 Symbol Table ({} variables):", vars.len());
    println!("┌─────────────────┬──────────────┬────────────┬─────────┬──────────────┐");
    println!("│ Name            │ Type         │ Mutability │ Vis     │ Initialized  │");
    println!("├─────────────────┼──────────────┼────────────┼─────────┼──────────────┤");
    
    for (name, info) in vars.iter() {
        let vis_str = match &info.visibility {
            Some(Visibility::Public) => "pub",
            Some(Visibility::Private) => "priv",
            Some(Visibility::Protected) => "prot",
            None => "-",
        };
        
        let mut_str = match info.mutability {
            Mutability::Let => "let",
            Mutability::Mut => "mut",
        };
        
        let init_str = if info.initialized { "✓" } else { "✗" };
        
        println!("│ {:<15} │ {:<12} │ {:<10} │ {:<7} │ {:<12} │",
            truncate_string(name, 15),
            truncate_string(&format!("{:?}", info.var_type), 12),
            mut_str,
            vis_str,
            init_str
        );
    }
    
    println!("└─────────────────┴──────────────┴────────────┴─────────┴──────────────┘");
}

fn truncate_string(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[..max_len-3])
    }
}
fn run_quick_tests() {
    use std::process;

    println!("🧪 Running Flux parser tests with Recursive Descent...\n");

    let test_cases = vec![
        ("Basic let", "let x: int32 = 42;"),
        ("Mutable", "mut count: int32 = 0;"),
        ("Type inference", "let name = \"Alice\";"),
        ("Public visibility", "public let config: string = \"prod\";"),
        ("No initializer", "let age: int32;"),
        ("Optional type", "let data: string?;"),
        ("Float", "let pi: float64 = 3.14159;"),
        ("Boolean", "let flag = true;"),
        ("Private", "private mut counter: int32 = 0;"),
        ("Protected", "protected let secret: string;"),
    ];

    println!("📝 Variable Declaration Tests:");
    let mut passed = 0;
    let total = test_cases.len();

    for (name, code) in &test_cases { // gunakan &test_cases agar tidak move
        print!("  Testing {:<18} ... ", name);

        let code_chars: Vec<char> = code.chars().collect();
        let mut lexer = Lexer::new(&code_chars);
        match lexer.tokenize() {
            Ok(tokens) => {
                let mut parser = RecursiveDescentParser::new(tokens);
                match parser.parse_single_var_decl() {
                    Ok(_) => {
                        println!("✅");
                        passed += 1;
                    }
                    Err(err) => println!("❌ Parse error: {}", err),
                }
            }
            Err(err) => println!("❌ Lex error: {}", err),
        }
    }

    println!("\n📊 Variable Declaration Results: {}/{} tests passed", passed, total);

    // Multi-declaration test
    println!("\n📝 Multi-Declaration Test:");
    let multi_code = r#"
        let x: int32 = 42;
        mut y: string = "hello";
        public let z = true;
        private let secret: int32;
    "#;

    print!("  Testing multiple declarations ... ");
    let multi_chars: Vec<char> = multi_code.chars().collect();
    let mut lexer = Lexer::new(&multi_chars);
    match lexer.tokenize() {
    Ok(tokens) => {
        let mut parser = RecursiveDescentParser::new(tokens);
        match parser.parse_program() {
            Ok(ast_node) => {
                // Pastikan ASTNode::Program
                if let ASTNode::Program(program) = ast_node {
                    if program.declarations.len() == 4 {
                        println!("✅ (parsed {} declarations)", program.declarations.len());
                        passed += 1;
                    } else {
                        println!("❌ Expected 4 declarations, got {}", program.declarations.len());
                    }
                } else {
                    println!("❌ Expected ASTNode::Program, got something else");
                }
            }
            Err(err) => println!("❌ Parse error: {}", err),
        }
    }
    Err(err) => println!("❌ Lex error: {}", err),
}

    // Error cases
    println!("\n🚨 Error Handling Tests:");
    let error_cases = vec![
        ("No type or init", "let x;"),
        ("Type mismatch", "let x: string = 42;"),
        ("Missing semicolon", "let x: int32 = 42"),
        ("Invalid syntax", "let : int32 = 42;"),
        ("Missing identifier", "let = 42;"),
    ];

    let mut error_passed = 0;
    for (name, code) in &error_cases { // gunakan &error_cases
        print!("  Testing {:<18} ... ", name);

        let code_chars: Vec<char> = code.chars().collect();
        let mut lexer = Lexer::new(&code_chars);
        match lexer.tokenize() {
            Ok(tokens) => {
                let mut parser = RecursiveDescentParser::new(tokens);
                match parser.parse_single_var_decl() {
                    Ok(_) => println!("❌ Should have failed"),
                    Err(_) => {
                        println!("✅ Correctly caught error");
                        error_passed += 1;
                    }
                }
            }
            Err(_) => {
                println!("✅ Correctly caught lexer error");
                error_passed += 1;
            }
        }
    }

    let total_tests = total + 1 + error_cases.len(); // +1 for multi-declaration
    let total_passed = passed + error_passed;

    println!("\n🏁 Overall Results: {}/{} tests passed", total_passed, total_tests);

    if total_passed == total_tests {
        println!("🎉 All tests passed! Recursive Descent parser is working correctly.");
    } else {
        println!("⚠️  Some tests failed. Check implementation.");
        process::exit(1);
    }
}

// Helper function to create sample Flux files for testing
#[allow(dead_code)]
fn create_sample_files() {
    let samples = vec![
        ("variables.flux", r#"
// Variable declaration examples
public let config: string = "production";
mut counter: int32 = 0;
private let secret_key: string;
let user_data = load_user_data();
protected mut cache: string? = null;
"#),
        ("types.flux", r#"
// Type system examples
let small_num: int8 = 127;
let big_num: int64 = 9223372036854775807;
let precise: float64 = 3.141592653589793;
let flag: bool = true;
let name: string = "Flux Language";
let maybe_age: int32? = 25;
"#),
        ("visibility.flux", r#"
// Visibility modifier examples
public let public_config: string = "everyone can see this";
private mut private_counter: int32 = 0;
protected let protected_data: string;
let package_visible: bool = false;
"#),
    ];

    for (filename, content) in samples {
        if let Err(e) = fs::write(filename, content) {
            eprintln!("Warning: Could not create {}: {}", filename, e);
        } else {
            println!("Created sample file: {}", filename);
        }
    }
}