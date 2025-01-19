use crate::render::svglib::{att, Att, Element, ElementType, Shape, Value};

pub fn path(d: &str) -> Path {
    Path::new(d)
}

pub struct Path {
    d: String,
    atts: Vec<Att>,
}

impl Element for Path {
    fn get_type(&self) -> ElementType {
        ElementType::Path
    }

    fn atts(&self) -> &[Att] {
        &self.atts
    }

    fn to_string(&self) -> String {
        todo!()
    }
}

impl Path {
    pub fn new(d: &str) -> Self {
        Self {
            d: d.to_string(),
            atts: vec![],
        }
    }

    fn id<V>(&mut self, id: V)
    where
        V: Into<Value>,
    {
        self.atts.push(att("id", id));
    }

    pub fn m(&mut self, x: usize, y: usize) {
        self.d.push_str(&format!(" m{} {}", x, y));
    }

    #[allow(non_snake_case)]
    pub fn M(&mut self, x: usize, y: usize) {
        self.d.push_str(&format!(" M{} {}", x, y));
    }

    pub fn z(&mut self) {
        self.d.push_str(" z");
    }

    pub fn l(&mut self, x: usize, y: usize) {
        self.d.push_str(&format!(" l{} {}", x, y));
    }

    #[allow(non_snake_case)]
    pub fn L(&mut self, x: usize, y: usize) {
        self.d.push_str(&format!(" L{} {}", x, y));
    }

    pub fn h(&mut self, x: usize) {
        self.d.push_str(&format!(" h{}", x));
    }

    #[allow(non_snake_case)]
    pub fn H(&mut self, x: usize) {
        self.d.push_str(&format!(" H{}", x));
    }

    pub fn v(&mut self, x: usize) {
        self.d.push_str(&format!(" v{}", x));
    }

    #[allow(non_snake_case)]
    pub fn V(&mut self, x: usize) {
        self.d.push_str(&format!(" V{}", x));
    }

    pub fn c(&mut self, x1: usize, y1: usize, x2: usize, y2: usize) {
        self.d.push_str(&format!(" c{} {} {} {}", x1, y1, x2, y2));
    }

    #[allow(non_snake_case)]
    pub fn C(&mut self, x1: usize, y1: usize, x2: usize, y2: usize) {
        self.d.push_str(&format!(" C{} {} {} {}", x1, y1, x2, y2));
    }

    pub fn s(&mut self, x1: usize, y1: usize, x2: usize, y2: usize) {
        self.d.push_str(&format!(" s{} {} {} {}", x1, y1, x2, y2));
    }

    #[allow(non_snake_case)]
    pub fn S(&mut self, x1: usize, y1: usize, x2: usize, y2: usize) {
        self.d.push_str(&format!(" S{} {} {} {}", x1, y1, x2, y2));
    }
}

impl Shape for Path {
    fn fill<V>(mut self, value: V) -> Self
    where
        V: Into<Value>,
    {
        self.atts.push(att("fill", value));
        self
    }

    fn stroke<V>(mut self, value: V) -> Self
    where
        V: Into<Value>,
    {
        self.atts.push(att("stroke", value));
        self
    }

    fn transform<V>(mut self, value: V) -> Self
    where
        V: Into<Value>,
    {
        self.atts.push(att("transform", value));
        self
    }
}
