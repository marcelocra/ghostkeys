# 0002 - Use Position-Based Mapping Strategy

**Status:** Accepted

**Date:** 2025-12-05

**Deciders:** Marcelo Almeida (repository owner)

## Context

Brazilian developers face a daily dilemma:
- **ABNT2 layout**: Natural Portuguese typing, but coding shortcuts are wrong (`;` is in a different position)
- **US layout**: Perfect for coding, but Portuguese characters require awkward Alt codes or IME

GhostKeys needs to bridge this gap. The core question: how should we map keys?

**Constraints:**
- Must preserve coding shortcuts (Ctrl+C, Ctrl+V, etc.)
- Must feel natural for users with ABNT2 muscle memory
- Must work without changing OS settings
- Must support dead keys (accents: ´ ~ ^ `)

## Decision

Use **position-based mapping** that intercepts physical key positions and outputs ABNT2 characters, regardless of OS layout setting.

Key mappings:
| US Key | ABNT2 Output | Notes |
|--------|--------------|-------|
| `;` (next to L) | `ç/Ç` | Cedilla position |
| `'` (next to ;) | Dead key `~` | Tilde for ã, õ |
| `Shift+'` | Dead key `^` | Circumflex for â, ê, ô |
| `[` (next to P) | Dead key `´` | Acute for á, é, í, ó, ú |
| `Shift+[` | Dead key `` ` `` | Grave for à |
| `/` | `;/:` | Semicolon position |
| `]` | `[/{` | Bracket position |
| `\` | `]/}` | Close bracket position |

## Alternatives Considered

### Option 1: Character-based mapping

Map US characters to ABNT2 characters (e.g., when user types `;`, output `ç`).

**Rejected** because:
- Breaks if OS layout changes
- Doesn't account for physical key positions
- Confusing when visual keyboard doesn't match output

### Option 2: OS layout switching

Toggle between US and ABNT2 layouts at the OS level.

**Rejected** because:
- Requires OS configuration changes
- Breaks coding shortcuts when in ABNT2 mode
- User must remember to switch modes
- Doesn't solve the core problem (best of both worlds)

### Option 3: Full ABNT2 emulation

Emulate the entire ABNT2 layout, including all keys.

**Rejected** because:
- Unnecessary complexity
- Most keys are identical between layouts
- Only ~10 keys actually differ
- Higher risk of bugs

## Consequences

### Positive

- **Natural typing** - ABNT2 users can type without thinking about layout
- **Coding shortcuts preserved** - Ctrl+C, Ctrl+V, etc. work normally
- **No OS changes** - Works with OS set to US layout
- **Predictable** - Same physical key always produces same character
- **Dead key support** - Full accent support (tilde, acute, circumflex, grave)

### Negative

- **US keyboards only** - Assumes US physical keyboard layout
- **Learning curve** - Users must know which US keys map to ABNT2 positions
- **No visual feedback** - Keyboard labels don't match output

### Neutral

- Works in all applications (system-level interception)
- Requires documentation of key mappings for users

## Notes

- Mapping table defined in `src/mapper.rs`
- Dead key state machine handles accent combinations
- Related: [0003 - Thread-Local Mapper State](0003-thread-local-mapper-state.md)

**References:**
- [ABNT2 Layout Wikipedia](https://en.wikipedia.org/wiki/ABNT_keyboard)
- [US Layout Wikipedia](https://en.wikipedia.org/wiki/QWERTY)
