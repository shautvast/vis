use crate::render::svglib::div::Div;
use crate::render::svglib::{att, att_str3, Att, Element, ElementType, Value};

pub fn foreign_object() -> ForeignObject {
    ForeignObject::new()
}

pub struct ForeignObject {
    child: Option<Div>, //for now
    atts: Vec<Att>,
}

impl ForeignObject {
    pub fn new() -> Self {
        Self {
            child: None,
            atts: vec![],
        }
    }

    pub fn id<V>(&mut self, id: V)
    where
        V: Into<Value>,
    {
        self.atts.push(att("id", id));
    }

    pub fn x<V: Into<Value>>(mut self, x: V) -> Self {
        self.atts.push(att("x", x));
        self
    }

    pub fn y<V: Into<Value>>(mut self, y: V) -> Self {
        self.atts.push(att("y", y));
        self
    }
    pub fn width<V: Into<Value>>(mut self, width: V) -> Self {
        self.atts.push(att("width", width));
        self
    }
    pub fn height<V: Into<Value>>(mut self, height: V) -> Self {
        self.atts.push(att("height", height));
        self
    }
    pub fn class<V: Into<Value>>(mut self, class: V) -> Self {
        self.atts.push(att("class", class));
        self
    }

    pub fn add(mut self, child: Div) -> Self {
        self.child = Some(child);
        self
    }
}

impl Element for ForeignObject {
    fn get_type(&self) -> ElementType {
        ElementType::ForeignObject
    }

    fn atts(&self) -> &[Att] {
        &self.atts
    }

    fn to_string(&self) -> String {
        format!(
            r#"<foreignObject{}>{}</foreignObject>"#,
            self.atts.iter().map(|a| att_str3(a)).collect::<String>(),
            self.child
                .as_ref()
                .map(|c| c.to_string())
                .unwrap_or("".to_string()),
        )
    }
}
