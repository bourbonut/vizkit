use super::{Direction, Draw, Horizontal, LineProperties, Vertical};
use crate::generator::{Constant1D, Function1D, Generator1D};
use std::marker::PhantomData;

use crate::{
    chromatic::Color,
    scale::{ScaleContinuous, Tick, Transformer},
};

pub struct Grid<
    D: Direction,
    W: Generator1D<Output = f32>,
    C: Generator1D<Output = Color>,
    O: Generator1D<Output = f32>,
> {
    marker: PhantomData<D>,
    boundaries: [f32; 2],
    width: W,
    color: C,
    opacity: O,
}

impl Grid<Vertical, Constant1D<f32>, Constant1D<Color>, Constant1D<f32>> {
    pub fn vertical(top: f32, down: f32) -> Self {
        Self {
            marker: PhantomData,
            boundaries: [top, down],
            width: Constant1D(1.),
            color: Constant1D(Color::default()),
            opacity: Constant1D(1.),
        }
    }
}

impl Grid<Horizontal, Constant1D<f32>, Constant1D<Color>, Constant1D<f32>> {
    pub fn horizontal(left: f32, right: f32) -> Self {
        Self {
            marker: PhantomData,
            boundaries: [left, right],
            width: Constant1D(1.),
            color: Constant1D(Color::default()),
            opacity: Constant1D(1.),
        }
    }
}

impl<D: Direction, C: Generator1D<Output = Color>, O: Generator1D<Output = f32>>
    Grid<D, Constant1D<f32>, C, O>
{
    pub fn width_with<F>(self, width_fn: F) -> Grid<D, Function1D<F, f32>, C, O>
    where
        F: Fn(f32) -> f32,
    {
        Grid::<D, Function1D<F, f32>, C, O> {
            marker: self.marker,
            boundaries: self.boundaries,
            width: Function1D(width_fn),
            color: self.color,
            opacity: self.opacity,
        }
    }
}

impl<D: Direction, W: Generator1D<Output = f32>, O: Generator1D<Output = f32>>
    Grid<D, W, Constant1D<Color>, O>
{
    pub fn color_with<F>(self, color_fn: F) -> Grid<D, W, Function1D<F, Color>, O>
    where
        F: Fn(f32) -> Color,
    {
        Grid::<D, W, Function1D<F, Color>, O> {
            marker: self.marker,
            boundaries: self.boundaries,
            width: self.width,
            color: Function1D(color_fn),
            opacity: self.opacity,
        }
    }
}

impl<D: Direction, W: Generator1D<Output = f32>, C: Generator1D<Output = Color>>
    Grid<D, W, C, Constant1D<f32>>
{
    pub fn opacity_with<F>(self, opacity_fn: F) -> Grid<D, W, C, Function1D<F, f32>>
    where
        F: Fn(f32) -> f32,
    {
        Grid::<D, W, C, Function1D<F, f32>> {
            marker: self.marker,
            boundaries: self.boundaries,
            width: self.width,
            color: self.color,
            opacity: Function1D(opacity_fn),
        }
    }
}

impl<D: Direction, C: Generator1D<Output = Color>, O: Generator1D<Output = f32>>
    Grid<D, Constant1D<f32>, C, O>
{
    pub fn width(self, width: f32) -> Self {
        Self {
            width: Constant1D(width),
            ..self
        }
    }
}

impl<D: Direction, W: Generator1D<Output = f32>, O: Generator1D<Output = f32>>
    Grid<D, W, Constant1D<Color>, O>
{
    pub fn color(self, color: Color) -> Self {
        Self {
            color: Constant1D(color),
            ..self
        }
    }
}

impl<D: Direction, W: Generator1D<Output = f32>, C: Generator1D<Output = Color>>
    Grid<D, W, C, Constant1D<f32>>
{
    pub fn opacity(self, opacity: f32) -> Self {
        Self {
            opacity: Constant1D(opacity),
            ..self
        }
    }
}

impl<
    Dir: Direction,
    W: Generator1D<Output = f32>,
    C: Generator1D<Output = Color>,
    O: Generator1D<Output = f32>,
> Grid<Dir, W, C, O>
{
    pub fn draw<D: Draw, T: Transformer + Tick>(
        &self,
        drawer: &mut D,
        scale: &ScaleContinuous<T>,
        values: &[f32],
    ) {
        for &value in values.iter() {
            let scaled = scale.apply(value);
            drawer.line(LineProperties {
                start: Dir::direction(scaled, self.boundaries[0]),
                end: Dir::direction(scaled, self.boundaries[1]),
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
        Grid::vertical(margin_top, height - margin_bottom)
            .width_with(|x| x / 50.)
            .color_with(|x| Color([x / 50.; 3]))
            .opacity_with(|x| x / 50.)
            .draw(&mut drawer, &scale, &values);

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
