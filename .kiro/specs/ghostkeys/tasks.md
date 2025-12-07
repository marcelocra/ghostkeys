# Implementation Plan

- [x] 1. Project Scaffold & Dependencies
  - [x] 1.1 Initialize Rust project with Cargo


    - Run `cargo init` and configure `Cargo.toml`
    - Add dependencies: `windows-rs` (Windows), `rdev` (Linux dev), `tray-icon`, `tao`, `thiserror`, `proptest`
    - Configure platform-specific features in `Cargo.toml`
    - _Requirements: 7.1_

  - [x] 1.2 Create module structure with platform abstraction


    - Create `src/mapper.rs` with module declaration
    - Create `src/error.rs` for error types
    - Create `src/state.rs` for shared state types
    - Create `src/interceptor.rs` with trait definition
    - Create `src/platform/mod.rs` with platform detection
    - Create `src/platform/windows.rs` (stub)
    - Create `src/platform/linux.rs` (stub)
    - Update `src/main.rs` with module imports
    - _Requirements: 7.1_

  - [x] 1.3 Define core types and interfaces

    - Implement `GhostKeysError` enum in `error.rs`
    - Implement `AppState`, `OperationMode`, `SharedState` in `state.rs`
    - Define `KeyAction` enum for interceptor responses
    - Define `MapperState` and `AccentType` enums
    - _Requirements: 3.1, 7.1_

  - [x] 1.4 Write unit tests for core types
    - Test `SharedState` thread-safe access
    - Test `OperationMode` default values
    - _Requirements: 3.1_

- [ ] 2. ABNT2 Position Mapper
  - [x] 2.1 Implement position mapping table

    - Create `HashMap<VirtualKey, (char, char)>` for direct mappings (unshifted, shifted)
    - Map `;` → (`ç`, `Ç`) - ABNT2 Cedilla Position
    - Map `]` → (`[`, `{`) - ABNT2 Bracket Key Position
    - Map `\` → (`]`, `}`) - ABNT2 Close Bracket Position
    - Map `/` → (`;`, `:`) - ABNT2 Semicolon Position
    - _Requirements: 1.1, 1.2, 1.7, 1.8, 1.9, 1.10, 1.11, 1.12_

  - [x] 2.2 Write property test for position mapping
    - **Property 1: Position Mapping Correctness**
    - **Validates: Requirements 1.1, 1.2, 1.3, 1.4, 1.8**
    - Generate random position-mapped keys with shift states
    - Verify correct ABNT2 character is returned

  - [x] 2.3 Implement dead key trigger detection

    - Identify dead key triggers based on corrected ABNT2 mapping:
      - `'` (unshifted) → Tilde (~), `'` (shifted) → Circumflex (^)
      - `[` (unshifted) → Acute (´), `[` (shifted) → Grave (`)
    - Return `Suppress` action and store accent type
    - _Requirements: 1.3, 1.4, 1.5, 1.6_

  - [x] 2.4 Write property test for dead key triggers
    - **Property 2: Dead Key Trigger Transition**
    - **Validates: Requirements 1.5, 1.6, 1.7, 3.2**
    - Generate random dead key triggers
    - Verify state transitions to PendingAccent with correct accent type

- [ ] 3. Checkpoint - Verify position mapping
  - Ensure all tests pass, ask the user if questions arise.

- [ ] 4. Dead Key State Machine
  - [x] 4.1 Implement accent combination table

    - Create `HashMap<(AccentType, char), char>` with all combinations
    - Tilde combinations: ã, Ã, õ, Õ, ñ, Ñ
    - Acute combinations: á, Á, é, É, í, Í, ó, Ó, ú, Ú
    - Grave combinations: à, À
    - _Requirements: 2.1-2.11_

  - [x] 4.2 Write property test for dead key combinations
    - **Property 3: Dead Key Combination Correctness**
    - **Validates: Requirements 2.1-2.11, 3.3**
    - Generate random valid accent + character pairs
    - Verify correct accented character is returned

  - [x] 4.3 Implement state machine core

    - Create `Mapper` struct with `MapperState` field
    - Implement `new()` to initialize in Idle state
    - Implement `process_key()` method for state transitions
    - Implement `reset()` method to return to Idle state
    - _Requirements: 3.1, 3.2, 3.3, 3.4_

  - [x] 4.4 Implement non-combinable fallback logic

    - Handle accent followed by non-combinable character
    - Return `ReplaceMultiple` with accent char + pressed char
    - Handle accent followed by space (output accent only)
    - _Requirements: 2.12, 2.13_

  - [x] 4.5 Write property test for non-combinable fallback
    - **Property 4: Non-Combinable Character Fallback**
    - **Validates: Requirements 2.12, 2.13, 3.4**
    - Generate random accent + non-combinable character pairs
    - Verify both characters are output in correct order

  - [x] 4.6 Implement timeout handling

    - Track `last_accent_time` using `Instant`
    - Implement `check_timeout()` method (500ms threshold)
    - Return stored accent char and transition to Idle on timeout
    - _Requirements: 3.5_

  - [x] 4.7 Write property test for timeout behavior
    - **Property 5: State Machine Timeout Behavior**
    - **Validates: Requirements 3.5**
    - Test that 500ms timeout outputs accent char and returns to Idle

- [ ] 5. Checkpoint - Verify state machine
  - Ensure all tests pass, ask the user if questions arise.

- [ ] 6. Keyboard Interceptor (Platform Abstraction)
  - [x] 6.1 Define KeyboardInterceptor trait


    - Create trait in `interceptor.rs` with `start()`, `stop()`, `is_running()`
    - Define `create_interceptor()` factory function
    - Keep trait platform-agnostic
    - _Requirements: 7.1_

  - [ ] 6.2 Implement Linux interceptor (development platform)
    - Create `LinuxInterceptor` struct in `platform/linux.rs`
    - Implement hook using `rdev::listen` with X11/Wayland
    - Implement key injection using `rdev::simulate`
    - This allows testing on Linux during development
    - _Requirements: 7.1, 7.2_

  - [x] 6.3 Implement Windows interceptor (primary target)
    - Create `WindowsInterceptor` struct in `platform/windows.rs`
    - Implement hook using `windows-rs` (SetWindowsHookEx with WH_KEYBOARD_LL)
    - Implement key injection using `windows-rs` (SendInput)
    - _Requirements: 7.1, 7.2_

  - [x] 6.4 Implement hook release and panic safety
    - Implement `stop()` method to release the hook
    - Set up global panic handler that releases hook
    - Implement `Drop` trait for automatic cleanup
    - _Requirements: 6.1, 6.2, 6.3, 6.4_

  - [x] 6.5 Implement thread spawning for hook
    - Spawn hook in separate thread
    - Pass `Arc<Mutex<AppState>>` to hook thread
    - Implement graceful shutdown via exit flag
    - _Requirements: 7.1, 7.4_

  - [x] 6.6 Integrate Mapper with Interceptor using thread-local storage
    - Create `thread_local!` storage for Mapper instance
    - Create `thread_local!` IS_INJECTING flag for recursion protection
    - Call `process_key()` for each keystroke via thread-local Mapper
    - Execute returned `KeyAction` (Pass, Suppress, Replace)
    - _Requirements: 5.4, 7.2_

  - [x] 6.7 Implement recursion protection
    - Set IS_INJECTING flag to true before SendInput calls
    - Check IS_INJECTING flag at start of hook callback
    - Return Pass immediately if IS_INJECTING is true
    - Reset IS_INJECTING flag after SendInput completes
    - _Requirements: 7.5_

  - [ ] 6.8 Write property test for recursion protection
    - **Property 9: Recursion Protection**
    - **Validates: Requirements 7.5**
    - Test that injected keys are not re-processed by the hook

  - [ ] 6.9 Write property test for passthrough mode
    - **Property 7: Passthrough Mode Transparency**
    - **Validates: Requirements 7.3**
    - Test that all keystrokes pass through unmodified in Passthrough mode

  - [ ] 6.10 Write property test for processing latency
    - **Property 8: Keystroke Processing Latency**
    - **Validates: Requirements 5.1**
    - Measure processing time for random keystrokes
    - Verify all complete within 10ms

- [ ] 7. Checkpoint - Verify interceptor
  - Ensure all tests pass, ask the user if questions arise.

- [ ] 8. System Tray Integration
  - [x] 8.1 Create TrayIconManager with dynamic icons
    - Create `TrayIconManager` struct to manage icon state
    - Generate 32x32 pixel icons dynamically at runtime
    - Create green icon variant for Active mode
    - Create yellow/orange icon variant for Passthrough mode
    - Initialize `tray-icon` with Active mode icon on startup
    - Set up `tao` event loop in main thread
    - _Requirements: 4.1, 4.2, 4.3_

  - [x] 8.2 Implement tooltip display
    - Set tooltip to "GhostKeys - Active" when in Active mode
    - Set tooltip to "GhostKeys - Paused" when in Passthrough mode
    - Update tooltip when mode changes
    - _Requirements: 4.4_

  - [ ] 8.3 Write property test for tooltip state consistency
    - **Property 10: Tooltip State Consistency**
    - **Validates: Requirements 4.4**
    - Test that tooltip text matches current OperationMode

  - [x] 8.4 Implement context menu structure
    - Create non-clickable status indicator item showing current state
    - Create "Pause"/"Resume" toggle menu item
    - Create "Help / Mappings" menu item
    - Create "About" menu item
    - Create "Exit" menu item
    - Handle menu item click events
    - _Requirements: 4.5, 4.6, 4.7, 4.8, 4.9, 4.10, 4.11_

  - [x] 8.5 Implement Pause/Resume toggle
    - Toggle `mode` in SharedState
    - When paused, enter Passthrough mode and update icon to yellow/orange
    - When resumed, enter Active mode and update icon to green
    - Update menu text dynamically (Pause ↔ Resume)
    - Update status indicator text
    - Default to Active mode on startup
    - _Requirements: 4.7, 4.8, 4.12_

  - [x] 8.6 Write property test for pause/resume round-trip
    - **Property 6: Enable/Disable Toggle Round-Trip**
    - **Validates: Requirements 4.7, 4.8**
    - Test that pause then resume restores Active mode

  - [x] 8.7 Implement Help / Mappings dialog
    - Create native Windows MessageBox with key mapping cheat sheet
    - Display US Key → ABNT2 Output mappings
    - Display dead key combination reference
    - _Requirements: 4.9_

  - [x] 8.8 Implement About dialog
    - Create native dialog with version info (v0.1.0)
    - Display application description
    - Display credits and repository link
    - _Requirements: 4.10_

  - [x] 8.9 Implement Exit functionality
    - Signal exit to hook thread via exit flag
    - Wait for hook thread to release hook and terminate
    - Exit event loop and terminate application
    - _Requirements: 4.11, 7.4_

- [ ] 9. Checkpoint - Verify System Tray
  - Ensure all tests pass, ask the user if questions arise.

- [x] 10. DevOps Infrastructure
  - [x] 10.1 Configure cross-compilation with cargo-xwin
    - Update Cargo.toml with platform-specific dependencies
    - Document cargo-xwin installation and usage
    - Test cross-compilation from Linux to Windows
    - _Requirements: 8.1, 8.3_

  - [x] 10.2 Create DevContainer configuration
    - Create `.devcontainer/devcontainer.json`
    - Configure Rust toolchain and cargo-xwin
    - Test development workflow in DevContainer
    - _Requirements: 8.4_

  - [x] 10.3 Create CI pipeline
    - Create `.github/workflows/ci.yml`
    - Configure to run on windows-latest runner
    - Add cargo check, cargo test, cargo build steps
    - _Requirements: 9.2_

  - [x] 10.4 Create Release pipeline
    - Create `.github/workflows/release.yml`
    - Configure to trigger on v* tags
    - Add release binary build step
    - Add git-cliff changelog generation
    - Add SHA256 checksum calculation
    - Add GitHub Release publishing
    - _Requirements: 9.3, 9.4, 9.5, 9.6_

  - [x] 10.5 Create Kiro agent hooks
    - Create `.kiro/hooks/quality-control.json`
    - Configure to run cargo check on file save
    - _Requirements: 9.1_

  - [x] 10.6 Create Static Context Manifest
    - Create `.kiro/context_manifest.json`
    - Document project structure and key files
    - _Requirements: 10.3_

- [ ] 11. Final Checkpoint - Complete integration
  - Ensure all tests pass, ask the user if questions arise.
