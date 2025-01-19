pub mod circle;
pub mod div;
pub mod ellipse;
pub mod foreign_object;
pub mod group;
pub mod image;
pub mod line;
pub mod link;
pub mod path;
pub mod rect;
pub mod svg;
pub mod text;

#[derive(Debug)]
pub struct Value(String);

impl From<&str> for Value {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

impl Value {
    pub fn to_string(&self) -> String {
        self.0.clone()
    }
}

impl From<usize> for Value {
    fn from(s: usize) -> Self {
        Self(s.to_string())
    }
}

impl From<u32> for Value {
    fn from(s: u32) -> Self {
        Self(s.to_string())
    }
}

impl From<i32> for Value {
    fn from(s: i32) -> Self {
        Self(s.to_string())
    }
}

impl From<f32> for Value {
    fn from(s: f32) -> Self {
        Self(s.to_string())
    }
}

impl From<String> for Value {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl From<&String> for Value {
    fn from(s: &String) -> Self {
        Self(s.to_string())
    }
}

pub enum ElementType {
    Circle,
    Ellipse,
    ForeignObject,
    Group(Vec<ElementType>),
    Image,
    Line,
    Link,
    Path,
    Rect,
}

pub trait Element {
    fn get_type(&self) -> ElementType;
    fn to_string(&self) -> String;
    fn atts(&self) -> &[Att];
}

pub trait Shape {
    fn fill<V>(self, value: V) -> Self
    where
        V: Into<Value>;

    fn stroke<V>(self, value: V) -> Self
    where
        V: Into<Value>;

    fn transform<V>(self, value: V) -> Self
    where
        V: Into<Value>;
}

#[derive(Debug)]
pub struct Att {
    name: String,
    value: Value,
}

impl Att {
    pub fn new(name: &str, value: Value) -> Self {
        Self {
            name: name.to_string(),
            value,
        }
    }
}

fn att<V>(name: &str, value: V) -> Att
where
    V: Into<Value>,
{
    Att::new(name, value.into())
}

fn att_str(att: &Option<Att>) -> String {
    att.as_ref()
        .map(|a| format!(r#" {}="{}""#, a.name, a.value.to_string()))
        .unwrap_or("".to_string())
}

fn att_str2(att_name: &str, att_val: &Option<String>) -> String {
    att_val
        .as_ref()
        .map(|val| format!(r#" {}="{}""#, att_name, val))
        .unwrap_or("".to_string())
}

fn att_str3(att: &Att) -> String {
    format!(r#" {}="{}""#, att.name, att.value.to_string())
}
