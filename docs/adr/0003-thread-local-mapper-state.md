# 0003 - Use Thread-Local Storage for Mapper State

**Status:** Accepted

**Date:** 2025-12-05

**Deciders:** Marcelo Almeida (repository owner)

## Context

Windows keyboard hooks run in a callback function with signature:
```rust
unsafe extern "system" fn low_level_keyboard_proc(
    code: i32, wparam: WPARAM, lparam: LPARAM
) -> LRESULT
```

This callback:
- Must be a static function (no closures with captured state)
- Is called by Windows on the same thread that installed the hook
- Needs access to the `Mapper` state machine (for dead key handling)
- Must avoid blocking or slow operations

We need a way to share mutable state with this callback safely.

## Decision

Use **thread-local storage** with `RefCell<Option<Mapper>>`:

```rust
thread_local! {
    static MAPPER: RefCell<Option<Mapper>> = RefCell::new(None);
    static HOOK_HANDLE: RefCell<Option<HHOOK>> = RefCell::new(None);
    static IS_INJECTING: RefCell<bool> = RefCell::new(false);
}
```

Access pattern in callback:
```rust
let action = MAPPER.with(|mapper| {
    if let Some(ref mut m) = *mapper.borrow_mut() {
        m.process_key(virtual_key, shift)
    } else {
        KeyAction::Pass
    }
});
```

## Alternatives Considered

### Option 1: Global static with Mutex

```rust
lazy_static! {
    static ref MAPPER: Mutex<Mapper> = Mutex::new(Mapper::new());
}
```

**Rejected** because:
- Mutex has locking overhead on every keystroke
- Risk of deadlock if callback is re-entered
- Unnecessary for single-threaded hook callback
- Potential for priority inversion issues

### Option 2: Unsafe global pointer

```rust
static mut MAPPER: *mut Mapper = std::ptr::null_mut();
```

**Rejected** because:
- Violates Rust safety guarantees
- Manual memory management required
- Race conditions possible
- Harder to reason about correctness

### Option 3: Pass state through LPARAM

Use `SetWindowsHookExW` with custom data pointer.

**Rejected** because:
- `WH_KEYBOARD_LL` doesn't support user data parameter
- Would require unsafe pointer casting
- More complex than thread-local

## Consequences

### Positive

- **Zero overhead** - `RefCell` has no locking cost (runtime borrow checking only)
- **Safe Rust** - No unsafe blocks needed for state management
- **Clean lifecycle** - `Option<Mapper>` allows explicit init/cleanup
- **Recursion prevention** - `IS_INJECTING` flag prevents hook from processing its own injected keys
- **Idiomatic** - Standard Rust pattern for single-threaded interior mutability

### Negative

- **Single-threaded only** - Cannot share state across threads (not needed for hooks)
- **Runtime borrow checking** - Panics if borrowed twice (prevented by design)
- **Thread affinity** - Hook must be installed and used on same thread

### Neutral

- Each thread gets its own `Mapper` instance (only one thread uses it)
- Initialization happens in `start()`, cleanup in `stop()`

## Notes

- Windows guarantees `WH_KEYBOARD_LL` callbacks run on the installing thread
- `IS_INJECTING` flag is critical to prevent infinite recursion when calling `SendInput`
- Related: [0001 - Use windows-rs](0001-use-windows-rs-for-keyboard-hooks.md)

**References:**
- [Rust RefCell Documentation](https://doc.rust-lang.org/std/cell/struct.RefCell.html)
- [thread_local! macro](https://doc.rust-lang.org/std/macro.thread_local.html)
- [LowLevelKeyboardProc callback](https://learn.microsoft.com/en-us/windows/win32/winmsg/lowlevelkeyboardproc)
