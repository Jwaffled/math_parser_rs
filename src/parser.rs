use crate::ast::Node;
use crate::scanner;
use crate::tokens::Token;
use crate::tokens::TokenType;
use std::iter::IntoIterator;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

pub fn parse_from_file(source: &str) -> Result<Node, String> {
    Parser::new(scanner::scan_from_file(source)?).parse()
}

pub fn parse_from_input(input: &str) -> Result<Node, String> {
    Parser::new(scanner::scan_from_input(input)?).parse()
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<Node, String> {
        self.parse_expression()
    }

    fn parse_expression(&mut self) -> Result<Node, String> {
        self.parse_logic_binary()
    }

    fn parse_logic_binary(&mut self) -> Result<Node, String> {
        let mut expr = self.parse_term()?;

        while self.match_token([
            TokenType::GreaterThan,
            TokenType::GreaterThanEqual,
            TokenType::LessThan,
            TokenType::LessThanEqual,
            TokenType::EqualEqual,
            TokenType::NotEqual,
            TokenType::Or,
            TokenType::And
        ]) {
            let operator = self.previous().token_type;
            let right = self.parse_term()?;
            expr = Node::BinaryExpr {
                operator,
                rhs: Box::new(right),
                lhs: Box::new(expr),
            }
        }

        Ok(expr)
    }

    fn parse_term(&mut self) -> Result<Node, String> {
        let mut expr = self.parse_factor()?;

        while self.match_token([TokenType::Plus, TokenType::Minus]) {
            let operator = self.previous().token_type;
            let right = self.parse_factor()?;
            expr = Node::BinaryExpr {
                operator,
                rhs: Box::new(right),
                lhs: Box::new(expr),
            }
        }

        Ok(expr)
    }

    fn parse_factor(&mut self) -> Result<Node, String> {
        let mut expr = self.parse_power()?;

        while self.match_token([TokenType::Star, TokenType::Slash]) {
            let operator = self.previous().token_type;
            let right = self.parse_power()?;
            expr = Node::BinaryExpr {
                operator,
                rhs: Box::new(right),
                lhs: Box::new(expr),
            }
        }

        Ok(expr)
    }

    fn parse_power(&mut self) -> Result<Node, String> {
        let mut expr = self.parse_unary()?;

        while self.match_token([TokenType::Carrot]) {
            let operator = self.previous().token_type;
            let right = self.parse_unary()?;
            expr = Node::BinaryExpr {
                operator,
                rhs: Box::new(right),
                lhs: Box::new(expr),
            }
        }

        Ok(expr)
    }

    fn parse_unary(&mut self) -> Result<Node, String> {
        while self.match_token([TokenType::Minus, TokenType::Not]) {
            let operator = self.previous().token_type;
            let expr = self.parse_unary()?;
            return Ok(Node::UnaryExpr {
                operator,
                child: Box::new(expr),
            });
        }

        self.parse_primary()
    }

    fn parse_primary(&mut self) -> Result<Node, String> {
        if !self.is_at_end() {
            match self.get_current().token_type {
                TokenType::Number(num) => {
                    self.advance();
                    return Ok(Node::Number(num));
                }
                TokenType::Identifier(val) => {
                    self.advance();
                    // This should probably be the job of the scanner, but I can't be bothered!
                    if val == "true" {
                        return Ok(Node::Boolean(true));
                    } else if val == "false" {
                        return Ok(Node::Boolean(false));
                    } else {
                        return Err(format!("Identifiers are not supported yet. '{}' on line {}.", val, self.previous().line))
                    }
                }
                TokenType::LeftParen => {
                    self.advance();
                    let expr = self.parse_expression()?;
                    self.consume_token(
                        TokenType::RightParen,
                        "Expected ')' after grouping expression.".to_string(),
                    )?;
                    return Ok(Node::GroupingExpr {
                        child: Box::new(expr),
                    });
                }
                _ => {}
            }
        }

        Err(format!(
            "Unexpected token '{}' at line {}",
            self.get_current().token_type,
            self.get_current().line
        ))
    }

    fn match_token<T: IntoIterator<Item = TokenType>>(&mut self, types: T) -> bool {
        for token_type in types.into_iter() {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        token_type == self.get_current().token_type
    }

    #[inline]
    fn is_at_end(&self) -> bool {
        self.get_current().token_type == TokenType::Eof
    }

    fn get_current(&self) -> Token {
        self.tokens.get(self.current).unwrap().clone()
    }

    fn consume_token(&mut self, token_type: TokenType, message: String) -> Result<Token, String> {
        if self.check(token_type) {
            return Ok(self.advance());
        }

        Err(message)
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn previous(&self) -> Token {
        self.tokens.get(self.current - 1).unwrap().clone()
    }
}
