use std::marker::PhantomData;

use crate::scale::{ScaleContinuous, Tick, Transformer};

#[derive(Debug)]
pub struct TickLine {
    pub start: [f32; 2],
    pub end: [f32; 2],
}

#[derive(Debug)]
pub struct Label {
    pub tick_value: f32,
    pub position: [f32; 2],
}

pub trait Orientation {
    fn direction() -> f32;
    fn orient(coord1: f32, coord2: f32) -> [f32; 2];
}

pub struct Top;
pub struct Right;
pub struct Bottom;
pub struct Left;

impl Orientation for Top {
    fn direction() -> f32 {
        -1.
    }

    fn orient(coord1: f32, coord2: f32) -> [f32; 2] {
        [coord1, coord2]
    }
}

impl Orientation for Right {
    fn direction() -> f32 {
        1.
    }

    fn orient(coord1: f32, coord2: f32) -> [f32; 2] {
        [coord2, coord1]
    }
}

impl Orientation for Bottom {
    fn direction() -> f32 {
        1.
    }

    fn orient(coord1: f32, coord2: f32) -> [f32; 2] {
        [coord1, coord2]
    }
}

impl Orientation for Left {
    fn direction() -> f32 {
        -1.
    }

    fn orient(coord1: f32, coord2: f32) -> [f32; 2] {
        [coord2, coord1]
    }
}

pub struct Axis<O: Orientation> {
    marker: PhantomData<O>,
    tick_size: f32,
    offset: f32,
    at: f32,
    count: Option<usize>,
}

impl Axis<Top> {
    pub fn top(at: f32) -> Self {
        Self {
            marker: PhantomData,
            tick_size: 7.5,
            count: None,
            offset: 0.5,
            at,
        }
    }
}

impl Axis<Right> {
    pub fn right(at: f32) -> Self {
        Self {
            marker: PhantomData,
            tick_size: 7.5,
            count: None,
            offset: 0.5,
            at,
        }
    }
}

impl Axis<Bottom> {
    pub fn bottom(at: f32) -> Self {
        Self {
            marker: PhantomData,
            tick_size: 7.5,
            count: None,
            offset: 0.5,
            at,
        }
    }
}

impl Axis<Left> {
    pub fn left(at: f32) -> Self {
        Self {
            marker: PhantomData,
            tick_size: 7.5,
            count: None,
            offset: 0.5,
            at,
        }
    }
}

impl<O: Orientation> Axis<O> {
    pub fn tick_size(self, tick_size: f32) -> Self {
        Self { tick_size, ..self }
    }

    pub fn count(self, count: Option<usize>) -> Self {
        Self { count, ..self }
    }

    pub fn draw<T: Tick + Transformer>(
        &self,
        scale: &ScaleContinuous<T>,
        mut draw_fn: impl FnMut(TickLine, Label),
    ) {
        for tick_value in scale.ticks(self.count) {
            let tick_coord = scale.apply(tick_value);
            let dir = O::direction();
            draw_fn(
                TickLine {
                    start: O::orient(tick_coord, self.at),
                    end: O::orient(tick_coord, self.at + dir * self.tick_size),
                },
                Label {
                    tick_value,
                    position: O::orient(tick_coord, self.at + dir * (self.tick_size + self.offset)),
                },
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Axis;
    use crate::scale::ScaleContinuous;

    #[test]
    fn test_axis_bottom() {
        let height = 100.;
        let width = 400.;
        let xmax = 50.;
        let scale = ScaleContinuous::linear()
            .domain([0., xmax])
            .range([0., width]);

        let mut ticks = Vec::new();
        let mut labels = Vec::new();
        let axis = Axis::bottom(height);
        axis.draw(&scale, |tick, label| {
            ticks.push(tick);
            labels.push(label);
        });

        for tick_line in ticks.iter() {
            assert_eq!(tick_line.start[0], tick_line.end[0]);
            assert_eq!(tick_line.start[1], height);
            assert_eq!(tick_line.end[1], height + 7.5);
        }

        let scale_ticks = scale
            .ticks(None)
            .into_iter()
            .map(|tick| scale.apply(tick))
            .collect::<Vec<f32>>();

        assert_eq!(
            ticks
                .iter()
                .map(|tick_line| tick_line.start[0])
                .collect::<Vec<f32>>(),
            scale_ticks,
        );

        for label in labels.iter() {
            assert_eq!(label.position[1], height + 7.5 + 0.5);
        }

        assert_eq!(
            labels
                .iter()
                .map(|label| label.position[0])
                .collect::<Vec<f32>>(),
            scale_ticks,
        );

        assert_eq!(
            labels
                .iter()
                .map(|label| label.tick_value)
                .collect::<Vec<f32>>(),
            scale.ticks(None)
        );
    }

    #[test]
    fn test_axis_top() {
        let margin_top = 10.;
        let width = 400.;
        let xmax = 50.;
        let scale = ScaleContinuous::linear()
            .domain([0., xmax])
            .range([0., width]);

        let mut ticks = Vec::new();
        let mut labels = Vec::new();
        let axis = Axis::top(margin_top);
        axis.draw(&scale, |tick, label| {
            ticks.push(tick);
            labels.push(label);
        });

        for tick_line in ticks.iter() {
            assert_eq!(tick_line.start[0], tick_line.end[0]);
            assert_eq!(tick_line.start[1], margin_top);
            assert_eq!(tick_line.end[1], margin_top - 7.5);
        }

        let scale_ticks = scale
            .ticks(None)
            .into_iter()
            .map(|tick| scale.apply(tick))
            .collect::<Vec<f32>>();

        assert_eq!(
            ticks
                .iter()
                .map(|tick_line| tick_line.start[0])
                .collect::<Vec<f32>>(),
            scale_ticks,
        );

        for label in labels.iter() {
            assert_eq!(label.position[1], margin_top - 7.5 - 0.5);
        }

        assert_eq!(
            labels
                .iter()
                .map(|label| label.position[0])
                .collect::<Vec<f32>>(),
            scale_ticks,
        );

        assert_eq!(
            labels
                .iter()
                .map(|label| label.tick_value)
                .collect::<Vec<f32>>(),
            scale.ticks(None)
        );
    }

    #[test]
    fn test_axis_left() {
        let height = 100.;
        let margin_left = 10.;
        let ymax = 50.;
        let scale = ScaleContinuous::linear()
            .domain([0., ymax])
            .range([height, 0.]);

        let mut ticks = Vec::new();
        let mut labels = Vec::new();
        let axis = Axis::left(margin_left);
        axis.draw(&scale, |tick, label| {
            ticks.push(tick);
            labels.push(label);
        });

        for tick_line in ticks.iter() {
            assert_eq!(tick_line.start[0], margin_left);
            assert_eq!(tick_line.end[0], margin_left - 7.5);
            assert_eq!(tick_line.start[1], tick_line.end[1]);
        }

        let scale_ticks = scale
            .ticks(None)
            .into_iter()
            .map(|tick| scale.apply(tick))
            .collect::<Vec<f32>>();

        assert_eq!(
            ticks
                .iter()
                .map(|tick_line| tick_line.start[1])
                .collect::<Vec<f32>>(),
            scale_ticks,
        );

        for label in labels.iter() {
            assert_eq!(label.position[0], margin_left - 7.5 - 0.5);
        }

        assert_eq!(
            labels
                .iter()
                .map(|label| label.position[1])
                .collect::<Vec<f32>>(),
            scale_ticks,
        );

        assert_eq!(
            labels
                .iter()
                .map(|label| label.tick_value)
                .collect::<Vec<f32>>(),
            scale.ticks(None)
        );
    }

    #[test]
    fn test_axis_right() {
        let height = 100.;
        let width = 400.;
        let ymax = 50.;
        let scale = ScaleContinuous::linear()
            .domain([0., ymax])
            .range([height, 0.]);

        let mut ticks = Vec::new();
        let mut labels = Vec::new();
        let axis = Axis::right(width);
        axis.draw(&scale, |tick, label| {
            ticks.push(tick);
            labels.push(label);
        });

        for tick_line in ticks.iter() {
            assert_eq!(tick_line.start[0], width);
            assert_eq!(tick_line.end[0], width + 7.5);
            assert_eq!(tick_line.start[1], tick_line.end[1]);
        }

        let scale_ticks = scale
            .ticks(None)
            .into_iter()
            .map(|tick| scale.apply(tick))
            .collect::<Vec<f32>>();

        assert_eq!(
            ticks
                .iter()
                .map(|tick_line| tick_line.start[1])
                .collect::<Vec<f32>>(),
            scale_ticks,
        );

        for label in labels.iter() {
            assert_eq!(label.position[0], width + 7.5 + 0.5);
        }

        assert_eq!(
            labels
                .iter()
                .map(|label| label.position[1])
                .collect::<Vec<f32>>(),
            scale_ticks,
        );

        assert_eq!(
            labels
                .iter()
                .map(|label| label.tick_value)
                .collect::<Vec<f32>>(),
            scale.ticks(None)
        );
    }
}
