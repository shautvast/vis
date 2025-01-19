use crate::Vis;

pub mod html;
mod rendering_svg_elements;
mod svg_renderer;
pub mod svglib;
mod svgrender2;

/// trait for turning the object model into a byte representation
pub trait Renderer {
    fn render(&self, vis: Vis) -> anyhow::Result<Vec<u8>>;
}
