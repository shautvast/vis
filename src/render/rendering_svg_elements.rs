use crate::render::svglib::rect::rect;
use crate::render::svglib::svg::{svg, Svg};
use crate::render::svglib::text::text;
use crate::{Element, StyleNode, Vis};

pub fn render_vis_with_grid_layout(
    vis: &Vis,
    grid_cell_size: f32,
    spacing: f32,
    padding: f32,
) -> String {
    let style = include_str!("svg_node.css");
    let mut svg = svg();
    svg.style(style);
    svg.viewbox("0 0 100 100"); // Default size; can adjust dynamically

    // Start recursive layout
    let (parent_width, parent_height) = layout_box(
        &mut svg,
        &vis.structure,
        grid_cell_size,
        spacing,
        padding,
        0.0,
        0.0,
    );

    svg.width(parent_width as usize);
    svg.height(parent_height as usize);
    svg.to_string()
}

fn layout_box(
    svg: &mut Svg,
    elements: &[Element],
    grid_cell_size: f32,
    spacing: f32,
    padding: f32,
    start_x: f32,
    start_y: f32,
) -> (f32, f32) {
    let current_x = start_x + padding;
    let current_y = start_y + padding;
    let mut max_width: f32 = 0.0;
    let mut max_height: f32 = 0.0;

    let grid_cols = (elements.len() as f32).sqrt().ceil() as usize;

    for (i, element) in elements.iter().enumerate() {
        let col = i % grid_cols;
        let row = i / grid_cols;

        let child_x = current_x + col as f32 * (grid_cell_size + spacing);
        let child_y = current_y + row as f32 * (grid_cell_size + spacing);

        match element {
            Element::Node(id, label, children) => {
                let (_, _) = layout_box(
                    svg,
                    children,
                    grid_cell_size,
                    spacing,
                    padding,
                    child_x,
                    child_y,
                );
                let width = label.as_ref().map(|l| l.len() * 10).unwrap_or(100);
                let height = label
                    .as_ref()
                    .map(|l| l.lines().collect::<String>().len() * 10)
                    .unwrap_or(100);
                svg.add(
                    rect()
                        .x(child_x)
                        .y(child_y)
                        .width(width)
                        .height(height)
                        .fill("none")
                        .stroke("green"),
                );

                if let Some(label) = label {
                    svg.add(
                        text()
                            .x(child_x + (width as f32 / 2.0))
                            .y(child_y + (height as f32 / 2.0))
                            .text(label)
                            .attr("text-anchor", "middle"),
                    );
                }

                // Update the bounding box dimensions
                max_width = max_width.max(child_x + width as f32 - start_x);
                max_height = max_height.max(child_y + height as f32 - start_y);
            }
            Element::Edge(_, _, _, _) => {
                // Edges are processed after all nodes are laid out to connect them
                // For now, assume no visual rendering of edges in this example
            }
        }
    }
    let total_width = max_width + padding * 2.0;
    let total_height = max_height + padding * 2.0;
    if elements.len() > 0 {
        let parent_box = rect()
            .x(start_x)
            .y(start_y)
            .width(total_width)
            .height(total_height)
            .fill("none")
            .stroke("red")
            .attr("stroke-width", "2");
        svg.add(parent_box);
    }

    (total_width, total_height)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ContainerType, Element, StyleNode, Vis};
    use std::fs::File;
    use std::io::Write;

    #[test]
    fn test_render_vis_with_grid_layout() {
        // Create a mock StyleNode
        let style_node = StyleNode {
            id_ref: "node_1".to_string(),
            containertype: ContainerType::Node,
            attributes: [("fill".to_string(), "white".to_string())]
                .iter()
                .cloned()
                .collect(),
        };

        // Create mock Elements
        let element = Element::Node(
            "node_1".to_string(),
            Some("Node 1".to_string()),
            vec![], // No child elements
        );
        let element2 = Element::Node(
            "node_1".to_string(),
            Some("Node 1".to_string()),
            vec![], // No child elements
        );

        // Create Vis structure
        let vis = Vis {
            styles: vec![style_node],
            structure: vec![element, element2],
        };

        let svg_output = render_vis_with_grid_layout(&vis, 50.0, 10.0, 5.0);

        let mut file = File::create("output_multiple_nodes.svg").expect("Unable to create file");
        file.write_all(&svg_output.as_bytes().to_vec())
            .expect("Unable to write data");
    }
}
