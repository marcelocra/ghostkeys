# ADR 003: Thread-Local Mapper State

## Status

Accepted

## Context

Windows keyboard hooks run in a callback function that must be `extern "system"` and cannot easily share state with the main application. We need to manage the `Mapper` state machine (for dead keys) in a thread-safe way.

Options:
1. **Global static with Mutex**: `lazy_static!` with `Mutex<Mapper>`
2. **Thread-local storage**: `thread_local!` with `RefCell<Option<Mapper>>`
3. **Unsafe global**: Raw pointer (unsafe)

## Decision

We will use **thread-local storage** with `RefCell<Option<Mapper>>`.

## Rationale

- **Single-Threaded Hook**: Windows keyboard hooks always run on the same thread
- **No Lock Contention**: `RefCell` has zero overhead compared to `Mutex`
- **Interior Mutability**: Allows mutation in callback without `mut` reference
- **Safe**: No unsafe code needed
- **Clean Lifecycle**: Easy to initialize/cleanup with `Option<Mapper>`

## Implementation

```rust
thread_local! {
    static MAPPER: RefCell<Option<Mapper>> = RefCell::new(None);
    static HOOK_HANDLE: RefCell<Option<HHOOK>> = RefCell::new(None);
    static IS_INJECTING: RefCell<bool> = RefCell::new(false);
}
```

## Consequences

### Positive

- Zero-cost abstraction (no mutex overhead)
- Safe Rust (no unsafe blocks for state management)
- Clean initialization/cleanup
- Prevents recursion with `IS_INJECTING` flag

### Negative

- Only works for single-threaded hooks (not an issue for Windows)
- Cannot share state across threads (not needed)

## Alternatives Considered

- **Mutex**: Unnecessary overhead for single-threaded access
- **Unsafe global**: Violates Rust safety guarantees
