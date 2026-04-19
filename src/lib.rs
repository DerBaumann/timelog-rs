pub mod entry;

use crate::entry::models::Entry;
use chrono::{Local, NaiveDate, TimeZone, Utc};
use serde::{Deserialize, Serialize};
use std::{fs, io, path::Path};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum JsonStoreError {
    #[error("io error: {0}")]
    IOError(#[from] io::Error),
    #[error("something went wrong when dealing with json: {0}")]
    JsonError(#[from] serde_json::Error),
    #[error("invalid store version")]
    InvalidVersion,
}

// TODO: Remove Clone
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JsonStore {
    // #[serde(skip)]
    // pub file_path: PathBuf,
    pub version: u32,
    pub entries: Vec<Entry>,
    pub next_id: u32,
}

#[derive(Debug, Deserialize)]
pub struct StoreVersion {
    pub version: u32,
}

// Both times store the timestamps as minutes since midnight
#[derive(Debug, Deserialize)]
struct EntryV1 {
    project_id: String,
    date: String,
    description: String,
    start_time: i32,
    end_time: i32,
}

#[derive(Debug, Deserialize)]
struct ProjectV1 {
    id: String,
    name: String,
}

#[derive(Debug, Deserialize)]
struct StoreV1 {
    entries: Vec<EntryV1>,
    projects: Vec<ProjectV1>,
}

impl JsonStore {
    pub fn read(file_path: &Path) -> Result<JsonStore, JsonStoreError> {
        let contents = fs::read_to_string(file_path).or_else(|e| match e.kind() {
            io::ErrorKind::NotFound => {
                let empty = JsonStore {
                    version: 2,
                    entries: vec![],
                    next_id: 1,
                };
                let json = serde_json::to_string(&empty)?;
                fs::write(file_path, json.clone())?;
                Ok(json)
            }
            _ => Err(e),
        })?;

        let version = serde_json::from_str::<StoreVersion>(&contents)?.version;

        match version {
            1 => {
                println!("Store version 1 detected. Running migration...");

                fs::copy(file_path, file_path.with_extension("json.bak"))?;
                println!(
                    "Created backup at {}",
                    file_path.with_extension("json.bak").display()
                );

                let store_v1 = serde_json::from_str::<StoreV1>(&contents)?;
                let mut next_id = 1;
                let entries = store_v1
                    .entries
                    .iter()
                    .map(|e| {
                        // TODO: Remove unwrap
                        let date = NaiveDate::parse_from_str(&e.date, "%Y-%m-%d").unwrap();
                        let start_time =
                            date.and_hms_opt(0, 0, 0).expect("00:00:00 is a valid time")
                                + chrono::Duration::minutes(e.start_time.into());
                        let end_time = date.and_hms_opt(0, 0, 0).expect("00:00:00 is a valid time")
                            + chrono::Duration::minutes(e.end_time.into());

                        let id = next_id;
                        next_id += 1;
                        Entry {
                            id,
                            project: store_v1
                                .projects
                                .iter()
                                .find(|p| p.id == e.project_id)
                                .map(|p| p.name.clone())
                                .unwrap_or("unknown".to_string()),
                            // TODO: Remove unwrap
                            start_time: Local
                                .from_local_datetime(&start_time)
                                .single()
                                .unwrap()
                                .with_timezone(&Utc),
                            // TODO: Remove unwrap
                            end_time: Local
                                .from_local_datetime(&end_time)
                                .single()
                                .unwrap()
                                .with_timezone(&Utc),
                            description: e.description.clone(),
                        }
                    })
                    .collect();

                let store = JsonStore {
                    version: 2,
                    entries,
                    next_id,
                };

                println!("Saving store...");
                // TODO: Remove clone
                JsonStore::write(file_path, store.clone())?;
                Ok(store)
            }
            2 => Ok(serde_json::from_str(&contents)?),
            _ => Err(JsonStoreError::InvalidVersion),
        }
    }

    // TODO: Make this a method on &self
    pub fn write(file_path: &Path, store: Self) -> Result<JsonStore, JsonStoreError> {
        let contents = serde_json::to_string(&store)?;
        fs::write(file_path, contents)?;
        Ok(store)
    }
}
