use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use clap::{Parser, Subcommand, ValueEnum};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};

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
    status: TaskStatus,
    description: String,
}

#[derive(Debug, Clone, ValueEnum, Serialize, Deserialize)]
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
            let task = Task {
                status,
                description,
            };
        }
        Command::Delete => todo!(),
        Command::Update => todo!(),
        Command::List => todo!(),
    }
}

fn load_tasks() -> Result<Vec<Task>, Box<dyn std::error::Error>> {
    if let Some(file_path_buf) = get_tasks_file_path() {
        let mut file = File::open(file_path_buf)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        if content.is_empty() {
            return Ok(Vec::new());
        }
        let tasks: Vec<Task> = serde_json::from_str(&content)?;
        Ok(tasks)
    } else {
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::NotFound, // Or `ErrorKind::Other`
            "Could not determine a suitable path for task storage.",
        )))
    }
}

fn get_tasks_file_path() -> Option<PathBuf> {
    if let Some(proj_dirs) = ProjectDirs::from("com", "leo1mml", "task-cli") {
        let data_dir = proj_dirs.data_dir();

        if !data_dir.exists() {
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
