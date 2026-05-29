#![allow(
    clippy::missing_errors_doc,
    clippy::needless_pass_by_value,
    clippy::missing_panics_doc,
    clippy::return_self_not_must_use
)]

use std::collections::HashMap;
use std::ffi::OsStr;
use std::io::Write as _;
use std::path::{Path, PathBuf};
use std::{env, fmt, fs};

use postit::config::Config;
#[cfg(any(feature = "mongo", feature = "sqlite"))]
use postit::db::{Orm, Protocol};

#[cfg(feature = "json")]
use postit::fs::Json;
#[cfg(feature = "xml")]
use postit::fs::Xml;
use postit::fs::{Csv, File, Format};
use postit::models::Todo;

#[cfg(any(feature = "mongo", feature = "sqlite"))]
use postit::traits::DbPersister;
use postit::traits::FilePersister;

pub struct MockEnvVar {
    vars: HashMap<String, Option<String>>,
}

impl Default for MockEnvVar {
    fn default() -> Self {
        Self::new()
    }
}

impl MockEnvVar {
    pub fn new() -> Self {
        Self { vars: HashMap::new() }
    }

    pub fn set<K, V, I>(mut self, iter: I) -> Self
    where
        K: Into<String>,
        V: AsRef<OsStr>,
        I: IntoIterator<Item = (K, V)>,
    {
        for (k, v) in iter {
            let key = k.into();
            let prev = env::var(&key).ok();

            env::set_var(&key, v);

            self.vars.insert(key, prev);
        }

        self
    }

    pub fn rm<K, I>(mut self, iter: I) -> Self
    where
        K: Into<String>,
        I: IntoIterator<Item = K>,
    {
        for k in iter {
            let key = k.into();
            let prev = env::var(&key).ok();

            env::remove_var(&key);

            self.vars.insert(key, prev);
        }

        self
    }
}

impl Drop for MockEnvVar {
    fn drop(&mut self) {
        for (k, v) in &self.vars {
            match v {
                Some(val) => env::set_var(k, val),
                None => env::remove_var(k),
            }
        }
    }
}

/// A temporary path used for testing purposes.
///
/// Implements the `Display` and `Drop` traits
/// to delete the temporary path when the test ends.
pub struct MockPath {
    pub instance: Box<dyn FilePersister>,
    pub path: PathBuf,
    _env: MockEnvVar,
}

impl MockPath {
    /// Main constructor of the `MockPath` struct.
    pub fn create(format: Format) -> postit::Result<Self> {
        let mock = Self::blank(format)?;

        mock.instance.write(&Todo::sample())?;

        Ok(mock)
    }

    /// Auxiliary constructor of the `MockPath` struct.
    pub fn blank(format: Format) -> postit::Result<Self> {
        let tmp = env::current_dir()?.join("tmp");
        let mock_env = MockEnvVar::new().set([("POSTIT_ROOT", tmp)]);

        let path = Config::build_path("test_file")?;
        let name = path.to_str().unwrap();

        let file = match &format {
            #[cfg(feature = "json")]
            Format::Json => Self::json(name),

            #[cfg(feature = "xml")]
            Format::Xml => Self::xml(name),

            // TODO display message when feature is not installed
            _ => Self::csv(name),
        };

        let path = file.path().clone();

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        fs::write(&path, file.default())?;

        Ok(Self { instance: file, path, _env: mock_env })
    }

    pub fn from<T: AsRef<Path>>(path: T) -> postit::Result<Self> {
        let tmp = env::current_dir()?.join("tmp");
        let mock_env = MockEnvVar::new().set([("POSTIT_ROOT", tmp)]);

        let mut path = path.as_ref().to_path_buf();
        let var = env::var("POSTIT_ROOT").map_err(postit::Error::wrap)?;
        let tmp = Path::new(&var);

        if !path.exists() {
            fs::File::create(&path)?;
        }

        if !path.starts_with(tmp) {
            path = tmp.join(path);
        }

        let file = File::get_persister(&path)?;

        Ok(Self { instance: file, path, _env: mock_env })
    }

    pub fn csv(name: &str) -> Box<dyn FilePersister> {
        Csv::new(format!("{name}.csv")).boxed()
    }

    #[cfg(feature = "json")]
    pub fn json(name: &str) -> Box<dyn FilePersister> {
        Json::new(format!("{name}.json")).boxed()
    }

    #[cfg(feature = "xml")]
    pub fn xml(name: &str) -> Box<dyn FilePersister> {
        Xml::new(format!("{name}.xml")).boxed()
    }

    pub fn path(&self) -> PathBuf {
        self.path.clone()
    }
}

impl fmt::Display for MockPath {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.path.to_str().unwrap())
    }
}

impl Drop for MockPath {
    fn drop(&mut self) {
        if let Err(err) = fs::remove_dir_all(self.path.parent().unwrap()) {
            eprintln!("Failed to delete MockPath file: {err}");
        }
    }
}

/// A temporary connection string used for testing purposes.
///
/// Implements the `Drop` and `Clone` traits
/// to delete the temporary connection string when the test ends.
#[cfg(any(feature = "mongo", feature = "sqlite"))]
pub struct MockConn {
    pub instance: Box<dyn DbPersister>,
    _env: MockEnvVar,
}

#[cfg(any(feature = "mongo", feature = "sqlite"))]
impl MockConn {
    /// Constructor of the `MockPath` struct.
    pub fn new(conn: &str) -> postit::Result<Self> {
        let tmp = env::current_dir()?.join("tmp");
        let mock_env = MockEnvVar::new().set([("POSTIT_ROOT", tmp)]);

        let env = env::var("POSTIT_ROOT").map_err(postit::Error::wrap)?;
        let path = PathBuf::from(env);

        if !path.exists() {
            fs::create_dir_all(path)?;
        }

        Ok(Self {
            instance: Orm::get_persister(conn)?,
            _env: mock_env,
        })
    }

    pub fn conn(&self) -> String {
        self.instance.conn()
    }

    pub fn create(protocol: Protocol) -> postit::Result<Self> {
        let mock = match protocol {
            Protocol::Sqlite => Self::sqlite(),
            Protocol::Mongo | Protocol::MongoSrv => Self::mongo(),
        }?;

        mock.instance.create()?;

        Ok(mock)
    }

    pub fn sqlite() -> postit::Result<Self> {
        Self::new("test_tasks.db")
    }

    pub fn mongo() -> postit::Result<Self> {
        Self::new("mongodb://localhost:27017")
    }
}

#[cfg(any(feature = "mongo", feature = "sqlite"))]
impl Drop for MockConn {
    fn drop(&mut self) {
        if Orm::is_sqlite(&self.instance.conn()) {
            self.instance.drop_database().unwrap();
        } else {
            self.instance.drop_table().unwrap();
        }
    }
}

/// The temporary representation of the [Config][`Config`] file.
///
/// Implements the `Display` and `Drop` traits
/// to delete the temporary path when the test ends.
pub struct MockConfig {
    pub path: PathBuf,
    pub config: Config,
    _env: MockEnvVar,
}

impl MockConfig {
    /// Constructor of the `MockConfig` struct.
    pub fn new() -> postit::Result<Self> {
        let tmp = env::current_dir()?.join("tmp");
        let mock_env = MockEnvVar::new().set([("POSTIT_ROOT", tmp)]);

        Config::init()?;

        let path = Config::path()?;

        Ok(Self {
            path,
            config: Config::load()?,
            _env: mock_env,
        })
    }

    pub fn save(&mut self) -> postit::Result<()> {
        let mut file = fs::File::create(self.path())?;
        let toml = toml::to_string_pretty(&self.config).map_err(postit::Error::wrap)?;

        file.write_all(toml.as_bytes())?;

        Ok(())
    }

    pub fn home() -> postit::Result<String> {
        env::var("HOME").map_err(postit::Error::wrap)
    }

    pub fn path(&self) -> PathBuf {
        self.path.clone()
    }
}

impl fmt::Display for MockConfig {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.path.to_str().unwrap())
    }
}

impl Drop for MockConfig {
    fn drop(&mut self) {
        if let Err(err) = fs::remove_dir_all(self.path.parent().unwrap()) {
            eprintln!("Failed to delete MockConfig directory ({}): {}", &self.path.display(), err);
        }
    }
}
