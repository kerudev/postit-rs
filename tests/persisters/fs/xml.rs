use std::fs;
use std::io::Read as _;
use std::ops::Not as _;

use postit::fs::{Format, Xml};
use postit::models::Todo;
use postit::traits::FilePersister as _;

use crate::mocks::MockPath;

#[test]
fn default() -> postit::Result<()> {
    let mock = MockPath::create(Format::Xml)?;

    let result = Xml::new(mock.path()).default();
    let expect = Xml::prolog() + &Xml::dtd();

    assert_eq!(result, expect);

    Ok(())
}

#[test]
fn tasks() -> postit::Result<()> {
    let mock = MockPath::create(Format::Xml)?;

    let result = Xml::new(mock.path()).tasks()?;
    let expect = Todo::sample().tasks;

    assert_eq!(result, expect);

    Ok(())
}

#[test]
fn open_ok() -> postit::Result<()> {
    let mock = MockPath::create(Format::Xml)?;

    let mut json = Xml::new(mock.path()).open()?;
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
    let err = Xml::new("tmp/fake.xml").open().unwrap_err();
    assert!(matches!(err, postit::fs::Error::Io(_)));
}

#[test]
fn clean() -> postit::Result<()> {
    let mock = MockPath::create(Format::Xml)?;
    Xml::new(mock.path()).clean()?;

    let result = Xml::new(mock.path()).tasks()?;
    let expect = Vec::new();

    assert_eq!(result, expect);

    Ok(())
}

#[test]
fn remove() -> postit::Result<()> {
    let mock = MockPath::create(Format::Xml)?;
    Xml::new(mock.path()).remove()?;

    assert!(mock.path().exists().not());

    Ok(())
}
