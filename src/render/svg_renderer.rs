// use crate::render::svglib::rect::{rect, Rect};
// use crate::render::svglib::svg::svg;
// use crate::render::svglib::text::text;
// use crate::render::svglib::Value;
// use crate::render::Renderer;
// use crate::{Element, Vis};
// use std::fs::File;
// use std::io::Write;
//
// struct SvgRenderer {}
// impl Renderer for SvgRenderer {
//     fn render(&self, vis: Vis) -> anyhow::Result<Vec<u8>> {
//         let style = include_str!("svg_node.css");
//         let mut svg = svg();
//         svg.viewbox("0, 0, 300, 300");
//         svg.style(style);
//
//         let current_x = start_x + padding;
//         let current_y = start_y + padding;
//         let mut max_width: f32 = 0.0;
//         let mut max_height: f32 = 0.0;
//
//         let grid_cols = (elements.len() as f32).sqrt().ceil() as usize;
//
//         for e in vis.structure {
//             let col = i % grid_cols;
//             let row = i / grid_cols;
//
//             let child_x = current_x + col as f32 * (grid_cell_size + spacing);
//             let child_y = current_y + row as f32 * (grid_cell_size + spacing);
//
//             if let Element::Node(_, label, children) = e {
//                 if let Some(label) = label {
//                     let mut longest_len = 0;
//                     for line in label.lines() {
//                         longest_len = longest_len.max(line.len());
//                     }
//                     let width = longest_len / 2;
//                     let linecount = label.lines().count();
//                     let height = linecount + 1;
//                     let x = 1;
//                     let y = ((height - linecount) / 2 + 2) as f64
//                         - if linecount == 1 { 0.25 } else { 0.0 };
//
//                     let width = label.len() * 10;
//                     let height = label.lines().collect::<String>().len() * 10;
//                     svg.add(
//                         rect()
//                             .x(child_x)
//                             .y(child_y)
//                             .width(width)
//                             .height(height)
//                             .fill("none")
//                             .stroke("green"),
//                     );
//
//                     if let Some(label) = label {
//                         svg.add(
//                             text()
//                                 .x(child_x + (width as f32 / 2.0))
//                                 .y(child_y + (height as f32 / 2.0))
//                                 .text(label)
//                                 .attr("text-anchor", "middle"),
//                         );
//                     }
//
//                     svg.add(text);
//                 } else {
//                     svg.add(rectangle(0, 0, 70, 30));
//                 }
//             }
//         }
//
//         let svg = svg.to_string();
//
//         let mut file = File::create("output.svg")?;
//         file.write_all(svg.as_bytes())?;
//         Ok(svg.as_bytes().to_vec())
//     }
// }
//
// fn rectangle<V>(x: usize, y: usize, width: V, height: V) -> Rect
// where
//     V: Into<Value>,
// {
//     Rect::new()
//         .x(x)
//         .y(y)
//         .width(width)
//         .height(height)
//         .class("rect")
//         .fill("none")
// }
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::Element;
//     use std::fs::File;
//     use std::io::Write;
//
//     #[test]
//     fn test_render() {
//         let vis = Vis {
//             structure: vec![Element::new_node(
//                 "id",
//                 Some("blokkendoos\nkoepeon\nknallen\nhond"),
//                 vec![],
//             )],
//             styles: vec![],
//         };
//         let renderer = SvgRenderer {};
//         let buf = renderer.render(vis).unwrap();
//         let mut file = File::create("output.svg").expect("Unable to create file");
//         file.write_all(&buf).expect("Unable to write data");
//     }
// }
