use crate::interval::TimeType;
use chrono::{DateTime, Datelike, TimeDelta, Timelike, Utc};

pub struct Year;
impl TimeType for Year {
    fn floor(&self, date: DateTime<Utc>) -> Option<DateTime<Utc>> {
        date.with_month(1)
            .and_then(|date| date.with_day(1))
            .and_then(|date| date.with_hour(0))
            .and_then(|date| date.with_minute(0))
            .and_then(|date| date.with_second(0))
            .and_then(|date| date.with_nanosecond(0))
    }

    fn offset(&self, date: DateTime<Utc>, step: i64) -> DateTime<Utc> {
        let sign = step.signum();
        let isleap = date.date_naive().leap_year() as i64;
        date + TimeDelta::days(step * 365 + sign * isleap)
    }

    fn count(&self, start: DateTime<Utc>, end: DateTime<Utc>) -> i64 {
        (end - start).num_days() / 365
    }

    fn field(&self, date: DateTime<Utc>) -> u32 {
        // TODO: maybe update the TimeType trait in order to accept negative year
        date.year() as u32
    }
}
