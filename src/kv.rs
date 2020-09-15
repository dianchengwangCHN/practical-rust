use std::path::PathBuf;
use std::fs;

use crate::Result;

#[derive(Default)]
pub struct KvStore {
    path: PathBuf,
}

impl KvStore {
    pub fn open(path: impl Into<PathBuf>) -> Result<KvStore> {
        let path = path.into();
        fs::create_dir_all(&path)?;

        Ok(KvStore {
            path,
        })
    }

    pub fn get(&self, key: String) -> Result<Option<String>> {
        panic!();
    }

    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        panic!();
    }

    pub fn remove(&mut self, key: String) -> Result<()> {
        panic!();
    }
}
