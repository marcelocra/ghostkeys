# Building GhostKeys with Kiro: Spec-Driven Development for Systems Programming

**Date:** December 5, 2025  
**Author:** Marcelo Almeida  
**Project:** [GhostKeys](https://github.com/marcelomd/ghostkeys) - ABNT2 keyboard emulation for US keyboards

## TL;DR

Built a Windows keyboard interceptor in Rust using Kiro's spec-driven development workflow. Key learnings:

- **Specs First**: Writing requirements and design docs before code caught edge cases early
- **Cross-Platform Dev**: Used cargo-xwin to build Windows binaries from Linux DevContainers
- **Agent Hooks**: Automated `cargo check` on every file save for instant feedback
- **Property-Based Testing**: Designed correctness properties upfront, not as an afterthought
- **90-Minute MVP**: From zero to working system tray + keyboard hook in emergency timeline

## The Challenge

Brazilian developers face a daily dilemma: use ABNT2 layout for Portuguese (breaking coding shortcuts) or use US layout for code (making Portuguese typing painful). GhostKeys solves this by intercepting keyboard input at the OS level and remapping US key positions to ABNT2 characters.

**Technical Requirements:**
- Low-level Windows keyboard hooks (`WH_KEYBOARD_LL`)
- Unicode character injection via `SendInput`
- Dead key state machine (tilde, acute, circumflex, grave)
- System tray UI with pause/resume
- Zero configuration, <5MB RAM

## The Kiro Workflow

### 1. Requirements First (Not Code First)

Instead of jumping into Rust, we started with `.kiro/specs/ghostkeys/requirements.md`:

```markdown
### Requirement 1: Direct Position Mapping

**User Story:** As a user with ABNT2 muscle memory, I want to press the semicolon key and get ç...

#### Acceptance Criteria
1. WHEN a user presses ; THEN the system SHALL output ç
2. WHEN a user presses Shift+; THEN the system SHALL output Ç
```

**Why this matters:** Writing acceptance criteria forced us to think about edge cases (What about Shift? What about Caps Lock?) before writing a single line of code.

### 2. Design Document with Correctness Properties

The design phase included a critical section: **Correctness Properties**. These are universal properties that must hold for all inputs:

```markdown
## Correctness Properties

Property 1: Position mapping consistency
*For any* key position, pressing it should always produce the same ABNT2 character regardless of OS layout

Property 2: Dead key round-trip
*For any* dead key followed by space, the system should output the accent character itself

Property 3: State machine safety
*For any* sequence of keys, the mapper should never enter an invalid state
```

**Why this matters:** These properties became the foundation for property-based tests. We knew *what* to test before implementing *how* to test it.

### 3. Task Breakdown with Optional Tests

The tasks document broke implementation into discrete steps:

```markdown
- [ ] 1. Implement Mapper core logic
- [ ] 1.1 Create VirtualKey enum
- [ ] 1.2 Implement position mapping table
- [ ]* 1.3 Write property test for position consistency
- [ ] 2. Implement Windows interceptor
- [ ] 2.1 Set up low-level keyboard hook
- [ ]* 2.2 Write property test for hook safety
```

**Why this matters:** The `*` marker indicated optional tasks. During the 90-minute emergency timeline, we skipped optional tests to hit the MVP deadline, but the spec preserved them for later.

### 4. Agent Hooks for Quality Control

Created `.kiro/hooks/quality-control.json`:

```json
{
  "name": "Quality Control",
  "trigger": { "type": "onFileSave", "pattern": "**/*.rs" },
  "action": { "type": "shell", "command": "cargo check" }
}
```

**Why this matters:** Every Rust file save triggered `cargo check`. Instant feedback loop caught compilation errors before committing.

## Key Technical Decisions (ADRs)

### ADR 001: windows-rs over rdev

**Decision:** Use Microsoft's official `windows-rs` bindings instead of cross-platform `rdev`.

**Rationale:**
- Official Microsoft support and security updates
- Direct access to `SetWindowsHookExW` and `SendInput`
- Type-safe Rust abstractions over Win32 APIs
- Production-ready (used by Tauri)

**Trade-off:** Windows-only, but that's our target platform.

### ADR 002: Position-Based Mapping

**Decision:** Map physical key positions, not character values.

**Rationale:**
- Preserves muscle memory for ABNT2 users
- Works regardless of OS layout setting
- Coding shortcuts (Ctrl+C, etc.) remain intact

**Trade-off:** Only works on US physical keyboards.

### ADR 003: Thread-Local Mapper State

**Decision:** Use `thread_local!` with `RefCell<Option<Mapper>>` instead of `Mutex`.

**Rationale:**
- Windows hooks are single-threaded
- Zero overhead compared to `Mutex`
- Safe Rust (no unsafe blocks)

**Trade-off:** Cannot share state across threads (not needed).

### ADR 004: cargo-xwin for Cross-Compilation

**Decision:** Build Windows binaries from Linux using `cargo-xwin`.

**Rationale:**
- Develop in Linux DevContainers
- No Windows VM needed
- CI/CD builds Windows binaries on Linux runners

**Trade-off:** Cannot test Windows-specific behavior on Linux.

## The 90-Minute Emergency Timeline

With 90 minutes to deadline, we had to prioritize ruthlessly:

**What we built:**
1. System tray UI with green/yellow icon (15 min)
2. Agent Hook for `cargo check` (5 min)
3. Windows keyboard interceptor (45 min)
4. Integration and testing (25 min)

**What we skipped (marked optional in spec):**
- Property-based tests
- Integration tests
- Linux interceptor implementation

**Why specs saved us:** The spec preserved the full vision. We could skip optional tasks without losing track of what needed to be done later.

## Lessons Learned

### 1. Specs Catch Edge Cases Early

Writing "WHEN a user presses Shift+; THEN output Ç" forced us to handle shift state before implementing the hook. Without the spec, we would have discovered this bug during testing.

### 2. Correctness Properties Guide Testing

Instead of "write tests for the mapper," we had specific properties:
- "Position mapping must be consistent"
- "Dead keys must round-trip"

This made test writing mechanical, not creative.

### 3. Cross-Compilation is a Superpower

Developing on Linux (with DevContainers) while targeting Windows (with cargo-xwin) gave us:
- Consistent build environment
- Fast iteration (no VM overhead)
- CI/CD that builds Windows binaries on Linux

### 4. Agent Hooks Create Tight Feedback Loops

Running `cargo check` on every save caught errors in seconds, not minutes. This is especially critical for systems programming where compilation errors are common.

### 5. Optional Tasks Enable Pragmatism

Marking tests as optional (`*`) let us ship an MVP under time pressure while preserving the full vision in the spec. No technical debt was hidden.

## Tools and Stack

- **Language:** Rust (stable)
- **Windows API:** windows-rs (official Microsoft bindings)
- **UI:** tray-icon + tao (Tauri ecosystem)
- **Cross-Compilation:** cargo-xwin
- **Task Runner:** just (like npm scripts for Rust)
- **Changelog:** git-cliff (Conventional Commits)
- **Development:** Kiro.dev with DevContainers

## Release Automation

GitHub Actions workflow (`.github/workflows/release.yml`):

```yaml
on:
  push:
    tags: ['v*']

jobs:
  release:
    runs-on: windows-latest
    steps:
      - Build ghostkeys.exe
      - Generate changelog with git-cliff
      - Calculate SHA256 checksum
      - Publish GitHub Release
```

**Result:** `git tag v1.0.0 && git push origin v1.0.0` triggers automatic release.

## Metrics

- **Lines of Code:** ~800 (excluding tests)
- **Binary Size:** 4.2 MB (stripped release build)
- **Memory Usage:** <5 MB RAM
- **Development Time:** 6 hours (including specs, docs, CI/CD)
- **Emergency MVP:** 90 minutes

## Conclusion

Spec-driven development isn't just for web apps. For systems programming (keyboard hooks, low-level APIs), specs provide:

1. **Safety net:** Catch edge cases before unsafe code
2. **Documentation:** ADRs explain *why* we chose thread-local over Mutex
3. **Pragmatism:** Optional tasks let us ship under pressure
4. **Quality:** Correctness properties guide testing strategy

GhostKeys proves that you can move fast *and* maintain quality when you have the right workflow.

## Links

- **Source:** [github.com/marcelomd/ghostkeys](https://github.com/marcelomd/ghostkeys)
- **Kiro:** [kiro.dev](https://kiro.dev)
- **Kiroween 2025:** [kiroween.devpost.com](https://kiroween.devpost.com)

---

*Built for Kiroween 2025 - A hackathon celebrating spec-driven development with AI agents.*
