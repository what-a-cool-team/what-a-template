use crate::FileSystem;
use tokio::fs;
use tokio::io::{AsyncRead, AsyncReadExt};
use std::path::Path;
use std::pin::Pin;
use anyhow::Context;
use async_trait::async_trait;

pub struct LocalFileSystem;

impl LocalFileSystem {
    pub fn new() -> Self {
        LocalFileSystem
    }
}

#[async_trait]
impl FileSystem for LocalFileSystem {
    async fn create(&mut self, path: &str) -> anyhow::Result<()> {
        fs::File::create(path).await.with_context(|| format!("Failed to create {}", path))?;
        Ok(())
    }

    async fn open(&self, path: &str) -> anyhow::Result<Pin<Box<dyn AsyncRead + Send>>> {
        let file = fs::File::open(path).await.with_context(|| format!("Failed to open {}", path))?;
        Ok(Box::pin(file))
    }

    async fn read(&self, path: &str) -> anyhow::Result<Vec<u8>> {
        let mut file = fs::File::open(path).await.context(format!("Failed to open file at {}", path))?;
        let mut buf = Vec::new();
        file.read_to_end(&mut buf).await.context(format!("Failed to read from {}", path))?;
        Ok(buf)
    }

    async fn delete(&mut self, path: &str) -> anyhow::Result<bool> {
        let result = fs::remove_file(path).await.is_ok();
        Ok(result)
    }

    fn exists(&self, path: &str) -> bool {
        Path::new(path).exists()
    }
}