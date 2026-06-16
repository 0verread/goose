# Commands

Goose installs a binary named `gs`.

## `gs push`

Runs a guided commit-and-push workflow for the current Git repository.

Current behavior:

1. Shows the active branch.
2. Prints `git status --porcelain`.
3. Prints `git diff --shortstat`.
4. Prompts for a commit message.
5. Runs `git add .`.
6. Runs `git commit -m <message>`.
7. Runs `git push origin HEAD`.

Planned improvements include colored changed-file output, line-count totals,
selected-file commits, and generated commit messages.

## `gs switch <branch>`

Planned command for switching to an existing branch.

## `gs new <branch>`

Planned command for creating a branch from `main`.

