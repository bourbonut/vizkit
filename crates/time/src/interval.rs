use crate::{
    day::Day, hour::Hour, millisecond::Millisecond, minute::Minute, month::Month, second::Second,
    year::Year,
};
use chrono::{DateTime, Local, TimeDelta, Utc};

pub trait Timing {
    fn floor(&self, date: DateTime<Utc>) -> Option<DateTime<Utc>>;
    fn offset(&self, date: DateTime<Utc>, step: i64) -> DateTime<Utc>;
    fn count(&self, start: DateTime<Utc>, end: DateTime<Utc>) -> i64;
    fn field(&self, date: DateTime<Utc>) -> u32;
}

pub struct Every<T: Timing> {
    time_type: T,
    step: u32,
}

impl<T: Timing> Timing for Every<T> {
    fn floor(&self, date: DateTime<Utc>) -> Option<DateTime<Utc>> {
        let mut date = self.time_type.floor(date)?;
        while self.time_type.field(date) % self.step != 0 {
            date = self.time_type.floor(date - TimeDelta::nanoseconds(1))?;
        }
        Some(date)
    }

    fn offset(&self, date: DateTime<Utc>, mut step: i64) -> DateTime<Utc> {
        let mut date = date;
        if step < 0 {
            step += 1;
            while step <= 0 {
                date = self.time_type.offset(date, -1);
                while self.time_type.field(date) % self.step != 0 {
                    date = self.time_type.offset(date, -1);
                }
                step += 1;
            }
        } else {
            step -= 1;
            while step >= 0 {
                date = self.time_type.offset(date, 1);
                while self.time_type.field(date) % self.step != 0 {
                    date = self.time_type.offset(date, 1);
                }
                step -= 1;
            }
        }
        date
    }

    fn count(&self, start: DateTime<Utc>, end: DateTime<Utc>) -> i64 {
        self.time_type.count(start, end)
    }

    fn field(&self, date: DateTime<Utc>) -> u32 {
        self.time_type.field(date)
    }
}

pub struct TimeInterval<T: Timing> {
    time_type: T,
}

impl<T: Timing> TimeInterval<T> {
    pub fn interval(&self, date: Option<DateTime<Utc>>) -> Option<DateTime<Utc>> {
        self.time_type.floor(date.unwrap_or(Local::now().to_utc()))
    }

    pub fn ceil(&self, date: DateTime<Utc>) -> Option<DateTime<Utc>> {
        self.time_type
            .floor(date + TimeDelta::nanoseconds(-1))
            .map(|d| self.time_type.offset(d, 1))
            .and_then(|d| self.time_type.floor(d))
    }

    pub fn round(&self, date: DateTime<Utc>) -> Option<DateTime<Utc>> {
        let d0 = self.interval(Some(date));
        let d1 = self.ceil(date);
        match (d0, d1) {
            (Some(d0), Some(d1)) => {
                if date - d0 < d1 - date {
                    Some(d0)
                } else {
                    Some(d1)
                }
            }
            _ => None,
        }
    }

    pub fn range(
        &self,
        start: DateTime<Utc>,
        stop: DateTime<Utc>,
        step: i64,
    ) -> Vec<DateTime<Utc>> {
        let mut current = match self.ceil(start) {
            None => return Vec::new(),
            Some(start_date) if start_date >= stop => return Vec::new(),
            Some(start_date) => start_date,
        };

        let mut range: Vec<DateTime<Utc>> = Vec::new();
        loop {
            range.push(current);
            if let Some(next) = self
                .time_type
                .floor(self.time_type.offset(current, step))
                .filter(|next| current < *next && *next < stop)
            {
                current = next;
            } else {
                break;
            }
        }
        range
    }

    pub fn every(self, step: u32) -> TimeInterval<Every<T>> {
        TimeInterval::<Every<T>> {
            time_type: Every {
                time_type: self.time_type,
                step,
            },
        }
    }
}

impl TimeInterval<Day> {
    pub fn day() -> Self {
        Self { time_type: Day }
    }
}

impl TimeInterval<Month> {
    pub fn month() -> Self {
        Self { time_type: Month }
    }
}

impl TimeInterval<Year> {
    pub fn year() -> Self {
        Self { time_type: Year }
    }
}

impl TimeInterval<Hour> {
    pub fn year() -> Self {
        Self { time_type: Hour }
    }
}

impl TimeInterval<Minute> {
    pub fn year() -> Self {
        Self { time_type: Minute }
    }
}

impl TimeInterval<Second> {
    pub fn year() -> Self {
        Self { time_type: Second }
    }
}

impl TimeInterval<Millisecond> {
    pub fn year() -> Self {
        Self {
            time_type: Millisecond,
        }
    }
}
