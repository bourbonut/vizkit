use crate::interval::TimeType;
use chrono::{DateTime, TimeDelta, Timelike, Utc};

pub struct Second;
impl TimeType for Second {
    fn floor(&self, date: DateTime<Utc>) -> Option<DateTime<Utc>> {
        date.with_nanosecond(0)
    }

    fn offset(&self, date: DateTime<Utc>, step: i64) -> DateTime<Utc> {
        date + TimeDelta::seconds(step)
    }

    fn count(&self, start: DateTime<Utc>, end: DateTime<Utc>) -> i64 {
        (end - start).num_seconds()
    }

    fn field(&self, date: DateTime<Utc>) -> u32 {
        date.second()
    }
}
