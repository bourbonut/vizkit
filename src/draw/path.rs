mod cardinal;
mod linear;
mod step;

use std::collections::VecDeque;

use self::cardinal::Cardinal;
use self::linear::Linear;
use self::step::Step;

#[derive(Clone)]
pub enum PathCommand {
    MoveTo([f32; 2]),
    LineTo([f32; 2]),
    BezierCurveTo([[f32; 2]; 3]),
    ArcTo(([f32; 2], [f32; 2], f32)),
    ClosePath,
}

pub(crate) trait PathCurve {
    fn point(&mut self, point: [f32; 2], buffer: &mut VecDeque<PathCommand>);
    fn end(&mut self, buffer: &mut VecDeque<PathCommand>);
}

#[derive(Default)]
pub enum Curve {
    #[default]
    Linear,
    Cardinal {
        tension: f32,
    },
    Step {
        tension: f32,
    },
}

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
