use std::fmt;

#[derive(Debug, Clone)]
pub struct Token {
    pub ttype: TokenType,
    pub lexeme: String,
    pub line: usize,
}

impl Token {
    pub fn new(ttype: TokenType, lexeme: String, line: usize) -> Self {
        Self {
            ttype,
            lexeme: lexeme.to_string(),
            line,
        }
    }
}

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen = 0,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier,
    StringLiteral,
    Integer,

    // Keywords.
    Void,
    Int,
    Else,
    For,
    If,
    Return,
    While,
    Error,
    Eof,
}

impl From<TokenType> for &'static str {
    fn from(ttype: TokenType) -> &'static str {
        match ttype {
            TokenType::LeftParen => "(",
            TokenType::RightParen => ")",
            TokenType::LeftBrace => "{",
            TokenType::RightBrace => "}",
            TokenType::Comma => ",",
            TokenType::Dot => ".",
            TokenType::Minus => "-",
            TokenType::Plus => "+",
            TokenType::Semicolon => ";",
            TokenType::Slash => "/",
            TokenType::Star => "*",
            TokenType::Bang => "!",
            TokenType::BangEqual => "!=",
            TokenType::Equal => "=",
            TokenType::EqualEqual => "==",
            TokenType::Greater => ">",
            TokenType::GreaterEqual => ">=",
            TokenType::Less => "<",
            TokenType::LessEqual => "<=",
            TokenType::Identifier => "identifier",
            TokenType::StringLiteral => "string",
            TokenType::Integer => "integer",
            TokenType::Void => "void",
            TokenType::Int => "int",
            TokenType::Else => "else",
            TokenType::For => "for",
            TokenType::If => "if",
            TokenType::Return => "return",
            TokenType::While => "while",
            TokenType::Error => "error",
            TokenType::Eof => "eof",
        }
    }
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", <&'static str>::from(*self))
    }
}
