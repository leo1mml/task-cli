mod message_handler;

use crate::{cli::message_handler::*, models::Task, storage::write_tasks};
use anyhow::Error;
use clap::{Parser, Subcommand, ValueEnum};
use crossterm::{
    event::{Event, KeyCode, KeyEvent, KeyEventKind, read},
    terminal::{disable_raw_mode, enable_raw_mode, is_raw_mode_enabled},
};
use serde::{Deserialize, Serialize};
use std::io::stdin;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Command>,
}

impl CliInteraction for Cli {
    fn loop_for_commands(&self) {
        loop {
            self.read_command()
                .expect("Could not read the given command");
        }
    }

    fn run_command(&self, command: Command) -> Result<(), Error> {
        match command {
            Command::Add {
                status,
                description,
            } => {
                let task = Task::new(status, description);
                write_tasks(task)
            }
            Command::Delete => todo!(),
            Command::Update => todo!(),
            Command::List => todo!(),
        }
    }
}

impl Cli {
    fn read_command(&self) -> Result<(), Error> {
        clear_and_reset();
        present_commands_prompt();
        loop {
            enable_raw_mode()?;
            match read() {
                Ok(Event::Key(key_event)) => {
                    let Some(command) = self.command_for_event(key_event) else {
                        break;
                    };
                    let _ = self.run_command(command);
                }
                _ => eprintln!("Error when reading event."),
            }
            disable_raw_mode()?;
        }
        Ok(())
    }
    fn command_for_event(&self, event: KeyEvent) -> Option<Command> {
        match event.code {
            KeyCode::Char(c) => match c.to_ascii_lowercase() {
                'a' => self.make_add_command(),
                _ => None,
            },
            _ => None,
        }
    }

    fn make_add_command(&self) -> Option<Command> {
        clear_and_reset();
        ask_for_status();
        let input = stdin();
        let mut status = String::new();
        let _ = input.read_line(&mut status);
        let Some(status) = self.status_from_str(status) else {
            print_invalid_value();
            _ = wait_for_any_key();
            clear_and_reset();
            return None;
        };
        let mut description = String::new();
        _ = input.read_line(&mut description);
        Some(Command::Add {
            status,
            description,
        })
    }

    fn status_from_str(&self, status: String) -> Option<TaskStatus> {
        todo!()
    }
}

pub fn wait_for_any_key() -> Result<(), Error> {
    // Enable raw mode to get immediate key presses.
    // This also disables echoing characters and special terminal processing.
    if !is_raw_mode_enabled()? {
        enable_raw_mode()?;
    }

    println!("\nPress any key to continue...");

    // Loop indefinitely until a key press event is received.
    loop {
        // `event::read()` blocks the current thread until an event is available.
        if let Event::Key(key_event) = read()? {
            // We are specifically looking for a key *press* event.
            // This distinguishes it from key release or key repeat events.
            if key_event.kind == KeyEventKind::Press {
                // Print the detected key for feedback (optional).
                println!("Key pressed: {:?}\r", key_event.code);
                break; // Exit the loop as a key was pressed.
            }
        }
    }

    // Always remember to disable raw mode to restore normal terminal behavior.
    disable_raw_mode()?;

    Ok(())
}

pub trait CliInteraction {
    fn loop_for_commands(&self);
    fn run_command(&self, command: Command) -> Result<(), Error>;
}

#[derive(Debug, Subcommand)]
pub enum Command {
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

#[derive(Debug, Clone, ValueEnum, Serialize, Deserialize)]
#[clap(rename_all = "lower")]
pub enum TaskStatus {
    Todo,
    InProgress,
    Done,
}
