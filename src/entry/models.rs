use chrono::{DateTime, Utc};
use cli_table::Table;
use serde::{Deserialize, Serialize};

use crate::entry::utils::display_time;

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
