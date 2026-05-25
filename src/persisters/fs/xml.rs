//! Utilities to handle XML files.
//!
//! The `XML` struct implements the [`FilePersister`] trait.

use std::io::Write as _;
use std::path::{Path, PathBuf};
use std::{fs, io};

use quick_xml::events::{BytesEnd, BytesStart, BytesText, Event};
use quick_xml::name::QName;
use quick_xml::{Reader, Writer};

use crate::models::{Priority, Task, Todo};
use crate::traits::FilePersister;

/// Representation of an XML file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Xml {
    /// Location of the XML file.
    path: PathBuf,
}

impl Xml {
    /// Constructor of the `Xml` struct.
    #[inline]
    pub fn new<T: AsRef<Path>>(path: T) -> Self {
        Self { path: path.as_ref().to_path_buf() }
    }

    /// Basic structure to initialize a XML file.
    #[inline]
    pub fn prolog() -> String {
        String::from(r#"<?xml version="1.0" encoding="UTF-8"?>"#) + "\n"
    }

    /// Document Type Definition of a XML file.
    #[rustfmt::skip]
    #[inline]
    pub fn dtd() -> String {
        String::from(
"<!DOCTYPE Tasks [
    <!ELEMENT Tasks (Task+)>
    <!ELEMENT Task (#PCDATA)>
    <!ATTLIST Task 
        id CDATA #REQUIRED
        priority (low | med | high | none) #REQUIRED
        checked (true | false) #REQUIRED
    >
]>\n",
        )
    }

    /// Writes a [Todo] instance into XML writer and returns a buffer with the content.
    ///
    /// # Errors
    /// - The XML Event can't be written.
    #[inline]
    pub fn todo_to_xml(todo: &Todo) -> super::Result<Vec<u8>> {
        let mut buffer = Vec::new();
        let mut writer = Writer::new_with_indent(&mut buffer, b' ', 4);

        writer.write_event(Event::Start(BytesStart::new("Tasks")))?;

        for task in &todo.tasks {
            Self::task_to_xml(&mut writer, task)?;
        }

        writer.write_event(Event::End(BytesEnd::new("Tasks")))?;

        Ok(buffer)
    }

    /// Writes a [Task] instance into XML writer.
    ///
    /// # Errors
    /// - The XML Event can't be written.
    #[inline]
    pub fn task_to_xml(writer: &mut Writer<&mut Vec<u8>>, task: &Task) -> io::Result<()> {
        let mut task_bytes = BytesStart::new("Task");
        task_bytes.push_attribute(("id", task.id.to_string().as_str()));
        task_bytes.push_attribute(("priority", task.priority.to_str()));
        task_bytes.push_attribute(("checked", task.checked.to_string().as_str()));

        writer.write_event(Event::Start(task_bytes))?;

        writer.write_event(Event::Text(BytesText::new(&task.content)))?;

        writer.write_event(Event::End(BytesEnd::new("Task")))
    }

    /// Reads the tasks from an XML reader and returns a vector of tasks.
    ///
    /// # Errors
    /// - A value can't be unescaped.
    #[inline]
    pub fn xml_to_tasks(mut reader: Reader<&[u8]>) -> super::Result<Vec<Task>> {
        let mut tasks = vec![];
        let mut task = None::<Task>;

        loop {
            match reader.read_event() {
                Ok(Event::Start(e)) if e.name() == QName(b"Task") => {
                    let mut new_task = Task::default();

                    for attr in e.attributes().flatten() {
                        let value = attr.unescape_value()?;
                        match attr.key {
                            QName(b"id") => new_task.id = value.parse().unwrap_or(0),
                            QName(b"priority") => new_task.priority = Priority::from(value),
                            QName(b"checked") => new_task.checked = value == "true",
                            _ => {}
                        }
                    }

                    task = Some(new_task);
                }

                Ok(Event::Text(e)) => {
                    if let Some(t) = &mut task {
                        t.content = e.unescape()?.into_owned();
                    }
                }

                Ok(Event::End(e)) if e.name() == QName(b"Task") => {
                    if let Some(t) = task.take() {
                        tasks.push(t);
                    }
                }

                Ok(Event::Eof) => break,

                Err(e) => {
                    eprintln!("Error reading the XML file: {e:?}");
                    break;
                }

                _ => {}
            }
        }

        Ok(tasks)
    }
}

impl FilePersister for Xml {
    #[inline]
    fn boxed(self) -> Box<dyn FilePersister> {
        Box::new(self)
    }

    #[inline]
    fn path(&self) -> &PathBuf {
        &self.path
    }

    #[inline]
    fn default(&self) -> String {
        Self::prolog() + &Self::dtd()
    }

    #[inline]
    fn tasks(&self) -> super::Result<Vec<Task>> {
        let xml = fs::read_to_string(&self.path)?;
        let reader = Reader::from_str(xml.trim());

        Self::xml_to_tasks(reader)
    }

    #[inline]
    fn open(&self) -> super::Result<fs::File> {
        let file = fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(true)
            .open(&self.path)?;

        Ok(file)
    }

    #[inline]
    fn write(&self, todo: &Todo) -> super::Result<()> {
        let buffer = Self::todo_to_xml(todo)?;
        let xml = String::from_utf8(buffer).map_err(super::Error::wrap)?;

        let bytes = [self.default(), xml].join("").into_bytes();

        self.open()?.write_all(&bytes)?;

        Ok(())
    }

    #[inline]
    fn clean(&self) -> super::Result<()> {
        fs::write(&self.path, self.default())?;

        Ok(())
    }

    #[inline]
    fn remove(&self) -> super::Result<()> {
        fs::remove_file(&self.path)?;

        Ok(())
    }
}
