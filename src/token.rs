use std::fmt;

use crate::token_type::TokenType;

#[derive(Debug)]
pub enum Object {
    Num(f64),
    Str(String),
    Nil,
    True,
    False,
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Object::Num(x) => write!(f, "{x}"),
            Object::Str(x) => write!(f, "\"{x}\""),
            Nil => write!(f, "nil"),
            True => write!(f, "true"),
            False => write!(f, "false"),
        }
    }
}

#[derive(Debug)]
pub struct Token {
    ttype: TokenType,
    lexeme: String,
    literal: Option<Object>,
    line: usize,
}

impl Token {
    pub fn new(ttype: TokenType, lexeme: String, literal: Option<Object>, line: usize) -> Token {
        Token {
            ttype,
            lexeme,
            literal,
            line,
        }
    }

    pub fn eof(line: usize) -> Token {
        Token {
            ttype: TokenType::Eof,
            lexeme: "".to_string(),
            literal: None,
            line,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:?}{}{}",
            self.ttype,
            self.lexeme,
            if let Some(literal) = &self.literal {
                literal.to_string()
            } else {
                "None".to_string()
            },
        )
    }
}

/*


pub enum Token {
    Literal { lexeme: String, literal: String },
    Keyword { lexeme: String, ttype: TokenType },
}

*/
