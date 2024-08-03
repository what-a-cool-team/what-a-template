use std::io::Read;
use config::{ConfigError};
use anyhow::{Context, Result};

pub trait FileSystem {
    fn create(&mut self, path: &str) -> Result<()>;
    fn open(&self, path: &str) -> Result<Box<dyn Read>>;
    fn read(&self, path: &str) -> Result<Vec<u8>> {
        let mut file = self.open(path)?;
        let mut buf = Vec::new();
        file.read_to_end(&mut buf).with_context(|| format!("Failed to read from {}", path))?;
        Ok(buf)
    }
    fn delete(&mut self, path: &str) -> Result<bool>;
    fn exists(&self, path: &str) -> bool;
    fn read_to_string(&self, path: &str) -> Result<String> {
        let data = self.read(path)?;
        Ok(String::from_utf8(data)?)
    }
}

pub struct FileSource {
    pub content: String,
}

impl FileSource {
    pub fn new(content: String) -> Self {
        FileSource { content }
    }

    pub fn from_path(file_system: &dyn FileSystem, path: &str) -> Result<Self, ConfigError> {
        if file_system.exists(path) {
            let content = file_system.read_to_string(path).unwrap();
            Ok(Self { content })
        } else {
            Err(ConfigError::Message("File does not exist".into()))
        }
    }
}

pub use self::local::LocalFileSystem;
pub use self::in_memory::InMemoryFileSystem;

mod local;
mod in_memory;