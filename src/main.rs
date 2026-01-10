use anyhow::Error;
use task_cli::{
    self,
    cli::{Cli, CliInteraction},
    storage,
};

fn main() -> Result<(), Error> {
    let mut cli = Cli::initialize();
    let task_storage = storage::FileStorage::new_default();
    cli.run_or_loop(&task_storage)
}
