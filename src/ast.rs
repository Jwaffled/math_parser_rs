use crate::tokens::TokenType;

#[derive(Debug)]
pub enum Node {
    Number(f64),
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
