use iced::widget::{Action, canvas};
use std::str::FromStr;
use vizkit::chromatic::Color;
use vizkit::draw::{axis_bottom_iter, axis_left_iter, grid_horizontal_iter, grid_vertical_iter};
use vizkit::{
    draw::{
        Alignment, AxisOptions, CircleProperties, LineAttrs, LineProperties, ShapeAttrs,
        TextProperties, circle_iter,
    },
    scale::{Axis, Linear, ScaleContinuous, ScaleOrdinal},
};

use crate::data::Data;
use crate::{COLOR_DOMAIN, COLOR_RANGE, Margin, Message, RADIUS_RANGE};

pub fn line(frame: &mut canvas::Frame, line: LineProperties) {
    let [r, g, b] = line.stroke_color.into();
    frame.stroke(
        &canvas::Path::line(line.start.into(), line.end.into()),
        canvas::Stroke::default()
            .with_color(iced::Color::from([r, g, b, line.stroke_opacity]))
            .with_width(line.stroke_width),
    );
}

pub fn text(frame: &mut canvas::Frame, text: TextProperties) {
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

pub fn circle(frame: &mut canvas::Frame, circle: &CircleProperties) {
    let circle_path = canvas::Path::circle(circle.center.into(), circle.radius);
    if let Some(fill_color) = circle.fill_color {
        let fill_color: [f32; 3] = fill_color.into();
        frame.fill(
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
        frame.stroke(
            &circle_path,
            canvas::Stroke::default()
                .with_width(circle.stroke_width)
                .with_color(iced::Color::from(stroke_color)),
        );
    }
}

pub struct Plot<'a> {
    data: &'a Data,
    margin: Margin,
}

impl<'a> Plot<'a> {
    pub fn new(data: &'a Data, margin: Margin) -> Self {
        Self { data: data, margin }
    }
}

pub struct PlotState {
    x_scale: ScaleContinuous<Linear>,
    y_scale: ScaleContinuous<Linear>,
    circles: Vec<CircleProperties>,
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
        let mut frame = canvas::Frame::new(renderer, bounds.size());
        let width = bounds.width;
        let height = bounds.height;

        let text_color = theme.palette().text;
        let bold_font = iced::Font {
            weight: iced::font::Weight::Bold,
            ..Default::default()
        };

        // X label with bold weight
        let tx = (self.margin.left + width - self.margin.right) * 0.5;
        let ty = self.margin.bottom * 0.5;
        frame.fill_text(canvas::Text {
            content: String::from("Occupation annual turnover rate"),
            position: [tx, height - self.margin.bottom + ty].into(),
            color: text_color,
            size: iced::Pixels(12.),
            font: bold_font,
            align_x: iced::Alignment::Center.into(),
            ..Default::default()
        });

        // X axis domain line
        line(
            &mut frame,
            LineProperties {
                start: [self.margin.left, height - self.margin.bottom],
                end: [width - self.margin.right, height - self.margin.bottom],
                ..Default::default()
            },
        );

        // X axis ticks
        axis_bottom_iter(
            &state.x_scale,
            height - self.margin.bottom,
            |tick| format!("{}%", (tick * 100.).round()),
            &AxisOptions {
                font_size: 10.,
                ..Default::default()
            },
        )
        .for_each(|(line_props, text_props)| {
            line(&mut frame, line_props);
            text(&mut frame, text_props);
        });

        // Vertical grid
        grid_vertical_iter(
            &state.x_scale.ticks(None),
            self.margin.top,
            height - self.margin.bottom,
            |&x| state.x_scale.apply(x),
            |_| LineAttrs::default(),
        )
        .for_each(|line_props| line(&mut frame, line_props));

        // Y label with bold weight and rotate the text
        frame.with_save(|frame| {
            frame.rotate(-std::f32::consts::PI * 0.5);

            let tx = self.margin.left * 0.9;
            let ty = (height - self.margin.bottom + self.margin.top) * 0.5;
            frame.translate([-ty, self.margin.left - tx].into());

            frame.fill_text(canvas::Text {
                content: String::from("Median wage, 2018"),
                position: [0., 0.].into(),
                color: text_color,
                size: iced::Pixels(12.),
                font: bold_font,
                align_x: iced::Alignment::Center.into(),
                ..Default::default()
            });
        });

        // Y axis domain line
        line(
            &mut frame,
            LineProperties {
                start: [self.margin.left, self.margin.top],
                end: [self.margin.left, height - self.margin.bottom],
                ..Default::default()
            },
        );

        // Y axis ticks
        axis_left_iter(
            &state.y_scale,
            self.margin.left,
            |tick| format!("${}k", (tick / 1000.).round()),
            &AxisOptions {
                font_size: 10.,
                ..Default::default()
            },
        )
        .for_each(|(line_props, text_props)| {
            line(&mut frame, line_props);
            text(&mut frame, text_props);
        });

        // Horizontal grid
        grid_horizontal_iter(
            &state.y_scale.ticks(None),
            self.margin.left,
            width - self.margin.right,
            |&y| state.y_scale.apply(y),
            |_| LineAttrs::default(),
        )
        .for_each(|line_props| line(&mut frame, line_props));

        // Circles
        state
            .circles
            .iter()
            .for_each(|circle_props| circle(&mut frame, circle_props));

        // Y reference (horizontal line)
        line(
            &mut frame,
            LineProperties {
                start: [self.margin.left, state.y_scale.apply(33_900.0)],
                end: [width - self.margin.right, state.y_scale.apply(33_900.0)],
                stroke_color: Color::from_str("666").unwrap_or_default(),
                stroke_width: 1.5,
                stroke_opacity: 0.75,
            },
        );

        vec![frame.into_geometry()]
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
            .range(RADIUS_RANGE);

        let color = ScaleOrdinal::default()
            .domain(&COLOR_DOMAIN)
            .range(&COLOR_RANGE);

        state.circles = circle_iter(
            &self.data.items,
            |d| state.x_scale.apply(d.turnover),
            |d| state.y_scale.apply(d.median_wage),
            |d| r_scale.apply(d.openings),
            |d| ShapeAttrs {
                fill_color: color
                    .apply(d.sector_cat.as_str())
                    .map(|s| Color::from_str(s).unwrap_or_default()),
                fill_opacity: 0.5,
                stroke_color: color
                    .apply(d.sector_cat.as_str())
                    .map(|s| Color::from_str(s).unwrap_or_default()),
                ..Default::default()
            },
        )
        .collect();

        // Hovered circle for tooltip information
        if let Some(position) = cursor.position() {
            // Computes the index of the closest circle to the cursor position
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
                    let dist = x.hypot(y);
                    if dist > r {
                        None
                    } else {
                        Some((r - dist, idx))
                    }
                })
                .min_by(|(a, _), (b, _)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
            match argmin {
                Some((_, idx)) => Some(Action::publish(Message::HoverCircle(idx))),
                None => Some(Action::publish(Message::None)),
            }
        } else {
            Some(Action::publish(Message::None))
        }
    }
}
