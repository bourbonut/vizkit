use super::{CircleAttrs, CircleProperties, Draw};

pub fn circle<Data, D: Draw + ?Sized>(
    drawer: &mut D,
    values: &[Data],
    x: impl Fn(&Data) -> f32,
    y: impl Fn(&Data) -> f32,
    r: impl Fn(&Data) -> f32,
    circle_attrs: &CircleAttrs<Data>,
) {
    for value in values.iter() {
        drawer.draw_circle(CircleProperties {
            center: [(x)(value), (y)(value)],
            radius: (r)(value),
            fill_color: (circle_attrs.fill_color)(value),
            fill_opacity: (circle_attrs.fill_opacity)(value),
            stroke_color: (circle_attrs.stroke_color)(value),
            stroke_width: (circle_attrs.stroke_width)(value),
            stroke_opacity: (circle_attrs.stroke_opacity)(value),
        })
    }
}
