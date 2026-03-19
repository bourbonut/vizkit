use crate::interval::TimeType;
use chrono::{DateTime, Datelike, TimeDelta, Timelike, Utc};

pub struct Month;
impl TimeType for Month {
    fn floor(&self, date: DateTime<Utc>) -> Option<DateTime<Utc>> {
        date.with_day(1)
            .and_then(|date| date.with_hour(0))
            .and_then(|date| date.with_minute(0))
            .and_then(|date| date.with_second(0))
            .and_then(|date| date.with_nanosecond(0))
    }

    fn offset(&self, date: DateTime<Utc>, step: i64) -> DateTime<Utc> {
        date + TimeDelta::days(step * 31)
    }

    fn count(&self, start: DateTime<Utc>, end: DateTime<Utc>) -> i64 {
        let days = (end - start).num_days();
        let remaider = days % 30;
        days / 30 + (0 < remaider && remaider <= 15) as i64
    }

    fn field(&self, date: DateTime<Utc>) -> u32 {
        date.month() - 1
    }
}
