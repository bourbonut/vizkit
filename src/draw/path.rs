mod cardinal;
mod linear;
mod step;

use std::collections::VecDeque;

use self::cardinal::Cardinal;
use self::linear::Linear;
use self::step::Step;

/// Commands for path building.
#[derive(Clone)]
pub enum PathCommand {
    /// Indicates to move the current point (or starting point) to the given point.
    MoveTo([f32; 2]),
    /// Indicates to connect the last point to the given point with a straight line.
    LineTo([f32; 2]),
    /// Indicates to create a bezier curve given the first two points as control points and the last
    /// point as end point.
    BezierCurveTo([[f32; 2]; 3]),
    /// Indicates to add a circular arc between the control points and with the specified radius.
    ArcTo(([f32; 2], [f32; 2], f32)),
    /// Indicates to close the path.
    ClosePath,
}

/// Trait for generating [`PathCommand`] in order to draw curves.
pub(crate) trait PathCurve {
    /// Indicates a new point in the current line. [`PathCommand`] must be added by using
    /// [`VecDeque::push_back`].
    fn point(&mut self, point: [f32; 2], buffer: &mut VecDeque<PathCommand>);
    /// Called when the curve is finished. Internal attributes should be reset and ending
    /// [`PathCommand`] must be added by using [`VecDeque::push_back`].
    fn end(&mut self, buffer: &mut VecDeque<PathCommand>);
}

/// Available curve variants for drawing a path or an area.
#[derive(Default)]
pub enum Curve {
    /// Linear curve produces lines with no specific modification (default)
    #[default]
    Linear,
    /// Cardinal curve produces cubic [cardinal
    /// splines](https://en.wikipedia.org/wiki/Cubic_Hermite_spline#Cardinal_spline) using the
    /// specified control points. The `tension` value must be in the range [0.0, 1.0]. If `tension`
    /// is `0.`, the cubic cardinal splines will be the most strechted whereas if `tension` is `1.`,
    /// the ended curve is closed to a curve drawn by [`Curve::Linear`].
    Cardinal { tension: f32 },
    /// Step curve produces alternative horirontal and vertical lines. The `tension` value must be
    /// in the range [0.0, 1.0]. If `tension` is `0.5`, the y-value change at the midpoint of each
    /// adjacent x-values whereas if `tension` is `0.0`, it produces y-value changes after the
    /// x-value and when `tension` is `1.0`, it produces y-value changes before the x-value.
    Step { tension: f32 },
}

/// Iterator for generating [`PathCommand`] giving a [`PathCurve`] structure
struct PathCurveIterator<I, C>
where
    I: Iterator<Item = [f32; 2]>,
    C: PathCurve,
{
    values: I,
    curve: C,
    buffer: VecDeque<PathCommand>,
    is_ended: bool,
}

impl<I, C> PathCurveIterator<I, C>
where
    I: Iterator<Item = [f32; 2]>,
    C: PathCurve,
{
    fn new(values: I, curve: C) -> Self {
        Self {
            values,
            curve,
            buffer: VecDeque::new(),
            is_ended: false,
        }
    }
}

impl<I, C> Iterator for PathCurveIterator<I, C>
where
    I: Iterator<Item = [f32; 2]>,
    C: PathCurve,
{
    type Item = PathCommand;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if !self.buffer.is_empty() {
                return self.buffer.pop_front();
            }

            if let Some(point) = self.values.next() {
                self.curve.point(point, &mut self.buffer);
            } else if !self.is_ended {
                self.curve.end(&mut self.buffer);
                self.is_ended = true;
            } else {
                return None;
            }
        }
    }
}

enum PathIterator<I>
where
    I: Iterator<Item = [f32; 2]>,
{
    Linear(PathCurveIterator<I, Linear>),
    Cardinal(PathCurveIterator<I, Cardinal>),
    Step(PathCurveIterator<I, Step>),
}

impl<I> PathIterator<I>
where
    I: Iterator<Item = [f32; 2]>,
{
    fn new(values: I, curve: &Curve, is_closed: bool) -> Self {
        match curve {
            Curve::Linear => Self::Linear(PathCurveIterator::new(values, Linear::new(is_closed))),
            Curve::Cardinal { tension } => Self::Cardinal(PathCurveIterator::new(
                values,
                Cardinal::new(*tension, is_closed),
            )),
            Curve::Step { tension } => Self::Step(PathCurveIterator::new(
                values,
                Step::new(*tension, is_closed),
            )),
        }
    }
}

impl<I> Iterator for PathIterator<I>
where
    I: Iterator<Item = [f32; 2]>,
{
    type Item = PathCommand;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Linear(it) => it.next(),
            Self::Cardinal(it) => it.next(),
            Self::Step(it) => it.next(),
        }
    }
}

/// Creates an iterator of [`PathCommand`] for drawing a path.
pub fn path_iter<'a, Data>(
    values: &'a [Data],
    x: impl Fn(&Data) -> f32 + 'a,
    y: impl Fn(&Data) -> f32 + 'a,
    curve: Curve,
) -> impl Iterator<Item = PathCommand> + 'a {
    PathIterator::new(
        values.iter().map(move |value| [x(value), y(value)]),
        &curve,
        false,
    )
}

/// Creates an iterator of [`PathCommand`] for drawing an area.
pub fn area_iter<'a, Data>(
    values: &'a [Data],
    x0: impl Fn(&Data) -> f32 + 'a,
    y0: impl Fn(&Data) -> f32 + 'a,
    x1: impl Fn(&Data) -> f32 + 'a,
    y1: impl Fn(&Data) -> f32 + 'a,
    curve: Curve,
) -> impl Iterator<Item = PathCommand> + 'a {
    PathIterator::new(
        values.iter().map(move |value| [x1(value), y1(value)]),
        &curve,
        false,
    )
    .chain(PathIterator::new(
        values.iter().map(move |value| [x0(value), y0(value)]).rev(),
        &curve,
        true,
    ))
}

/// Convenient function for creating an iterator of [`PathCommand`] for drawing area with shared x
/// values.
pub fn area_horizontal_iter<'a, Data>(
    values: &'a [Data],
    x: impl Fn(&Data) -> f32 + Clone + 'a,
    y0: impl Fn(&Data) -> f32 + 'a,
    y1: impl Fn(&Data) -> f32 + 'a,
    curve: Curve,
) -> impl Iterator<Item = PathCommand> + 'a {
    area_iter(values, x.clone(), y0, x, y1, curve)
}

/// Convenient function for creating an iterator of [`PathCommand`] for drawing area with shared y
/// values.
pub fn area_vertical_iter<'a, Data>(
    values: &'a [Data],
    y: impl Fn(&Data) -> f32 + Clone + 'a,
    x0: impl Fn(&Data) -> f32 + 'a,
    x1: impl Fn(&Data) -> f32 + 'a,
    curve: Curve,
) -> impl Iterator<Item = PathCommand> + 'a {
    area_iter(values, x0, y.clone(), x1, y.clone(), curve)
}
