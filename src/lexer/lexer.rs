use crate::lexer::token::{Token, TokenType};
use std::collections::HashMap;
use once_cell::sync::Lazy;

static KEYWORDS: Lazy<HashMap<&'static str, TokenType>> = Lazy::new(|| {
    let mut map = HashMap::new();
    // Core keywords
    map.insert("use", TokenType::Use);
    map.insert("struct", TokenType::Struct);
    map.insert("enum", TokenType::Enum);
    map.insert("type", TokenType::Type);
    map.insert("let", TokenType::Let);
    map.insert("mut", TokenType::Mut);
    map.insert("const", TokenType::Const);
    map.insert("fn", TokenType::Fn);
    map.insert("if", TokenType::If);
    map.insert("else", TokenType::Else);
    map.insert("while", TokenType::While);
    map.insert("for", TokenType::For);
    map.insert("in", TokenType::In);
    map.insert("return", TokenType::Return);
    map.insert("true", TokenType::BooleanLiteral(true));
    map.insert("false", TokenType::BooleanLiteral(false));
    map.insert("async", TokenType::Async);
    map.insert("sync", TokenType::Sync);
    map.insert("par", TokenType::Par);
    map.insert("spawn", TokenType::Spawn);
    map.insert("await", TokenType::Await);

    map.insert("public", TokenType::Public);
    map.insert("private", TokenType::Private);
    map.insert("protected", TokenType::Protected);

    // DSL keywords
    for &dsl in &["sql", "html", "css", "ml", "json", "js", "regex"] {
        map.insert(dsl, TokenType::DSL(dsl.to_string()));
    }

    map
});

#[derive(Debug)]
pub struct LexError {
    pub message: String,
    pub line: usize,
    pub column: usize,
}

impl std::fmt::Display for LexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Lexer error at line {}, column {}: {}", self.line, self.column, self.message)
    }
}

impl std::error::Error for LexError {}

pub struct Lexer<'a> {
    input: &'a [char],
    position: usize,
    current_char: Option<char>,
    line: usize,
    column: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a [char]) -> Self {
        let current_char = input.get(0).copied();
        Self { input, position: 0, current_char, line: 1, column: 1 }
    }

    fn error(&self, message: impl Into<String>) -> LexError {
        LexError { message: message.into(), line: self.line, column: self.column }
    }

    fn advance(&mut self) {
        if let Some('\n') = self.current_char {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }

        self.position += 1;
        self.current_char = self.input.get(self.position).copied();
    }

    fn peek(&self) -> Option<char> {
        self.input.get(self.position + 1).copied()
    }

    fn skip_whitespace(&mut self) {
        while self.current_char.map_or(false, |c| c.is_whitespace()) {
            self.advance();
        }
    }

    fn skip_comment(&mut self) -> Result<(), LexError> {
        if self.current_char == Some('/') {
            if self.peek() == Some('/') {
                while let Some(c) = self.current_char {
                    if c == '\n' { break; }
                    self.advance();
                }
                return Ok(());
            } else if self.peek() == Some('*') {
                let start_line = self.line;
                let start_column = self.column;
                self.advance(); // skip /
                self.advance(); // skip *
                while let Some(c) = self.current_char {
                    if c == '*' && self.peek() == Some('/') {
                        self.advance(); // skip *
                        self.advance(); // skip /
                        return Ok(());
                    }
                    self.advance();
                }
                return Err(LexError { message: "Unterminated multi-line comment".to_string(), line: start_line, column: start_column });
            }
        }
        Ok(())
    }


    fn read_string(&mut self) -> Result<String, LexError> {
        let mut value = String::new();
        let start_line = self.line;
        let start_column = self.column;
        self.advance(); // skip "

        while let Some(ch) = self.current_char {
            match ch {
                '"' => { self.advance(); return Ok(value); }
                '\\' => {
                    self.advance();
                    match self.current_char {
                        Some('n') => value.push('\n'),
                        Some('t') => value.push('\t'),
                        Some('r') => value.push('\r'),
                        Some('"') => value.push('"'),
                        Some('\\') => value.push('\\'),
                        Some('0') => value.push('\0'),
                        Some('u') => {
                            self.advance();
                            let mut hex = String::new();
                            for _ in 0..4 {
                                let ch = self.current_char.ok_or(self.error("Unexpected end in unicode escape"))?;
                                hex.push(ch);
                                self.advance();
                            }
                            let code = u32::from_str_radix(&hex, 16).map_err(|_| self.error("Invalid unicode hex"))?;
                            value.push(char::from_u32(code).ok_or(self.error("Invalid unicode code point"))?);
                            continue;
                        }
                        Some('$') if self.peek() == Some('{') => {
                            value.push_str("${");
                            self.advance(); // skip $
                            self.advance(); // skip {
                            let mut brace_count = 1;
                            while let Some(c) = self.current_char {
                                self.advance();
                                value.push(c);
                                match c {
                                    '{' => brace_count += 1,
                                    '}' => { brace_count -= 1; if brace_count == 0 { break; } }
                                    _ => {}
                                }
                            }
                            if brace_count > 0 { return Err(self.error("Unterminated string interpolation")); }
                        }
                        Some(c) => return Err(self.error(format!("Invalid escape sequence: \\{}", c))),
                        None => return Err(self.error("Unexpected end in string")),
                    }
                    self.advance();
                }
                _ => { value.push(ch); self.advance(); }
            }
        }

        Err(LexError { message: "Unterminated string".to_string(), line: start_line, column: start_column })
    }

    fn read_number(&mut self) -> Result<TokenType, LexError> {
        let mut value = String::new();
        let mut is_float = false;

        while let Some(ch) = self.current_char {
            if ch.is_ascii_digit() { value.push(ch); self.advance(); }
            else if ch == '.' && !is_float && self.peek().map_or(false, |c| c.is_ascii_digit()) { value.push(ch); is_float = true; self.advance(); }
            else { break; }
        }

        if is_float { value.parse().map(TokenType::FloatLiteral).map_err(|_| self.error("Invalid float literal")) }
        else { value.parse().map(TokenType::IntLiteral).map_err(|_| self.error("Invalid integer literal")) }
    }

    fn read_identifier(&mut self) -> String {
        let mut value = String::new();
        while let Some(ch) = self.current_char {
            if ch.is_alphanumeric() || ch == '_' { value.push(ch); self.advance(); }
            else { break; }
        }
        value
    }

    fn read_dsl_content(&mut self, dsl_type: &str) -> Result<String, LexError> {
        let start_line = self.line;
        let start_column = self.column;
        self.advance(); // skip {
        let mut content = String::new();
        let mut brace_count = 1;
        let mut in_string = false;
        let mut escape_next = false;

        while let Some(ch) = self.current_char {
            self.advance();
            if escape_next { content.push(ch); escape_next = false; continue; }
            match ch {
                '\\' if in_string => { escape_next = true; content.push(ch); }
                '"' => { in_string = !in_string; content.push(ch); }
                '{' if !in_string => { brace_count += 1; content.push(ch); }
                '}' if !in_string => { brace_count -= 1; if brace_count == 0 { return Ok(content.trim().to_string()); } content.push(ch); }
                _ => { content.push(ch); }
            }
        }

        Err(LexError { message: format!("Unterminated {} block", dsl_type), line: start_line, column: start_column })
    }

    pub fn next_token(&mut self) -> Result<Token, LexError> {
        loop {
            self.skip_whitespace();
            if self.current_char == Some('/') && (self.peek() == Some('/') || self.peek() == Some('*')) { self.skip_comment()?; continue; }

            let line = self.line;
            let column = self.column;

            match self.current_char {
                None => return Ok(Token::new(TokenType::Eof, "".to_string(), line, column)),
                Some('"') => { let val = self.read_string()?; return Ok(Token::new(TokenType::StringLiteral(val.clone()), val, line, column)); }
                Some(ch) if ch.is_ascii_digit() => { let token_type = self.read_number()?; let lexeme = match &token_type { TokenType::IntLiteral(n) => n.to_string(), TokenType::FloatLiteral(f) => f.to_string(), _ => unreachable!() }; return Ok(Token::new(token_type, lexeme, line, column)); }
                Some(ch) if ch.is_alphabetic() || ch == '_' => {
                    let id = self.read_identifier();
                    if let Some(tt) = KEYWORDS.get(id.as_str()) {
                        match tt {
                            TokenType::DSL(dsl_name) => {
                                self.skip_whitespace();
                                if self.current_char == Some('{') {
                                    let content = self.read_dsl_content(dsl_name)?;
                                    return Ok(Token::new(TokenType::DSLContent { dsl_type: dsl_name.clone(), content: content.clone() }, content, line, column));
                                }
                            }
                            _ => {}
                        }
                        return Ok(Token::new(tt.clone(), id, line, column));
                    }
                    return Ok(Token::new(TokenType::Identifier(id.clone()), id, line, column));
                }
                Some('+') => { self.advance(); return Ok(Token::new(TokenType::Plus, "+".to_string(), line, column)); }
                Some('-') => {
                    if self.peek() == Some('>') { self.advance(); self.advance(); return Ok(Token::new(TokenType::Arrow, "->".to_string(), line, column)); }
                    self.advance(); return Ok(Token::new(TokenType::Minus, "-".to_string(), line, column));
                }
                Some('*') => { self.advance(); return Ok(Token::new(TokenType::Star, "*".to_string(), line, column)); }
                Some('/') => { self.advance(); return Ok(Token::new(TokenType::Slash, "/".to_string(), line, column)); }
                Some('%') => { self.advance(); return Ok(Token::new(TokenType::Percent, "%".to_string(), line, column)); }
                Some('=') => { if self.peek() == Some('=') { self.advance(); self.advance(); return Ok(Token::new(TokenType::EqualEqual, "==".to_string(), line, column)); } self.advance(); return Ok(Token::new(TokenType::Equal, "=".to_string(), line, column)); }
                Some('!') => { if self.peek() == Some('=') { self.advance(); self.advance(); return Ok(Token::new(TokenType::NotEqual, "!=".to_string(), line, column)); } self.advance(); return Ok(Token::new(TokenType::Not, "!".to_string(), line, column)); }
                Some('<') => { if self.peek() == Some('=') { self.advance(); self.advance(); return Ok(Token::new(TokenType::LessEqual, "<=".to_string(), line, column)); } self.advance(); return Ok(Token::new(TokenType::Less, "<".to_string(), line, column)); }
                Some('>') => { if self.peek() == Some('=') { self.advance(); self.advance(); return Ok(Token::new(TokenType::GreaterEqual, ">=".to_string(), line, column)); } self.advance(); return Ok(Token::new(TokenType::Greater, ">".to_string(), line, column)); }
                Some('&') => { if self.peek() == Some('&') { self.advance(); self.advance(); return Ok(Token::new(TokenType::And, "&&".to_string(), line, column)); } else { return Err(self.error("Unexpected character '&', did you mean '&&'?")); } }
                Some('|') => { if self.peek() == Some('|') { self.advance(); self.advance(); return Ok(Token::new(TokenType::Or, "||".to_string(), line, column)); } else { return Err(self.error("Unexpected character '|', did you mean '||'?")); } }
                Some('?') => { self.advance(); return Ok(Token::new(TokenType::Question, "?".to_string(), line, column)); }
                Some(':') => { self.advance(); return Ok(Token::new(TokenType::Colon, ":".to_string(), line, column)); }
                Some('(') => { self.advance(); return Ok(Token::new(TokenType::LeftParen, "(".to_string(), line, column)); }
                Some(')') => { self.advance(); return Ok(Token::new(TokenType::RightParen, ")".to_string(), line, column)); }
                Some('{') => { self.advance(); return Ok(Token::new(TokenType::LeftBrace, "{".to_string(), line, column)); }
                Some('}') => { self.advance(); return Ok(Token::new(TokenType::RightBrace, "}".to_string(), line, column)); }
                Some('[') => { self.advance(); return Ok(Token::new(TokenType::LeftBracket, "[".to_string(), line, column)); }
                Some(']') => { self.advance(); return Ok(Token::new(TokenType::RightBracket, "]".to_string(), line, column)); }
                Some(';') => { self.advance(); return Ok(Token::new(TokenType::Semicolon, ";".to_string(), line, column)); }
                Some(',') => { self.advance(); return Ok(Token::new(TokenType::Comma, ",".to_string(), line, column)); }
                Some('.') => { self.advance(); return Ok(Token::new(TokenType::Dot, ".".to_string(), line, column)); }
                Some(c) => { return Err(self.error(format!("Unexpected character '{}'", c))); }
            }
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, LexError> {
        let mut tokens = Vec::new();
        loop {
            let tok = self.next_token()?;
            let is_eof = matches!(tok.token_type, TokenType::Eof);
            tokens.push(tok);
            if is_eof { break; }
        }
        Ok(tokens)
    }
}
