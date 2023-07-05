use std::mem;

use crate::lexer::{Lexer, Token};
use crate::parser::{Expression, Identifier, LetStatement, Program, Statement};

pub struct Parser {
    pub lexer: Lexer,

    pub cur_token: Token,
    pub peek_token: Token,
}

impl Parser {
    pub fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token().unwrap();
    }

    pub fn new(lexer: Lexer) -> Parser {
        let mut p = Parser {
            lexer,
            cur_token: Token::Illegal,
            peek_token: Token::Illegal,
        };

        p.next_token();
        p.next_token();

        return p;
    }

    pub fn parse_progam(mut self) -> Program {
        let mut program = Program { statements: vec![] };

        while self.cur_token != Token::Eof {
            if let Some(stmt) = self.parse_statement() {
                program.statements.push(stmt);
            } else {
                panic!("Returned None from parse_statement")
            }

            self.next_token();
        }

        return program;
    }

    fn parse_statement(&mut self) -> Option<Statement> {
        match self.cur_token {
            Token::Let => self.parse_let_statement(),
            _ => None,
        }
    }

    fn expect_peek(&mut self, t: Token) -> bool {
        if mem::discriminant(&self.peek_token) == mem::discriminant(&t) {
            self.next_token();
            return true;
        }

        return false;
    }
}

impl Parser {
    fn parse_let_statement(&mut self) -> Option<Statement> {
        let token = self.cur_token.clone();

        if !self.expect_peek(Token::Ident("".to_string())) {
            return None;
        }

        let literal = self.cur_token.literal().to_owned();

        let name = Identifier {
            token: Token::Ident(literal),
        };

        if !self.expect_peek(Token::Assign) {
            return None;
        }

        while !(self.cur_token == Token::Semicolon) {
            self.next_token();
        }

        return Some(Statement::LetStatement(LetStatement {
            token,
            name,
            value: Expression::TestExp,
        }));
    }
}
