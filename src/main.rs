use clap::{Parser, Subcommand, ValueEnum};

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

#[derive(Debug)]
struct Task {
    status: TaskStatus,
    description: String,
}

#[derive(Debug, Clone, ValueEnum)]
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
        } => todo!(),
        Command::Delete => todo!(),
        Command::Update => todo!(),
        Command::List => todo!(),
    }
}
