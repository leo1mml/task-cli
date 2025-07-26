mod cli;
mod models;
mod storage;
mod utils;

use anyhow::Error;
use clap::Parser;
use cli::{Cli, CliInteraction};

fn main() -> Result<(), Error> {
    let cli = Cli::parse();

    if let Some(ref command) = cli.command {
        cli.run_command(command)
    } else {
        cli.loop_for_commands();
        Ok(())
    }
}
