//! Argument parsing utilities with [clap].

use arguments as args;
use clap::{Parser, Subcommand};

/// Contains the arguments struct used.
pub mod arguments {
    use clap::Args;

    use super::subcommands as sub;
    use crate::models::Priority;

    /// Arguments of the 'docs' command.
    #[derive(Args, Debug)]
    pub struct Docs {
        /// Subcommand the `Docs` command will use.
        #[command(subcommand)]
        pub subcommand: sub::Docs,
    }

    /// Defines a common argument for commands that just use the persister value.
    #[derive(Args, Debug)]
    pub struct Persister {
        /// Used to read from and save tasks to.
        #[arg(long, short)]
        pub persister: Option<String>,
    }

    /// Arguments of the 'add' command.
    #[derive(Args, Debug)]
    pub struct Add {
        /// Used to read from and save tasks to.
        #[arg(long, short)]
        pub persister: Option<String>,

        /// Priority of the task (none, low, med or high).
        #[arg(value_enum)]
        pub priority: Priority,

        /// The content or description of a task.
        pub content: String,
    }

    /// Arguments of the 'check', 'uncheck', and 'drop' commands.
    #[derive(Args, Debug)]
    pub struct Edit {
        /// Used to read from and save tasks to.
        #[arg(long, short)]
        pub persister: Option<String>,

        /// Identifiers of tasks separated by commas.
        #[arg(value_delimiter = ',', required = true)]
        pub ids: Vec<u32>,
    }

    /// Arguments of the 'set' command.
    #[derive(Args, Debug)]
    pub struct Set {
        /// Used to read from and save tasks to.
        #[arg(long, short)]
        pub persister: Option<String>,

        /// Subcommand the `Set` command will use.
        #[command(subcommand)]
        pub subcommand: sub::Set,
    }

    /// Arguments of the 'set priority' subcommand.
    #[derive(Args, Debug)]
    pub struct SetPriority {
        /// Identifiers of tasks separated by commas.
        #[arg(value_delimiter = ',', required = true)]
        pub ids: Vec<u32>,

        /// Priority of the task (none, low, med or high).
        #[arg(value_enum)]
        pub priority: Priority,
    }

    /// Arguments of the 'set content' subcommand.
    #[derive(Args, Debug)]
    pub struct SetContent {
        /// Identifiers of tasks separated by commas.
        #[arg(value_delimiter = ',', required = true)]
        pub ids: Vec<u32>,

        /// The content or description of a task.
        pub content: String,
    }

    /// Arguments of the 'copy' command.
    #[derive(Args, Debug)]
    pub struct Copy {
        /// The persister that contains the tasks.
        pub left: String,

        /// Where the tasks will be copied to.
        pub right: String,
    }

    /// Arguments of the 'config' command.
    #[derive(Args, Debug)]
    pub struct Config {
        /// Subcommand the 'Config' command will use.
        #[command(subcommand)]
        pub subcommand: sub::Config,
    }

    /// Arguments for the 'config set' subcommand
    #[derive(Args, Clone, Debug, PartialEq, Eq)]
    pub struct ConfigSet {
        /// Defines where tasks are stored. It can be the path to a file or a database connection string (including protocol).
        #[arg(long, value_name = "STRING")]
        pub persister: Option<String>,

        /// If 'true', allows dropping tasks without them being checked.
        #[arg(long, value_name = "BOOL")]
        pub force_drop: Option<bool>,

        /// If 'true', allows overwriting files if they already exist.
        #[arg(long, value_name = "BOOL")]
        pub force_copy: Option<bool>,

        /// If 'true', drops the old file after copying its contents to the new file.
        #[arg(long, value_name = "BOOL")]
        pub drop_after_copy: Option<bool>,
    }
}

/// Contains the subcommands available used by parent commands.
pub mod subcommands {
    use clap::Subcommand;

    use super::arguments as args;

    /// Subcommands for setting the task's value.
    #[derive(Subcommand, Debug)]
    pub enum Set {
        /// Changes the 'content' value.
        Content(args::SetContent),
        /// Changes the 'priority' value.
        Priority(args::SetPriority),
    }

    /// Subcommands for managing the config file.
    #[derive(Subcommand, Debug)]
    pub enum Config {
        /// Shows the value of the `POSTIT_ROOT` env var.
        Env,
        /// Shows the config file path.
        Path,
        /// Creates the config file.
        Init,
        /// Displays a list of the current config values.
        #[command(alias = "ls")]
        List,
        /// Changes the values of config properties.
        #[command(alias = "s")]
        Set(args::ConfigSet),
        /// Deletes the config file
        #[command(alias = "rm")]
        Remove,
    }


    /// Subcommands for the 'Docs' command
    #[derive(Subcommand, Debug)]
    pub enum Docs {
        /// Documentation of the 'config' command
        Config,
        /// Documentation of the 'view' command
        View,
        /// Documentation of the 'add' command
        Add,
        /// Documentation of the 'set' command
        Set,
        /// Documentation of the 'check' command
        Check,
        /// Documentation of the 'uncheck' command
        Uncheck,
        /// Documentation of the 'drop' command
        Drop,
        /// Documentation of the 'copy' command
        Copy,
        /// Documentation of the 'clean' command
        Clean,
        /// Documentation of the 'remove' command
        Remove,
        /// Documentation of the 'sample' command
        Sample,
        /// Documentation of the 'persister' flag
        Persister,
    }
}

/// Contains the different commands available.
#[derive(Subcommand, Debug)]
pub enum Command {
    /// Manages the configuration file.
    #[command(alias = "conf")]
    Config(args::Config),

    /// Shows a list of the current tasks.
    #[command(alias = "v")]
    View(args::Persister),

    /// Adds a new task to the list.
    #[command(alias = "a")]
    Add(args::Add),

    /// Changes values inside of tasks.
    #[command(alias = "s")]
    Set(args::Set),

    /// Marks a task as checked.
    #[command(alias = "c")]
    Check(args::Edit),

    /// Marks a task as unchecked.
    #[command(alias = "uc")]
    Uncheck(args::Edit),

    /// Deletes a task from the list.
    #[command(alias = "d")]
    Drop(args::Edit),

    /// Creates a copy of a file (can parse formats, like csv to json).
    #[command(alias = "cp")]
    Copy(args::Copy),

    /// Cleans the tasks from a persister
    #[command(alias = "cl")]
    Clean(args::Persister),

    /// Removes a persister completely (file or table)
    #[command(alias = "rm")]
    Remove(args::Persister),

    /// Creates a sample of tasks. Useful to test postit's features.
    #[command(alias = "sa")]
    Sample(args::Persister),

    /// Provides documentation and use examples for commands
    #[command(alias = "man")]
    Docs(args::Docs),
}

/// Manages the command and arguments received from console.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None, next_line_help = false)]
pub struct Cli {
    /// Command to execute
    #[command(subcommand)]
    pub command: Command,
}
