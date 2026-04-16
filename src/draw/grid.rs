use super::{Draw, LineProperties};
use crate::{
    draw::Orientation,
    generator::{Constant, Function, Generator},
};
use std::marker::PhantomData;

use crate::chromatic::Color;

pub struct Grid<
    Data,
    S: Generator<Data, Output = f32>,
    W: Generator<Data, Output = f32>,
    C: Generator<Data, Output = Color>,
    O: Generator<Data, Output = f32>,
> {
    data: PhantomData<Data>,
    orientation: Orientation,
    boundaries: [f32; 2],
    scale: S,
    width: W,
    color: C,
    opacity: O,
}

impl<F, Data> Grid<Data, Function<F, Data, f32>, Constant<f32>, Constant<Color>, Constant<f32>>
where
    F: Fn(&Data) -> f32,
{
    pub fn vertical(top: f32, down: f32, scale_fn: F) -> Self {
        Self {
            data: PhantomData,
            orientation: Orientation::Same,
            boundaries: [top, down],
            scale: Function::new(scale_fn),
            width: Constant(1.),
            color: Constant(Color::default()),
            opacity: Constant(1.),
        }
    }

    pub fn horizontal(left: f32, right: f32, scale_fn: F) -> Self {
        Self {
            data: PhantomData,
            orientation: Orientation::Flip,
            boundaries: [left, right],
            scale: Function::new(scale_fn),
            width: Constant(1.),
            color: Constant(Color::default()),
            opacity: Constant(1.),
        }
    }
}

impl<
    Data,
    S: Generator<Data, Output = f32>,
    W: Generator<Data, Output = f32>,
    C: Generator<Data, Output = Color>,
    O: Generator<Data, Output = f32>,
> Grid<Data, S, W, C, O>
{
    pub fn width_with<F>(self, width_fn: F) -> Grid<Data, S, Function<F, Data, f32>, C, O>
    where
        F: Fn(&Data) -> f32,
    {
        Grid::<Data, S, Function<F, Data, f32>, C, O> {
            data: self.data,
            orientation: self.orientation,
            boundaries: self.boundaries,
            scale: self.scale,
            width: Function::new(width_fn),
            color: self.color,
            opacity: self.opacity,
        }
    }

    pub fn color_with<F>(self, color_fn: F) -> Grid<Data, S, W, Function<F, Data, Color>, O>
    where
        F: Fn(&Data) -> Color,
    {
        Grid::<Data, S, W, Function<F, Data, Color>, O> {
            data: self.data,
            orientation: self.orientation,
            boundaries: self.boundaries,
            scale: self.scale,
            width: self.width,
            color: Function::new(color_fn),
            opacity: self.opacity,
        }
    }

    pub fn opacity_with<F>(self, opacity_fn: F) -> Grid<Data, S, W, C, Function<F, Data, f32>>
    where
        F: Fn(&Data) -> f32,
    {
        Grid::<Data, S, W, C, Function<F, Data, f32>> {
            data: self.data,
            orientation: self.orientation,
            boundaries: self.boundaries,
            scale: self.scale,
            width: self.width,
            color: self.color,
            opacity: Function::new(opacity_fn),
        }
    }

    pub fn width(self, width: f32) -> Grid<Data, S, Constant<f32>, C, O> {
        Grid::<Data, S, Constant<f32>, C, O> {
            data: self.data,
            orientation: self.orientation,
            boundaries: self.boundaries,
            scale: self.scale,
            width: Constant(width),
            color: self.color,
            opacity: self.opacity,
        }
    }

    pub fn color(self, color: Color) -> Grid<Data, S, W, Constant<Color>, O> {
        Grid::<Data, S, W, Constant<Color>, O> {
            data: self.data,
            orientation: self.orientation,
            boundaries: self.boundaries,
            scale: self.scale,
            width: self.width,
            color: Constant(color),
            opacity: self.opacity,
        }
    }

    pub fn opacity(self, opacity: f32) -> Grid<Data, S, W, C, Constant<f32>> {
        Grid::<Data, S, W, C, Constant<f32>> {
            data: self.data,
            orientation: self.orientation,
            boundaries: self.boundaries,
            scale: self.scale,
            width: self.width,
            color: self.color,
            opacity: Constant(opacity),
        }
    }

    pub fn draw<D: Draw>(&self, drawer: &mut D, values: &[Data]) {
        for value in values.iter() {
            let scaled = self.scale.generate(value);
            drawer.line(LineProperties {
                start: self.orientation.apply(scaled, self.boundaries[0]),
                end: self.orientation.apply(scaled, self.boundaries[1]),
                color: self.color.generate(value),
                width: self.width.generate(value),
                opacity: self.opacity.generate(value),
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Grid;
    use crate::chromatic::Color;
    use crate::draw::{Draw, LineProperties, TextProperties};
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
        Grid::vertical(margin_top, height - margin_bottom, |x| scale.apply(*x))
            .width_with(|x| x / 50.)
            .color_with(|x| Color([x / 50.; 3]))
            .opacity_with(|x| x / 50.)
            .draw(&mut drawer, &values);

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
