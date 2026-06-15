### Goose

A better minimalistic way to manage your git workflow + some intesting feature

#### Run locally

Prerequisites:

- Rust and Cargo installed. If needed, install them from [rustup.rs](https://rustup.rs/).
- Git installed and available on your `PATH`.

Clone the repository and enter the project directory:

```sh
git clone <repository-url>
cd goose
```

Build the CLI:

```sh
cargo build
```

Run the CLI locally with Cargo:

```sh
cargo run -- push
```

The binary is named `gs`. To install it locally from this checkout:

```sh
cargo install --path .
gs push
```

#### Git Workflow

**gs push**

- [x] ask user  for commit message
- [x] show active branch / where changes are going to be pushed
- [ ] colored modified, added, deeleted files
- [ ] get total lines added, deleted
- [ ] store analytics, local sqldb
- [ ] AI to generate commit messages based on current changes
- [ ] ability to push all files or some

**gs liens**

shows total lines in current dir/files, respect .giotignore. features: TBD

**gs switch <branch_name>**

TBD

**gs new <brnach_name>**

create a new branch with branch_name on main: TBD


#### LICESNSE
This project is under [MIT LICESNSE](./LICESNSE)
