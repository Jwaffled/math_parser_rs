use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    LeftParen,
    RightParen,
    Comma,

    // Math
    Plus,
    Minus,
    Star,
    Slash,
    Carrot,

    // Boolean logic
    Not,
    GreaterThan,
    GreaterThanEqual,
    LessThan,
    LessThanEqual,
    EqualEqual,
    NotEqual,
    And,
    Or,

    Identifier(String),
    Number(f64),

    Eof,
}

impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{:?}", &self)
    }
}

#[derive(Clone, Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: usize,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, line: usize) -> Self {
        Token {
            token_type,
            lexeme,
            line,
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(
            f,
            "Type: {}, Lexeme: {}, Line: {}",
            &self.token_type, &self.lexeme, &self.line
        )
    }
}
