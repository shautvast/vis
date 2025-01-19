use crate::render::svglib::{att_str2, Att, Element, Value};

pub fn svg() -> Svg {
    Svg::new()
}

pub struct Svg {
    style: Option<String>,
    elements: Vec<Box<dyn Element>>,
    width: Option<String>,
    height: Option<String>,
    viewbox: Option<String>,
    preserveaspectratio: Option<String>,
    transform: Option<String>,
    atts: Vec<Att>,
}

impl Svg {
    pub fn new() -> Self {
        Self {
            style: None,
            elements: Vec::new(),
            width: None,
            height: None,
            viewbox: None,
            preserveaspectratio: None,
            transform: None,
            atts: vec![],
        }
    }

    pub fn style(&mut self, style: &str) {
        self.style = Some(style.to_string());
    }

    pub fn add(&mut self, child: impl Element + 'static) {
        self.elements.push(Box::new(child));
    }

    pub fn width<V: Into<Value>>(&mut self, width: V) {
        self.width = Some(width.into().to_string());
    }

    pub fn height<V: Into<Value>>(&mut self, height: V) {
        self.height = Some(height.into().to_string());
    }

    pub fn viewbox(&mut self, viewbox: &str) {
        self.viewbox = Some(viewbox.to_string());
    }

    pub fn preserveaspectratio(&mut self, preserveaspectratio: &str) {
        self.preserveaspectratio = Some(preserveaspectratio.to_string());
    }

    fn transform<V: Into<Value>>(&mut self, value: V) {
        self.transform = Some(value.into().to_string());
    }

    pub fn to_string(&self) -> String {
        let mut svg = String::new();
        svg.push_str(
            format!(
                r#"<svg{}{}{}{}{} xmlns="http://www.w3.org/2000/svg">{}"#,
                att_str2("width", &self.width),
                att_str2("height", &self.height),
                att_str2("viewBox", &self.viewbox),
                att_str2("preserveAspectRatio", &self.preserveaspectratio),
                att_str2("transform", &self.transform),
                self.style
                    .as_ref()
                    .map(|s| format!("<style>{}</style>", s.to_string()))
                    .unwrap_or("".to_string())
            )
            .as_str(),
        );

        for e in &self.elements {
            svg.push_str(e.to_string().as_str());
        }
        svg.push_str("</svg>");
        svg
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render::svglib::rect::rect;

    #[test]
    fn style() {
        let mut svg = Svg::new();
        svg.style(".id { background-color: red; }");
        assert_eq!(
            r#"<svg xmlns="http://www.w3.org/2000/svg"><style>.id { background-color: red; }</style></svg>"#,
            svg.to_string()
        )
    }

    #[test]
    fn add_rect() {
        let mut svg = Svg::new();
        svg.preserveaspectratio("none");
        svg.add(rect().x(0).y(0).width(10).height(10));
        assert_eq!(
            r#"<svg preserveAspectRatio="none" xmlns="http://www.w3.org/2000/svg"><rect x="0" y="0" width="10" height="10" /></svg>"#,
            svg.to_string()
        )
    }
}
