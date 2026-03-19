use crate::interval::TimeType;
use chrono::{DateTime, Datelike, TimeDelta, Timelike, Utc};

pub struct Day;
impl TimeType for Day {
    fn floor(&self, date: DateTime<Utc>) -> Option<DateTime<Utc>> {
        date.with_hour(0)
            .and_then(|date| date.with_minute(0))
            .and_then(|date| date.with_second(0))
            .and_then(|date| date.with_nanosecond(0))
    }

    fn offset(&self, date: DateTime<Utc>, step: i64) -> DateTime<Utc> {
        date + TimeDelta::days(step)
    }

    fn count(&self, start: DateTime<Utc>, end: DateTime<Utc>) -> i64 {
        (end - start).num_days()
    }

    fn field(&self, date: DateTime<Utc>) -> u32 {
        date.day() - 1
    }
}
