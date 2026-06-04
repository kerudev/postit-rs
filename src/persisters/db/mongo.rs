//! Utilities to handle `Mongo` files.
//!
//! The `Mongo` struct implements the [`DbPersister`] trait.

use std::time::Duration;

use mongodb::bson::{doc, Bson, Document};
use mongodb::options::ClientOptions;
use mongodb::sync::{Client, Collection, Database};

use crate::models::{Task, Todo};
use crate::traits::DbPersister;
use crate::Action;

/// Representation of a `Mongo` database.
#[derive(Debug)]
pub struct Mongo {
    /// URI used to connect to the `Mongo` database.
    conn_str: String,
    /// Connection to the `Mongo` database.
    connection: Client,
}

impl Clone for Mongo {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            conn_str: self.conn_str.clone(),
            connection: self.connection.clone(),
        }
    }
}

impl Mongo {
    /// Creates a `Mongo` instance from a URI.
    ///
    /// # Errors
    /// - [`ClientOptions`] can't be parsed.
    /// - [`Client`] couldn't be opened.
    #[inline]
    pub fn from<T: AsRef<str>>(uri: T) -> super::Result<Self> {
        let uri = uri.as_ref();

        let mut options = ClientOptions::parse(uri).run()?;
        options.server_selection_timeout = Some(Duration::from_secs(5));
        options.connect_timeout = Some(Duration::from_secs(5));

        let instance = Self {
            conn_str: uri.to_owned(),
            connection: Client::with_options(options)?,
        };

        Ok(instance)
    }

    /// Gets a handle to a database specified by name in the cluster the Client is connected to.
    #[inline]
    pub fn db(&self) -> Database {
        self.connection.database(&self.database())
    }

    /// Gets a handle to a collection with type T specified by name of the database.
    #[inline]
    pub fn collection<T: Send + Sync>(&self) -> Collection<T> {
        self.db().collection::<T>(&self.table())
    }
}

impl DbPersister for Mongo {
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
        String::from("test")
    }

    #[inline]
    fn exists(&self) -> super::Result<bool> {
        let names = self.db().list_collection_names().run()?;

        Ok(names.contains(&self.table()))
    }

    #[inline]
    fn tasks(&self) -> super::Result<Vec<Task>> {
        if !self.exists()? {
            let err = format!(
                "The '{}' collection doesn't exist; add a task first to use this command",
                self.table()
            );
            return Err(super::Error::wrap(err));
        }

        let tasks = self
            .collection::<Task>()
            .find(doc! {})
            .run()?
            .map(|doc| doc.unwrap())
            .collect();

        Ok(tasks)
    }

    #[inline]
    fn count(&self) -> super::Result<u32> {
        if !self.exists()? {
            return Ok(0);
        }

        let n = self
            .collection::<u32>()
            .count_documents(doc! {})
            .run()?
            .try_into()
            .unwrap_or(0);

        Ok(n)
    }

    #[inline]
    fn create(&self) -> super::Result<()> {
        let table = self.table();

        self.db().create_collection(&table).run()?;

        println!("Created the '{table}' table in the '{}' collection", self.database());

        Ok(())
    }

    #[inline]
    fn insert(&self, todo: &Todo) -> super::Result<()> {
        let docs: Vec<Document> = todo
            .tasks
            .iter()
            .map(|task| {
                doc! {
                    "id": task.id,
                    "content": &task.content,
                    "priority": task.priority.to_str(),
                    "checked": task.checked,
                }
            })
            .collect();

        self.collection::<Document>().insert_many(&docs).run()?;

        Ok(())
    }

    #[inline]
    fn update(&self, todo: &Todo, ids: &[u32], action: &Action) -> super::Result<()> {
        if matches!(action, Action::Drop) {
            return self.delete(ids);
        }

        let (field, value) = match action {
            Action::Check => ("checked", Bson::Boolean(true)),
            Action::Uncheck => ("checked", Bson::Boolean(false)),
            Action::SetContent => ("content", Bson::String(todo.get(ids)[0].content.clone())),
            Action::SetPriority => {
                ("priority", Bson::String(todo.get(ids)[0].priority.to_string()))
            }
            Action::Drop => unreachable!(),
        };

        let query = doc! { "id": { "$in": ids } };
        let update = doc! { "$set": { field: value } };

        self.collection::<Document>()
            .update_many(query, update)
            .run()?;

        Ok(())
    }

    #[inline]
    fn delete(&self, ids: &[u32]) -> super::Result<()> {
        let query = doc! { "id": {"$in": ids }};

        self.collection::<String>().delete_many(query).run()?;

        Ok(())
    }

    #[inline]
    fn drop_table(&self) -> super::Result<()> {
        self.collection::<Task>().drop().run()?;

        // println!("Removed the '{}' collection", self.table());

        Ok(())
    }

    #[inline]
    fn drop_database(&self) -> super::Result<()> {
        self.db().drop().run()?;

        println!("Removed the '{}' database", self.database());

        Ok(())
    }

    #[inline]
    fn clean(&self) -> super::Result<()> {
        self.collection::<String>().delete_many(doc! {}).run()?;

        Ok(())
    }
}
