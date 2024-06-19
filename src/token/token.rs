use crate::TokenType;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum Object {
    Null,
}

#[derive(Debug)]
pub struct Token {
    pub token_types: TokenType,
    pub lexeme: String,
    pub literal: Object,
    pub line: usize,
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl Token {
    pub fn new(token_types: TokenType, lexeme: String, literal: Object, line: usize) -> Self {
        Token {
            token_types,
            lexeme,
            literal,
            line,
        }
    }

    pub fn to_string(&self) -> String {
        format!("{:?} {} {:?}", self.token_types, self.lexeme, self.literal)
    }
}
