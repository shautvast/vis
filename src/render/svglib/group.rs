use crate::render::svglib::{att, att_str3, Att, Element, ElementType, Value};

pub fn group() -> Group {
    Group {
        children: Vec::new(),
        atts: vec![],
    }
}

pub struct Group {
    children: Vec<Box<dyn Element>>,
    atts: Vec<Att>,
}

impl Group {
    pub fn add(&mut self, child: impl Element + 'static) {
        self.children.push(Box::new(child));
    }

    pub fn id<V>(&mut self, id: V)
    where
        V: Into<Value>,
    {
        self.atts.push(att("id", id));
    }

    pub fn transform<V>(&mut self, value: V)
    where
        V: Into<Value>,
    {
        self.atts.push(att("transform", value));
    }
}

impl Element for Group {
    fn get_type(&self) -> ElementType {
        ElementType::Group(Vec::new())
    }

    fn atts(&self) -> &[Att] {
        &self.atts
    }

    fn to_string(&self) -> String {
        let mut svg = format!("<g{}>", self.atts.iter().map(att_str3).collect::<String>());

        for e in &self.children {
            svg.push_str(e.to_string().as_str());
        }
        svg.push_str("</g>");
        svg
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render::svglib::rect::rect;

    #[test]
    fn test_group() {
        let mut g = group();
        g.id("testgroup");
        g.add(rect().x(0).y(0).width(10).height(10));
        assert_eq!(
            r#"<g id="testgroup"><rect x="0" y="0" width="10" height="10" /></g>"#,
            g.to_string()
        )
    }
}
