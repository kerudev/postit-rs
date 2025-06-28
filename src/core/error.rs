//! Defines errors related to file management.

use thiserror::Error;

/// Convenience type for database related operations.
pub type Result<T> = std::result::Result<T, self::Error>;

/// Errors related to file and path management.
#[derive(Error, Debug)]
pub enum Error {
    /// Used for config related [errors][`crate::config::Error`].
    #[error("{0}")]
    Config(#[from] crate::config::Error),

    /// Used for file system related [errors][`crate::fs::Error`].
    #[error("{0}")]
    Fs(#[from] crate::fs::Error),

    /// Used for database related [errors][`crate::fs::Error`].
    #[cfg(any(feature = "mongo", feature = "sqlite"))]
    #[error("{0}")]
    Db(#[from] crate::db::Error),

    /// Used for I/O errors ([`std::io::Error`]).
    #[error("{0}")]
    Io(#[from] std::io::Error),

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
