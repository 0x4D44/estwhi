# Estimation Whist (EstWhi)

A modern, high-performance Rust port of the classic Windows 3.1 card game "Estimation Whist" (also known as "Oh Hell").

![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)
![Platform: Windows](https://img.shields.io/badge/platform-Windows-blue)
![Language: Rust](https://img.shields.io/badge/language-Rust-orange)
![Tests](https://img.shields.io/badge/tests-passing-brightgreen)
![Coverage](https://img.shields.io/badge/coverage-%3E99%25%20logic-brightgreen)

## About

Estimation Whist is a strategic trick-taking card game where players must predict exactly how many tricks they will win each hand. This project modernizes the original Turbo Pascal Windows 3.1 application while strictly preserving its classic "pixel-perfect" look, gameplay rules, and charm.

Under the hood, it uses a **modern "Humble Object" architecture**, separating the pure game logic (platform-agnostic) from the Windows GDI rendering layer, allowing for **near 100% logic test coverage**.

## Features

- **Classic Experience:** Faithful reproduction of the Windows 3.1 UI using native Win32 APIs.
- **Configurable Rules:**
  - 2-6 Player support.
  - "Mountain Deal" mode (1 to 15 cards and back to 1).
  - Scoring: Vanilla (10 + tricks) or Squared (10 + tricks²).
  - "Hard Score" penalty option.
- **Smart AI:** Computer players with weighted bidding heuristics and legal play validation.
- **Persistence:** Game settings and high scores saved to Windows Registry.
- **Modern Rendering:** Double-buffered GDI drawing for flicker-free resizing and High-DPI support.
- **Easter Eggs:** Legacy "Random Things" and "Cheat Window" features preserved.

## Architecture

The project is a Cargo workspace designed for testability:

### 1. `estwhi-core` (Game Engine)
A pure Rust library with **>99% test coverage**. It contains no platform dependencies.
- **`lib.rs`**: Core rules, card management, shuffling, trick resolution.
- **`ai.rs`**: Bidding heuristics and card selection logic.
- **`state.rs`**: Game flow state machine (turn rotation, deal calculation).
- **`config.rs`**: Configuration validation logic.

### 2. `estwhi` (Windows Application)
The Win32 executable that acts as a "Humble View".
- **`game_controller.rs`**: **[NEW]** Encapsulates the entire game orchestration loop (Human turns, AI turns, Hand scoring). Fully unit-tested without a GUI.
- **`main.rs`**: Thin Window procedure layer. Handles message loop, painting, and delegates events to the controller.
- **`registry.rs`**: Type-safe Registry I/O with RAII-based test overrides.
- **`rendering.rs`**: GDI drawing calls. Geometry logic (`calculate_hand_layout`) is extracted and tested.
- **Resources**: Embedded icons and menus via `build.rs` and `.rc` files.

### 3. `tools/`
Helper utilities for development.
- **`extract-res`**: Extracts bitmaps from the original 16-bit executable or resource files.
- **`card-normalizer`**: Processes card assets for transparency (greenscreen removal).

## Prerequisites

### Required
- **Windows OS**: Windows 7 or newer (Win32 API requirement).
- **Rust Toolchain**: 1.70 or newer (via [rustup](https://rustup.rs/)).
- **Windows SDK**: Required for `rc.exe` (resource compilation).
  - *Typically installed with Visual Studio Build Tools ("Desktop development with C++").*

## Building & Running

### Standard Build
This builds the fully featured game with icon and menus.

```powershell
git clone https://github.com/0x4D44/estwhi.git
cd estwhi
cargo build --release
```
*Executable location:* `target/release/estwhi.exe`

### Build Without Windows SDK
If you lack `rc.exe`, you can still build a functional game (without custom icon/dialogs):

```powershell
cargo build --release --features no-res
```

## Testing & Quality

The project maintains a very high standard of code quality.

### Unit Tests
The workspace has comprehensive test coverage (~66 tests) verifying all logic paths.

```powershell
# Run all tests (Core Logic + Game Controller + Registry + Config)
cargo test --workspace
```

### Coverage Areas
- **Game Rules:** Bidding, Playing, Scoring (Vanilla/Squared/Hard).
- **AI Behavior:** Correct bidding math and legal card selection.
- **Game Flow:** Dealing, Trick completion, Round transitions.
- **UI Logic:** Layout calculations, "Random Things" collision physics.
- **Persistence:** Registry read/write round-trips.

### Code Style
Strict adherence to `rustfmt` and `clippy`.
```powershell
cargo fmt --all -- --check
cargo clippy --all-targets --workspace -- -D warnings
```

## Usage

1. **Launch:** Run `estwhi.exe`.
2. **Options:** Go to **Game → Options** to set players (2-6), max cards, and scoring style.
3. **Play:** Press **F2** or click **Deal** to start.
   - **Bidding:** Guess your tricks! (Total bids often don't equal cards dealt).
   - **Playing:** Follow suit if possible. Trump beats lead. High card wins.
   - **Scoring:** Hit your bid exactly to get the bonus.

### Tools Usage

**Extract Resources:**
```powershell
# Extracts raw bitmaps from the original executable (if you have it)
cargo run -p extract-res -- original_game.exe ./output_folder
```

**Normalize Cards:**
```powershell
# Converts green backgrounds to transparent/white for assets
cargo run -p card-normalizer -- ./raw_cards ./processed_cards
```

## Credits

- **Original Game:** Turbo Pascal Windows 3.1 version (c. 1990s).
- **Rust Port:** Modern Win32 implementation (2025-2026).
- **Assets:** Original bitmap assets preserved for nostalgia.

---
**Built with Rust 🦀 & Win32 API**
