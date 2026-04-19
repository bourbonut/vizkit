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
    axis_options: &AxisOptions,
    line_attrbs: &LineAttrs<f32>,
    text_attrbs: &TextAttrs<f32>,
) {
    axis(
        drawer,
        scaler,
        y,
        Orientation::Same,
        -1.,
        Alignment::Center,
        Alignment::End,
        axis_options,
        line_attrbs,
        text_attrbs,
    );
}

pub fn axis_right<D: Draw + ?Sized, T: Transformer + Tick>(
    drawer: &mut D,
    scaler: &ScaleContinuous<T>,
    x: f32,
    axis_options: &AxisOptions,
    line_attrbs: &LineAttrs<f32>,
    text_attrbs: &TextAttrs<f32>,
) {
    axis(
        drawer,
        scaler,
        x,
        Orientation::Flip,
        1.,
        Alignment::Start,
        Alignment::Center,
        axis_options,
        line_attrbs,
        text_attrbs,
    );
}

pub fn axis_bottom<D: Draw + ?Sized, T: Transformer + Tick>(
    drawer: &mut D,
    scaler: &ScaleContinuous<T>,
    y: f32,
    axis_options: &AxisOptions,
    line_attrbs: &LineAttrs<f32>,
    text_attrbs: &TextAttrs<f32>,
) {
    axis(
        drawer,
        scaler,
        y,
        Orientation::Same,
        1.,
        Alignment::Center,
        Alignment::Start,
        axis_options,
        line_attrbs,
        text_attrbs,
    );
}

pub fn axis_left<D: Draw + ?Sized, T: Transformer + Tick>(
    drawer: &mut D,
    scaler: &ScaleContinuous<T>,
    x: f32,
    axis_options: &AxisOptions,
    line_attrbs: &LineAttrs<f32>,
    text_attrbs: &TextAttrs<f32>,
) {
    axis(
        drawer,
        scaler,
        x,
        Orientation::Flip,
        -1.,
        Alignment::End,
        Alignment::Center,
        axis_options,
        line_attrbs,
        text_attrbs,
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
    axis_options: &AxisOptions,
    line_attrbs: &LineAttrs<f32>,
    text_attrbs: &TextAttrs<f32>,
) {
    for tick_value in scaler.ticks(axis_options.count) {
        let tick_coord = scaler.apply(tick_value);
        drawer.draw_line(LineProperties {
            start: orientation.apply(tick_coord, at),
            end: orientation.apply(tick_coord, at + direction * axis_options.tick_size),
            stroke_color: (line_attrbs.stroke_color)(&tick_value),
            stroke_width: (line_attrbs.stroke_width)(&tick_value),
            stroke_opacity: (line_attrbs.stroke_opacity)(&tick_value),
        });
        drawer.draw_text(TextProperties {
            position: orientation.apply(
                tick_coord,
                at + direction * (axis_options.tick_size + axis_options.offset),
            ),
            content: (text_attrbs.formatter)(&tick_value),
            fill_color: (text_attrbs.fill_color)(&tick_value),
            font_size: text_attrbs.font_size,
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

    #[test]
    fn test_axis_bottom() {
        let height = 100.;
        let width = 400.;
        let xmax = 50.;
        let scale = ScaleContinuous::linear()
            .domain([0., xmax])
            .range([0., width]);

        let mut drawer = Drawer::default();
        drawer.axis_bottom(
            &scale,
            height,
            &AxisOptions::default(),
            &LineAttrs::default(),
            &TextAttrs::new(|x: &f32| x.to_string()),
        );

        for line in drawer.lines.iter() {
            assert_eq!(line.start[0], line.end[0]);
            assert_eq!(line.start[1], height);
            assert_eq!(line.end[1], height + 7.5);
        }

        let scale_ticks = scale
            .ticks(None)
            .into_iter()
            .map(|tick| scale.apply(tick))
            .collect::<Vec<f32>>();

        assert_eq!(
            drawer
                .lines
                .iter()
                .map(|line| line.start[0])
                .collect::<Vec<f32>>(),
            scale_ticks,
        );

        for text in drawer.texts.iter() {
            assert_eq!(text.position[1], height + 7.5 + 0.5);
        }

        assert_eq!(
            drawer
                .texts
                .iter()
                .map(|text| text.position[0])
                .collect::<Vec<f32>>(),
            scale_ticks,
        );

        assert_eq!(
            drawer
                .texts
                .iter()
                .map(|text| text.content.clone())
                .collect::<Vec<String>>(),
            scale
                .ticks(None)
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<String>>()
        );
    }

    #[test]
    fn test_axis_top() {
        let margin_top = 10.;
        let width = 400.;
        let xmax = 50.;
        let scale = ScaleContinuous::linear()
            .domain([0., xmax])
            .range([0., width]);

        let mut drawer = Drawer::default();
        drawer.axis_top(
            &scale,
            margin_top,
            &AxisOptions::default(),
            &LineAttrs::default(),
            &TextAttrs::new(|x: &f32| x.to_string()),
        );

        for line in drawer.lines.iter() {
            assert_eq!(line.start[0], line.end[0]);
            assert_eq!(line.start[1], margin_top);
            assert_eq!(line.end[1], margin_top - 7.5);
        }

        let scale_ticks = scale
            .ticks(None)
            .into_iter()
            .map(|tick| scale.apply(tick))
            .collect::<Vec<f32>>();

        assert_eq!(
            drawer
                .lines
                .iter()
                .map(|line| line.start[0])
                .collect::<Vec<f32>>(),
            scale_ticks,
        );

        for text in drawer.texts.iter() {
            assert_eq!(text.position[1], margin_top - 7.5 - 0.5);
        }

        assert_eq!(
            drawer
                .texts
                .iter()
                .map(|text| text.position[0])
                .collect::<Vec<f32>>(),
            scale_ticks,
        );

        assert_eq!(
            drawer
                .texts
                .iter()
                .map(|text| text.content.clone())
                .collect::<Vec<String>>(),
            scale
                .ticks(None)
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<String>>()
        );
    }

    #[test]
    fn test_axis_left() {
        let height = 100.;
        let margin_left = 10.;
        let ymax = 50.;
        let scale = ScaleContinuous::linear()
            .domain([0., ymax])
            .range([height, 0.]);

        let mut drawer = Drawer::default();
        drawer.axis_left(
            &scale,
            margin_left,
            &AxisOptions::default(),
            &LineAttrs::default(),
            &TextAttrs::new(|x: &f32| x.to_string()),
        );

        for line in drawer.lines.iter() {
            assert_eq!(line.start[0], margin_left);
            assert_eq!(line.end[0], margin_left - 7.5);
            assert_eq!(line.start[1], line.end[1]);
        }

        let scale_ticks = scale
            .ticks(None)
            .into_iter()
            .map(|tick| scale.apply(tick))
            .collect::<Vec<f32>>();

        assert_eq!(
            drawer
                .lines
                .iter()
                .map(|line| line.start[1])
                .collect::<Vec<f32>>(),
            scale_ticks,
        );

        for text in drawer.texts.iter() {
            assert_eq!(text.position[0], margin_left - 7.5 - 0.5);
        }

        assert_eq!(
            drawer
                .texts
                .iter()
                .map(|text| text.position[1])
                .collect::<Vec<f32>>(),
            scale_ticks,
        );

        assert_eq!(
            drawer
                .texts
                .iter()
                .map(|text| text.content.clone())
                .collect::<Vec<String>>(),
            scale
                .ticks(None)
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<String>>()
        );
    }

    #[test]
    fn test_axis_right() {
        let height = 100.;
        let width = 400.;
        let ymax = 50.;
        let scale = ScaleContinuous::linear()
            .domain([0., ymax])
            .range([height, 0.]);

        let mut drawer = Drawer::default();
        drawer.axis_right(
            &scale,
            width,
            &AxisOptions::default(),
            &LineAttrs::default(),
            &TextAttrs::new(|x: &f32| x.to_string()),
        );

        for line in drawer.lines.iter() {
            assert_eq!(line.start[0], width);
            assert_eq!(line.end[0], width + 7.5);
            assert_eq!(line.start[1], line.end[1]);
        }

        let scale_ticks = scale
            .ticks(None)
            .into_iter()
            .map(|tick| scale.apply(tick))
            .collect::<Vec<f32>>();

        assert_eq!(
            drawer
                .lines
                .iter()
                .map(|line| line.start[1])
                .collect::<Vec<f32>>(),
            scale_ticks,
        );

        for text in drawer.texts.iter() {
            assert_eq!(text.position[0], width + 7.5 + 0.5);
        }

        assert_eq!(
            drawer
                .texts
                .iter()
                .map(|text| text.position[1])
                .collect::<Vec<f32>>(),
            scale_ticks,
        );

        assert_eq!(
            drawer
                .texts
                .iter()
                .map(|text| text.content.clone())
                .collect::<Vec<String>>(),
            scale
                .ticks(None)
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<String>>()
        );
    }
}
