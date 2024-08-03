use config::{ConfigError};
use anyhow::{Context, Result};
use async_trait::async_trait;
use tokio::io::{AsyncRead, AsyncReadExt};
use std::pin::Pin;

#[async_trait]
pub trait FileSystem: Send + Sync {
    async fn create(&mut self, path: &str) -> Result<()>;
    async fn open(&self, path: &str) -> Result<Pin<Box<dyn AsyncRead + Send>>>;
    async fn read(&self, path: &str) -> Result<Vec<u8>> {
        let mut file = self.open(path).await?;
        let mut buf = Vec::new();
        file.read_to_end(&mut buf).await.with_context(|| format!("Failed to read from {}", path))?;
        Ok(buf)
    }
    async fn delete(&mut self, path: &str) -> Result<bool>;
    fn exists(&self, path: &str) -> bool;
    async fn read_to_string(&self, path: &str) -> Result<String> {
        let data = self.read(path).await?;
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

    pub async fn from_path(file_system: &dyn FileSystem, path: &str) -> Result<Self, ConfigError> {
        if file_system.exists(path) {
            let content = file_system.read_to_string(path).await.unwrap();
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