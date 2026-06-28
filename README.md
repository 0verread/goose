### Goose

A small Rust CLI for wrapping common git workflows.

#### Usage

```sh
cargo run -- push "commit message"
```

`push` runs:

- `git add .`
- `git commit -m "<commit message>"`
- `git push origin HEAD`

#### Planned Git Workflows

- `switch <branch_name>` -> `git checkout main` + `git checkout <branch_name>`
- `new <branch_name>` -> `git checkout main` + `git pull origin main` + `git checkout -b <branch_name>`

#### License

This project is under the [MIT License](./LICENSE).
