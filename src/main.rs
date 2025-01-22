mod config;
mod project;
mod service;
mod description;

use clap::{Parser, Subcommand};
use config::{run as c, ConfigArgs};
use project::{run as p, ProjectArgs};

#[derive(Subcommand)]
enum Command {
    Project(ProjectArgs),
    Task,
    Config(ConfigArgs),
}

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Command::Config(v) => c(v),
        Command::Project(v) => p(v),
        Command::Task => println!("Task Subcommands"),
    }
}
