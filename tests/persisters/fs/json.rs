use std::fs;
use std::io::Read as _;
use std::ops::Not as _;

use postit::fs::{Format, Json};
use postit::models::Todo;
use postit::traits::FilePersister as _;

use crate::mocks::MockPath;

#[test]
fn tasks() -> postit::Result<()> {
    let mock = MockPath::create(Format::Json)?;

    let result = Json::new(mock.path()).tasks()?;
    let expect = Todo::sample().tasks;

    assert_eq!(result, expect);

    Ok(())
}

#[test]
fn open_ok() -> postit::Result<()> {
    let mock = MockPath::create(Format::Json)?;

    let mut json = Json::new(mock.path()).open()?;
    let mut file = fs::File::open(mock.path())?;

    let mut result = Vec::new();
    let mut expect = Vec::new();

    json.read_to_end(&mut result)?;
    file.read_to_end(&mut expect)?;

    assert_eq!(result, expect);

    Ok(())
}

#[test]
fn open_err() {
    let err = Json::new("tmp/fake.json").open().unwrap_err();
    assert!(matches!(err, postit::fs::Error::Io(_)));
}

#[test]
fn clean() -> postit::Result<()> {
    let mock = MockPath::create(Format::Json)?;
    Json::new(mock.path()).clean()?;

    let result = Json::new(mock.path()).tasks()?;
    let expect = Vec::new();

    assert_eq!(result, expect);

    Ok(())
}

#[test]
fn remove() -> postit::Result<()> {
    let mock = MockPath::create(Format::Json)?;
    Json::new(mock.path()).remove()?;

    assert!(mock.path().exists().not());

    Ok(())
}
