use super::{Draw, TextProperties};
use crate::{
    chromatic::Color,
    generator::{Constant1D, Constant2D, Function1D, Function2D, Generator1D, Generator2D},
};

pub struct Text1D<
    Fmt: Generator1D<Output = String>,
    C: Generator1D<Output = Color>,
    S: Generator1D<Output = f32>,
> {
    direction: Function2D<fn(f32, f32) -> [f32; 2], [f32; 2]>,
    scale: S,
    at: f32,
    format: Fmt,
    color: C,
}

impl
    Text1D<
        Function1D<fn(f32) -> String, String>,
        Constant1D<Color>,
        Function1D<fn(f32) -> f32, f32>,
    >
{
    pub fn vertical(at: f32) -> Self {
        Self {
            direction: Function2D(|x, y| [y, x]),
            scale: Function1D(|x| x),
            at,
            format: Function1D(|x: f32| x.to_string()),
            color: Constant1D(Color::default()),
        }
    }
}

impl
    Text1D<
        Function1D<fn(f32) -> String, String>,
        Constant1D<Color>,
        Function1D<fn(f32) -> f32, f32>,
    >
{
    pub fn horizontal(at: f32) -> Self {
        Self {
            direction: Function2D(|x, y| [x, y]),
            scale: Function1D(|x| x),
            at,
            format: Function1D(|x: f32| x.to_string()),
            color: Constant1D(Color::default()),
        }
    }
}

impl<
    Fmt: Generator1D<Output = String>,
    C: Generator1D<Output = Color>,
    S: Generator1D<Output = f32>,
> Text1D<Fmt, C, S>
{
    pub fn scale_with<F>(self, scale_fn: F) -> Text1D<Fmt, C, Function1D<F, f32>>
    where
        F: Fn(f32) -> f32,
    {
        Text1D::<Fmt, C, Function1D<F, f32>> {
            direction: self.direction,
            scale: Function1D(scale_fn),
            at: self.at,
            format: self.format,
            color: self.color,
        }
    }
}

impl<
    Fmt: Generator1D<Output = String>,
    C: Generator1D<Output = Color>,
    S: Generator1D<Output = f32>,
> Text1D<Fmt, C, S>
{
    pub fn format_with<F>(self, format_fn: F) -> Text1D<Function1D<F, String>, C, S>
    where
        F: Fn(f32) -> String,
    {
        Text1D::<Function1D<F, String>, C, S> {
            direction: self.direction,
            scale: self.scale,
            at: self.at,
            format: Function1D(format_fn),
            color: self.color,
        }
    }
}

impl<
    Fmt: Generator1D<Output = String>,
    C: Generator1D<Output = Color>,
    S: Generator1D<Output = f32>,
> Text1D<Fmt, C, S>
{
    pub fn color(self, color: Color) -> Text1D<Fmt, Constant1D<Color>, S> {
        Text1D::<Fmt, Constant1D<Color>, S> {
            direction: self.direction,
            scale: self.scale,
            at: self.at,
            format: self.format,
            color: Constant1D(color),
        }
    }
}

impl<
    Fmt: Generator1D<Output = String>,
    C: Generator1D<Output = Color>,
    S: Generator1D<Output = f32>,
> Text1D<Fmt, C, S>
{
    pub fn color_with<F>(self, color_fn: F) -> Text1D<Fmt, Function1D<F, Color>, S>
    where
        F: Fn(f32) -> Color,
    {
        Text1D::<Fmt, Function1D<F, Color>, S> {
            direction: self.direction,
            scale: self.scale,
            at: self.at,
            format: self.format,
            color: Function1D(color_fn),
        }
    }
}

impl<
    Fmt: Generator1D<Output = String>,
    C: Generator1D<Output = Color>,
    S: Generator1D<Output = f32>,
> Text1D<Fmt, C, S>
{
    pub fn draw<D: Draw>(&self, drawer: &mut D, values: &[f32]) {
        for &value in values.iter() {
            let scaled = self.scale.generate(value);
            drawer.text(TextProperties {
                position: self.direction.generate(scaled, self.at),
                content: self.format.generate(value),
                color: self.color.generate(value),
            })
        }
    }
}

pub struct Text<
    X: Generator1D<Output = f32>,
    Y: Generator1D<Output = f32>,
    Fmt: Generator2D<Output = String>,
    C: Generator2D<Output = Color>,
> {
    x_scale: X,
    y_scale: Y,
    format: Fmt,
    color: C,
}

impl
    Text<
        Function1D<fn(f32) -> f32, f32>,
        Function1D<fn(f32) -> f32, f32>,
        Function2D<fn(f32, f32) -> String, String>,
        Constant2D<Color>,
    >
{
    pub fn new() -> Self {
        Self {
            x_scale: Function1D(|x| x),
            y_scale: Function1D(|y| y),
            format: Function2D(|_: f32, y: f32| y.to_string()),
            color: Constant2D(Color::default()),
        }
    }
}

impl<
    X: Generator1D<Output = f32>,
    Y: Generator1D<Output = f32>,
    Fmt: Generator2D<Output = String>,
    C: Generator2D<Output = Color>,
> Text<X, Y, Fmt, C>
{
    pub fn x_scale_with<F>(self, x_scale_fn: F) -> Text<Function1D<F, f32>, Y, Fmt, C>
    where
        F: Fn(f32) -> f32,
    {
        Text::<Function1D<F, f32>, Y, Fmt, C> {
            x_scale: Function1D(x_scale_fn),
            y_scale: self.y_scale,
            format: self.format,
            color: self.color,
        }
    }
}

impl<
    X: Generator1D<Output = f32>,
    Y: Generator1D<Output = f32>,
    Fmt: Generator2D<Output = String>,
    C: Generator2D<Output = Color>,
> Text<X, Y, Fmt, C>
{
    pub fn y_scale_with<F>(self, y_scale_fn: F) -> Text<X, Function1D<F, f32>, Fmt, C>
    where
        F: Fn(f32) -> f32,
    {
        Text::<X, Function1D<F, f32>, Fmt, C> {
            x_scale: self.x_scale,
            y_scale: Function1D(y_scale_fn),
            format: self.format,
            color: self.color,
        }
    }
}

impl<
    X: Generator1D<Output = f32>,
    Y: Generator1D<Output = f32>,
    Fmt: Generator2D<Output = String>,
    C: Generator2D<Output = Color>,
> Text<X, Y, Fmt, C>
{
    pub fn format_with<F>(self, format_fn: F) -> Text<X, Y, Function2D<F, String>, C>
    where
        F: Fn(f32, f32) -> String,
    {
        Text::<X, Y, Function2D<F, String>, C> {
            x_scale: self.x_scale,
            y_scale: self.y_scale,
            format: Function2D(format_fn),
            color: self.color,
        }
    }
}

impl<
    X: Generator1D<Output = f32>,
    Y: Generator1D<Output = f32>,
    Fmt: Generator2D<Output = String>,
    C: Generator2D<Output = Color>,
> Text<X, Y, Fmt, C>
{
    pub fn color(self, color: Color) -> Text<X, Y, Fmt, Constant2D<Color>> {
        Text::<X, Y, Fmt, Constant2D<Color>> {
            x_scale: self.x_scale,
            y_scale: self.y_scale,
            format: self.format,
            color: Constant2D(color),
        }
    }
}

impl<
    X: Generator1D<Output = f32>,
    Y: Generator1D<Output = f32>,
    Fmt: Generator2D<Output = String>,
    C: Generator2D<Output = Color>,
> Text<X, Y, Fmt, C>
{
    pub fn color_with<F>(self, color_fn: F) -> Text<X, Y, Fmt, Function2D<F, Color>>
    where
        F: Fn(f32, f32) -> Color,
    {
        Text::<X, Y, Fmt, Function2D<F, Color>> {
            x_scale: self.x_scale,
            y_scale: self.y_scale,
            format: self.format,
            color: Function2D(color_fn),
        }
    }
}

impl<
    X: Generator1D<Output = f32>,
    Y: Generator1D<Output = f32>,
    Fmt: Generator2D<Output = String>,
    C: Generator2D<Output = Color>,
> Text<X, Y, Fmt, C>
{
    pub fn draw<D: Draw>(&self, drawer: &mut D, x_values: &[f32], y_values: &[f32]) {
        for (&x_value, &y_value) in x_values.iter().zip(y_values.iter()) {
            let x_scaled = self.x_scale.generate(x_value);
            let y_scaled = self.y_scale.generate(y_value);
            drawer.text(TextProperties {
                position: [x_scaled, y_scaled],
                content: self.format.generate(x_value, y_value),
                color: self.color.generate(x_value, y_value),
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Text, Text1D};
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

    #[test]
    fn test_text() {
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

        let mut drawer = Drawer::default();
        Text::new()
            .x_scale_with(|x| x_scale.apply(x))
            .y_scale_with(|y| y_scale.apply(y))
            .format_with(|x, y| (x * y).to_string())
            .color_with(|_, y| color_scale.apply(y))
            .draw(&mut drawer, &x_values, &y_values);

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
        Text1D::horizontal(height - margin_bottom)
            .scale_with(|x| scale.apply(x))
            .format_with(|x| (x / 50.).to_string())
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
