use std::path::PathBuf;

use thiserror::Error;

use crate::JsonStore;
use crate::JsonStoreError;
use crate::entry::models::Entry;

#[derive(Debug, Error)]
pub enum RepositoryError {
    #[error("entry not found")]
    NotFound,
    #[error("io error: {0}")]
    JsonStoreError(#[from] JsonStoreError),
}

pub struct EntryRepository {
    pub file_path: PathBuf,
}

impl EntryRepository {
    pub fn fetch_all(&self) -> Result<Vec<Entry>, RepositoryError> {
        let store = JsonStore::read(&self.file_path)?;
        Ok(store.entries)
    }

    pub fn create(&self, mut entity: Entry) -> Result<Entry, RepositoryError> {
        let mut store = JsonStore::read(&self.file_path)?;
        entity.id = store.next_id;
        store.next_id += 1;
        store.entries.push(entity.clone());
        JsonStore::write(&self.file_path, store)?;
        Ok(entity)
    }

    pub fn update(&self, entity: Entry) -> Result<Entry, RepositoryError> {
        let mut store = JsonStore::read(&self.file_path)?;
        let mut found = false;

        for e in store.entries.iter_mut() {
            if e.id == entity.id {
                *e = entity.clone();
                found = true;
                break;
            }
        }

        JsonStore::write(&self.file_path, store)?;
        if found {
            Ok(entity)
        } else {
            Err(RepositoryError::NotFound)
        }
    }

    pub fn delete(&self, id: u32) -> Result<(), RepositoryError> {
        let mut store = JsonStore::read(&self.file_path)?;
        store.entries.retain(|e| e.id != id);
        JsonStore::write(&self.file_path, store)?;
        Ok(())
    }

    pub fn fetch_one(&self, id: u32) -> Result<Entry, RepositoryError> {
        JsonStore::read(&self.file_path)?
            .entries
            .into_iter()
            .find(|e| e.id == id)
            .ok_or(RepositoryError::NotFound)
    }
}
