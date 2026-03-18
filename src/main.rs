use chrono::{DateTime, Datelike, Local, TimeDelta, TimeZone, Timelike, Utc};

trait TimeType {
    fn floor(&self, date: DateTime<Utc>) -> Option<DateTime<Utc>>;
    fn offset(&self, date: DateTime<Utc>, step: i64) -> DateTime<Utc>;
    fn count(&self, start: DateTime<Utc>, end: DateTime<Utc>) -> i64;
    fn field(&self, date: DateTime<Utc>) -> u32;
}

struct Day;
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
        date.day()
    }
}

struct Month;
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

struct Every<T: TimeType> {
    time_type: T,
    step: u32,
}

impl<T: TimeType> TimeType for Every<T> {
    fn floor(&self, date: DateTime<Utc>) -> Option<DateTime<Utc>> {
        let mut date = self.time_type.floor(date)?;
        while self.time_type.field(date) % self.step != 0 {
            date = self.time_type.floor(date - TimeDelta::nanoseconds(1))?;
        }
        Some(date)
    }

    fn offset(&self, date: DateTime<Utc>, mut step: i64) -> DateTime<Utc> {
        if step < 0 {
            step += 1;
            while step <= 0 {
                let mut date = self.time_type.offset(date, -1);
                while self.time_type.field(date) % self.step != 0 {
                    date = self.time_type.offset(date, -1);
                }
                step += 1;
            }
        } else {
            step -= 1;
            while step >= 0 {
                let mut date = self.time_type.offset(date, 1);
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

struct TimeInterval<T: TimeType> {
    time_type: T,
}

impl<T: TimeType> TimeInterval<T> {
    fn interval(&self, date: Option<DateTime<Utc>>) -> Option<DateTime<Utc>> {
        self.time_type.floor(date.unwrap_or(Local::now().to_utc()))
    }

    fn ceil(&self, date: DateTime<Utc>) -> Option<DateTime<Utc>> {
        self.time_type
            .floor(date + TimeDelta::nanoseconds(-1))
            .map(|d| self.time_type.offset(d, 1))
            .and_then(|d| self.time_type.floor(d))
    }

    fn round(&self, date: DateTime<Utc>) -> Option<DateTime<Utc>> {
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

    fn range(&self, start: DateTime<Utc>, stop: DateTime<Utc>, step: i64) -> Vec<DateTime<Utc>> {
        let mut current = match self.ceil(start) {
            None => return Vec::new(),
            Some(start_date) if start_date >= stop => return Vec::new(),
            Some(start_date) => start_date,
        };

        let mut range: Vec<DateTime<Utc>> = vec![];
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

    fn every(self, step: u32) -> TimeInterval<Every<T>> {
        TimeInterval::<Every<T>> {
            time_type: Every {
                time_type: self.time_type,
                step,
            },
        }
    }
}

impl TimeInterval<Day> {
    fn day() -> Self {
        Self { time_type: Day }
    }
}

impl TimeInterval<Month> {
    fn month() -> Self {
        Self { time_type: Month }
    }
}

fn main() {
    // println!("{:?}", DayTime::floor(Local::now().to_utc()));
    println!(
        "{:?}",
        TimeInterval::day().range(
            Utc.with_ymd_and_hms(2010, 1, 1, 0, 0, 0).unwrap(),
            Utc.with_ymd_and_hms(2010, 1, 5, 0, 0, 0).unwrap(),
            1
        )
    );
    println!(
        "{:?}",
        TimeInterval::day().every(3).range(
            Utc.with_ymd_and_hms(2010, 1, 1, 0, 0, 0).unwrap(),
            Utc.with_ymd_and_hms(2010, 1, 8, 0, 0, 0).unwrap(),
            1
        )
    )
    // println!("{:?}", DayTime::range(DateTime::max, stop, step));
    // println!("{:?}", TimeInterval::<Month>::floor(5.0));
    // println!("{:?}", TimeInterval::<Month>::method1(5.0));
}
