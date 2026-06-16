# Goose

Goose is a small Rust CLI for streamlining common Git workflow tasks. The
binary is installed as `gs`.

The project is early-stage. Today, `gs push` is the main implemented command:
it shows the current branch and working tree status, asks for a commit message,
stages all changes, commits them, and pushes `HEAD` to `origin`.

## Installation

Build the CLI from source:

```sh
cargo build --release
```

Run it locally during development:

```sh
cargo run -- push
```

Install it into your Cargo bin directory:

```sh
cargo install --path .
```

After installation, use the `gs` command:

```sh
gs push
```

## Commands

### `gs push`

Runs a guided commit-and-push flow for the current Git repository:

1. Prints the active branch.
2. Prints the current `git status --porcelain` output.
3. Prints the current diff shortstat.
4. Prompts for a commit message.
5. Runs `git add .`.
6. Runs `git commit -m <message>`.
7. Runs `git push origin HEAD`.

Use this when you want to quickly stage all current changes, commit them, and
push the current branch.

### `gs switch <branch>`

Planned. This command is intended to switch to an existing branch.

### `gs new <branch>`

Planned. This command is intended to create a new branch from `main`.

## Roadmap

- Improve changed-file display with color-coded added, modified, and deleted
  files.
- Show total lines added and deleted before committing.
- Support committing a selected subset of files.
- Generate commit messages from the current diff.
- Store local workflow analytics in SQLite.
- Add a line-counting command that respects `.gitignore`.
- Implement branch switching and branch creation commands.

## Development

Format and check the project with Cargo:

```sh
cargo fmt
cargo check
```

## License

This project is licensed under the [MIT License](./LICENSE).
