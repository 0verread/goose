use std::process::Command;

pub fn run(message: &str) -> Result<(), String> {
    run_git(&["add", "."])?;
    run_git(&["commit", "-m", message])?;
    run_git(&["push", "origin", "HEAD"])
}

fn run_git(args: &[&str]) -> Result<(), String> {
    let status = Command::new("git")
        .args(args)
        .status()
        .map_err(|error| format!("failed to run git {}: {error}", args.join(" ")))?;

    if status.success() {
        Ok(())
    } else {
        Err(format!(
            "git {} failed with status {status}",
            args.join(" ")
        ))
    }
}
