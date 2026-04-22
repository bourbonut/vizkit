use std::fs::File;
use std::mem;
use svg::Document;
use svg::node::element;
use vizkit::{
    chromatic::Color,
    draw::{Alignment, AxisOptions, Draw, ShapeAttrs},
};

use serde::Deserialize;
use vizkit::scale::ScaleContinuous;

#[derive(Debug, Deserialize)]
struct Record {
    sex: String,
    height: Option<f32>,
    weight: Option<u32>,
}

struct SvgDrawer(element::SVG);

impl Draw for SvgDrawer {
    fn draw_line(&mut self, line: vizkit::draw::LineProperties) {
        let old = mem::replace(&mut self.0, element::SVG::new());
        self.0 = old.add(
            element::Line::new()
                .set("x1", line.start[0])
                .set("y1", line.start[1])
                .set("x2", line.end[0])
                .set("y2", line.end[1])
                .set("stroke", line.stroke_color.to_string())
                .set("stroke-width", line.stroke_width),
        );
    }

    fn draw_text(&mut self, text: vizkit::draw::TextProperties) {
        let old = mem::replace(&mut self.0, element::SVG::new());
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
        self.0 = old.add(element)
    }

    fn draw_circle(&mut self, circle: vizkit::draw::CircleProperties) {
        let old = mem::replace(&mut self.0, element::SVG::new());
        self.0 = old.add(
            element::Circle::new()
                .set("cx", circle.center[0])
                .set("cy", circle.center[1])
                .set("r", circle.radius)
                .set(
                    "stroke",
                    circle
                        .stroke_color
                        .map(|c| c.to_string())
                        .unwrap_or("none".to_string()),
                ),
        )
    }

    fn draw_rect(&mut self, _: vizkit::draw::RectProperties) {
        todo!()
    }
}

#[derive(Debug)]
struct Row {
    sex: bool,
    height: f32,
    weight: u32,
}

fn main() {
    let reader = File::open("src/athletes.csv").unwrap();
    let mut rdr = csv::Reader::from_reader(reader);
    let mut data: Vec<Row> = Vec::new();
    for result in rdr.deserialize() {
        let record: Record = result.unwrap();
        if let Some(height) = record.height
            && let Some(weight) = record.weight
        {
            data.push(Row {
                sex: if record.sex == "male" { false } else { true },
                height,
                weight,
            })
        }
    }

    let width = 640.;
    let height = 400.;

    let margin_top = 10.;
    let margin_left = 50.;
    let margin_bottom = 40.;
    let margin_right = 20.;

    let (x_domain, y_domain) = data.iter().fold(
        (
            [f32::INFINITY, f32::NEG_INFINITY],
            [f32::INFINITY, f32::NEG_INFINITY],
        ),
        |([xmin, xmax], [ymin, ymax]), row| {
            let w = row.weight as f32;
            let h = row.height;
            ([xmin.min(w), xmax.max(w)], [ymin.min(h), ymax.max(h)])
        },
    );

    let x = ScaleContinuous::linear()
        .domain(x_domain)
        .range([margin_left, width - margin_right]);

    let y = ScaleContinuous::linear()
        .domain(y_domain)
        .range([height - margin_bottom, margin_top]);

    let document = Document::new()
        .set("width", width)
        .set("height", height)
        .set("viewBox", (0, 0, width, height))
        .set("style", "background: black;");
    let mut drawer = SvgDrawer(document);

    let axis_options = AxisOptions {
        offset: 5.0,
        ..Default::default()
    };

    drawer.axis_bottom(
        &x,
        height - margin_bottom,
        |tick| tick.to_string(),
        &axis_options,
    );

    drawer.axis_left(&y, margin_left, |tick| tick.to_string(), &axis_options);

    drawer.circle(
        &data,
        |row| x.apply(row.weight as f32),
        |row| y.apply(row.height as f32),
        |_| 2.,
        |row| ShapeAttrs {
            stroke_color: Some(Color(if row.sex {
                [0.25, 0.25, 1.]
            } else {
                [0.75, 0.75, 0.]
            })),
            ..Default::default()
        },
    );

    svg::save("plot.svg", &drawer.0).unwrap()
}
