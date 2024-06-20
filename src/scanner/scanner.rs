use crate::{Lox, Object, Token, TokenType};

pub struct Scanner<'a> {
    source: &'a str,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    interpreter: &'a mut Lox,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str, interpreter: &'a mut Lox) -> Scanner<'a> {
        Scanner {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 0,
            interpreter,
        }
    }

    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        while !self.is_at_end() {
            // the beginning of the next lexeme.
            self.start = self.current;
            self.scan_token()
        }
        let token = Token::new(TokenType::EOF, "".to_string(), Object::Null, 0);
        self.tokens.push(token);
        // this manipulation is necessary for now to return a copy of the token
        // TODO: make this cleaner
        &self.tokens
    }

    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            '(' => self.add_token(TokenType::LEFT_PAREN),
            ')' => self.add_token(TokenType::RIGHT_PAREN),
            '{' => self.add_token(TokenType::LEFT_BRACE),
            '}' => self.add_token(TokenType::RIGHT_BRACE),
            ',' => self.add_token(TokenType::COMMA),
            '.' => self.add_token(TokenType::DOT),
            '-' => self.add_token(TokenType::MINUS),
            '+' => self.add_token(TokenType::PLUS),
            ';' => self.add_token(TokenType::SEMICOLON),
            '*' => self.add_token(TokenType::STAR),
            '!' => {
                if self.match_char('=') {
                    self.add_token(TokenType::BANG_EQUAL)
                } else {
                    self.add_token(TokenType::BANG)
                }
            }
            '=' => {
                if self.match_char('=') {
                    self.add_token(TokenType::EQUAL_EQUAL)
                } else {
                    self.add_token(TokenType::EQUAL)
                }
            }
            '<' => {
                if self.match_char('=') {
                    self.add_token(TokenType::LESS_EQUAL)
                } else {
                    self.add_token(TokenType::LESS)
                }
            }
            '>' => {
                if self.match_char('=') {
                    self.add_token(TokenType::GREATER_EQUAL)
                } else {
                    self.add_token(TokenType::GREATER)
                }
            }
            '/' => {
                if self.match_char('/') {
                    while self.peak() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::SLASH)
                }
            }
            ' ' => {}
            '\r' => {}
            '\t' => {}
            '\n' => {
                self.line += 1;
            }
            '"' => self.string(),
            _ => {
                if self.is_digit(c) {
                    self.number();
                } else {
                    self.interpreter
                        .error(self.line as i32, "Unexpected character".to_string());
                }
            }
        }
    }

    fn string(&mut self) {
        while self.peak() != '"' && !self.is_at_end() {
            if self.peak() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            self.interpreter
                .error(self.line as i32, "Unterminated string.".to_string());
        }

        // The closing ".
        self.advance();

        // Trim the surrounding quotes.
        let value = self
            .source
            .get(self.start + 1..self.current - 1)
            .unwrap()
            .to_string();
        self.add_token_with_literal(TokenType::STRING, Object::String(value))
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        // This should not panic because of the guard in the `is_at_end()`
        if self.source.chars().nth(self.current).unwrap() != expected {
            return false;
        }

        self.current += 1;
        true
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&self) -> char {
        // There is some guarantee in the code that current with overflow
        // This guarantee is in the is_at_end() function.
        self.source.chars().nth(self.current + 1).unwrap()
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.add_token_with_literal(token_type, Object::Null)
    }

    fn add_token_with_literal(&mut self, token_type: TokenType, literal: Object) {
        // TODO: better error handling here
        // This should not panic because of the safe guard in the is_at_end() function
        let text = self.source.get(self.start..self.current).unwrap();
        let text = String::from(text);
        self.tokens
            .push(Token::new(token_type, text, literal, self.line))
    }

    // Move the cursor to the end of the line, if it is at the end, send an end line character
    fn peak(&self) -> char {
        if self.is_at_end() {
            return '\n';
        }

        self.source.chars().nth(self.current).unwrap()
    }

    // A function to look two characters ahead.
    fn peak_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }

        self.source.chars().nth(self.current + 1).unwrap()
    }
    fn is_digit(&self, c: char) -> bool {
        // Ideally, this should only allow arabic numerals
        c.is_ascii_digit()
    }
    fn number(&mut self) {
        while self.is_digit(self.peak()) {
            self.advance();
        }

        // Look for a fractional part.
        if self.peak() == '.' && self.is_digit(self.peak_next()) {
            // consume the decimal point
            self.advance();

            while self.is_digit(self.peak()) {
                self.advance();
            }
        }

        let num = self
            .source
            .get(self.start..self.current)
            .unwrap()
            .parse::<f64>()
            .unwrap();

        self.add_token_with_literal(TokenType::NUMBER, Object::Double(num));
    }
}
