use std::array::from_fn;

use crate::{scale::Axis, time::TimeInterval};

use chrono::{DateTime, FixedOffset, Local, TimeDelta, TimeZone, Utc};

const DURATION_YEAR: f64 = 365. * 24. * 60. * 60. * 1000.;

enum IntervalStrategy {
    Millisecond,
    Second,
    Minute,
    Hour,
    Day,
    Month,
    Year,
}

impl IntervalStrategy {
    #[rustfmt::skip]
    fn create_range(
        &self,
        start: DateTime<Utc>,
        stop: DateTime<Utc>,
        step: u32,
    ) -> Vec<DateTime<Utc>> {
        let stop_offset = stop + TimeDelta::microseconds(1);
        match self {
            Self::Millisecond => TimeInterval::millisecond().every(step).range(start, stop_offset, 1),
            Self::Second =>      TimeInterval::second().every(step).range(start, stop_offset, 1),
            Self::Minute =>      TimeInterval::minute().every(step).range(start, stop_offset, 1),
            Self::Hour =>        TimeInterval::hour().every(step).range(start, stop_offset, 1),
            Self::Day =>         TimeInterval::day().every(step).range(start, stop_offset, 1),
            Self::Month =>       TimeInterval::month().every(step).range(start, stop_offset, 1),
            Self::Year =>        TimeInterval::year().every(step).range(start, stop_offset, 1),
        }
    }
}

struct TickDelta {
    delta: TimeDelta,
    strategy: IntervalStrategy,
    step: u32,
}

#[rustfmt::skip]
const TICK_DELTAS: &[TickDelta] = &[
    TickDelta {delta: TimeDelta::seconds(1),    strategy: IntervalStrategy::Second, step: 1},
    TickDelta {delta: TimeDelta::seconds(5),    strategy: IntervalStrategy::Second, step: 5},
    TickDelta {delta: TimeDelta::seconds(15),   strategy: IntervalStrategy::Second, step: 15},
    TickDelta {delta: TimeDelta::seconds(30),   strategy: IntervalStrategy::Second, step: 30},
    TickDelta {delta: TimeDelta::minutes(1),    strategy: IntervalStrategy::Minute, step: 1},
    TickDelta {delta: TimeDelta::minutes(5),    strategy: IntervalStrategy::Minute, step: 5},
    TickDelta {delta: TimeDelta::minutes(15),   strategy: IntervalStrategy::Minute, step: 15},
    TickDelta {delta: TimeDelta::minutes(30),   strategy: IntervalStrategy::Minute, step: 30},
    TickDelta {delta: TimeDelta::hours(1),      strategy: IntervalStrategy::Hour,   step: 1},
    TickDelta {delta: TimeDelta::hours(3),      strategy: IntervalStrategy::Hour,   step: 3},
    TickDelta {delta: TimeDelta::hours(6),      strategy: IntervalStrategy::Hour,   step: 6},
    TickDelta {delta: TimeDelta::hours(9),      strategy: IntervalStrategy::Hour,   step: 12},
    TickDelta {delta: TimeDelta::days(1),       strategy: IntervalStrategy::Day,    step: 1},
    TickDelta {delta: TimeDelta::days(2),       strategy: IntervalStrategy::Day,    step: 2},
    TickDelta {delta: TimeDelta::days(31),      strategy: IntervalStrategy::Month,  step: 1},
    TickDelta {delta: TimeDelta::days(31 * 3),  strategy: IntervalStrategy::Month,  step: 3},
    TickDelta {delta: TimeDelta::weeks(52),     strategy: IntervalStrategy::Year,   step: 1},
];

/// Scaler for domain defined with [`chrono::DateTime`] and a continuous range.
pub struct ScaleTime<Tz: TimeZone> {
    domain: [DateTime<Tz>; 2],
    range: [f64; 2],
    clamp: bool,
}

macro_rules! impl_default {
    ($timezone:ident) => {
        impl Default for ScaleTime<$timezone> {
            fn default() -> Self {
                Self {
                    domain: [
                        DateTime::default(),
                        DateTime::default() + TimeDelta::days(1),
                    ],
                    range: [0., 1.],
                    clamp: false,
                }
            }
        }
    };
}

impl_default!(Utc);
impl_default!(FixedOffset);
impl_default!(Local);

impl<Tz: TimeZone> ScaleTime<Tz> {
    /// Returns a new [`ScaleTime`] with the specified domain.
    pub fn domain(self, domain: [DateTime<Tz>; 2]) -> Self {
        Self { domain, ..self }
    }

    /// Returns a new [`ScaleTime`] with the specified range.
    pub fn range(self, range: [f64; 2]) -> Self {
        Self { range, ..self }
    }

    /// Returns a new [`ScaleTime`] with the specified clamp value. If `true`, it clamps the value
    /// passed to the transform step (see [`ScaleTime::apply`]) and the returned value after
    /// untransform step (see [`ScaleTime::invert`]) with the domain values.
    pub fn clamp(self, clamp: bool) -> Self {
        Self { clamp, ..self }
    }

    /// Given a specified [`chrono::DateTime`] value, it transforms it as timestamp value, it clamps
    /// the value, transforms it and returns the corresponding value of the range.
    pub fn apply(&self, x: DateTime<Tz>) -> f64 {
        let mut x = x.timestamp() as f64;
        let [a, b]: [f64; 2] = from_fn(|i| self.domain[i].timestamp() as f64);
        if self.clamp {
            x = x.clamp(a, b);
        }

        // Normalize value to [0, 1]
        let b = b - a;
        let t = if b.is_nan() {
            f64::NAN
        } else if b == 0.0 {
            0.5
        } else {
            (x - a) / b
        };

        // Interpolate to range values
        let [a, b] = self.range;
        a * (1. - t) + b * t
    }

    /// Given the specified value in the range, it computes the corresponding value of the domain,
    /// untransforms it, computes the clamped value and returns the corresponding
    /// [`chrono::DateTime`] value by using the final value as timestamp.
    pub fn invert(&self, y: f64) -> DateTime<Utc> {
        let [a, b] = self.range;

        // Normalize value to [0, 1]
        let b = b - a;
        let t = if b.is_nan() {
            f64::NAN
        } else if b == 0.0 {
            0.5
        } else {
            (y - a) / b
        };

        // Interpolate to range values
        let [a, b]: [f64; 2] = from_fn(|i| self.domain[i].timestamp() as f64);
        let mut x = a * (1. - t) + b * t;

        if self.clamp {
            x = x.clamp(a, b);
        }
        DateTime::from_timestamp_nanos(x as i64)
    }
}

// Tick spec with double precision
fn tick_spec(start: f64, stop: f64, count: usize) -> f64 {
    let step = (stop - start) / (1 as f64).max(count as f64);
    let power = step.log10().floor();
    let error = step / (10 as f64).powf(power);
    let factor = if error >= (50 as f64).sqrt() {
        10
    } else if error >= (10 as f64).sqrt() {
        5
    } else if error >= (2 as f64).sqrt() {
        2
    } else {
        1
    };

    let mut inc;
    let mut i1;
    let mut i2;
    if power < 0. {
        inc = (10 as f64).powf(-power) / factor as f64;
        i1 = (start * inc).round();
        i2 = (stop * inc).round();
        if i1 / inc < start {
            i1 += 1.;
        }
        if i2 / inc > stop {
            i2 -= 1.;
        }
        inc = -inc;
    } else {
        inc = (10 as f64).powf(power) * factor as f64;
        i1 = (start / inc).round();
        i2 = (stop / inc).round();
        if i1 * inc < start {
            i1 += 1.;
        }
        if i2 * inc > stop {
            i2 -= 1.;
        }
    }
    if i2 < i1 && count < 2 {
        return tick_spec(start, stop, count * 2);
    }
    inc
}

// Tick step with double precision
fn tick_step(start: f64, stop: f64, count: usize) -> f64 {
    if stop == start {
        return 1.;
    }
    let reverse = stop < start;
    let inc = if reverse {
        tick_spec(stop, start, count)
    } else {
        tick_spec(start, stop, count)
    };

    let sign = if reverse { -1. } else { 1. };
    let step = if inc < 0. { 1. / -inc } else { inc };
    sign * step
}

impl<Tz: TimeZone> Axis for ScaleTime<Tz> {
    type Tick = DateTime<Utc>;

    fn ticks(&self, count: Option<usize>) -> Vec<Self::Tick> {
        let count = count.unwrap_or(10);
        let [mut start, mut stop] = [self.domain[0].to_utc(), self.domain[1].to_utc()];
        let reverse = stop < start;
        if reverse {
            std::mem::swap(&mut start, &mut stop);
        }
        let duration = stop - start;
        let target = duration.abs() / count as i32;

        if target < TICK_DELTAS.first().unwrap().delta {
            let step = tick_step(start.timestamp() as f64, stop.timestamp() as f64, count) * 1e3;
            return IntervalStrategy::Millisecond.create_range(start, stop, step.max(1.) as u32);
        }

        if target > TICK_DELTAS.last().unwrap().delta {
            let step = tick_step(
                start.timestamp() as f64 / DURATION_YEAR,
                stop.timestamp() as f64 / DURATION_YEAR,
                count,
            ) * 1e3;
            let mut dates = IntervalStrategy::Year.create_range(start, stop, step as u32);
            if reverse {
                dates.reverse();
            }
            return dates;
        }

        let deltas: Vec<_> = TICK_DELTAS
            .iter()
            .map(|tick_delta| tick_delta.delta)
            .collect();
        let idx = match deltas.binary_search(&target) {
            Ok(i) => i,
            Err(i) => {
                let current = deltas[i].num_milliseconds() as f64;
                let previous = deltas[i - 1].num_milliseconds() as f64;
                let target = target.num_milliseconds() as f64;
                if (target / previous) < (current / target) {
                    i - 1
                } else {
                    i
                }
            }
        };
        let tick_delta = &TICK_DELTAS[idx];
        let mut dates = tick_delta
            .strategy
            .create_range(start, stop, tick_delta.step);
        if reverse {
            dates.reverse();
        }
        return dates;
    }

    fn tick_position(&self, x: Self::Tick) -> f32 {
        self.apply(x.with_timezone(&self.domain[0].timezone())) as f32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{DateTime, NaiveDate, Timelike, Utc};
    use rstest::rstest;

    fn datetime(
        year: i32,
        month: u32,
        day: u32,
        hour: u32,
        minute: u32,
        second: u32,
    ) -> DateTime<Utc> {
        NaiveDate::from_ymd_opt(year, month, day)
            .and_then(|date| date.and_hms_opt(hour, minute, second))
            .expect("invalid time values")
            .and_utc()
    }

    #[rstest]
    #[case(datetime(2011, 1, 1, 12, 0, 0), datetime(2011, 1, 1, 12, 0, 1), 4, &[
        datetime(2011, 1, 1, 12, 0, 0),
        datetime(2011, 1, 1, 12, 0, 0).with_nanosecond(200000000).unwrap(),
        datetime(2011, 1, 1, 12, 0, 0).with_nanosecond(400000000).unwrap(),
        datetime(2011, 1, 1, 12, 0, 0).with_nanosecond(600000000).unwrap(),
        datetime(2011, 1, 1, 12, 0, 0).with_nanosecond(800000000).unwrap(),
        datetime(2011, 1, 1, 12, 0, 1),
    ])]
    #[case(datetime(2011, 1, 1, 12, 0, 0), datetime(2011, 1, 1, 12, 0, 4), 4, &[
        datetime(2011, 1, 1, 12, 0, 0),
        datetime(2011, 1, 1, 12, 0, 1),
        datetime(2011, 1, 1, 12, 0, 2),
        datetime(2011, 1, 1, 12, 0, 3),
        datetime(2011, 1, 1, 12, 0, 4),
    ])]
    #[case(datetime(2011, 1, 1, 12, 0, 0), datetime(2011, 1, 1, 12, 0, 20), 4, &[
        datetime(2011, 1, 1, 12, 0, 0),
        datetime(2011, 1, 1, 12, 0, 5),
        datetime(2011, 1, 1, 12, 0, 10),
        datetime(2011, 1, 1, 12, 0, 15),
        datetime(2011, 1, 1, 12, 0, 20),
    ])]
    #[case(datetime(2011, 1, 1, 12, 0, 0), datetime(2011, 1, 1, 12, 0, 50), 4, &[
        datetime(2011, 1, 1, 12, 0, 0),
        datetime(2011, 1, 1, 12, 0, 15),
        datetime(2011, 1, 1, 12, 0, 30),
        datetime(2011, 1, 1, 12, 0, 45),
    ])]
    #[case(datetime(2011, 1, 1, 12, 0, 0), datetime(2011, 1, 1, 12, 1, 50), 4, &[
        datetime(2011, 1, 1, 12, 0, 0),
        datetime(2011, 1, 1, 12, 0, 30),
        datetime(2011, 1, 1, 12, 1, 0),
        datetime(2011, 1, 1, 12, 1, 30),
    ])]
    #[case(datetime(2011, 1, 1, 12, 0, 27), datetime(2011, 1, 1, 12, 4, 12), 4, &[
        datetime(2011, 1, 1, 12, 1, 0),
        datetime(2011, 1, 1, 12, 2, 0),
        datetime(2011, 1, 1, 12, 3, 0),
        datetime(2011, 1, 1, 12, 4, 0),
    ])]
    #[case(datetime(2011, 1, 1, 12, 3, 27), datetime(2011, 1, 1, 12, 21, 12), 4, &[
        datetime(2011, 1, 1, 12, 5, 0),
        datetime(2011, 1, 1, 12, 10, 0),
        datetime(2011, 1, 1, 12, 15, 0),
        datetime(2011, 1, 1, 12, 20, 0),
    ])]
    #[case(datetime(2011, 1, 1, 12, 8, 27), datetime(2011, 1, 1, 13, 4, 12), 4, &[
        datetime(2011, 1, 1, 12, 15, 0),
        datetime(2011, 1, 1, 12, 30, 0),
        datetime(2011, 1, 1, 12, 45, 0),
        datetime(2011, 1, 1, 13, 0, 0),
    ])]
    #[case(datetime(2011, 1, 1, 12, 28, 27), datetime(2011, 1, 1, 14, 4, 12), 4, &[
        datetime(2011, 1, 1, 12, 30, 0),
        datetime(2011, 1, 1, 13, 0, 0),
        datetime(2011, 1, 1, 13, 30, 0),
        datetime(2011, 1, 1, 14, 0, 0),
    ])]
    #[case(datetime(2011, 1, 1, 12, 28, 27), datetime(2011, 1, 1, 16, 34, 12), 4, &[
        datetime(2011, 1, 1, 13, 0, 0),
        datetime(2011, 1, 1, 14, 0, 0),
        datetime(2011, 1, 1, 15, 0, 0),
        datetime(2011, 1, 1, 16, 0, 0),
    ])]
    #[case(datetime(2011, 1, 1, 14, 28, 27), datetime(2011, 1, 2, 1, 34, 12), 4, &[
        datetime(2011, 1, 1, 15, 0, 0),
        datetime(2011, 1, 1, 18, 0, 0),
        datetime(2011, 1, 1, 21, 0, 0),
        datetime(2011, 1, 2, 0, 0, 0),
    ])]
    #[case(datetime(2011, 1, 1, 16, 28, 27), datetime(2011, 1, 2, 14, 34, 12), 4, &[
        datetime(2011, 1, 1, 18, 0, 0),
        datetime(2011, 1, 2, 0, 0, 0),
        datetime(2011, 1, 2, 6, 0, 0),
        datetime(2011, 1, 2, 12, 0, 0),
    ])]
    #[case(datetime(2011, 1, 1, 16, 28, 27), datetime(2011, 1, 3, 21, 34, 12), 4, &[
        datetime(2011, 1, 2, 0, 0, 0),
        datetime(2011, 1, 2, 12, 0, 0),
        datetime(2011, 1, 3, 0, 0, 0),
        datetime(2011, 1, 3, 12, 0, 0),
    ])]
    #[case(datetime(2011, 1, 1, 16, 28, 27), datetime(2011, 1, 5, 21, 34, 12), 4, &[
        datetime(2011, 1, 2, 0, 0, 0),
        datetime(2011, 1, 3, 0, 0, 0),
        datetime(2011, 1, 4, 0, 0, 0),
        datetime(2011, 1, 5, 0, 0, 0),
    ])]
    #[case(datetime(2011, 1, 2, 16, 28, 27), datetime(2011, 1, 9, 21, 34, 12), 4, &[
        datetime(2011, 1, 3, 0, 0, 0),
        datetime(2011, 1, 5, 0, 0, 0),
        datetime(2011, 1, 7, 0, 0, 0),
        datetime(2011, 1, 9, 0, 0, 0),
    ])]
    // // Week case
    // #[case(datetime(2011, 1, 1, 16, 28, 27), datetime(2011, 1, 23, 21, 34, 12), 4, &[
    //     datetime(2011, 1, 2, 0, 0, 0),
    //     datetime(2011, 1, 9, 0, 0, 0),
    //     datetime(2011, 1, 16, 0, 0, 0),
    //     datetime(2011, 1, 23, 0, 0, 0),
    // ])]
    #[case(datetime(2011, 1, 18, 0, 0, 0), datetime(2011, 5, 2, 0, 0, 0), 4, &[
        datetime(2011, 2, 1, 0, 0, 0),
        datetime(2011, 3, 1, 0, 0, 0),
        datetime(2011, 4, 1, 0, 0, 0),
        datetime(2011, 5, 1, 0, 0, 0),
    ])]
    #[case(datetime(2010, 12, 18, 0, 0, 0), datetime(2011, 11, 2, 0, 0, 0), 4, &[
        datetime(2011, 1, 1, 0, 0, 0),
        datetime(2011, 4, 1, 0, 0, 0),
        datetime(2011, 7, 1, 0, 0, 0),
        datetime(2011, 10, 1, 0, 0, 0),
    ])]
    #[case(datetime(2010, 12, 18, 0, 0, 0), datetime(2014, 3, 2, 0, 0, 0), 4, &[
        datetime(2011, 1, 1, 0, 0, 0),
        datetime(2012, 1, 1, 0, 0, 0),
        datetime(2013, 1, 1, 0, 0, 0),
        datetime(2014, 1, 1, 0, 0, 0),
    ])]
    #[case(datetime(2014, 3, 2, 0, 0, 0), datetime(2014, 3, 2, 0, 0, 0), 6, &[datetime(2014, 3, 2, 0, 0, 0)])]
    #[case(datetime(2014, 3, 2, 0, 0, 0), datetime(2010, 12, 18, 0, 0, 0), 4, &[
        datetime(2014, 1, 1, 0, 0, 0),
        datetime(2013, 1, 1, 0, 0, 0),
        datetime(2012, 1, 1, 0, 0, 0),
        datetime(2011, 1, 1, 0, 0, 0),
    ])]
    #[case(datetime(2011, 11, 2, 0, 0, 0), datetime(2010, 12, 18, 0, 0, 0), 4, &[
        datetime(2011, 10, 1, 0, 0, 0),
        datetime(2011, 7, 1, 0, 0, 0),
        datetime(2011, 4, 1, 0, 0, 0),
        datetime(2011, 1, 1, 0, 0, 0),
    ])]
    fn test_scale_time(
        #[case] domain_left: DateTime<Utc>,
        #[case] domain_right: DateTime<Utc>,
        #[case] count: usize,
        #[case] expect: &[DateTime<Utc>],
    ) {
        assert_eq!(
            ScaleTime::default()
                .domain([domain_left, domain_right])
                .ticks(Some(count)),
            expect
        );
    }
}
