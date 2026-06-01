//! Defines errors related to database management.

use thiserror::Error;

/// Errors related to databases and connection strings.
#[non_exhaustive]
#[derive(Error, Debug)]
pub enum Error {
    /// Used when the provided connection string is not supported.
    #[error("Unsupported database; defaulting to Sqlite")]
    UnsupportedDatabase,

    /// Used when the provided connection string is incorrect.
    #[error("The provided connection string is incorrect")]
    IncorrectConnectionString,

    /// Represent an `SQLite` error.
    #[cfg(feature = "sqlite")]
    #[error("Error on SQLite: {0}")]
    Sqlite(#[from] sqlite::Error),

    /// Represent a `MongoDB` error.
    #[cfg(feature = "mongo")]
    #[error("Error on MongoDB: {0}")]
    Mongo(#[from] mongodb::error::Error),
}
