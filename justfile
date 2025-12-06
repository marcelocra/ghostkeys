# GhostKeys Task Runner
# Install just: cargo install just
# Run: just <task>

# Default task - show available commands
default:
    @just --list

# Build for current platform (debug)
build:
    cargo build

# Build for current platform (release)
build-release:
    cargo build --release

# Build for Windows (cross-compile from Linux)
build-windows:
    cargo xwin build --target x86_64-pc-windows-msvc --release

# Run the application
run:
    cargo run

# Run tests
test:
    cargo test

# Run tests with output
test-verbose:
    cargo test -- --nocapture

# Check code without building
check:
    cargo check

# Format code
fmt:
    cargo fmt

# Lint with clippy
lint:
    cargo clippy -- -W clippy::all

# Clean build artifacts
clean:
    cargo clean

# Generate changelog
changelog:
    git cliff -o CHANGELOG.md

# Generate changelog for current tag only
changelog-release:
    git cliff --current --strip header

# Install development dependencies
setup:
    cargo install cargo-xwin just git-cliff

# Full CI check (format, lint, test)
ci: fmt lint test
    @echo "All CI checks passed!"
