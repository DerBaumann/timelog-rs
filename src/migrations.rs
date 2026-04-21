use std::{fs, path::Path};

use chrono::{Local, NaiveDate, TimeZone, Utc};
use serde::Deserialize;

use crate::{JsonStore, JsonStoreError, entry::models::Entry};

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

pub fn v1_to_v2(file_path: &Path, contents: &str) -> Result<JsonStore, JsonStoreError> {
    println!("Store version 1 detected. Running migration...");

    fs::copy(file_path, file_path.with_extension("json.bak"))?;
    println!(
        "Created backup at {}",
        file_path.with_extension("json.bak").display()
    );

    let store_v1 = serde_json::from_str::<StoreV1>(contents)?;
    let mut next_id = 1;
    let entries = store_v1
        .entries
        .iter()
        .map(|e| {
            // TODO: Remove unwrap
            let date = NaiveDate::parse_from_str(&e.date, "%Y-%m-%d").unwrap();
            let start_time = date.and_hms_opt(0, 0, 0).expect("00:00:00 is a valid time")
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
    store.write(file_path)?;
    Ok(store)
}
