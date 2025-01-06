use crate::{
    tokens::{
        Token,
        TokenType::{self, *},
    },
    Element, StyleNode, Vis,
};
use anyhow::anyhow;

pub fn parse_vis(contents: &str) -> anyhow::Result<Vis> {
    let tokens = crate::scanner::scan(contents)?;
    println!("{:?}", tokens);
    let mut parser = Parser::new(tokens);

    Ok(Vis {
        markup: parser.markup()?,
        styles: parser.styles()?,
    })
}

struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    fn markup(&mut self) -> anyhow::Result<Vec<Element>> {
        if self.match_token(Markup) {
            self.nodes()
        } else {
            Ok(vec![])
        }
    }

    fn nodes(&mut self) -> anyhow::Result<Vec<Element>> {
        println!("nodes {:?}", self.peek());
        self.consume(LeftBrace, "Expected '{'")?;
        let mut nodes = vec![];
        while !self.match_token(RightBrace) {
            nodes.push(self.node()?);
        }
        Ok(nodes)
    }

    fn node(&mut self) -> anyhow::Result<Element> {
        println!("node {:?}", self.peek());
        let id = self.id()?;
        println!("id {}", id);
        let current = self.peek().clone();
        if self.match_tokens(vec![
            ArrowRight,
            ArrowLeft,
            DiamondArrowRight,
            DiamondArrowLeft,
        ]) {
            self.edge(id, current)
        } else {
            let title = self.title()?;
            println!("title {:?}", title);
            let children = if self.check(&LeftBrace) {
                self.nodes()?
            } else {
                vec![]
            };

            Ok(Element::Node(id, title, children))
        }
    }

    fn edge(&mut self, from_id: String, arrow: Token) -> anyhow::Result<Element> {
        let to_id = self.id()?;
        let title = self.title()?;
        Ok(Element::Edge(from_id, to_id, arrow.tokentype, title))
    }

    fn title(&mut self) -> anyhow::Result<Option<String>> {
        if self.check(&Colon) {
            self.advance();
            Ok(Some(self.string()?))
        } else {
            Ok(None)
        }
    }

    fn id(&mut self) -> anyhow::Result<String> {
        self.text()
    }

    fn string(&mut self) -> anyhow::Result<String> {
        let text = self.peek().clone();
        self.consume(Str, "Expected quoted string")?;
        Ok(text.lexeme.to_owned())
    }

    fn text(&mut self) -> anyhow::Result<String> {
        let text = self.peek().clone();
        self.consume(Identifier, "Expected text")?;
        Ok(text.lexeme.to_owned())
    }

    fn styles(&mut self) -> anyhow::Result<Vec<StyleNode>> {
        Ok(vec![])
    }

    fn consume(&mut self, tokentype: TokenType, expect: &str) -> anyhow::Result<&Token> {
        let current = self.peek();
        if self.check(&tokentype) {
            Ok(self.advance())
        } else {
            Err(anyhow!("Error: {} on line {}", expect, current.line))
        }
    }

    fn match_tokens(&mut self, tokentypes: Vec<TokenType>) -> bool {
        for tokentype in tokentypes.iter() {
            if self.check(tokentype) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn match_token(&mut self, tokentype: TokenType) -> bool {
        if self.check(&tokentype) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn check(&self, tokentype: &TokenType) -> bool {
        if self.is_at_end() {
            false
        } else {
            &self.peek().tokentype == tokentype
        }
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    fn is_at_end(&self) -> bool {
        self.peek().tokentype == TokenType::Eof
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }
}
