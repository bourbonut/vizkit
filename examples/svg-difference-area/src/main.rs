use std::cmp::Ordering;
use std::fs::File;

use chrono::{DateTime, Datelike, Utc};
use lyon_path::{Event, Path};
use svg::{Document, node::element};
use vizkit::{
    chromatic::{DivergingSpace, Scheme},
    draw::{
        Alignment, AxisOptions, Curve, LineProperties, PathCommand, TextProperties, area_iter,
        axis_bottom_iter, axis_left_iter, path_iter,
    },
    scale::{ScaleContinuous, ScaleTime},
};

// Structures for data loading and processing

// Data directly found into the CSV files
#[derive(Debug, serde::Deserialize)]
struct Record {
    #[serde(alias = "Date")]
    date: String,
    #[serde(alias = "Mean.TemperatureF")]
    mean: Option<f32>,
}

// Data processed
struct Row {
    date: DateTime<Utc>,
    nyc: f32,
    sanfrancisco: f32,
}

fn load_data(source: &str) -> Vec<Result<Record, csv::Error>> {
    let reader = File::open(source).unwrap();
    let mut rdr = csv::Reader::from_reader(reader);
    rdr.deserialize().collect()
}

fn parse_date(date: &str, fmt: &str) -> Result<DateTime<Utc>, chrono::ParseError> {
    chrono::NaiveDate::parse_from_str(date, fmt)
        .map(|naive_date| naive_date.and_hms_opt(0, 0, 0).unwrap().and_utc())
}

// Compares two floats and returns the order
fn cmp_f32(a: &f32, b: &f32) -> Ordering {
    a.partial_cmp(&b).unwrap_or(Ordering::Equal)
}

// Functions for creating SVG elements such as lines, texts and paths

fn line(line: LineProperties) -> element::Line {
    element::Line::new()
        .set("x1", line.start[0])
        .set("y1", line.start[1])
        .set("x2", line.end[0])
        .set("y2", line.end[1])
        .set("stroke", line.stroke_color.to_string())
        .set("stroke-width", line.stroke_width)
}

fn text(text: TextProperties) -> element::Text {
    let element = element::Text::new(&text.content)
        .set("fill", text.fill_color.to_string())
        .set("font-size", text.font_size)
        .set(
            "transform",
            format!("translate({}, {})", text.position[0], text.position[1]),
        );
    let element = match text.align_x {
        Alignment::Start => element.set("text-anchor", "start"),
        Alignment::End => element.set("text-anchor", "end"),
        Alignment::Center => element.set("text-anchor", "middle"),
    };
    let element = match text.align_y {
        Alignment::Start => element.set("y", "0.71em"),
        Alignment::Center => element.set("y", "0.31em"),
        Alignment::End => element.set("y", "0px"),
    };
    element
}

fn axis_builder(
    g: element::Group,
    (line_props, text_props): (LineProperties, TextProperties),
) -> element::Group {
    g.add(line(line_props)).add(text(text_props))
}

fn path_build<I: Iterator<Item = PathCommand>>(path_commands: I) -> element::Path {
    let mut builder = Path::builder().with_svg();
    for path_command in path_commands {
        match path_command {
            PathCommand::MoveTo(point) => {
                builder.move_to(point.into());
            }
            PathCommand::LineTo(point) => {
                builder.line_to(point.into());
            }
            PathCommand::BezierCurveTo([p1, p2, p3]) => {
                builder.cubic_bezier_to(p1.into(), p2.into(), p3.into());
            }
            PathCommand::ClosePath => {
                builder.close();
            }
            _ => (),
        }
    }
    let mut d_string = String::new();
    for event in &builder.build() {
        match event {
            Event::Begin { at } => {
                d_string.push_str(&format!("M{},{}", at.x, at.y));
            }
            Event::Line { from: _, to } => {
                d_string.push_str(&format!("L{},{}", to.x, to.y));
            }
            Event::Quadratic { from: _, ctrl, to } => {
                d_string.push_str(&format!("Q{},{},{},{}", ctrl.x, ctrl.y, to.x, to.y));
            }
            Event::Cubic {
                from: _,
                ctrl1,
                ctrl2,
                to,
            } => {
                d_string.push_str(&format!(
                    "C{},{},{},{},{},{}",
                    ctrl1.x, ctrl1.y, ctrl2.x, ctrl2.y, to.x, to.y
                ));
            }
            Event::End {
                last: _,
                first: _,
                close,
            } => {
                if close {
                    d_string.push_str("Z");
                }
            }
        }
    }
    element::Path::new().set("d", d_string)
}

fn main() {
    // Loads the data
    let mut nyc_data = load_data("src/nyc.csv");
    let mut sanfrancisco_data = load_data("src/sanfrancisco.csv");

    // Finds corresponding dates between New York City and San Francisco
    // and store them into `data`
    let mut data: Vec<Row> = Vec::new();
    let mut next_nyc = nyc_data.pop();
    let mut next_sfc = sanfrancisco_data.pop();
    while let Some(Ok(ref nyc)) = next_nyc
        && let Some(Ok(ref sfc)) = next_sfc
    {
        let nyc_date = parse_date(&nyc.date, "%Y-%m-%d").unwrap();
        let sfc_date = parse_date(&sfc.date, "%Y-%m-%d").unwrap();
        if nyc_date == sfc_date {
            if nyc.mean.is_some() && sfc.mean.is_some() {
                data.push(Row {
                    date: nyc_date.to_utc(),
                    nyc: nyc.mean.unwrap(),
                    sanfrancisco: sfc.mean.unwrap(),
                });
            }
            next_nyc = nyc_data.pop();
            next_sfc = sanfrancisco_data.pop();
        } else if nyc_date > sfc_date {
            next_nyc = nyc_data.pop();
        } else {
            next_sfc = sanfrancisco_data.pop();
        }
    }

    // Take a small window of data
    let data: Vec<Row> = data
        .into_iter()
        .filter(|d| {
            parse_date("2011-10-01", "%Y-%m-%d").unwrap() <= d.date
                && d.date < parse_date("2012-09-30", "%Y-%m-%d").unwrap()
        })
        .rev() // not necessary
        .collect();

    // Dimensions of the SVG
    let width = 928.;
    let height = 600.;

    let margin_top: f32 = 10.;
    let margin_left: f32 = 50.;
    let margin_bottom: f32 = 40.;
    let margin_right: f32 = 20.;

    // Time scaler for `date: DateTime<Utc>` and linear continuous scaler for `nyc: f32` and
    // `sanfrancisco: f32`
    let x_scale = ScaleTime::default()
        .domain([
            data.iter().map(|d| d.date).min().unwrap(),
            data.iter().map(|d| d.date).max().unwrap(),
        ])
        .range([margin_left as f64, (width - margin_right) as f64]);

    let y_scale = ScaleContinuous::linear()
        .domain([
            data.iter()
                .map(|d| d.nyc.min(d.sanfrancisco))
                .min_by(cmp_f32)
                .unwrap(),
            data.iter()
                .map(|d| d.nyc.max(d.sanfrancisco))
                .max_by(cmp_f32)
                .unwrap(),
        ])
        .range([height - margin_bottom, margin_top]);

    // SVG container
    let document = Document::new()
        .set("width", width)
        .set("height", height)
        .set("viewBox", (0, 0, width, height))
        .set("style", "background: black;");

    // Build axis (axis bottom for x and axis left for y)
    let axis_options = AxisOptions {
        offset: 5.0, // offset of ticks
        ..Default::default()
    };

    let x_axis = axis_bottom_iter(
        &x_scale,
        height - margin_bottom,
        |tick| format!("{:02}/{}", tick.month(), tick.year() - 2000),
        &axis_options,
    )
    .fold(element::Group::new().set("class", "x-axis"), axis_builder);

    let y_axis = axis_left_iter(
        &y_scale,
        margin_left,
        |tick| tick.to_string(),
        &axis_options,
    )
    .fold(element::Group::new().set("class", "y-axis"), axis_builder);

    // Middle white line
    let path = path_build(path_iter(
        &data,
        |d| x_scale.scale(d.date) as f32,
        |d| y_scale.scale(d.sanfrancisco),
        Curve::Step { tension: 0.5 },
    ))
    .set("fill", "none")
    .set("stroke", "white");

    // Areas
    let scheme = DivergingSpace::RdYlBu.scheme();
    let area_blue = path_build(area_iter(
        &data,
        |d| x_scale.scale(d.date) as f32,
        |d| y_scale.scale(d.sanfrancisco),
        |d| x_scale.scale(d.date) as f32,
        |d| y_scale.scale(d.nyc.max(d.sanfrancisco)),
        Curve::Step { tension: 0.5 },
    ))
    .set("fill", format!("#{}", scheme[8]))
    .set("stroke", "none");

    let area_orange = path_build(area_iter(
        &data,
        |d| x_scale.scale(d.date) as f32,
        |d| y_scale.scale(d.nyc),
        |d| x_scale.scale(d.date) as f32,
        |d| y_scale.scale(d.nyc.max(d.sanfrancisco)),
        Curve::Step { tension: 0.5 },
    ))
    .set("fill", format!("#{}", scheme[2]))
    .set("stroke", "none");

    let y_label = text(TextProperties {
        content: "Temperature (°F)".to_string(),
        position: [10., 15.],
        align_x: Alignment::Start,
        align_y: Alignment::Center,
        ..Default::default()
    });

    svg::save(
        "plot.svg",
        &document
            .add(x_axis)
            .add(y_axis)
            .add(y_label)
            .add(area_blue)
            .add(area_orange)
            .add(path),
    )
    .unwrap()
}
