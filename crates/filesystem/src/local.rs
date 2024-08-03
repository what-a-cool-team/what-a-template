use crate::FileSystem;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use anyhow::Context;

pub struct LocalFileSystem;

impl FileSystem for LocalFileSystem {
    fn create(&mut self, path: &str) -> anyhow::Result<()> {
        File::create(path).with_context(|| format!("Failed to create {}", path))?;
        Ok(())
    }

    fn open(&self, path: &str) -> anyhow::Result<Box<dyn Read>> {
        let file = File::open(path).with_context(|| format!("Failed to open {}", path))?;
        Ok(Box::new(file))
    }

    fn read(&self, path: &str) -> anyhow::Result<Vec<u8>> {
        let mut file = File::open(path).context(format!("Failed to open file at {}", path))?;
        let mut buf = Vec::new();
        file.read_to_end(&mut buf).context(format!("Failed to read from {}", path))?;
        Ok(buf)
    }

    fn delete(&mut self, path: &str) -> anyhow::Result<bool> {
        let result = fs::remove_file(path).is_ok();
        Ok(result)
    }

    fn exists(&self, path: &str) -> bool {
        Path::new(path).exists()
    }
}