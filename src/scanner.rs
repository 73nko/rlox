use std::collections::HashMap;

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
    keywords: HashMap<String, TokenType>,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        let mut keywords = HashMap::new();

        Scanner {
            source: source.chars().collect(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            keywords,
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

    fn peek_next(&self) -> char {
        if let Some(ret) = self.source.get(self.current + 1) {
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
            '"' => self.string()?,
            d if d.is_ascii_digit() => self.number()?,
            i if i.is_ascii_alphabetic() || i == '_' => self.identifier()?,
            '/' => {
                if self.matches('/') {
                    self.consume_line_comment();
                } else {
                    self.add_token(TokenType::Slash);
                }
            }
            ' ' | '\r' | '\t' => {}
            '\n' => self.line += 1,
            _ => return Err(LoxError::error(self.line, "Unexpected token".to_string())),
        }

        Ok(())
    }

    fn value(&self) -> String {
        self.source[self.start..self.current].iter().collect()
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
        let s: String = self.value();
        self.tokens
            .push(Token::new(token_type, s, literal, self.line));
    }

    fn string(&mut self) -> Result<(), LoxError> {
        let mut escaped = false;

        while !self.is_at_end() {
            match self.advance() {
                '"' if !escaped => {
                    let value = self.source[self.start + 1..self.current - 1]
                        .iter()
                        .collect();
                    self.add_token_object(TokenType::String, Some(Object::Str(value)));
                    return Ok(());
                }
                '\n' => self.line += 1,
                '\\' if !escaped => escaped = true,
                _ => escaped = false,
            };
        }

        Err(LoxError::error(
            self.line,
            "Unterminated string".to_string(),
        ))
    }

    fn number(&mut self) -> Result<(), LoxError> {
        while char::is_ascii_digit(&self.peek()) {
            self.advance();
        }

        if self.peek() == '.' && char::is_ascii_digit(&self.peek_next()) {
            self.advance();
            while char::is_ascii_digit(&self.peek()) {
                self.advance();
            }
        }

        let number = self.value().parse::<f64>().unwrap();
        self.add_token_object(TokenType::Number, Some(Object::Num(number)));

        Ok(())
    }

    fn identifier(&mut self) -> Result<(), LoxError> {
        while char::is_ascii_alphanumeric(&self.peek()) {
            self.advance();
        }
        if let Some(typ) = TokenType::reserved(&self.value()) {
            self.add_token(typ);
        } else {
            self.add_token(TokenType::Identifier);
        }
        Ok(())
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
