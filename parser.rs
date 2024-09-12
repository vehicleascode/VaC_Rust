use crate::lexer::{Lexer, Token, TokenType};

#[derive(Debug)]
pub enum Expr {
    NumberLiteral(i32),
    Variable(String),
    Binary(Box<Expr>, String, Box<Expr>),
}

#[derive(Debug)]
pub enum Stmt {
    Assignment(String, Expr),
    FunctionDeclaration(String, Vec<String>, Vec<Stmt>),
    If(Box<Expr>, Vec<Stmt>),
}

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    tokens: Vec<Token>,
    position: usize,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Self {
        let mut parser = Parser {
            lexer,
            tokens: Vec::new(),
            position: 0,
        };
        parser.tokens = parser.lexer.tokenize();
        parser
    }

    pub fn parse(&mut self) -> Vec<Stmt> {
        let mut stmts = Vec::new();
        while self.current_token().token_type != TokenType::EOF {
            stmts.push(self.parse_statement());
        }
        stmts
    }

    fn parse_statement(&mut self) -> Stmt {
        match self.current_token().token_type {
            TokenType::Keyword if self.current_token().value == "function" => self.parse_function_declaration(),
            TokenType::Keyword if self.current_token().value == "if" => self.parse_if_statement(),
            TokenType::Identifier => self.parse_assignment(),
            _ => panic!("Unexpected token: {:?}", self.current_token()),
        }
    }

    fn parse_function_declaration(&mut self) -> Stmt {
        self.consume_token(); // 'function'
        let name = self.consume_token().value;
        self.consume_token(); // '('
        let params = self.parse_parameter_list();
        self.consume_token(); // ')'
        self.consume_token(); // '{'
        let body = self.parse_statement_list();
        self.consume_token(); // '}'
        Stmt::FunctionDeclaration(name, params, body)
    }

    fn parse_parameter_list(&mut self) -> Vec<String> {
        let mut params = Vec::new();
        while self.current_token().token_type != TokenType::Delimiter || self.current_token().value != ")" {
            if self.current_token().token_type == TokenType::Identifier {
                params.push(self.consume_token().value);
            }
            if self.current_token().token_type == TokenType::Delimiter && self.current_token().value == "," {
                self.consume_token();
            }
        }
        params
    }

    fn parse_if_statement(&mut self) -> Stmt {
        self.consume_token(); // 'if'
        self.consume_token(); // '('
        let condition = self.parse_expression();
        self.consume_token(); // ')'
        self.consume_token(); // '{'
        let body = self.parse_statement_list();
        self.consume_token(); // '}'
        Stmt::If(Box::new(condition), body)
    }

    fn parse_assignment(&mut self) -> Stmt {
        let var_name = self.consume_token().value;
        self.consume_token(); // '='
        let value = self.parse_expression();
        self.consume_token(); // ';'
        Stmt::Assignment(var_name, value)
    }

    fn parse_expression(&mut self) -> Expr {
        let left = self.parse_term();
        if self.current_token().token_type == TokenType::Operator {
            let op = self.consume_token().value;
            let right = self.parse_expression();
            Expr::Binary(Box::new(left), op, Box::new(right))
        } else {
            left
        }
    }

    fn parse_term(&mut self) -> Expr {
        match self.current_token().token_type {
            TokenType::Number => {
                let value = self.consume_token().value.parse().unwrap();
                Expr::NumberLiteral(value)
            },
            TokenType::Identifier => {
                let value = self.consume_token().value;
                Expr::Variable(value)
            },
            TokenType::Delimiter if self.current_token().value == "(" => {
                self.consume_token(); // '('
                let expr = self.parse_expression();
                self.consume_token(); // ')'
                expr
            },
            _ => panic!("Unexpected token: {:?}", self.current_token()),
        }
    }

    fn current_token(&self) -> &Token {
        &self.tokens[self.position]
    }

    fn consume_token(&mut self) -> Token {
        let token = self.current_token().clone();
        self.position += 1;
        token
    }
}
