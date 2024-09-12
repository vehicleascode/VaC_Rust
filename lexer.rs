use regex::Regex;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum TokenType {
    Keyword,
    Identifier,
    Number,
    Operator,
    Delimiter,
    EOF,
}

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub value: String,
}

pub struct Lexer<'a> {
    input: &'a str,
    position: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer { input, position: 0 }
    }

    fn next_char(&self) -> Option<char> {
        self.input[self.position..].chars().next()
    }

    fn consume_char(&mut self) {
        self.position += self.next_char().map(|c| c.len_utf8()).unwrap_or(0);
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        while self.position < self.input.len() {
            let c = self.next_char().unwrap();
            match c {
                ' ' | '\t' | '\n' => {
                    self.consume_char();
                }
                '0'..='9' => tokens.push(self.tokenize_number()),
                'a'..='z' | 'A'..='Z' => tokens.push(self.tokenize_identifier()),
                '+' | '-' | '*' | '/' => tokens.push(self.tokenize_operator(c)),
                '(' | ')' | '{' | '}' | ',' | ';' => tokens.push(self.tokenize_delimiter(c)),
                _ => panic!("Unexpected character: {}", c),
            }
        }
        tokens.push(Token { token_type: TokenType::EOF, value: String::new() });
        tokens
    }

    fn tokenize_number(&mut self) -> Token {
        let start = self.position;
        while self.next_char().map(|c| c.is_digit(10)).unwrap_or(false) {
            self.consume_char();
        }
        Token {
            token_type: TokenType::Number,
            value: self.input[start..self.position].to_string(),
        }
    }

    fn tokenize_identifier(&mut self) -> Token {
        let start = self.position;
        while self.next_char().map(|c| c.is_alphanumeric()).unwrap_or(false) {
            self.consume_char();
        }
        let value = self.input[start..self.position].to_string();
        if value == "function" || value == "if" {
            Token {
                token_type: TokenType::Keyword,
                value,
            }
        } else {
            Token {
                token_type: TokenType::Identifier,
                value,
            }
        }
    }

    fn tokenize_operator(&mut self, c: char) -> Token {
        self.consume_char();
        Token {
            token_type: TokenType::Operator,
            value: c.to_string(),
        }
    }

    fn tokenize_delimiter(&mut self, c: char) -> Token {
        self.consume_char();
        Token {
            token_type: TokenType::Delimiter,
            value: c.to_string(),
        }
    }
}
