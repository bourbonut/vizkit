use super::{LineAttrs, LineProperties, Orientation};

pub fn grid_vertical_iter<Data>(
    values: &[Data],
    y1: f32,
    y2: f32,
    x: impl Fn(&Data) -> f32,
    line_attrs: impl Fn(&Data) -> LineAttrs,
) -> impl Iterator<Item = LineProperties> {
    grid_iter(values, Orientation::Same, [y1, y2], x, line_attrs)
}

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
    use crate::chromatic::Color;
    use crate::draw::{CircleProperties, Draw, LineAttrs, LineProperties, TextProperties};
    use crate::scale::ScaleContinuous;

    #[derive(Default)]
    struct Drawer {
        lines: Vec<LineProperties>,
    }

    impl<'a> Draw for Drawer {
        fn draw_line(&mut self, line: LineProperties) {
            self.lines.push(line);
        }

        fn draw_text(&mut self, _: TextProperties) {
            todo!()
        }

        fn draw_circle(&mut self, _: CircleProperties) {
            todo!()
        }
    }

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

        let mut drawer = Drawer::default();
        drawer.grid_vertical(
            &values,
            margin_top,
            height - margin_bottom,
            |x| scale.apply(*x),
            |x| LineAttrs {
                stroke_width: x / 50.,
                stroke_color: Color([x / 50.; 3]),
                stroke_opacity: x / 50.,
            },
        );

        assert_eq!(drawer.lines.len(), values.len());
        for (line, x) in drawer.lines.iter().zip(values.iter()) {
            let z = x / 50.;
            let s = scale.apply(*x);
            assert_eq!(line.stroke_width, z);
            assert_eq!(line.stroke_opacity, z);
            assert_eq!(line.stroke_color.0, [z; 3]);
            assert_eq!(line.start, [s, margin_top]);
            assert_eq!(line.end, [s, height - margin_bottom]);
        }
    }
}
