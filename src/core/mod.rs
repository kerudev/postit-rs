//! This is where all the task related management happens.

pub mod cli;
pub mod config;
mod error;
mod postit;

pub use cli::{Cli, Command};
pub use error::{Error, Result};
pub use postit::Postit;

use core::fmt;

/// Possible actions taken when editing a persister's tasks.
#[derive(Clone)]
#[non_exhaustive]
pub enum Action {
    /// Used to check tasks.
    Check,
    /// Used to uncheck tasks.
    Uncheck,
    /// Used to drop tasks.
    Drop,
    /// Used to set the content of tasks.
    SetContent,
    /// Used to set the priority of tasks.
    SetPriority,
}

impl fmt::Display for Action {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::Check => write!(f, "check"),
            Self::Uncheck => write!(f, "uncheck"),
            Self::Drop => write!(f, "drop"),
            Self::SetContent => write!(f, "set content"),
            Self::SetPriority => write!(f, "set priority"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_fmt() {
        assert_eq!(Action::Check.to_string(), "check");
        assert_eq!(Action::Uncheck.to_string(), "uncheck");
        assert_eq!(Action::Drop.to_string(), "drop");
        assert_eq!(Action::SetContent.to_string(), "set content");
        assert_eq!(Action::SetPriority.to_string(), "set priority");
    }
}
