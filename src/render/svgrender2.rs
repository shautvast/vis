use crate::render::Renderer;
use crate::Vis;

struct SvgRender;

impl Renderer for SvgRender {
    fn render(&self, vis: Vis) -> anyhow::Result<Vec<u8>> {
        Ok(vec![])
    }
}
