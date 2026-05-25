//! Contains the `Config` struct, which has properties to specify or override behaviors.

use std::io::Write as _;
use std::path::{Path, PathBuf};
use std::{env, fmt, fs};

use serde::{Deserialize, Serialize};

use crate::cli::{arguments as args, subcommands as sub};

use std::ffi::OsString;

use thiserror::Error;

/// Convenience type for configuration related operations.
pub type Result<T> = std::result::Result<T, self::Error>;

/// Error enum for configuration related operations.
#[derive(Error, Debug)]
pub enum Error {
    /// Used when the 'config set' command is used but no flags are passed.
    #[error("You must provide arguments to set (e.g.: --persister tasks.json)")]
    EmptySetArgs,

    /// Used for [env errors][`std::env::VarError`].
    #[error("{0}")]
    Env(#[from] std::env::VarError),

    /// Used when the `POSTIT_ROOT` has a blank value.
    #[error("The 'POSTIT_ROOT' environment variable is empty")]
    EmptyEnvVar,

    /// Used when the value of `POSTIT_ROOT` is not a valid path.
    #[error("The value of 'POSTIT_ROOT' is not a valid path or is a relative path: {0}")]
    InvalidPathEnvVar(PathBuf),

    /// Used when the `POSTIT_ROOT`
    #[error("The value of 'POSTIT_ROOT' is not unicode: {0:?}")]
    NotUnicodeEnv(OsString),

    /// Used for [I/O errors][`std::io::Error`].
    #[error("{0}")]
    Io(#[from] std::io::Error),

    /// Used when the configuration file doesn't exist when it was expected to.
    #[error("The configuration file doesn't exist at '{0}'")]
    FileDoesntExist(PathBuf),

    /// Used when the configuration file already exists when it wasn't expected to.
    #[error("The configuration file already exists at '{0}'")]
    FileAlreadyExists(PathBuf),

    /// Used when there is an error serializing a TOML structure ([`toml::ser::Error`]).
    #[error("Failed to serialize config to TOML: {0}")]
    TOMLSerialize(#[from] toml::ser::Error),

    /// Used when there is an error deserializing a TOML structure ([`toml::de::Error`]).
    #[error("Failed to deserialize TOML to config: {0}")]
    TOMLDeserialize(#[from] toml::de::Error),

    /// Any error that doesn't belong into the previous variants.
    #[error("{0}")]
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

/// Contains the configuration used while running `postit`.
///
/// If the configuration file doesn't exist, it uses the default values defined
/// in the [Default] trait implementation.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Config {
    /// Defines where tasks are stored. It can be the path to a file or a database connection string (including protocol).
    pub persister: String,
    /// If `true`, allows dropping tasks without them being checked.
    pub force_drop: bool,
    /// If `true`, allows overwriting files if they already exist.
    pub force_copy: bool,
    /// If `true`, drops the old file after copying its contents to the new file.
    pub drop_after_copy: bool,
}

impl Default for Config {
    #[inline]
    fn default() -> Self {
        Self {
            persister: String::from("tasks.csv"),
            force_drop: false,
            force_copy: false,
            drop_after_copy: false,
        }
    }
}

impl fmt::Display for Config {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "persister: {}", self.persister)?;
        writeln!(f, "force_drop: {}", self.force_drop)?;
        writeln!(f, "force_copy: {}", self.force_copy)?;
        write!(f, "drop_after_copy: {}", self.drop_after_copy)
    }
}

// Methods for managing the 'config' commands
impl Config {
    /// Manages the `.postit.toml` file using a `ConfigSubcommand` instance.
    ///
    /// # Errors
    /// - Any error while doing operations on a the configuration file.
    #[inline]
    pub fn manage(subcommand: sub::Config) -> Result<()> {
        match subcommand {
            sub::Config::Env => Self::print_env(),
            sub::Config::Path => Self::print_path(),
            sub::Config::Init => Self::init(),
            sub::Config::Remove => Self::remove(),
            sub::Config::List => Self::list(),
            sub::Config::Set(args) => Self::set(args),
        }
    }

    /// Creates the config file from the default values.
    ///
    /// # Errors
    /// - The path can't be obtained.
    /// - The config file already exists at the used path.
    #[inline]
    pub fn init() -> Result<()> {
        let path = Self::path()?;

        if path.exists() {
            return Err(Error::FileAlreadyExists(path));
        }

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        let mut file = fs::File::create(&path)?;
        let toml = toml::to_string_pretty(&Self::default())?;

        file.write_all(toml.as_bytes()).map_err(|e| {
            eprintln!("Failed to write default config to file");
            Error::Io(e)
        })?;

        println!("Configuration file created at '{}'", path.display());

        Ok(())
    }

    /// Prints the value of the `POSTIT_ROOT` env var.
    ///
    /// # Errors
    /// - The `POSTIT_ROOT` exists but is empty.
    #[inline]
    pub fn print_env() -> Result<()> {
        let env = Self::env().unwrap_or_default();

        if env.is_empty() {
            return Err(Error::EmptyEnvVar);
        }

        println!("{env}");

        Ok(())
    }

    /// Prints the path of the config file.
    ///
    /// # Errors
    /// - The file doesn't exist at the parent path.
    /// - The path can't be obtained from the `POSTIT_ROOT` env var.
    #[inline]
    pub fn print_path() -> Result<()> {
        Self::_check_path_exists()?;

        let path = Self::path()?;

        println!("{}", path.display());

        Ok(())
    }

    /// Deletes the config file.
    ///
    /// # Errors
    /// - The path can't be obtained from the `POSTIT_ROOT` env var.
    /// - The file doesn't exist at the parent path.
    ///
    /// # Panics
    /// - The parent can't be obtained from the path.
    #[inline]
    pub fn remove() -> Result<()> {
        let path = Self::path()?;

        if !path.exists() {
            let parent = path.parent().unwrap().to_path_buf();
            return Err(Error::FileDoesntExist(parent));
        }

        fs::remove_file(&path).map_err(|e| {
            eprintln!("Config file couldn't be deleted.");
            Error::Io(e)
        })?;

        println!("Config file removed from '{}'", path.parent().unwrap().display());

        Ok(())
    }

    /// Displays a list of the current config values.
    ///
    /// # Errors
    /// - The file doesn't exist at the parent path (displays default config too).
    /// - The configuration can't be loaded.
    #[inline]
    pub fn list() -> Result<()> {
        let result = Self::_check_path_exists();

        if let Err(e) = result {
            println!("Default configuration:");
            println!("{}", Self::default());
            println!();

            return Err(e);
        }

        println!("{}", Self::load()?);

        Ok(())
    }

    /// Sets a value for the passed key.
    ///
    /// # Errors
    /// - The file doesn't exist at the parent path.
    /// - There are no values provided.
    /// - The configuration can't be loaded.
    #[inline]
    pub fn set(args: args::ConfigSet) -> Result<()> {
        Self::_check_path_exists()?;

        if args.persister.is_none()
            && args.force_drop.is_none()
            && args.force_copy.is_none()
            && args.drop_after_copy.is_none()
        {
            return Err(Error::EmptySetArgs);
        }

        let mut config = Self::load()?;

        if let Some(new) = args.persister {
            println!("persister: {} -> {}", config.persister, new);
            config.persister = new;
        }

        if let Some(new) = args.force_drop {
            println!("force_drop: {} -> {}", config.force_drop, new);
            config.force_drop = new;
        }

        if let Some(new) = args.force_copy {
            println!("force_copy: {} -> {}", config.force_copy, new);
            config.force_copy = new;
        }

        if let Some(new) = args.drop_after_copy {
            println!("drop_after_copy: {} -> {}", config.drop_after_copy, new);
            config.drop_after_copy = new;
        }

        println!();

        config.save()
    }
}

// Utility methods to interact with the configuration
impl Config {
    /// Returns the value of the `POSTIT_ROOT` environment variable, which must
    /// have a path structure.
    ///
    /// # Errors
    /// - The `POSTIT_ROOT` env var is not present or has not unicode characters.
    #[inline]
    pub fn env() -> Result<String> {
        env::var("POSTIT_ROOT").map_err(Error::Env)
    }

    /// Returns the name of the config file.
    #[inline]
    pub fn config_file_name() -> String {
        String::from(".postit.toml")
    }

    /// Returns the value of the `POSTIT_ROOT` environment variable, which must
    /// have a path structure.
    ///
    /// # Errors
    /// - The `POSTIT_ROOT` exists but is empty.
    /// - The value of `POSTIT_ROOT` contains not unicode characters.
    /// - The path from `POSTIT_ROOT` is relative.
    #[inline]
    pub fn path_from_env() -> Result<PathBuf> {
        let env = Self::env();

        let path = match env {
            Ok(v) if v.is_empty() => Err(Error::EmptyEnvVar),
            Ok(v) => Ok(PathBuf::from(v)),

            Err(Error::Env(e)) => match e {
                env::VarError::NotPresent => Ok(Self::default_config_parent()),
                env::VarError::NotUnicode(msg) => Err(Error::NotUnicodeEnv(msg)),
            },

            Err(_) => unreachable!(),
        }?;

        if path.is_relative() {
            return Err(Error::InvalidPathEnvVar(path));
        }

        Ok(path)
    }

    /// Returns the HOME path of the currently used OS, which will be the
    /// default path of postit's generated files.
    ///
    /// # Panics
    /// - The user's home directory can't be located.
    #[inline]
    pub fn home() -> PathBuf {
        dirs::home_dir().expect("Couldn't locate the user's home directory")
    }

    /// Returns the default path of postit's config file.
    #[inline]
    pub fn default_config_parent() -> PathBuf {
        Self::home().join(".postit")
    }

    /// Returns the path of the config file in the `POSTIT_ROOT` env var.
    ///
    /// # Errors
    /// - The path can't be obtained from the `POSTIT_ROOT` env var.
    #[inline]
    pub fn path() -> Result<PathBuf> {
        Ok(Self::path_from_env()?.join(Self::config_file_name()))
    }

    /// Checks if the path exists.
    ///
    /// # Errors
    /// - The file doesn't exist at the parent path.
    ///
    /// # Panics
    /// - The parent can't be obtained from the path.
    #[inline]
    pub fn _check_path_exists() -> Result<()> {
        let path = Self::path()?;

        if !path.exists() {
            let parent = path.parent().unwrap().to_path_buf();
            return Err(Error::FileDoesntExist(parent));
        }

        Ok(())
    }

    /// Obtains the path for the File instance, which is the parent path that
    /// the stores the config file.
    ///
    /// # Errors
    /// - The path can't be obtained.
    ///
    /// # Panics
    /// - The parent path can't be extracted from the configuration path.
    #[inline]
    pub fn get_parent_path() -> Result<PathBuf> {
        Ok(Self::path()?.parent().unwrap().to_path_buf())
    }

    /// Returns the path constructed from pushing the file persister path to
    /// the parent path (the one where .postit.toml is stored).
    ///
    /// # Errors
    /// - The path passed doesn't have a parent path.
    ///
    /// # Panics
    /// - If the path can't be converted to str.
    /// - If the parent path can't be converted to str.
    #[inline]
    pub fn build_path<T: AsRef<Path>>(path: T) -> Result<PathBuf> {
        let path = path.as_ref();

        let parent = Self::get_parent_path()?;

        if path.starts_with(&parent) {
            return Ok(path.to_path_buf());
        }

        Ok(parent.join(path))
    }

    /// Loads the config from a file or creates it if it doesn't exist.
    ///
    /// # Errors
    /// - The config file can't be loaded.
    /// - The config file can't be read.
    #[inline]
    pub fn load() -> Result<Self> {
        let path = Self::path()?;

        if !path.exists() {
            return Ok(Self::default());
        }

        let content = fs::read_to_string(path).map_err(|e| {
            eprintln!("Failed to read config file");
            Error::Io(e)
        })?;

        let config = toml::from_str(&content)?;

        Ok(config)
    }

    /// Saves the config instance to a file.
    ///
    /// # Errors
    /// - The config path can't be obtained.
    /// - The config file can't be created.
    /// - The config can't be formatted to TOML.
    /// - The config file can't be saved.
    #[inline]
    pub fn save(&self) -> Result<()> {
        let path = Self::path()?;

        let mut file = fs::File::create(&path).map_err(|e| {
            eprintln!("Failed to open the config file {}: {e}", path.display());
            Error::Io(e)
        })?;

        let toml = toml::to_string_pretty(self)?;

        file.write_all(toml.as_bytes()).map_err(|e| {
            eprintln!("Failed to save config to file: {e}");
            Error::Io(e)
        })?;

        println!("Configuration saved");

        Ok(())
    }
}
