use super::{TextAttrs, TextProperties};

/// Creates an iterator of [`TextProperties`] for drawing text.
pub fn text_iter<Data>(
    values: &[Data],
    x: impl Fn(&Data) -> f32,
    y: impl Fn(&Data) -> f32,
    text_attrs: impl Fn(&Data) -> TextAttrs,
) -> impl Iterator<Item = TextProperties> {
    values.iter().map(move |value| {
        let text_values = (text_attrs)(value);
        TextProperties {
            position: [(x)(value), (y)(value)],
            content: text_values.content,
            fill_color: text_values.fill_color,
            font_size: text_values.font_size,
            align_x: text_values.align_x,
            align_y: text_values.align_y,
        }
    })
}

#[cfg(test)]
mod tests {
    use super::text_iter;
    use crate::chromatic::{Color, Rainbow};
    use crate::draw::{TextAttrs, TextProperties};
    use crate::scale::{Axis, ScaleColor, ScaleContinuous};

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

        let color = color_scale.clone();
        let texts: Vec<TextProperties> = text_iter(
            &pairs,
            |pair| x_scale.tick_position(pair.x),
            |pair| y_scale.tick_position(pair.y),
            |pair| TextAttrs {
                content: (pair.x * pair.y).to_string(),
                fill_color: color.apply(pair.y),
                ..Default::default()
            },
        )
        .collect();

        assert_eq!(texts.len(), y_values.len());

        for (text, (x, y)) in texts.iter().zip(x_values.iter().zip(y_values.iter())) {
            let x_scaled = x_scale.tick_position(*x);
            let y_scaled = y_scale.tick_position(*y);
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

        let texts: Vec<TextProperties> = text_iter(
            &values,
            |x| scale.tick_position(*x),
            |_| height - margin_bottom,
            |x| TextAttrs {
                content: (*x / 50.).to_string(),
                fill_color: Color([x / 50.; 3]),
                ..Default::default()
            },
        )
        .collect();

        assert_eq!(texts.len(), values.len());

        for (text, x) in texts.iter().zip(values.iter()) {
            let scaled = scale.tick_position(*x);
            assert_eq!(text.position, [scaled, height - margin_bottom]);
            assert_eq!(text.content, (x / 50.).to_string());
            assert_eq!(text.fill_color.0, [x / 50.; 3]);
        }
    }
}
