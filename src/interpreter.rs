use crate::{ast::Node, parser, tokens::TokenType};

pub struct Interpreter;

pub trait Compile {
    type Output;

    fn from_ast(ast: Node) -> Self::Output;

    fn from_file(source: &str) -> Result<Self::Output, String> {
        println!("Interpreting expressions in {}", source);
        let ast = parser::parse_from_file(source)?;
        Ok(Self::from_ast(ast))
    }

    fn from_input(input: &str) -> Result<Self::Output, String> {
        let ast = parser::parse_from_input(input)?;
        Ok(Self::from_ast(ast))
    }
}

impl Compile for Interpreter {
    type Output = f64;

    fn from_ast(ast: Node) -> Self::Output {
        let eval = Eval::new();
        eval.evaluate(&ast)
    }
}

struct Eval;

impl Eval {
    pub fn new() -> Self {
        Self
    }

    pub fn evaluate(&self, node: &Node) -> f64 {
        match node {
            Node::Number(num) => *num,
            Node::UnaryExpr { operator, child } => {
                let child = self.evaluate(child);
                match operator {
                    TokenType::Minus => -child,
                    TokenType::Plus => child,
                    // This should never be reached, but just in case
                    _ => panic!("Unknown unary operator: {}", operator),
                }
            }
            Node::BinaryExpr { operator, lhs, rhs } => {
                let rhs = self.evaluate(rhs);
                let lhs = self.evaluate(lhs);
                match operator {
                    TokenType::Plus => lhs + rhs,
                    TokenType::Minus => lhs - rhs,
                    TokenType::Carrot => lhs.powf(rhs),
                    TokenType::Slash => lhs / rhs,
                    TokenType::Star => lhs * rhs,
                    // Same here, should never be reached.
                    _ => panic!("Unknown binary operator: {}", operator),
                }
            }
            Node::GroupingExpr { child } => self.evaluate(child),
        }
    }
}
