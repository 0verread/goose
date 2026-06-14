// push: git status (colored), git add, git commit, git push
// shows active branch, colorized file list (added/modified/deleted/untracked),
// diff line-change summary, then prompts for a commit message.

use colored::Colorize;
use inquire::Text;
use std::process::Command;

fn user_commit_msg() -> String {
    Text::new("")
        .with_placeholder("Write a commit message")
        .prompt()
        .unwrap_or_default()
}

/// Print each file from `git status --porcelain` with color-coded status codes.
fn print_colored_status(porcelain: &str) {
    for line in porcelain.lines() {
        if line.len() < 3 {
            continue;
        }
        // porcelain format: XY filename  (X = index, Y = worktree)
        let xy = &line[..2];
        let file = line[3..].trim();

        // Derive a human-readable label + color from the status code.
        let colored_line = match xy.trim() {
            "A" | "AM" => format!("  {} {}", "added    ".green().bold(), file.green()),
            "M" | "MM" | " M" => format!("  {} {}", "modified ".yellow().bold(), file.yellow()),
            "D" | " D" => format!("  {} {}", "deleted  ".red().bold(), file.red()),
            "R" | "RM" => format!("  {} {}", "renamed  ".cyan().bold(), file.cyan()),
            "C" | "CM" => format!("  {} {}", "copied   ".cyan().bold(), file.cyan()),
            "??" => format!("  {} {}", "untracked".dimmed(), file.dimmed()),
            "!!" => continue, // ignored files — skip
            _ => format!("  {} {}", xy.normal(), file),
        };
        eprintln!("{}", colored_line);
    }
}

/// Print total lines added / deleted from `git diff --shortstat` (staged + unstaged).
fn print_diff_summary() {
    // unstaged changes
    let unstaged = Command::new("git")
        .args(["diff", "--shortstat"])
        .output()
        .expect("Failed to get diff stat");
    // staged changes
    let staged = Command::new("git")
        .args(["diff", "--cached", "--shortstat"])
        .output()
        .expect("Failed to get cached diff stat");

    let unstaged_str = String::from_utf8_lossy(&unstaged.stdout);
    let staged_str = String::from_utf8_lossy(&staged.stdout);

    // Parse and aggregate lines added/deleted from both outputs.
    let (mut added, mut deleted) = (0i64, 0i64);
    for text in [unstaged_str.as_ref(), staged_str.as_ref()] {
        for part in text.split(',') {
            let part = part.trim();
            if part.contains("insertion") {
                if let Some(n) = part.split_whitespace().next().and_then(|s| s.parse::<i64>().ok()) {
                    added += n;
                }
            } else if part.contains("deletion") {
                if let Some(n) = part.split_whitespace().next().and_then(|s| s.parse::<i64>().ok()) {
                    deleted += n;
                }
            }
        }
    }

    if added > 0 || deleted > 0 {
        eprintln!(
            "  {} {} {}",
            format!("+{} lines", added).green().bold(),
            "/".dimmed(),
            format!("-{} lines", deleted).red().bold(),
        );
    }
}

fn git_add_all() {
    Command::new("git")
        .args(["add", "."])
        .output()
        .expect("Failed to add all files");
}

fn git_commit(commit_msg: &str) -> bool {
    let output = Command::new("git")
        .args(["commit", "-m", commit_msg])
        .output()
        .expect("Failed to commit");
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("{} {}", "Commit error:".red().bold(), stderr.trim());
        return false;
    }
    true
}

fn git_push_head() -> bool {
    let output = Command::new("git")
        .args(["push", "origin", "HEAD"])
        .output()
        .expect("Failed to push to remote");
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("{} {}", "Push error:".red().bold(), stderr.trim());
        return false;
    }
    true
}

pub fn run() {
    // Current branch
    let current_br = Command::new("git")
        .args(["branch", "--show-current"])
        .output()
        .expect("Failed to get current branch");
    let branch = String::from_utf8_lossy(&current_br.stdout);
    eprintln!("Pushing on branch: {}", branch.trim().blue().bold());

    // Colored file list
    let status_output = Command::new("git")
        .args(["status", "--porcelain"])
        .output()
        .expect("Failed to get git status");
    let porcelain = String::from_utf8_lossy(&status_output.stdout);

    if porcelain.trim().is_empty() {
        eprintln!("{}", "Nothing to commit — working tree clean.".dimmed());
        return;
    }

    print_colored_status(&porcelain);
    eprintln!();
    print_diff_summary();
    eprintln!();

    // Commit message
    let commit_msg = user_commit_msg();
    if commit_msg.trim().is_empty() {
        eprintln!("{}", "Aborted: empty commit message.".red());
        return;
    }

    git_add_all();
    if git_commit(&commit_msg) && git_push_head() {
        eprintln!("{} Pushed to {}", "✔".green().bold(), branch.trim().green().bold());
    }
}
