//! Defines errors related to database management.

use thiserror::Error;

/// Convenience type for database related operations.
pub type Result<T> = std::result::Result<T, self::Error>;

/// Errors related to databases and connection strings.
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

    /// Any error that doesn't belong into the previous variants.
    #[error("{0}")]
    Other(#[from] Box<dyn std::error::Error + Send + Sync>),
}

impl Error {
    /// Wraps any error-like value into [`Error::Other`].
    #[inline]
    pub fn wrap<E>(err: E) -> Self
    where
        E: Into<Box<dyn std::error::Error + Send + Sync>>,
    {
        Self::Other(err.into())
    }
}
