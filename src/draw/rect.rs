use crate::draw::{RectProperties, ShapeAttrs};

/// Creates an iterator of properties used for rectangles.
pub fn rect_iter<Data>(
    values: &[Data],
    x: impl Fn(&Data) -> f32,
    y: impl Fn(&Data) -> f32,
    width: impl Fn(&Data) -> f32,
    height: impl Fn(&Data) -> f32,
    corner_radius: Option<f32>,
    shape_attrs: impl Fn(&Data) -> ShapeAttrs,
) -> impl Iterator<Item = RectProperties> {
    values.iter().map(move |value| {
        let shape_values = shape_attrs(value);
        RectProperties {
            top_left: [x(value), y(value)],
            size: [width(value), height(value)],
            corner_radius,
            fill_color: shape_values.fill_color,
            fill_opacity: shape_values.fill_opacity,
            stroke_color: shape_values.stroke_color,
            stroke_width: shape_values.stroke_width,
            stroke_opacity: shape_values.stroke_opacity,
        }
    })
}
