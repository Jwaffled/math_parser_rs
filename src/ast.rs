use std::fmt::Display;

use crate::{parser, tokens::TokenType, interpreter::evaluate};

#[derive(Debug)]
pub enum Node {
    Number(f64),
    Boolean(bool),
    UnaryExpr {
        operator: TokenType,
        child: Box<Node>,
    },
    BinaryExpr {
        operator: TokenType,
        lhs: Box<Node>,
        rhs: Box<Node>,
    },
    GroupingExpr {
        child: Box<Node>,
    },
}

#[derive(Debug)]
pub enum Value {
    Number(f64),
    Boolean(bool),
}

impl Value {
    pub fn from_file(source: &str) -> Result<Self, String> {
        println!("Interpreting expressions in {}", source);
        
        Self::try_from(parser::parse_from_file(source)?)
    }

    #[inline]
    pub fn from_input(input: &str) -> Result<Self, String> {
        Self::try_from(parser::parse_from_input(input)?)
    }
}

impl TryFrom<Node> for Value {
    type Error = String;
  
    #[inline]
    fn try_from(ast: Node) -> Result<Self, Self::Error> {
        evaluate(&ast)
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Number(num) => write!(f, "{}", num),
            Self::Boolean(val) => write!(f, "{}", val),
        }
    }
}
