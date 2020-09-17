use std::fs::{self, File, OpenOptions};
use std::io::{BufWriter, Write};
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::Result;

pub struct KvStore {
    path: PathBuf,
    writer: BufWriter<File>,
}

impl KvStore {
    pub fn open(path: impl Into<PathBuf>) -> Result<KvStore> {
        let path = path.into();
        fs::create_dir_all(&path)?;

        let file_path = path.join("store.log");

        let writer = BufWriter::new(
            OpenOptions::new()
                .create(true)
                .write(true)
                .append(true)
                .open(&file_path)?,
        );

        Ok(KvStore { path, writer })
    }

    pub fn get(&self, key: String) -> Result<Option<String>> {
        panic!();
    }

    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        let cmd = Command::Set { key, value };
        serde_json::to_writer(&mut self.writer, &cmd)?;
        self.writer.flush()?;
        Ok(())
    }

    pub fn remove(&mut self, key: String) -> Result<()> {
        panic!();
    }
}

#[derive(Serialize, Deserialize, Debug)]
enum Command {
    Set { key: String, value: String },
    Remove { key: String },
}
