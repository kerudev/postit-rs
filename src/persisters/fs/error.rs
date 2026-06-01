//! Defines errors related to file management.

use thiserror::Error;

/// Errors related to file and path management.
#[non_exhaustive]
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
}
