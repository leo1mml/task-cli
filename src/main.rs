mod cli;
mod models;
mod storage;
mod utils;

use anyhow::Error;
use clap::Parser;
use cli::{Cli, Command};
use crossterm::{
    cursor::MoveTo,
    event::{self, Event, KeyCode, read},
    execute,
    terminal::{Clear, ClearType},
};
use models::Task;
use std::io::stdout;
use storage::{load_tasks, write_tasks};

fn main() -> Result<(), Error> {
    let args = Cli::parse();

    let Some(command) = args.command else {
        loop_for_commands();
        return Ok(());
    };
    run_command(command)
}

fn loop_for_commands() -> Result<(), Error> {
    loop {
        clear_and_reset();
        print_command_prompt();
        if let Event::Key(event) = read()? {
            let command: Option<Command> = match event.code {
                KeyCode::Char('a') => {
                    clear_and_reset();
                    ask_for_status();
                    None
                }
                _ => None,
            } 
            let command = command else {
                continue;
            };
            run_command(command);
        }
    }
}

fn ask_for_status() {}

fn clear_and_reset() {
    let mut stdout = stdout();
    execute!(
        stdout,
        Clear(ClearType::All), // clear entire screen
        MoveTo(0, 0)           // move cursor to top-left corner
    )
    .unwrap();
}

fn print_command_prompt() {
    println!("Please enter the command initial letter to execute:");
    println!();
    println!("  [A]dd     - Add a new task with status and description");
    println!("  [D]elete  - Delete an existing task");
    println!("  [U]pdate  - Update a task");
    println!("  [L]ist    - List all tasks");
    println!();
    print!("Your choice: ");
}

fn run_command(command: Command) -> Result<(), Error> {
    match command {
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
            println!("{tasks:#?}");
        }
    }
    Ok(())
}
