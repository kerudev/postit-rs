//! Defines errors related to file management.

use std::path::PathBuf;

/// Convenience type for database related operations.
pub type Result<T> = std::result::Result<T, self::Error>;

/// Errors related to file and path management.
#[non_exhaustive]
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// Used when the file is actually a directory.
    #[error("The persister can't be a directory")]
    IsDirectory,

    /// Used for file format related issues.
    #[error("Unsupported file format; defaulting to CSV")]
    UnsupportedFormat,

    /// Used when a file doesn't exist when it was expected to.
    #[error("The file '{0}' doesn't exist")]
    FileDoesntExist(PathBuf),

    /// Used for I/O errors ([`std::io::Error`]).
    #[error(transparent)]
    Io(#[from] std::io::Error),

    /// Used for JSON serde errors ([`serde_json::Error`]).
    #[cfg(feature = "json")]
    #[error(transparent)]
    Json(#[from] serde_json::Error),

    /// Used for JSON serde errors ([`quick_xml::Error`]).
    #[cfg(feature = "xml")]
    #[error(transparent)]
    Xml(#[from] quick_xml::Error),

    /// Any error that doesn't belong into the previous variants.
    #[error(transparent)]
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
