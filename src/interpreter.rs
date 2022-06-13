use crate::{ast::{Node, Value}, tokens::TokenType};

pub fn evaluate(node: &Node) -> Result<Value, String> {
    match node {
        Node::Number(num) => Ok(Value::Number(*num)),
        Node::Boolean(val) => Ok(Value::Boolean(*val)),
        Node::UnaryExpr { operator, child } => {
            match evaluate(child)? {
                Value::Number(num) => match operator {
                    TokenType::Minus => Ok(Value::Number(-num)),
                    TokenType::Plus => Ok(Value::Number(num)),
                    other => Err(format!("Incompatible operand type (Number) for operation '{}'.", other))
                },
                Value::Boolean(val) => match operator {
                    TokenType::Not => Ok(Value::Boolean(!val)),
                    other => Err(format!("Incompatible operand type (Boolean) for operation '{}'.", other))
                }
            }
        }
        Node::BinaryExpr { operator, lhs, rhs } => {
            let rhs = evaluate(rhs)?;
            let lhs = evaluate(lhs)?;
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
                (rhs, lhs) => Err(format!("Left hand and right hand side of the expression are not of the same type. (Operating on '{:?}' and '{:?}')", rhs, lhs))
            }
        }
        Node::GroupingExpr { child } => evaluate(child),
    }
}