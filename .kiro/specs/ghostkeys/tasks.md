# Implementation Plan

- [ ] 1. Project Scaffold & Dependencies
  - [ ] 1.1 Initialize Rust project with Cargo
    - Run `cargo init` and configure `Cargo.toml`
    - Add dependencies: `rdev`, `tray-icon`, `tao`, `thiserror`, `proptest`
    - Configure Windows-specific settings in `Cargo.toml`
    - _Requirements: 7.1_

  - [ ] 1.2 Create module structure
    - Create `src/interceptor.rs` with module declaration
    - Create `src/mapper.rs` with module declaration
    - Create `src/error.rs` for error types
    - Create `src/state.rs` for shared state types
    - Update `src/main.rs` with module imports
    - _Requirements: 7.1_

  - [ ] 1.3 Define core types and interfaces
    - Implement `GhostKeysError` enum in `error.rs`
    - Implement `AppState`, `OperationMode`, `SharedState` in `state.rs`
    - Define `KeyAction` enum for interceptor responses
    - Define `MapperState` and `AccentType` enums
    - _Requirements: 3.1, 7.1_

  - [ ]* 1.4 Write unit tests for core types
    - Test `SharedState` thread-safe access
    - Test `OperationMode` default values
    - _Requirements: 3.1_

- [ ] 2. ABNT2 Position Mapper
  - [ ] 2.1 Implement position mapping table
    - Create `HashMap<VirtualKey, (char, char)>` for direct mappings (unshifted, shifted)
    - Map `;` → (`ç`, `Ç`)
    - Map `` ` `` → (`"`, `"`)
    - Map `\` → (`]`, `}`)
    - _Requirements: 1.1, 1.2, 1.3, 1.4, 1.8_

  - [ ]* 2.2 Write property test for position mapping
    - **Property 1: Position Mapping Correctness**
    - **Validates: Requirements 1.1, 1.2, 1.3, 1.4, 1.8**
    - Generate random position-mapped keys with shift states
    - Verify correct ABNT2 character is returned

  - [ ] 2.3 Implement dead key trigger detection
    - Identify dead key triggers: `'` (tilde), `[` (acute), `]` (grave)
    - Return `Suppress` action and store accent type
    - _Requirements: 1.5, 1.6, 1.7_

  - [ ]* 2.4 Write property test for dead key triggers
    - **Property 2: Dead Key Trigger Transition**
    - **Validates: Requirements 1.5, 1.6, 1.7, 3.2**
    - Generate random dead key triggers
    - Verify state transitions to PendingAccent with correct accent type

- [ ] 3. Checkpoint - Verify position mapping
  - Ensure all tests pass, ask the user if questions arise.

- [ ] 4. Dead Key State Machine
  - [ ] 4.1 Implement accent combination table
    - Create `HashMap<(AccentType, char), char>` with all combinations
    - Tilde combinations: ã, Ã, õ, Õ, ñ, Ñ
    - Acute combinations: á, Á, é, É, í, Í, ó, Ó, ú, Ú
    - Grave combinations: à, À
    - _Requirements: 2.1-2.11_

  - [ ]* 4.2 Write property test for dead key combinations
    - **Property 3: Dead Key Combination Correctness**
    - **Validates: Requirements 2.1-2.11, 3.3**
    - Generate random valid accent + character pairs
    - Verify correct accented character is returned

  - [ ] 4.3 Implement state machine core
    - Create `Mapper` struct with `MapperState` field
    - Implement `new()` to initialize in Idle state
    - Implement `process_key()` method for state transitions
    - Implement `reset()` method to return to Idle state
    - _Requirements: 3.1, 3.2, 3.3, 3.4_

  - [ ] 4.4 Implement non-combinable fallback logic
    - Handle accent followed by non-combinable character
    - Return `ReplaceMultiple` with accent char + pressed char
    - Handle accent followed by space (output accent only)
    - _Requirements: 2.12, 2.13_

  - [ ]* 4.5 Write property test for non-combinable fallback
    - **Property 4: Non-Combinable Character Fallback**
    - **Validates: Requirements 2.12, 2.13, 3.4**
    - Generate random accent + non-combinable character pairs
    - Verify both characters are output in correct order

  - [ ] 4.6 Implement timeout handling
    - Track `last_accent_time` using `Instant`
    - Implement `check_timeout()` method (500ms threshold)
    - Return stored accent char and transition to Idle on timeout
    - _Requirements: 3.5_

  - [ ]* 4.7 Write property test for timeout behavior
    - **Property 5: State Machine Timeout Behavior**
    - **Validates: Requirements 3.5**
    - Test that 500ms timeout outputs accent char and returns to Idle

- [ ] 5. Checkpoint - Verify state machine
  - Ensure all tests pass, ask the user if questions arise.

- [ ] 6. Keyboard Interceptor
  - [ ] 6.1 Implement keyboard hook installation
    - Create `KeyboardInterceptor` struct in `interceptor.rs`
    - Implement hook installation using `rdev::listen`
    - Set up callback to receive key events
    - _Requirements: 7.1_

  - [ ] 6.2 Implement hook release and panic safety
    - Implement `stop()` method to release the hook
    - Set up global panic handler that releases hook
    - Implement `Drop` trait for automatic cleanup
    - _Requirements: 6.1, 6.2, 6.3, 6.4_

  - [ ] 6.3 Implement thread spawning for hook
    - Spawn hook in separate thread
    - Pass `Arc<Mutex<AppState>>` to hook thread
    - Implement graceful shutdown via exit flag
    - _Requirements: 7.1, 7.4_

  - [ ] 6.4 Integrate Mapper with Interceptor
    - Create Mapper instance in hook thread
    - Call `process_key()` for each keystroke
    - Execute returned `KeyAction` (Pass, Suppress, Replace)
    - Implement key injection using `rdev::simulate`
    - _Requirements: 7.2_

  - [ ]* 6.5 Write property test for passthrough mode
    - **Property 7: Passthrough Mode Transparency**
    - **Validates: Requirements 7.3**
    - Test that all keystrokes pass through unmodified in Passthrough mode

  - [ ]* 6.6 Write property test for processing latency
    - **Property 8: Keystroke Processing Latency**
    - **Validates: Requirements 5.1**
    - Measure processing time for random keystrokes
    - Verify all complete within 10ms

- [ ] 7. Checkpoint - Verify interceptor
  - Ensure all tests pass, ask the user if questions arise.

- [ ] 8. System Tray Integration
  - [ ] 8.1 Create System Tray icon
    - Initialize `tray-icon` with application icon
    - Set up `tao` event loop in main thread
    - Display icon in Windows System Tray on startup
    - _Requirements: 4.1_

  - [ ] 8.2 Implement context menu
    - Create menu with "Disable"/"Enable" and "Exit" options
    - Handle menu item click events
    - Update menu text dynamically (Disable ↔ Enable)
    - _Requirements: 4.2, 4.3, 4.4_

  - [ ] 8.3 Implement Enable/Disable toggle
    - Toggle `mode` in SharedState
    - When disabled, force Passthrough mode
    - When enabled, restore Active mode
    - Default to Active mode on startup
    - _Requirements: 4.3, 4.4, 4.6_

  - [ ]* 8.4 Write property test for enable/disable round-trip
    - **Property 6: Enable/Disable Toggle Round-Trip**
    - **Validates: Requirements 4.3, 4.4**
    - Test that disable then enable restores Active mode

  - [ ] 8.5 Implement Exit functionality
    - Signal exit to hook thread via exit flag
    - Wait for hook thread to release hook and terminate
    - Exit event loop and terminate application
    - _Requirements: 4.5, 7.4_

- [ ] 9. Final Checkpoint - Complete integration
  - Ensure all tests pass, ask the user if questions arise.
