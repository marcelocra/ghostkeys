//! Tests for SharedState
//!
//! These tests verify the state management properties.

use proptest::prelude::*;

use ghostkeys::state::{OperationMode, SharedState};

proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]

    // **Feature: ghostkeys, Property 6: Enable/Disable Toggle Round-Trip**
    // **Validates: Requirements 4.3, 4.4**
    #[test]
    fn prop_toggle_round_trip(_seed in any::<u64>()) {
        let state = SharedState::new();

        // Initial state should be Active
        prop_assert_eq!(state.get_mode().unwrap(), OperationMode::Active);

        // Toggle to Passthrough
        let mode1 = state.toggle_mode().unwrap();
        prop_assert_eq!(mode1, OperationMode::Passthrough);

        // Toggle back to Active
        let mode2 = state.toggle_mode().unwrap();
        prop_assert_eq!(mode2, OperationMode::Active);
    }

    #[test]
    fn prop_set_mode_is_consistent(mode in prop_oneof![
        Just(OperationMode::Active),
        Just(OperationMode::Passthrough),
    ]) {
        let state = SharedState::new();

        state.set_mode(mode).unwrap();
        prop_assert_eq!(state.get_mode().unwrap(), mode);
    }
}

#[cfg(test)]
mod thread_safety_tests {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_shared_state_across_threads() {
        let state = SharedState::new();
        let state_clone = state.clone();

        let handle = thread::spawn(move || {
            state_clone.set_mode(OperationMode::Passthrough).unwrap();
        });

        handle.join().unwrap();

        assert_eq!(state.get_mode().unwrap(), OperationMode::Passthrough);
    }

    #[test]
    fn test_exit_flag_across_threads() {
        let state = SharedState::new();
        let state_clone = state.clone();

        assert!(!state.should_exit());

        let handle = thread::spawn(move || {
            state_clone.signal_exit();
        });

        handle.join().unwrap();

        assert!(state.should_exit());
    }

    #[test]
    fn test_concurrent_toggles() {
        let state = Arc::new(SharedState::new());
        let mut handles = vec![];

        // Spawn multiple threads that toggle the mode
        for _ in 0..10 {
            let state_clone = Arc::clone(&state);
            handles.push(thread::spawn(move || {
                for _ in 0..100 {
                    let _ = state_clone.toggle_mode();
                }
            }));
        }

        for handle in handles {
            handle.join().unwrap();
        }

        // State should still be valid (either Active or Passthrough)
        let mode = state.get_mode().unwrap();
        assert!(mode == OperationMode::Active || mode == OperationMode::Passthrough);
    }
}
