//! Argument parsing utilities with [clap].

use arguments as args;
use clap::{Parser, Subcommand};

/// Contains the arguments struct used.
#[doc(hidden)]
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
#[doc(hidden)]
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
        /// Command > config    (alias: conf)
        #[command(alias = "conf")]
        Config,
        /// Command > view      (alias: v)
        #[command(alias = "v")]
        View,
        /// Command > add       (alias: a)
        #[command(alias = "a")]
        Add,
        /// Command > set       (alias: s)
        #[command(alias = "s")]
        Set,
        /// Command > check     (alias: c)
        #[command(alias = "c")]
        Check,
        /// Command > uncheck   (alias: uc)
        #[command(alias = "uc")]
        Uncheck,
        /// Command > drop      (alias: d)
        #[command(alias = "d")]
        Drop,
        /// Command > copy      (alias: cp)
        #[command(alias = "cp")]
        Copy,
        /// Command > clean     (alias: cl)
        #[command(alias = "cl")]
        Clean,
        /// Command > remove    (alias: rm)
        #[command(alias = "rm")]
        Remove,
        /// Command > sample    (alias: sa)
        #[command(alias = "sa")]
        Sample,
        /// Flag    > persister
        Persister,
    }
}

/// Contains the different commands available.
#[doc(hidden)]
#[derive(Subcommand, Debug)]
pub enum Command {
    /// Manages the configuration file                  (alias: conf)
    #[command(alias = "conf")]
    Config(args::Config),

    /// Shows a list of the current tasks               (alias: v)
    #[command(alias = "v")]
    View(args::Persister),

    /// Adds a new task to the list                     (alias: a)
    #[command(alias = "a")]
    Add(args::Add),

    /// Changes values inside of tasks                  (alias: s)
    #[command(alias = "s")]
    Set(args::Set),

    /// Marks a task as checked                         (alias: c)
    #[command(alias = "c")]
    Check(args::Edit),

    /// Marks a task as unchecked                       (alias: uc)
    #[command(alias = "uc")]
    Uncheck(args::Edit),

    /// Deletes a task from the list                    (alias: d)
    #[command(alias = "d")]
    Drop(args::Edit),

    /// Copies tasks to other files or formats          (alias: cp)
    #[command(alias = "cp")]
    Copy(args::Copy),

    /// Cleans tasks from a persister                   (alias: cl)
    #[command(alias = "cl")]
    Clean(args::Persister),

    /// Removes a persister (file or table) completely  (alias: rm)
    #[command(alias = "rm")]
    Remove(args::Persister),

    /// Creates a sample of tasks for testing purposes  (alias: sa)
    #[command(alias = "sa")]
    Sample(args::Persister),

    /// Documentation and use examples                  (alias: man)
    #[command(alias = "man")]
    Docs(args::Docs),
}

/// Manages the command and arguments received from console.
#[doc(hidden)]
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None, next_line_help = false)]
pub struct Cli {
    /// Command to execute
    #[command(subcommand)]
    pub command: Command,
}
