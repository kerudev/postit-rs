use postit::cli::subcommands as sub;

use assert_cmd::Command;
use postit::Docs;
use std::process::Output;

fn get_docs_output(command: &str) -> Output {
    Command::cargo_bin("postit")
        .unwrap()
        .args(["docs", command])
        .output()
        .expect("Error while running the test")
}

#[test]
fn docs_sample_output() {
    let output = get_docs_output("sample");
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(output.status.success());
    assert!(stdout.contains("Usage: postit sample [--persister|-p]"));
    assert!(stdout.contains("Alias: postit sa ..."));
}

#[test]
fn docs_sample_no_panic() {
    Docs::run(&sub::Docs::Sample);
}

#[test]
fn docs_view_output() {
    let output = get_docs_output("view");
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(output.status.success());
    assert!(stdout.contains("Usage: postit view [--persister|-p]"));
    assert!(stdout.contains("Alias: postit v ..."));
}

#[test]
fn docs_view_no_panic() {
    Docs::run(&sub::Docs::View);
}

#[test]
fn docs_add_output() {
    let output = get_docs_output("add");
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(output.status.success());
    assert!(stdout.contains("Usage: postit add <PRIORITY> <CONTENT> [--persister|-p]"));
    assert!(stdout.contains("Alias: postit a ..."));
}

#[test]
fn docs_add_no_panic() {
    Docs::run(&sub::Docs::Add);
}

#[test]
fn docs_set_output() {
    let output = get_docs_output("set");
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(output.status.success());
    assert!(stdout.contains("Usage: postit set <COMMAND> [--persister|-p]"));
    assert!(stdout.contains("Alias: postit s ..."));
}

#[test]
fn docs_set_no_panic() {
    Docs::run(&sub::Docs::Set);
}

#[test]
fn docs_check_output() {
    let output = get_docs_output("check");
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(output.status.success());
    assert!(stdout.contains("Usage: postit check <IDS> [--persister|-p]"));
    assert!(stdout.contains("Alias: postit c ..."));
}

#[test]
fn docs_check_no_panic() {
    Docs::run(&sub::Docs::Check);
}

#[test]
fn docs_uncheck_output() {
    let output = get_docs_output("uncheck");
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(output.status.success());
    assert!(stdout.contains("Usage: postit uncheck <IDS> [--persister|-p]"));
    assert!(stdout.contains("Alias: postit uc ..."));
}

#[test]
fn docs_uncheck_no_panic() {
    Docs::run(&sub::Docs::Uncheck);
}

#[test]
fn docs_drop_output() {
    let output = get_docs_output("drop");
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(output.status.success());
    assert!(stdout.contains("Usage: postit drop <IDS> [--persister|-p]"));
    assert!(stdout.contains("Alias: postit d ..."));
}

#[test]
fn docs_drop_no_panic() {
    Docs::run(&sub::Docs::Drop);
}

#[test]
fn docs_copy_output() {
    let output = get_docs_output("copy");
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(output.status.success());
    assert!(stdout.contains("Usage: postit copy <LEFT> <RIGHT>"));
    assert!(stdout.contains("Alias: postit cp ..."));
}

#[test]
fn docs_copy_no_panic() {
    Docs::run(&sub::Docs::Copy);
}

#[test]
fn docs_clean_output() {
    let output = get_docs_output("clean");
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(output.status.success());
    assert!(stdout.contains("Usage: postit clean [--persister|-p]"));
    assert!(stdout.contains("Alias: postit cl ..."));
}

#[test]
fn docs_clean_no_panic() {
    Docs::run(&sub::Docs::Clean);
}

#[test]
fn docs_remove_output() {
    let output = get_docs_output("remove");
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(output.status.success());
    assert!(stdout.contains("Usage: postit remove [--persister|-p]"));
    assert!(stdout.contains("Alias: postit rm ..."));
}

#[test]
fn docs_remove_no_panic() {
    Docs::run(&sub::Docs::Remove);
}

#[test]
fn docs_config_output() {
    let output = get_docs_output("config");
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(output.status.success());

    assert!(stdout.contains("Usage: postit config <COMMAND>"));
    assert!(stdout.contains("Alias: postit conf ..."));
}

#[test]
fn docs_config_no_panic() {
    Docs::run(&sub::Docs::Config);
}

#[test]
fn flag_persister_output() {
    let output = get_docs_output("persister");
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(output.status.success());
    assert!(stdout.contains("Usage: postit <COMMAND> [--persister | -p] <PATH_OR_CONN>"));
}

#[test]
fn flag_persister_no_panic() {
    Docs::persister();
}
