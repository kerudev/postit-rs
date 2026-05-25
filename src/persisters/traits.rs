//! Contains traits related to data persisting actions, such as reading or writing.

use std::fmt::{self, Debug};
use std::fs::File;
use std::path::PathBuf;

use crate::models::{Task, Todo};
use crate::Action;

use super::fs;

#[cfg(any(feature = "mongo", feature = "sqlite"))]
use super::db;

/// Serves as a base for structures that store instances of structs that contain
/// either the [`FilePersister`] trait or the [`DbPersister`] trait.
pub trait Persister: fmt::Debug {
    /// Returns the persister instance inside a [`Box`] pointer.
    fn boxed(self) -> Box<dyn Persister>;

    /// The value that created the `Persister` instance.
    fn to_string(&self) -> String;

    /// Creates the persister instance.
    ///
    /// # Errors
    /// - The persister can't be created.
    fn create(&self) -> crate::Result<()>;

    /// Checks wether a persister exists or not.
    ///
    /// # Errors
    /// - The persister's existence can't be checked.
    fn exists(&self) -> crate::Result<bool>;

    /// Displays the tasks stored at the persister.
    ///
    /// # Errors
    /// - The tasks to display can't be obtained.
    fn view(&self) -> crate::Result<()>;

    /// Returns the tasks collected from the persister's contents.
    ///
    /// # Errors
    /// - The tasks can't be extracted from the persister.
    fn tasks(&self) -> crate::Result<Vec<Task>>;

    /// Edits a persister by managing an [`Action`] variant.
    ///
    /// # Errors
    /// - The persister can't be edited.
    fn edit(&self, todo: &Todo, ids: &[u32], action: &Action) -> crate::Result<()>;

    /// Saves a Todo instance as the persister's content.
    ///
    /// # Errors
    /// - The persister's contents can't be saved.
    fn save(&self, todo: &Todo) -> crate::Result<()>;

    /// Replaces the current data with a new [`Todo`] instance.
    ///
    /// # Errors
    /// - The persister's contents can't be replaced.
    fn replace(&self, todo: &Todo) -> crate::Result<()>;

    /// Deletes all tasks from the persister.
    ///
    /// # Errors
    /// - The persister can't be cleaned.
    fn clean(&self) -> crate::Result<()>;

    /// Removes a persister completely (file or table).
    ///
    /// # Errors
    /// - The persister can't be removed.
    fn remove(&self) -> crate::Result<()>;
}

impl PartialEq for Box<dyn Persister> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        (self.to_string() == other.to_string()) && (self.tasks().unwrap() == other.tasks().unwrap())
    }
}

impl Clone for Box<dyn Persister> {
    #[inline]
    fn clone(&self) -> Self {
        crate::Postit::get_persister(Some(self.to_string())).unwrap()
    }
}

/// Interface for data management in a file.
pub trait FilePersister: Debug {
    /// Returns the file instance inside a [`Box`] pointer.
    fn boxed(self) -> Box<dyn FilePersister>;

    /// Returns the file's path.
    fn path(&self) -> &PathBuf;

    /// Returns the default value used to initialize the file.
    fn default(&self) -> String;

    /// Returns the tasks collected from the file's contents.
    ///
    /// # Errors
    /// - The tasks can't be extracted from the file.
    fn tasks(&self) -> fs::Result<Vec<Task>>;

    /// Grants access to an open file.
    ///
    /// # Errors
    /// - The file can't be opened.
    fn open(&self) -> fs::Result<File>;

    /// Writes into a file.
    ///
    /// # Errors
    /// - Tasks can't be written.
    fn write(&self, todo: &Todo) -> fs::Result<()>;

    /// Deletes all tasks from the persister.
    ///
    /// # Errors
    /// - The file can't be cleaned.
    fn clean(&self) -> fs::Result<()>;

    /// Removes or deletes a file.
    ///
    /// # Errors
    /// - The file can't be removed.
    fn remove(&self) -> fs::Result<()>;
}

impl PartialEq for Box<dyn FilePersister> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        (self.path() == other.path()) && (self.tasks().unwrap() == other.tasks().unwrap())
    }
}

/// Interface for data management in a database.
#[cfg(any(feature = "mongo", feature = "sqlite"))]
pub trait DbPersister: Debug {
    /// Returns the database instance inside a [`Box`] pointer.
    fn boxed(self) -> Box<dyn DbPersister>;

    /// Returns the connection string.
    fn conn(&self) -> String;

    /// Returns the table used.
    fn table(&self) -> String;

    /// Returns the database used.
    fn database(&self) -> String;

    /// Checks if a table exists.
    ///
    /// # Errors
    /// - The database's or table's existence can't be checked.
    fn exists(&self) -> db::Result<bool>;

    /// Returns the tasks collected from the database's contents.
    ///
    /// # Errors
    /// - The tasks can't be extracted from the database.
    fn tasks(&self) -> db::Result<Vec<Task>>;

    /// Returns the number of results in a table.
    ///
    /// # Errors
    /// - The number of tasks can't be returned.
    fn count(&self) -> db::Result<u32>;

    /// Creates a table.
    ///
    /// # Errors
    /// - The table can't be created.
    fn create(&self) -> db::Result<()>;

    /// Inserts data into a table.
    ///
    /// # Errors
    /// - Tasks can't be inserted.
    fn insert(&self, todo: &Todo) -> db::Result<()>;

    /// Updates data from a table.
    ///
    /// # Errors
    /// - Tasks can't be updated.
    fn update(&self, todo: &Todo, ids: &[u32], action: &Action) -> db::Result<()>;

    /// Deletes data from a table.
    ///
    /// # Errors
    /// - Tasks can't be deleted.
    fn delete(&self, ids: &[u32]) -> db::Result<()>;

    /// Drops the specified table.
    ///
    /// # Errors
    /// - The table can't be dropped.
    fn drop_table(&self) -> db::Result<()>;

    /// Drops the specified database.
    ///
    /// # Errors
    /// - The database can't be dropped.
    fn drop_database(&self) -> db::Result<()>;

    /// Deletes all tasks from the persister.
    ///
    /// # Errors
    /// - The table can't be cleaned
    fn clean(&self) -> db::Result<()>;
}

#[cfg(any(feature = "mongo", feature = "sqlite"))]
impl PartialEq for Box<dyn DbPersister> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        (self.conn() == other.conn()) && (self.tasks().unwrap() == other.tasks().unwrap())
    }
}
