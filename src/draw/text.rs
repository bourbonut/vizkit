use std::marker::PhantomData;

use super::{Draw, TextProperties};
use crate::{
    draw::TextAttrs,
    generator::{Constant, Function, Generator},
};

/// It distributes text in two directions jointly.
pub struct Text<Data, ProjectionX, ProjectionY>
where
    ProjectionX: Generator<Data, Output = f32>,
    ProjectionY: Generator<Data, Output = f32>,
{
    projection_x: ProjectionX,
    projection_y: ProjectionY,
    marker: PhantomData<Data>,
}

impl<Data, ProjectionX, ProjectionY>
    Text<Data, Function<ProjectionX, Data, f32>, Function<ProjectionY, Data, f32>>
where
    ProjectionX: Fn(&Data) -> f32,
    ProjectionY: Fn(&Data) -> f32,
{
    pub fn new(projection_x: ProjectionX, projection_y: ProjectionY) -> Self {
        Text {
            projection_x: Function::new(projection_x),
            projection_y: Function::new(projection_y),
            marker: PhantomData,
        }
    }
}

impl<Data, ProjectionY> Text<Data, Constant<f32>, Function<ProjectionY, Data, f32>>
where
    ProjectionY: Fn(&Data) -> f32,
{
    /// Creates a vertical text distribution drawer.
    pub fn vertical(x_value: f32, projection_y: ProjectionY) -> Self {
        Text {
            projection_x: Constant(x_value),
            projection_y: Function::new(projection_y),
            marker: PhantomData,
        }
    }
}

impl<Data, ProjectionX> Text<Data, Function<ProjectionX, Data, f32>, Constant<f32>>
where
    ProjectionX: Fn(&Data) -> f32,
{
    /// Creates an horizontal text distribution drawer.
    pub fn horizontal(
        projection_x: ProjectionX,
        y_value: f32,
    ) -> Text<Data, Function<ProjectionX, Data, f32>, Constant<f32>> {
        Text {
            projection_x: Function::new(projection_x),
            projection_y: Constant(y_value),
            marker: PhantomData,
        }
    }
}

impl<Data, ProjectionX, ProjectionY> Text<Data, ProjectionX, ProjectionY>
where
    ProjectionX: Generator<Data, Output = f32>,
    ProjectionY: Generator<Data, Output = f32>,
{
    /// Draws text on X-values and Y-values by applying the scaler functions respectively.
    pub fn draw<D: Draw>(&self, drawer: &mut D, values: &[Data], text_attrbs: &TextAttrs<Data>) {
        for value in values.iter() {
            let x_projected = self.projection_x.generate(value);
            let y_projected = self.projection_y.generate(value);
            drawer.text(TextProperties {
                position: [x_projected, y_projected],
                content: (text_attrbs.formatter)(value),
                color: (text_attrbs.color)(value),
                align_x: text_attrbs.align_x.clone(),
                align_y: text_attrbs.align_y.clone(),
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Text;
    use crate::chromatic::{Color, Rainbow};
    use crate::draw::{Draw, LineProperties, TextAttrs, TextProperties};
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
        let color = color_scale.clone();
        Text::new(
            |pair: &Pair| x_scale.apply(pair.x),
            |pair: &Pair| y_scale.apply(pair.y),
        )
        .draw(
            &mut drawer,
            &pairs,
            &TextAttrs::new(|pair: &Pair| (pair.x * pair.y).to_string())
                .color_with(move |pair| color.apply(pair.y)),
        );

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
        Text::horizontal(|x| scale.apply(*x), height - margin_bottom).draw(
            &mut drawer,
            &values,
            &TextAttrs::new(|x: &f32| (*x / 50.).to_string()).color_with(|x| Color([x / 50.; 3])),
        );

        assert_eq!(drawer.texts.len(), values.len());

        for (text, x) in drawer.texts.iter().zip(values.iter()) {
            let scaled = scale.apply(*x);
            assert_eq!(text.position, [scaled, height - margin_bottom]);
            assert_eq!(text.content, (x / 50.).to_string());
            assert_eq!(text.color.0, [x / 50.; 3]);
        }
    }
}
