pub mod controller;
pub mod models;
pub mod repository;

use chrono::{DateTime, Local, NaiveDateTime, TimeZone, Utc};

pub fn naive_to_utc(dt: NaiveDateTime) -> Result<DateTime<Utc>, &'static str> {
    Local
        .from_local_datetime(&dt)
        .earliest()
        .ok_or("Invalid Time")
        .map(|dt| dt.with_timezone(&Utc))
}
