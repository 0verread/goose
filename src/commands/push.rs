// push contains four commands: git status, git add, and git commit, git push
// git commit asks the user for a commit message
// git push pushes the changes to the remote repository on the current branch

use colored::Colorize;
use inquire::Text;
use std::process::Command;

#[derive(Debug, PartialEq, Eq)]
enum ChangeKind {
    Added,
    Modified,
    Deleted,
    Other,
}

fn user_commit_msg() -> String {
    let commit_message = Text::new("")
        .with_placeholder("Write a commit message")
        .prompt()
        .unwrap_or_default();
    commit_message
}

fn git_diff_stat() {
    let diff_output = Command::new("git")
        .args(["diff", "--shortstat"])
        .output()
        .expect("Failed to get diff stat");
    eprintln!("{}", String::from_utf8_lossy(&diff_output.stdout));
}

fn change_kind(status_line: &str) -> ChangeKind {
    let status = status_line.get(..2).unwrap_or(status_line);

    if status.contains('D') {
        ChangeKind::Deleted
    } else if status.contains('A') || status.contains('?') {
        ChangeKind::Added
    } else if status.contains('M') {
        ChangeKind::Modified
    } else {
        ChangeKind::Other
    }
}

fn print_colored_status(status_output: &[u8]) {
    let status = String::from_utf8_lossy(status_output);

    if status.trim().is_empty() {
        eprintln!("{}", "No file changes found.".dimmed());
        return;
    }

    for line in status.lines() {
        match change_kind(line) {
            ChangeKind::Added => eprintln!("{}", line.green()),
            ChangeKind::Modified => eprintln!("{}", line.yellow()),
            ChangeKind::Deleted => eprintln!("{}", line.red()),
            ChangeKind::Other => eprintln!("{}", line),
        }
    }
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
    let status_output = Command::new("git")
        .args(["status", "--porcelain"])
        .output()
        .expect("");
    let current_br = Command::new("git")
        .args(["branch", "--show-current"])
        .output()
        .expect("");
    eprint!(
        "Pushing on branch: {}",
        String::from_utf8_lossy(&current_br.stdout).blue()
    );
    print_colored_status(&status_output.stdout);
    git_diff_stat();
    let user_commit_msg = user_commit_msg();
    git_add_all();
    git_commit(&user_commit_msg);
    git_push_head();
}

#[cfg(test)]
mod tests {
    use super::{ChangeKind, change_kind};

    #[test]
    fn detects_added_files() {
        assert_eq!(change_kind("A  src/main.rs"), ChangeKind::Added);
        assert_eq!(change_kind("?? src/main.rs"), ChangeKind::Added);
    }

    #[test]
    fn detects_modified_files() {
        assert_eq!(change_kind(" M src/main.rs"), ChangeKind::Modified);
        assert_eq!(change_kind("M  src/main.rs"), ChangeKind::Modified);
    }

    #[test]
    fn detects_deleted_files() {
        assert_eq!(change_kind(" D src/main.rs"), ChangeKind::Deleted);
        assert_eq!(change_kind("D  src/main.rs"), ChangeKind::Deleted);
    }
}
