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

/// Time interval for manipulating [`chrono::DateTime`] such as _floor_, _ceil_, _round_, _range_,
/// _every_ operations.
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
    ///
    /// ```
    /// use chrono::NaiveDate;
    /// use vizkit::time::TimeInterval;
    ///
    /// // Returns `DateTime<Utc>`
    /// let datetime = |year, month, day, hour| {
    ///     NaiveDate::from_ymd_opt(year, month, day)
    ///         .and_then(|date| date.and_hms_opt(hour, 0, 0))
    ///         .expect("invalid time values")
    ///         .and_utc()
    /// };
    ///
    /// assert_eq!(TimeInterval::day().round(datetime(2010, 12, 30, 13)), Some(datetime(2010, 12, 31, 0)));
    /// ```
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
        step: u64,
    ) -> Vec<DateTime<Utc>> {
        if step == 0 {
            return Vec::new();
        }
        let mut current = match self.ceil(start) {
            None => return Vec::new(),
            Some(start_date) if start_date >= stop => return Vec::new(),
            Some(start_date) => start_date,
        };

        let step = step as i64;
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

    /// Counts the number of interval boundaries after `start` (exclusive) and before or equal `end`
    /// (inclusive).
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
    /// assert_eq!(TimeInterval::day().count(datetime(2011, 1, 1), datetime(2011, 5, 9)), 128);
    /// ```
    pub fn count(&self, start: DateTime<Utc>, end: DateTime<Utc>) -> i64 {
        self.time_type.count(start, end)
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

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{DateTime, NaiveDate, Utc};
    use rstest::rstest;

    fn datetime(year: i32, month: u32, day: u32, hour: u32) -> DateTime<Utc> {
        NaiveDate::from_ymd_opt(year, month, day)
            .and_then(|date| date.and_hms_opt(hour, 0, 0))
            .expect("invalid time values")
            .and_utc()
    }

    #[rstest]
    #[case(datetime(2010, 12, 31, 23), datetime(2010, 12, 31, 0))]
    #[case(datetime(2011, 1, 1, 0), datetime(2011, 1, 1, 0))]
    #[case(datetime(2011, 1, 1, 1), datetime(2011, 1, 1, 0))]
    #[case(datetime(2011, 3, 13, 7), datetime(2011, 3, 13, 0))]
    #[case(datetime(2011, 3, 13, 8), datetime(2011, 3, 13, 0))]
    #[case(datetime(2011, 3, 13, 9), datetime(2011, 3, 13, 0))]
    #[case(datetime(2011, 3, 13, 10), datetime(2011, 3, 13, 0))]
    #[case(datetime(2011, 11, 6, 7), datetime(2011, 11, 6, 0))]
    #[case(datetime(2011, 11, 6, 8), datetime(2011, 11, 6, 0))]
    #[case(datetime(2011, 11, 6, 9), datetime(2011, 11, 6, 0))]
    #[case(datetime(2011, 11, 6, 10), datetime(2011, 11, 6, 0))]
    #[case(datetime(9, 11, 6, 7), datetime(9, 11, 6, 0))]
    fn test_day_interval(#[case] input: DateTime<Utc>, #[case] expect: DateTime<Utc>) {
        assert_eq!(TimeInterval::day().interval(Some(input)), Some(expect));
    }

    #[rstest]
    #[case(datetime(2010, 12, 30, 13), datetime(2010, 12, 31, 0))]
    #[case(datetime(2010, 12, 30, 11), datetime(2010, 12, 30, 0))]
    #[case(datetime(2011, 3, 13, 7), datetime(2011, 3, 13, 0))]
    #[case(datetime(2011, 3, 13, 8), datetime(2011, 3, 13, 0))]
    #[case(datetime(2011, 3, 13, 9), datetime(2011, 3, 13, 0))]
    #[case(datetime(2011, 3, 13, 20), datetime(2011, 3, 14, 0))]
    #[case(datetime(2011, 11, 6, 7), datetime(2011, 11, 6, 0))]
    #[case(datetime(2011, 11, 6, 8), datetime(2011, 11, 6, 0))]
    #[case(datetime(2011, 11, 6, 9), datetime(2011, 11, 6, 0))]
    #[case(datetime(2011, 11, 6, 20), datetime(2011, 11, 7, 0))]
    #[case(datetime(2012, 3, 1, 0), datetime(2012, 3, 1, 0))]
    #[case(datetime(2012, 3, 1, 0), datetime(2012, 3, 1, 0))]
    fn test_day_round(#[case] input: DateTime<Utc>, #[case] expect: DateTime<Utc>) {
        assert_eq!(TimeInterval::day().round(input), Some(expect));
    }

    #[rstest]
    #[case(datetime(2010, 12, 30, 23), datetime(2010, 12, 31, 0))]
    #[case(datetime(2010, 12, 31, 0), datetime(2010, 12, 31, 0))]
    #[case(datetime(2010, 12, 31, 1), datetime(2011, 1, 1, 0))]
    #[case(datetime(2011, 3, 13, 7), datetime(2011, 3, 14, 0))]
    #[case(datetime(2011, 3, 13, 8), datetime(2011, 3, 14, 0))]
    #[case(datetime(2011, 3, 13, 9), datetime(2011, 3, 14, 0))]
    #[case(datetime(2011, 3, 13, 10), datetime(2011, 3, 14, 0))]
    #[case(datetime(2011, 11, 6, 7), datetime(2011, 11, 7, 0))]
    #[case(datetime(2011, 11, 6, 8), datetime(2011, 11, 7, 0))]
    #[case(datetime(2011, 11, 6, 9), datetime(2011, 11, 7, 0))]
    #[case(datetime(2011, 11, 6, 10), datetime(2011, 11, 7, 0))]
    #[case(datetime(2012, 3, 1, 0), datetime(2012, 3, 1, 0))]
    #[case(datetime(2012, 3, 1, 0), datetime(2012, 3, 1, 0))]
    fn test_day_ceil(#[case] input: DateTime<Utc>, #[case] expect: DateTime<Utc>) {
        assert_eq!(TimeInterval::day().ceil(input), Some(expect));
    }

    #[rstest]
    #[case(
        datetime(2011, 11, 4, 0),
        datetime(2011, 11, 10, 0),
        1,
        &[datetime(2011, 11, 4, 0), datetime(2011, 11, 5, 0), datetime(2011, 11, 6, 0), datetime(2011, 11, 7, 0), datetime(2011, 11, 8, 0), datetime(2011, 11, 9, 0)]
    )]
    #[case(
        datetime(2011, 11, 4, 2),
        datetime(2011, 11, 10, 13),
        1,
        &[datetime(2011, 11, 5, 0), datetime(2011, 11, 6, 0), datetime(2011, 11, 7, 0), datetime(2011, 11, 8, 0), datetime(2011, 11, 9, 0), datetime(2011, 11, 10, 0)]
    )]
    #[case(
        datetime(2011, 11, 4, 0),
        datetime(2011, 11, 7, 0),
        1,
        &[datetime(2011, 11, 4, 0), datetime(2011, 11, 5, 0), datetime(2011, 11, 6, 0)]
    )]
    #[case(
        datetime(2011, 11, 10, 0),
        datetime(2011, 11, 4, 0),
        1,
        &[]
    )]
    #[case(
        datetime(2011, 11, 10, 0),
        datetime(2011, 11, 10, 0),
        1,
        &[]
    )]
    #[case(
        datetime(2011, 11, 4, 2),
        datetime(2011, 11, 14, 13),
        3,
        &[datetime(2011, 11, 5, 0), datetime(2011, 11, 8, 0), datetime(2011, 11, 11, 0), datetime(2011, 11, 14, 0)]
    )]
    #[case(
        datetime(2011, 1, 1, 0),
        datetime(2011, 5, 9, 0),
        0,
        &[]
    )]
    fn test_day_range(
        #[case] start: DateTime<Utc>,
        #[case] stop: DateTime<Utc>,
        #[case] step: u64,
        #[case] expect: &[DateTime<Utc>],
    ) {
        assert_eq!(TimeInterval::day().range(start, stop, step), expect);
    }

    #[rstest]
    #[case(datetime(2011, 1, 1, 0), datetime(2011, 5, 9, 0), 128)]
    #[case(datetime(2011, 1, 1, 1), datetime(2011, 5, 9, 0), 127)]
    #[case(datetime(2010, 12, 31, 23), datetime(2011, 5, 9, 0), 128)]
    #[case(datetime(2011, 1, 1, 0), datetime(2011, 5, 8, 23), 127)]
    #[case(datetime(2011, 1, 1, 0), datetime(2011, 5, 9, 1), 128)]
    #[case(datetime(2011, 1, 1, 0), datetime(2011, 3, 13, 1), 71)]
    #[case(datetime(2011, 1, 1, 0), datetime(2011, 3, 13, 3), 71)]
    #[case(datetime(2011, 1, 1, 0), datetime(2011, 3, 13, 4), 71)]
    #[case(datetime(2011, 1, 1, 0), datetime(2011, 11, 6, 0), 309)]
    #[case(datetime(2011, 1, 1, 0), datetime(2011, 11, 6, 1), 309)]
    #[case(datetime(2011, 1, 1, 0), datetime(2011, 11, 6, 2), 309)]
    #[case(datetime(1999, 1, 1, 0), datetime(1999, 12, 31, 0), 364)]
    #[case(datetime(2000, 1, 1, 0), datetime(2000, 12, 31, 0), 365)]
    #[case(datetime(2001, 1, 1, 0), datetime(2001, 12, 31, 0), 364)]
    #[case(datetime(2002, 1, 1, 0), datetime(2002, 12, 31, 0), 364)]
    #[case(datetime(2003, 1, 1, 0), datetime(2003, 12, 31, 0), 364)]
    #[case(datetime(2004, 1, 1, 0), datetime(2004, 12, 31, 0), 365)]
    #[case(datetime(2005, 1, 1, 0), datetime(2005, 12, 31, 0), 364)]
    #[case(datetime(2006, 1, 1, 0), datetime(2006, 12, 31, 0), 364)]
    #[case(datetime(2007, 1, 1, 0), datetime(2007, 12, 31, 0), 364)]
    #[case(datetime(2008, 1, 1, 0), datetime(2008, 12, 31, 0), 365)]
    #[case(datetime(2009, 1, 1, 0), datetime(2009, 12, 31, 0), 364)]
    #[case(datetime(2010, 1, 1, 0), datetime(2010, 12, 31, 0), 364)]
    #[case(datetime(2011, 1, 1, 0), datetime(2011, 12, 31, 0), 364)]
    fn test_day_count(
        #[case] start: DateTime<Utc>,
        #[case] end: DateTime<Utc>,
        #[case] expect: i64,
    ) {
        assert_eq!(TimeInterval::day().count(start, end), expect);
    }

    fn datetime2(year: i32, month: u32, day: u32, hour: u32, minute: u32) -> DateTime<Utc> {
        NaiveDate::from_ymd_opt(year, month, day)
            .and_then(|date| date.and_hms_opt(hour, minute, 0))
            .expect("invalid time values")
            .and_utc()
    }

    #[rstest]
    #[case(
        3,
        datetime2(2008, 12, 30, 0, 12),
        datetime2(2009, 1, 5, 23, 48),
        &[datetime2(2008, 12, 31, 0, 0), datetime2(2009, 1, 1, 0, 0), datetime2(2009, 1, 4, 0, 0)]
    )]
    #[case(
        5,
        datetime2(2008, 12, 30, 0, 12),
        datetime2(2009, 1, 6, 23, 48),
        &[datetime2(2008, 12, 31, 0, 0), datetime2(2009, 1, 1, 0, 0), datetime2(2009, 1, 6, 0, 0)]
    )]
    #[case(
        7,
        datetime2(2008, 12, 30, 0, 12),
        datetime2(2009, 1, 8, 23, 48),
        &[datetime2(2009, 1, 1, 0, 0), datetime2(2009, 1, 8, 0, 0)]
    )]
    fn test_day_every(
        #[case] every: u32,
        #[case] start: DateTime<Utc>,
        #[case] end: DateTime<Utc>,
        #[case] expect: &[DateTime<Utc>],
    ) {
        assert_eq!(
            TimeInterval::day().every(every).range(start, end, 1),
            expect
        );
    }
}
