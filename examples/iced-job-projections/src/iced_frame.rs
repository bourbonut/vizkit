use iced::widget::canvas;
use std::ops::{Deref, DerefMut};

use vizkit::draw::{
    Alignment, CircleProperties, Draw, LineProperties, RectProperties, TextProperties,
};

pub struct IcedFrame<'a>(pub &'a mut canvas::Frame);

impl<'a> Deref for IcedFrame<'a> {
    type Target = canvas::Frame;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// It allows to use at the same time the `Frame` methods and the `Draw` methods
impl<'a> DerefMut for IcedFrame<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'a> Draw for IcedFrame<'a> {
    fn draw_line(&mut self, line: LineProperties) {
        let [r, g, b] = line.stroke_color.into();
        self.0.stroke(
            &canvas::Path::line(line.start.into(), line.end.into()),
            canvas::Stroke::default()
                .with_color(iced::Color::from([r, g, b, line.stroke_opacity]))
                .with_width(line.stroke_width),
        );
    }

    fn draw_text(&mut self, text: TextProperties) {
        let color: [f32; 3] = text.fill_color.into();
        self.0.fill_text(canvas::Text {
            content: text.content,
            position: text.position.into(),
            color: iced::Color::from(color),
            size: iced::Pixels(text.font_size),
            align_x: match text.align_x {
                Alignment::Start => iced::Alignment::Start.into(),
                Alignment::Center => iced::Alignment::Center.into(),
                Alignment::End => iced::Alignment::End.into(),
            },
            align_y: match text.align_y {
                Alignment::Start => iced::Alignment::Start.into(),
                Alignment::Center => iced::Alignment::Center.into(),
                Alignment::End => iced::Alignment::End.into(),
            },
            ..Default::default()
        })
    }

    fn draw_circle(&mut self, circle: CircleProperties) {
        let circle_path = canvas::Path::circle(circle.center.into(), circle.radius);
        if let Some(fill_color) = circle.fill_color {
            let fill_color: [f32; 3] = fill_color.into();
            self.0.fill(
                &circle_path,
                canvas::Fill {
                    style: canvas::Style::Solid(
                        iced::Color::from(fill_color).scale_alpha(circle.fill_opacity),
                    ),
                    rule: canvas::fill::Rule::EvenOdd,
                },
            );
        }

        if let Some(stroke_color) = circle.stroke_color {
            let stroke_color: [f32; 3] = stroke_color.into();
            self.0.stroke(
                &circle_path,
                canvas::Stroke::default()
                    .with_width(circle.stroke_width)
                    .with_color(iced::Color::from(stroke_color)),
            );
        }
    }

    fn draw_rect(&mut self, _: RectProperties) {
        todo!()
    }
}
