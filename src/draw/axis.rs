use super::{Alignment, Draw, LineAttrs, LineProperties, Orientation, TextAttrs, TextProperties};
use crate::scale::{ScaleContinuous, Tick, Transformer};

pub struct AxisOptions {
    pub tick_size: f32,
    pub offset: f32,
    pub count: Option<usize>,
}

impl Default for AxisOptions {
    fn default() -> Self {
        Self {
            tick_size: 7.5,
            offset: 0.5,
            count: None,
        }
    }
}

pub fn axis_top<D: Draw + ?Sized, T: Transformer + Tick>(
    drawer: &mut D,
    scaler: &ScaleContinuous<T>,
    y: f32,
    line_attrs: impl Fn(f32) -> LineAttrs,
    text_attrs: impl Fn(f32) -> TextAttrs,
    axis_options: &AxisOptions,
) {
    axis(
        drawer,
        scaler,
        y,
        Orientation::Same,
        -1.,
        Alignment::Center,
        Alignment::End,
        line_attrs,
        text_attrs,
        axis_options,
    );
}

pub fn axis_right<D: Draw + ?Sized, T: Transformer + Tick>(
    drawer: &mut D,
    scaler: &ScaleContinuous<T>,
    x: f32,
    line_attrs: impl Fn(f32) -> LineAttrs,
    text_attrs: impl Fn(f32) -> TextAttrs,
    axis_options: &AxisOptions,
) {
    axis(
        drawer,
        scaler,
        x,
        Orientation::Flip,
        1.,
        Alignment::Start,
        Alignment::Center,
        line_attrs,
        text_attrs,
        axis_options,
    );
}

pub fn axis_bottom<D: Draw + ?Sized, T: Transformer + Tick>(
    drawer: &mut D,
    scaler: &ScaleContinuous<T>,
    y: f32,
    line_attrs: impl Fn(f32) -> LineAttrs,
    text_attrs: impl Fn(f32) -> TextAttrs,
    axis_options: &AxisOptions,
) {
    axis(
        drawer,
        scaler,
        y,
        Orientation::Same,
        1.,
        Alignment::Center,
        Alignment::Start,
        line_attrs,
        text_attrs,
        axis_options,
    );
}

pub fn axis_left<D: Draw + ?Sized, T: Transformer + Tick>(
    drawer: &mut D,
    scaler: &ScaleContinuous<T>,
    x: f32,
    line_attrs: impl Fn(f32) -> LineAttrs,
    text_attrs: impl Fn(f32) -> TextAttrs,
    axis_options: &AxisOptions,
) {
    axis(
        drawer,
        scaler,
        x,
        Orientation::Flip,
        -1.,
        Alignment::End,
        Alignment::Center,
        line_attrs,
        text_attrs,
        axis_options,
    );
}

fn axis<D: Draw + ?Sized, T: Transformer + Tick>(
    drawer: &mut D,
    scaler: &ScaleContinuous<T>,
    at: f32,
    orientation: Orientation,
    direction: f32,
    align_x: Alignment,
    align_y: Alignment,
    line_attrs: impl Fn(f32) -> LineAttrs,
    text_attrs: impl Fn(f32) -> TextAttrs,
    axis_options: &AxisOptions,
) {
    for tick_value in scaler.ticks(axis_options.count) {
        let tick_coord = scaler.apply(tick_value);
        let line_values = (line_attrs)(tick_value);
        let text_values = (text_attrs)(tick_value);
        drawer.draw_line(LineProperties {
            start: orientation.apply(tick_coord, at),
            end: orientation.apply(tick_coord, at + direction * axis_options.tick_size),
            stroke_color: line_values.stroke_color,
            stroke_width: line_values.stroke_width,
            stroke_opacity: line_values.stroke_opacity,
        });
        drawer.draw_text(TextProperties {
            position: orientation.apply(
                tick_coord,
                at + direction * (axis_options.tick_size + axis_options.offset),
            ),
            content: text_values.content,
            fill_color: text_values.fill_color,
            font_size: text_values.font_size,
            align_x: align_x.clone(),
            align_y: align_y.clone(),
        });
    }
}

#[cfg(test)]
mod tests {
    use crate::draw::{
        AxisOptions, CircleProperties, Draw, LineAttrs, LineProperties, TextAttrs, TextProperties,
    };
    use crate::scale::ScaleContinuous;
    use rstest::rstest;

    #[derive(Default)]
    struct Drawer {
        lines: Vec<LineProperties>,
        texts: Vec<TextProperties>,
    }

    impl Draw for Drawer {
        fn draw_line(&mut self, line: LineProperties) {
            self.lines.push(line);
        }

        fn draw_text(&mut self, text: TextProperties) {
            self.texts.push(text);
        }

        fn draw_circle(&mut self, _: CircleProperties) {
            todo!()
        }
    }

    const WIDTH: f32 = 400.;
    const HEIGHT: f32 = 100.;

    const MARGIN_LEFT: f32 = 10.;
    const MARGIN_TOP: f32 = 10.;

    const XMAX: f32 = 50.;
    const YMAX: f32 = 50.;

    #[rstest]
    #[case("bottom", 0, [0., XMAX], [0., WIDTH], HEIGHT, HEIGHT, HEIGHT + 7.5, HEIGHT + 7.5 + 0.5)]
    #[case("top", 0, [0., XMAX], [0., WIDTH], MARGIN_TOP, MARGIN_TOP, MARGIN_TOP - 7.5, MARGIN_TOP - 7.5 - 0.5)]
    #[case("left", 1, [0., YMAX], [HEIGHT, 0.], MARGIN_LEFT, MARGIN_LEFT, MARGIN_LEFT - 7.5, MARGIN_LEFT - 7.5 - 0.5)]
    #[case("right", 1, [0., YMAX], [HEIGHT, 0.], WIDTH, WIDTH, WIDTH + 7.5, WIDTH + 7.5 + 0.5)]
    fn test_axis(
        #[case] title: &str,
        #[case] index: usize,
        #[case] domain: [f32; 2],
        #[case] range: [f32; 2],
        #[case] at: f32,
        #[case] start: f32,
        #[case] end: f32,
        #[case] position: f32,
    ) {
        let mut drawer = Drawer::default();
        let scale = ScaleContinuous::linear().domain(domain).range(range);

        let line_fn = |_| LineAttrs::default();
        let text_fn = |x: f32| TextAttrs {
            content: x.to_string(),
            ..Default::default()
        };
        let options = AxisOptions::default();
        match title {
            "bottom" => drawer.axis_bottom(&scale, at, line_fn, text_fn, &options),
            "top" => drawer.axis_top(&scale, at, line_fn, text_fn, &options),
            "left" => drawer.axis_left(&scale, at, line_fn, text_fn, &options),
            "right" => drawer.axis_right(&scale, at, line_fn, text_fn, &options),
            _ => unreachable!(),
        }

        // Indices
        let a = index;
        let b = (index + 1) % 2;

        // Expected values
        let tick_fn = |&tick: &f32| scale.apply(tick);
        let scale_ticks: Vec<f32> = scale.ticks(None).iter().map(tick_fn).collect();
        let string_ticks: Vec<String> = scale.ticks(None).iter().map(ToString::to_string).collect();

        // Test line properties
        for (i, line) in drawer.lines.iter().enumerate() {
            assert_eq!(line.start[a], line.end[a], "{}", title);
            assert_eq!(line.start[b], start, "{}", title);
            assert_eq!(line.end[b], end, "{}", title);
            assert_eq!(line.start[a], scale_ticks[i], "{}", title);
        }

        // Test text properties
        for (i, text) in drawer.texts.iter().enumerate() {
            assert_eq!(text.position[b], position, "{}", title);
            assert_eq!(text.position[a], scale_ticks[i], "{}", title);
            assert_eq!(text.content, string_ticks[i], "{}", title);
        }
    }
}
