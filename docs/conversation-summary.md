# GhostKeys Spec Conversation Summary

**Date:** December 3, 2025

## Key Decision: Conceptual Shift

The original spec files were based on a **wrong assumption**:
- ❌ Old: US-International layout with dead keys (`'` + `c` → `ç`)
- ✅ New: Plain US layout with ABNT2 position mapping

### The Correct Vision (from README)

GhostKeys maps **ABNT2 physical key positions** to their pt-BR meanings when typed on US hardware with US OS layout.

**Example:**
```
Typed on US layout (using ABNT2 key positions): ~Mergulhar de cal;'ao [e dif[icil~
Corrected output: "Mergulhar de calção é difícil"
```

**Key mappings:**
- `~` → `"` (tilde key on US maps to quote on ABNT2)
- `;` → `ç` (semicolon on US is cedilla on ABNT2)
- `'` + `a` → `ã` (apostrophe is tilde dead-key position on ABNT2)
- `[` + `e` → `é` (left bracket is acute accent dead-key on ABNT2)

## Spec Files Rewritten

All three spec files were completely rewritten:

### 1. requirements.md
- Changed prerequisite from US-International to plain US layout
- Direct position mappings: `;` → `ç`, `` ` `` → `"`, `\` → `]`
- Dead key triggers based on ABNT2 positions: `'` (tilde), `[` (acute), `]` (grave)
- Simplified to Enable/Disable toggle (no context-awareness for v1)
- Kept: performance, panic safety, hook management requirements

### 2. design.md
- Updated architecture for position-based mapping
- New mapping tables:
  - Position map (US key → ABNT2 char)
  - Accent combination table (accent + char → accented char)
- 8 correctness properties for property-based testing
- Removed window monitor (no context-awareness in v1)

### 3. tasks.md
- 9 task groups with checkpoints
- Property tests marked as optional (`*`)
- Incremental build order:
  1. Project scaffold
  2. Position mapper
  3. Dead key state machine
  4. Keyboard interceptor
  5. System tray integration

## What Was Reused (~40%)

- System Tray integration structure
- Panic safety and hook release logic
- Keyboard hook management basics
- Performance requirements
- State machine concept (different triggers)
- Multi-threaded architecture

## What Was Rewritten (~60%)

- All character mapping logic
- Dead key trigger detection
- State machine triggers
- Context awareness → removed (simple toggle instead)
- All correctness properties

## Platform Abstraction (Added Later)

User requested ability to develop on Linux while targeting Windows. Design updated:

**Project Structure:**
```
src/
├── main.rs              # Entry point, platform detection
├── mapper.rs            # Pure logic - testable on any OS
├── state.rs             # Shared state types - pure Rust
├── error.rs             # Error types - pure Rust
├── interceptor.rs       # Trait definition for keyboard hook
└── platform/
    ├── mod.rs           # Platform detection and factory
    ├── windows.rs       # Windows implementation (primary)
    └── linux.rs         # Linux implementation (dev/testing)
```

**Benefits:**
- Core mapper logic is pure Rust, testable on any OS
- Property-based tests run on Linux during development
- Only `platform/` code requires target OS for integration testing
- Tasks 6.2 (Linux) and 6.3 (Windows) implement the same trait

## Implementation Progress

**Completed without running commands:**
- [x] 1.1 Cargo.toml with platform-specific dependencies
- [x] 1.2 Module structure with platform abstraction
- [x] 1.3 Core types (error.rs, state.rs, interceptor.rs)
- [x] 2.1 Position mapping table (mapper.rs)
- [x] 2.3 Dead key trigger detection (mapper.rs)
- [x] 4.1 Accent combination table (mapper.rs)
- [x] 4.3 State machine core (mapper.rs)
- [x] 4.4 Non-combinable fallback logic (mapper.rs)
- [x] 4.6 Timeout handling (mapper.rs)
- [x] 6.1 KeyboardInterceptor trait (interceptor.rs)

**Platform stubs created:**
- `src/platform/windows.rs` - Windows interceptor (TODO: implement hooks)
- `src/platform/linux.rs` - Linux interceptor (TODO: implement hooks)

## Next Steps

1. **Run `cargo build`** to verify the code compiles
2. **Run `cargo test`** to verify unit tests pass
3. **Continue with remaining tasks** (property tests, interceptor implementations, tray integration)

## Important Notes

- The ABNT2 position mappings in the spec may need adjustment based on your actual keyboard muscle memory
- Primary target is Windows 11, but development can happen on Linux
- Context-awareness (per-app rules, language detection) deferred to future versions
