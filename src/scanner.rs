use std::fs;
use crate::tokens::Token;
use crate::tokens::TokenType;

pub struct Scanner<'a> {
    input: &'a str,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

pub fn scan_from_file(source: &str) -> Result<Vec<Token>, String> {
    match fs::read_to_string(source) {
        Ok(content) => Scanner::new(content.as_str()).scan_tokens(),
        Err(_) => Err("Failed to read file!".to_string()),
    }
}

#[inline]
pub fn scan_from_input(input: &str) -> Result<Vec<Token>, String> {
    Scanner::new(input).scan_tokens()
}

impl<'a> Scanner<'a> {
    #[inline]
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> Result<Vec<Token>, String> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token()?;
        }

        self.tokens
            .push(Token::new(TokenType::Eof, String::new(), self.line));
        
        Ok(self.tokens.clone())
    }

    fn scan_token(&mut self) -> Result<(), String> {
        match self.advance() {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            ',' => self.add_token(TokenType::Comma),

            '+' => self.add_token(TokenType::Plus),
            '-' => self.add_token(TokenType::Minus),
            '*' => self.add_token(TokenType::Star),
            '/' => self.add_token(TokenType::Slash),
            '^' => self.add_token(TokenType::Carrot),
            '.' => self.number()?,

            '=' => {
                if self.get_current() == '=' {
                    self.advance();
                    self.add_token(TokenType::EqualEqual);
                } else {
                    return Err("Expected '=' to follow '=' character.".to_string());
                }
            }
            '<' => {
                if self.get_current() == '=' {
                    self.advance();
                    self.add_token(TokenType::LessThanEqual);
                } else {
                    self.add_token(TokenType::LessThan);
                }
            }
            '>' => {
                if self.get_current() == '=' {
                    self.advance();
                    self.add_token(TokenType::GreaterThanEqual);
                } else {
                    self.add_token(TokenType::GreaterThan);
                }
            }
            '!' => {
                if self.get_current() == '=' {
                    self.advance();
                    self.add_token(TokenType::NotEqual);
                } else {
                    self.add_token(TokenType::Not);
                }
            }
            '&' => {
                if self.get_current() == '&' {
                    self.advance();
                    self.add_token(TokenType::And);
                } else {
                    return Err("Expected '&' to follow '&' character.".to_string());
                }
            }
            '|' => {
                if self.get_current() == '|' {
                    self.advance();
                    self.add_token(TokenType::Or)
                } else {
                    return Err("Expected '|' to follow '|' character.".to_string());
                }
            }

            ' ' | '\r' | '\t' => {}
            '\n' => self.line += 1,
            char => {
                if char.is_digit(10) {
                    self.number()?;
                } else if char.is_alphabetic() {
                    self.identifier();
                } else {
                    return Err(format!(
                        "Unexpected character '{}' on line {}.",
                        char, self.line
                    ));
                }
            }
        }
        
        Ok(())
    }

    fn identifier(&mut self) {
        while self.get_current().is_alphabetic() {
            self.advance();
        }

        self.add_token(TokenType::Identifier(
            self.input[self.start..self.current].to_string()
        ))
    }

    fn number(&mut self) -> Result<(), String> {
        let mut had_decimal = false;

        while self.get_current().is_digit(10) || self.get_current() == '.' {
            if had_decimal && self.get_current() == '.' {
                return Err("Numbers may only have one decimal point.".to_string());
            }

            if self.get_current() == '.' {
                had_decimal = true;
                self.advance();
            }

            self.advance();
        }

        match self.input[self.start..self.current].parse::<f64>() {
            Ok(num) => {
                self.add_token(TokenType::Number(num));
                
                Ok(())
            }
            
            _ => Err("Something went wrong when scanning number token.".to_string()),
        }
    }

    fn add_token(&mut self, token_type: TokenType) {
        let text: &str = &self.input[self.start..self.current];
        self.tokens
            .push(Token::new(token_type, String::from(text), self.line));
    }

    fn get_current(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.input.chars().nth(self.current).unwrap()
    }

    fn advance(&mut self) -> char {
        let temp = self.input.chars().nth(self.current).unwrap();
        self.current += 1;
        temp
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.input.len()
    }
}
