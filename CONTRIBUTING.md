# Contributing

Goose is an early-stage Rust CLI. Keep changes small, focused, and easy to
verify from the command line.

## Development

Run these checks before opening a change:

```sh
cargo fmt
cargo check
```

For manual testing, run the CLI from the repository root:

```sh
cargo run -- push
```

## Pull Requests

- Describe the workflow or command being changed.
- Include the commands you used to verify the change.
- Update `README.md` or files in `docs/` when user-facing behavior changes.

