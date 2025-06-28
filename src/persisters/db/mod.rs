//! Module for database management.
//!
//! The currently supported databases are:
//! - sqlite
//! - mongodb

#[cfg(any(feature = "mongo", feature = "sqlite"))]
mod error;

#[cfg(feature = "mongo")]
mod mongo;
#[cfg(any(feature = "mongo", feature = "sqlite"))]
mod orm;

#[cfg(feature = "sqlite")]
mod sqlite;
#[cfg(any(feature = "mongo", feature = "sqlite"))]
pub use error::{Error, Result};
#[cfg(feature = "mongo")]
pub use mongo::Mongo;

#[cfg(any(feature = "mongo", feature = "sqlite"))]
pub use orm::{Orm, Protocol};

#[cfg(feature = "sqlite")]
pub use sqlite::Sqlite;
