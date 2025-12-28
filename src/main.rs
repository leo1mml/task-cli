mod cli;
mod models;
mod storage;
mod utils;

use anyhow::Error;
use clap::Parser;
use cli::{Cli, CliInteraction};

const QUALIFIER: &str = "com";
const ORGANIZATION: &str = "leo1mml";
const APPLICATION: &str = "task-cli";
const DATA_FILE_NAME: &str = "tasks.json";

fn main() -> Result<(), Error> {
    let mut cli = Cli::parse();
    let task_storage = storage::FileStorage {
        qualifier: QUALIFIER.to_string(),
        organization: ORGANIZATION.to_string(),
        application: APPLICATION.to_string(),
        data_file_name: DATA_FILE_NAME.to_string(),
    };

    if let Some(command) = cli.command.take() {
        cli.run_command(command, &task_storage)
    } else {
        cli.loop_for_commands(&task_storage);
        Ok(())
    }
}
