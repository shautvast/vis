use crate::render::svglib::{att, att_str3, Att, Value};

pub fn div() -> Div {
    Div::new()
}

pub struct Div {
    atts: Vec<Att>,
    child: String,
}

impl Div {
    pub fn new() -> Self {
        Self {
            atts: vec![],
            child: "".to_string(),
        }
    }

    pub fn id<V>(&mut self, id: V)
    where
        V: Into<Value>,
    {
        self.atts.push(att("id", id));
    }

    pub fn class<V>(mut self, class: V) -> Self
    where
        V: Into<Value>,
    {
        self.atts.push(att("class", class));
        self
    }

    pub fn innerHTML<V: Into<String>>(mut self, html: V) -> Self {
        self.child = html.into();
        self
    }

    pub fn to_string(&self) -> String {
        format!(
            r#"<div xmlns="http://www.w3.org/1999/xhtml"{}>{}</div>"#,
            self.atts.iter().map(att_str3).collect::<String>(),
            self.child
        )
    }

    fn atts(&self) -> &[Att] {
        &self.atts
    }
}
