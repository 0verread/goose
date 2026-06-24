# Goose (`gs`)

A minimalistic, opinionated CLI tool to streamline your Git workflow. Written in Rust.

## Overview

Goose exposes a single binary — `gs` — that wraps common Git operations with a cleaner, more interactive interface. It shows you exactly what is changing, where it is going, and asks only what it needs before acting.

## Installation

```bash
cargo build --release
# then put the binary somewhere on your PATH
cp target/release/gs /usr/local/bin/gs
```

## Commands

### `gs push`

Stages all changes, prompts for a commit message, commits, and pushes to the current remote branch.

**What it does, step by step:**
1. Runs `git status --porcelain` and prints each changed file with color coding:
   - **Green** — added / untracked files
   - **Yellow** — modified files
   - **Red** — deleted files
2. Prints a short diff stat (`git diff --shortstat`) showing lines changed.
3. Shows the active branch name so you know exactly where you are pushing.
4. Prompts you interactively for a commit message.
5. Runs `git add .`, `git commit -m <message>`, and `git push origin HEAD`.

**Example:**
```
Pushing on branch: feature/my-feature
M  src/main.rs
A  src/commands/new.rs
 D src/old_file.rs

 2 files changed, 45 insertions(+), 12 deletions(-)

> Write a commit message: add new branch command
```

**Planned:**
- [ ] Show total lines added / deleted across all files
- [ ] Store push analytics in a local SQLite database
- [ ] AI-generated commit messages based on the current diff
- [ ] Ability to selectively stage and push individual files

---

### `gs new <branch_name>`

Creates a new branch off of the latest `origin/main` and switches to it immediately.

**What it does, step by step:**
1. Fetches the latest `main` from `origin`.
2. Creates a new branch from `origin/main` using `git checkout -b <branch_name> origin/main`.
3. Prints a confirmation with the new branch name.
4. Exits with a non-zero status code and a clear error message if branch creation fails.

**Example:**
```
Fetching latest main from origin...
Creating and switching to new branch feature/login off of main
✓ Now on branch feature/login
```

---

### `gs switch <branch_name>`

> **Status: TBD** — The subcommand is registered in the CLI but not yet implemented.

Switch to an existing branch.

---

### `gs liens`

> **Status: TBD** — Planned command, not yet implemented.

Shows the total number of lines across all files in the current directory, respecting `.gitignore`.

---

## Tech Stack

| Crate | Version | Purpose |
|---|---|---|
| [clap](https://crates.io/crates/clap) | 4.6.1 | CLI argument parsing (derive API) |
| [colored](https://crates.io/crates/colored) | 3.1.1 | Terminal color output |
| [inquire](https://crates.io/crates/inquire) | 0.9.4 | Interactive prompts (commit message input) |

Rust edition: **2024**

## Project Structure

```
goose/
├── src/
│   ├── main.rs           # CLI definition (clap), subcommand dispatch
│   ├── commands.rs       # Module declarations
│   └── commands/
│       ├── push.rs       # gs push implementation
│       └── new.rs        # gs new implementation
├── Cargo.toml
└── LICENSE               # MIT
```

## Running Tests

```bash
cargo test
```

Unit tests in `push.rs` cover file change kind detection (added, modified, deleted).

## License

This project is under the [MIT License](./LICENSE).
