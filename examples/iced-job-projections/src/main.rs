mod data;
use std::ops::{Deref, DerefMut};
use std::str::FromStr;

use iced::{
    Element,
    widget::{canvas, column, container, row, space, text, tooltip},
};
use vizkit::draw::{Alignment, Axis, Draw, LineAttrbs, LineProperties, TextProperties};
use vizkit::scale::{Linear, ScaleContinuous, ScaleOrdinal};

use crate::data::Data;

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

const RRANGE: [f32; 2] = [4., 40.];

const RBASE: f32 = 10.;

struct Margin {
    top: f32,
    right: f32,
    bottom: f32,
    left: f32,
}

struct Plot<'a> {
    data: &'a Data,
    margin: Margin,
}

impl<'a> Plot<'a> {
    fn new(data: &'a Data, margin: Margin) -> Self {
        Self { data: data, margin }
    }
}

struct IcedFrame<'a>(&'a mut canvas::Frame);

impl<'a> Deref for IcedFrame<'a> {
    type Target = canvas::Frame;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> DerefMut for IcedFrame<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'a> Draw for IcedFrame<'a> {
    fn line(&mut self, line: LineProperties) {
        let [r, g, b] = line.color.into();
        self.0.stroke(
            &canvas::Path::line(line.start.into(), line.end.into()),
            canvas::Stroke::default()
                .with_color(iced::Color::from([r, g, b, line.opacity]))
                .with_width(line.width),
        );
    }

    fn text(&mut self, text: TextProperties) {
        let color: [f32; 3] = text.color.into();
        self.0.fill_text(canvas::Text {
            content: text.content,
            position: text.position.into(),
            color: iced::Color::from(color),
            size: iced::Pixels(10.),
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
}

fn line(from: [f32; 2], to: [f32; 2]) -> canvas::Path {
    canvas::Path::line(from.into(), to.into())
}

enum Message {
    HoverCircle(usize),
    None,
}

struct PlotCircle {
    center: [f32; 2],
    radius: f32,
    fill_color: iced::Color,
    stroke_color: iced::Color,
}

impl PlotCircle {
    fn new(center: [f32; 2], radius: f32, fill_color: &str, stroke_color: &str) -> Self {
        Self {
            center,
            radius,
            fill_color: iced::Color::from_str(fill_color)
                .unwrap_or_default()
                .scale_alpha(0.5),
            stroke_color: iced::Color::from_str(stroke_color).unwrap_or_default(),
        }
    }
}

struct PlotState {
    x_scale: ScaleContinuous<Linear>,
    y_scale: ScaleContinuous<Linear>,
    circles: Vec<PlotCircle>,
}

impl Default for PlotState {
    fn default() -> Self {
        Self {
            x_scale: ScaleContinuous::linear(),
            y_scale: ScaleContinuous::linear(),
            circles: Vec::new(),
        }
    }
}

impl<'a> canvas::Program<Message> for Plot<'a> {
    type State = PlotState;

    fn draw(
        &self,
        state: &Self::State,
        renderer: &iced::Renderer,
        theme: &iced::Theme,
        bounds: iced::Rectangle,
        _cursor: iced::mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        let mut iced_frame = canvas::Frame::new(renderer, bounds.size());
        let mut frame = IcedFrame(&mut iced_frame);
        let width = bounds.width;
        let height = bounds.height;

        let text_color = theme.palette().text;
        let stroke_color = canvas::Stroke::default().with_color(text_color);
        let bold_font = iced::Font {
            weight: iced::font::Weight::Bold,
            ..Default::default()
        };

        // X label
        let tx = (self.margin.left + width - self.margin.right) * 0.5;
        let ty = self.margin.bottom * 0.5;
        let text = canvas::Text {
            content: String::from("Occupation annual turnover rate"),
            position: [tx, height - self.margin.bottom + ty].into(),
            color: text_color,
            size: iced::Pixels(12.),
            font: bold_font,
            align_x: iced::Alignment::Center.into(),
            ..Default::default()
        };
        frame.fill_text(text);

        // X axis domain line
        let start = [self.margin.left, height - self.margin.bottom];
        let end = [width - self.margin.right, height - self.margin.bottom];
        frame.stroke(&line(start, end), stroke_color);

        Axis::bottom(height - self.margin.bottom).draw(
            &mut frame,
            &state.x_scale,
            &LineAttrbs::default(),
            |tick: &f32| format!("{}%", (tick * 100.).round()),
        );

        // Grid lines
        for tick in state.x_scale.ticks(None) {
            let x_pos = state.x_scale.apply(tick);
            let start = [x_pos, self.margin.top];
            let end = [x_pos, height - self.margin.bottom];
            frame.stroke(&line(start, end), stroke_color);
        }

        // Y label
        let text = canvas::Text {
            content: String::from("Median wage, 2018"),
            position: [0., 0.].into(),
            color: text_color,
            size: iced::Pixels(12.),
            font: bold_font,
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
        frame.stroke(&line(start, end), stroke_color);

        Axis::left(self.margin.left).draw(
            &mut frame,
            &state.y_scale,
            &LineAttrbs::default(),
            |tick: &f32| format!("${}k", (tick / 1000.).round()),
        );

        // Grid lines
        for tick in state.y_scale.ticks(Some(5)) {
            let y_pos = state.y_scale.apply(tick);
            let start = [self.margin.left, y_pos];
            let end = [width - self.margin.right, y_pos];
            frame.stroke(&line(start, end), stroke_color);
        }

        // Circles
        for plot_circle in state.circles.iter() {
            let circle = canvas::Path::circle(plot_circle.center.into(), plot_circle.radius);
            frame.fill(
                &circle,
                canvas::Fill {
                    style: canvas::Style::Solid(plot_circle.fill_color),
                    rule: canvas::fill::Rule::EvenOdd,
                },
            );

            frame.stroke(
                &circle,
                canvas::Stroke::default()
                    .with_width(0.75)
                    .with_color(plot_circle.stroke_color),
            );
        }

        // Y reference (horizontal line)
        let start = [self.margin.left, state.y_scale.apply(33_900.0)];
        let end = [width - self.margin.right, state.y_scale.apply(33_900.0)];
        frame.stroke(
            &line(start, end),
            canvas::Stroke::default().with_width(1.5).with_color(
                iced::Color::from_str("#666")
                    .unwrap_or_default()
                    .scale_alpha(0.75),
            ),
        );

        vec![iced_frame.into_geometry()]
    }

    fn update(
        &self,
        state: &mut Self::State,
        _event: &iced::Event,
        bounds: iced::Rectangle,
        cursor: iced::mouse::Cursor,
    ) -> Option<canvas::Action<Message>> {
        let width = bounds.width;
        let height = bounds.height;

        state.x_scale = ScaleContinuous::linear()
            .domain(self.data.x_domain)
            .range([self.margin.left, width - self.margin.right])
            .nice(None);

        state.y_scale = ScaleContinuous::linear()
            .domain([0., 140_000.])
            .range([height - self.margin.bottom, self.margin.top])
            .nice(None);

        let r_scale = ScaleContinuous::sqrt()
            .domain(self.data.radius_domain)
            .range(RRANGE);

        let mut color = ScaleOrdinal::default()
            .domain(&COLOR_DOMAIN)
            .range(&COLOR_RANGE);

        state.circles = self
            .data
            .into_iter()
            .map(|row| {
                let cx = state.x_scale.apply(row.turnover);
                let cy = state.y_scale.apply(row.median_wage);
                let r = r_scale.apply(row.openings);
                let fill_color = color.apply(&row.sector_cat).map_or("", |v| v);
                let stroke_color = color.apply(&row.sector_cat).map_or("", |v| v);
                PlotCircle::new([cx, cy], r, fill_color, stroke_color)
            })
            .collect();

        if let Some(position) = cursor.position() {
            let argmin = state
                .circles
                .iter()
                .enumerate()
                .filter_map(|(idx, circle)| {
                    let center = iced::Point::from(circle.center);
                    let r = circle.radius;
                    let delta = position - center;
                    let x = delta.x;
                    let y = delta.y;
                    let h = x.hypot(y);
                    if h > r { None } else { Some((r - h, idx)) }
                })
                .min_by(|(a, _), (b, _)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
            match argmin {
                Some((_, idx)) => Some(iced::widget::Action::publish(Message::HoverCircle(idx))),
                None => Some(iced::widget::Action::publish(Message::None)),
            }
        } else {
            Some(iced::widget::Action::publish(Message::None))
        }
    }
}

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

fn legend<'a>(data: &Data) -> iced::widget::Column<'a, Message> {
    let radius = ScaleContinuous::sqrt()
        .domain(data.radius_domain)
        .range(RRANGE);

    let mut color = ScaleOrdinal::default()
        .domain(&COLOR_DOMAIN)
        .range(&COLOR_RANGE);

    // Section with different radii values
    let column_element = column![
        text("Openings projected").font(iced::Font {
            weight: iced::font::Weight::Bold,
            ..Default::default()
        }),
        space().height(10.),
    ];

    let rmax = radius.apply(2000.);
    let column_element = [10.0, 100.0, 500.0, 1_000.0, 2_000.0]
        .into_iter()
        .fold(column_element, |col, r| {
            let string = r.to_string();
            let r = radius.apply(r);
            col.push(
                row![
                    canvas(Circle {
                        color: iced::Color::WHITE,
                        radius: r,
                        center: [rmax, r].into(),
                    })
                    .width(iced::Length::Fixed(rmax * 2.0))
                    .height(iced::Length::Fixed(r * 2.0)),
                    text(string),
                ]
                .spacing(15.)
                .align_y(iced::Alignment::Center),
            )
        })
        .push(space().height(10.)) // Add the next title
        .push(text("Occupation sector").font(iced::Font {
            weight: iced::font::Weight::Bold,
            ..Default::default()
        }));

    COLOR_DOMAIN // Section with different colors
        .iter()
        .fold(column_element, |col, value| {
            let color_str = color.apply(value).map_or("", |v| v);
            col.push(
                row![
                    canvas(Circle {
                        color: iced::Color::from_str(color_str).unwrap_or_default(),
                        radius: RBASE,
                        center: [RBASE, RBASE].into(),
                    })
                    .width(iced::Length::Fixed(RBASE * 2.0))
                    .height(iced::Length::Fixed(RBASE * 2.0)),
                    text(*value),
                ]
                .spacing(15.)
                .align_y(iced::Alignment::Center),
            )
        })
        .spacing(5.)
}

struct App {
    data: Data,
    hovered_index: Option<usize>,
}

impl App {
    fn new() -> Self {
        Self {
            data: Data::new().unwrap(),
            hovered_index: None,
        }
    }
}

impl App {
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
            let row_data = self.data.row(idx);
            tooltip(
                row_element,
                container(column![
                    "Occupation",
                    row_data.soc_title,
                    "Sector",
                    row_data.sector,
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
