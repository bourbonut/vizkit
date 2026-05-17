use super::{Alignment, LineAttrs, LineProperties, Orientation, TextProperties};
use crate::chromatic::Color;
use crate::scale::Axis;

pub struct AxisOptions {
    pub tick_size: f32,
    pub offset: f32,
    pub count: Option<usize>,
    pub line_attrs: LineAttrs,
    pub text_fill_color: Color,
    pub font_size: f32,
}

struct AxisPlacement {
    at: f32,
    orientation: Orientation,
    direction: f32,
    align_x: Alignment,
    align_y: Alignment,
}

impl Default for AxisOptions {
    fn default() -> Self {
        Self {
            tick_size: 7.5,
            offset: 0.5,
            count: None,
            line_attrs: LineAttrs::default(),
            text_fill_color: Color::default(),
            font_size: 12.,
        }
    }
}

/// Creates an iterator of properties used for axis positioned on the top of the chart.
pub fn axis_top_iter<A: Axis>(
    scaler: &A,
    y: f32,
    formatter: impl Fn(&A::Tick) -> String,
    axis_options: &AxisOptions,
) -> impl Iterator<Item = (LineProperties, TextProperties)> {
    axis(
        scaler,
        AxisPlacement {
            at: y,
            orientation: Orientation::Same,
            direction: -1.,
            align_x: Alignment::Center,
            align_y: Alignment::End,
        },
        formatter,
        axis_options,
    )
}

/// Creates an iterator of properties used for axis positioned on the right of the chart.
pub fn axis_right_iter<A: Axis>(
    scaler: &A,
    x: f32,
    formatter: impl Fn(&A::Tick) -> String,
    axis_options: &AxisOptions,
) -> impl Iterator<Item = (LineProperties, TextProperties)> {
    axis(
        scaler,
        AxisPlacement {
            at: x,
            orientation: Orientation::Flip,
            direction: 1.,
            align_x: Alignment::Start,
            align_y: Alignment::Center,
        },
        formatter,
        axis_options,
    )
}

/// Creates an iterator of properties used for axis positioned on the bottom of the chart.
pub fn axis_bottom_iter<A: Axis>(
    scaler: &A,
    y: f32,
    formatter: impl Fn(&A::Tick) -> String,
    axis_options: &AxisOptions,
) -> impl Iterator<Item = (LineProperties, TextProperties)> {
    axis(
        scaler,
        AxisPlacement {
            at: y,
            orientation: Orientation::Same,
            direction: 1.,
            align_x: Alignment::Center,
            align_y: Alignment::Start,
        },
        formatter,
        axis_options,
    )
}

/// Creates an iterator of properties used for axis positioned on the left of the chart.
pub fn axis_left_iter<A: Axis>(
    scaler: &A,
    x: f32,
    formatter: impl Fn(&A::Tick) -> String,
    axis_options: &AxisOptions,
) -> impl Iterator<Item = (LineProperties, TextProperties)> {
    axis(
        scaler,
        AxisPlacement {
            at: x,
            orientation: Orientation::Flip,
            direction: -1.,
            align_x: Alignment::End,
            align_y: Alignment::Center,
        },
        formatter,
        axis_options,
    )
}

fn axis<A: Axis>(
    scaler: &A,
    placement: AxisPlacement,
    formatter: impl Fn(&A::Tick) -> String,
    axis_options: &AxisOptions,
) -> impl Iterator<Item = (LineProperties, TextProperties)> {
    let ticks = scaler.ticks(axis_options.count);
    let AxisPlacement {
        at,
        orientation,
        direction,
        align_x,
        align_y,
    } = placement;
    ticks.into_iter().map(move |tick| {
        let content = formatter(&tick);
        let pos = scaler.tick_position(tick);
        (
            LineProperties {
                start: orientation.apply(pos, at),
                end: orientation.apply(pos, at + direction * axis_options.tick_size),
                stroke_color: axis_options.line_attrs.stroke_color,
                stroke_width: axis_options.line_attrs.stroke_width,
                stroke_opacity: axis_options.line_attrs.stroke_opacity,
            },
            TextProperties {
                position: orientation.apply(
                    pos,
                    at + direction * (axis_options.tick_size + axis_options.offset),
                ),
                content,
                fill_color: axis_options.text_fill_color,
                font_size: axis_options.font_size,
                align_x: align_x.clone(),
                align_y: align_y.clone(),
            },
        )
    })
}

#[cfg(test)]
mod tests {
    use super::{axis_bottom_iter, axis_left_iter, axis_right_iter, axis_top_iter};
    use crate::draw::{AxisOptions, LineProperties, TextProperties};
    use crate::scale::{Axis, ScaleContinuous};
    use rstest::rstest;

    const WIDTH: f32 = 400.;
    const HEIGHT: f32 = 100.;

    const MARGIN_LEFT: f32 = 10.;
    const MARGIN_TOP: f32 = 10.;

    const XMAX: f32 = 50.;
    const YMAX: f32 = 50.;

    #[rstest]
    #[case("bottom", 0, [0., XMAX], [0., WIDTH], HEIGHT, HEIGHT, HEIGHT + 7.5, HEIGHT + 7.5 + 0.5)]
    #[case("top", 0, [0., XMAX], [0., WIDTH], MARGIN_TOP, MARGIN_TOP, MARGIN_TOP - 7.5, MARGIN_TOP - 7.5 - 0.5)]
    #[case("left", 1, [0., YMAX], [HEIGHT, 0.], MARGIN_LEFT, MARGIN_LEFT, MARGIN_LEFT - 7.5, MARGIN_LEFT - 7.5 - 0.5)]
    #[case("right", 1, [0., YMAX], [HEIGHT, 0.], WIDTH, WIDTH, WIDTH + 7.5, WIDTH + 7.5 + 0.5)]
    fn test_axis(
        #[case] title: &str,
        #[case] index: usize,
        #[case] domain: [f32; 2],
        #[case] range: [f32; 2],
        #[case] at: f32,
        #[case] start: f32,
        #[case] end: f32,
        #[case] position: f32,
    ) {
        let scale = ScaleContinuous::linear().domain(domain).range(range);

        let formatter = |x: &f32| x.to_string();
        let options = AxisOptions::default();
        let (lines, texts): (Vec<LineProperties>, Vec<TextProperties>) = match title {
            "bottom" => axis_bottom_iter(&scale, at, formatter, &options).unzip(),
            "top" => axis_top_iter(&scale, at, formatter, &options).unzip(),
            "left" => axis_left_iter(&scale, at, formatter, &options).unzip(),
            "right" => axis_right_iter(&scale, at, formatter, &options).unzip(),
            _ => unreachable!(),
        };

        // Indices
        let a = index;
        let b = (index + 1) % 2;

        // Expected values
        let tick_fn = |&tick: &f32| scale.tick_position(tick);
        let scale_ticks: Vec<f32> = scale.ticks(None).iter().map(tick_fn).collect();
        let string_ticks: Vec<String> = scale.ticks(None).iter().map(ToString::to_string).collect();

        // Test line properties
        for (i, line) in lines.iter().enumerate() {
            assert_eq!(line.start[a], line.end[a], "{}", title);
            assert_eq!(line.start[b], start, "{}", title);
            assert_eq!(line.end[b], end, "{}", title);
            assert_eq!(line.start[a], scale_ticks[i], "{}", title);
        }

        // Test text properties
        for (i, text) in texts.iter().enumerate() {
            assert_eq!(text.position[b], position, "{}", title);
            assert_eq!(text.position[a], scale_ticks[i], "{}", title);
            assert_eq!(text.content, string_ticks[i], "{}", title);
        }
    }
}
