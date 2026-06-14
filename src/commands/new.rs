// new creates a fresh branch from main (or master), then switches to it

use colored::Colorize;
use std::process::Command;

fn default_base() -> String {
    // prefer "main", fall back to "master"
    let output = Command::new("git")
        .args(["branch", "--list", "main"])
        .output()
        .expect("Failed to list branches");
    let out = String::from_utf8_lossy(&output.stdout);
    if out.trim().is_empty() {
        "master".to_string()
    } else {
        "main".to_string()
    }
}

fn git_fetch() {
    Command::new("git")
        .args(["fetch", "--all", "--prune"])
        .output()
        .expect("Failed to fetch from remote");
}

fn git_checkout_new(branch: &str, base: &str) -> bool {
    // try origin/<base> first so we get the freshest commit
    let remote_base = format!("origin/{}", base);
    let output = Command::new("git")
        .args(["checkout", "-b", branch, &remote_base])
        .output()
        .expect("Failed to run git checkout");
    if output.status.success() {
        return true;
    }
    // fall back to local base
    let output = Command::new("git")
        .args(["checkout", "-b", branch, base])
        .output()
        .expect("Failed to run git checkout (local base)");
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("{} {}", "Error:".red().bold(), stderr.trim());
        return false;
    }
    true
}

pub fn run(branch: &str) {
    let base = default_base();
    eprintln!(
        "Creating {} from {}",
        branch.blue().bold(),
        base.yellow()
    );

    eprintln!("{}", "Fetching latest refs...".dimmed());
    git_fetch();

    if git_checkout_new(branch, &base) {
        eprintln!(
            "{} Branch {} created and checked out",
            "✔".green().bold(),
            branch.green().bold()
        );
    }
}
