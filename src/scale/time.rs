use std::array::from_fn;

use super::ticks::ticks_f64;

use chrono::{DateTime, FixedOffset, Local, TimeDelta, TimeZone, Utc};

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
    pub fn domain(self, domain: [DateTime<Tz>; 2]) -> Self {
        Self { domain, ..self }
    }

    pub fn range(self, range: [f64; 2]) -> Self {
        Self { range, ..self }
    }

    pub fn clamp(self, clamp: bool) -> Self {
        Self { clamp, ..self }
    }

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

    pub fn inverse(&self, y: f64) -> DateTime<Utc> {
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

    pub fn ticks(&self, count: Option<usize>) -> Vec<DateTime<Utc>> {
        let [start, stop]: [f64; 2] = from_fn(|i| self.domain[i].timestamp() as f64);
        ticks_f64(start, stop, count.unwrap_or(10))
            .into_iter()
            .map(|tick| DateTime::from_timestamp_nanos(tick as i64))
            .collect()
    }
}
