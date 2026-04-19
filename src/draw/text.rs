use super::{Draw, TextAttrs, TextProperties};

pub fn text<Data, D: Draw + ?Sized>(
    drawer: &mut D,
    values: &[Data],
    x: impl Fn(&Data) -> f32,
    y: impl Fn(&Data) -> f32,
    text_attrs: &TextAttrs<Data>,
) {
    for value in values.iter() {
        let x_projected = (x)(value);
        let y_projected = (y)(value);
        drawer.draw_text(TextProperties {
            position: [x_projected, y_projected],
            content: (text_attrs.formatter)(value),
            fill_color: (text_attrs.fill_color)(value),
            font_size: text_attrs.font_size,
            align_x: text_attrs.align_x.clone(),
            align_y: text_attrs.align_y.clone(),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::chromatic::{Color, Rainbow};
    use crate::draw::{CircleProperties, Draw, LineProperties, TextAttrs, TextProperties};
    use crate::scale::{ScaleColor, ScaleContinuous};

    #[derive(Default)]
    struct Drawer {
        texts: Vec<TextProperties>,
    }

    impl<'a> Draw for Drawer {
        fn draw_line(&mut self, _: LineProperties) {
            todo!()
        }

        fn draw_text(&mut self, text: TextProperties) {
            self.texts.push(text);
        }

        fn draw_circle(&mut self, _: CircleProperties) {
            todo!()
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
        drawer.text(
            &pairs,
            |pair| x_scale.apply(pair.x),
            |pair| y_scale.apply(pair.y),
            &TextAttrs::new(|pair: &Pair| (pair.x * pair.y).to_string())
                .fill_color_with(move |pair| color.apply(pair.y)),
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
            assert_eq!(text.fill_color.0, color_scale.apply::<[f32; 3]>(*y));
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
        drawer.text_horizontal(
            &values,
            |x| scale.apply(*x),
            height - margin_bottom,
            &TextAttrs::new(|x: &f32| (*x / 50.).to_string())
                .fill_color_with(|x| Color([x / 50.; 3])),
        );

        assert_eq!(drawer.texts.len(), values.len());

        for (text, x) in drawer.texts.iter().zip(values.iter()) {
            let scaled = scale.apply(*x);
            assert_eq!(text.position, [scaled, height - margin_bottom]);
            assert_eq!(text.content, (x / 50.).to_string());
            assert_eq!(text.fill_color.0, [x / 50.; 3]);
        }
    }
}
