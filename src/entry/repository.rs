use crate::JsonStore;
use crate::JsonStoreError;
use crate::entry::models::Entry;
use std::collections::HashSet;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RepositoryError {
    #[error("entry not found")]
    NotFound,
    #[error("io error: {0}")]
    JsonStoreError(#[from] JsonStoreError),
}

// TODO: replace file_path with store object
pub struct EntryRepository {
    pub file_path: PathBuf,
}

impl EntryRepository {
    pub fn fetch_all(&self) -> Result<Vec<Entry>, RepositoryError> {
        let store = JsonStore::read(&self.file_path)?;
        Ok(store.entries)
    }

    pub fn fetch_one(&self, id: u32) -> Result<Entry, RepositoryError> {
        JsonStore::read(&self.file_path)?
            .entries
            .into_iter()
            .find(|e| e.id == id)
            .ok_or(RepositoryError::NotFound)
    }

    pub fn fetch_projects(&self) -> Result<Vec<String>, RepositoryError> {
        let projects = JsonStore::read(&self.file_path)?
            .entries
            .into_iter()
            .map(|e| e.project)
            .collect::<HashSet<_>>()
            .into_iter()
            .collect();

        Ok(projects)
    }

    pub fn create(&self, mut entity: Entry) -> Result<Entry, RepositoryError> {
        let mut store = JsonStore::read(&self.file_path)?;
        entity.id = store.next_id;
        store.next_id += 1;
        store.entries.push(entity.clone());
        store.write(&self.file_path)?;
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

        store.write(&self.file_path)?;
        if found {
            Ok(entity)
        } else {
            Err(RepositoryError::NotFound)
        }
    }

    pub fn delete(&self, id: u32) -> Result<(), RepositoryError> {
        let mut store = JsonStore::read(&self.file_path)?;
        store.entries.retain(|e| e.id != id);
        store.write(&self.file_path)?;
        Ok(())
    }
}
