use chrono::{DateTime, Datelike, Timelike, Utc};
use std::fmt::Display;

pub fn display_time(value: &DateTime<Utc>) -> impl Display {
    format!(
        "{:02}.{:02}.{:04} {:02}:{:02}",
        value.day(),
        value.month(),
        value.year(),
        value.hour(),
        value.minute()
    )
}
