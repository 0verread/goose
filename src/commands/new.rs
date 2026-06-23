// new creates a new branch off of main and switches to it

use colored::Colorize;
use std::process::Command;

fn git_fetch_main() {
    Command::new("git")
        .args(["fetch", "origin", "main"])
        .output()
        .expect("Failed to fetch main from origin");
}

fn git_create_branch(branch: &str) {
    let output = Command::new("git")
        .args(["checkout", "-b", branch, "origin/main"])
        .output()
        .expect("Failed to create new branch");

    if !output.status.success() {
        eprintln!(
            "{} {}",
            "Failed to create branch:".red(),
            String::from_utf8_lossy(&output.stderr).trim()
        );
        std::process::exit(1);
    }
}

pub fn run(branch: &str) {
    eprintln!("Fetching latest {} from origin...", "main".blue());
    git_fetch_main();

    eprintln!(
        "Creating and switching to new branch {} off of {}",
        branch.green(),
        "main".blue()
    );
    git_create_branch(branch);

    eprintln!("{} {}", "✓ Now on branch".green(), branch.green().bold());
}
