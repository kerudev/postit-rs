use std::fs;
use std::ops::Not;

use postit::fs::{Csv, File, Format};
use postit::models::Todo;
use postit::traits::{FilePersister, Persister};
use postit::Action;

use crate::mocks::MockPath;

#[test]
fn error_wrap() {
    let msg = "Error";
    let err = postit::fs::Error::wrap(msg);

    assert!(matches!(err, postit::fs::Error::Other(_)));
}

#[test]
fn exists_return_true() -> postit::Result<()> {
    let mock = MockPath::create(Format::Csv)?;
    let file = File::from(mock.to_string())?;

    assert!(file.exists().is_ok_and(|bool| bool));

    Ok(())
}

#[test]
fn format_from() {
    assert_eq!(Format::from("txt"), Format::Csv);
    assert_eq!(Format::from("csv"), Format::Csv);
    #[cfg(feature = "json")]
    assert_eq!(Format::from("json"), Format::Json);
    #[cfg(feature = "xml")]
    assert_eq!(Format::from("xml"), Format::Xml);
}

#[test]
fn file_fmt_debug() -> postit::Result<()> {
    let mock = MockPath::create(Format::Csv)?;

    let persister = File::get_persister(mock.path())?;
    let file = File::new(persister);

    let debug_output = format!("{:?}", file);
    let expected_output = format!("File {{ file: {:?} }}", mock.path());

    assert_eq!(debug_output, expected_output);

    Ok(())
}

#[test]
fn path() -> postit::Result<()> {
    let mock = MockPath::create(Format::Csv)?;

    let file = File::from(mock.to_string())?;

    let result = file.path().to_path_buf();
    let expect = mock.path();

    assert_eq!(result, expect);

    Ok(())
}

#[test]
fn from() -> postit::Result<()> {
    let mock = MockPath::create(Format::Csv)?;

    let result = File::from(mock.to_string())?;
    let expect = File::new(Csv::new(mock.path()).boxed());

    assert_eq!(result, expect);

    Ok(())
}

#[test]
fn check_name_ok() -> postit::Result<()> {
    let mock = MockPath::create(Format::Csv)?;
    let mock_path = mock.path();

    let checked_path = File::check_name(mock_path.clone());

    let result = checked_path.file_name().unwrap();
    let expect = mock_path.file_name().unwrap();

    assert_eq!(result, expect);

    Ok(())
}

#[test]
fn check_content_is_empty() -> postit::Result<()> {
    let mock = MockPath::blank(Format::Csv)?;

    let persister = File::get_persister(mock.path())?;
    let expect = persister.default();

    let file = File::new(persister);
    file.check_content()?;

    let result = fs::read_to_string(mock.path())?;

    assert_eq!(result, expect);

    Ok(())
}

#[test]
fn check_content_exists() -> postit::Result<()> {
    let mock = MockPath::blank(Format::Csv)?;

    let persister = File::get_persister(mock.path())?;
    let expect = persister.default();

    let file = File::new(persister);
    file.check_content()?;

    let result = fs::read_to_string(mock.path())?;

    assert_eq!(result, expect);

    Ok(())
}

#[test]
fn check_content_not_exists() -> postit::Result<()> {
    let mock = MockPath::blank(Format::Csv)?;
    mock.instance.remove()?;

    let persister = File::get_persister(mock.path())?;
    let expect = persister.default();

    let file = File::new(persister);
    file.check_content()?;

    let result = fs::read_to_string(mock.path())?;

    assert_eq!(result, expect);

    Ok(())
}

#[test]
fn check_name_no_ext() {
    let path = "test";

    let checked_path = File::check_name(path);
    let expected_path = format!("{path}.csv");

    let result = checked_path.file_name().unwrap();
    let expect = expected_path.as_str();

    assert_eq!(result, expect);
}

#[test]
fn get_persister_csv() -> postit::Result<()> {
    let mock = MockPath::create(Format::Csv)?;

    let file = File::get_persister(mock.path())?;

    let result = file.path().extension().unwrap();
    let expect = "csv";

    assert_eq!(result, expect);

    Ok(())
}

#[test]
#[cfg(feature = "json")]
fn get_persister_json() -> postit::Result<()> {
    let mock = MockPath::create(Format::Json)?;

    let file = File::get_persister(mock.path())?;

    let result = file.path().extension().unwrap();
    let expect = "json";

    assert_eq!(result, expect);

    Ok(())
}

#[test]
#[cfg(feature = "xml")]
fn get_persister_xml() -> postit::Result<()> {
    let mock = MockPath::create(Format::Xml)?;

    let file = File::get_persister(mock.path())?;

    let result = file.path().extension().unwrap();
    let expect = "xml";

    assert_eq!(result, expect);

    Ok(())
}

#[test]
fn get_persister_txt() -> postit::Result<()> {
    let file = File::get_persister("test.txt")?;

    let result = file.path().extension().unwrap();
    let expect = "csv";

    assert_eq!(result, expect);

    Ok(())
}

#[test]
fn get_persister_any() -> postit::Result<()> {
    let file = File::get_persister("test.toml")?;

    let result = file.path().extension().unwrap();
    let expect = "csv";

    assert_eq!(result, expect);

    Ok(())
}

#[test]
fn check_name_no_name() -> postit::Result<()> {
    let path = ".csv";
    let file = File::from(path)?;

    assert_eq!(file.path().file_name().unwrap(), "tasks.csv");

    Ok(())
}

#[test]
fn get_persister_dot() {
    let err = File::get_persister(".");

    assert!(matches!(err.unwrap_err(), postit::Error::Fs(postit::fs::Error::IsDirectory)));
}

#[test]
fn file_persister_eq() -> postit::Result<()> {
    let mock = MockPath::create(Format::Csv)?;

    let left = File::get_persister(mock.path())?;
    let right = File::get_persister(mock.path())?;

    assert!(left == right);

    Ok(())
}

#[test]
fn create_err_already_exists() -> postit::Result<()> {
    let mock = MockPath::create(Format::Csv)?;
    let file = File::from(mock.to_string())?;

    assert!(file.create().is_err());

    Ok(())
}

#[test]
fn view_err_doesnt_exist() -> postit::Result<()> {
    let mock = MockPath::create(Format::Csv)?;
    let file = File::from(mock.to_string())?;
    file.remove()?;

    let err = file.view().unwrap_err();

    assert!(matches!(err, postit::Error::Fs(postit::fs::Error::FileDoesntExist(_))));

    Ok(())
}

#[test]
fn edit_err_doesnt_exist() -> postit::Result<()> {
    let mock = MockPath::create(Format::Csv)?;
    let file = File::from(mock.to_string())?;
    file.remove()?;

    let todo = Todo::sample();
    let ids = &[2, 3];
    let err = file.edit(&todo, ids, &Action::Check).unwrap_err();

    assert!(matches!(err, postit::Error::Fs(postit::fs::Error::FileDoesntExist(_))));

    Ok(())
}

#[test]
fn clean_err_doesnt_exist() -> postit::Result<()> {
    let mock = MockPath::create(Format::Csv)?;
    let file = File::from(mock.to_string())?;
    file.remove()?;

    let err = file.clean().unwrap_err();

    assert!(matches!(err, postit::Error::Fs(postit::fs::Error::FileDoesntExist(_))));

    Ok(())
}

#[test]
fn remove_ok() -> postit::Result<()> {
    let mock = MockPath::create(Format::Csv)?;
    let file = File::from(mock.to_string())?;

    assert!(file.remove().is_ok());
    assert!(mock.path().exists().not());

    Ok(())
}

#[test]
fn remove_err_doesnt_exist() -> postit::Result<()> {
    let mock = MockPath::create(Format::Csv)?;
    let file = File::from(mock.to_string())?;
    file.remove()?;

    let err = file.remove().unwrap_err();

    assert!(matches!(err, postit::Error::Fs(postit::fs::Error::FileDoesntExist(_))));

    Ok(())
}
