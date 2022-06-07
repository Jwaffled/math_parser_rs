use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    LeftParen,
    RightParen,
    Comma,

    Plus,
    Minus,
    Star,
    Slash,
    Carrot,

    Identifier(String),
    Number(f64),

    Eof,
}

pub enum Operators {
    Plus,
    Minus,
    Star,
    Slash,
    Carrot,
}

impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{:?}", &self)
    }
}

// #[derive(Debug, Clone)]
// pub enum TokenValue<'a> {
//     String(&'a str),
//     Number(f64),
// }

// impl TryInto<f64> for TokenValue<'_> {
//     type Error = String;

//     fn try_into(self) -> Result<f64, Self::Error> {
//         match self {
//             TokenValue::Number(num) => Ok(num),
//             TokenValue::String(_str) => Err(String::from("TokenValue was string, expected f64"))
//         }
//     }
// }

// impl Display for TokenValue<'_> {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
//         write!(f, "{:?}", &self)
//     }
// }

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
