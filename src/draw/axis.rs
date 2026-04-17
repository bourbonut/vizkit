use super::{
    Alignment, Draw, LineAttribs, LineProperties, Orientation, TextAttrbs, TextProperties,
};
use crate::{
    chromatic::Color,
    generator::Generator,
    scale::{ScaleContinuous, Tick, Transformer},
};

pub struct Axis {
    orientation: Orientation,
    direction: f32,
    tick_size: f32,
    at: f32,
    offset: f32,
    count: Option<usize>,
    align_x: Alignment,
    align_y: Alignment,
}

impl Axis {
    pub fn top(y_value: f32) -> Self {
        Self {
            orientation: Orientation::Same,
            direction: -1.,
            tick_size: 7.5,
            offset: 0.5,
            at: y_value,
            count: None,
            align_x: Alignment::Center,
            align_y: Alignment::End,
        }
    }

    pub fn right(x_value: f32) -> Self {
        Self {
            orientation: Orientation::Flip,
            direction: 1.,
            tick_size: 7.5,
            offset: 0.5,
            at: x_value,
            count: None,
            align_x: Alignment::Start,
            align_y: Alignment::Center,
        }
    }

    pub fn bottom(y_value: f32) -> Self {
        Self {
            orientation: Orientation::Same,
            direction: 1.,
            tick_size: 7.5,
            offset: 0.5,
            at: y_value,
            count: None,
            align_x: Alignment::Center,
            align_y: Alignment::Start,
        }
    }

    pub fn left(x_value: f32) -> Self {
        Self {
            orientation: Orientation::Flip,
            direction: -1.,
            tick_size: 7.5,
            offset: 0.5,
            at: x_value,
            count: None,
            align_x: Alignment::End,
            align_y: Alignment::Center,
        }
    }
}

impl Axis {
    pub fn tick_size(self, tick_size: f32) -> Self {
        Self { tick_size, ..self }
    }

    pub fn offset(self, offset: f32) -> Self {
        Self { offset, ..self }
    }

    pub fn count(self, count: Option<usize>) -> Self {
        Self { count, ..self }
    }

    pub fn draw<D, T, StrokeColor, StrokeWidth, StrokeOpacity, Attribs, Formatter, FillColor>(
        &self,
        drawer: &mut D,
        scaler: &ScaleContinuous<T>,
        line_attrbs: &LineAttribs<f32, StrokeColor, StrokeWidth, StrokeOpacity>,
        text_attrbs: Attribs,
    ) where
        D: Draw,
        T: Transformer + Tick,
        StrokeColor: Generator<f32, Output = Color>,
        StrokeWidth: Generator<f32, Output = f32>,
        StrokeOpacity: Generator<f32, Output = f32>,
        Attribs: Into<TextAttrbs<f32, Formatter, FillColor>>,
        Formatter: Fn(&f32) -> String,
        FillColor: Generator<f32, Output = Color>,
        TextAttrbs<f32, Formatter, FillColor>: From<Attribs>,
    {
        let text_attrbs: TextAttrbs<f32, Formatter, FillColor> = text_attrbs.into();
        for tick_value in scaler.ticks(self.count) {
            let tick_coord = scaler.apply(tick_value);
            drawer.line(LineProperties {
                start: self.orientation.apply(tick_coord, self.at),
                end: self
                    .orientation
                    .apply(tick_coord, self.at + self.direction * self.tick_size),
                color: line_attrbs.color.generate(&tick_value),
                width: line_attrbs.width.generate(&tick_value),
                opacity: line_attrbs.opacity.generate(&tick_value),
            });
            drawer.text(TextProperties {
                position: self.orientation.apply(
                    tick_coord,
                    self.at + self.direction * (self.tick_size + self.offset),
                ),
                content: (text_attrbs.formatter)(&tick_value),
                color: text_attrbs.color.generate(&tick_value),
                align_x: self.align_x.clone(),
                align_y: self.align_y.clone(),
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Axis;
    use crate::draw::{Draw, LineAttribs, LineProperties, TextProperties};
    use crate::scale::ScaleContinuous;

    #[derive(Default)]
    struct Drawer {
        lines: Vec<LineProperties>,
        texts: Vec<TextProperties>,
    }

    impl Draw for Drawer {
        fn line(&mut self, line: LineProperties) {
            self.lines.push(line);
        }

        fn text(&mut self, text: TextProperties) {
            self.texts.push(text);
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
        Axis::bottom(height).draw(&mut drawer, &scale, &LineAttribs::default(), |x: &f32| {
            x.to_string()
        });

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
        Axis::top(margin_top).draw(&mut drawer, &scale, &LineAttribs::default(), |x: &f32| {
            x.to_string()
        });

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
        Axis::left(margin_left).draw(&mut drawer, &scale, &LineAttribs::default(), |x: &f32| {
            x.to_string()
        });

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
        Axis::right(width).draw(&mut drawer, &scale, &LineAttribs::default(), |x: &f32| {
            x.to_string()
        });

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
