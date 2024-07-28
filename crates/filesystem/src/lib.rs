pub trait FileSystem {
    fn add_file(&mut self, path: &str, contents: &str) -> Result<(), String>;
    fn remove_file(&mut self, path: &str) -> Result<(), String>;
    fn read_file(&self, path: &str) -> Result<String, String>;
}

pub use self::real::RealFileSystem;
pub use self::mock::MockFileSystem;

mod real;
mod mock;