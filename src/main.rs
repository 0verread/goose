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
    if let Subc::Push {} = &args.subcommand {
        eprintln!("pushing to the head");
    }
}
