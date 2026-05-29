//! Contains commands used for documentation purposes.
//!
//! Their information can be accessed by using:
//! - postit docs <COMMAND>

#![allow(clippy::single_call_fn)]

use crate::cli::subcommands as sub;
use crate::models::{Priority, Task, Todo};

/// Contains use cases for every command and flag.
#[doc(hidden)]
#[non_exhaustive]
pub struct Docs;

impl Docs {
    /// Uses the [`sub::Docs`] value passed to show its corresponding example.
    #[inline]
    pub fn run(cmnd: &sub::Docs) {
        match *cmnd {
            sub::Docs::Config => Self::config(),
            sub::Docs::View => Self::view(),
            sub::Docs::Add => Self::add(),
            sub::Docs::Set => Self::set(),
            sub::Docs::Check => Self::check(),
            sub::Docs::Uncheck => Self::uncheck(),
            sub::Docs::Drop => Self::drop(),
            sub::Docs::Sample => Self::sample(),
            sub::Docs::Copy => Self::copy(),
            sub::Docs::Clean => Self::clean(),
            sub::Docs::Remove => Self::remove(),
            sub::Docs::Persister => Self::persister(),
        }
    }

    /// Use case of the 'sample' command.
    ///
    /// # Panics
    /// If there is an unexpected error while displaying the example.
    #[inline]
    pub fn sample() {
        println!(
            "
Usage: postit sample [--persister|-p]
Alias: postit sa ...

Description:
    Populates a persister with fake data so you can test other commands.

How to use:
    postit sample -p tasks.csv

Sample:"
        );

        Todo::sample().view().unwrap();
    }

    /// Use case of the 'view' command.
    ///
    /// # Panics
    /// If there is an unexpected error while displaying the example.
    #[inline]
    pub fn view() {
        println!(
            "
Usage: postit view [--persister|-p]
Alias: postit v ...

Description:
    Shows the list of tasks stored in a persister.

How to use:
    postit view -p tasks.csv
"
        );

        Todo::sample().view().unwrap();
    }

    /// Use case of the 'add' command.
    ///
    /// # Panics
    /// If there is an unexpected error while displaying the example.
    #[inline]
    pub fn add() {
        let line = "5,New task,low,false";
        let task = Task::from(line);

        println!(
            "
Usage: postit add <PRIORITY> <CONTENT> [--persister|-p]
Alias: postit a ...

Description:
    Creates a task with the format 'id,content,priority,checked': 
    - id: a unique unsigned integer.
    - content: description of the task.
    - priority: high, med, low or none.
    - checked: true or false.

    To add a task, just provide the priority and the content of the task.

How to use:
    postit add low \"New task\" -p tasks.csv

    The new task will be displayed like this: {task}
"
        );

        let mut todo = Todo::sample();

        println!("Before:");

        todo.view().unwrap();

        println!();
        println!("After:");

        todo.add(task);
        todo.view().unwrap();
    }

    /// Use case of the 'set' command.
    #[inline]
    pub fn set() {
        fn set_content() {
            let mut todo = Todo::sample();
            let new_content = "New content";
            let line = format!("2,{new_content},med,false");
            let task = Task::from(line);

            println!(
                "
How to use (content):
    postit set content \"{}\" 2

    Old task: {}
    New task: {}
",
                new_content, todo.tasks[1], task
            );

            println!("Before:");

            todo.view().unwrap();

            println!();
            println!("After:");

            todo.set_content(&[2], new_content).unwrap();
            todo.view().unwrap();
        }

        fn set_priority() {
            let mut todo = Todo::sample();
            let new_priority = Priority::Low;
            let line = format!("2,Task,{new_priority},false");
            let task = Task::from(line);

            println!(
                "
How to use (priority):
    postit set priority low 3

    Old task: {}
    New task: {}
",
                todo.tasks[1], task
            );

            println!("Before:");

            todo.view().unwrap();

            println!();
            println!("After:");

            todo.set_priority(&[2], &new_priority).unwrap();
            todo.view().unwrap();
        }

        println!(
            "
Usage: postit set <COMMAND> [--persister|-p]
Alias: postit s ...

Description:
    Changes the value of task's properties.
    
    These are the available subcommands:
    - content: postit set content <CONTENT> [IDS]...
    - priority: postit set priority <PRIORITY> [IDS]..."
        );

        set_content();
        set_priority();
    }

    /// Use case of the 'check' command.
    ///
    /// # Panics
    /// If there is an unexpected error while displaying the example.
    #[inline]
    pub fn check() {
        println!(
            "
Usage: postit check <IDS> [--persister|-p]
Alias: postit c ...

Description:
    Checks tasks if they are unchecked.

How to use:
    postit check 2,3 -p tasks.csv
"
        );

        let mut todo = Todo::sample();

        println!("Before:");

        todo.view().unwrap();

        println!();
        println!("After:");

        todo.check(&[2, 3]).unwrap();
        todo.view().unwrap();
    }

    /// Use case of the 'uncheck' command.
    ///
    /// # Panics
    /// If there is an unexpected error while displaying the example.
    #[inline]
    pub fn uncheck() {
        println!(
            "
Usage: postit uncheck <IDS> [--persister|-p]
Alias: postit uc ...

Description:
    Unchecks tasks if they are checked.

How to use:
    postit uncheck 2,3 -p tasks.csv
"
        );

        let mut todo = Todo::sample();

        println!("Before:");

        todo.view().unwrap();

        println!();
        println!("After:");

        todo.uncheck(&[2, 3]).unwrap();
        todo.view().unwrap();
    }

    /// Use case of the 'drop' command.
    ///
    /// # Panics
    /// If there is an unexpected error while displaying the example.
    #[inline]
    pub fn drop() {
        fn force_drop() {
            println!(
                "
Config:
    You can set the 'force_drop' config to 'true' to drop tasks whether 
    they are checked or not.
"
            );

            let mut todo = Todo::sample();

            println!("Before:");

            todo.view().unwrap();

            println!();
            println!("After:");

            todo.check(&[2]).unwrap();
            todo.drop(&[2, 3]).unwrap();
            todo.view().unwrap();
        }

        println!(
            "
Usage: postit drop <IDS> [--persister|-p]
Alias: postit d ...

Description:
    By default, only checked tasks can be dropped.

How to use:
    postit drop 2,3 -p tasks.csv
"
        );

        let mut todo = Todo::sample();

        println!("Before:");

        todo.view().unwrap();

        println!();
        println!("After:");

        todo.drop(&[2, 3]).unwrap();
        todo.view().unwrap();

        force_drop();
    }

    /// Use case of the 'copy' command.
    #[inline]
    pub fn copy() {
        println!(
            "
Usage: postit copy <LEFT> <RIGHT>
Alias: postit cp ...

Description:
    Copies a persister's contents into another, meaning you can use this
    command to 'translate' tasks to a different format.

How to use:
    postit copy tasks.csv tasks.json
    
    postit copy tasks.xml tasks.db

    postit copy tasks.db tasks.json

    ...

Config:
    By default, if the persister at '<RIGHT>' exists, 'postit' will refuse to
    overwrite its tasks in case you are using that persister as a backup or you
    simply don't want to overwrite it.

    You can set the 'force_copy' config to 'true' to overwrite it anyways.

    If you want to copy your tasks and delete the '<LEFT>' persister, you can do so
    by setting the 'drop_after_copy' config to 'true'. This will delete the file or
    table located at '<LEFT>'.

Special parameters:
    There are two special parameters that go into the '<LEFT>' argument.

    Assuming the persister defined at the configuration file is 'tasks.csv':

    - 'from': copies the tasks from '<RIGHT>' to the persister defined at the config.
      'postit copy from tasks.json' is the same as 'postit copy tasks.json tasks.csv'

    - 'to': copies the tasks from the persister defined at the config to '<RIGHT>'.
      'postit copy to tasks.json' is the same as 'postit copy tasks.csv tasks.json'"
        );
    }

    /// Use case of the 'clean' command.
    #[inline]
    pub fn clean() {
        println!(
            "
Usage: postit clean [--persister|-p]
Alias: postit cl ...

Description:
    Deletes all tasks from a persister.

How to use:
    postit clean"
        );
    }

    /// Use case of the 'remove' command.
    #[inline]
    pub fn remove() {
        println!(
            "
Usage: postit remove [--persister|-p]
Alias: postit rm ...

Description:
    Deletes the persister completely (file or table).

How to use:
    postit remove"
        );
    }

    /// Use case of the 'config' command.
    #[inline]
    pub fn config() {
        println!(
            "
Usage: postit config <COMMAND>
Alias: postit conf ...

Description:
    Manages the config file. Uses the 'POSTIT_ROOT' environment variable to
    locate the file.

Available subcommands:
    env       Shows the value of the 'POSTIT_ROOT' env var
    path      Shows the path of the config file
    init      Creates the .postit.toml file
    list      Shows the current config values     (alias: ls)
    set       Changes config values               (alias: s)
    remove    Deletes the config file             (alias: rm)

How to use:
    postit config env

    postit config path

    postit config init

    postit config list

    postit config set [OPTIONS]

    postit config remove

Examples:
    postit config set --persister tasks.json --force-copy true

    postit config set  // You must provide a flag and value to set

Config values:
    After running 'postit config init', postit will generate a file with the
    default settings, which you can change by using 'postit config set [OPTIONS]':

    - persister (string): 'tasks.csv' by default.
      Defines where tasks are stored (the '-p' or '--persister' flag can override this).
      It can be the path to a file or a database connection string (including protocol).

    - force_drop (bool): false by default.
      If 'true', allows dropping tasks even if they are not checked.

    - force_copy (bool): false by default.
      If 'true', allows overwriting persisters when using the 'copy' command.

    - drop_after_copy (bool): false by default.
      If 'true', drops a persister (file or table) after copying.
    
You can also check https://docs.rs/postit/latest/postit/struct.Config.html for more info."
        );
    }

    /// Use case of the 'persister' flag.
    #[inline]
    pub fn persister() {
        println!(
            "
Usage: postit <COMMAND> [--persister | -p] <PATH_OR_CONN>

Description:
    Specifies the persister where the tasks will be read from and saved to.

    It can be a file (CSV, JSON, etc.) or a database (SQLite, etc.). The persister
    is defined in '.postit.toml', or you can override it with the `-p` flag.

    There are currently 4 supported persisters:

    - Files
      - csv             (e.g.: tasks.csv)
      - json            (e.g.: tasks.json)
      - xml             (e.g.: tasks.xml)

    - Databases
      - SQLite          (e.g.: tasks.db, tasks.sqlite or tasks.sqlite3)
      - MongoDB         (e.g.: mongodb://user:pass@host:port)
      - MongoDB Atlas   (e.g.: mongodb+srv://user:pass@cluster)

How to use:
    postit view --persister tasks.csv

    postit view --persister tasks.db

    postit view --persister mongodb://localhost:27017
    
    postit view --persister mongodb+srv://my_user:my_pass@cluster.mongodb.net
    
    ..."
        );
    }
}
