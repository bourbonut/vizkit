mod data;
use std::str::FromStr;

use iced::{Element, widget::canvas};
use polars::{
    frame::DataFrame,
    prelude::{ChunkedArray, Column, PolarsDataType},
};
use vizkit::scale::{ScaleContinuous, ScaleOrdinal};

use crate::data::load_transform_data;

struct Margin {
    top: f32,
    right: f32,
    bottom: f32,
    left: f32,
}

/// Finds the min in a `Column`
fn min(column: &Column) -> Result<f32, Box<dyn std::error::Error>> {
    Ok(column.min_reduce()?.into_value().try_extract::<f32>()?)
}

/// Finds the max in a `Column`
fn max(column: &Column) -> Result<f32, Box<dyn std::error::Error>> {
    Ok(column.max_reduce()?.into_value().try_extract::<f32>()?)
}

/// Converts a `ChunkedArray` into `Vec<T>`
fn into_vec<'a, T: std::default::Default, U: PolarsDataType>(
    chunk_arr: &'a ChunkedArray<U>,
    f: impl Fn(U::Physical<'a>) -> T,
) -> Vec<T> {
    chunk_arr
        .iter()
        .map(|x| x.map(|v| f(v)).unwrap_or_default())
        .collect()
}

struct Plot {
    /// Radius domain for circles
    radius_domain: [f32; 2],
    /// X domain
    x_domain: [f32; 2],
    /// Margin dimensions
    margin: Margin,
    /// Turnover values
    turnover: Vec<f32>,
    /// Median wage values
    median_wage: Vec<f32>,
    /// Openings values
    openings: Vec<f32>,
    /// Sector cat values
    sector_cat: Vec<String>,
}

impl Plot {
    fn new(df: &DataFrame, margin: Margin) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            radius_domain: [min(&df["openings"])?, max(&df["openings"])?],
            x_domain: [0., max(&df["turnover"])?],
            turnover: into_vec(df["turnover"].f64()?, |v| v as f32),
            median_wage: into_vec(df["Median Wage 2018"].f64()?, |v| v as f32),
            openings: into_vec(df["openings"].f64()?, |v| v as f32),
            sector_cat: into_vec(df["sector_cat"].str()?, |v| v.to_string()),
            margin,
        })
    }
}

fn line(from: [f32; 2], to: [f32; 2]) -> canvas::Path {
    canvas::Path::line(from.into(), to.into())
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

        let white = iced::Color::WHITE;
        let stroke_white = canvas::Stroke::default().with_color(white);

        let radius = ScaleContinuous::sqrt()
            .domain(self.radius_domain.clone())
            .range([2., 20.]);

        let x = ScaleContinuous::linear()
            .domain(self.x_domain.clone())
            .range([self.margin.left, width - self.margin.right])
            .nice(None);

        let y = ScaleContinuous::linear()
            .domain([0., 140_000.])
            .range([height - self.margin.bottom, self.margin.top])
            .nice(None);

        let mut color = ScaleOrdinal::default()
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

        // X label
        let tx = (self.margin.left + width - self.margin.right) * 0.5;
        let ty = self.margin.bottom * 0.5;
        let text = canvas::Text {
            content: String::from("Occupation annual turnover rate"),
            position: [tx, height - self.margin.bottom + ty].into(),
            color: white,
            size: iced::Pixels(12.),
            font: iced::Font {
                weight: iced::font::Weight::Bold,
                ..Default::default()
            },
            align_x: iced::Alignment::Center.into(),
            ..Default::default()
        };
        frame.fill_text(text);

        // X axis domain line
        let start = [self.margin.left, height - self.margin.bottom];
        let end = [width - self.margin.right, height - self.margin.bottom];
        frame.stroke(&line(start, end), stroke_white);

        for tick in x.ticks(None) {
            let name = format!("{}%", (tick * 100.).round());
            let x_pos = x.apply(tick);

            // Tick lines
            let start = [x_pos, height - self.margin.bottom];
            let end = [x_pos, height - self.margin.bottom + 7.5];
            frame.stroke(&line(start, end), stroke_white);

            // Tick labels
            frame.fill_text(canvas::Text {
                content: name,
                position: [x_pos, height - self.margin.bottom + 8.].into(),
                color: white,
                size: iced::Pixels(10.),
                align_x: iced::Alignment::Center.into(),
                ..Default::default()
            });

            // Grid lines
            let start = [x_pos, self.margin.top];
            let end = [x_pos, height - self.margin.bottom];
            frame.stroke(&line(start, end), stroke_white);
        }

        // Y label
        let text = canvas::Text {
            content: String::from("Median wage, 2018"),
            position: [0., 0.].into(),
            color: iced::Color::WHITE,
            size: iced::Pixels(12.),
            font: iced::Font {
                weight: iced::font::Weight::Bold,
                ..Default::default()
            },
            align_x: iced::Alignment::Center.into(),
            ..Default::default()
        };

        // Rotate the text (Y label)
        frame.with_save(|frame| {
            frame.rotate(-std::f32::consts::PI * 0.5);

            let tx = self.margin.left * 0.9;
            let ty = (height - self.margin.bottom + self.margin.top) * 0.5;
            frame.translate([-ty, self.margin.left - tx].into());

            frame.fill_text(text);
        });

        // Y axis domain line
        let start = [self.margin.left, self.margin.top];
        let end = [self.margin.left, height - self.margin.bottom];
        frame.stroke(&line(start, end), stroke_white);

        for tick in y.ticks(Some(5)) {
            let name = format!("${}k", (tick / 1000.).round());
            let y_pos = y.apply(tick);

            // Tick lines
            let start = [self.margin.left - 7.5, y_pos];
            let end = [self.margin.left, y_pos];
            frame.stroke(&line(start, end), stroke_white);

            // Tick labels
            frame.fill_text(canvas::Text {
                content: name,
                position: [self.margin.left - 8., y_pos].into(),
                color: white,
                size: iced::Pixels(10.),
                align_x: iced::Alignment::End.into(),
                align_y: iced::Alignment::Center.into(),
                ..Default::default()
            });

            // Grid lines
            let start = [self.margin.left, y_pos];
            let end = [width - self.margin.right, y_pos];
            frame.stroke(&line(start, end), stroke_white);
        }

        // Circles
        for idx in 0..self.turnover.len() {
            let cx = x.apply(self.turnover[idx]);
            let cy = y.apply(self.median_wage[idx]);
            let r = radius.apply(self.openings[idx]);
            let fill_color = color.apply(&self.sector_cat[idx]).map_or("", |v| v);
            let stroke_color = color.apply(&self.sector_cat[idx]).map_or("", |v| v);

            let circle = canvas::Path::circle([cx, cy].into(), r);
            frame.fill(
                &circle,
                canvas::Fill {
                    style: canvas::Style::Solid(
                        iced::Color::from_str(fill_color)
                            .unwrap_or_default()
                            .scale_alpha(0.5),
                    ),
                    rule: canvas::fill::Rule::EvenOdd,
                },
            );

            frame.stroke(
                &circle,
                canvas::Stroke::default()
                    .with_width(0.75)
                    .with_color(iced::Color::from_str(stroke_color).unwrap_or_default()),
            );
        }

        // Y reference (horizontal line)
        let start = [self.margin.left, y.apply(33_900.0)];
        let end = [width - self.margin.right, y.apply(33_900.0)];
        frame.stroke(
            &line(start, end),
            canvas::Stroke::default().with_width(1.5).with_color(
                iced::Color::from_str("#666")
                    .unwrap_or_default()
                    .scale_alpha(0.75),
            ),
        );

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
    iced::application(App::default, App::update, App::view)
        .antialiasing(true)
        .run()
}
