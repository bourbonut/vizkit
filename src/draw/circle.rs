use super::{CircleAttrs, CircleProperties, Draw};

pub fn circle<Data, D: Draw + ?Sized>(
    drawer: &mut D,
    values: &[Data],
    x: impl Fn(&Data) -> f32,
    y: impl Fn(&Data) -> f32,
    r: impl Fn(&Data) -> f32,
    circle_attrs: impl Fn(&Data) -> CircleAttrs,
) {
    for value in values.iter() {
        let circle_values = (circle_attrs)(value);
        drawer.draw_circle(CircleProperties {
            center: [(x)(value), (y)(value)],
            radius: (r)(value),
            fill_color: circle_values.fill_color,
            fill_opacity: circle_values.fill_opacity,
            stroke_color: circle_values.stroke_color,
            stroke_width: circle_values.stroke_width,
            stroke_opacity: circle_values.stroke_opacity,
        })
    }
}
