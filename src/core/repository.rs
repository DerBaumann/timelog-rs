use crate::core::store::json_store;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RepositoryError {
    #[error("entry not found")]
    NotFound,
    #[error("io error: {0}")]
    JsonStoreError(#[from] json_store::JsonStoreError),
}

pub trait Repository<T> {
    fn fetch_all(&self) -> Result<Vec<T>, RepositoryError>;
    fn create(&self, entity: T) -> Result<T, RepositoryError>;
    fn update(&self, entity: T) -> Result<T, RepositoryError>;
    fn delete(&self, id: u32) -> Result<(), RepositoryError>;
    fn fetch_one(&self, id: u32) -> Result<T, RepositoryError>;
}
