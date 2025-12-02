# Implementation Plan

- [ ] 1. Project Scaffold & Dependencies
  - [ ] 1.1 Initialize Rust project with Cargo
    - Run `cargo init` and configure `Cargo.toml`
    - Add dependencies: `rdev`, `tray-icon`, `tao`, `active-win-pos-rs`, `thiserror`, `proptest`
    - Configure Windows-specific settings in `Cargo.toml`
    - _Requirements: 7.1_

  - [ ] 1.2 Create module structure
    - Create `src/interceptor.rs` with module declaration
    - Create `src/mapper.rs` with module declaration
    - Create `src/window_monitor.rs` with module declaration
    - Create `src/error.rs` for error types
    - Create `src/state.rs` for shared state types
    - Update `src/main.rs` with module imports
    - _Requirements: 7.1_

  - [ ] 1.3 Define core types and interfaces
    - Implement `GhostKeysError` enum in `error.rs`
    - Implement `AppState`, `OperationMode`, `SharedState` in `state.rs`
    - Define `KeyAction` enum for interceptor responses
    - Define `MapperState` and `DeadKey` enums
    - _Requirements: 2.1, 7.1_

  - [ ]* 1.4 Write unit tests for core types
    - Test `SharedState` thread-safe access
    - Test `OperationMode` default values
    - _Requirements: 2.1_

- [ ] 2. Basic Interceptor (Print Keys to Stdout)
  - [ ] 2.1 Implement keyboard hook installation
    - Create `KeyboardInterceptor` struct in `interceptor.rs`
    - Implement hook installation using `rdev::listen`
    - Set up callback to receive key events
    - Print key events to stdout for debugging
    - _Requirements: 7.1, 7.2_

  - [ ] 2.2 Implement hook release and panic safety
    - Implement `stop()` method to release the hook
    - Set up global panic handler that releases hook
    - Implement `Drop` trait for automatic cleanup
    - _Requirements: 6.1, 6.2, 6.3, 6.4_

  - [ ] 2.3 Implement thread spawning for hook
    - Spawn hook in separate thread
    - Pass `Arc<Mutex<AppState>>` to hook thread
    - Implement graceful shutdown via exit flag
    - _Requirements: 7.1, 7.4_

  - [ ]* 2.4 Write property test for passthrough mode
    - **Property 8: Passthrough Mode Transparency**
    - **Validates: Requirements 7.3**
    - Test that all keystrokes pass through unmodified in Passthrough mode

- [ ] 3. Checkpoint - Verify basic interceptor
  - Ensure all tests pass, ask the user if questions arise.

- [ ] 4. The Mapper Logic (State Machine for Accents)
  - [ ] 4.1 Implement character mapping table
    - Create `HashMap<(DeadKey, char), char>` with all ABNT2 mappings
    - Include mappings for: ç, Ç, ã, Ã, õ, Õ, ñ, Ñ, â, ê, ô, à, è, ü
    - Implement lookup function for dead key + character combinations
    - _Requirements: 1.1, 1.2, 1.3, 1.4, 1.5, 1.6_

  - [ ]* 4.2 Write property test for dead key combination mapping
    - **Property 1: Dead Key Combination Mapping**
    - **Validates: Requirements 1.1, 1.2, 1.3, 1.4, 1.5, 1.6**
    - Generate random valid dead key + character pairs
    - Verify correct combined character is returned

  - [ ] 4.3 Implement state machine core
    - Create `Mapper` struct with `MapperState` field
    - Implement `new()` to initialize in Idle state
    - Implement `process_key()` method for state transitions
    - Implement `reset()` method to return to Idle state
    - _Requirements: 2.1, 2.2, 2.3, 2.4_

  - [ ]* 4.4 Write property test for state machine transitions
    - **Property 3: State Machine Idle to PendingAccent Transition**
    - **Validates: Requirements 2.2**
    - Test that dead key input transitions from Idle to PendingAccent

  - [ ]* 4.5 Write property test for state machine completion
    - **Property 4: State Machine PendingAccent to Idle Transition**
    - **Validates: Requirements 2.3, 2.4**
    - Test that any character input in PendingAccent returns to Idle

  - [ ] 4.6 Implement non-matching fallback logic
    - Handle dead key followed by non-combinable character
    - Return `ReplaceMultiple` with dead key + pressed character
    - Handle dead key followed by space (output dead key only)
    - _Requirements: 1.7, 1.8_

  - [ ]* 4.7 Write property test for non-matching fallback
    - **Property 2: Non-Matching Dead Key Fallback**
    - **Validates: Requirements 1.7, 2.4**
    - Generate random dead key + non-combinable character pairs
    - Verify both characters are output in correct order

  - [ ] 4.8 Implement timeout handling
    - Track `last_dead_key_time` using `Instant`
    - Implement `check_timeout()` method (500ms threshold)
    - Return stored dead key and transition to Idle on timeout
    - _Requirements: 2.5_

  - [ ]* 4.9 Write property test for timeout behavior
    - **Property 5: State Machine Timeout Behavior**
    - **Validates: Requirements 2.5**
    - Test that 500ms timeout outputs dead key and returns to Idle

  - [ ] 4.10 Integrate Mapper with Interceptor
    - Create Mapper instance in hook thread
    - Call `process_key()` for each keystroke
    - Execute returned `KeyAction` (Pass, Suppress, Replace)
    - Implement key injection using `rdev::simulate`
    - _Requirements: 7.2_

  - [ ]* 4.11 Write property test for processing latency
    - **Property 9: Keystroke Processing Latency**
    - **Validates: Requirements 5.1**
    - Measure processing time for random keystrokes
    - Verify all complete within 10ms

- [ ] 5. Checkpoint - Verify mapper logic
  - Ensure all tests pass, ask the user if questions arise.

- [ ] 6. System Tray Integration
  - [ ] 6.1 Create System Tray icon
    - Initialize `tray-icon` with application icon
    - Set up `tao` event loop in main thread
    - Display icon in Windows System Tray on startup
    - _Requirements: 4.1_

  - [ ] 6.2 Implement context menu
    - Create menu with "Pause", "Resume", "Exit" options
    - Handle menu item click events
    - Update menu text dynamically (Pause ↔ Resume)
    - _Requirements: 4.2, 4.3, 4.4_

  - [ ] 6.3 Implement Pause/Resume toggle
    - Toggle `is_paused` flag in SharedState
    - When paused, force Passthrough mode globally
    - When resumed, restore context-aware mode detection
    - _Requirements: 4.3, 4.4_

  - [ ]* 6.4 Write property test for pause/resume round-trip
    - **Property 7: Pause/Resume Toggle Round-Trip**
    - **Validates: Requirements 4.3, 4.4**
    - Test that pause then resume restores original behavior

  - [ ] 6.5 Implement Exit functionality
    - Signal exit to hook thread via exit flag
    - Wait for hook thread to release hook and terminate
    - Exit event loop and terminate application
    - _Requirements: 4.5, 7.4_

- [ ] 7. Checkpoint - Verify tray integration
  - Ensure all tests pass, ask the user if questions arise.

- [ ] 8. Active Window Detection
  - [ ] 8.1 Implement WindowMonitor
    - Create `WindowMonitor` struct in `window_monitor.rs`
    - Implement `get_active_window()` using `active-win-pos-rs`
    - Return `WindowInfo` with process name and title
    - _Requirements: 3.4_

  - [ ] 8.2 Implement window rule matching
    - Define `WindowRule` and `WindowPattern` types
    - Implement `determine_mode()` based on rules
    - Add default rules for VSCode (Passthrough) and Slack (Active)
    - Default to Active mode for unmatched windows
    - _Requirements: 3.1, 3.2, 3.3_

  - [ ]* 8.3 Write property test for window mode detection
    - **Property 6: Window Mode Detection**
    - **Validates: Requirements 3.1, 3.2, 3.3**
    - Generate random WindowInfo with various patterns
    - Verify correct mode is returned for each pattern

  - [ ] 8.4 Implement polling loop
    - Poll active window every 100ms
    - Update SharedState with current window and mode
    - Handle window detection failures gracefully (default to Active)
    - _Requirements: 3.4, 3.5_

  - [ ] 8.5 Integrate with Interceptor
    - Read current mode from SharedState in hook callback
    - Apply Passthrough or Active mode based on window
    - Respect global pause override
    - _Requirements: 3.1, 3.2, 3.3, 7.2, 7.3_

- [ ] 9. Final Checkpoint - Complete integration
  - Ensure all tests pass, ask the user if questions arise.
