mod commands;

use crate::commands::{new, push, switch};
use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "goose", version = "0.1.0", about = "A CLI tool for devs")]
struct Args {
    #[clap(subcommand)]
    subcommand: Subc,
}

#[derive(Debug, Subcommand)]
enum Subc {
    /// Push current changes to the remote branch
    Push {},
    /// Switch to an existing branch
    Switch { branch: String },
    /// Create a new branch from the latest main
    New { branch: String },
}

fn main() {
    let args = Args::parse();
    match &args.subcommand {
        Subc::Push {} => {
            if let Err(err) = push::run() {
                eprintln!("error: {err}");
                std::process::exit(1);
            }
        }
        Subc::Switch { branch } => {
            switch::run(branch);
        }
        Subc::New { branch } => {
            new::run(branch);
        }
    }
}
