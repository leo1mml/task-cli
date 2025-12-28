use crate::models::Task;
use anyhow::{Error, anyhow};
use directories::ProjectDirs;
use serde_json::{from_reader, to_writer_pretty};
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

    fn write_task_to_file(&self, tasks: &Vec<Task>, file: File) -> Result<(), Error> {
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
        self.write_task_to_file(&tasks, file)?;
        Ok(())
    }

    fn remove_task(&self, id: &str) -> Result<(), Error> {
        let file_path = self.get_tasks_file_path()?;

        let mut tasks = self.load_tasks()?; // Load existing tasks

        if let Some(index) = tasks.iter().position(|x| x.id.to_string() == id) {
            tasks.remove(index);
            let file = File::create(&file_path)?; // Create or truncate the file for writing
            self.write_task_to_file(&tasks, file)?;
            Ok(())
        } else {
            Err(anyhow!("No item with specified id found"))
        }
    }
}
