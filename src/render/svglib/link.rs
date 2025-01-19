use crate::render::svglib::{att, Att, Element, ElementType, Value};

pub fn link(href: &str) -> Link {
    let mut atts = vec![];
    atts.push(att("href", href));
    Link { atts }
}

struct Link {
    atts: Vec<Att>,
}

impl Link {
    fn id<V>(&mut self, id: V)
    where
        V: Into<Value>,
    {
        self.atts.push(att("id", id));
    }
}

impl Element for Link {
    fn get_type(&self) -> ElementType {
        ElementType::Link
    }

    fn atts(&self) -> &[Att] {
        &self.atts
    }

    fn to_string(&self) -> String {
        todo!()
    }
}
