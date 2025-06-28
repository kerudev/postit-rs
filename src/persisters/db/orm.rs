//! Contains structures related to a ORM's operations:
//! - mod [`error`]: error handling for database related problems.
//! - enum [`Protocol`]: used to distinguish different database protocols.
//! - struct [`Orm`]: manages database connections and their operations.

use std::fmt;
use std::path::Path;

#[cfg(feature = "mongo")]
use super::Mongo;
#[cfg(feature = "sqlite")]
use super::Sqlite;
use crate::db;
use crate::models::{Task, Todo};
use crate::traits::{DbPersister, Persister};
use crate::Action;

/// A database protocol.
#[derive(Debug, PartialEq, Eq)]
pub enum Protocol {
    /// An `SQLite` database (associated persister: [`Sqlite`]).
    Sqlite,
    /// A `MongoDB` database (associated persister: [`Mongo`]).
    #[cfg(feature = "mongo")]
    Mongo,
    /// A `MongoDB` database on a remote server (associated persister: [`Mongo`]).
    #[cfg(feature = "mongo")]
    MongoSrv,
}

impl<T: AsRef<str>> From<T> for Protocol {
    /// Transforms a string slice into a `Protocol` variant.
    #[inline]
    fn from(s: T) -> Self {
        match s.as_ref().to_lowercase().trim() {
            "sqlite" => Self::Sqlite,
            "mongodb" => Self::Mongo,
            "mongodb+srv" => Self::MongoSrv,
            _ => {
                eprintln!("{}", db::Error::UnsupportedDatabase);
                Self::Sqlite
            }
        }
    }
}

impl Protocol {
    /// Returns the `Protocol` value as its string representation.
    #[inline]
    pub const fn to_str(&self) -> &str {
        match *self {
            Self::Sqlite => "sqlite",
            Self::Mongo => "mongo",
            Self::MongoSrv => "mongo+srv",
        }
    }
}

impl fmt::Display for Protocol {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::Sqlite => write!(f, "sqlite"),
            Self::Mongo => write!(f, "mongo"),
            Self::MongoSrv => write!(f, "mongo+srv"),
        }
    }
}

/// Abstraction of database actions, used to manage a [`Todo`] structure.
pub struct Orm {
    /// Database that implements the [`DbPersister`] trait.
    db: Box<dyn DbPersister>,
}

impl fmt::Debug for Orm {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Orm")
            .field("db", &self.to_string())
            .finish()
    }
}

impl Orm {
    /// Constructor of the `Orm` struct, which controls instances of structs
    /// that implement the [`DbPersister`] trait.
    #[inline]
    pub const fn new(db: Box<dyn DbPersister>) -> Self {
        Self { db }
    }

    /// Creates a `Orm` instance from a connection string.
    ///
    /// # Errors
    /// - The database persister can't be obtained.
    #[inline]
    pub fn from<T: AsRef<str>>(conn: T) -> crate::Result<Self> {
        Ok(Self { db: Self::get_persister(conn)? })
    }

    /// Checks if the passed connection string has an Sqlite format.
    ///
    /// # Panics
    /// - The extension can't be converted to `&str`.
    #[inline]
    pub fn is_sqlite(conn: &str) -> bool {
        conn.eq(":memory:")
            || conn.starts_with("sqlite:///")
            || Path::new(conn)
                .extension()
                .is_some_and(|ext| matches!(ext.to_str().unwrap(), "db" | "sqlite3" | "sqlite"))
    }

    /// Returns a struct that implements the [`DbPersister`] trait based on
    /// a connection string.
    ///
    /// # Errors
    /// - If the persister can't be obtained.
    /// - If the connection string is empty.
    #[inline]
    pub fn get_persister<T: AsRef<str>>(conn: T) -> crate::Result<Box<dyn DbPersister>> {
        let conn = conn.as_ref();

        #[cfg(feature = "sqlite")]
        if Self::is_sqlite(conn) {
            return Ok(Sqlite::from(conn.replace("sqlite:///", ""))?.boxed());
        }

        let parts: Vec<&str> = conn.split("://").collect();

        if parts[0].is_empty() {
            return Err(crate::Error::Db(db::Error::IncorrectConnectionString));
        }

        let protocol = parts[0];

        match Protocol::from(protocol) {
            #[cfg(feature = "mongo")]
            Protocol::Mongo | Protocol::MongoSrv => Ok(Mongo::from(conn)?.boxed()),

            #[cfg(feature = "sqlite")]
            Protocol::Sqlite => Ok(Sqlite::from("tasks.db")?.boxed()),
        }
    }
}

impl Persister for Orm {
    #[inline]
    fn boxed(self) -> Box<dyn Persister> {
        Box::new(self)
    }

    #[inline]
    fn to_string(&self) -> String {
        self.db.conn()
    }

    #[inline]
    fn create(&self) -> crate::Result<()> {
        self.db.create().map_err(|e| {
            eprintln!("Can't create the table");
            crate::Error::Db(e)
        })
    }

    #[inline]
    fn exists(&self) -> crate::Result<bool> {
        self.db.exists().map_err(|e| {
            eprintln!("The table doesn't exist; add a task first to use this command");
            crate::Error::Db(e)
        })
    }

    #[inline]
    fn view(&self) -> crate::Result<()> {
        Todo::new(self.tasks()?).view()?;

        Ok(())
    }

    #[inline]
    fn tasks(&self) -> crate::Result<Vec<Task>> {
        self.db.tasks().map_err(crate::Error::Db)
    }

    #[inline]
    fn edit(&self, todo: &Todo, ids: &[u32], action: &Action) -> crate::Result<()> {
        self.db.update(todo, ids, action).map_err(|e| {
            eprintln!("Can't perform the '{action}' action");
            crate::Error::Db(e)
        })
    }

    #[inline]
    fn save(&self, todo: &Todo) -> crate::Result<()> {
        if self.db.count()? == 0 {
            return self.db.insert(todo).map_err(|e| {
                eprintln!("Can't insert into the table");
                crate::Error::Db(e)
            });
        }

        let last = todo.tasks.last().unwrap().clone();
        let task = Todo::new(last);

        self.db.insert(&task).map_err(|e| {
            eprintln!("Can't insert into the table");
            crate::Error::Db(e)
        })
    }

    #[inline]
    fn replace(&self, todo: &Todo) -> crate::Result<()> {
        if self.exists()? {
            self.db.clean()?;
        }

        self.db.insert(todo).map_err(|e| {
            eprintln!("Can't insert into the table");
            crate::Error::Db(e)
        })?;

        println!("Replaced the tasks of '{}'", self.db.conn());

        Ok(())
    }

    #[inline]
    fn clean(&self) -> crate::Result<()> {
        if self.tasks()?.is_empty() {
            eprintln!("There are no tasks to delete in the table");
            return Ok(());
        }

        self.db.clean().map_err(|e| {
            eprintln!("Can't clean the table");
            crate::Error::Db(e)
        })?;

        println!("Cleaned the tasks from the '{}' table", self.db.table());

        Ok(())
    }

    #[inline]
    fn remove(&self) -> crate::Result<()> {
        let table = self.db.table();

        if !self.exists()? {
            eprintln!("There is no '{table}' table to remove at '{}'", self.to_string());
            return Ok(());
        }

        self.db.drop_table().map_err(|e| {
            eprintln!("Can't drop the table");
            crate::Error::Db(e)
        })?;

        print!("Removed the '{}' table from '{}'", self.db.table(), self.db.conn());

        Ok(())
    }
}
