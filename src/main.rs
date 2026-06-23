mod commands;

use crate::commands::push;
use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "goose", version = "0.1.0", about = "A CLI tool for devs")]
struct Args {
    #[clap(subcommand)]
    subcommand: Subc,
}

#[derive(Debug, Subcommand)]
enum Subc {
    // Push current changes  to remote branch
    Push {},
    Switch { branch: String },
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
            eprintln!("switch is not implemented yet: {branch}");
        }
        Subc::New { branch } => {
            eprintln!("new is not implemented yet: {branch}");
        }
    }
}
