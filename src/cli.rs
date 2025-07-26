mod message_handler;
use crate::cli::message_handler::*;
use anyhow::Error;
use clap::{Parser, Subcommand, ValueEnum};
use crossterm::event::{Event, KeyCode, KeyEvent, read};
use serde::{Deserialize, Serialize};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Command>,
}

pub trait CliInteraction {
    fn loop_for_commands(&self);
    fn run_command(&self, command: &Command) -> Result<(), Error>;
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

impl CliInteraction for Cli {
    fn loop_for_commands(&self) {
        loop {
            clear_and_reset();
            present_commands_prompt();
            if let Ok(Event::Key(key_event)) = read() {
                let Some(command) = command_for_event(key_event) else {
                    continue;
                };
                let _ = self.run_command(&command);
            } else {
                eprintln!("Error when reading event.")
            }
        }
    }

    fn run_command(&self, command: &Command) -> Result<(), Error> {
        todo!()
    }
}

fn command_for_event(event: KeyEvent) -> Option<Command> {
    match event.code {
        KeyCode::Char('a') => {
            clear_and_reset();
            ask_for_status();
            None
        }
        _ => None,
    }
}
