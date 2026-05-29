use std::ops::Not as _;

use postit::config::Config;
use postit::db::{Mongo, Orm, Protocol};
use postit::models::{Task, Todo};
use postit::traits::{DbPersister as _, Persister as _};
use postit::Action;

use crate::mocks::{MockConfig, MockConn};

#[test]
fn error_wrap() {
    let msg = "Error";
    let err = postit::db::Error::wrap(msg);

    assert!(matches!(err, postit::db::Error::Other(_)));
}

#[test]
fn protocol_from() {
    assert_eq!(Protocol::from("file"), Protocol::Sqlite);
    assert_eq!(Protocol::from("sqlite"), Protocol::Sqlite);
    assert_eq!(Protocol::from("mongodb"), Protocol::Mongo);
    assert_eq!(Protocol::from("mongodb+srv"), Protocol::MongoSrv);
}

#[test]
fn protocol_to_str() {
    assert_eq!(Protocol::Sqlite.to_str(), "sqlite");
    assert_eq!(Protocol::Mongo.to_str(), "mongo");
    assert_eq!(Protocol::MongoSrv.to_str(), "mongo+srv");
}

#[test]
fn display() {
    assert_eq!(Protocol::Sqlite.to_string(), "sqlite");
    assert_eq!(Protocol::Mongo.to_string(), "mongo");
    assert_eq!(Protocol::MongoSrv.to_string(), "mongo+srv");
}

#[test]
fn orm_fmt_debug() -> postit::Result<()> {
    let mock = MockConn::create(Protocol::Sqlite)?;

    let persister = Orm::get_persister(mock.conn())?;
    let orm = Orm::new(persister);

    let debug_output = format!("{orm:?}");
    let expected_output = format!("Orm {{ db: {:?} }}", mock.conn());

    assert_eq!(debug_output, expected_output);

    Ok(())
}

#[test]
fn is_sqlite() {
    assert!(Orm::is_sqlite(":memory:"));
    assert!(Orm::is_sqlite("sqlite:///tasks.db"));
    assert!(Orm::is_sqlite("test.db"));
    assert!(Orm::is_sqlite("test.sqlite"));
    assert!(Orm::is_sqlite("test.sqlite3"));
    assert!(Orm::is_sqlite("test.sqlite3"));
    assert!(Orm::is_sqlite("test.csv").not());
}

#[test]
fn get_persister() -> postit::Result<()> {
    let mock = MockConn::create(Protocol::Sqlite)?;
    let persister = Orm::get_persister(mock.conn())?;

    assert_eq!(persister.conn(), mock.conn());

    Ok(())
}

#[test]
fn get_persister_empty() {
    let err = Orm::get_persister("").unwrap_err();
    assert!(matches!(err, postit::Error::Db(postit::db::Error::IncorrectConnectionString)));
}

#[test]
fn get_persister_unsupported() -> postit::Result<()> {
    let conn = "tasks.db";

    let _config = MockConfig::new()?;
    let _mock = MockConn::new(conn);

    let result = Orm::get_persister("http://localhost:27017")?;
    let persister = Orm::get_persister(conn)?;

    assert_eq!(result.conn(), persister.conn());

    Ok(())
}

#[test]
fn get_persister_sqlite_protocol() -> postit::Result<()> {
    let conn = "sqlite:///tasks.db";

    let _mock = MockConn::new(conn);
    let persister = Orm::get_persister(conn)?;

    let path = Config::build_path(conn.replace("sqlite:///", ""))?;
    let conn_str = path.to_str().unwrap();

    assert_eq!(persister.conn(), conn_str);

    Ok(())
}

#[test]
fn to_string() -> postit::Result<()> {
    let mock = MockConn::create(Protocol::Sqlite)?;
    let orm = Orm::from(mock.conn())?;

    assert_eq!(orm.to_string(), mock.conn());

    Ok(())
}

#[test]
fn create() -> postit::Result<()> {
    let mock = MockConn::create(Protocol::Sqlite)?;
    let orm = Orm::from(mock.conn())?;

    assert!(orm.create().is_ok());
    assert!(mock.instance.exists().is_ok());

    Ok(())
}

#[test]
fn exists() -> postit::Result<()> {
    let mock = MockConn::create(Protocol::Sqlite)?;
    let orm = Orm::from(mock.conn())?;

    assert!(orm.exists().is_ok_and(|bool| bool));

    Ok(())
}

#[test]
fn view_ok() -> postit::Result<()> {
    let mock = MockConn::create(Protocol::Sqlite)?;
    let orm = Orm::from(mock.conn())?;

    orm.save(&Todo::sample())?;

    assert!(orm.view().is_ok());

    Ok(())
}

#[test]
fn view_err() -> postit::Result<()> {
    let mock = MockConn::create(Protocol::Sqlite)?;
    let orm = Orm::from(mock.conn())?;

    assert!(orm.view().is_err());

    Ok(())
}

#[test]
fn save_twice() -> postit::Result<()> {
    let mock = MockConn::create(Protocol::Sqlite)?;
    let mut todo = Todo::sample();
    let task = Task::from("5,task,med,false");

    let orm = Orm::from(mock.conn())?;

    orm.save(&todo)?;
    todo.add(task);
    orm.save(&todo)?;

    let result = orm.tasks()?;
    let expect = todo.tasks;

    assert_eq!(result, expect);

    Ok(())
}

#[test]
fn save_and_tasks() -> postit::Result<()> {
    let mock = MockConn::create(Protocol::Sqlite)?;
    let todo = Todo::sample();

    let orm = Orm::from(mock.conn())?;

    orm.save(&todo)?;

    let result = orm.tasks()?;
    let expect = todo.tasks;

    assert_eq!(result, expect);

    Ok(())
}

#[test]
fn edit_check() -> postit::Result<()> {
    let mock = MockConn::create(Protocol::Sqlite)?;
    let mut todo = Todo::sample();

    let orm = Orm::from(mock.conn())?;
    let ids = vec![2, 3];

    assert!(orm.save(&todo).is_ok());
    assert!(orm.edit(&todo, &ids, &Action::Check).is_ok());

    todo.check(&ids)?;

    let result = orm.tasks()?;
    let expect = todo.tasks;

    assert_eq!(result, expect);

    Ok(())
}

#[test]
fn edit_uncheck() -> postit::Result<()> {
    let mock = MockConn::create(Protocol::Sqlite)?;
    let mut todo = Todo::sample();

    let orm = Orm::from(mock.conn())?;
    let ids = vec![2, 3];

    assert!(orm.save(&todo).is_ok());
    assert!(orm.edit(&todo, &ids, &Action::Uncheck).is_ok());

    todo.uncheck(&ids)?;

    let result = orm.tasks()?;

    assert_eq!(result, todo.tasks);

    Ok(())
}

#[test]
fn edit_drop() -> postit::Result<()> {
    let mock = MockConn::create(Protocol::Sqlite)?;
    let mut todo = Todo::sample();

    let orm = Orm::from(mock.conn())?;
    let ids = vec![2, 3];

    assert!(orm.save(&todo).is_ok());
    assert!(orm.edit(&todo, &ids, &Action::Drop).is_ok());

    todo.check(&ids)?;
    todo.drop(&ids)?;

    let result = orm.tasks()?;

    assert_eq!(result, todo.tasks);

    Ok(())
}

#[test]
fn tasks() -> postit::Result<()> {
    let mock = MockConn::create(Protocol::Sqlite)?;
    let orm = Orm::from(mock.conn())?;
    let todo = Todo::sample();

    assert!(orm.save(&todo).is_ok());

    let result = orm.tasks()?;
    let expect = todo.tasks;

    assert_eq!(result, expect);

    Ok(())
}

#[test]
fn replace() -> postit::Result<()> {
    let mock = MockConn::create(Protocol::Sqlite)?;
    let mut todo = Todo::sample();
    todo.add(Task::from("5,test,med,false"));

    let orm = Orm::from(mock.conn())?;

    assert!(orm.replace(&todo).is_ok());

    let result = orm.tasks()?;
    let expect = todo.tasks;

    assert_eq!(result, expect);

    Ok(())
}

#[test]
fn clean_empty() -> postit::Result<()> {
    let mock = MockConn::create(Protocol::Sqlite)?;
    let orm = Orm::from(mock.conn())?;

    assert!(orm.clean().is_ok());

    Ok(())
}

#[test]
fn clean_not_empty() -> postit::Result<()> {
    let mock = MockConn::create(Protocol::Sqlite)?;
    let orm = Orm::from(mock.conn())?;
    orm.save(&Todo::sample())?;

    assert!(orm.clean().is_ok());

    let result = orm.tasks()?;
    let expect = Vec::new();

    assert_eq!(result, expect);

    Ok(())
}

#[test]
fn remove() -> postit::Result<()> {
    let mongo = Mongo::from("mongodb://localhost:27017")?;
    mongo.create()?;

    let orm = Orm::from(mongo.conn())?;

    assert!(orm.remove().is_ok());
    assert!(mongo.exists().is_ok_and(|bool| !bool));

    Ok(())
}
