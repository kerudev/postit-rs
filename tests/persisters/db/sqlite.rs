use std::ops::Not as _;
use std::path::PathBuf;

use postit::config::Config;
use postit::db::{Protocol, Sqlite};
use postit::models::Todo;
use postit::traits::DbPersister as _;
use postit::Action;

use crate::mocks::MockConn;

#[test]
fn fmt_debug() -> postit::Result<()> {
    let mock = MockConn::create(Protocol::Sqlite)?;
    let sqlite = Sqlite::from(mock.conn())?;

    let result = format!("{sqlite:?}");
    let expect =
        format!("Sqlite {{ conn_str: {:?}, connection: \"[connection omitted]\" }}", sqlite.conn());

    assert_eq!(result, expect);

    Ok(())
}

#[test]
fn clone() -> postit::Result<()> {
    let mock = MockConn::create(Protocol::Sqlite)?;

    let expect = Sqlite::from(mock.conn())?;
    let result = expect.clone();

    assert_eq!(result.conn(), expect.conn());

    Ok(())
}

#[test]
fn count_ok() -> postit::Result<()> {
    let mock = MockConn::create(Protocol::Sqlite)?;
    mock.instance.insert(&Todo::sample())?;

    assert_eq!(Sqlite::from(mock.conn())?.count()?, 4);

    Ok(())
}

#[test]
fn count_table_doesnt_exist() -> postit::Result<()> {
    let mock = MockConn::create(Protocol::Sqlite)?;
    mock.instance.drop_table()?;

    let path = PathBuf::from(mock.conn());
    let file = path.file_name().unwrap();

    assert_eq!(Sqlite::from(file)?.count()?, 0);

    Ok(())
}

#[test]
fn exists() -> postit::Result<()> {
    let mock = MockConn::create(Protocol::Sqlite)?;
    let sqlite = Sqlite::from(mock.conn())?;

    assert!(sqlite.exists().is_ok_and(|bool| bool));

    Ok(())
}

#[test]
fn format_ids() -> postit::Result<()> {
    let mock = MockConn::create(Protocol::Sqlite)?;

    let ids = vec![1, 2, 3];

    let result = Sqlite::from(mock.conn())?.format_ids(&ids);
    let expect = "1, 2, 3";

    assert_eq!(result, expect);

    Ok(())
}

#[test]
fn conn() -> postit::Result<()> {
    let conn = "test.db";
    let mock = MockConn::new(conn)?;

    let path = Config::build_path(conn)?;
    let conn_str = path.to_str().unwrap();

    assert_eq!(conn_str, mock.conn());

    Ok(())
}

#[test]
fn boxed() -> postit::Result<()> {
    let conn = "test.db";
    let mock = MockConn::new(conn)?;

    let sqlite = Sqlite::from(mock.conn())?;
    let result = sqlite.clone().boxed();

    assert_eq!(result.conn(), sqlite.conn());

    Ok(())
}

#[test]
fn reset_autoincrement() -> postit::Result<()> {
    let mock = MockConn::create(Protocol::Sqlite)?;
    let todo = Todo::sample();
    let task = Todo::new(&todo.tasks[0]);

    let sqlite = Sqlite::from(mock.conn())?;

    sqlite.insert(&todo)?;
    sqlite.clean()?;
    sqlite.insert(&task)?;

    let result = sqlite.tasks()?[0].id;
    let expect = 1;

    assert_eq!(result, expect);

    Ok(())
}

#[test]
fn create() -> postit::Result<()> {
    let mock = MockConn::create(Protocol::Sqlite)?;
    mock.instance.create()?;

    let sqlite = Sqlite::from(mock.conn())?;

    assert!(sqlite.exists().is_ok_and(|bool| bool));

    Ok(())
}

#[test]
fn insert_and_tasks() -> postit::Result<()> {
    let todo = Todo::sample();

    let mock = MockConn::create(Protocol::Sqlite)?;
    mock.instance.insert(&todo)?;

    let result = mock.instance.tasks()?;

    assert_eq!(result, todo.tasks);

    Ok(())
}

#[test]
fn update_check() -> postit::Result<()> {
    let mut todo = Todo::sample();
    let ids = vec![2, 3];

    let mock = MockConn::create(Protocol::Sqlite)?;
    mock.instance.insert(&todo)?;
    mock.instance.update(&todo, &ids, &Action::Check)?;

    todo.check(&ids)?;

    let result = mock.instance.tasks()?;

    assert_eq!(result, todo.tasks);

    Ok(())
}

#[test]
fn update_uncheck() -> postit::Result<()> {
    let mut todo = Todo::sample();
    let ids = vec![2, 3];

    let mock = MockConn::create(Protocol::Sqlite)?;
    mock.instance.insert(&todo)?;
    mock.instance.update(&todo, &ids, &Action::Uncheck)?;

    todo.uncheck(&ids)?;

    let result = mock.instance.tasks()?;

    assert_eq!(result, todo.tasks);

    Ok(())
}

#[test]
fn update_set_content() -> postit::Result<()> {
    let ids = vec![2, 3];

    let mut todo = Todo::sample();
    todo.set_content(&ids, "test")?;

    let mock = MockConn::create(Protocol::Sqlite)?;
    mock.instance.insert(&todo)?;
    mock.instance.update(&todo, &ids, &Action::SetContent)?;

    let result = mock.instance.tasks()?;

    assert_eq!(result, todo.tasks);

    Ok(())
}

#[test]
fn update_set_priority() -> postit::Result<()> {
    let ids = vec![2, 3];

    let mut todo = Todo::sample();
    todo.set_priority(&ids, &postit::models::Priority::High)?;

    let mock = MockConn::create(Protocol::Sqlite)?;
    mock.instance.insert(&todo)?;
    mock.instance.update(&todo, &ids, &Action::SetPriority)?;

    let result = mock.instance.tasks()?;

    assert_eq!(result, todo.tasks);

    Ok(())
}

#[test]
fn update_delete() -> postit::Result<()> {
    let mut todo = Todo::sample();
    let ids = vec![2, 3];

    let mock = MockConn::create(Protocol::Sqlite)?;
    mock.instance.insert(&todo)?;
    mock.instance.update(&todo, &ids, &Action::Drop)?;

    todo.check(&ids)?;
    todo.drop(&ids)?;

    let result = mock.instance.tasks()?;

    assert_eq!(result, todo.tasks);

    Ok(())
}

#[test]
fn drop_table() -> postit::Result<()> {
    let mock = MockConn::create(Protocol::Sqlite)?;
    mock.instance.drop_table()?;

    assert!(mock.instance.exists().is_ok_and(|bool| !bool));

    Ok(())
}

#[test]
fn drop_database() -> postit::Result<()> {
    // Doesn't use mocks because of conflicts with the Drop trait.
    let sqlite = Sqlite::from("test_tasks.db")?;
    sqlite.drop_database()?;

    assert!(std::path::PathBuf::from(sqlite.conn()).exists().not());

    Ok(())
}

#[test]
fn tasks_ok() -> postit::Result<()> {
    let mock = MockConn::create(Protocol::Sqlite)?;
    let todo = Todo::sample();

    let sqlite = Sqlite::from(mock.conn())?;
    sqlite.insert(&todo)?;

    assert_eq!(todo.tasks, sqlite.tasks()?);

    Ok(())
}

#[test]
fn tasks_err() -> postit::Result<()> {
    let mock = MockConn::create(Protocol::Sqlite)?;
    mock.instance.drop_table()?;

    assert!(mock.instance.tasks().is_err());

    Ok(())
}

#[test]
fn clean() -> postit::Result<()> {
    let mock = MockConn::create(Protocol::Sqlite)?;
    let todo = Todo::sample();

    let sqlite = Sqlite::from(mock.conn())?;
    sqlite.insert(&todo)?;
    sqlite.clean()?;

    let result = sqlite.tasks()?;
    let expect = Vec::new();

    assert_eq!(result, expect);

    Ok(())
}
