use anyhow::Error;
use clap::{Parser, Subcommand, ValueEnum};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use serde_json::{from_reader, to_writer_pretty};
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::PathBuf;
use uuid::Uuid;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    Add {
        #[arg(short, long)]
        status: TaskStatus,
        #[arg(short, long)]
        description: String,
    },
    Delete,
    Update,
    List,
}

#[derive(Debug, Serialize, Deserialize)]
struct Task {
    id: Uuid,
    status: TaskStatus,
    description: String,
}

impl Task {
    fn new(status: TaskStatus, description: String) -> Self {
        Self {
            id: generate_uuid(),
            status,
            description,
        }
    }
}

fn generate_uuid() -> Uuid {
    Uuid::new_v4()
}

#[derive(Debug, Clone, ValueEnum, Serialize, Deserialize)]
#[clap(rename_all = "lower")]
enum TaskStatus {
    Todo,
    InProgress,
    Done,
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Command::Add {
            status,
            description,
        } => {
            let task = Task::new(status, description);
            match write_tasks(task) {
                Ok(_) => {
                    println!("Successfuly recorded task")
                }
                Err(error) => {
                    eprint!("{:#?}", error);
                }
            }
        }
        Command::Delete => todo!(),
        Command::Update => todo!(),
        Command::List => match load_tasks() {
            Ok(tasks) => {
                println!("{:#?}", tasks);
            }
            Err(error) => {
                eprintln!("{}", error);
            }
        },
    }
}

fn write_tasks(task: Task) -> Result<(), Error> {
    if let Some(file_path_buf) = get_tasks_file_path() {
        if !file_path_buf.exists() {
            File::create(&file_path_buf)?;
        }
        match File::open(&file_path_buf) {
            Ok(file) => write_task_to_file(task, file)?,
            Err(e) => {
                eprintln!("{:#?}", Error::new(e));
            }
        }
        Ok(())
    } else {
        Err(Error::new(std::io::Error::new(
            std::io::ErrorKind::NotFound, // Or `ErrorKind::Other`
            "Could not determine a suitable path for task storage.",
        )))
    }
}

fn write_task_to_file(task: Task, file: File) -> Result<(), Error> {
    let mut tasks = load_tasks()?;
    tasks.push(task);
    let writer = BufWriter::new(file);
    to_writer_pretty(writer, &tasks)?;
    Ok(())
}

fn load_tasks() -> Result<Vec<Task>, Error> {
    if let Some(file_path_buf) = get_tasks_file_path() {
        if file_path_buf.exists() {
            let file = File::open(file_path_buf)?;
            let buf_reader = BufReader::new(file);
            let tasks: Vec<Task> = from_reader(buf_reader)?;
            Ok(tasks)
        } else {
            Ok(Vec::new())
        }
    } else {
        Err(Error::new(std::io::Error::new(
            std::io::ErrorKind::NotFound, // Or `ErrorKind::Other`
            "Could not determine a suitable path for task storage.",
        )))
    }
}

fn get_tasks_file_path() -> Option<PathBuf> {
    if let Some(proj_dirs) = ProjectDirs::from("com", "leo1mml", "task-cli") {
        let data_dir = proj_dirs.data_dir();

        if !data_dir.exists() {
            println!("Data directory does not exist, creating one...");
            if let Err(e) = std::fs::create_dir_all(data_dir) {
                eprintln!("Error creating data directory {:?}: {}", data_dir, e);
                return None;
            }
        }
        let tasks_file_path = data_dir.join("tasks.json");
        Some(tasks_file_path)
    } else {
        eprintln!("Could not determine project directories");
        None
    }
}
