use super::{Draw, LineProperties};
use crate::draw::{LineAttrs, Orientation};
use std::marker::PhantomData;

pub struct Grid<Data, Projection>
where
    Projection: Fn(&Data) -> f32,
{
    projection: Projection,
    orientation: Orientation,
    boundaries: [f32; 2],
    marker: PhantomData<Data>,
}

impl<Data, Projection> Grid<Data, Projection>
where
    Projection: Fn(&Data) -> f32,
{
    pub fn vertical(top: f32, down: f32, projection: Projection) -> Self {
        Self {
            orientation: Orientation::Same,
            boundaries: [top, down],
            projection: projection,
            marker: PhantomData,
        }
    }

    pub fn horizontal(left: f32, right: f32, projection: Projection) -> Self {
        Self {
            orientation: Orientation::Flip,
            boundaries: [left, right],
            projection: projection,
            marker: PhantomData,
        }
    }

    pub fn draw<D: Draw>(&self, drawer: &mut D, values: &[Data], line_attrbs: &LineAttrs<Data>) {
        for value in values.iter() {
            let projected = (self.projection)(value);
            drawer.line(LineProperties {
                start: self.orientation.apply(projected, self.boundaries[0]),
                end: self.orientation.apply(projected, self.boundaries[1]),
                color: (line_attrbs.color)(value),
                width: (line_attrbs.width)(value),
                opacity: (line_attrbs.opacity)(value),
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Grid;
    use crate::chromatic::Color;
    use crate::draw::{Draw, LineAttrs, LineProperties, TextProperties};
    use crate::scale::ScaleContinuous;

    #[derive(Default)]
    struct Drawer {
        lines: Vec<LineProperties>,
    }

    impl<'a> Draw for Drawer {
        fn line(&mut self, line: LineProperties) {
            self.lines.push(line);
        }

        fn text(&mut self, _: TextProperties) {
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
        Grid::vertical(margin_top, height - margin_bottom, |x| scale.apply(*x)).draw(
            &mut drawer,
            &values,
            &LineAttrs::default()
                .width_with(|x| x / 50.)
                .color_with(|x| Color([x / 50.; 3]))
                .opacity_with(|x| x / 50.),
        );

        assert_eq!(drawer.lines.len(), values.len());
        for (line, x) in drawer.lines.iter().zip(values.iter()) {
            let z = x / 50.;
            let s = scale.apply(*x);
            assert_eq!(line.width, z);
            assert_eq!(line.opacity, z);
            assert_eq!(line.color.0, [z; 3]);
            assert_eq!(line.start, [s, margin_top]);
            assert_eq!(line.end, [s, height - margin_bottom]);
        }
    }
}
