use crate::lexer::TokenType;

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub enum Expr {
    Literal(Literal),
    Identifier(String),
    Unary { op: UnaryOp, expr: Box<Expr> },
    Binary { left: Box<Expr>, op: BinaryOp, right: Box<Expr> },
    Assignment { name: String, value: Box<Expr> },
    Grouping(Box<Expr>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Int(i64),
    Float(f64),
    String(String),
    Boolean(bool),
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOp {
    Positive,
    Negate, // -
    Not,    // !
}

impl UnaryOp {
    pub fn from_token(token: &TokenType) -> Option<Self> {
        match token {
            TokenType::Plus  => Some(UnaryOp::Positive), // unary plus
            TokenType::Minus => Some(UnaryOp::Negate),   // unary minus
            TokenType::Not   => Some(UnaryOp::Not),      // logical not
            _ => None,
        }
    }
}


#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOp {
    Add, Sub, Mul, Div, Mod,
    Greater, GreaterEqual,
    Less, LessEqual,
    Equal, NotEqual,
    And, Or,
}

#[allow(dead_code)]
impl BinaryOp {
    pub fn from_token(token: &TokenType) -> Option<Self> {
        match token {
            TokenType::Plus => Some(BinaryOp::Add),
            TokenType::Minus => Some(BinaryOp::Sub),
            TokenType::Star => Some(BinaryOp::Mul),
            TokenType::Slash => Some(BinaryOp::Div),
            TokenType::Percent => Some(BinaryOp::Mod),
            TokenType::EqualEqual => Some(BinaryOp::Equal),
            TokenType::NotEqual => Some(BinaryOp::NotEqual),
            TokenType::Greater => Some(BinaryOp::Greater),
            TokenType::GreaterEqual => Some(BinaryOp::GreaterEqual),
            TokenType::Less => Some(BinaryOp::Less),
            TokenType::LessEqual => Some(BinaryOp::LessEqual),
            TokenType::And => Some(BinaryOp::And),
            TokenType::Or => Some(BinaryOp::Or),
            _ => None,
        }
    }
}
