use std::collections::HashMap;
use std::io::{Cursor, Read};
use crate::FileSystem;

#[derive(Clone)]
pub struct InMemoryFileSystem {
    files: HashMap<String, Vec<u8>>,
}

impl InMemoryFileSystem {
    pub fn new() -> Self {
        Self {
            files: HashMap::new(),
        }
    }
}

impl FileSystem for InMemoryFileSystem {
    fn create(&mut self, path: &str) -> anyhow::Result<()> {
        if self.files.contains_key(path) {
            return Err(anyhow::anyhow!("File already exists: {}", path));
        }
        self.files.insert(path.to_string(), Vec::new());
        Ok(())
    }

    fn open(&self, path: &str) -> anyhow::Result<Box<dyn Read>> {
        if let Some(content) = self.files.get(path) {
            Ok(Box::new(Cursor::new(content.clone())))
        } else {
            Err(anyhow::anyhow!("File not found: {}", path))
        }
    }

    fn read(&self, path: &str) -> anyhow::Result<Vec<u8>> {
        if let Some(content) = self.files.get(path) {
            Ok(content.clone())
        } else {
            Err(anyhow::anyhow!("File not found"))
        }
    }

    fn delete(&mut self, path: &str) -> anyhow::Result<bool> {
        if self.files.remove(path).is_some() {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn exists(&self, path: &str) -> bool {
        self.files.contains_key(path)
    }
}