//! This module provides time interval operations for working with time in general.
//!
//! You should use mainly [`TimeInterval`] which provides all time operations available.
//!
//! ```
//! use chrono::NaiveDate;
//! use vizkit::time::TimeInterval;
//!
//! // Returns `DateTime<Utc>`
//! let datetime = |year, month, day| {
//!     NaiveDate::from_ymd_opt(year, month, day)
//!         .and_then(|date| date.and_hms_opt(0, 0, 0))
//!         .expect("invalid time values")
//!         .and_utc()
//! };
//!
//! assert_eq!(
//!     TimeInterval::month()
//!         .every(3)
//!         .range(datetime(2008, 12, 3), datetime(2010, 7, 5), 1),
//!     vec![
//!         datetime(2009, 1, 1),
//!         datetime(2009, 4, 1),
//!         datetime(2009, 7, 1),
//!         datetime(2009, 10, 1),
//!         datetime(2010, 1, 1),
//!         datetime(2010, 4, 1),
//!         datetime(2010, 7, 1),
//!     ]
//! )
//! ```

mod day;
mod hour;
mod interval;
mod millisecond;
mod minute;
mod month;
mod second;
mod year;

pub use self::{
    day::Day,
    hour::Hour,
    interval::{Every, TimeInterval, Timing},
    millisecond::Millisecond,
    minute::Minute,
    month::Month,
    second::Second,
    year::Year,
};
