use crate::models::Task;
use anyhow::{Error, anyhow};
use directories::ProjectDirs;
use serde_json::{from_reader, to_writer_pretty};
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::PathBuf; // Import Task from models module

const QUALIFIER: &str = "com";
const ORGANIZATION: &str = "leo1mml";
const APPLICATION: &str = "task-cli";
const DATA_FILE_NAME: &str = "tasks.json";

fn get_tasks_file_path() -> Result<PathBuf, Error> {
    if let Some(proj_dirs) = ProjectDirs::from(QUALIFIER, ORGANIZATION, APPLICATION) {
        let data_dir = proj_dirs.data_dir();

        if !data_dir.exists() {
            println!(
                "Data directory does not exist, creating one at {:?}",
                data_dir
            );
            std::fs::create_dir_all(data_dir)?;
        }
        Ok(data_dir.join(DATA_FILE_NAME))
    } else {
        Err(Error::new(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Could not determine a suitable path for task storage.",
        )))
    }
}

pub fn load_tasks() -> Result<Vec<Task>, Error> {
    let file_path = get_tasks_file_path()?;
    if file_path.exists() {
        let file = File::open(file_path)?;
        let buf_reader = BufReader::new(file);
        let tasks: Vec<Task> = from_reader(buf_reader).unwrap_or(Vec::new());
        Ok(tasks)
    } else {
        Ok(Vec::new()) // Return an empty vector if the file doesn't exist yet
    }
}

fn write_task_to_file(tasks: &Vec<Task>, file: File) -> Result<(), Error> {
    let writer = BufWriter::new(file);
    to_writer_pretty(writer, tasks)?;
    Ok(())
}

pub fn write_task(task: Task) -> Result<(), Error> {
    let file_path = get_tasks_file_path()?;

    let mut tasks = load_tasks()?; // Load existing tasks

    // Check if the task already exists (e.g., for update scenarios, though not fully implemented yet)
    // For "Add" command, we always add a new task.
    tasks.push(task);

    let file = File::create(&file_path)?; // Create or truncate the file for writing
    write_task_to_file(&tasks, file)?;
    Ok(())
}

pub fn remove_task(id: &str) -> Result<(), Error> {
    let file_path = get_tasks_file_path()?;

    let mut tasks = load_tasks()?; // Load existing tasks

    if let Some(index) = tasks.iter().position(|x| x.id.to_string() == id) {
        tasks.remove(index);
        let file = File::create(&file_path)?; // Create or truncate the file for writing
        write_task_to_file(&tasks, file)?;
        Ok(())
    } else {
        Err(anyhow!("No item with specified id found"))
    }
}
