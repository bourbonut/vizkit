use chrono::{DateTime, Datelike, NaiveDate, Utc};
use iced::widget::canvas::Path;
use iced::{Element, Length, widget::Action, widget::canvas};
use std::cmp::Ordering;
use std::{collections::HashMap, fs::File};
use vizkit::{
    draw::{
        Alignment, AxisOptions, Curve, LineProperties, PathCommand, TextProperties,
        axis_bottom_iter, axis_left_iter, grid_horizontal_iter, path_iter,
    },
    scale::{Axis, ScaleContinuous, ScaleTime},
};

const MARGIN_TOP: f32 = 20.;
const MARGIN_RIGHT: f32 = 30.;
const MARGIN_BOTTOM: f32 = 50.;
const MARGIN_LEFT: f32 = 40.;

// Easy function for creating a datetime
fn datetime(year: i32, month: u32, day: u32) -> DateTime<Utc> {
    NaiveDate::from_ymd_opt(year, month, day)
        .and_then(|date| date.and_hms_opt(0, 0, 0))
        .expect("invalid time values")
        .and_utc()
}

// Functions for drawing lines and texts and building a path
fn line_frame(frame: &mut canvas::Frame, line: LineProperties) {
    let [r, g, b] = line.stroke_color.into();
    frame.stroke(
        &canvas::Path::line(line.start.into(), line.end.into()),
        canvas::Stroke::default()
            .with_color(iced::Color::from([r, g, b, line.stroke_opacity]))
            .with_width(line.stroke_width),
    );
}

fn text_frame(frame: &mut canvas::Frame, text: TextProperties) {
    let color: [f32; 3] = text.fill_color.into();
    frame.fill_text(canvas::Text {
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

fn build_path(rows: &[Row], x: impl Fn(&Row) -> f32, y: impl Fn(&Row) -> f32) -> canvas::Path {
    canvas::Path::new(|builder| {
        // If tension == 1.0, it has the same shape as `Curve::Linear` but with more complexity
        path_iter(rows, x, y, Curve::Cardinal { tension: 0.0 }).for_each(move |path_command| {
            match path_command {
                PathCommand::MoveTo(point) => builder.move_to(point.into()),
                PathCommand::LineTo(point) => builder.line_to(point.into()),
                PathCommand::BezierCurveTo([p1, p2, p3]) => {
                    builder.bezier_curve_to(p1.into(), p2.into(), p3.into())
                }
            }
        })
    })
}

// Data from CSV
#[derive(Debug, serde::Deserialize)]
struct Record {
    cbsatitle: String,
    unemp_jan20: f32,
    unemp_feb20: f32,
    unemp_mar20: f32,
    unemp_apr20: f32,
    unemp_may20: f32,
    unemp_jun20: f32,
    unemp_jul20: f32,
    unemp_aug20: f32,
    unemp_sep20: f32,
    unemp_oct20: f32,
    unemp_nov20: f32,
    unemp_dec20: f32,
    unemp_jan21: f32,
    unemp_feb21: f32,
    unemp_mar21: f32,
}

// Processed data
#[derive(Debug)]
struct Row {
    date: DateTime<Utc>,
    unemp_value: f32,
}

struct Data {
    /// Key: division (= cbsatitle), values: list[(date, unemployment value)]
    values: HashMap<String, Vec<Row>>,
    unemp_domain: [f32; 2],
}

// Partial comparison between two floats
fn cmp_f32(a: &f32, b: &f32) -> Ordering {
    a.partial_cmp(&b).unwrap_or(std::cmp::Ordering::Equal)
}

fn load_data() -> Data {
    let reader = File::open("src/bls-unemployment-rate.csv").unwrap();
    let mut rdr = csv::Reader::from_reader(reader);
    let mut values: HashMap<String, Vec<Row>> = HashMap::new();
    let mut unemp_min = f32::INFINITY;
    let mut unemp_max = f32::NEG_INFINITY;
    for result in rdr.deserialize() {
        let record: Record = result.unwrap();
        let rows = vec![
            |record: &Record| (record.unemp_jan20, datetime(2020, 1, 1)),
            |record: &Record| (record.unemp_feb20, datetime(2020, 2, 1)),
            |record: &Record| (record.unemp_mar20, datetime(2020, 3, 1)),
            |record: &Record| (record.unemp_apr20, datetime(2020, 4, 1)),
            |record: &Record| (record.unemp_may20, datetime(2020, 5, 1)),
            |record: &Record| (record.unemp_jun20, datetime(2020, 6, 1)),
            |record: &Record| (record.unemp_jul20, datetime(2020, 7, 1)),
            |record: &Record| (record.unemp_aug20, datetime(2020, 8, 1)),
            |record: &Record| (record.unemp_sep20, datetime(2020, 9, 1)),
            |record: &Record| (record.unemp_oct20, datetime(2020, 10, 1)),
            |record: &Record| (record.unemp_nov20, datetime(2020, 11, 1)),
            |record: &Record| (record.unemp_dec20, datetime(2020, 12, 1)),
            |record: &Record| (record.unemp_jan21, datetime(2021, 1, 1)),
            |record: &Record| (record.unemp_feb21, datetime(2021, 2, 1)),
            |record: &Record| (record.unemp_mar21, datetime(2021, 3, 1)),
        ]
        .into_iter()
        .map(|f| {
            let (unemp_value, date) = f(&record);
            Row { date, unemp_value }
        })
        .collect::<Vec<Row>>();
        unemp_min = unemp_min.min(
            rows.iter()
                .map(|row| row.unemp_value)
                .min_by(cmp_f32)
                .unwrap_or(f32::INFINITY),
        );
        unemp_max = unemp_max.max(
            rows.iter()
                .map(|row| row.unemp_value)
                .max_by(cmp_f32)
                .unwrap_or(f32::NEG_INFINITY),
        );
        values.insert(record.cbsatitle, rows);
    }
    Data {
        values,
        unemp_domain: [unemp_min, unemp_max],
    }
}

struct Plot<'a> {
    data: &'a Data,
}

impl canvas::Program<Message> for Plot<'_> {
    type State = Option<iced::Point>;
    fn draw(
        &self,
        state: &Self::State,
        renderer: &iced::Renderer,
        _theme: &iced::Theme,
        bounds: iced::Rectangle,
        _cursor: iced::mouse::Cursor,
    ) -> Vec<canvas::Geometry<iced::Renderer>> {
        let mut frame = canvas::Frame::new(renderer, bounds.size());
        let width = bounds.width;
        let height = bounds.height;
        let x_scale = ScaleTime::default()
            .domain([datetime(2020, 1, 1), datetime(2021, 3, 1)])
            .range([MARGIN_LEFT as f64, (width - MARGIN_RIGHT) as f64]);

        let y_scale = ScaleContinuous::linear()
            .domain(self.data.unemp_domain)
            .range([height - MARGIN_BOTTOM, MARGIN_TOP]);

        // Compute the closest line and the closest point given the cursor position
        let argmin = state.and_then(|position| {
            let dist = |row: &Row| {
                (x_scale.apply(row.date) as f32 - position.x)
                    .hypot(y_scale.apply(row.unemp_value) - position.y)
            };
            self.data
                .values
                .iter()
                .map(|(key, rows)| {
                    let (point_idx, min_value) = rows
                        .iter()
                        .map(dist)
                        .enumerate()
                        .min_by(|(_, a), (_, b)| cmp_f32(a, b))
                        .unwrap_or((0, f32::INFINITY));
                    (key, point_idx, min_value)
                })
                .min_by(|(_, _, a), (_, _, b)| cmp_f32(a, b))
        });

        if let Some((division, row_idx, _)) = argmin {
            // Draw lines
            self.data.values.iter().for_each(|(key, rows)| {
                let path = build_path(
                    rows,
                    |row| x_scale.apply(row.date) as f32,
                    |row| y_scale.apply(row.unemp_value),
                );
                let (color, stroke_width) = if key == division {
                    (iced::Color::from_rgb(0.275, 0.51, 0.706), 3.)
                } else {
                    let rgb = 221. / 255.;
                    (iced::Color::from_rgb(rgb, rgb, rgb).scale_alpha(0.2), 1.)
                };
                frame.stroke(
                    &path,
                    canvas::Stroke::default()
                        .with_color(color)
                        .with_width(stroke_width),
                );
            });

            // Draw the closest point and the division over the point
            if let Some(row) = self
                .data
                .values
                .get(division)
                .and_then(|rows| rows.get(row_idx))
            {
                let x = x_scale.apply(row.date) as f32;
                let y = y_scale.apply(row.unemp_value);
                let circle = Path::circle([x, y].into(), 4.);
                frame.fill(&circle, iced::Color::WHITE);

                text_frame(
                    &mut frame,
                    TextProperties {
                        content: division.clone(),
                        position: [x, y - 15.],
                        fill_color: vizkit::chromatic::Color([1., 1., 1.]),
                        font_size: 12.,
                        align_x: Alignment::Center,
                        align_y: Alignment::Center,
                    },
                );
            }
        } else {
            // Draw all lines in blue
            self.data.values.values().for_each(|rows| {
                let path = build_path(
                    rows,
                    |row| x_scale.apply(row.date) as f32,
                    |row| y_scale.apply(row.unemp_value),
                );
                frame.stroke(
                    &path,
                    canvas::Stroke::default()
                        .with_color(iced::Color::from_rgb(0.275, 0.51, 0.706).scale_alpha(0.8)),
                );
            });
        }

        // Draw axises
        axis_bottom_iter(
            &x_scale,
            height - MARGIN_BOTTOM,
            |tick| format!("{}/{}", tick.month(), tick.year()),
            &AxisOptions::default(),
        )
        .chain(axis_left_iter(
            &y_scale,
            MARGIN_LEFT,
            |tick| format!("{}%", tick),
            &AxisOptions {
                offset: 3.,
                ..Default::default()
            },
        ))
        .for_each(|(line, text)| {
            line_frame(&mut frame, line);
            text_frame(&mut frame, text);
        });

        // Draw horizontal grid
        grid_horizontal_iter(
            &y_scale.ticks(None),
            MARGIN_LEFT,
            width - MARGIN_RIGHT,
            |&d| y_scale.apply(d),
            |_| vizkit::draw::LineAttrs {
                stroke_opacity: 0.2,
                ..Default::default()
            },
        )
        .for_each(|line| line_frame(&mut frame, line));
        vec![frame.into_geometry()]
    }

    fn update(
        &self,
        state: &mut Self::State,
        _event: &iced::Event,
        _bounds: iced::Rectangle,
        cursor: iced::mouse::Cursor,
    ) -> Option<canvas::Action<Message>> {
        *state = cursor.position();
        Some(Action::request_redraw())
    }
}

struct App {
    data: Data,
}

impl Default for App {
    fn default() -> Self {
        Self { data: load_data() }
    }
}

enum Message {}

impl App {
    fn update(&mut self, _message: Message) {}

    fn view(&self) -> Element<'_, Message> {
        canvas(Plot { data: &self.data })
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

fn main() -> iced::Result {
    iced::application(App::default, App::update, App::view)
        .antialiasing(true)
        .run()
}
