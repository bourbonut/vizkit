use super::{Draw, TextAttrs, TextProperties};

pub fn text<Data, D: Draw + ?Sized>(
    drawer: &mut D,
    values: &[Data],
    x: impl Fn(&Data) -> f32,
    y: impl Fn(&Data) -> f32,
    text_attrs: impl Fn(&Data) -> TextAttrs,
) {
    for value in values.iter() {
        let text_values = (text_attrs)(value);
        drawer.draw_text(TextProperties {
            position: [(x)(value), (y)(value)],
            content: text_values.content,
            fill_color: text_values.fill_color,
            font_size: text_values.font_size,
            align_x: text_values.align_x,
            align_y: text_values.align_y,
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
            |pair| TextAttrs {
                content: (pair.x * pair.y).to_string(),
                fill_color: color.apply(pair.y),
                ..Default::default()
            },
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
            |x| TextAttrs {
                content: (*x / 50.).to_string(),
                fill_color: Color([x / 50.; 3]),
                ..Default::default()
            },
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
