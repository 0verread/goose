// switch: git fetch --all --prune, git switch <branch>
// shows current branch before switching, confirms success after

use colored::Colorize;
use std::process::Command;

fn get_current_branch() -> String {
    let output = Command::new("git")
        .args(["branch", "--show-current"])
        .output()
        .expect("Failed to get current branch");
    String::from_utf8_lossy(&output.stdout).trim().to_string()
}

fn git_fetch() {
    Command::new("git")
        .args(["fetch", "--all", "--prune"])
        .output()
        .expect("Failed to fetch from remote");
}

fn git_switch(branch: &str) -> bool {
    let output = Command::new("git")
        .args(["switch", branch])
        .output()
        .expect("Failed to run git switch");
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("{} {}", "Error:".red().bold(), stderr.trim());
        return false;
    }
    true
}

pub fn run(branch: &str) {
    let from = get_current_branch();
    eprintln!("Switching from {} → {}", from.yellow(), branch.blue().bold());

    eprintln!("{}", "Fetching latest refs...".dimmed());
    git_fetch();

    if git_switch(branch) {
        let now = get_current_branch();
        eprintln!("{} Now on branch {}", "✔".green().bold(), now.green().bold());
    }
}
