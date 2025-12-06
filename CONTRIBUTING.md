# Contributing to GhostKeys

Thank you for your interest in contributing to GhostKeys! This document provides guidelines and information for contributors.

## Code of Conduct

Please be respectful and constructive in all interactions. We welcome contributors of all experience levels.

## Conventional Commits

We follow the [Conventional Commits](https://www.conventionalcommits.org/) specification. Please prefix your commits with the appropriate type:

| Prefix      | Description                                         |
| ----------- | --------------------------------------------------- |
| `feat:`     | New features                                        |
| `fix:`      | Bug fixes                                           |
| `docs:`     | Documentation only                                  |
| `style:`    | Formatting, missing semicolons, etc.                |
| `refactor:` | Code restructuring without changing behavior        |
| `test:`     | Adding or updating tests                            |
| `chore:`    | Updating build tasks, package manager configs, etc. |
| `perf:`     | Performance improvements                            |

### Examples

```bash
# Good commit messages
git commit -m "feat: add ABNT2 to US layout key mapping for Portuguese accents"
git commit -m "fix: resolve incorrect accent mapping for tilde key"
git commit -m "docs: update README with installation instructions"

# Bad commit messages (too vague)
git commit -m "fix: update code"
git commit -m "feat: add hashmap"
```

## Development Setup

### Using DevContainer (Recommended)

Open the project in VS Code or Kiro with the Dev Containers extension. All dependencies are pre-installed automatically.

### Manual Setup

1. Install Rust (stable toolchain)
2. Clone the repository
3. Install system dependencies:

**Linux/WSL:**
```bash
sudo apt-get update && sudo apt-get install -y \
    libx11-dev libxcb1-dev libxkbcommon-dev pkg-config \
    libxi-dev libxtst-dev libxrandr-dev libxcursor-dev \
    libgtk-3-dev clang lld libxdo-dev libayatana-appindicator3-dev
```

**Windows:** No additional system dependencies needed.

4. Install development tools:
```bash
cargo install just cargo-xwin git-cliff
```

5. Build and test:
```bash
just build    # or: cargo build
just test     # or: cargo test
```

### Task Runner

We use [just](https://github.com/casey/just) for common tasks (like npm scripts):

```bash
just              # List all commands
just build        # Build debug
just build-windows # Cross-compile to Windows
just test         # Run tests
just lint         # Run clippy
just fmt          # Format code
just ci           # Run all CI checks
```

## Pull Request Process

1. Fork the repository
2. Create a feature branch (`git checkout -b feat/amazing-feature`)
3. Make your changes following the coding standards
4. Write or update tests as needed
5. Commit using Conventional Commits format
6. Push to your fork
7. Open a Pull Request

## Coding Standards

-   Follow Rust idioms and best practices
-   Use `cargo fmt` before committing
-   Ensure `cargo clippy` passes without warnings
-   Add documentation comments for public APIs
-   Keep functions small and focused

## Security

This project handles keyboard input. Please:

-   Never log or transmit keystrokes
-   Use only trusted, well-maintained dependencies
-   Report security issues privately

## Questions?

Open an issue for any questions or concerns!
