use crate::{ast::{Node, Value}, parser, tokens::TokenType};

pub struct Interpreter;

pub trait Compile {
    type Output;

    fn from_ast(ast: Node) -> Result<Self::Output, String>;

    fn from_file(source: &str) -> Result<Self::Output, String> {
        println!("Interpreting expressions in {}", source);
        let ast = parser::parse_from_file(source)?;
        Ok(Self::from_ast(ast)?)
    }

    fn from_input(input: &str) -> Result<Self::Output, String> {
        let ast = parser::parse_from_input(input)?;
        Ok(Self::from_ast(ast)?)
    }
}

impl Compile for Interpreter {
    type Output = Value;

    fn from_ast(ast: Node) -> Result<Self::Output, String> {
        let eval = Eval::new();
        eval.evaluate(&ast)
    }
}

struct Eval;

impl Eval {
    pub fn new() -> Self {
        Self
    }

    pub fn evaluate(&self, node: &Node) -> Result<Value, String> {
        match node {
            Node::Number(num) => Ok(Value::Number(*num)),
            Node::Boolean(val) => Ok(Value::Boolean(*val)),
            Node::UnaryExpr { operator, child } => {
                let child = self.evaluate(child)?;
                match child {
                    Value::Number(num) => match operator {
                        TokenType::Minus => return Ok(Value::Number(-num)),
                        TokenType::Plus => return Ok(Value::Number(num)),
                        other => return Err(format!("Incompatible operand type (Number) for operation '{}'.", other))
                    },
                    Value::Boolean(val) => match operator {
                        TokenType::Not => return Ok(Value::Boolean(!val)),
                        other => return Err(format!("Incompatible operand type (Boolean) for operation '{}'.", other))
                    }
                }
            }
            Node::BinaryExpr { operator, lhs, rhs } => {
                let rhs = self.evaluate(rhs)?;
                let lhs = self.evaluate(lhs)?;
                match (rhs, lhs) {
                    (Value::Number(rhs), Value::Number(lhs)) => {
                        match operator {
                            TokenType::Plus => Ok(Value::Number(lhs + rhs)),
                            TokenType::Minus => Ok(Value::Number(lhs - rhs)),
                            TokenType::Star => Ok(Value::Number(lhs * rhs)),
                            TokenType::Slash => Ok(Value::Number(lhs / rhs)),
                            TokenType::Carrot => Ok(Value::Number(lhs.powf(rhs))),
                            TokenType::EqualEqual => Ok(Value::Boolean(lhs == rhs)),
                            TokenType::NotEqual => Ok(Value::Boolean(lhs != rhs)),
                            TokenType::LessThan => Ok(Value::Boolean(lhs < rhs)),
                            TokenType::LessThanEqual => Ok(Value::Boolean(lhs <= rhs)),
                            TokenType::GreaterThan => Ok(Value::Boolean(lhs > rhs)),
                            TokenType::GreaterThanEqual => Ok(Value::Boolean(lhs >= rhs)),
                            other => Err(format!("Cannot apply operator '{}' to two Numbers.", other))
                        }
                    },
                    (Value::Boolean(rhs), Value::Boolean(lhs)) => {
                        match operator {
                            TokenType::NotEqual => Ok(Value::Boolean(lhs != rhs)),
                            TokenType::EqualEqual => Ok(Value::Boolean(lhs == rhs)),
                            TokenType::Or => Ok(Value::Boolean(lhs || rhs)),
                            TokenType::And => Ok(Value::Boolean(lhs && rhs)),
                            other => Err(format!("Cannot apply operator '{}' to two Booleans.", other))
                        }
                    }
                    (rhs, lhs) => return Err(format!("Left hand and right hand side of the expression are not of the same type. (Operating on '{:?}' and '{:?}')", rhs, lhs))
                }
            }
            Node::GroupingExpr { child } => self.evaluate(child),
        }
    }
}
