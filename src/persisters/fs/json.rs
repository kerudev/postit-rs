//! Utilities to handle JSON files with [serde] and [`serde_json`].
//!
//! The `Json` struct implements the [`FilePersister`] trait.

use std::fs;
use std::path::{Path, PathBuf};

use crate::models::{Task, Todo};
use crate::traits::FilePersister;

/// Representation of a JSON file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Json {
    /// Location of the JSON file.
    path: PathBuf,
}

impl Json {
    /// Constructor of the `Json` struct.
    #[inline]
    pub fn new<T: AsRef<Path>>(path: T) -> Self {
        Self { path: path.as_ref().to_path_buf() }
    }

    /// Returns the basic structure to initialize a JSON file.
    #[inline]
    pub fn array() -> String {
        String::from("[]")
    }
}

impl FilePersister for Json {
    #[inline]
    fn boxed(self) -> Box<dyn FilePersister> {
        Box::new(self)
    }

    #[inline]
    fn path(&self) -> &Path {
        &self.path
    }

    #[inline]
    fn default(&self) -> String {
        Self::array()
    }

    #[inline]
    fn tasks(&self) -> super::Result<Vec<Task>> {
        let content = fs::read_to_string(&self.path)?;
        let tasks = serde_json::from_str(content.trim())?;

        Ok(tasks)
    }

    #[inline]
    fn open(&self) -> super::Result<fs::File> {
        let file = fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(true)
            .open(&self.path)?;

        Ok(file)
    }

    #[inline]
    fn write(&self, todo: &Todo) -> super::Result<()> {
        serde_json::to_writer_pretty(self.open()?, &todo.tasks)?;

        Ok(())
    }

    #[inline]
    fn clean(&self) -> super::Result<()> {
        fs::write(&self.path, self.default())?;

        Ok(())
    }

    #[inline]
    fn remove(&self) -> super::Result<()> {
        fs::remove_file(&self.path)?;

        Ok(())
    }
}
