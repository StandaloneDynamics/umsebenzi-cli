mod config;
mod defaults;
mod description;
mod enums;
mod project;
mod request;
mod response;
mod service;
mod task;

use clap::{Parser, Subcommand};
use config::{run as c, ConfigArgs};
use project::{run as p, ProjectArgs};
use task::{run as t, TaskArgs};

#[derive(Subcommand)]
enum Command {
    Project(ProjectArgs),
    Task(TaskArgs),
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
        Command::Task(v) => t(v),
    }
}
