# Goose

Goose is a small CLI tool for developers that wraps common Git workflow commands.

## Features

- `goose push`
  - Push current changes to the remote branch.
  - Intended workflow: `git add .`, commit with a user-provided message, then `git push origin HEAD`.
- `goose switch <branch>`
  - Switch back to `main`, then switch to the target branch.
  - Intended workflow: `git checkout main`, then `git checkout <branch>`.
- `goose new <branch>`
  - Create a new branch from the latest `main`.
  - Intended workflow: `git checkout main`, `git pull origin main`, then `git checkout -b <branch>`.

## Git Workflow

- Create a feature branch with `goose new <branch>`.
- Move between existing branches with `goose switch <branch>`.
- Commit and push current work with `goose push`.

## License

This project is under the [MIT License](./LICENSE), so feel free to make it your own.
