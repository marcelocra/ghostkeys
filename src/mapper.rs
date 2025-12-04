//! ABNT2 position mapper and dead key state machine
//!
//! This module contains the core mapping logic that translates US keyboard
//! positions to ABNT2 characters. It is pure Rust with no platform dependencies,
//! making it testable on any OS.

use std::collections::HashMap;
use std::time::{Duration, Instant};

// Re-export KeyAction for convenience
pub use crate::interceptor::KeyAction;

/// Timeout for pending accent state (500ms)
const ACCENT_TIMEOUT: Duration = Duration::from_millis(500);

/// Virtual key codes for keys we intercept
/// These are platform-agnostic representations
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VirtualKey {
    /// Semicolon key (;) - maps to ç on ABNT2
    Semicolon,
    /// Apostrophe key (') - tilde/circumflex dead key on ABNT2
    Apostrophe,
    /// Left bracket key ([) - acute/grave dead key on ABNT2
    LeftBracket,
    /// Right bracket key (]) - maps to [ or { on ABNT2
    RightBracket,
    /// Backslash key (\) - maps to ] or } on ABNT2
    Backslash,
    /// Slash key (/) - maps to ; or : on ABNT2
    Slash,
    /// Regular character key
    Char(char),
    /// Space key
    Space,
    /// Other keys we don't handle
    Other,
}

/// Accent types for dead key handling
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AccentType {
    /// Tilde accent (~) - triggered by ' key on US (unshifted)
    Tilde,
    /// Acute accent (´) - triggered by [ key on US (unshifted)
    Acute,
    /// Grave accent (`) - triggered by Shift+[ on US
    Grave,
    /// Circumflex accent (^) - triggered by Shift+' on US
    Circumflex,
}

impl AccentType {
    /// Get the character representation of this accent
    pub fn to_char(self) -> char {
        match self {
            AccentType::Tilde => '~',
            AccentType::Acute => '´',
            AccentType::Grave => '`',
            AccentType::Circumflex => '^',
        }
    }
}

/// State of the mapper state machine
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MapperState {
    /// Idle state, waiting for input
    Idle,
    /// Pending accent, waiting for next character
    PendingAccent(AccentType),
}

/// ABNT2 position mapper
///
/// Handles position-based character mapping and dead key state machine.
pub struct Mapper {
    state: MapperState,
    last_accent_time: Option<Instant>,
    position_map: HashMap<(VirtualKey, bool), char>,
    accent_combinations: HashMap<(AccentType, char), char>,
}

impl Mapper {
    /// Create a new mapper with default ABNT2 mappings
    pub fn new() -> Self {
        let mut mapper = Self {
            state: MapperState::Idle,
            last_accent_time: None,
            position_map: HashMap::new(),
            accent_combinations: HashMap::new(),
        };
        mapper.init_position_map();
        mapper.init_accent_combinations();
        mapper
    }

    /// Initialize the position mapping table
    /// Based on ABNT2 Positional Mapping Reference Table
    fn init_position_map(&mut self) {
        // Direct position mappings: (key, shift) -> output char

        // ; (next to L) -> ç/Ç (ABNT2 Cedilla Position)
        self.position_map.insert((VirtualKey::Semicolon, false), 'ç');
        self.position_map.insert((VirtualKey::Semicolon, true), 'Ç');

        // ] (next to [) -> [/{ (ABNT2 Bracket Key Position)
        self.position_map.insert((VirtualKey::RightBracket, false), '[');
        self.position_map.insert((VirtualKey::RightBracket, true), '{');

        // \ (above Enter) -> ]/} (ABNT2 Close Bracket Position)
        self.position_map.insert((VirtualKey::Backslash, false), ']');
        self.position_map.insert((VirtualKey::Backslash, true), '}');

        // / (next to .) -> ;/: (ABNT2 Semicolon Position)
        self.position_map.insert((VirtualKey::Slash, false), ';');
        self.position_map.insert((VirtualKey::Slash, true), ':');
    }

    /// Initialize the accent combination table
    fn init_accent_combinations(&mut self) {
        // Tilde combinations
        self.accent_combinations.insert((AccentType::Tilde, 'a'), 'ã');
        self.accent_combinations.insert((AccentType::Tilde, 'A'), 'Ã');
        self.accent_combinations.insert((AccentType::Tilde, 'o'), 'õ');
        self.accent_combinations.insert((AccentType::Tilde, 'O'), 'Õ');
        self.accent_combinations.insert((AccentType::Tilde, 'n'), 'ñ');
        self.accent_combinations.insert((AccentType::Tilde, 'N'), 'Ñ');

        // Acute combinations
        self.accent_combinations.insert((AccentType::Acute, 'a'), 'á');
        self.accent_combinations.insert((AccentType::Acute, 'A'), 'Á');
        self.accent_combinations.insert((AccentType::Acute, 'e'), 'é');
        self.accent_combinations.insert((AccentType::Acute, 'E'), 'É');
        self.accent_combinations.insert((AccentType::Acute, 'i'), 'í');
        self.accent_combinations.insert((AccentType::Acute, 'I'), 'Í');
        self.accent_combinations.insert((AccentType::Acute, 'o'), 'ó');
        self.accent_combinations.insert((AccentType::Acute, 'O'), 'Ó');
        self.accent_combinations.insert((AccentType::Acute, 'u'), 'ú');
        self.accent_combinations.insert((AccentType::Acute, 'U'), 'Ú');

        // Grave combinations
        self.accent_combinations.insert((AccentType::Grave, 'a'), 'à');
        self.accent_combinations.insert((AccentType::Grave, 'A'), 'À');

        // Circumflex combinations
        self.accent_combinations.insert((AccentType::Circumflex, 'a'), 'â');
        self.accent_combinations.insert((AccentType::Circumflex, 'A'), 'Â');
        self.accent_combinations.insert((AccentType::Circumflex, 'e'), 'ê');
        self.accent_combinations.insert((AccentType::Circumflex, 'E'), 'Ê');
        self.accent_combinations.insert((AccentType::Circumflex, 'o'), 'ô');
        self.accent_combinations.insert((AccentType::Circumflex, 'O'), 'Ô');
    }

    /// Process a key press and return the action to take
    pub fn process_key(&mut self, key: VirtualKey, shift: bool) -> KeyAction {
        match &self.state {
            MapperState::Idle => self.process_idle(key, shift),
            MapperState::PendingAccent(accent) => {
                let accent = *accent;
                self.process_pending_accent(accent, key, shift)
            }
        }
    }

    /// Process a key in Idle state
    fn process_idle(&mut self, key: VirtualKey, shift: bool) -> KeyAction {
        // Check for dead key triggers
        if let Some(accent) = self.get_dead_key_accent(key, shift) {
            self.state = MapperState::PendingAccent(accent);
            self.last_accent_time = Some(Instant::now());
            return KeyAction::Suppress;
        }

        // Check for direct position mappings
        if let Some(&output) = self.position_map.get(&(key, shift)) {
            return KeyAction::Replace(output);
        }

        // Pass through unhandled keys
        KeyAction::Pass
    }

    /// Get the accent type for a dead key trigger, if any
    /// Based on ABNT2 Positional Mapping Reference Table
    fn get_dead_key_accent(&self, key: VirtualKey, shift: bool) -> Option<AccentType> {
        match (key, shift) {
            // ' (next to ;) -> Tilde (~) unshifted, Circumflex (^) shifted
            (VirtualKey::Apostrophe, false) => Some(AccentType::Tilde),
            (VirtualKey::Apostrophe, true) => Some(AccentType::Circumflex),
            // [ (next to P) -> Acute (´) unshifted, Grave (`) shifted
            (VirtualKey::LeftBracket, false) => Some(AccentType::Acute),
            (VirtualKey::LeftBracket, true) => Some(AccentType::Grave),
            _ => None,
        }
    }

    /// Process a key in PendingAccent state
    fn process_pending_accent(&mut self, accent: AccentType, key: VirtualKey, shift: bool) -> KeyAction {
        self.state = MapperState::Idle;
        self.last_accent_time = None;

        // Handle space: output just the accent character
        if key == VirtualKey::Space {
            return KeyAction::Replace(accent.to_char());
        }

        // Get the character for this key
        let char_key = match key {
            VirtualKey::Char(c) => {
                if shift {
                    c.to_ascii_uppercase()
                } else {
                    c.to_ascii_lowercase()
                }
            }
            _ => {
                // Non-character key: output accent + original key action
                return KeyAction::Replace(accent.to_char());
            }
        };

        // Check for accent combination
        if let Some(&combined) = self.accent_combinations.get(&(accent, char_key)) {
            return KeyAction::Replace(combined);
        }

        // Non-combinable character: output accent + character
        KeyAction::ReplaceMultiple(vec![accent.to_char(), char_key])
    }

    /// Check for timeout and return action if timeout occurred
    pub fn check_timeout(&mut self) -> Option<KeyAction> {
        if let MapperState::PendingAccent(accent) = &self.state {
            if let Some(time) = self.last_accent_time {
                if time.elapsed() >= ACCENT_TIMEOUT {
                    let accent_char = accent.to_char();
                    self.state = MapperState::Idle;
                    self.last_accent_time = None;
                    return Some(KeyAction::Replace(accent_char));
                }
            }
        }
        None
    }

    /// Reset the mapper to Idle state
    pub fn reset(&mut self) {
        self.state = MapperState::Idle;
        self.last_accent_time = None;
    }

    /// Get the current state (for testing)
    pub fn state(&self) -> &MapperState {
        &self.state
    }
}

impl Default for Mapper {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // === Direct Position Mapping Tests ===

    #[test]
    fn test_semicolon_to_cedilla() {
        let mut mapper = Mapper::new();
        // ; -> ç (ABNT2 Cedilla Position)
        assert_eq!(
            mapper.process_key(VirtualKey::Semicolon, false),
            KeyAction::Replace('ç')
        );
        // Shift+; -> Ç
        assert_eq!(
            mapper.process_key(VirtualKey::Semicolon, true),
            KeyAction::Replace('Ç')
        );
    }

    #[test]
    fn test_right_bracket_to_left_bracket() {
        let mut mapper = Mapper::new();
        // ] -> [ (ABNT2 Bracket Key Position)
        assert_eq!(
            mapper.process_key(VirtualKey::RightBracket, false),
            KeyAction::Replace('[')
        );
        // Shift+] -> {
        assert_eq!(
            mapper.process_key(VirtualKey::RightBracket, true),
            KeyAction::Replace('{')
        );
    }

    #[test]
    fn test_backslash_to_right_bracket() {
        let mut mapper = Mapper::new();
        // \ -> ] (ABNT2 Close Bracket Position)
        assert_eq!(
            mapper.process_key(VirtualKey::Backslash, false),
            KeyAction::Replace(']')
        );
        // Shift+\ -> }
        assert_eq!(
            mapper.process_key(VirtualKey::Backslash, true),
            KeyAction::Replace('}')
        );
    }

    #[test]
    fn test_slash_to_semicolon() {
        let mut mapper = Mapper::new();
        // / -> ; (ABNT2 Semicolon Position)
        assert_eq!(
            mapper.process_key(VirtualKey::Slash, false),
            KeyAction::Replace(';')
        );
        // Shift+/ -> :
        assert_eq!(
            mapper.process_key(VirtualKey::Slash, true),
            KeyAction::Replace(':')
        );
    }

    // === Dead Key Trigger Tests ===

    #[test]
    fn test_dead_key_tilde() {
        let mut mapper = Mapper::new();

        // ' (unshifted) -> tilde dead key
        assert_eq!(
            mapper.process_key(VirtualKey::Apostrophe, false),
            KeyAction::Suppress
        );
        assert_eq!(mapper.state(), &MapperState::PendingAccent(AccentType::Tilde));

        // Press 'a' -> should produce ã
        assert_eq!(
            mapper.process_key(VirtualKey::Char('a'), false),
            KeyAction::Replace('ã')
        );
        assert_eq!(mapper.state(), &MapperState::Idle);
    }

    #[test]
    fn test_dead_key_circumflex() {
        let mut mapper = Mapper::new();

        // Shift+' -> circumflex dead key
        assert_eq!(
            mapper.process_key(VirtualKey::Apostrophe, true),
            KeyAction::Suppress
        );
        assert_eq!(mapper.state(), &MapperState::PendingAccent(AccentType::Circumflex));

        // Press 'a' -> should produce â
        assert_eq!(
            mapper.process_key(VirtualKey::Char('a'), false),
            KeyAction::Replace('â')
        );
        assert_eq!(mapper.state(), &MapperState::Idle);
    }

    #[test]
    fn test_dead_key_acute() {
        let mut mapper = Mapper::new();

        // [ (unshifted) -> acute dead key
        assert_eq!(
            mapper.process_key(VirtualKey::LeftBracket, false),
            KeyAction::Suppress
        );
        assert_eq!(mapper.state(), &MapperState::PendingAccent(AccentType::Acute));

        // Press 'e' -> should produce é
        assert_eq!(
            mapper.process_key(VirtualKey::Char('e'), false),
            KeyAction::Replace('é')
        );
    }

    #[test]
    fn test_dead_key_grave() {
        let mut mapper = Mapper::new();

        // Shift+[ -> grave dead key
        assert_eq!(
            mapper.process_key(VirtualKey::LeftBracket, true),
            KeyAction::Suppress
        );
        assert_eq!(mapper.state(), &MapperState::PendingAccent(AccentType::Grave));

        // Press 'a' -> should produce à
        assert_eq!(
            mapper.process_key(VirtualKey::Char('a'), false),
            KeyAction::Replace('à')
        );
    }

    // === Dead Key Combination Tests ===

    #[test]
    fn test_dead_key_non_combinable() {
        let mut mapper = Mapper::new();

        // Press apostrophe (tilde dead key)
        mapper.process_key(VirtualKey::Apostrophe, false);

        // Press 'x' (non-combinable) -> should produce ~ followed by x
        assert_eq!(
            mapper.process_key(VirtualKey::Char('x'), false),
            KeyAction::ReplaceMultiple(vec!['~', 'x'])
        );
    }

    #[test]
    fn test_dead_key_space() {
        let mut mapper = Mapper::new();

        // Press apostrophe (tilde dead key)
        mapper.process_key(VirtualKey::Apostrophe, false);

        // Press space -> should produce just ~
        assert_eq!(
            mapper.process_key(VirtualKey::Space, false),
            KeyAction::Replace('~')
        );
    }

    #[test]
    fn test_passthrough_unhandled_keys() {
        let mut mapper = Mapper::new();
        assert_eq!(
            mapper.process_key(VirtualKey::Other, false),
            KeyAction::Pass
        );
    }
}
