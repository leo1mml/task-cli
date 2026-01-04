use crate::models::{Task, TaskStatus};
use anyhow::{Error, anyhow};
use directories::ProjectDirs;
use serde_json::{from_reader, to_writer_pretty};
#[cfg(test)]
use std::cell::RefCell;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::PathBuf; // Import Task from models module

#[derive(Debug)]
pub struct FileStorage {
    pub qualifier: String,
    pub organization: String,
    pub application: String,
    pub data_file_name: String,
}

pub trait TaskStorage {
    fn load_tasks(&self) -> Result<Vec<Task>, Error>;
    fn write_task(&self, task: Task) -> Result<(), Error>;
    fn remove_task(&self, id: &str) -> Result<(), Error>;
    fn update_task(&self, id: &str, status: TaskStatus, description: &str) -> Result<(), Error>;
}

#[cfg(test)]
#[allow(dead_code)]
pub struct MockStorage {
    pub tasks: RefCell<Vec<Option<Task>>>,
    pub should_have_write_error: bool,
}

#[cfg(test)]
impl TaskStorage for MockStorage {
    fn load_tasks(&self) -> Result<Vec<Task>, Error> {
        let mut storage = self.tasks.borrow_mut();
        let tasks: Vec<Task> = storage.iter_mut().filter_map(|x| x.take()).collect();
        Ok(tasks)
    }

    fn write_task(&self, task: Task) -> Result<(), Error> {
        let mut storage = self.tasks.borrow_mut();
        storage.push(Some(task));
        if self.should_have_write_error {
            Err(anyhow!("Write error occurred."))
        } else {
            Ok(())
        }
    }

    fn remove_task(&self, id: &str) -> Result<(), Error> {
        let mut tasks = self.tasks.borrow_mut();
        tasks.retain(|slot| match slot {
            Some(task) => task.id.to_string() != id,
            None => false,
        });
        Ok(())
    }

    fn update_task(&self, id: &str, status: TaskStatus, description: &str) -> Result<(), Error> {
        if let Some(task_to_update) = self
            .tasks
            .borrow_mut()
            .iter_mut()
            .find_map(|slot| match slot {
                Some(task) if task.id.to_string() == id => Some(task),
                _ => None,
            })
        {
            task_to_update.status = status;
            task_to_update.description = description.to_string();
            Ok(())
        } else {
            Err(anyhow!("Not found"))
        }
    }
}

impl FileStorage {
    fn get_tasks_file_path(&self) -> Result<PathBuf, Error> {
        if let Some(proj_dirs) =
            ProjectDirs::from(&self.qualifier, &self.organization, &self.application)
        {
            let data_dir = proj_dirs.data_dir();

            if !data_dir.exists() {
                println!(
                    "Data directory does not exist, creating one at {:?}",
                    data_dir
                );
                std::fs::create_dir_all(data_dir)?;
            }
            Ok(data_dir.join(&self.data_file_name))
        } else {
            Err(Error::new(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Could not determine a suitable path for task storage.",
            )))
        }
    }

    fn write_tasks_to_file(&self, tasks: &Vec<Task>, file: File) -> Result<(), Error> {
        let writer = BufWriter::new(file);
        to_writer_pretty(writer, tasks)?;
        Ok(())
    }
}

impl TaskStorage for FileStorage {
    fn load_tasks(&self) -> Result<Vec<Task>, Error> {
        let file_path = self.get_tasks_file_path()?;
        if file_path.exists() {
            let file = File::open(file_path)?;
            let buf_reader = BufReader::new(file);
            let tasks: Vec<Task> = from_reader(buf_reader).unwrap_or(Vec::new());
            Ok(tasks)
        } else {
            Ok(Vec::new()) // Return an empty vector if the file doesn't exist yet
        }
    }

    fn write_task(&self, task: Task) -> Result<(), Error> {
        let file_path = self.get_tasks_file_path()?;

        let mut tasks = self.load_tasks()?; // Load existing tasks

        // Check if the task already exists (e.g., for update scenarios, though not fully implemented yet)
        // For "Add" command, we always add a new task.
        tasks.push(task);

        let file = File::create(&file_path)?; // Create or truncate the file for writing
        self.write_tasks_to_file(&tasks, file)?;
        Ok(())
    }

    fn remove_task(&self, id: &str) -> Result<(), Error> {
        let file_path = self.get_tasks_file_path()?;

        let mut tasks = self.load_tasks()?; // Load existing tasks

        if let Some(index) = tasks.iter().position(|x| x.id.to_string() == id) {
            tasks.remove(index);
            let file = File::create(&file_path)?; // Create or truncate the file for writing
            self.write_tasks_to_file(&tasks, file)?;
            Ok(())
        } else {
            Err(anyhow!("No item with specified id found"))
        }
    }

    fn update_task(&self, id: &str, status: TaskStatus, description: &str) -> Result<(), Error> {
        let file_path = self.get_tasks_file_path()?;
        let mut tasks = self.load_tasks()?; // Load existing tasks
        let Some(index) = tasks.iter().position(|x| x.id.to_string() == id) else {
            return Err(anyhow!("No item with specified id found"));
        };
        let mut task = tasks.remove(index);
        task.description = description.to_string();
        task.status = status;
        tasks.insert(index, task);
        let file = File::create(&file_path)?; // Create or truncate the file for writing
        self.write_tasks_to_file(&tasks, file)?;
        Ok(())
    }
}
