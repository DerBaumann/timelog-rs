pub mod entry;

use crate::entry::models::Entry;
use serde::{Deserialize, Serialize};
use std::{fs, io, path::Path};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum JsonStoreError {
    #[error("io error: {0}")]
    IOError(#[from] io::Error),
    #[error("something went wrong when dealing with json: {0}")]
    JsonError(#[from] serde_json::Error),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonStore {
    pub version: u32,
    pub entries: Vec<Entry>,
    pub next_id: u32,
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
        let store = serde_json::from_str(&contents)?;
        Ok(store)
    }

    pub fn write(file_path: &Path, store: Self) -> Result<JsonStore, JsonStoreError> {
        let contents = serde_json::to_string(&store)?;
        fs::write(file_path, contents)?;
        Ok(store)
    }
}
