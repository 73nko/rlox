#[path = "../token/mod.rs"]
mod token;

use token::{Token, Token_Type};

#[derive(Debug, Clone, PartialEq)]
pub struct Scanner {
    pub source: String,
    pub tokens: Vec<Token>,
    pub start: usize,
    pub current: usize,
    pub line: usize,
}

impl Scanner {
    pub fn new(source: String) -> Scanner {
        Scanner {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> &mut Vec<Token> {
        while (!self.is_EOF()) {
            self.start = self.current;
            self.scan_token();
        }
        self.tokens.push(Token::new(
            Token_Type::EOF,
            String::from(""),
            String::from(""),
            self.line,
        ));

        &mut self.tokens
    }

    fn scan_token(&mut self) {}

    fn is_EOF(&self) -> bool {
        self.current == self.source.len()
    }
}
