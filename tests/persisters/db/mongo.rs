use postit::db::{Mongo, Protocol};
use postit::models::Todo;
use postit::traits::DbPersister as _;
use postit::Action;

use crate::mocks::MockConn;

#[test]
fn clone() -> postit::Result<()> {
    let mock = MockConn::create(Protocol::Mongo)?;

    let expect = Mongo::from(mock.conn())?;
    let result = expect.clone();

    assert_eq!(result.conn(), expect.conn());

    Ok(())
}

#[test]
fn count_ok() -> postit::Result<()> {
    let mock = MockConn::create(Protocol::Mongo)?;
    mock.instance.insert(&Todo::sample())?;

    assert_eq!(Mongo::from(mock.conn())?.count()?, 4);

    Ok(())
}

#[test]
fn count_table_doesnt_exist() -> postit::Result<()> {
    let mock = MockConn::create(Protocol::Mongo)?;
    mock.instance.drop_table()?;

    assert_eq!(Mongo::from(mock.conn())?.count()?, 0);

    Ok(())
}

#[test]
fn exists() -> postit::Result<()> {
    let mock = MockConn::create(Protocol::Mongo)?;
    let mongo = Mongo::from(mock.conn())?;

    assert!(mongo.exists().is_ok_and(|bool| bool));

    Ok(())
}

#[test]
fn conn() -> postit::Result<()> {
    let uri = "mongodb://localhost:27017";
    let mock = MockConn::new(uri)?;

    assert_eq!(uri, mock.conn());

    Ok(())
}

#[test]
fn boxed() -> postit::Result<()> {
    let uri = "mongodb://localhost:27017";

    let mock = MockConn::new(uri)?;
    let mongo = Mongo::from(mock.conn())?;
    let result = mongo.clone().boxed();

    assert_eq!(result.conn(), mongo.conn());

    Ok(())
}

#[test]
fn reset_autoincrement() -> postit::Result<()> {
    let todo = Todo::sample();
    let task = Todo::new(&todo.tasks[0]);

    let mock = MockConn::create(Protocol::Mongo)?;
    let mongo = Mongo::from(mock.conn())?;

    mongo.insert(&todo)?;
    mongo.clean()?;
    mongo.insert(&task)?;

    let result = mongo.tasks()?[0].id;
    let expect = 1;

    assert_eq!(result, expect);

    Ok(())
}

#[test]
fn create() -> postit::Result<()> {
    let mock = MockConn::create(Protocol::Mongo)?;
    mock.instance.create()?;

    let mongo = Mongo::from(mock.conn())?;

    assert!(mongo.exists().is_ok_and(|bool| bool));

    Ok(())
}

#[test]
fn insert_and_tasks() -> postit::Result<()> {
    let todo = Todo::sample();

    let mock = MockConn::create(Protocol::Mongo)?;
    mock.instance.insert(&todo)?;

    let result = mock.instance.tasks()?;

    assert_eq!(result, todo.tasks);

    Ok(())
}

#[test]
fn update_check() -> postit::Result<()> {
    let mut todo = Todo::sample();
    let ids = vec![2, 3];

    let mock = MockConn::create(Protocol::Mongo)?;
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

    let mock = MockConn::create(Protocol::Mongo)?;
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

    let mock = MockConn::create(Protocol::Mongo)?;
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

    let mock = MockConn::create(Protocol::Mongo)?;
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

    let mock = MockConn::create(Protocol::Mongo)?;
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
    // Doesn't use mocks because of conflicts with the Drop trait.
    let mongo = Mongo::from("mongodb://localhost:27017")?;

    assert!(mongo.drop_table().is_ok());
    assert!(mongo.exists().is_ok_and(|bool| !bool));

    Ok(())
}

#[test]
fn drop_database() -> postit::Result<()> {
    let mongo = Mongo::from("mongodb://localhost:27017")?;

    assert!(mongo.drop_database().is_ok());

    Ok(())
}

#[test]
fn tasks_ok() -> postit::Result<()> {
    let mock = MockConn::create(Protocol::Mongo)?;
    let todo = Todo::sample();

    let mongo = Mongo::from(mock.conn())?;
    mongo.insert(&todo)?;

    assert_eq!(todo.tasks, mongo.tasks()?);

    Ok(())
}

#[test]
fn tasks_err() -> postit::Result<()> {
    let mock = MockConn::create(Protocol::Mongo)?;
    mock.instance.drop_table()?;

    assert!(matches!(mock.instance.tasks().unwrap_err(), postit::db::Error::Other(_)));

    Ok(())
}

#[test]
fn clean() -> postit::Result<()> {
    let mock = MockConn::create(Protocol::Mongo)?;
    let todo = Todo::sample();

    let mongo = Mongo::from(mock.conn())?;
    mongo.insert(&todo)?;
    mongo.clean()?;

    let result = mongo.tasks()?;
    let expect = Vec::new();

    assert_eq!(result, expect);

    Ok(())
}
