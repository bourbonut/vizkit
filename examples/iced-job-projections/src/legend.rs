use iced::widget::{Column, canvas, column, row, space, text};
use std::str::FromStr;

use vizkit::scale::{ScaleContinuous, ScaleOrdinal};

use crate::data::Data;
use crate::{COLOR_DOMAIN, COLOR_RANGE, Message, RADIUS_BASE, RADIUS_RANGE};

// For drawing a simple circle
struct Circle {
    color: iced::Color,
    radius: f32,
    center: iced::Point,
}

impl<Message> canvas::Program<Message> for Circle {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &iced::Renderer,
        _theme: &iced::Theme,
        bounds: iced::Rectangle,
        _cursor: iced::mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        let mut frame = canvas::Frame::new(renderer, bounds.size());
        frame.fill(&canvas::Path::circle(self.center, self.radius), self.color);
        vec![frame.into_geometry()]
    }
}

pub fn legend<'a>(data: &Data) -> Column<'a, Message> {
    let bold_font = iced::Font {
        weight: iced::font::Weight::Bold,
        ..Default::default()
    };

    let radius = ScaleContinuous::sqrt()
        .domain(data.radius_domain)
        .range(RADIUS_RANGE);

    let color = ScaleOrdinal::default()
        .domain(&COLOR_DOMAIN)
        .range(&COLOR_RANGE);

    // Section with different radius values
    let column_element = column![
        // Section title
        text("Openings projected").font(bold_font),
        space().height(10.),
    ];

    let rmax = radius.apply(2000.);
    let column_element =
        [10.0, 100.0, 500.0, 1_000.0, 2_000.0]
            .into_iter()
            .fold(column_element, |col, r| {
                let string = r.to_string();
                let r = radius.apply(r);
                let circle = Circle {
                    color: iced::Color::WHITE,
                    radius: r,
                    center: [rmax, r].into(),
                };
                col.push(
                    row![
                        canvas(circle).width(rmax * 2.0).height(r * 2.0),
                        text(string),
                    ]
                    .spacing(15.)
                    .align_y(iced::Alignment::Center),
                )
            });

    // Add the next section with different colored circles
    let column_element = column_element
        .push(space().height(10.))
        // Section title
        .push(text("Occupation sector").font(bold_font));

    COLOR_DOMAIN
        .iter()
        .fold(column_element, |col, value| {
            let color_str = color.apply(value).map_or("", |v| v);
            let circle = Circle {
                color: iced::Color::from_str(color_str).unwrap_or_default(),
                radius: RADIUS_BASE,
                center: [RADIUS_BASE, RADIUS_BASE].into(),
            };
            col.push(
                row![
                    canvas(circle)
                        .width(RADIUS_BASE * 2.0)
                        .height(RADIUS_BASE * 2.0),
                    text(*value),
                ]
                .spacing(15.)
                .align_y(iced::Alignment::Center),
            )
        })
        .spacing(5.)
}
