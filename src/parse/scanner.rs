use unicode_segmentation::UnicodeSegmentation;

use crate::parse::tokens::{
    Token,
    TokenType::{self, *},
    KEYWORDS,
};

pub fn scan(vis: &str) -> anyhow::Result<Vec<Token>> {
    let mut scanner = Scanner::new(vis);
    scanner.scan();
    Ok(scanner.tokens)
}

struct Scanner<'a> {
    tokens: Vec<Token>,
    chars: Vec<&'a str>,
    current_pos: usize,
    current_line: usize,
    start_pos: usize,
}
impl<'a> Scanner<'a> {
    fn new(vis: &'a str) -> Self {
        Self {
            tokens: vec![],
            chars: UnicodeSegmentation::graphemes(vis, true).collect::<Vec<&str>>(),
            current_pos: 0,
            current_line: 0,
            start_pos: 0,
        }
    }

    fn scan(&mut self) {
        while !self.is_at_end() {
            self.start_pos = self.current_pos;
            self.scan_token();
        }
        self.add_token(Eof);
    }

    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            "(" => self.add_token(LeftParen),
            ")" => self.add_token(RightParen),
            "{" => self.add_token(LeftBrace),
            "}" => self.add_token(RightBrace),
            "," => self.add_token(Comma),
            "." => self.add_token(Dot),

            "-" => {
                if self.match_token("-") {
                    if self.match_token(">") {
                        self.add_token(ArrowRight);
                    } else if self.match_token("<") {
                        if self.match_token(">") {
                            self.add_token(DiamondArrowRight);
                        } else {
                            println!("Wrong arrow at {}", self.current_line);
                        }
                    } else {
                        println!("Wrong arrow at {}", self.current_line);
                    }
                } else {
                    self.add_token(Minus);
                }
            }
            "<" => {
                if self.match_token("-") {
                    if self.match_token("-") {
                        self.add_token(ArrowLeft);
                    } else {
                        println!("Wrong arrow at {}", self.current_line);
                    }
                } else if self.match_token(">") {
                    if self.match_token("-") {
                        if self.match_token("-") {
                            self.add_token(DiamondArrowLeft);
                        } else {
                            println!("Wrong arrow at {}", self.current_line);
                        }
                    } else {
                        println!("Wrong arrow at {}", self.current_line);
                    }
                }
            }
            ":" => self.add_token(Colon),
            "/" => {
                if self.match_token("/") {
                    while self.peek() != "\n" && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(Slash);
                }
            }
            " " | "\t" | "\r" => {}
            "\n" => {
                self.current_line += 1;
            }
            "\"" => self.string(),
            _ => {
                if is_digit(c) || c == "." {
                    self.number()
                } else if is_alpha(c) {
                    self.identifier()
                }
            }
        }
    }

    fn peek(&self) -> &str {
        if self.current_pos < self.chars.len() {
            self.chars[self.current_pos]
        } else {
            ""
        }
    }

    fn peek_next(&self) -> &str {
        self.chars[self.current_pos + 1]
    }

    fn identifier(&mut self) {
        while is_alpha(self.peek()) || is_digit(self.peek()) {
            self.advance();
        }
        let text = self.chars[self.start_pos..self.current_pos].concat();
        let tokentype = KEYWORDS
            .get(text.as_str())
            .map(|d| *d)
            .unwrap_or(Identifier);

        self.add_token(tokentype);
    }

    fn number(&mut self) {
        while is_digit(self.peek()) || (self.peek() == "." && is_digit(self.peek_next())) {
            self.advance();
        }

        self.add_token(TokenType::Number);
    }

    fn string(&mut self) {
        while self.peek() != "\"" && !self.is_at_end() {
            if self.peek() == "\n" {
                self.current_line = 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            println!("Unterminated string");
            return;
        }

        self.advance();

        self.add_token(TokenType::Str);
    }

    fn add_token(&mut self, tokentype: TokenType) {
        // println!("add token {:?}", tokentype);
        let token = Token::new(
            tokentype,
            self.chars[self.start_pos..self.current_pos].concat(),
            self.current_line,
        );
        self.tokens.push(token);
    }

    fn match_token(&mut self, expected: &str) -> bool {
        if self.is_at_end() || self.peek() != expected {
            return false;
        }
        self.current_pos += 1;
        return true;
    }

    fn advance(&mut self) -> &str {
        let c = self.chars[self.current_pos];
        if self.current_pos < self.chars.len() {
            self.current_pos += 1;
        }
        c
    }

    fn is_at_end(&self) -> bool {
        self.current_pos >= self.chars.len()
    }
}

fn is_digit(char: &str) -> bool {
    char.len() > 0 && char.chars().next().unwrap().is_ascii_digit()
}

fn is_alpha(char: &str) -> bool {
    if char.len() == 0 {
        false
    } else {
        let char = char.chars().next().unwrap();
        if char.is_alphabetic() || char == '_' {
            true
        } else {
            false
        }
    }
}
