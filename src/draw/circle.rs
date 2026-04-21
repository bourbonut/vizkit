use super::{CircleProperties, ShapeAttrs};

pub fn circle_iter<Data>(
    values: &[Data],
    x: impl Fn(&Data) -> f32,
    y: impl Fn(&Data) -> f32,
    r: impl Fn(&Data) -> f32,
    shape_attrs: impl Fn(&Data) -> ShapeAttrs,
) -> impl Iterator<Item = CircleProperties> {
    values.iter().map(move |value| {
        let circle_values = (shape_attrs)(value);
        CircleProperties {
            center: [x(value), y(value)],
            radius: r(value),
            fill_color: circle_values.fill_color,
            fill_opacity: circle_values.fill_opacity,
            stroke_color: circle_values.stroke_color,
            stroke_width: circle_values.stroke_width,
            stroke_opacity: circle_values.stroke_opacity,
        }
    })
}
