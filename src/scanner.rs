use crate::{
    error::LoxError,
    token::{Object, Token},
    token_type::*,
};

#[derive(Debug)]
pub struct Scanner {
    source: Vec<char>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Scanner {
            source: source.chars().collect(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> Result<&Vec<Token>, LoxError> {
        let mut had_error: Option<LoxError> = None;
        while !self.is_at_end() {
            self.start = self.current;
            match self.scan_token() {
                Ok(token) => {}
                Err(e) => {
                    let token = self.source.get(self.current - 1).unwrap();
                    e.report(token.to_string());

                    had_error = Some(e);
                }
            }
        }

        self.tokens.push(Token::eof(self.line));

        if let Some(e) = had_error {
            Err(e)
        } else {
            Ok(&self.tokens)
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn peek(&self) -> char {
        if let Some(ret) = self.source.get(self.current) {
            *ret
        } else {
            '\0'
        }
    }

    fn consume_line_comment(&mut self) {
        while !self.is_at_end() && self.peek() != '\n' {
            self.advance();
        }
    }

    fn scan_token(&mut self) -> Result<(), LoxError> {
        match self.advance() {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ';' => self.add_token(TokenType::Semicolon),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            '*' => self.add_token(TokenType::Star),
            '!' => self.matches_or('=', TokenType::BangEqual, TokenType::Bang),
            '=' => self.matches_or('=', TokenType::EqualEqual, TokenType::Equal),
            '<' => self.matches_or('=', TokenType::LessEqual, TokenType::Less),
            '>' => self.matches_or('=', TokenType::GreaterEqual, TokenType::Greater),
            '/' => {
                if self.matches('/') {
                    self.consume_line_comment();
                } else {
                    self.add_token(TokenType::Slash);
                }
            }
            ' ' => {}
            _ => return Err(LoxError::error(self.line, "Unexpected token".to_string())),
        }

        Ok(())
    }

    fn advance(&mut self) -> char {
        let result = self.source.get(self.current).unwrap();
        self.current += 1;
        *result
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.add_token_object(token_type, None);
    }

    fn add_token_object(&mut self, token_type: TokenType, literal: Option<Object>) {
        let s: String = self.source[self.start..self.current].iter().collect();
        self.tokens.push(Token::new(token_type, s, literal));
    }

    fn matches(&mut self, c: char) -> bool {
        match self.source.get(self.current) {
            Some(ch) if *ch == c => {
                self.current += 1;
                true
            }
            _ => false,
        }
    }

    fn matches_or(&mut self, c: char, ok: TokenType, or: TokenType) {
        let token = if self.matches(c) { ok } else { or };
        self.add_token(token);
    }
}
