use std::collections::HashMap;
use std::io::{Cursor};
use std::pin::Pin;
use async_trait::async_trait;
use tokio::io::AsyncRead;
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

#[async_trait]
impl FileSystem for InMemoryFileSystem {
    async fn create(&mut self, path: &str) -> anyhow::Result<()> {
        if self.files.contains_key(path) {
            return Err(anyhow::anyhow!("File already exists: {}", path));
        }
        self.files.insert(path.to_string(), Vec::new());
        Ok(())
    }

    async fn open(&self, path: &str) -> anyhow::Result<Pin<Box<dyn AsyncRead + Send>>> {
        if let Some(content) = self.files.get(path) {
            Ok(Box::pin(Cursor::new(content.clone())))
        } else {
            Err(anyhow::anyhow!("File not found: {}", path))
        }
    }

    async fn read(&self, path: &str) -> anyhow::Result<Vec<u8>> {
        if let Some(content) = self.files.get(path) {
            Ok(content.clone())
        } else {
            Err(anyhow::anyhow!("File not found"))
        }
    }

    async fn delete(&mut self, path: &str) -> anyhow::Result<bool> {
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