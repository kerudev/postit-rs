//! Contains the [ORM][`Orm`] and handlers for the supported databases.
//!
//! The currently supported databases are:
//! - [`SQLite`][`Sqlite`]
//! - [`MongoDB`][`Mongo`]
//! - [`MongoDB Atlas`][`Mongo`]

mod error;

#[cfg(feature = "mongo")]
mod mongo;
mod orm;

#[cfg(feature = "sqlite")]
mod sqlite;
pub use error::{Error, Result};
#[cfg(feature = "mongo")]
pub use mongo::Mongo;

pub use orm::{Orm, Protocol};

#[cfg(feature = "sqlite")]
pub use sqlite::Sqlite;
