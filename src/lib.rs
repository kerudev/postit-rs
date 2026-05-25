//! # postit-rs - A task manager on your terminal
//!
//!
//! Postit is a CLI utility aimed to help you complete your tasks.
//!
//! It allows you to manage tasks and save a list of them for later use.
//!
//! Some of its features are:
//! - Different task colors depending on priority.
//! - Completed tasks are crossed out.
//! - Support for csv and json files.
//!
//! To get more info, run `postit -h` or take a look to the README file.

#![warn(
    clippy::single_call_fn,
    clippy::missing_docs_in_private_items,
    clippy::missing_inline_in_public_items,
    missing_docs
)]
#![allow(
    // TMP
    clippy::expect_used,
)]

mod core;
pub mod models;
mod persisters;

pub use core::*;

pub use persisters::*;
