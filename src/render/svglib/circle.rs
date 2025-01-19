use crate::render::svglib::{att, att_str3, Att, Element, ElementType, Shape, Value};

pub fn circle() -> Circle {
    Circle::new()
}

pub struct Circle(Vec<Att>);

impl Circle {
    fn new() -> Self {
        Self(vec![])
    }

    fn id<V: Into<Value>>(mut self, id: V) -> Self {
        self.0.push(att("id", id));
        self
    }
    fn cx<V: Into<Value>>(mut self, cx: V) -> Self {
        self.0.push(att("cx", cx));
        self
    }
    fn cy<V: Into<Value>>(mut self, cy: V) -> Self {
        self.0.push(att("cy", cy));
        self
    }
    fn r<V: Into<Value>>(mut self, r: V) -> Self {
        self.0.push(att("r", r));
        self
    }
}

impl Shape for Circle {
    fn fill<V: Into<Value>>(mut self, value: V) -> Self {
        self.0.push(att("fill", value));
        self
    }

    fn stroke<V: Into<Value>>(mut self, value: V) -> Self {
        self.0.push(att("stroke", value));
        self
    }

    fn transform<V: Into<Value>>(mut self, value: V) -> Self {
        self.0.push(att("transform", value));
        self
    }
}

impl Element for Circle {
    fn get_type(&self) -> ElementType {
        ElementType::Circle
    }

    fn atts(&self) -> &[Att] {
        &self.0
    }

    fn to_string(&self) -> String {
        format!(
            r#"<circle{} />"#,
            self.0.iter().map(att_str3).collect::<String>()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circle() {
        let circle = circle().cx("1em").cy(0).r("10").id("c");
        assert_eq!(
            r#"<circle cx="1em" cy="0" r="10" id="c" />"#,
            circle.to_string()
        )
    }
}
