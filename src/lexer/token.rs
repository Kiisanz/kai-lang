#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // Literals
    IntLiteral(i64),
    FloatLiteral(f64),
    StringLiteral(String),
    BooleanLiteral(bool),
    Identifier(String),

    // Keywords
    Use, Struct, Enum, Type, Let, Mut, Const, Fn,
    If, Else, While, For, In, Return, Async, Sync, Par, Spawn, Await,
    Public, Private, Protected,

    // DSL Keywords
    DSL(String), 

    // Operators
    Plus,           // +
    Minus,          // -
    Star,           // *
    Slash,          // /
    Percent,        // %
    
    Equal,          // =
    EqualEqual,     // ==
    NotEqual,       // !=
    Less,           // <
    Greater,        // >
    LessEqual,      // <=
    GreaterEqual,   // >=
    
    And,            // &&
    Or,             // ||
    Not,            // !
    
    Question,       // ?
    Colon,          // :
    Arrow,          // ->

    // Delimiters
    LeftParen,      // (
    RightParen,     // )
    LeftBrace,      // {
    RightBrace,     // }
    LeftBracket,    // [
    RightBracket,   // ]
    Semicolon,      // ;
    Comma,          // ,
    Dot,            // .

    // Special
    Eof,
    
    // DSL Content
   DSLContent { dsl_type: String, content: String },
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: usize,
    pub column: usize,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, line: usize, column: usize) -> Self {
        Token { token_type, lexeme, line, column }
    }
}

