//! Property-based tests for the Mapper
//!
//! These tests verify the correctness properties defined in the design document.
//! They run on any OS since the Mapper is pure Rust with no platform dependencies.

use proptest::prelude::*;

// Import from the main crate
use ghostkeys::mapper::{AccentType, KeyAction, Mapper, MapperState, VirtualKey};

/// Generator for position-mapped keys (;, ], \, /)
/// These keys have direct character mappings (not dead keys)
fn position_key_strategy() -> impl Strategy<Value = VirtualKey> {
    prop_oneof![
        Just(VirtualKey::Semicolon),    // ; -> ç/Ç
        Just(VirtualKey::RightBracket), // ] -> [/{
        Just(VirtualKey::Backslash),    // \ -> ]/}
        Just(VirtualKey::Slash),        // / -> ;/:
    ]
}

/// Generator for dead key triggers (', [)
/// Note: ] is no longer a dead key in the corrected mapping
fn dead_key_strategy() -> impl Strategy<Value = VirtualKey> {
    prop_oneof![
        Just(VirtualKey::Apostrophe), // ' -> Tilde (unshifted), Circumflex (shifted)
        Just(VirtualKey::LeftBracket), // [ -> Acute (unshifted), Grave (shifted)
    ]
}

/// Generator for combinable characters (those that can combine with accents)
fn combinable_char_strategy() -> impl Strategy<Value = char> {
    prop_oneof![
        Just('a'),
        Just('A'),
        Just('e'),
        Just('E'),
        Just('i'),
        Just('I'),
        Just('o'),
        Just('O'),
        Just('u'),
        Just('U'),
        Just('n'),
        Just('N'),
    ]
}

/// Generator for non-combinable characters
fn non_combinable_char_strategy() -> impl Strategy<Value = char> {
    prop_oneof![
        Just('b'),
        Just('c'),
        Just('d'),
        Just('f'),
        Just('g'),
        Just('h'),
        Just('j'),
        Just('k'),
        Just('l'),
        Just('m'),
        Just('p'),
        Just('q'),
        Just('r'),
        Just('s'),
        Just('t'),
        Just('v'),
        Just('w'),
        Just('x'),
        Just('y'),
        Just('z'),
    ]
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]

    // **Feature: ghostkeys, Property 1: Position Mapping Correctness**
    // **Validates: Requirements 1.1, 1.2, 1.7, 1.8, 1.9, 1.10, 1.11, 1.12**
    #[test]
    fn prop_position_mapping_correctness(
        key in position_key_strategy(),
        shift in any::<bool>()
    ) {
        let mut mapper = Mapper::new();
        let action = mapper.process_key(key, shift);

        // All position-mapped keys should return Replace action
        match action {
            KeyAction::Replace(c) => {
                // Verify the correct character is returned based on ABNT2 Positional Mapping
                match (key, shift) {
                    // ; -> ç/Ç (ABNT2 Cedilla Position)
                    (VirtualKey::Semicolon, false) => prop_assert_eq!(c, 'ç'),
                    (VirtualKey::Semicolon, true) => prop_assert_eq!(c, 'Ç'),
                    // ] -> [/{ (ABNT2 Bracket Key Position)
                    (VirtualKey::RightBracket, false) => prop_assert_eq!(c, '['),
                    (VirtualKey::RightBracket, true) => prop_assert_eq!(c, '{'),
                    // \ -> ]/} (ABNT2 Close Bracket Position)
                    (VirtualKey::Backslash, false) => prop_assert_eq!(c, ']'),
                    (VirtualKey::Backslash, true) => prop_assert_eq!(c, '}'),
                    // / -> ;/: (ABNT2 Semicolon Position)
                    (VirtualKey::Slash, false) => prop_assert_eq!(c, ';'),
                    (VirtualKey::Slash, true) => prop_assert_eq!(c, ':'),
                    _ => prop_assert!(false, "Unexpected key"),
                }
            }
            _ => prop_assert!(false, "Expected Replace action for position-mapped key"),
        }

        // State should remain Idle
        prop_assert_eq!(mapper.state(), &MapperState::Idle);
    }

    // **Feature: ghostkeys, Property 2: Dead Key Trigger Transition**
    // **Validates: Requirements 1.3, 1.4, 1.5, 1.6, 3.2**
    #[test]
    fn prop_dead_key_trigger_transition(
        key in dead_key_strategy(),
        shift in any::<bool>()
    ) {
        let mut mapper = Mapper::new();
        let action = mapper.process_key(key, shift);

        // Dead key triggers should return Suppress action
        prop_assert_eq!(action, KeyAction::Suppress);

        // State should be PendingAccent with correct accent type
        // Based on corrected ABNT2 Positional Mapping:
        // ' (unshifted) -> Tilde, ' (shifted) -> Circumflex
        // [ (unshifted) -> Acute, [ (shifted) -> Grave
        match mapper.state() {
            MapperState::PendingAccent(accent) => {
                match (key, shift) {
                    (VirtualKey::Apostrophe, false) => prop_assert_eq!(*accent, AccentType::Tilde),
                    (VirtualKey::Apostrophe, true) => prop_assert_eq!(*accent, AccentType::Circumflex),
                    (VirtualKey::LeftBracket, false) => prop_assert_eq!(*accent, AccentType::Acute),
                    (VirtualKey::LeftBracket, true) => prop_assert_eq!(*accent, AccentType::Grave),
                    _ => prop_assert!(false, "Unexpected key"),
                }
            }
            _ => prop_assert!(false, "Expected PendingAccent state"),
        }
    }

    // **Feature: ghostkeys, Property 3: Dead Key Combination Correctness**
    // **Validates: Requirements 2.1-2.14, 3.3**
    #[test]
    fn prop_dead_key_combination_correctness(
        dead_key in dead_key_strategy(),
        shift_dead in any::<bool>(),
        follow_char in combinable_char_strategy(),
    ) {
        let mut mapper = Mapper::new();

        // Press dead key
        mapper.process_key(dead_key, shift_dead);

        // Determine expected accent based on corrected mapping:
        // ' (unshifted) -> Tilde, ' (shifted) -> Circumflex
        // [ (unshifted) -> Acute, [ (shifted) -> Grave
        let accent = match (dead_key, shift_dead) {
            (VirtualKey::Apostrophe, false) => AccentType::Tilde,
            (VirtualKey::Apostrophe, true) => AccentType::Circumflex,
            (VirtualKey::LeftBracket, false) => AccentType::Acute,
            (VirtualKey::LeftBracket, true) => AccentType::Grave,
            _ => return Ok(()), // Skip invalid combinations
        };

        // Press follow-up character
        let action = mapper.process_key(VirtualKey::Char(follow_char), false);

        // Check if this is a valid combination
        let expected = get_expected_combination(accent, follow_char);

        match (action, expected) {
            (KeyAction::Replace(c), Some(expected_char)) => {
                prop_assert_eq!(c, expected_char);
            }
            (KeyAction::ReplaceMultiple(_), None) => {
                // Non-combinable, which is fine
            }
            (KeyAction::Replace(_), None) => {
                // Got a replacement for non-combinable - this is wrong
                prop_assert!(false, "Got Replace for non-combinable character");
            }
            _ => {}
        }

        // State should return to Idle
        prop_assert_eq!(mapper.state(), &MapperState::Idle);
    }

    // **Feature: ghostkeys, Property 4: Non-Combinable Character Fallback**
    // **Validates: Requirements 2.15, 2.16, 3.4**
    #[test]
    fn prop_non_combinable_fallback(
        dead_key in dead_key_strategy(),
        shift_dead in any::<bool>(),
        follow_char in non_combinable_char_strategy(),
    ) {
        let mut mapper = Mapper::new();

        // Press dead key
        mapper.process_key(dead_key, shift_dead);

        // Get the accent character based on corrected mapping:
        // ' (unshifted) -> Tilde, ' (shifted) -> Circumflex
        // [ (unshifted) -> Acute, [ (shifted) -> Grave
        let accent = match (dead_key, shift_dead) {
            (VirtualKey::Apostrophe, false) => AccentType::Tilde,
            (VirtualKey::Apostrophe, true) => AccentType::Circumflex,
            (VirtualKey::LeftBracket, false) => AccentType::Acute,
            (VirtualKey::LeftBracket, true) => AccentType::Grave,
            _ => return Ok(()),
        };

        // Press non-combinable character
        let action = mapper.process_key(VirtualKey::Char(follow_char), false);

        // Should return ReplaceMultiple with accent + character
        match action {
            KeyAction::ReplaceMultiple(chars) => {
                prop_assert_eq!(chars.len(), 2);
                prop_assert_eq!(chars[0], accent.to_char());
                prop_assert_eq!(chars[1], follow_char);
            }
            _ => prop_assert!(false, "Expected ReplaceMultiple for non-combinable character"),
        }

        // State should return to Idle
        prop_assert_eq!(mapper.state(), &MapperState::Idle);
    }

    // **Feature: ghostkeys, Property 7: Passthrough Mode Transparency**
    // **Validates: Requirements 7.3**
    #[test]
    fn prop_passthrough_unhandled_keys(key_code in 0u8..128u8) {
        let mut mapper = Mapper::new();

        // Use Other for keys we don't handle
        let action = mapper.process_key(VirtualKey::Other, false);

        // Unhandled keys should pass through
        prop_assert_eq!(action, KeyAction::Pass);
    }
}

/// Helper function to get expected combination result
fn get_expected_combination(accent: AccentType, c: char) -> Option<char> {
    match (accent, c) {
        // Tilde
        (AccentType::Tilde, 'a') => Some('ã'),
        (AccentType::Tilde, 'A') => Some('Ã'),
        (AccentType::Tilde, 'o') => Some('õ'),
        (AccentType::Tilde, 'O') => Some('Õ'),
        (AccentType::Tilde, 'n') => Some('ñ'),
        (AccentType::Tilde, 'N') => Some('Ñ'),
        // Acute
        (AccentType::Acute, 'a') => Some('á'),
        (AccentType::Acute, 'A') => Some('Á'),
        (AccentType::Acute, 'e') => Some('é'),
        (AccentType::Acute, 'E') => Some('É'),
        (AccentType::Acute, 'i') => Some('í'),
        (AccentType::Acute, 'I') => Some('Í'),
        (AccentType::Acute, 'o') => Some('ó'),
        (AccentType::Acute, 'O') => Some('Ó'),
        (AccentType::Acute, 'u') => Some('ú'),
        (AccentType::Acute, 'U') => Some('Ú'),
        // Grave
        (AccentType::Grave, 'a') => Some('à'),
        (AccentType::Grave, 'A') => Some('À'),
        // Circumflex
        (AccentType::Circumflex, 'a') => Some('â'),
        (AccentType::Circumflex, 'A') => Some('Â'),
        (AccentType::Circumflex, 'e') => Some('ê'),
        (AccentType::Circumflex, 'E') => Some('Ê'),
        (AccentType::Circumflex, 'o') => Some('ô'),
        (AccentType::Circumflex, 'O') => Some('Ô'),
        _ => None,
    }
}

#[cfg(test)]
mod timeout_tests {
    use super::*;
    use std::thread::sleep;
    use std::time::Duration;

    // **Feature: ghostkeys, Property 5: State Machine Timeout Behavior**
    // **Validates: Requirements 3.5**
    #[test]
    fn test_timeout_outputs_accent_and_returns_to_idle() {
        let mut mapper = Mapper::new();

        // Press dead key
        mapper.process_key(VirtualKey::Apostrophe, false);
        assert!(matches!(mapper.state(), MapperState::PendingAccent(_)));

        // Wait for timeout (500ms + buffer)
        sleep(Duration::from_millis(550));

        // Check timeout
        let action = mapper.check_timeout();
        assert!(action.is_some());
        assert_eq!(action.unwrap(), KeyAction::Replace('~'));
        assert_eq!(mapper.state(), &MapperState::Idle);
    }

    #[test]
    fn test_no_timeout_before_500ms() {
        let mut mapper = Mapper::new();

        // Press dead key
        mapper.process_key(VirtualKey::Apostrophe, false);

        // Wait less than timeout
        sleep(Duration::from_millis(100));

        // Check timeout - should be None
        let action = mapper.check_timeout();
        assert!(action.is_none());
        assert!(matches!(mapper.state(), MapperState::PendingAccent(_)));
    }
}
