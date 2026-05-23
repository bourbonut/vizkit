use super::{LineAttrs, LineProperties, Orientation};

/// Creates an iterator of [`LineProperties`] for drawing vertical lines.
///
/// # Example
///
/// ```
/// use vizkit::{
///     chromatic::{ColorMap, Rainbow},
///     draw::{LineAttrs, LineProperties, grid_vertical_iter},
///     scale::{Axis, ScaleColor, ScaleContinuous},
/// };
///
/// let width = 900.;
/// let height = 400.;
///
/// let margin_top = 20.;
/// let margin_left = 30.;
/// let margin_right = 20.;
/// let margin_bottom = 30.;
///
/// let x = ScaleContinuous::linear()
///     .domain([0., 100.])
///     .range([margin_left, width - margin_right]);
///
/// let color = ScaleColor::linear(Rainbow::default()).domain([0., 100.]);
///
/// let grid: Vec<LineProperties> = grid_vertical_iter(
///     &x.ticks(None),
///     height - margin_bottom, // y bottom
///     margin_top,             // y top
///     |&tick| tick,
///     |&tick| LineAttrs {
///         stroke_color: color.apply(tick), // rainbow colors
///         ..Default::default()
///     }
/// ).collect();
/// ```
pub fn grid_vertical_iter<Data>(
    values: &[Data],
    y1: f32,
    y2: f32,
    x: impl Fn(&Data) -> f32,
    line_attrs: impl Fn(&Data) -> LineAttrs,
) -> impl Iterator<Item = LineProperties> {
    grid_iter(values, Orientation::Same, [y1, y2], x, line_attrs)
}

/// Creates an iterator of [`LineProperties`] for drawing horizontal lines.
///
/// # Example
///
/// ```
/// use vizkit::{
///     chromatic::{ColorMap, Rainbow},
///     draw::{LineAttrs, LineProperties, grid_horizontal_iter},
///     scale::{Axis, ScaleColor, ScaleContinuous},
/// };
///
/// let width = 900.;
/// let height = 400.;
///
/// let margin_top = 20.;
/// let margin_left = 30.;
/// let margin_right = 20.;
/// let margin_bottom = 30.;
///
/// let y = ScaleContinuous::linear()
///     .domain([0., 100.])
///     .range([height - margin_bottom, margin_top]);
///
/// let color = ScaleColor::linear(Rainbow::default()).domain([0., 100.]);
///
/// let grid: Vec<LineProperties> = grid_horizontal_iter(
///     &y.ticks(None),
///     margin_left,            // x left
///     width - margin_right,   // x right
///     |&tick| tick,
///     |&tick| LineAttrs {
///         stroke_color: color.apply(tick), // rainbow colors
///         ..Default::default()
///     }
/// ).collect();
/// ```
pub fn grid_horizontal_iter<Data>(
    values: &[Data],
    x1: f32,
    x2: f32,
    y: impl Fn(&Data) -> f32,
    line_attrs: impl Fn(&Data) -> LineAttrs,
) -> impl Iterator<Item = LineProperties> {
    grid_iter(values, Orientation::Flip, [x1, x2], y, line_attrs)
}

fn grid_iter<Data>(
    values: &[Data],
    orientation: Orientation,
    boundaries: [f32; 2],
    projection: impl Fn(&Data) -> f32,
    line_attrs: impl Fn(&Data) -> LineAttrs,
) -> impl Iterator<Item = LineProperties> {
    values.iter().map(move |value| {
        let projected = (projection)(value);
        let line_values = (line_attrs)(value);
        LineProperties {
            start: orientation.apply(projected, boundaries[0]),
            end: orientation.apply(projected, boundaries[1]),
            stroke_color: line_values.stroke_color,
            stroke_width: line_values.stroke_width,
            stroke_opacity: line_values.stroke_opacity,
        }
    })
}

#[cfg(test)]
mod tests {
    use super::grid_vertical_iter;
    use crate::chromatic::Color;
    use crate::draw::{LineAttrs, LineProperties};
    use crate::scale::{Axis, ScaleContinuous};

    #[test]
    fn test_grid() {
        let width = 400.;
        let margin_top = 10.;
        let margin_bottom = 40.;
        let height = 100.;

        let scale = ScaleContinuous::linear()
            .domain([0., 50.])
            .range([0., width]);
        let values = scale.ticks(None);

        let lines: Vec<LineProperties> = grid_vertical_iter(
            &values,
            margin_top,
            height - margin_bottom,
            |x| scale.tick_position(*x),
            |x| LineAttrs {
                stroke_width: x / 50.,
                stroke_color: Color([x / 50.; 3]),
                stroke_opacity: x / 50.,
            },
        )
        .collect();

        assert_eq!(lines.len(), values.len());
        for (line, x) in lines.iter().zip(values.iter()) {
            let z = x / 50.;
            let s = scale.tick_position(*x);
            assert_eq!(line.stroke_width, z);
            assert_eq!(line.stroke_opacity, z);
            assert_eq!(line.stroke_color.0, [z; 3]);
            assert_eq!(line.start, [s, margin_top]);
            assert_eq!(line.end, [s, height - margin_bottom]);
        }
    }
}
