use crate::FileSystem;
use std::fs;
use std::io::Write;

pub struct RealFileSystem;

impl FileSystem for RealFileSystem {
    fn add_file(&mut self, path: &str, contents: &str) -> Result<(), String> {
        let mut file = fs::File::create(path).map_err(|e| e.to_string())?;
        file.write_all(contents.as_bytes()).map_err(|e| e.to_string())?;
        Ok(())
    }

    fn remove_file(&mut self, path: &str) -> Result<(), String> {
        fs::remove_file(path).map_err(|e| e.to_string())
    }

    fn read_file(&self, path: &str) -> Result<String, String> {
        fs::read_to_string(path).map_err(|e| e.to_string())
    }
}