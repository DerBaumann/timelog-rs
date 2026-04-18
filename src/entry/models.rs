use std::fmt::Display;

use chrono::{DateTime, Local, Utc};
use cli_table::Table;
use serde::{Deserialize, Serialize};

fn display_time(value: &DateTime<Utc>) -> impl Display {
    value.with_timezone(&Local).format("%d.%m.%Y - %H:%M")
}

#[derive(Debug, Clone, Serialize, Deserialize, Table)]
pub struct Entry {
    pub id: u32,
    #[table(title = "Projekt")]
    pub project: String,
    #[table(title = "Startzeit", display_fn = "display_time")]
    pub start_time: DateTime<Utc>,
    #[table(title = "Endzeit", display_fn = "display_time")]
    pub end_time: DateTime<Utc>,
    #[table(title = "Beschreibung")]
    pub description: String,
}

impl Entry {
    pub fn new(
        project: String,
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
        description: String,
    ) -> Self {
        Entry {
            id: 0,
            project,
            start_time,
            end_time,
            description,
        }
    }
}
