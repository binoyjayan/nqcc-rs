#[cfg(test)]
pub mod tests;
pub mod token;

use crate::scanner::token::*;

pub struct Scanner {
    source: Vec<char>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: &str) -> Self {
        Self {
            source: source.chars().collect::<Vec<char>>(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_token(&mut self) -> Token {
        self.skip_whitespace();
        self.start = self.current;
        if self.is_at_end() {
            return self.make_token(TokenType::Eof);
        }

        let c = self.advance();
        match c {
            '(' => self.make_token(TokenType::LeftParen),
            ')' => self.make_token(TokenType::RightParen),
            '{' => self.make_token(TokenType::LeftBrace),
            '}' => self.make_token(TokenType::RightBrace),
            ';' => self.make_token(TokenType::Semicolon),
            ',' => self.make_token(TokenType::Comma),
            '.' => self.make_token(TokenType::Dot),
            '-' => self.make_token(TokenType::Minus),
            '+' => self.make_token(TokenType::Plus),
            '/' => self.make_token(TokenType::Slash),
            '*' => self.make_token(TokenType::Star),
            '!' => {
                let ne = self.matches('=');
                self.make_token(if ne {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                })
            }
            '=' => {
                let eq = self.matches('=');
                self.make_token(if eq {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                })
            }
            '<' => {
                let lt = self.matches('=');
                self.make_token(if lt {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                })
            }
            '>' => {
                let gt = self.matches('=');
                self.make_token(if gt {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                })
            }
            '"' => self.string(),
            '0'..='9' => self.number(),
            _ if c.is_alphabetic() || c == '_' => self.identifier(),
            _ => self.error_token("Unexpected character", c),
        }
    }

    pub fn matches(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source[self.current] != expected {
            return false;
        }
        self.current += 1;
        true
    }

    fn skip_whitespace(&mut self) {
        loop {
            match self.peek() {
                ' ' | '\r' | '\t' => {
                    self.advance();
                }
                '\n' => {
                    self.line += 1;
                    self.advance();
                }
                '/' => {
                    match self.peek_next() {
                        '/' => {
                            // A single-line comment goes until end of the line
                            while self.peek() != '\n' && !self.is_at_end() {
                                self.advance();
                            }
                        }
                        '*' => {
                            // A multi-line comment goes until '*/'
                            self.advance(); // skip over '*'
                            while !(self.peek() == '*' && self.peek_next() == '/')
                                && !self.is_at_end()
                            {
                                self.advance();
                                if self.peek() == '\n' {
                                    self.line += 1;
                                }
                            }
                            if !self.is_at_end() {
                                self.advance(); // skip over '*'
                                self.advance(); // skip over '/'
                            }
                        }
                        _ => {
                            return;
                        }
                    }
                }
                _ => {
                    return;
                }
            }
        }
    }

    fn string(&mut self) -> Token {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }
        if self.is_at_end() {
            return self.error_message("Unterminated string.");
        }
        self.advance();
        self.make_token(TokenType::StringLiteral)
    }

    fn _number(&mut self) -> Token {
        while self.peek().is_ascii_digit() {
            self.advance();
        }
        // Look for fraction
        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            // consume '.'
            self.advance();
            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }
        self.make_token(TokenType::Integer)
    }

    fn number(&mut self) -> Token {
        while self.peek().is_ascii_digit() {
            self.advance();
        }
        // Look for fraction
        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            // consume '.'
            self.advance();
            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }
        let c = self.peek();
        if c.is_ascii_alphabetic() {
            return self.error_token("Unexpected character", c);
        }
        self.make_token(TokenType::Integer)
    }

    fn identifier(&mut self) -> Token {
        while self.peek_is_identifier() {
            self.advance();
        }
        let ttype = self.identifier_type();
        self.make_token(ttype)
    }

    fn peek_is_identifier(&self) -> bool {
        let c = self.peek();
        c.is_alphabetic() || c.is_ascii_digit() || c == '_'
    }

    fn identifier_type(&mut self) -> TokenType {
        match self.source[self.start] {
            'e' => return self.check_keyword(1, 3, "lse", TokenType::Else),
            'i' => {
                if self.current - self.start > 1 {
                    match self.source[self.start + 1] {
                        'n' => return self.check_keyword(2, 1, "t", TokenType::Int),
                        'f' => return TokenType::If,
                        _ => {}
                    }
                }
            }
            'f' => {
                if self.current - self.start > 1 {
                    match self.source[self.start + 1] {
                        'o' => return self.check_keyword(2, 1, "r", TokenType::For),
                        _ => {}
                    }
                }
            }
            'r' => return self.check_keyword(1, 5, "eturn", TokenType::Return),
            'w' => return self.check_keyword(1, 4, "hile", TokenType::While),
            'v' => return self.check_keyword(1, 3, "oid", TokenType::Void),
            _ => {}
        }
        TokenType::Identifier
    }

    fn check_keyword(
        &mut self,
        start: usize,
        length: usize,
        rest: &str,
        ttype: TokenType,
    ) -> TokenType {
        if self.current - self.start == start + length {
            let rest_str: String = self.source[self.start + start..self.current]
                .iter()
                .collect();
            if rest_str == rest {
                return ttype;
            }
        }
        TokenType::Identifier
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source[self.current]
        }
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            '\0'
        } else {
            self.source[self.current + 1]
        }
    }

    fn is_at_end(&self) -> bool {
        self.current == self.source.len()
    }

    fn make_token(&self, ttype: TokenType) -> Token {
        Token::new(
            ttype,
            self.source[self.start..self.current].iter().collect(),
            self.line,
        )
    }

    fn error_token(&self, message: &str, c: char) -> Token {
        Token {
            ttype: TokenType::Error,
            lexeme: format!("{} {}", message, c),
            line: self.line,
        }
    }

    fn error_message(&self, message: &str) -> Token {
        Token {
            ttype: TokenType::Error,
            lexeme: message.to_string(),
            line: self.line,
        }
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        self.source[self.current - 1]
    }
}
