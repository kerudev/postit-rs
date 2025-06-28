//! Module for file management.
//!
//! The currently supported formats are:
//! - csv
//! - json
//! - xml

mod csv;
mod error;
mod file;

#[cfg(feature = "json")]
mod json;

#[cfg(feature = "xml")]
mod xml;

pub use csv::Csv;
pub use error::{Error, Result};
pub use file::{File, Format};

#[cfg(feature = "json")]
pub use json::Json;

#[cfg(feature = "xml")]
pub use xml::Xml;
