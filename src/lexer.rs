// src/lexer.rs
pub struct Lexer {
    input: Vec<char>,
    position: usize,
    current_char: Option<char>,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        let mut lexer = Lexer {
            input: input.chars().collect(),
            position: 0,
            current_char: None,
        };
        lexer.read_char();
        lexer
    }

    fn read_char(&mut self) {
        if self.position >= self.input.len() {
            self.current_char = None;
        } else {
            self.current_char = Some(self.input[self.position]);
        }
        self.position += 1;
    }

    pub fn next_token(&mut self) -> Option<Token> {
        while let Some(c) = self.current_char {
            match c {
                ' ' | '\t' | '\n' | '\r' => self.read_char(),
                '+' => {
                    self.read_char();
                    return Some(Token::Plus);
                }
                '-' => {
                    self.read_char();
                    return Some(Token::Minus);
                }
                // Handle more cases for different tokens
                _ => {
                    if c.is_alphabetic() {
                        return Some(self.read_identifier());
                    } else if c.is_numeric() {
                        return Some(self.read_number());
                    } else {
                        self.read_char();
                        return None;
                    }
                }
            }
        }
        None
    }

    fn read_identifier(&mut self) -> Token {
        let start = self.position - 1;
        while let Some(c) = self.current_char {
            if c.is_alphanumeric() {
                self.read_char();
            } else {
                break;
            }
        }
        Token::Identifier(self.input[start..self.position - 1].iter().collect())
    }

    fn read_number(&mut self) -> Token {
        let start = self.position - 1;
        while let Some(c) = self.current_char {
            if c.is_numeric() {
                self.read_char();
            } else {
                break;
            }
        }
        Token::Number(self.input[start..self.position - 1].iter().collect())
    }
}

pub enum Token {
    Identifier(String),
    Number(String),
    Plus,
    Minus,
    // Add more token types as needed
}
