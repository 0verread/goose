mod commands;

use crate::commands::push;
use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(
    name = "goose",
    version = "0.1.0",
    about = "minimalistic git workflow management tool"
)]
struct Args {
    #[clap(subcommand)]
    subcommand: Subc,
}

#[derive(Debug, Subcommand)]
enum Subc {
    /// Stage, commit, and push current changes to remote branch
    Push {},
    /// Switch to an existing branch
    Switch { branch: String },
    /// Create a new branch off main and switch to it
    New { branch: String },
}

fn main() {
    let args = Args::parse();
    match &args.subcommand {
        Subc::Push {} => commands::push::run(),
        Subc::Switch { branch } => commands::switch::run(branch),
        Subc::New { branch } => commands::new::run(branch),
        Subc::Push {} => {
            if let Err(err) = push::run() {
                eprintln!("error: {err}");
                std::process::exit(1);
            }
        }
        Subc::Switch { branch } => {
            eprintln!("switch is not implemented yet: {branch}");
        }
        Subc::New { branch } => {
            eprintln!("new is not implemented yet: {branch}");
        }
    }
}
