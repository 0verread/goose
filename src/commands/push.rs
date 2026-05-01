// push contains four commands: git status, git add, and git commit, git push
// git commit asks the user for a commit message
// git push pushes the changes to the remote repository on the current branch

use colored::Colorize;
use inquire::Text;
use std::process::Command;

fn user_commit_msg() -> String {
    let commit_message = Text::new("")
        .with_placeholder("Write a commit message")
        .prompt()
        .unwrap_or_default();
    commit_message
}

fn git_add_all() {
    Command::new("git")
        .args(["add", "."])
        .output()
        .expect("Failed to add all files");
}

fn git_commit(commit_msg: &str) {
    Command::new("git")
        .args(["commit", "-m", commit_msg])
        .output()
        .expect("Failed to commit");
}

fn git_push_head() {
    Command::new("git")
        .args(["push", "origin", "HEAD"])
        .output()
        .expect("Failed to push to remote");
}

pub fn run() {
    let pwd_output = Command::new("git")
        .args(["status", "--porcelain"])
        .output()
        .expect("");
    let current_br = Command::new("git")
        .args(["branch", "--show-current"])
        .output()
        .expect("");
    eprint!(
        "Pushing on active branch: {}",
        String::from_utf8_lossy(&current_br.stdout).blue()
    );
    eprintln!("{}", String::from_utf8_lossy(&pwd_output.stdout));
    let user_commit_msg = user_commit_msg();
    git_add_all();
    git_commit(&user_commit_msg);
    git_push_head();
}
