use std::fmt::Display;

use crate::tokens::TokenType;

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

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Number(num) => write!(f, "{}", num),
            Boolean(val) => write!(f, "{}", val),
        }
    }
}
