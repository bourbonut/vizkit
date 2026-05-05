use chrono::{DateTime, NaiveDate, Utc};
use iced::{Element, Length, widget::canvas};
use std::{collections::HashMap, fs::File};
use vizkit::{
    draw::{Curve, PathCommand, path_iter},
    scale::{ScaleContinuous, ScaleTime},
};

const MARGIN_TOP: f32 = 10.;
const MARGIN_RIGHT: f32 = 30.;
const MARGIN_BOTTOM: f32 = 30.;
const MARGIN_LEFT: f32 = 40.;

fn datetime(year: i32, month: u32, day: u32) -> DateTime<Utc> {
    NaiveDate::from_ymd_opt(year, month, day)
        .and_then(|date| date.and_hms_opt(0, 0, 0))
        .expect("invalid time values")
        .and_utc()
}

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

struct Row {
    date: DateTime<Utc>,
    unemp_value: f32,
}

struct Data {
    values: HashMap<String, Vec<Row>>,
    unemp_domain: [f32; 2],
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
                .min_by(|a, b| a.partial_cmp(&b).unwrap_or(std::cmp::Ordering::Equal))
                .unwrap_or(f32::INFINITY),
        );
        unemp_max = unemp_max.max(
            rows.iter()
                .map(|row| row.unemp_value)
                .max_by(|a, b| a.partial_cmp(&b).unwrap_or(std::cmp::Ordering::Equal))
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
    type State = ();
    fn draw(
        &self,
        _state: &Self::State,
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

        self.data.values.values().for_each(|rows| {
            let path = canvas::Path::new(|builder| {
                path_iter(
                    rows,
                    |row| x_scale.apply(row.date) as f32,
                    |row| y_scale.apply(row.unemp_value),
                    Curve::Cardinal { tension: 0.0 },
                )
                .for_each(move |path_command| match path_command {
                    PathCommand::MoveTo(point) => builder.move_to(point.into()),
                    PathCommand::LineTo(point) => builder.line_to(point.into()),
                    PathCommand::BezierCurveTo([p1, p2, p3]) => {
                        builder.bezier_curve_to(p1.into(), p2.into(), p3.into())
                    }
                })
            });
            frame.stroke(
                &path,
                canvas::Stroke::default().with_color(iced::Color::WHITE),
            );
        });
        vec![frame.into_geometry()]
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
