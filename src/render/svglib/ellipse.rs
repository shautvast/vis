use crate::render::svglib::{att, att_str3, Att, Element, ElementType, Shape, Value};

pub fn ellipse() -> Ellipse {
    Ellipse(vec![])
}

#[derive(Debug)]
pub struct Ellipse(Vec<Att>);

impl Ellipse {
    fn cx<V: Into<Value>>(mut self, cx: V) -> Self {
        self.0.push(att("cx", cx));
        self
    }
    fn cy<V: Into<Value>>(mut self, cy: V) -> Self {
        self.0.push(att("cy", cy));
        self
    }
    fn rx<V: Into<Value>>(mut self, rx: V) -> Self {
        self.0.push(att("rx", rx));
        self
    }
    fn ry<V: Into<Value>>(mut self, ry: V) -> Self {
        self.0.push(att("ry", ry));
        self
    }
}

impl Shape for Ellipse {
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

impl Element for Ellipse {
    fn get_type(&self) -> ElementType {
        ElementType::Ellipse
    }

    fn atts(&self) -> &[Att] {
        &self.0
    }

    fn to_string(&self) -> String {
        format!(
            r#"<ellipse{} />"#,
            self.0.iter().map(att_str3).collect::<String>()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ellipse() {
        let ellipse = ellipse().cx(0).cy(0).rx(10).ry(15);
        println!("{:?}", ellipse);
        assert_eq!(
            r#"<ellipse cx="0" cy="0" rx="10" ry="15" />"#,
            ellipse.to_string()
        )
    }
}
