# 0004 - Use cargo-xwin for Cross-Compilation

**Status:** Accepted

**Date:** 2025-12-05

**Deciders:** Marcelo Almeida (repository owner)

## Context

GhostKeys development happens in Linux DevContainers, but the target platform is Windows 11. We need to:

- Build Windows `.exe` binaries from Linux
- Use the MSVC toolchain (not MinGW) for native Windows binaries
- Integrate with CI/CD pipelines
- Avoid maintaining separate Windows development environments

**Constraints:**
- No Windows VM available during development
- CI/CD should build release binaries
- Binary must be native Windows (not Wine/emulation)
- Should work in GitHub Codespaces and local DevContainers

## Decision

Use **cargo-xwin** for cross-compiling from Linux to Windows:

```bash
# Install
cargo install cargo-xwin

# Build
cargo xwin build --target x86_64-pc-windows-msvc --release
```

Output: `target/x86_64-pc-windows-msvc/release/ghostkeys.exe`

## Alternatives Considered

### Option 1: Native Windows development

Develop directly on Windows using Visual Studio or VS Code.

**Rejected** because:
- Requires Windows machine or VM
- Inconsistent environment across team members
- Harder to integrate with Linux-based CI/CD
- DevContainers provide better reproducibility

### Option 2: MinGW cross-compilation

Use `x86_64-pc-windows-gnu` target with MinGW.

**Rejected** because:
- Produces GNU binaries, not native MSVC
- May have compatibility issues with Windows APIs
- `windows-rs` is optimized for MSVC target
- Some Windows features require MSVC

### Option 3: Wine for testing

Run Windows binaries under Wine on Linux.

**Rejected** because:
- Emulation overhead
- Keyboard hooks may not work correctly
- Not a real Windows environment
- Debugging is harder

### Option 4: GitHub Actions Windows runner only

Only build on `windows-latest` in CI.

**Partially accepted** - We use this for releases, but cargo-xwin enables local development builds.

## Consequences

### Positive

- **Linux development** - Full development workflow in DevContainers
- **No Windows required** - Build Windows binaries without Windows
- **Native MSVC binaries** - Real Windows executables, not emulated
- **CI/CD flexibility** - Can build on Linux or Windows runners
- **Fast iteration** - No VM startup time

### Negative

- **Cannot test locally** - Must transfer `.exe` to Windows for testing
- **Additional tool** - `cargo-xwin` must be installed
- **Download overhead** - First build downloads Windows SDK components

### Neutral

- Release builds still use `windows-latest` in GitHub Actions (native)
- Local builds use cargo-xwin for quick iteration
- Both produce identical binaries

## Notes

- cargo-xwin is installed in DevContainer `postCreateCommand`
- Also available via `just setup` for manual installation
- Release workflow uses native Windows runner for official builds
- Related: DevContainer configuration in `.devcontainer/devcontainer.json`

**References:**
- [cargo-xwin GitHub](https://github.com/rust-cross/cargo-xwin)
- [Cross-compilation in Rust](https://rust-lang.github.io/rustup/cross-compilation.html)
- [MSVC vs GNU targets](https://doc.rust-lang.org/rustc/platform-support.html)
