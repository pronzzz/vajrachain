use sled::Db;
use std::path::Path;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum StorageError {
    #[error("Database IO error")]
    IoError(#[from] std::io::Error),
    #[error("Sled error")]
    SledError(#[from] sled::Error),
    #[error("Serialization error")]
    SerializationError(#[from] bincode::Error),
}

pub trait StateStore {
    fn put(&self, key: &[u8], value: &[u8]) -> Result<(), StorageError>;
    fn get(&self, key: &[u8]) -> Result<Option<Vec<u8>>, StorageError>;
    fn del(&self, key: &[u8]) -> Result<(), StorageError>;
    fn flush(&self) -> Result<(), StorageError>;
}

pub struct SledStore {
    db: Db,
}

impl SledStore {
    pub fn new(path: &Path) -> Result<Self, StorageError> {
        Self::new_with_cache(path, 1024 * 1024 * 1024) // Default 1GB
    }

    pub fn new_with_cache(path: &Path, cache_bytes: u64) -> Result<Self, StorageError> {
        let db = sled::Config::default()
            .path(path)
            .cache_capacity(cache_bytes)
            .open()?;
        Ok(Self { db })
    }
}

impl StateStore for SledStore {
    fn put(&self, key: &[u8], value: &[u8]) -> Result<(), StorageError> {
        self.db.insert(key, value)?;
        Ok(())
    }

    fn get(&self, key: &[u8]) -> Result<Option<Vec<u8>>, StorageError> {
        let result = self.db.get(key)?;
        Ok(result.map(|iv| iv.to_vec()))
    }

    fn del(&self, key: &[u8]) -> Result<(), StorageError> {
        self.db.remove(key)?;
        Ok(())
    }

    fn flush(&self) -> Result<(), StorageError> {
        self.db.flush()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_persistence() {
        let dir = tempdir().unwrap();
        let store = SledStore::new(dir.path()).unwrap();

        store.put(b"key", b"value").unwrap();
        let val = store.get(b"key").unwrap();
        assert_eq!(val, Some(b"value".to_vec()));
    }

    #[test]
    fn test_deletion() {
        let dir = tempdir().unwrap();
        let store = SledStore::new(dir.path()).unwrap();

        store.put(b"key", b"value").unwrap();
        store.del(b"key").unwrap();
        let val = store.get(b"key").unwrap();
        assert_eq!(val, None);
    }
}
