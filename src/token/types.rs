use crate::TokenType::{
    AND, CLASS, ELSE, FALSE, FOR, FUN, IF, NIL, OR, PRINT, RETURN, SUPER, THIS, TRUE, VAR, WHILE,
};
use std::collections::HashMap;

// TODO: implement the token struct
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone)]
pub enum TokenType {
    // Single-character tokens.
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

    // One or two character tokens.
    BANG,
    BANG_EQUAL,
    EQUAL,
    EQUAL_EQUAL,
    GREATER,
    GREATER_EQUAL,
    LESS,
    LESS_EQUAL,

    // Literals.
    IDENTIFIERS,
    STRING,
    NUMBER,

    // Keywords.
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

pub fn get_keywords_hashmap() -> HashMap<String, TokenType> {
    let mut keywords = HashMap::new();
    keywords.insert("and".to_string(), AND);
    keywords.insert("class".to_string(), CLASS);
    keywords.insert("else".to_string(), ELSE);
    keywords.insert("false".to_string(), FALSE);
    keywords.insert("for".to_string(), FOR);
    keywords.insert("fun".to_string(), FUN);
    keywords.insert("if".to_string(), IF);
    keywords.insert("nil".to_string(), NIL);
    keywords.insert("or".to_string(), OR);
    keywords.insert("print".to_string(), PRINT);
    keywords.insert("return".to_string(), RETURN);
    keywords.insert("super".to_string(), SUPER);
    keywords.insert("this".to_string(), THIS);
    keywords.insert("true".to_string(), TRUE);
    keywords.insert("var".to_string(), VAR);
    keywords.insert("while".to_string(), WHILE);

    keywords
}
