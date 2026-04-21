pub mod entry;
mod migrations;

use crate::entry::models::Entry;
use serde::{Deserialize, Serialize};
use std::{fs, io, path::Path};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum JsonStoreError {
    #[error(transparent)]
    ChronoParseError(#[from] chrono::ParseError),
    #[error("Error parsing timezone")]
    TimeZoneError,
    #[error(transparent)]
    IOError(#[from] io::Error),
    #[error(transparent)]
    SerdeJsonError(#[from] serde_json::Error),
    #[error("invalid store version")]
    InvalidVersion,
}

// TODO: Attach path to store
#[derive(Debug, Serialize, Deserialize)]
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

impl JsonStore {
    pub fn read(file_path: &Path) -> Result<JsonStore, JsonStoreError> {
        let contents = fs::read_to_string(file_path).or_else(|e| match e.kind() {
            io::ErrorKind::NotFound => {
                let empty = JsonStore {
                    version: 2,
                    entries: Vec::new(),
                    next_id: 1,
                };
                let json = serde_json::to_string(&empty)?;
                fs::write(file_path, &json)?;
                Ok(json)
            }
            _ => Err(e),
        })?;

        let version = serde_json::from_str::<StoreVersion>(&contents)?.version;

        match version {
            1 => migrations::v1_to_v2(file_path, &contents),
            2 => Ok(serde_json::from_str(&contents)?),
            _ => Err(JsonStoreError::InvalidVersion),
        }
    }

    pub fn write(&self, file_path: &Path) -> Result<(), JsonStoreError> {
        let contents = serde_json::to_string(self)?;
        fs::write(file_path, contents)?;
        Ok(())
    }
}
