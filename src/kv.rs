use std::collections::HashMap;
use std::fs::{self, File, OpenOptions};
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use serde_json::Deserializer;

use crate::{KvsError, Result};

pub struct KvStore {
    path: PathBuf,
    writer: BufWriter<File>,
    reader: BufReader<File>,
    map: HashMap<String, String>,
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

        let mut reader = BufReader::new(File::open(&file_path)?);

        let mut map = HashMap::new();

        load(&mut reader, &mut map);

        Ok(KvStore {
            path,
            writer,
            reader,
            map,
        })
    }

    pub fn get(&self, key: String) -> Result<Option<String>> {
        Ok(self.map.get(&key).cloned())
    }

    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        let cmd = Command::Set {
            key: key.clone(),
            value: value.clone(),
        };
        serde_json::to_writer(&mut self.writer, &cmd)?;
        self.writer.flush()?;
        self.map.insert(key, value);
        Ok(())
    }

    pub fn remove(&mut self, key: String) -> Result<()> {
        let cmd = Command::Remove { key: key.clone() };
        serde_json::to_writer(&mut self.writer, &cmd)?;
        self.writer.flush()?;
        if let Some(_) = self.map.remove(&key) {
            Ok(())
        } else {
            Err(KvsError::KeyNotFound)
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
enum Command {
    Set { key: String, value: String },
    Remove { key: String },
}

fn load(reader: &mut BufReader<File>, map: &mut HashMap<String, String>) -> Result<()> {
    let mut stream = Deserializer::from_reader(reader).into_iter::<Command>();
    while let Some(cmd) = stream.next() {
        match cmd? {
            Command::Set { key, value } => {
                map.insert(key, value);
            }
            Command::Remove { key } => {
                map.remove(&key);
            }
        }
    }
    Ok(())
}
