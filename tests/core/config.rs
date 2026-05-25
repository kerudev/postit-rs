use std::ops::Not;
use std::path::PathBuf;

use postit::cli::{arguments as args, subcommands as sub};
use postit::config::Config;

use crate::mocks::{MockConfig, MockEnvVar};

#[test]
fn error_wrap() {
    let msg = "Error";
    let err = postit::config::Error::wrap(msg);

    assert!(matches!(err, postit::config::Error::Other(_)));
}

#[test]
fn fmt_display() -> postit::Result<()> {
    let config = Config {
        persister: "tasks.json".to_string(),
        force_drop: true,
        force_copy: false,
        drop_after_copy: true,
    };

    let result = format!("{}", config);

    let expect = "
persister: tasks.json
force_drop: true
force_copy: false
drop_after_copy: true";

    assert_eq!(result.trim(), expect.trim());

    Ok(())
}

#[test]
fn manage_path() -> postit::Result<()> {
    let mock = MockConfig::new()?;

    Config::manage(sub::Config::Path)?;

    assert!(mock.path().exists());

    Ok(())
}

#[test]
fn path_exists_output() -> postit::Result<()> {
    let mock = MockConfig::new()?;

    let output = assert_cmd::Command::cargo_bin("postit")
        .map_err(postit::Error::wrap)?
        .args(["config", "path"])
        .output()
        .map_err(postit::Error::wrap)?;

    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(output.status.success());
    assert!(stdout.contains(mock.path().to_str().unwrap()));

    Ok(())
}

#[test]
fn print_path_not_exists_error() -> postit::Result<()> {
    let _mock = MockConfig::new()?;

    Config::remove()?;

    assert!(Config::print_path().is_err());

    Ok(())
}

#[test]
fn path_not_exists_output() -> postit::Result<()> {
    let _env = MockEnvVar::new().rm(["POSTIT_ROOT"]);

    let home = MockConfig::home()?;
    let path = PathBuf::from(home);

    let output = assert_cmd::Command::cargo_bin("postit")
        .map_err(postit::Error::wrap)?
        .args(["config", "path"])
        .output()
        .map_err(postit::Error::wrap)?;

    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(output.status.success().not());
    assert!(stderr.contains(path.parent().unwrap().to_str().unwrap()));

    Ok(())
}

#[test]
fn manage_env() -> postit::Result<()> {
    let mock = MockConfig::new()?;

    Config::manage(sub::Config::Env)?;

    assert!(mock.path().exists());

    Ok(())
}

#[test]
fn env_output() -> postit::Result<()> {
    let _mock = MockConfig::new()?;

    let output = assert_cmd::Command::cargo_bin("postit")
        .map_err(postit::Error::wrap)?
        .args(["config", "env"])
        .output()
        .map_err(postit::Error::wrap)?;

    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(output.status.success());
    assert!(stdout.contains(&Config::env().map_err(postit::Error::wrap)?));

    Ok(())
}

#[test]
fn env_is_empty() -> postit::Result<()> {
    let _env = MockEnvVar::new().set([("POSTIT_ROOT", "")]);

    assert!(Config::print_env().is_err());

    Ok(())
}

#[test]
fn manage_init_path_exists() -> postit::Result<()> {
    let mock = MockConfig::new()?;

    assert!(Config::manage(sub::Config::Init).is_err());

    let result = Config::load()?;
    let expect = Config::default();

    assert!(mock.path().exists());
    assert_eq!(result, expect);

    Ok(())
}

#[test]
fn manage_remove() -> postit::Result<()> {
    let mock = MockConfig::new()?;

    assert!(Config::manage(sub::Config::Remove).is_ok());
    assert!(mock.path().exists().not());

    Ok(())
}

#[test]
fn manage_remove_config_doesnt_exist() -> postit::Result<()> {
    let mock = MockConfig::new()?;

    assert!(Config::manage(sub::Config::Remove).is_ok());

    assert!(Config::manage(sub::Config::Remove).is_err());
    assert!(mock.path().exists().not());

    Ok(())
}

#[test]
fn manage_list() -> postit::Result<()> {
    let _mock = MockConfig::new()?;

    Config::manage(sub::Config::List)?;

    Ok(())
}

#[test]
fn manage_list_err() {
    let _env = MockEnvVar::new().rm(["POSTIT_ROOT"]);

    assert!(Config::manage(sub::Config::List).is_err());
}

#[test]
fn manage_list_output() -> postit::Result<()> {
    let _mock = MockConfig::new()?;

    let output = assert_cmd::Command::cargo_bin("postit")
        .map_err(postit::Error::wrap)?
        .args(["config", "list"])
        .output()
        .map_err(postit::Error::wrap)?;

    let stdout = String::from_utf8_lossy(&output.stdout);

    let expect = "
persister: tasks.csv
force_drop: false
force_copy: false
drop_after_copy: false";

    assert!(output.status.success());
    assert!(stdout.trim().contains(expect.trim()));

    Ok(())
}

#[test]
fn manage_set_any() -> postit::Result<()> {
    let _mock = MockConfig::new()?;

    let args = args::ConfigSet {
        persister: Some(String::from("tasks.json")),
        force_drop: None,
        force_copy: None,
        drop_after_copy: None,
    };

    Config::manage(sub::Config::Set(args))?;

    let result = Config::load()?;
    let expect = Config {
        persister: String::from("tasks.json"),
        force_drop: false,
        force_copy: false,
        drop_after_copy: false,
    };

    assert_eq!(result, expect);

    Ok(())
}

#[test]
fn manage_set_all() -> postit::Result<()> {
    let _mock = MockConfig::new()?;

    let args = args::ConfigSet {
        persister: Some(String::from("tasks.json")),
        force_drop: Some(true),
        force_copy: Some(true),
        drop_after_copy: Some(true),
    };

    Config::manage(sub::Config::Set(args))?;

    let result = Config::load()?;
    let expect = Config {
        persister: String::from("tasks.json"),
        force_drop: true,
        force_copy: true,
        drop_after_copy: true,
    };

    assert_eq!(result, expect);

    Ok(())
}

#[test]
fn manage_set_err_path_doesnt_exist() -> postit::Result<()> {
    let args = args::ConfigSet {
        persister: None,
        force_drop: None,
        force_copy: None,
        drop_after_copy: None,
    };

    let err = Config::manage(sub::Config::Set(args)).unwrap_err();

    assert!(matches!(err, postit::config::Error::FileDoesntExist(_)));

    Ok(())
}

#[test]
fn manage_set_err_none_set() -> postit::Result<()> {
    let _mock = MockConfig::new()?;

    let args = args::ConfigSet {
        persister: None,
        force_drop: None,
        force_copy: None,
        drop_after_copy: None,
    };

    let err = Config::manage(sub::Config::Set(args)).unwrap_err();

    assert!(matches!(err, postit::config::Error::EmptySetArgs));

    Ok(())
}

#[test]
fn default() -> postit::Result<()> {
    let config = Config::default();

    assert_eq!(config.persister, "tasks.csv");
    assert!(config.force_drop.not());
    assert!(config.force_copy.not());
    assert!(config.drop_after_copy.not());

    Ok(())
}

#[test]
fn path_from_env_err_not_unicode() {
    use std::os::unix::ffi::OsStrExt;

    let value = std::ffi::OsStr::from_bytes(b"\xFFinvalid");
    let _env = MockEnvVar::new().set([("POSTIT_ROOT", value)]);

    let err = Config::path_from_env().unwrap_err();

    assert!(matches!(err, postit::config::Error::NotUnicodeEnv(_)));
}

#[test]
fn path_from_env_err_relative_path() {
    let _env = MockEnvVar::new().set([("POSTIT_ROOT", ".")]);

    let err = Config::path_from_env().unwrap_err();

    assert!(matches!(err, postit::config::Error::InvalidPathEnvVar(_)));
}

#[test]
fn path_default() -> postit::Result<()> {
    let _env = MockEnvVar::new().rm(["POSTIT_ROOT"]);

    let expect = Config::path()?;

    let result = Config::home()
        .join(".postit")
        .join(Config::config_file_name());

    assert_eq!(result, expect);

    Ok(())
}

#[test]
fn path_empty_env() -> postit::Result<()> {
    let _env = MockEnvVar::new().set([("POSTIT_ROOT", "")]);

    assert!(Config::path().is_err());

    Ok(())
}

#[test]
fn path_custom() -> postit::Result<()> {
    let home = Config::home();
    let tmp = home.join("tmp").to_string_lossy().into_owned();

    let _env = MockEnvVar::new().set([("POSTIT_ROOT", &tmp)]);

    let result = Config::path()?;
    let expect = PathBuf::from(tmp).join(".postit.toml");

    assert_eq!(result, expect);

    Ok(())
}

#[test]
fn load_default() -> postit::Result<()> {
    let _mock = MockConfig::new()?;

    let result = Config::load()?;
    let expect = Config::default();

    assert_eq!(result, expect);

    Ok(())
}

#[test]
fn save() -> postit::Result<()> {
    let _mock = MockConfig::new()?;

    let config = Config::default();
    config.save()?;

    assert_eq!(Config::load()?, Config::default());

    Ok(())
}

#[test]
#[should_panic]
fn save_file_doesnt_exist() {
    let _mock = MockConfig::new().unwrap();

    let _env = MockEnvVar::new().set([("POSTIT_ROOT", "//")]);

    let config = Config::default();
    config.save().unwrap();

    assert_eq!(Config::load().unwrap(), Config::default());
}
