use clap::{Parser, Subcommand};

mod commands;

#[derive(Debug, Parser)]
#[command(name = "goose", version = "0.1.0", about = "A CLI tool for devs")]
struct Args {
    #[clap(subcommand)]
    subcommand: Subc,
}

#[derive(Debug, Subcommand)]
enum Subc {
    // Push current changes  to remote branch
    Push {
        #[arg(required = true, trailing_var_arg = true)]
        message: Vec<String>,
    },
    Switch {
        branch: String,
    },
    New {
        branch: String,
    },
}

fn main() {
    let args = Args::parse();

    let result = match &args.subcommand {
        Subc::Push { message } => commands::push::run(&message.join(" ")),
        Subc::Switch { branch } => {
            eprintln!("switch is not implemented yet: {branch}");
            Ok(())
        }
        Subc::New { branch } => {
            eprintln!("new is not implemented yet: {branch}");
            Ok(())
        }
    };

    if let Err(error) = result {
        eprintln!("{error}");
        std::process::exit(1);
    }
}
