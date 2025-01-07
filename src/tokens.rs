use std::cell::LazyCell;
use std::collections::HashMap;
use TokenType::*;

pub const KEYWORDS: LazyCell<HashMap<&str, TokenType>> = LazyCell::new(|| {
    let mut m = HashMap::new();
    m.insert("structure", Structure);
    m.insert("styles", Styles);
    m.insert("group", Group);
    m.insert("px", Px);
    m
});

#[derive(Debug, Clone)]
pub struct Token {
    pub tokentype: TokenType,
    pub lexeme: String,
    pub line: usize,
}

impl Token {
    pub(crate) fn new(tokentype: TokenType, lexeme: String, line: usize) -> Self {
        Token {
            tokentype,
            lexeme,
            line,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TokenType {
    ArrowRight,
    ArrowLeft,
    DiamondArrowRight,
    DiamondArrowLeft,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Colon,
    Slash,
    Identifier,
    Str,
    Number,
    Minus,
    Structure,
    Styles,
    Group,
    Eof,
    Px,
}
