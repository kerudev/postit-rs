//! Utilities to handle `SQLite` files.
//!
//! The `Sqlite` struct implements the [`DbPersister`] trait.

use std::path::Path;
use std::{fmt, fs};

use sqlite::{Connection, State, Statement};

use crate::config::Config;
use crate::models::{Task, Todo};
use crate::traits::DbPersister;
use crate::Action;

/// Representation of a `SQLite` database.
pub struct Sqlite {
    /// Connection string used to connect to the `SQLite` file.
    conn_str: String,
    /// Connection to the `SQLite` file.
    connection: Connection,
}

impl fmt::Debug for Sqlite {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Sqlite")
            .field("conn_str", &self.conn_str)
            .field("connection", &"[connection omitted]") // evitamos el campo problemático
            .finish()
    }
}

impl Clone for Sqlite {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            conn_str: self.conn_str.clone(),
            connection: sqlite::open(&self.conn_str).unwrap(),
        }
    }
}

impl Sqlite {
    /// Creates a `Sqlite` instance from a connection string.
    ///
    /// # Errors
    /// - If the path of the file can't be constructed from the Config path.
    /// - If a connection to the `SQLite` file can't be opened.
    ///
    /// # Panics
    /// - The path can't be converted to &str.
    #[inline]
    pub fn from<T: AsRef<Path>>(conn: T) -> crate::Result<Self> {
        let path = Config::build_path(conn.as_ref())?;

        if !path.exists() {
            fs::create_dir_all(path.parent().unwrap())?;
        }

        let instance = Self {
            conn_str: path.to_string_lossy().into_owned(),
            connection: sqlite::open(path).map_err(super::Error::Sqlite)?,
        };

        Ok(instance)
    }

    /// Returns the desired ids format to be used in a query.
    #[inline]
    pub fn format_ids(&self, ids: &[u32]) -> String {
        ids.iter()
            .map(|&n| n.to_string())
            .collect::<Vec<String>>()
            .join(", ")
    }

    /// Reads one row from the current statement.
    ///
    /// # Errors
    /// - A value can't be read.
    #[inline]
    pub fn read_row(&self, stmt: &Statement) -> super::Result<String> {
        let row = format!(
            "{},{},{},{}",
            stmt.read::<i64, _>("id")?,
            stmt.read::<String, _>("content")?,
            stmt.read::<String, _>("priority")?,
            stmt.read::<String, _>("checked")?,
        );

        Ok(row)
    }

    /// Resets the autoincrement value.
    ///
    /// # Errors
    /// - The statement can't be evaluated.
    #[inline]
    pub fn reset_autoincrement(&self, table: &str) -> sqlite::Result<State> {
        #[rustfmt::skip]
        let query = format!("
            UPDATE sqlite_sequence
            SET SEQ=0
            WHERE NAME='{table}'
        ");

        self.connection.prepare(query)?.next()
    }
}

impl DbPersister for Sqlite {
    #[inline]
    fn boxed(self) -> Box<dyn DbPersister> {
        Box::new(self)
    }

    #[inline]
    fn conn(&self) -> String {
        self.conn_str.clone()
    }

    #[inline]
    fn table(&self) -> String {
        String::from("tasks")
    }

    #[inline]
    fn database(&self) -> String {
        Path::new(&self.conn_str)
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_owned()
    }

    /// Checks if a table exists.
    ///
    /// # Errors
    /// - The statement can't be prepared.
    /// - The name column can't be read.
    #[inline]
    fn exists(&self) -> super::Result<bool> {
        #[rustfmt::skip]
        let query = format!("
            SELECT *
            FROM sqlite_master
            WHERE type='table'
              AND name='{}'
        ", self.table());

        let mut stmt = self.connection.prepare(query)?;

        let mut result = vec![];

        while matches!(stmt.next(), Ok(State::Row)) {
            result.push(stmt.read::<String, _>("name")?);
        }

        Ok(!result.is_empty())
    }

    #[inline]
    fn tasks(&self) -> super::Result<Vec<Task>> {
        if !self.exists()? {
            let err = format!(
                "The '{}' table has no tasks; add a task first to use this command",
                self.table()
            );
            return Err(super::Error::wrap(err));
        }

        let query = format!("SELECT * FROM {}", self.table());
        let mut stmt = self.connection.prepare(query)?;

        let mut result = vec![];

        while matches!(stmt.next(), Ok(State::Row)) {
            result.push(Task::from(self.read_row(&stmt)?));
        }

        Ok(result)
    }

    #[inline]
    fn count(&self) -> super::Result<u32> {
        if !self.exists()? {
            return Ok(0);
        }

        let query = format!("SELECT COUNT(*) AS count FROM {}", self.table());

        let mut stmt = self.connection.prepare(query)?;
        stmt.next()?;

        let n = stmt.read::<i64, _>("count")?.try_into().unwrap_or(0);

        Ok(n)
    }

    #[inline]
    fn create(&self) -> super::Result<()> {
        #[rustfmt::skip]
        let query = format!("
            CREATE TABLE IF NOT EXISTS {} (
                id          INTEGER PRIMARY KEY AUTOINCREMENT,
                content     TEXT NOT NULL,
                priority    TEXT NOT NULL,
                checked     BOOLEAN NOT NULL CHECK (checked IN (0, 1))
            )
        ", self.table());

        self.connection.execute(query)?;

        println!("Created the '{}' table in the '{}' database", self.table(), self.database());

        Ok(())
    }

    #[inline]
    fn insert(&self, todo: &Todo) -> super::Result<()> {
        #[rustfmt::skip]
        let query = format!("
            INSERT INTO {} (content, priority, checked)
            VALUES (?, ?, ?)
        ", self.table());

        let mut stmt = self.connection.prepare(query)?;

        for task in &todo.tasks {
            stmt.reset()?;

            #[rustfmt::skip]
            stmt.bind(&[
                &task.content,
                task.priority.to_str(),
                i32::from(task.checked).to_string().as_str()
            ][..])?;

            stmt.next()?;
        }

        Ok(())
    }

    #[inline]
    fn update(&self, todo: &Todo, ids: &[u32], action: &Action) -> super::Result<()> {
        if matches!(action, Action::Drop) {
            return self.delete(ids);
        }

        let (field, value) = match action {
            Action::Check => ("checked", "1"),
            Action::Uncheck => ("checked", "0"),
            Action::SetContent => ("content", todo.get(ids)[0].content.as_str()),
            Action::SetPriority => ("priority", todo.get(ids)[0].priority.to_str()),
            Action::Drop => unreachable!(),
        };

        #[rustfmt::skip]
        let query = format!("
            UPDATE {}
            SET {field} = \"{value}\"
            WHERE id
            IN ({})
        ", self.table(), self.format_ids(ids));

        let mut stmt = self.connection.prepare(query)?;

        stmt.next()?;

        Ok(())
    }

    #[inline]
    fn delete(&self, ids: &[u32]) -> super::Result<()> {
        #[rustfmt::skip]
        let query = format!("
            DELETE FROM {}
            WHERE id
            IN ({})
        ", self.table(), self.format_ids(ids));

        let mut stmt = self.connection.prepare(query)?;

        stmt.next()?;

        Ok(())
    }

    #[inline]
    fn drop_table(&self) -> super::Result<()> {
        let table = self.table();
        let query = format!("DROP TABLE {table}");

        let mut stmt = self.connection.prepare(query)?;

        stmt.next()?;

        Ok(())
    }

    #[inline]
    fn drop_database(&self) -> super::Result<()> {
        fs::remove_file(self.conn()).map_err(super::Error::wrap)?;

        println!("Removed the '{}' file", self.database());

        Ok(())
    }

    #[inline]
    fn clean(&self) -> super::Result<()> {
        let table = self.table();
        let query = format!("DELETE FROM {table}");

        let mut stmt = self.connection.prepare(query)?;
        stmt.next()?;

        self.reset_autoincrement(&table)?;

        Ok(())
    }
}
