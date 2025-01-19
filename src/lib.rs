pub mod parse;
pub mod render;

use parse::tokens::TokenType;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Vis {
    pub structure: Vec<Element>,
    pub styles: Vec<StyleNode>,
}

#[derive(Debug)]
pub enum Element {
    Node(String, Option<String>, Vec<Element>),
    Edge(String, String, TokenType, Option<String>),
}

impl Element {
    pub fn new_node(id: &str, label: Option<&str>, children: Vec<Element>) -> Element {
        Element::Node(id.into(), label.map(|s| s.into()), children)
    }
    pub fn new_edge(
        id: String,
        source: String,
        token: TokenType,
        label: Option<String>,
    ) -> Element {
        Element::Edge(id, source, token, label)
    }
}

#[derive(Debug, Clone)]
pub struct StyleNode {
    pub id_ref: String,
    pub containertype: ContainerType,
    pub attributes: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub enum ContainerType {
    Node,
    Group,
}
