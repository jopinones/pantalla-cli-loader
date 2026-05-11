# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Early-stage prototype for the UI layer of `carga-cli`, a CSV-to-MySQL data loading tool. The repo contains two artifacts:

- [tweaks-panel.jsx](tweaks-panel.jsx) — A self-contained React component library for a floating settings/tweaks panel
- [tui_layout_csv2mysql.svg](tui_layout_csv2mysql.svg) — Reference mockup of the Terminal UI layout (folder browser + execution controls)

There is no `package.json`. These files are intended to be copy-pasted or manually imported into consuming projects — not published as an npm package.

## Architecture: tweaks-panel.jsx

A ~570-line, dependency-free React module. React itself must be present in the consuming app; everything else (styles, state, event handling) is self-contained.

**Public API:**
- `useTweaks(defaults)` — hook for state + persistence
- `TweaksPanel` — fixed-position floating container (bottom-right, draggable)
- Layout: `TweakSection`, `TweakRow`
- Controls: `TweakSlider`, `TweakToggle`, `TweakRadio`, `TweakSelect`, `TweakText`, `TweakNumber`, `TweakColor`, `TweakButton`

**Key internals:**
- All CSS lives in `__TWEAKS_STYLE` and is injected as a single `<style>` tag on mount
- `__twkIsLight()` uses relative luminance for automatic contrast detection
- Viewport clamping runs on `window.resize` to keep the panel in view
- Segmented controls fall back to `<select>` when option labels are long

**Host communication via `window.postMessage`:**

| Direction | Message key |
|-----------|-------------|
| Outbound (to parent) | `__edit_mode_available`, `__edit_mode_set_keys`, `__edit_mode_dismissed`, `__deck_rail_visible` |
| Inbound (from parent) | `__activate_edit_mode`, `__deactivate_edit_mode`, `__omelette_rail_enabled` |
| Same-window broadcast | CustomEvent `tweakchange` |

Deck stage rail visibility is persisted to `localStorage`.

**Design tokens:** primary `#D97757`, dark `#29261b`, light `#f6f4ef`. Supports compact/regular/comfy density and dark mode.
