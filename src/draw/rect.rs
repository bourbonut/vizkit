use crate::draw::{RectProperties, ShapeAttrs};

/// Creates an iterator of [`RectProperties`] for drawing rectangles.
///
/// # Example
///
/// ```
/// use vizkit::{
///     chromatic::Color,
///     draw::{RectProperties, ShapeAttrs, rect_iter},
///     scale::{ScaleContinuous, ScaleDiscrete},
/// };
///
/// let x = ScaleDiscrete::band().domain(&[0, 15, 35, 65]).range([0., 1.]);
/// let y = ScaleContinuous::linear().domain([0., 3_000.]);
///
/// let data = [(0, 15.), (15, 1000.), (35, 2000.), (65, 2800.)];
///
/// let rects: Vec<RectProperties> = rect_iter(
///     &data,
///     |d| [x.scale(d.0).unwrap_or_default(), y.scale(0.)],
///     |d| [x.bandwidth(), y.scale(0.) - y.scale(d.1)],
///     Some(5.),
///     |d| ShapeAttrs {
///         fill_color: Some(Color(if d.1 < 1500. {
///             [1., 0., 0.] // red
///         } else {
///             [0., 0., 1.] // blue
///         })),
///         fill_opacity: 0.5,
///         ..Default::default()
///     },
/// ).collect();
/// ```
pub fn rect_iter<Data>(
    values: &[Data],
    top_left: impl Fn(&Data) -> [f32; 2],
    size: impl Fn(&Data) -> [f32; 2],
    corner_radius: Option<f32>,
    shape_attrs: impl Fn(&Data) -> ShapeAttrs,
) -> impl Iterator<Item = RectProperties> {
    values.iter().map(move |value| {
        let shape_values = shape_attrs(value);
        RectProperties {
            top_left: top_left(value),
            size: size(value),
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
    use crate::scale::{ScaleContinuous, ScaleDiscrete};

    #[test]
    fn test_rect_iter() {
        let width = 928.;
        let height = 500.;
        let margin_top = 30.;
        let margin_right = 0.;
        let margin_bottom = 30.;
        let margin_left = 40.;
        let x = ScaleDiscrete::band()
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
            |d| {
                [
                    x.scale(d.0).expect("Undefined values in x domain"),
                    y.scale(d.1),
                ]
            },
            |d| [x.bandwidth(), y.scale(0.) - y.scale(d.1)],
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
