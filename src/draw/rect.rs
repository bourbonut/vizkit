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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scale::{ScaleBand, ScaleContinuous};

    #[test]
    fn test_rect_iter() {
        let width = 928.;
        let height = 500.;
        let margin_top = 30.;
        let margin_right = 0.;
        let margin_bottom = 30.;
        let margin_left = 40.;
        let x = ScaleBand::default()
            .domain(&["a", "b", "c", "d", "e", "f"])
            .range([margin_left, width - margin_right])
            .padding(0.1);

        let y = ScaleContinuous::linear()
            .domain([0., 200.])
            .range([height - margin_bottom, margin_top]);

        let values = [
            ("a", 90.),
            ("b", 30.),
            ("c", 120.),
            ("d", 20.),
            ("e", 50.),
            ("f", 200.),
        ];

        for rect_property in rect_iter(
            &values,
            |d| *x.apply(d.0).expect("Undefined values in x domain"),
            |d| y.apply(d.1),
            |_| x.bandwidth(),
            |d| y.apply(0.) - y.apply(d.1),
            None,
            |_| ShapeAttrs::stroke_default(),
        ) {
            assert!(
                rect_property.top_left[0] >= margin_left
                    && rect_property.top_left[0] <= width - margin_right
            );
            assert!(
                rect_property.top_left[1] >= margin_top
                    && rect_property.top_left[1] <= height - margin_bottom
            );
            assert_eq!(rect_property.size[0], x.bandwidth());
            assert_ne!(rect_property.size[1], 0.);
        }
    }
}
