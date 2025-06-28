//! Defines errors related to file management.

use thiserror::Error;

/// Convenience type for database related operations.
pub type Result<T> = std::result::Result<T, self::Error>;

/// Errors related to file and path management.
#[derive(Error, Debug)]
pub enum Error {
    /// Used when the file is actually a directory.
    #[error("The persister can't be a directory")]
    IsDirectory,

    /// Used for file format related issues.
    #[error("Unsupported file format; defaulting to CSV")]
    UnsupportedFormat,

    /// Used when a file doesn't exist when it was expected to.
    #[error("The file '{0}' doesn't exist")]
    FileDoesntExist(String),

    /// Used for I/O errors ([`std::io::Error`]).
    #[error("{0}")]
    Io(#[from] std::io::Error),

    /// Used for JSON serde errors ([`serde_json::Error`]).
    #[cfg(feature = "json")]
    #[error("{0}")]
    Json(#[from] serde_json::Error),

    /// Used for JSON serde errors ([`quick_xml::Error`]).
    #[cfg(feature = "xml")]
    #[error("{0}")]
    Xml(#[from] quick_xml::Error),

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
