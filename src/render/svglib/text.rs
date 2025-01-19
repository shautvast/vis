use crate::render::svglib::{att, att_str3, Att, Element, ElementType, Value};

pub fn text() -> Text {
    Text::new()
}

pub struct Text {
    atts: Vec<Att>,
    text: String,
}

impl Text {
    pub fn new() -> Self {
        let mut atts = vec![];
        Self {
            atts,
            text: "".to_string(),
        }
    }

    pub fn text<V: Into<String>>(mut self, text: V) -> Self {
        self.text = text.into();
        self
    }

    pub fn rounded() -> Self {
        Self {
            atts: vec![],
            text: "".to_string(),
        }
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

    pub fn fill<V: Into<Value>>(mut self, value: V) -> Self {
        self.atts.push(att("fill", value));
        self
    }

    pub fn stroke<V: Into<Value>>(mut self, value: V) -> Self {
        self.atts.push(att("stroke", value));
        self
    }

    fn transform<V: Into<Value>>(mut self, value: V) -> Self {
        self.atts.push(att("transform", value));
        self
    }

    pub fn attr<V: Into<Value>>(mut self, key: &str, value: V) -> Self {
        self.atts.push(att(key, value));
        self
    }
}

impl Element for Text {
    fn get_type(&self) -> ElementType {
        ElementType::Rect
    }

    fn atts(&self) -> &[Att] {
        &self.atts
    }

    fn to_string(&self) -> String {
        format!(
            r#"<text{}>{}</text>"#,
            self.atts.iter().map(att_str3).collect::<String>(),
            self.text,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rect() {
        let rect = text().x(0).y(0).width(10).height(10);
        assert_eq!(
            r#"<rect x="0" y="0" width="10" height="10" />"#,
            rect.to_string()
        )
    }
}
