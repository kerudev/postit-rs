use postit::models::{Priority, Task};

fn fake_task_unchecked() -> Task {
    Task::new(1, String::from("Test"), Priority::Med, false)
}

fn fake_task_checked() -> Task {
    Task::new(1, String::from("Test"), Priority::High, true)
}

#[test]
fn new() {
    let result = Task::new(1, String::from("Test"), Priority::Med, false);
    let expected = fake_task_unchecked();

    assert_eq!(result, expected);
}

#[test]
fn from() {
    let line = "1,Test,med,false";

    let result = Task::from(line);
    let expected = fake_task_unchecked();

    assert_eq!(result, expected);
}

#[test]
fn unpack() {
    let line = "1,Test,med,false";

    let (id, content, priority, checked) = Task::split(line);

    let expected_id = 1;
    let expected_content = String::from("Test");
    let expected_priority = Priority::Med;
    let expected_checked = false;

    assert_eq!(expected_id, id);
    assert_eq!(expected_content, content);
    assert_eq!(expected_priority, priority);
    assert_eq!(expected_checked, checked);
}

#[test]
fn to_string() {
    let task = fake_task_unchecked();

    let result = task.as_line();
    let expect = format!("{},{},{},{}", task.id, task.content, task.priority, task.checked);

    assert_eq!(result, expect);
}

#[test]
fn check_ok() {
    let mut task = fake_task_unchecked();
    let result = task.check();

    assert!(result.is_ok());
}

#[test]
fn check_err() {
    let mut task = fake_task_checked();
    let result = task.check();

    assert!(result.is_err());
}

#[test]
fn uncheck_ok() {
    let mut task = fake_task_checked();
    let result = task.uncheck();

    assert!(result.is_ok());
}

#[test]
fn uncheck_err() {
    let mut task = fake_task_unchecked();
    let result = task.uncheck();

    assert!(result.is_err());
}

#[test]
fn priority_as_str() {
    assert_eq!(Priority::Low.to_str(), "low");
    assert_eq!(Priority::Med.to_str(), "med");
    assert_eq!(Priority::High.to_str(), "high");
    assert_eq!(Priority::None.to_str(), "none");
}
