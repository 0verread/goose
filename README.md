# Goose

A better minimalistic way to manage your git workflow + some intesting feature

The current codebase is a Rust CLI scaffold using `clap` for command parsing.
It defines the core command surface and has placeholder behavior for `push`
while the Git operations are still being implemented.

**gs push**

- [x] ask user  for commit message
- [x] show active branch / where changes are going to be pushed
- [x] colored modified, added, deleted files
- [ ] get total lines added, deleted
- [ ] store analytics, local sqldb
- [ ] AI to generate commit messages based on current changes
- [ ] ability to push all files or some

**gs liens**

shows total lines in current dir/files, respect .giotignore. features: TBD

**gs switch <branch_name>**

TBD

**gs new <branch_name>**

- [x] fetch latest main from origin
- [x] create and switch to new branch off of main

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

#### LICESNSE
This project is under [MIT LICESNSE](./LICESNSE)
