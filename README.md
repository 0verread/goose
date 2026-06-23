# Goose

Goose is a small developer CLI for wrapping common Git workflows behind short,
memorable commands.

The current codebase is a Rust CLI scaffold using `clap` for command parsing.
It defines the core command surface and has placeholder behavior for `push`
while the Git operations are still being implemented.

## Current Status

- Migrated the project from the earlier Go/Bubble Tea prototype to Rust.
- Added a Cargo project with Rust 2024 edition support.
- Added `clap` with derive support for CLI parsing.
- Defined the `goose` binary metadata: name, version, and description.
- Added subcommands for the intended Git workflow:
  - `push`
  - `switch <branch>`
  - `new <branch>`
- Added an initial command module at `src/commands/push.rs`.
- Added an MIT license.
- Added `.gitignore` and checked in `Cargo.lock` for reproducible builds.

## Intended Git Workflow

The planned shorthand commands are:

```sh
gs push
```

Stage all changes, create a commit from user input, and push the current HEAD to
the remote branch.

```sh
gs switch <branch_name>
```

Return to `main`, then switch to the requested branch.

```sh
gs new <branch_name>
```

Return to `main`, pull the latest `main` from origin, then create and switch to
a new branch.

## Implemented Behavior

At the moment, the CLI parses these commands:

```sh
cargo run -- push
cargo run -- switch <branch_name>
cargo run -- new <branch_name>
```

`push` currently prints a placeholder message:

```text
pushing to the head
```

`switch` and `new` are parsed but do not yet execute Git operations.

## Project Layout

```text
.
|-- Cargo.toml
|-- Cargo.lock
|-- LICENSE
|-- README.md
`-- src
    |-- main.rs
    `-- commands
        `-- push.rs
```

## Development

Build the CLI:

```sh
cargo build
```

Run the CLI:

```sh
cargo run -- <command>
```

Format the code:

```sh
cargo fmt
```

Check the code:

```sh
cargo check
```

## Work History

The project started as a Go prototype with a Bubble Tea terminal UI. That work
included list rendering, input box UI, checklist support, and modularized UI
components. The current branch later migrated the project to Rust and replaced
the previous Go code with the present `clap`-based CLI architecture.

## License

This project is licensed under the MIT License. See [LICENSE](./LICENSE).
