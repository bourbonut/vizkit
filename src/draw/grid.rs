use super::{Draw, LineProperties};
use std::marker::PhantomData;

use crate::{
    chromatic::Color,
    scale::{ScaleContinuous, Tick, Transformer},
};

pub trait Direction {
    fn direction(coord1: f32, coord2: f32) -> [f32; 2];
}

struct Vectical;
struct Horizontal;

impl Direction for Vectical {
    fn direction(coord1: f32, coord2: f32) -> [f32; 2] {
        [coord1, coord2]
    }
}

impl Direction for Horizontal {
    fn direction(coord1: f32, coord2: f32) -> [f32; 2] {
        [coord2, coord1]
    }
}

pub trait Generator {
    type Output;
    fn generate(&self, value: f32) -> Self::Output;
}

struct Constant<T>(pub T);

impl<T: Clone> Generator for Constant<T> {
    type Output = T;
    fn generate(&self, _: f32) -> Self::Output {
        self.0.clone()
    }
}

struct Function<F, T>(pub F)
where
    F: Fn(f32) -> T;

impl<F, T> Generator for Function<F, T>
where
    F: Fn(f32) -> T,
{
    type Output = T;
    fn generate(&self, value: f32) -> Self::Output {
        (self.0)(value)
    }
}

pub struct Grid<
    D: Direction,
    W: Generator<Output = f32>,
    C: Generator<Output = Color>,
    O: Generator<Output = f32>,
> {
    marker: PhantomData<D>,
    boundaries: [f32; 2],
    width: W,
    color: C,
    opacity: O,
}

impl Grid<Vectical, Constant<f32>, Constant<Color>, Constant<f32>> {
    pub fn vertical(top: f32, down: f32) -> Self {
        Self {
            marker: PhantomData,
            boundaries: [top, down],
            width: Constant(1.),
            color: Constant(Color::default()),
            opacity: Constant(1.),
        }
    }
}

impl Grid<Horizontal, Constant<f32>, Constant<Color>, Constant<f32>> {
    pub fn horizontal(left: f32, right: f32) -> Self {
        Self {
            marker: PhantomData,
            boundaries: [left, right],
            width: Constant(1.),
            color: Constant(Color::default()),
            opacity: Constant(1.),
        }
    }
}

impl<D: Direction, C: Generator<Output = Color>, O: Generator<Output = f32>>
    Grid<D, Constant<f32>, C, O>
{
    pub fn width_with<F>(self, width_fn: F) -> Grid<D, Function<F, f32>, C, O>
    where
        F: Fn(f32) -> f32,
    {
        Grid::<D, Function<F, f32>, C, O> {
            marker: self.marker,
            boundaries: self.boundaries,
            width: Function(width_fn),
            color: self.color,
            opacity: self.opacity,
        }
    }
}

impl<D: Direction, W: Generator<Output = f32>, O: Generator<Output = f32>>
    Grid<D, W, Constant<Color>, O>
{
    pub fn color_with<F>(self, color_fn: F) -> Grid<D, W, Function<F, Color>, O>
    where
        F: Fn(f32) -> Color,
    {
        Grid::<D, W, Function<F, Color>, O> {
            marker: self.marker,
            boundaries: self.boundaries,
            width: self.width,
            color: Function(color_fn),
            opacity: self.opacity,
        }
    }
}

impl<D: Direction, W: Generator<Output = f32>, C: Generator<Output = Color>>
    Grid<D, W, C, Constant<f32>>
{
    pub fn opacity_with<F>(self, opacity_fn: F) -> Grid<D, W, C, Function<F, f32>>
    where
        F: Fn(f32) -> f32,
    {
        Grid::<D, W, C, Function<F, f32>> {
            marker: self.marker,
            boundaries: self.boundaries,
            width: self.width,
            color: self.color,
            opacity: Function(opacity_fn),
        }
    }
}

impl<D: Direction, C: Generator<Output = Color>, O: Generator<Output = f32>>
    Grid<D, Constant<f32>, C, O>
{
    pub fn width(self, width: f32) -> Self {
        Self {
            width: Constant(width),
            ..self
        }
    }
}

impl<D: Direction, W: Generator<Output = f32>, O: Generator<Output = f32>>
    Grid<D, W, Constant<Color>, O>
{
    pub fn color(self, color: Color) -> Self {
        Self {
            color: Constant(color),
            ..self
        }
    }
}

impl<D: Direction, W: Generator<Output = f32>, C: Generator<Output = Color>>
    Grid<D, W, C, Constant<f32>>
{
    pub fn opacity(self, opacity: f32) -> Self {
        Self {
            opacity: Constant(opacity),
            ..self
        }
    }
}

impl<
    Dir: Direction,
    W: Generator<Output = f32>,
    C: Generator<Output = Color>,
    O: Generator<Output = f32>,
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
