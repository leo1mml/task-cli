use crate::cli::{Cli, Command};
use crate::models::Task;
use crate::storage::{load_tasks, write_tasks};
use anyhow::Error;
use clap::Parser;

mod cli;
mod models;
mod storage;
mod utils;

fn main() -> Result<(), Error> {
    let args = Cli::parse();

    match args.command {
        Command::Add {
            status,
            description,
        } => {
            let task = Task::new(status, description);
            write_tasks(task)?;
            println!("Successfully recorded task");
        }
        Command::Delete => {
            todo!("Implement delete functionality");
        }
        Command::Update => {
            todo!("Implement update functionality");
        }
        Command::List => {
            let tasks = load_tasks()?;
            println!("{:#?}", tasks);
        }
    }
    Ok(())
}
