use super::{
    day::Day, hour::Hour, millisecond::Millisecond, minute::Minute, month::Month, second::Second,
    year::Year,
};
use chrono::{DateTime, Local, TimeDelta, Utc};

/// Trait for operations on specific time interval boundary
pub trait Timing {
    /// Returns a new date representing the latest interval boundary date before or equal to date.
    fn floor(&self, date: DateTime<Utc>) -> Option<DateTime<Utc>>;
    /// Returns a new date equal to date plus `step` intervals.
    fn offset(&self, date: DateTime<Utc>, step: i64) -> DateTime<Utc>;
    /// Returns the number of interval boundaries after start (exclusive) and before or equal to end
    /// (inclusive).
    fn count(&self, start: DateTime<Utc>, end: DateTime<Utc>) -> i64;
    /// Returns the unit of the date based on the interval boundary
    fn field(&self, date: DateTime<Utc>) -> u32;
}

/// Every interval
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
                while !self.time_type.field(date).is_multiple_of(self.step) {
                    date = self.time_type.offset(date, -1);
                }
                step += 1;
            }
        } else {
            step -= 1;
            while step >= 0 {
                date = self.time_type.offset(date, 1);
                while !self.time_type.field(date).is_multiple_of(self.step) {
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

/// Time interval for manipulating [`chrono::DateTime`] such as floor, ceil, round, range, every
/// operations.
pub struct TimeInterval<T: Timing> {
    time_type: T,
}

impl<T: Timing> TimeInterval<T> {
    /// Returns a new date representing the latest interval boundary date before or equal to date.
    /// Default: current time.
    ///
    /// ```
    /// use chrono::NaiveDate;
    /// use vizkit::time::TimeInterval;
    ///
    /// // Returns `DateTime<Utc>`
    /// let datetime = |year, month, day| {
    ///     NaiveDate::from_ymd_opt(year, month, day)
    ///         .and_then(|date| date.and_hms_opt(0, 0, 0))
    ///         .expect("invalid time values")
    ///         .and_utc()
    /// };
    ///
    /// assert_eq!(
    ///     TimeInterval::month().interval(Some(datetime(2015, 05, 15))),
    ///     Some(datetime(2015, 05, 01))
    /// )
    /// ```
    pub fn interval(&self, date: Option<DateTime<Utc>>) -> Option<DateTime<Utc>> {
        self.time_type.floor(date.unwrap_or(Local::now().to_utc()))
    }

    /// Returns a new date representing the earliest interval boundary date before or equal to date.
    ///
    /// ```
    /// use chrono::NaiveDate;
    /// use vizkit::time::TimeInterval;
    ///
    /// // Returns `DateTime<Utc>`
    /// let datetime = |year, month, day| {
    ///     NaiveDate::from_ymd_opt(year, month, day)
    ///         .and_then(|date| date.and_hms_opt(0, 0, 0))
    ///         .expect("invalid time values")
    ///         .and_utc()
    /// };
    ///
    /// assert_eq!(
    ///     TimeInterval::month().ceil(datetime(2015, 05, 15)),
    ///     Some(datetime(2015, 06, 01))
    /// )
    /// ```
    pub fn ceil(&self, date: DateTime<Utc>) -> Option<DateTime<Utc>> {
        self.time_type
            .floor(date + TimeDelta::nanoseconds(-1))
            .map(|d| self.time_type.offset(d, 1))
            .and_then(|d| self.time_type.floor(d))
    }

    /// Returns a new date representing the closest interval boundary date before or equal to date.
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

    /// Returns a collection of dates representing every interval boundary after or equal to start
    /// (inclusive) and before stop (exclusive).
    ///
    /// ```
    /// use chrono::NaiveDate;
    /// use vizkit::time::TimeInterval;
    ///
    /// // Returns `DateTime<Utc>`
    /// let datetime = |year, month, day| {
    ///     NaiveDate::from_ymd_opt(year, month, day)
    ///         .and_then(|date| date.and_hms_opt(0, 0, 0))
    ///         .expect("invalid time values")
    ///         .and_utc()
    /// };
    ///
    /// assert_eq!(
    ///     TimeInterval::month()
    ///         .range(datetime(2015, 05, 15), datetime(2015, 10, 20), 1),
    ///     vec![
    ///         datetime(2015, 6, 1),
    ///         datetime(2015, 7, 1),
    ///         datetime(2015, 8, 1),
    ///         datetime(2015, 9, 1),
    ///         datetime(2015, 10, 1),
    ///     ]
    /// )
    /// ```
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

    /// Returns a filtered view of this interval representing every step-th date.
    ///
    /// ```
    /// use chrono::NaiveDate;
    /// use vizkit::time::TimeInterval;
    ///
    /// // Returns `DateTime<Utc>`
    /// let datetime = |year, month, day| {
    ///     NaiveDate::from_ymd_opt(year, month, day)
    ///         .and_then(|date| date.and_hms_opt(0, 0, 0))
    ///         .expect("invalid time values")
    ///         .and_utc()
    /// };
    ///
    /// assert_eq!(
    ///     TimeInterval::month()
    ///         .every(3)
    ///         .range(datetime(2008, 12, 3), datetime(2010, 7, 5), 1),
    ///     vec![
    ///         datetime(2009, 1, 1),
    ///         datetime(2009, 4, 1),
    ///         datetime(2009, 7, 1),
    ///         datetime(2009, 10, 1),
    ///         datetime(2010, 1, 1),
    ///         datetime(2010, 4, 1),
    ///         datetime(2010, 7, 1),
    ///     ]
    /// )
    /// ```
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
    /// Day interval
    pub fn day() -> Self {
        Self { time_type: Day }
    }
}

impl TimeInterval<Month> {
    /// Month interval
    pub fn month() -> Self {
        Self { time_type: Month }
    }
}

impl TimeInterval<Year> {
    /// Year interval
    pub fn year() -> Self {
        Self { time_type: Year }
    }
}

impl TimeInterval<Hour> {
    /// Hour interval
    pub fn hour() -> Self {
        Self { time_type: Hour }
    }
}

impl TimeInterval<Minute> {
    /// Minute interval
    pub fn minute() -> Self {
        Self { time_type: Minute }
    }
}

impl TimeInterval<Second> {
    /// Second interval
    pub fn second() -> Self {
        Self { time_type: Second }
    }
}

impl TimeInterval<Millisecond> {
    /// Millisecond interval
    pub fn millisecond() -> Self {
        Self {
            time_type: Millisecond,
        }
    }
}
