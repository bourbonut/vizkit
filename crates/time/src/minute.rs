use crate::interval::Timing;
use chrono::{DateTime, TimeDelta, Timelike, Utc};

pub struct Minute;
impl Timing for Minute {
    fn floor(&self, date: DateTime<Utc>) -> Option<DateTime<Utc>> {
        date.with_second(0).and_then(|date| date.with_nanosecond(0))
    }

    fn offset(&self, date: DateTime<Utc>, step: i64) -> DateTime<Utc> {
        date + TimeDelta::minutes(step)
    }

    fn count(&self, start: DateTime<Utc>, end: DateTime<Utc>) -> i64 {
        (end - start).num_minutes()
    }

    fn field(&self, date: DateTime<Utc>) -> u32 {
        date.minute()
    }
}
