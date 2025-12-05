# Contributing to GhostKeys

We love your input! We want to make contributing to GhostKeys as easy and transparent as possible.

## Development Setup

### Option 1: VS Code DevContainer (Recommended for Linux/Mac users)

_(We use [DevMagic](https://devmagic.run) as the source of truth for our devcontainer environment.)_

1. Open the project in VS Code.
2. Click "Reopen in Container".
3. Use `cargo xwin build --target x86_64-pc-windows-msvc` to build.

### Option 2: Windows Native

1. Install Rust via [rustup.rs](https://rustup.rs).
2. Run `cargo run` to test.

## Pull Request Process

1. Update the `README.md` with details of changes if applicable.
2. Increase the version numbers in `Cargo.toml` if strictly necessary.
3. Ensure all tests pass (`cargo test`) and the code is formatted (`cargo fmt`).
