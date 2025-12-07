# Cross-Compilation with cargo-xwin

GhostKeys is a Windows application, but it can be developed and built on Linux using `cargo-xwin`.

## Prerequisites

1.  **Rust Toolchain**: Ensure you have Rust installed.
2.  **LLVM/Clang**: Required for `cargo-xwin`.
    *   Ubuntu/Debian: `sudo apt-get install clang llvm lld`
3.  **cargo-xwin**: Install the tool.
    ```bash
    cargo install cargo-xwin
    ```

## Building for Windows

To build a release binary for Windows (x86_64):

```bash
cargo xwin build --release --target x86_64-pc-windows-msvc
```

The resulting binary will be located at:
`target/x86_64-pc-windows-msvc/release/ghostkeys.exe`

## Development Notes

*   **Linux Interceptor**: The project includes a stub `LinuxInterceptor` for basic logic testing on Linux, but the core Windows hooking logic is only active on Windows targets.
*   **Testing**: Unit tests and property-based tests that do not rely on `windows-rs` specific types can be run on Linux.
