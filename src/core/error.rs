//! Defines errors related to file management.

use thiserror::Error;

/// Global error handler.
#[non_exhaustive]
#[derive(Error, Debug)]
pub enum Error {
    /// Used for config related [errors][`crate::config::Error`].
    #[error(transparent)]
    Config(#[from] crate::config::Error),

    /// Used for file system related [errors][`crate::fs::Error`].
    #[error(transparent)]
    Fs(#[from] crate::fs::Error),

    /// Used for database related [errors][`crate::fs::Error`].
    #[cfg(any(feature = "mongo", feature = "sqlite"))]
    #[error(transparent)]
    Db(#[from] crate::db::Error),

    /// Used for I/O errors ([`std::io::Error`]).
    #[error(transparent)]
    Io(#[from] std::io::Error),
}
