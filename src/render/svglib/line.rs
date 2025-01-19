use crate::render::svglib::{att, att_str3, Att, Element, ElementType, Shape, Value};

pub fn line() -> Line {
    Line(vec![])
}

pub struct Line(Vec<Att>);

impl Line {
    fn id<V>(&mut self, id: V)
    where
        V: Into<Value>,
    {
        self.0.push(att("id", id));
    }

    pub fn x1<V: Into<Value>>(mut self, x: V) -> Self {
        self.0.push(att("x1", x));
        self
    }
    pub fn y1<V: Into<Value>>(mut self, y: V) -> Self {
        self.0.push(att("y1", y));
        self
    }
    pub fn x2<V: Into<Value>>(mut self, x2: V) -> Self {
        self.0.push(att("x2", x2));
        self
    }
    pub fn y2<V: Into<Value>>(mut self, y2: V) -> Self {
        self.0.push(att("y2", y2));
        self
    }
    pub fn attr<V: Into<Value>>(mut self, key: &str, value: V) -> Self {
        self.0.push(att(key, value));
        self
    }
}

impl Element for Line {
    fn get_type(&self) -> ElementType {
        ElementType::Line
    }

    fn atts(&self) -> &[Att] {
        &self.0
    }

    fn to_string(&self) -> String {
        format!(
            r#" <line{} />"#,
            self.0.iter().map(att_str3).collect::<String>()
        )
    }
}

impl Shape for Line {
    fn fill<V>(mut self, value: V) -> Self
    where
        V: Into<Value>,
    {
        self.0.push(att("fill", value));
        self
    }

    fn stroke<V>(mut self, value: V) -> Self
    where
        V: Into<Value>,
    {
        self.0.push(att("stroke", value));
        self
    }

    fn transform<V>(mut self, value: V) -> Self
    where
        V: Into<Value>,
    {
        self.0.push(att("transform", value));
        self
    }
}
