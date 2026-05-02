use super::{CircleProperties, ShapeAttrs};

/// Creates an iterator of properties used for circles.
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scale::ScaleContinuous;

    #[test]
    fn test_rect_iter() {
        let width = 928.;
        let height = 500.;
        let margin_top = 30.;
        let margin_right = 0.;
        let margin_bottom = 30.;
        let margin_left = 40.;
        let x = ScaleContinuous::linear()
            .domain([0., 50.])
            .range([margin_left, width - margin_right]);

        let y = ScaleContinuous::linear()
            .domain([0., 200.])
            .range([height - margin_bottom, margin_top]);

        let values = [
            (5., 90.),
            (20., 30.),
            (30., 120.),
            (40., 20.),
            (45., 50.),
            (50., 200.),
        ];

        for circle_property in circle_iter(
            &values,
            |d| x.apply(d.0),
            |d| y.apply(d.1),
            |_| 5.,
            |_| ShapeAttrs::stroke_default(),
        ) {
            assert!(
                circle_property.center[0] >= margin_left
                    && circle_property.center[0] <= width - margin_right
            );
            assert!(
                circle_property.center[1] >= margin_top
                    && circle_property.center[1] <= height - margin_bottom
            );
            assert_eq!(circle_property.radius, 5.);
        }
    }
}
