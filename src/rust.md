# Rust tooling basics
### create exectuable named main
- `rustc main.rs`
    - run with `./main`

### create new project
- `cargo new project_name`

### build project
- `cargo build`
- loads new deps (defined in cargo.toml)

### build and run executable
- `cargo run`

### build for release 
- `cargo build --release`
    - puts executable in /target/release dir
