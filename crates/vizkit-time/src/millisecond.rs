use crate::interval::Timing;
use chrono::{DateTime, TimeDelta, Timelike, Utc};

pub struct Millisecond;
impl Timing for Millisecond {
    fn floor(&self, date: DateTime<Utc>) -> Option<DateTime<Utc>> {
        date.with_nanosecond((date.nanosecond() / 1_000_000) * 1_000_000)
    }

    fn offset(&self, date: DateTime<Utc>, step: i64) -> DateTime<Utc> {
        date + TimeDelta::milliseconds(step)
    }

    fn count(&self, start: DateTime<Utc>, end: DateTime<Utc>) -> i64 {
        (end - start).num_milliseconds()
    }

    fn field(&self, date: DateTime<Utc>) -> u32 {
        date.nanosecond() / 1_000_000
    }
}
