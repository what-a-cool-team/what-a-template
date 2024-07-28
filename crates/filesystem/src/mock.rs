use std::collections::HashMap;
use crate::FileSystem;

pub struct MockFileSystem {
    files: HashMap<String, String>,
}

impl MockFileSystem {
    pub fn new() -> Self {
        MockFileSystem {
            files: HashMap::new(),
        }
    }
}

impl FileSystem for MockFileSystem {
    fn add_file(&mut self, path: &str, contents: &str) -> Result<(), String> {
        self.files.insert(path.to_string(), contents.to_string());
        Ok(())
    }

    fn remove_file(&mut self, path: &str) -> Result<(), String> {
        self.files.remove(path).map(|_| ()).ok_or_else(|| "File not found".to_string())
    }

    fn read_file(&self, path: &str) -> Result<String, String> {
        self.files.get(path).cloned().ok_or_else(|| "File not found".to_string())
    }
}