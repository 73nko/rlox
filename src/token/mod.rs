use std::{
    cmp,
    fmt::{self, Display},
};

#[derive(Debug, Clone, PartialEq)]
pub enum Token_Type {
    // Single character token
    LEFT_PAREN,
    RIGHT_PAREN,
    LEFT_BRACE,
    RIGHT_BRACE,
    COMMA,
    DOT,
    MINUS,
    PLUS,
    SEMICOLON,
    SLASH,
    STAR,

    // One or two character tokens
    BANG,
    BANG_EQUAL,
    EQUAL,
    EQUAL_EQUAL,
    GREATER,
    GREATER_EQUAL,
    LESS,
    LESS_EQUAL,

    // Literals
    IDENTIFIER,
    STRING,
    NUMBER,

    // Keywords
    AND,
    CLASS,
    ELSE,
    FALSE,
    FUN,
    FOR,
    IF,
    NIL,
    OR,
    PRINT,
    RETURN,
    SUPER,
    THIS,
    TRUE,
    VAR,
    WHILE,

    EOF,
}

impl fmt::Display for Token_Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", format!("{:?}", self).to_lowercase())
    }
}

/// A region of source code with a start and an end
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl Span {
    /// Create and return a new span from a start and end index
    pub fn new(start: usize, end: usize) -> Self {
        Span { start, end }
    }
    /// Create and return a new span that minimally covers all of the spans in `spans`
    pub fn merge(spans: Vec<&Span>) -> Self {
        let mut start = usize::max_value();
        let mut end = 0;
        for span in spans {
            start = cmp::min(start, span.start);
            end = cmp::max(end, span.end);
        }
        Span::new(start, end)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub token_type: Token_Type,
    pub lexeme: String,
    pub literal: String,
    pub line: usize,
}

impl Token {
    pub fn new(token_type: Token_Type, lexeme: String, literal: String, line: usize) -> Self {
        Token {
            token_type,
            lexeme,
            literal,
            line,
        }
    }

    pub fn to_string(&self) -> String {
        format!("{} {} {:?}", self.token_type, self.lexeme, self.literal)
    }
}
