use std::marker::PhantomData;

use super::{Alignment, Draw, Orientation, TextProperties};
use crate::{
    chromatic::Color,
    generator::{Constant, Function, Generator},
};

/// It distributes text vertically or horizontally based on a specified value.
pub struct Text1D<
    Data,
    S: Generator<Data, Output = f32>,
    Fmt: Generator<Data, Output = String>,
    C: Generator<Data, Output = Color>,
> {
    data: PhantomData<Data>,
    orientation: Orientation,
    scale: S,
    at: f32,
    format: Fmt,
    color: C,
}

impl<S, Fmt, Data>
    Text1D<Data, Function<S, Data, f32>, Function<Fmt, Data, String>, Constant<Color>>
where
    S: Fn(&Data) -> f32,
    Fmt: Fn(&Data) -> String,
{
    /// Creates a vertical text distribution drawer.
    pub fn vertical(x_value: f32, scale_fn: S, format_fn: Fmt) -> Self {
        Self {
            data: PhantomData,
            orientation: Orientation::Flip,
            scale: Function::new(scale_fn),
            at: x_value,
            format: Function::new(format_fn),
            color: Constant(Color::default()),
        }
    }

    /// Creates an horizontal text distribution drawer.
    pub fn horizontal(y_value: f32, scale_fn: S, format_fn: Fmt) -> Self {
        Self {
            data: PhantomData,
            orientation: Orientation::Same,
            scale: Function::new(scale_fn),
            at: y_value,
            format: Function::new(format_fn),
            color: Constant(Color::default()),
        }
    }
}

impl<
    Data,
    S: Generator<Data, Output = f32>,
    Fmt: Generator<Data, Output = String>,
    C: Generator<Data, Output = Color>,
> Text1D<Data, S, Fmt, C>
{
    /// Sets a constant color used as the color of the text.
    pub fn color(self, color: Color) -> Text1D<Data, S, Fmt, Constant<Color>> {
        Text1D::<Data, S, Fmt, Constant<Color>> {
            data: self.data,
            orientation: self.orientation,
            scale: self.scale,
            at: self.at,
            format: self.format,
            color: Constant(color),
        }
    }

    /// Sets a function for generating the color of the text given the distributed values.
    pub fn color_with<F>(self, color_fn: F) -> Text1D<Data, S, Fmt, Function<F, Data, Color>>
    where
        F: Fn(&Data) -> Color,
    {
        Text1D::<Data, S, Fmt, Function<F, Data, Color>> {
            data: self.data,
            orientation: self.orientation,
            scale: self.scale,
            at: self.at,
            format: self.format,
            color: Function::new(color_fn),
        }
    }

    /// Draws the text given the specified values.
    pub fn draw<D: Draw>(&self, drawer: &mut D, values: &[Data]) {
        for value in values.iter() {
            let scaled = self.scale.generate(value);
            drawer.text(TextProperties {
                position: self.orientation.apply(scaled, self.at),
                content: self.format.generate(value),
                color: self.color.generate(value),
                align_x: Alignment::Center,
                align_y: Alignment::Center,
            })
        }
    }
}

/// It distributes text in two directions jointly.
pub struct Text2D<
    Data,
    X: Generator<Data, Output = f32>,
    Y: Generator<Data, Output = f32>,
    Fmt: Generator<Data, Output = String>,
    C: Generator<Data, Output = Color>,
> {
    data: PhantomData<Data>,
    x_scale: X,
    y_scale: Y,
    format: Fmt,
    color: C,
}

/// By default, scalers are equivalent to identity transformations, only the Y-values are formatted
/// into string.
impl<Data, X, Y, Fmt>
    Text2D<
        Data,
        Function<X, Data, f32>,
        Function<Y, Data, f32>,
        Function<Fmt, Data, String>,
        Constant<Color>,
    >
where
    X: Fn(&Data) -> f32,
    Y: Fn(&Data) -> f32,
    Fmt: Fn(&Data) -> String,
{
    pub fn new(x_fn: X, y_fn: Y, format_fn: Fmt) -> Self {
        Self {
            data: PhantomData,
            x_scale: Function::new(x_fn),
            y_scale: Function::new(y_fn),
            format: Function::new(format_fn),
            color: Constant(Color::default()),
        }
    }
}

impl<
    Data,
    X: Generator<Data, Output = f32>,
    Y: Generator<Data, Output = f32>,
    Fmt: Generator<Data, Output = String>,
    C: Generator<Data, Output = Color>,
> Text2D<Data, X, Y, Fmt, C>
{
    /// Sets a constant color used as the color of the text.
    pub fn color(self, color: Color) -> Text2D<Data, X, Y, Fmt, Constant<Color>> {
        Text2D::<Data, X, Y, Fmt, Constant<Color>> {
            data: self.data,
            x_scale: self.x_scale,
            y_scale: self.y_scale,
            format: self.format,
            color: Constant(color),
        }
    }

    /// Sets a function for generating the color of the text based on X-values and Y-values.
    pub fn color_with<F>(self, color_fn: F) -> Text2D<Data, X, Y, Fmt, Function<F, Data, Color>>
    where
        F: Fn(&Data) -> Color,
    {
        Text2D::<Data, X, Y, Fmt, Function<F, Data, Color>> {
            data: self.data,
            x_scale: self.x_scale,
            y_scale: self.y_scale,
            format: self.format,
            color: Function::new(color_fn),
        }
    }

    /// Draws text on X-values and Y-values by applying the scaler functions respectively.
    pub fn draw<D: Draw>(&self, drawer: &mut D, values: &[Data]) {
        for value in values.iter() {
            let x_scaled = self.x_scale.generate(value);
            let y_scaled = self.y_scale.generate(value);
            drawer.text(TextProperties {
                position: [x_scaled, y_scaled],
                content: self.format.generate(value),
                color: self.color.generate(value),
                align_x: Alignment::Center,
                align_y: Alignment::Center,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Text1D, Text2D};
    use crate::chromatic::{Color, Rainbow};
    use crate::draw::{Draw, LineProperties, TextProperties};
    use crate::scale::{ScaleColor, ScaleContinuous};

    #[derive(Default)]
    struct Drawer {
        texts: Vec<TextProperties>,
    }

    impl<'a> Draw for Drawer {
        fn line(&mut self, _: LineProperties) {
            todo!()
        }

        fn text(&mut self, text: TextProperties) {
            self.texts.push(text);
        }
    }

    struct Pair {
        x: f32,
        y: f32,
    }

    #[test]
    fn test_text_2d() {
        let width = 400.;
        let height = 200.;
        let margin_top = 10.;
        let margin_bottom = 40.;
        let margin_left = 50.;
        let margin_right = 10.;

        let x_scale = ScaleContinuous::linear()
            .domain([0., 50.])
            .range([margin_left, width - margin_right]);
        let y_scale = ScaleContinuous::linear()
            .domain([0., 200.])
            .range([height - margin_bottom, margin_top]);
        let color_scale = ScaleColor::linear(Rainbow::default()).domain([0., 200.]);

        let x_values = x_scale.ticks(Some(10));
        let y_values = y_scale.ticks(Some(10));

        let pairs: Vec<Pair> = x_values
            .iter()
            .zip(y_values.iter())
            .map(|(&x, &y)| Pair { x, y })
            .collect();

        let mut drawer = Drawer::default();
        Text2D::new(
            |pair: &Pair| x_scale.apply(pair.x),
            |pair: &Pair| y_scale.apply(pair.y),
            |pair: &Pair| (pair.x * pair.y).to_string(),
        )
        .color_with(|pair| color_scale.apply(pair.y))
        .draw(&mut drawer, &pairs);

        assert_eq!(drawer.texts.len(), y_values.len());

        for (text, (x, y)) in drawer
            .texts
            .iter()
            .zip(x_values.iter().zip(y_values.iter()))
        {
            let x_scaled = x_scale.apply(*x);
            let y_scaled = y_scale.apply(*y);
            assert_eq!(text.position, [x_scaled, y_scaled]);
            assert_eq!(text.content, (x * y).to_string());
            assert_eq!(text.color.0, color_scale.apply::<[f32; 3]>(*y));
        }
    }

    #[test]
    fn test_text_1d() {
        let width = 400.;
        let height = 200.;
        let margin_bottom = 40.;
        let margin_left = 50.;
        let margin_right = 10.;

        let scale = ScaleContinuous::linear()
            .domain([0., 50.])
            .range([margin_left, width - margin_right]);

        let values = scale.ticks(None);

        let mut drawer = Drawer::default();
        Text1D::horizontal(
            height - margin_bottom,
            |x| scale.apply(*x),
            |x| (*x / 50.).to_string(),
        )
        .color_with(|x| Color([x / 50.; 3]))
        .draw(&mut drawer, &values);

        assert_eq!(drawer.texts.len(), values.len());

        for (text, x) in drawer.texts.iter().zip(values.iter()) {
            let scaled = scale.apply(*x);
            assert_eq!(text.position, [scaled, height - margin_bottom]);
            assert_eq!(text.content, (x / 50.).to_string());
            assert_eq!(text.color.0, [x / 50.; 3]);
        }
    }
}
