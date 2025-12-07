# Requirements Document

## Introduction

GhostKeys is a Windows System Tray application written in Rust that enables fluent Brazilian Portuguese (pt-BR) typing while keeping the operating system set to the standard **US keyboard layout**. The application intercepts keystrokes and translates them as if the user were typing on an ABNT2 keyboard, allowing users to leverage ABNT2 muscle memory on US physical hardware.

**Key Concept:** GhostKeys maps **ABNT2 physical key positions** to their pt-BR meanings. When the user presses a key on their US keyboard, GhostKeys checks where that key would be on an ABNT2 layout and outputs the corresponding Brazilian Portuguese character. This is NOT US-International dead-key behavior—it's position-based layout translation.

**Prerequisite:** The system must have Windows configured with the **English (US)** keyboard layout (not US-International). GhostKeys intercepts raw keystrokes and performs the ABNT2 position mapping in software.

## Glossary

- **GhostKeys**: The Windows System Tray application that intercepts and remaps keyboard input
- **ABNT2**: The Brazilian keyboard layout standard with dedicated keys for `ç`, accent dead keys, and Portuguese punctuation
- **US Layout**: The standard United States keyboard layout without dead keys
- **Position Mapping**: Translating a key press based on its physical position on ABNT2, not its US character
- **Dead Key (ABNT2)**: A key on ABNT2 that modifies the next character (e.g., acute accent `´`, tilde `~`)
- **Keyboard Hook**: A system-level mechanism that intercepts keyboard events before they reach applications
- **State Machine**: A computational model that transitions between states based on input events
- **Active Mode**: A mode where GhostKeys intercepts and remaps keyboard input
- **Passthrough Mode**: A mode where GhostKeys does not modify keyboard input
- **System Tray**: The notification area in the Windows taskbar where background applications display icons
- **Recursion Protection**: An internal flag (`IS_INJECTING`) that prevents the hook from processing keys injected by GhostKeys itself
- **Thread-Local Storage**: A mechanism to store data that is unique to each thread, avoiding mutex locking overhead

## Requirements

### Requirement 1: ABNT2 Position-Based Character Mapping

**User Story:** As a Brazilian Portuguese user with ABNT2 muscle memory, I want to type on a US keyboard with US layout and have GhostKeys output the characters I expect from ABNT2 positions, so that I can type Portuguese naturally without switching layouts.

#### ABNT2 Positional Mapping Reference Table

| US Key (Physical) | Output (No Shift) | Output (Shift) | Logic |
| :--- | :--- | :--- | :--- |
| `[` (next to P) | Dead Key Acute (´) | Dead Key Backtick (`) | ABNT2 Accent Key Position |
| `]` (next to `[`) | `[` | `{` | ABNT2 Bracket Key Position |
| `\` (above Enter) | `]` | `}` | ABNT2 Close Bracket Position |
| `'` (next to ;) | Dead Key Tilde (~) | Dead Key Circumflex (^) | ABNT2 Tilde Key Position |
| `;` (next to L) | `ç` | `Ç` | ABNT2 Cedilla Position |
| `/` (next to .) | `;` | `:` | ABNT2 Semicolon Position |

#### Acceptance Criteria

1. WHEN the user presses the `;` key (US semicolon position), THEN GhostKeys SHALL output the character `ç` (ABNT2 Cedilla Position)
2. WHEN the user presses `Shift` + `;` key, THEN GhostKeys SHALL output the character `Ç`
3. WHEN the user presses the `'` key (US apostrophe position), THEN GhostKeys SHALL enter the tilde dead key state (ABNT2 Tilde Key Position)
4. WHEN the user presses `Shift` + `'` key, THEN GhostKeys SHALL enter the circumflex dead key state (ABNT2 Circumflex Position)
5. WHEN the user presses the `[` key (US left bracket position), THEN GhostKeys SHALL enter the acute accent dead key state (ABNT2 Accent Key Position)
6. WHEN the user presses `Shift` + `[` key, THEN GhostKeys SHALL enter the grave accent dead key state (ABNT2 Backtick Position)
7. WHEN the user presses the `]` key (US right bracket position), THEN GhostKeys SHALL output the character `[` (ABNT2 Bracket Key Position)
8. WHEN the user presses `Shift` + `]` key, THEN GhostKeys SHALL output the character `{`
9. WHEN the user presses the `\` key (US backslash position), THEN GhostKeys SHALL output the character `]` (ABNT2 Close Bracket Position)
10. WHEN the user presses `Shift` + `\` key, THEN GhostKeys SHALL output the character `}`
11. WHEN the user presses the `/` key (US slash position), THEN GhostKeys SHALL output the character `;` (ABNT2 Semicolon Position)
12. WHEN the user presses `Shift` + `/` key, THEN GhostKeys SHALL output the character `:`

### Requirement 2: ABNT2 Dead Key Sequences

**User Story:** As a user, I want GhostKeys to handle ABNT2-style dead key sequences, so that I can type accented characters using the same key combinations I would use on an ABNT2 keyboard.

#### Acceptance Criteria

1. WHEN the user types `'` (tilde dead key) followed by `a`, THEN GhostKeys SHALL output the character `ã`
2. WHEN the user types `'` (tilde dead key) followed by `A`, THEN GhostKeys SHALL output the character `Ã`
3. WHEN the user types `'` (tilde dead key) followed by `o`, THEN GhostKeys SHALL output the character `õ`
4. WHEN the user types `'` (tilde dead key) followed by `O`, THEN GhostKeys SHALL output the character `Õ`
5. WHEN the user types `'` (tilde dead key) followed by `n`, THEN GhostKeys SHALL output the character `ñ`
6. WHEN the user types `[` (acute dead key) followed by `a`, THEN GhostKeys SHALL output the character `á`
7. WHEN the user types `[` (acute dead key) followed by `e`, THEN GhostKeys SHALL output the character `é`
8. WHEN the user types `[` (acute dead key) followed by `i`, THEN GhostKeys SHALL output the character `í`
9. WHEN the user types `[` (acute dead key) followed by `o`, THEN GhostKeys SHALL output the character `ó`
10. WHEN the user types `[` (acute dead key) followed by `u`, THEN GhostKeys SHALL output the character `ú`
11. WHEN the user types `Shift+[` (grave dead key) followed by `a`, THEN GhostKeys SHALL output the character `à`
12. WHEN the user types `Shift+'` (circumflex dead key) followed by `a`, THEN GhostKeys SHALL output the character `â`
13. WHEN the user types `Shift+'` (circumflex dead key) followed by `e`, THEN GhostKeys SHALL output the character `ê`
14. WHEN the user types `Shift+'` (circumflex dead key) followed by `o`, THEN GhostKeys SHALL output the character `ô`
15. WHEN the user types a dead key followed by a non-combinable character, THEN GhostKeys SHALL output the dead key character followed by the typed character
16. WHEN the user types a dead key followed by a space, THEN GhostKeys SHALL output only the dead key character

### Requirement 3: State Machine for Dead Key Handling

**User Story:** As a user, I want GhostKeys to track dead key state accurately, so that character sequences are processed correctly without losing keystrokes.

#### Acceptance Criteria

1. WHEN GhostKeys starts, THEN GhostKeys SHALL initialize the state machine in the Idle state
2. WHEN the state machine is in Idle state and the user presses a dead key trigger (`, [, ]), THEN GhostKeys SHALL transition to the PendingAccent state and store the accent type
3. WHEN the state machine is in PendingAccent state and the user presses a combinable character, THEN GhostKeys SHALL output the combined character and transition to Idle state
4. WHEN the state machine is in PendingAccent state and the user presses a non-combinable character, THEN GhostKeys SHALL output the accent character followed by the pressed character and transition to Idle state
5. WHEN the state machine is in PendingAccent state and a timeout of 500 milliseconds elapses, THEN GhostKeys SHALL output the accent character and transition to Idle state

### Requirement 4: System Tray Integration

**User Story:** As a user, I want GhostKeys to run in the System Tray with visual feedback and a context menu, so that I can see the current state and control the application without a visible window.

#### Acceptance Criteria

1. WHEN GhostKeys starts, THEN GhostKeys SHALL display a 32x32 pixel icon in the Windows System Tray
2. WHEN GhostKeys is in Active Mode, THEN GhostKeys SHALL display the tray icon with a green border or center indicator
3. WHEN GhostKeys is in Passthrough Mode, THEN GhostKeys SHALL display the tray icon with a yellow or orange indicator
4. WHEN the user hovers over the System Tray icon, THEN GhostKeys SHALL display a tooltip showing the current state (e.g., "GhostKeys - Active" or "GhostKeys - Paused")
5. WHEN the user right-clicks the System Tray icon, THEN GhostKeys SHALL display a context menu with options
6. WHEN the context menu is displayed, THEN GhostKeys SHALL show a non-clickable status indicator item displaying the current state
7. WHEN the user selects "Pause" from the context menu, THEN GhostKeys SHALL enter Passthrough Mode and update the menu to show "Resume"
8. WHEN the user selects "Resume" from the context menu, THEN GhostKeys SHALL enter Active Mode and update the menu to show "Pause"
9. WHEN the user selects "Help / Mappings" from the context menu, THEN GhostKeys SHALL open a native Windows Message Box displaying the cheat sheet of US key to ABNT2 character mappings
10. WHEN the user selects "About" from the context menu, THEN GhostKeys SHALL open a native dialog displaying version information and credits
11. WHEN the user selects "Exit" from the context menu, THEN GhostKeys SHALL release the keyboard hook and terminate gracefully
12. WHEN GhostKeys starts, THEN GhostKeys SHALL default to Active Mode

### Requirement 5: Performance and Latency

**User Story:** As a user, I want GhostKeys to process keystrokes with minimal latency, so that my typing experience remains smooth and responsive.

#### Acceptance Criteria

1. WHILE GhostKeys is processing a keystroke, GhostKeys SHALL complete the processing within 10 milliseconds
2. WHILE GhostKeys is running, GhostKeys SHALL consume less than 50 megabytes of memory
3. WHILE GhostKeys is idle, GhostKeys SHALL consume less than 1 percent CPU usage
4. WHILE GhostKeys is processing keystrokes, GhostKeys SHALL use thread-local storage for the Mapper to avoid mutex locking on the hot path

### Requirement 6: Panic Safety and Hook Release

**User Story:** As a user, I want GhostKeys to release the keyboard hook immediately if it crashes, so that my keyboard remains functional.

#### Acceptance Criteria

1. WHEN GhostKeys encounters an unrecoverable error, THEN GhostKeys SHALL release the keyboard hook before terminating
2. WHEN GhostKeys receives a termination signal, THEN GhostKeys SHALL release the keyboard hook within 100 milliseconds
3. WHEN the keyboard hook thread panics, THEN GhostKeys SHALL catch the panic and release the hook before propagating the error
4. WHEN GhostKeys starts, THEN GhostKeys SHALL register a panic handler that ensures hook release

### Requirement 7: Keyboard Hook Management

**User Story:** As a system administrator, I want GhostKeys to manage the keyboard hook responsibly, so that it does not interfere with other applications or system stability.

#### Acceptance Criteria

1. WHEN GhostKeys starts, THEN GhostKeys SHALL install a low-level keyboard hook using the Windows API (WH_KEYBOARD_LL via windows-rs)
2. WHEN GhostKeys is in Active Mode and intercepts a remappable key, THEN GhostKeys SHALL suppress the original keystroke and inject the replacement character using SendInput
3. WHEN GhostKeys is in Passthrough Mode, THEN GhostKeys SHALL allow all keystrokes to pass through unmodified
4. WHEN GhostKeys terminates, THEN GhostKeys SHALL uninstall the keyboard hook and release all system resources
5. WHEN GhostKeys injects a replacement character, THEN GhostKeys SHALL set an internal recursion protection flag (IS_INJECTING) to prevent the hook from processing injected keys



### Requirement 8: Cross-Platform Development Support

**User Story:** As a developer, I want to develop and test GhostKeys on Linux while targeting Windows, so that I can use my preferred development environment without needing a Windows machine for every code change.

#### Acceptance Criteria

1. WHEN a developer builds GhostKeys on Linux, THEN the build system SHALL support cross-compilation to Windows using cargo-xwin with the MSVC toolchain
2. WHEN a developer runs tests on Linux, THEN the test suite SHALL execute all pure logic tests (mapper, state machine) without requiring Windows
3. WHEN the codebase is structured, THEN GhostKeys SHALL use a KeyboardInterceptor trait to abstract platform-specific code into separate modules (windows.rs, linux.rs)
4. WHEN a developer uses VS Code, THEN the project SHALL support development inside a Linux DevContainer

### Requirement 9: Automation and Quality Assurance

**User Story:** As a developer, I want automated quality checks and release processes, so that I can maintain code quality and release new versions efficiently.

#### Acceptance Criteria

1. WHEN a developer saves a code file, THEN Kiro agent hooks SHALL automatically run cargo check via the quality-control.json hook configuration
2. WHEN code is pushed to the repository, THEN the CI pipeline (ci.yml) SHALL run tests and build on a windows-latest GitHub Actions runner
3. WHEN a version tag (v*) is pushed, THEN the release pipeline (release.yml) SHALL build the release binary
4. WHEN a release is created, THEN the release pipeline SHALL generate a changelog automatically using git-cliff and Conventional Commits
5. WHEN a release is created, THEN the release pipeline SHALL calculate SHA256 checksums for all artifacts
6. WHEN a release is created, THEN the release pipeline SHALL publish a GitHub Release with all artifacts attached

### Requirement 10: Documentation Standards

**User Story:** As a developer or contributor, I want clear documentation standards, so that I can understand the project architecture and contribute effectively.

#### Acceptance Criteria

1. WHEN architectural decisions are made, THEN the team SHALL document them as Architecture Decision Records (ADRs) in the docs/adr/ directory
2. WHEN the project is configured, THEN the project SHALL maintain requirements and design documents in the .kiro/specs/ directory as the source of truth
3. WHEN AI agents interact with the project, THEN the project SHALL provide context via a Static Context Manifest (.kiro/context_manifest.json)
