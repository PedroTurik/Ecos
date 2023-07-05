use anyhow::{Ok, Result};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Token {
    Ident(String),
    Int(String),
    Assign,
    Plus,
    Comma,
    Semicolon,
    LParen,
    RParen,
    LBrace,
    RBrace,
    Function,
    Let,
    Eof,
    Illegal,
}

impl Token {
    pub fn literal(&self) -> &str {
        match self {
            Token::Ident(x) => x,
            Token::Int(x) => x,
            Token::Assign => "=",
            Token::Plus => "+",
            Token::Comma => ",",
            Token::Semicolon => ";",
            Token::LParen => "(",
            Token::RParen => ",",
            Token::LBrace => ",",
            Token::RBrace => ",",
            Token::Function => ",",
            Token::Let => "let",
            Token::Eof => "EOF",
            Token::Illegal => unreachable!(),
        }
    }
}

pub struct Lexer {
    input: Vec<u8>,
    position: usize,
    read_position: usize,
    ch: u8,
}

impl Lexer {
    pub fn new(program: String) -> Lexer {
        let mut lexer = Lexer {
            input: program.into_bytes(),
            position: 0,
            read_position: 0,
            ch: 0,
        };

        lexer.read_char();

        return lexer;
    }

    fn skip_whitespace(&mut self) {
        while self.ch.is_ascii_whitespace() {
            self.read_char();
        }
    }

    fn read_ident(&mut self) -> String {
        let initial_pos = self.position;

        while self.input[self.read_position].is_ascii_alphabetic()
            || self.input[self.read_position] == b'_'
        {
            self.read_char();
        }

        String::from_utf8_lossy(&self.input[initial_pos..=self.position]).to_string()
    }

    fn read_int(&mut self) -> String {
        let initial_pos = self.position;

        while self.input[self.read_position].is_ascii_digit() {
            self.read_char();
        }

        String::from_utf8_lossy(&self.input[initial_pos..=self.position]).to_string()
    }

    pub fn next_token(&mut self) -> Result<Token> {
        self.skip_whitespace();

        let token = match self.ch {
            b'+' => Token::Plus,
            b'=' => Token::Assign,
            b'(' => Token::LParen,
            b')' => Token::RParen,
            b'{' => Token::LBrace,
            b'}' => Token::RBrace,
            b',' => Token::Comma,
            b';' => Token::Semicolon,
            0 => Token::Eof,
            b'a'..=b'z' | b'A'..=b'Z' | b'_' => match self.read_ident().as_str() {
                "let" => Token::Let,
                "fn" => Token::Function,
                id => Token::Ident(id.to_string()),
            },
            b'0'..=b'9' => Token::Int(self.read_int()),

            _ => Token::Illegal,
        };

        self.read_char();

        return Ok(token);
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = 0;
        } else {
            self.ch = self.input[self.read_position];
        }

        self.position = self.read_position;
        self.read_position += 1;
    }
}
