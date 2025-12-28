mod message_handler;

use crate::{
    models::{Task, TaskStatus},
    storage,
};
use anyhow::Error;
use clap::{Parser, Subcommand};
use crossterm::{
    event::{Event, KeyCode, KeyEventKind, read},
    terminal::{disable_raw_mode, enable_raw_mode, is_raw_mode_enabled},
};
use std::io::stdin;
use std::str::FromStr;

pub trait CliInteraction {
    fn loop_for_commands<T: storage::TaskStorage>(&self, task_storage: &T);
    fn run_command<T: storage::TaskStorage>(
        &self,
        command: Command,
        task_storage: &T,
    ) -> Result<(), Error>;
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    Add {
        #[arg(short, long)]
        status: TaskStatus,
        #[arg(short, long)]
        description: String,
    },
    Delete {
        #[arg(long)]
        id: String,
    },
    Update {
        #[arg(long)]
        id: String,
        #[arg(short, long)]
        status: TaskStatus,
        #[arg(short, long)]
        description: String,
    },
    List,
}

impl CliInteraction for Cli {
    fn loop_for_commands<T: storage::TaskStorage>(&self, task_storage: &T) {
        loop {
            match self.read_command() {
                Ok(command) => {
                    if let Err(error) = self.run_command(command, task_storage) {
                        return eprintln!("{error:?}");
                    }
                    _ = self.listen_for_key();
                    continue;
                }
                Err(error) => match self.listen_for_key_on_error(error) {
                    Ok(key) => {
                        if key.eq_ignore_ascii_case(&'q') {
                            break;
                        }
                    }
                    Err(error) => {
                        eprintln!("{}", error);
                        break;
                    }
                },
            }
        }
    }

    fn run_command<T: storage::TaskStorage>(
        &self,
        command: Command,
        task_storage: &T,
    ) -> Result<(), Error> {
        match command {
            Command::Add {
                status,
                description,
            } => {
                let task = Task::new(status, description);
                task_storage.write_task(task)
            }
            Command::Delete { id } => task_storage.remove_task(&id),
            Command::Update {
                id,
                status,
                description,
            } => task_storage.update_task(&id, status, &description),
            Command::List => {
                let tasks = task_storage.load_tasks();
                println!("{tasks:#?}");
                Ok(())
            }
        }
    }
}

impl Cli {
    fn listen_for_key_on_error(&self, error: Error) -> Result<char, Error> {
        println!("There has been an error.");
        eprintln! {"{error}"};
        println!("Press Q to quit. Or any key to restart");
        self.listen_for_key()
    }
    fn read_command(&self) -> Result<Command, Error> {
        message_handler::clear_and_reset();
        message_handler::present_commands_prompt();
        let code = self.listen_for_key()?.to_ascii_lowercase();
        let Some(command) = self.command_for_event(code) else {
            let error = anyhow::anyhow!("Key not supported");
            return Err(error);
        };
        Ok(command)
    }

    fn listen_for_key(&self) -> Result<char, Error> {
        loop {
            enable_raw_mode()?;
            let Ok(Event::Key(event)) = read() else {
                _ = disable_raw_mode();
                continue;
            };
            let KeyCode::Char(character) = event.code else {
                _ = disable_raw_mode();
                continue;
            };
            _ = disable_raw_mode();
            return Ok(character);
        }
    }

    fn command_for_event(&self, code: char) -> Option<Command> {
        match code {
            'a' => self.make_add_command(),
            'l' => Some(Command::List),
            'd' => todo!(),
            'u' => todo!(),
            _ => None,
        }
    }

    fn make_add_command(&self) -> Option<Command> {
        message_handler::clear_and_reset();
        message_handler::ask_for_status();
        let status = match self.listen_for_key() {
            Ok(code) => TaskStatus::from_str(&code.to_string()).ok(),
            Err(_) => {
                message_handler::print_invalid_value();
                _ = wait_for_any_key();
                message_handler::clear_and_reset();
                return None;
            }
        }?;

        message_handler::clear_and_reset();
        message_handler::ask_for_description();

        let input = stdin();
        let mut description = String::new();
        _ = input.read_line(&mut description);
        Some(Command::Add {
            status,
            description,
        })
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
