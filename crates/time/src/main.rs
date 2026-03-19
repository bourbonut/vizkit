use chrono::{TimeZone, Utc};
use time::TimeInterval;

fn main() {
    // println!(
    //     "{:?}",
    //     TimeInterval::day().range(
    //         Utc.with_ymd_and_hms(2010, 1, 1, 0, 0, 0).unwrap(),
    //         Utc.with_ymd_and_hms(2010, 1, 5, 0, 0, 0).unwrap(),
    //         1
    //     )
    // );
    println!(
        "{:?}",
        TimeInterval::day().every(3).range(
            Utc.with_ymd_and_hms(2010, 1, 1, 0, 0, 0).unwrap(),
            Utc.with_ymd_and_hms(2010, 1, 8, 0, 0, 0).unwrap(),
            1
        )
    )
}
