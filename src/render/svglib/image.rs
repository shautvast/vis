use crate::render::svglib::{att, att_str3, Att, Element, ElementType, Value};

pub fn image() -> Image {
    Image::new()
}

struct Image {
    atts: Vec<Att>,
}

impl Image {
    fn new() -> Self {
        Self { atts: vec![] }
    }

    fn id<V>(&mut self, id: V)
    where
        V: Into<Value>,
    {
        self.atts.push(att("id", id));
    }

    fn transform<V>(&mut self, value: V)
    where
        V: Into<Value>,
    {
        self.atts.push(att("transform", value));
    }
    fn x<V: Into<Value>>(mut self, x: V) -> Self {
        self.atts.push(att("x", x));
        self
    }
    fn y<V: Into<Value>>(mut self, y: V) -> Self {
        self.atts.push(att("y", y));
        self
    }
    fn width<V: Into<Value>>(mut self, width: V) -> Self {
        self.atts.push(att("width", width));
        self
    }
    fn height<V: Into<Value>>(mut self, height: V) -> Self {
        self.atts.push(att("height", height));
        self
    }
    fn href<V: Into<Value>>(mut self, href: V) -> Self {
        self.atts.push(att("href", href));
        self
    }
}

impl Element for Image {
    fn get_type(&self) -> ElementType {
        ElementType::Image
    }

    fn atts(&self) -> &[Att] {
        &self.atts
    }

    fn to_string(&self) -> String {
        format!(
            r#"<image{} />"#,
            self.atts.iter().map(att_str3).collect::<String>()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_image() {
        let rect = image()
            .href("http://acme.com/coyote.jpg")
            .x(0)
            .y(0)
            .width(10)
            .height(10);
        assert_eq!(
            r#"<image href="http://acme.com/coyote.jpg" x="0" y="0" width="10" height="10" />"#,
            rect.to_string()
        )
    }
}
