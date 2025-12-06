# ADR 002: Position-Based Mapping Strategy

## Status

Accepted

## Context

Users with ABNT2 muscle memory need to type Portuguese characters on US keyboards. We need to decide between:

1. **Character-based mapping**: Map US characters to ABNT2 characters (e.g., `;` → `ç`)
2. **Position-based mapping**: Map physical key positions regardless of OS layout
3. **Layout switching**: Toggle between US and ABNT2 OS layouts

## Decision

We will use **position-based mapping** that intercepts physical key positions and outputs ABNT2 characters.

## Rationale

- **Muscle Memory**: Users can type naturally without thinking about layout differences
- **No OS Changes**: Works with OS set to US layout (preserving coding shortcuts)
- **Predictable**: Same physical key always produces same character
- **Dead Keys**: Supports ABNT2 dead keys (tilde, acute, circumflex, grave)
- **Best of Both Worlds**: US layout for code, ABNT2 for text

## Implementation

Key mappings:
- `;` (next to L) → `ç/Ç`
- `'` (next to ;) → Tilde dead key `~`
- `[` (next to P) → Acute dead key `´`
- `Shift+[` → Grave dead key `` ` ``
- `Shift+'` → Circumflex dead key `^`
- `/` → `;/:`
- `]` → `[/{`
- `\` → `]/}`

## Consequences

### Positive

- Natural typing experience for ABNT2 users
- No need to change OS layout
- Coding shortcuts remain intact (Ctrl+C, Ctrl+V, etc.)
- Works in all applications

### Negative

- Only works on US physical keyboards
- Requires learning which US keys map to ABNT2 positions
- Cannot easily switch between layouts

## Alternatives Considered

- **Character mapping**: Would break with different OS layouts
- **Layout switching**: Requires OS changes, breaks coding shortcuts
