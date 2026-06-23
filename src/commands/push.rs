use std::fmt;
use std::io::{self, Write};
use std::process::{Command, ExitStatus};

const COLOR_RESET: &str = "\x1b[0m";
const COLOR_GREEN: &str = "\x1b[32m";
const COLOR_YELLOW: &str = "\x1b[33m";
const COLOR_RED: &str = "\x1b[31m";
const COLOR_CYAN: &str = "\x1b[36m";
const COLOR_MAGENTA: &str = "\x1b[35m";

#[derive(Debug)]
pub enum PushError {
    EmptyCommitMessage,
    Git {
        args: Vec<String>,
        status: ExitStatus,
        stderr: String,
    },
    Io(io::Error),
}

impl fmt::Display for PushError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PushError::EmptyCommitMessage => write!(formatter, "commit message cannot be empty"),
            PushError::Git {
                args,
                status,
                stderr,
            } => {
                write!(formatter, "git {} failed with {status}", args.join(" "))?;
                if !stderr.trim().is_empty() {
                    write!(formatter, ": {}", stderr.trim())?;
                }
                Ok(())
            }
            PushError::Io(err) => write!(formatter, "{err}"),
        }
    }
}

impl std::error::Error for PushError {}

impl From<io::Error> for PushError {
    fn from(err: io::Error) -> Self {
        PushError::Io(err)
    }
}

#[derive(Debug, PartialEq, Eq)]
enum ChangeKind {
    Added,
    Modified,
    Deleted,
    Renamed,
    Copied,
    Untracked,
    Other,
}

impl ChangeKind {
    fn color(&self) -> &'static str {
        match self {
            ChangeKind::Added => COLOR_GREEN,
            ChangeKind::Modified => COLOR_YELLOW,
            ChangeKind::Deleted => COLOR_RED,
            ChangeKind::Renamed => COLOR_CYAN,
            ChangeKind::Copied => COLOR_CYAN,
            ChangeKind::Untracked => COLOR_MAGENTA,
            ChangeKind::Other => COLOR_RESET,
        }
    }
}

fn change_kind(status_line: &str) -> ChangeKind {
    let status = status_line.get(..2).unwrap_or(status_line);

    if status == "??" {
        return ChangeKind::Untracked;
    }

    if status.contains('R') {
        ChangeKind::Renamed
    } else if status.contains('C') {
        ChangeKind::Copied
    } else if status.contains('A') {
        ChangeKind::Added
    } else if status.contains('D') {
        ChangeKind::Deleted
    } else if status.contains('M') {
        ChangeKind::Modified
    } else {
        ChangeKind::Other
    }
}

pub fn print_colored_status(status: &str) -> Result<(), PushError> {
    let stderr = io::stderr();
    let mut stderr = stderr.lock();
    write_colored_status(&mut stderr, status)?;
    Ok(())
}

fn write_colored_status(writer: &mut impl Write, status: &str) -> io::Result<()> {
    for line in status.lines() {
        let kind = change_kind(line);
        writeln!(writer, "{}{}{}", kind.color(), line, COLOR_RESET)?;
    }

    Ok(())
}

fn git_output(args: &[&str]) -> Result<String, PushError> {
    let output = Command::new("git").args(args).output()?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        Err(PushError::Git {
            args: args.iter().map(|arg| (*arg).to_string()).collect(),
            status: output.status,
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
        })
    }
}

fn git_status(args: &[&str]) -> Result<(), PushError> {
    let status = Command::new("git").args(args).status()?;

    if status.success() {
        Ok(())
    } else {
        Err(PushError::Git {
            args: args.iter().map(|arg| (*arg).to_string()).collect(),
            status,
            stderr: String::new(),
        })
    }
}

fn read_commit_message() -> Result<String, PushError> {
    eprint!("Commit message: ");
    io::stderr().flush()?;

    let mut commit_message = String::new();
    io::stdin().read_line(&mut commit_message)?;
    let commit_message = commit_message.trim().to_string();

    if commit_message.is_empty() {
        Err(PushError::EmptyCommitMessage)
    } else {
        Ok(commit_message)
    }
}

pub fn run() -> Result<(), PushError> {
    let branch = git_output(&["branch", "--show-current"])?;
    let status = git_output(&["status", "--porcelain"])?;

    eprintln!("Pushing on branch: {}", branch.trim());

    if status.trim().is_empty() {
        eprintln!("No working tree changes to commit.");
    } else {
        print_colored_status(&status)?;

        let diff_stat = git_output(&["diff", "--shortstat", "HEAD"])?;
        if !diff_stat.trim().is_empty() {
            eprintln!("{}", diff_stat.trim());
        }

        let commit_message = read_commit_message()?;
        git_status(&["add", "."])?;
        git_status(&["commit", "-m", &commit_message])?;
    }

    git_status(&["push", "origin", "HEAD"])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_change_kind_from_porcelain_status() {
        assert_eq!(change_kind(" M src/main.rs"), ChangeKind::Modified);
        assert_eq!(change_kind("A  src/main.rs"), ChangeKind::Added);
        assert_eq!(change_kind(" D src/main.rs"), ChangeKind::Deleted);
        assert_eq!(change_kind("R  old.rs -> new.rs"), ChangeKind::Renamed);
        assert_eq!(change_kind("C  old.rs -> new.rs"), ChangeKind::Copied);
        assert_eq!(change_kind("?? src/main.rs"), ChangeKind::Untracked);
    }

    #[test]
    fn prints_colored_file_status_lines() {
        let mut output = Vec::new();

        write_colored_status(&mut output, " M src/main.rs\n?? README.md").unwrap();

        let output = String::from_utf8(output).unwrap();
        assert!(output.contains("\x1b[33m M src/main.rs\x1b[0m"));
        assert!(output.contains("\x1b[35m?? README.md\x1b[0m"));
    }
}
