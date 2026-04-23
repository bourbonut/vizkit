use iced::widget::{Action, canvas};
use vizkit::chromatic::Color;
use vizkit::draw::{
    AxisOptions, CircleProperties, Draw, LineAttrs, LineProperties, ShapeAttrs, circle_iter,
};
use vizkit::scale::{Linear, ScaleContinuous, ScaleOrdinal};

use crate::data::Data;
use crate::iced_frame::IcedFrame;
use crate::{COLOR_DOMAIN, COLOR_RANGE, Margin, Message, RADIUS_RANGE};

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
        let mut iced_frame = IcedFrame(&mut frame);
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
        let text = canvas::Text {
            content: String::from("Occupation annual turnover rate"),
            position: [tx, height - self.margin.bottom + ty].into(),
            color: text_color,
            size: iced::Pixels(12.),
            font: bold_font,
            align_x: iced::Alignment::Center.into(),
            ..Default::default()
        };
        iced_frame.fill_text(text);

        // X axis domain line
        iced_frame.draw_line(LineProperties {
            start: [self.margin.left, height - self.margin.bottom],
            end: [width - self.margin.right, height - self.margin.bottom],
            ..Default::default()
        });

        // X axis ticks
        iced_frame.axis_bottom(
            &state.x_scale,
            height - self.margin.bottom,
            |tick| format!("{}%", (tick * 100.).round()),
            &AxisOptions {
                font_size: 10.,
                ..Default::default()
            },
        );

        // Vertical grid
        iced_frame.grid_vertical(
            &state.x_scale.ticks(None),
            self.margin.top,
            height - self.margin.bottom,
            |&x| state.x_scale.apply(x),
            |_| LineAttrs::default(),
        );

        // Y label with bold weight
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
        iced_frame.with_save(|frame| {
            frame.rotate(-std::f32::consts::PI * 0.5);

            let tx = self.margin.left * 0.9;
            let ty = (height - self.margin.bottom + self.margin.top) * 0.5;
            frame.translate([-ty, self.margin.left - tx].into());

            frame.fill_text(text);
        });

        // Y axis domain line
        iced_frame.draw_line(LineProperties {
            start: [self.margin.left, self.margin.top],
            end: [self.margin.left, height - self.margin.bottom],
            ..Default::default()
        });

        // Y axis ticks
        iced_frame.axis_left(
            &state.y_scale,
            self.margin.left,
            |tick| format!("${}k", (tick / 1000.).round()),
            &AxisOptions {
                font_size: 10.,
                ..Default::default()
            },
        );

        // Horizontal grid
        iced_frame.grid_horizontal(
            &state.y_scale.ticks(None),
            self.margin.left,
            width - self.margin.right,
            |&y| state.y_scale.apply(y),
            |_| LineAttrs::default(),
        );

        // Circles
        iced_frame.circle_from_props(state.circles.iter().cloned());

        // Y reference (horizontal line)
        iced_frame.draw_line(LineProperties {
            start: [self.margin.left, state.y_scale.apply(33_900.0)],
            end: [width - self.margin.right, state.y_scale.apply(33_900.0)],
            stroke_color: Color::from("666"),
            stroke_width: 1.5,
            stroke_opacity: 0.75,
        });

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

        // Note `Color::from(&s[1..])` because valid color strings are "??????" or "???" and not
        // "#??????" or "#???"
        state.circles = circle_iter(
            &self.data.items,
            |d| state.x_scale.apply(d.turnover),
            |d| state.y_scale.apply(d.median_wage),
            |d| r_scale.apply(d.openings),
            |d| ShapeAttrs {
                fill_color: color
                    .apply(d.sector_cat.as_str())
                    .map(|s| Color::from(&s[1..])),
                fill_opacity: 0.5,
                stroke_color: color
                    .apply(d.sector_cat.as_str())
                    .map(|s| Color::from(&s[1..])),
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
