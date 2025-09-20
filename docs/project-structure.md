flux/
├── .gitignore
├── README.md
├── LICENSE
├── Cargo.toml # Rust package manifest
├── Makefile # Build automation
│
├── src/ # Core compiler/interpreter
│ ├── main.rs # CLI entry point
│ ├── lib.rs # Library entry point
│ │
│ ├── lexer/ # Tokenization
│ │ ├── mod.rs
│ │ ├── token.rs # Token definitions
│ │ ├── lexer.rs # Main lexer logic
│ │ └── tests.rs # Lexer unit tests
│ │
│ ├── parser/ # Syntax analysis
│ │ ├── mod.rs
│ │ ├── ast.rs # AST node definitions
│ │ ├── parser.rs # Recursive descent parser
│ │ ├── error.rs # Parse error handling
│ │ └── tests.rs # Parser unit tests
│ │
│ ├── semantic/ # Semantic analysis
│ │ ├── mod.rs
│ │ ├── type_checker.rs # Type checking
│ │ ├── symbol_table.rs # Symbol resolution
│ │ └── error.rs # Semantic errors
│ │
│ ├── interpreter/ # Tree-walking interpreter
│ │ ├── mod.rs
│ │ ├── value.rs # Runtime value types
│ │ ├── environment.rs # Variable scoping
│ │ ├── interpreter.rs # Main interpreter
│ │ ├── builtins.rs # Built-in functions
│ │ └── concurrency.rs # Async/par execution
│ │
│ ├── dsl/ # DSL processors
│ │ ├── mod.rs
│ │ ├── processor.rs # DSL processor trait
│ │ ├── sql.rs # SQL block processor
│ │ ├── html.rs # HTML block processor
│ │ ├── css.rs # CSS block processor
│ │ └── ml.rs # ML block processor
│ │
│ ├── stdlib/ # Standard library
│ │ ├── mod.rs
│ │ ├── io.rs # I/O functions
│ │ ├── collections.rs # Arrays, maps, etc.
│ │ ├── string.rs # String manipulation
│ │ ├── math.rs # Math functions
│ │ └── async_runtime.rs # Async runtime
│ │
│ ├── codegen/ # Code generation (future)
│ │ ├── mod.rs
│ │ ├── llvm.rs # LLVM backend
│ │ └── bytecode.rs # Bytecode generation
│ │
│ └── cli/ # Command-line interface
│ ├── mod.rs
│ ├── args.rs # Argument parsing
│ ├── repl.rs # Interactive REPL
│ └── file_runner.rs # File execution
│
├── stdlib/ # Standard library (Flux code)
│ ├── std/
│ │ ├── io.flux
│ │ ├── collections.flux
│ │ ├── string.flux
│ │ ├── math.flux
│ │ └── async.flux
│ │
│ ├── web/
│ │ ├── server.flux
│ │ ├── client.flux
│ │ └── http.flux
│ │
│ └── db/
│ ├── connection.flux
│ ├── query.flux
│ └── postgres.flux
│
├── examples/ # Example Flux programs
│ ├── hello_world.flux
│ ├── web_server.flux
│ ├── database_example.flux
│ ├── concurrent_processing.flux
│ ├── dsl_examples/
│ │ ├── sql_demo.flux
│ │ ├── html_template.flux
│ │ └── ml_pipeline.flux
│ └── algorithms/
│ ├── sorting.flux
│ ├── fibonacci.flux
│ └── tree_traversal.flux
│
├── tests/ # Integration tests
│ ├── integration/
│ │ ├── basic_programs.rs
│ │ ├── concurrency.rs
│ │ ├── dsl_blocks.rs
│ │ └── stdlib.rs
│ │
│ ├── fixtures/ # Test data
│ │ ├── valid_programs/
│ │ ├── invalid_programs/
│ │ └── expected_outputs/
│ │
│ └── benchmarks/ # Performance tests
│ ├── parsing.rs
│ ├── execution.rs
│ └── concurrent.rs
│
├── docs/ # Documentation
│ ├── language_spec.md # Language specification
│ ├── grammar.ebnf # EBNF grammar
│ ├── type_system.md # Type system docs
│ ├── concurrency.md # Concurrency model
│ ├── dsl_guide.md # DSL development guide
│ ├── stdlib_reference.md # Standard library docs
│ └── examples/ # Documentation examples
│
├── tools/ # Development tools
│ ├── grammar_tester/ # EBNF validation
│ ├── ast_viewer/ # AST visualization
│ ├── benchmark_suite/ # Performance testing
│ └── flux_fmt/ # Code formatter (future)
│
├── editor_support/ # IDE/Editor plugins
│ ├── vscode/ # VS Code extension
│ ├── vim/ # Vim plugin
│ └── intellij/ # IntelliJ plugin
│
└── scripts/ # Build/deployment scripts
├── build.sh
├── test.sh
├── release.sh
└── install.sh
