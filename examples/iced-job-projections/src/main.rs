mod data;
use core::f32;

use iced::{
    Element,
    widget::{canvas, text},
};
use polars::{frame::DataFrame, prelude::Column};
use vizkit::scale::{ScaleContinuous, ScaleOrdinal};

use crate::data::load_transform_data;

#[allow(unused)]
struct Margin {
    top: f32,
    right: f32,
    bottom: f32,
    left: f32,
}

fn min(column: &Column) -> Result<f32, Box<dyn std::error::Error>> {
    Ok(column.min_reduce()?.into_value().try_extract::<f32>()?)
}

fn max(column: &Column) -> Result<f32, Box<dyn std::error::Error>> {
    Ok(column.max_reduce()?.into_value().try_extract::<f32>()?)
}

struct Plot {
    radius_domain: [f32; 2],
    x_domain: [f32; 2],
    margin: Margin,
}

impl Plot {
    fn new(df: &DataFrame, margin: Margin) -> Result<Self, Box<dyn std::error::Error>> {
        let xmax = df["turnover"]
            .max_reduce()?
            .into_value()
            .try_extract::<f32>()?;

        Ok(Self {
            radius_domain: [min(&df["openings"])?, max(&df["openings"])?],
            x_domain: [0., xmax],
            margin,
        })
    }
}

enum Message {}

impl<Message> canvas::Program<Message> for Plot {
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
        let width = bounds.width;
        let height = bounds.height;

        let _radius = ScaleContinuous::sqrt()
            .domain(self.radius_domain.clone())
            .range([2., 20.]);

        let x = ScaleContinuous::linear()
            .domain(self.x_domain.clone())
            .range([self.margin.left, width - self.margin.right])
            .nice(None);

        let y = ScaleContinuous::linear()
            .domain([0., 140000.])
            .range([height - self.margin.bottom, self.margin.top])
            .nice(None);

        let _color = ScaleOrdinal::default()
            .domain(vec![
                "Natural Resources",
                "Construction",
                "Manufacturing",
                "Trade",
                "Services",
                "Healthcare",
                "Education/Government",
            ])
            .range(vec![
                "#1b9e77", "#d95f02", "#7570b3", "#e7298a", "#66a61e", "#e6ab02", "#a6761d",
            ]);

        let x_axis = canvas::Path::line(
            [self.margin.left, height - self.margin.bottom].into(),
            [width - self.margin.right, height - self.margin.bottom].into(),
        );
        frame.stroke(
            &x_axis,
            canvas::Stroke::default().with_color(iced::Color::WHITE),
        );

        let tx = (self.margin.left + width - self.margin.right) * 0.5;
        let ty = self.margin.bottom * 0.5;
        let text = canvas::Text {
            content: String::from("Occupation annual turnover rate"),
            position: [tx, height - self.margin.bottom + ty].into(),
            max_width: f32::INFINITY,
            color: iced::Color::WHITE,
            size: (12.).into(),
            line_height: text::LineHeight::default(),
            font: iced::Font::default(),
            align_x: iced::Alignment::Center.into(),
            ..Default::default()
        };
        frame.fill_text(text);

        for tick in x.ticks(None) {
            let name = format!("{}%", (tick * 100.).round());
            let x_pos = x.apply(tick);
            let line = canvas::Path::line(
                [x_pos, height - self.margin.bottom].into(),
                [x_pos, height - self.margin.bottom + 7.5].into(),
            );
            frame.stroke(
                &line,
                canvas::Stroke::default().with_color(iced::Color::WHITE),
            );

            let text = canvas::Text {
                content: name,
                position: [x_pos, height - self.margin.bottom + 8.].into(),
                max_width: f32::INFINITY,
                color: iced::Color::WHITE,
                size: (10.).into(),
                line_height: text::LineHeight::default(),
                font: iced::Font::default(),
                align_x: iced::Alignment::Center.into(),
                ..Default::default()
            };
            frame.fill_text(text);
        }

        let y_axis = canvas::Path::line(
            [self.margin.left, self.margin.top].into(),
            [self.margin.left, height - self.margin.bottom].into(),
        );
        frame.stroke(
            &y_axis,
            canvas::Stroke::default().with_color(iced::Color::WHITE),
        );

        let text = canvas::Text {
            content: String::from("Median wage, 2018"),
            position: [0., 0.].into(),
            max_width: f32::INFINITY,
            color: iced::Color::WHITE,
            size: (12.).into(),
            line_height: text::LineHeight::default(),
            font: iced::Font::default(),
            align_x: iced::Alignment::Center.into(),
            ..Default::default()
        };

        frame.with_save(|frame| {
            frame.rotate(-std::f32::consts::PI * 0.5);

            let tx = self.margin.left * 0.9;
            let ty = (height - self.margin.bottom + self.margin.top) * 0.5;
            frame.translate([-ty, self.margin.left - tx].into());

            frame.fill_text(text);
        });

        for tick in y.ticks(Some(5)) {
            let name = format!("{}k", (tick / 1000.).round());
            let y_pos = y.apply(tick);
            let line = canvas::Path::line(
                [self.margin.left - 7.5, y_pos].into(),
                [self.margin.left, y_pos].into(),
            );
            frame.stroke(
                &line,
                canvas::Stroke::default().with_color(iced::Color::WHITE),
            );

            let text = canvas::Text {
                content: name,
                position: [self.margin.left - 8., y_pos].into(),
                max_width: f32::INFINITY,
                color: iced::Color::WHITE,
                size: (10.).into(),
                line_height: text::LineHeight::default(),
                font: iced::Font::default(),
                align_x: iced::Alignment::End.into(),
                align_y: iced::Alignment::Center.into(),
                ..Default::default()
            };
            frame.fill_text(text);
        }

        vec![frame.into_geometry()]
    }
}

struct App {
    df: DataFrame,
}

impl Default for App {
    fn default() -> Self {
        Self {
            df: load_transform_data().unwrap(),
        }
    }
}

impl App {
    fn update(&mut self, _: Message) {}
    fn view(&self) -> Element<'_, Message> {
        let margin = Margin {
            top: 10.,
            right: 15.,
            bottom: 40.,
            left: 55.,
        };
        canvas(Plot::new(&self.df, margin).unwrap())
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .into()
    }
}

fn main() -> iced::Result {
    iced::application(App::default, App::update, App::view).run()
}
