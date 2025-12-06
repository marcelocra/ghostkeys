# ADR 004: cargo-xwin for Cross-Compilation

## Status

Accepted

## Context

Development happens on Linux (DevContainers), but the target platform is Windows. We need a way to build Windows binaries from Linux without:
- Setting up a Windows VM
- Using Wine
- Maintaining separate build environments

## Decision

We will use **cargo-xwin** for cross-compiling from Linux to Windows.

## Rationale

- **No Windows Required**: Build Windows binaries entirely on Linux
- **Native Binaries**: Produces real Windows executables (not Wine wrappers)
- **CI/CD Friendly**: Works in GitHub Actions and DevContainers
- **MSVC Toolchain**: Uses Microsoft's official toolchain
- **Zero Runtime Dependencies**: No DLLs needed on target Windows system

## Implementation

```bash
# Install
cargo install cargo-xwin

# Build
cargo xwin build --target x86_64-pc-windows-msvc --release
```

## Consequences

### Positive

- Develop on Linux, deploy to Windows
- Consistent build environment (DevContainer)
- Fast iteration (no VM overhead)
- CI/CD can build Windows binaries on Linux runners

### Negative

- Cannot test Windows-specific behavior on Linux
- Requires Windows machine for final testing
- Additional tool to install

## Alternatives Considered

- **Native Windows builds**: Requires Windows development environment
- **MinGW**: Produces GNU binaries, not native MSVC
- **Wine**: Emulation overhead, compatibility issues
