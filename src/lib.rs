use std::collections::HashMap;

use tokens::TokenType;

pub mod parser;
mod scanner;
mod tokens;

#[derive(Debug)]
pub struct Vis {
    pub markup: Vec<Element>,
    pub styles: Vec<StyleNode>,
}

#[derive(Debug)]
pub enum Element {
    Node(String, Option<String>, Vec<Element>),
    Edge(String, String, TokenType, Option<String>),
}

#[derive(Debug)]
pub struct StyleNode {
    pub id_ref: String,
    pub containertype: ContainerType,
    pub attributes: HashMap<String, String>,
}

#[derive(Debug)]
pub enum ContainerType {
    Node,
    Group,
}