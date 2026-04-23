mod data;
mod iced_frame;
mod legend;
mod plot;

use iced::{
    Element,
    widget::{canvas, column, container, row, text, tooltip},
};

use crate::data::Data;
use crate::legend::legend;
use crate::plot::Plot;

const COLOR_DOMAIN: [&str; 7] = [
    "Natural Resources",
    "Construction",
    "Manufacturing",
    "Trade",
    "Services",
    "Healthcare",
    "Education/Government",
];

const COLOR_RANGE: [&str; 7] = [
    "#1b9e77", "#d95f02", "#7570b3", "#e7298a", "#66a61e", "#e6ab02", "#a6761d",
];

const RADIUS_RANGE: [f32; 2] = [4., 40.];

const RADIUS_BASE: f32 = 10.;

struct Margin {
    top: f32,
    right: f32,
    bottom: f32,
    left: f32,
}

enum Message {
    HoverCircle(usize),
    None,
}

struct App {
    data: Data,
    hovered_index: Option<usize>,
}

impl App {
    fn new() -> Self {
        Self {
            data: Data::new(),
            hovered_index: None,
        }
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::HoverCircle(idx) => self.hovered_index = Some(idx),
            Message::None => self.hovered_index = None,
        }
    }
    fn view(&self) -> Element<'_, Message> {
        let margin = Margin {
            top: 10.,
            right: 15.,
            bottom: 40.,
            left: 55.,
        };
        let row_element = row![
            canvas(Plot::new(&self.data, margin))
                .width(iced::Length::Fill)
                .height(iced::Length::Fill),
            container(legend(&self.data))
                .width(iced::Length::Shrink)
                .padding(20.)
        ];
        if let Some(idx) = self.hovered_index {
            let row_data = &self.data.items[idx];
            tooltip(
                row_element,
                container(column![
                    "Occupation",
                    row_data.soc_title.as_str(),
                    "Sector",
                    row_data.sector.as_str(),
                    "Median Wage 2018",
                    text(format!("${}k", (row_data.median_wage / 1000.).round())),
                    "Turnover",
                    text(format!("{}%", (row_data.turnover * 100.).round())),
                ])
                .style(container::rounded_box)
                .padding(10.),
                tooltip::Position::FollowCursor,
            )
            .into()
        } else {
            row_element.into()
        }
    }
}

fn main() -> iced::Result {
    iced::application(App::new, App::update, App::view)
        .antialiasing(true)
        .run()
}
