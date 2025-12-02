# GhostKeys

## Overview

GhostKeys is a system that enables fluent typing in Brazilian Portuguese (pt-BR) while keeping the operating system and the physical keyboard on the standard US layout. The goal is to avoid switching layouts and to preserve shortcut reliability across applications, while still achieving the convenience and speed of ABNT2-style accent input.

### Problem

-   **ABNT2 is superior for pt-BR typing**: It's faster, more comfortable, and more natural for writing Portuguese—dedicated accent keys and cedilla make typing feel effortless.
-   **US layout is essential for code and power users**: For developers and heavy keyboard users, US layout on a US physical keyboard is non-negotiable. Trying to use ABNT2 layout on US hardware creates constant friction:
    -   Simple tasks become multi-key ordeals (e.g., typing `/` requires `AltGr+Q` or switching layouts with `Win+Space`).
    -   Code editors, terminal tools, and keyboard-driven apps (like Vim/Neovim) become nearly unusable—muscle memory breaks, shortcuts fail, and productivity craters.
-   **Layout switching is a workflow killer**: Constantly toggling between `US` ↔ `US-International` ↔ `ABNT2` disrupts flow, introduces errors, and forces mental context-switching.
-   **No good middle ground exists**: US-International solves the shortcut problem but creates new friction—dead keys force multi-keypress sequences (`'+<space>`, `~+<space>`, `` `+<space> ``) for common programming characters (quotes, backticks, tilde), slowing down code editing. And it still doesn't match ABNT2 comfort/speed for pt-BR typing, leaving you stuck switching layouts constantly.

### Proposed Solution

-   A background process that performs real-time "layout-aware autocorrection": it intercepts keystrokes from a **physical US keyboard** with the **OS set to US layout**, and translates them as if the user were typing on an ABNT2 keyboard.
-   The user types using **ABNT2 muscle memory** (where keys are located on ABNT2), but on a US physical keyboard. GhostKeys maps those physical positions to the intended pt-BR characters.
-   The system acts like an instant autocorrect: when it detects an ABNT2-style sequence on US hardware, it replaces it with the intended Portuguese character immediately.

### Example

-   Typed on US layout (using ABNT2 key positions): `~Mergulhar de cal;'ao [e dif[icil~`
-   Corrected output: `"Mergulhar de calção é difícil"`
-   **Key insight**: This is **NOT** US-International dead-key behavior. GhostKeys maps **ABNT2 physical key positions** to their pt-BR meanings, even when typed on US hardware with US OS layout.
-   For instance, on ABNT2, the `;` key position produces `ç` directly. GhostKeys preserves that muscle memory:
    ```
    ~  => "    (tilde key on US keyboard maps to quote on ABNT2)
    ; => ç     (semicolon key on US keyboard is the cedilla key on ABNT2)
    'a => ã    (apostrophe + a, mapped from ABNT2 tilde dead-key position)
    [e => é    (left bracket on US is the acute accent dead-key on ABNT2)
    [i => í
    ```

### Activation and Context Awareness

-   To be truly seamless, GhostKeys should decide when to operate based on context (e.g., focused input fields, local text language detection, window/application hints).
-   For multilingual text, users may need an override to disable corrections temporarily. One option is to expose GhostKeys as a selectable keyboard layout so it can be toggled via the OS layout switcher (e.g., Win+Space), while still retaining the US base behavior for shortcuts.

### Cross-Platform Considerations

-   Windows, Linux, and macOS have different input method frameworks and APIs. A portable design likely requires:
    -   An abstraction layer over platform-specific text input hooks (e.g., IME/TSF on Windows, IBus/Fcitx on Linux, Input Method Kit on macOS).
    -   A shared core for parsing, normalization, and correction logic.
    -   Careful handling of security and privacy (e.g., processing locally, no network requirement, minimal access scope).

### Implementation Notes

-   A first Rust prototype can focus on the core transformation rules and a minimal input hook for one platform, then expand.
-   Early milestones may include:
    -   A pure library that maps US keystroke sequences to pt-BR characters.
    -   A simple tray/daemon with enable/disable toggle.
    -   Basic heuristics for activation (focused field detection, quick language hinting).

### Open Source and Security

-   Input interception is **extremely sensitive** from a security standpoint. A keylogger and a legitimate input method look nearly identical at the system level.
-   For this reason, GhostKeys should:
    -   Be fully **open source** to allow audits and build trust.
    -   Use only **trusted, well-supported libraries** for input handling—no obscure or unmaintained dependencies.
    -   Process all data **locally**, with no network requirement and minimal access scope.
-   An open-source approach also enables community contributions for platform support, rulesets, and security reviews.

GhostKeys aims to combine the reliability of a single US layout with the expressiveness of pt-BR typing, minimizing friction while respecting cross-platform constraints and user control.
