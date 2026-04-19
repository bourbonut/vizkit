use super::{Draw, LineProperties};

use crate::chromatic::Color;

pub struct Line {
    color: Color,
    width: f32,
    opacity: f32,
}

impl Default for Line {
    fn default() -> Self {
        Self {
            color: Color::default(),
            width: 1.,
            opacity: 1.,
        }
    }
}

impl Line {
    pub fn color(self, color: Color) -> Self {
        Self { color, ..self }
    }

    pub fn width(self, width: f32) -> Self {
        Self { width, ..self }
    }

    pub fn opacity(self, opacity: f32) -> Self {
        Self { opacity, ..self }
    }

    pub fn draw<D: Draw>(&self, drawer: &mut D, start: [f32; 2], end: [f32; 2]) {
        drawer.line(LineProperties {
            start,
            end,
            stroke_color: self.color,
            stroke_width: self.width,
            stroke_opacity: self.opacity,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::Line;

    use crate::chromatic::Color;
    use crate::draw::{Draw, LineProperties, TextProperties};

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
    fn test_line() {
        let start = [10., 20.];
        let end = [30., 40.];

        let mut drawer = Drawer::default();
        Line::default()
            .color(Color([0.1, 0.2, 0.3]))
            .width(1.23)
            .opacity(0.321)
            .draw(&mut drawer, start, end);

        assert_eq!(drawer.lines.len(), 1);
        let line = &drawer.lines[0];
        assert_eq!(line.start, start);
        assert_eq!(line.end, end);
        assert_eq!(line.stroke_color.0, [0.1, 0.2, 0.3]);
        assert_eq!(line.stroke_width, 1.23);
        assert_eq!(line.stroke_opacity, 0.321);
    }
}
