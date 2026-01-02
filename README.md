# Estimation Whist (EstWhi)

A modern, high-performance Rust port of the classic Windows 3.1 card game "Estimation Whist" (also known as "Oh Hell").

![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)
![Platform: Windows](https://img.shields.io/badge/platform-Windows-blue)
![Language: Rust](https://img.shields.io/badge/language-Rust-orange)
![Tests](https://img.shields.io/badge/tests-passing-brightgreen)

## About

Estimation Whist is a strategic trick-taking card game where players must predict exactly how many tricks they will win each hand. This project modernizes the original Turbo Pascal Windows 3.1 application while strictly preserving its classic "pixel-perfect" look, gameplay rules, and charm.

Under the hood, it uses a **modern "Humble Object" architecture**, separating the pure game logic (platform-agnostic) from the Windows GDI rendering layer.

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

The project is a Cargo workspace divided into two primary crates:

### 1. `estwhi-core` (Game Engine)
A pure Rust library with **>98% logic test coverage**. It contains no platform dependencies.
- **`lib.rs`**: Core rules, card management, shuffling, trick resolution.
- **`ai.rs`**: Bidding heuristics and card selection logic.
- **`state.rs`**: Game flow state machine (turn rotation, deal calculation).
- **`config.rs`**: Configuration validation logic.

### 2. `estwhi` (Windows Application)
The Win32 executable that acts as a "Humble View".
- **`main.rs`**: Window procedure, message loop, GDI rendering.
- **`registry.rs`**: Type-safe Registry I/O (integration tested).
- **Resources**: Embedded icons and menus via `build.rs` and `.rc` files.

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

### Running Tests
The project maintains a high standard of code quality with comprehensive unit and integration tests.

```powershell
# Run all tests (Logic + Registry integration)
cargo test --all

# Run only core logic tests
cargo test -p estwhi-core
```

## Usage

1. **Launch:** Run `estwhi.exe`.
2. **Options:** Go to **Game → Options** to set players (2-6), max cards, and scoring style.
3. **Play:** Press **F2** or click **Deal** to start.
   - **Bidding:** Guess your tricks! (Total bids often don't equal cards dealt).
   - **Playing:** Follow suit if possible. Trump beats lead. High card wins.
   - **Scoring:** Hit your bid exactly to get the bonus.

## Development

### Code Quality
This project adheres to strict Rust standards.
- **Formatting:** `cargo fmt --all`
- **Linting:** `cargo clippy --all-targets -- -D warnings`
- **Cleanliness:** No warnings allowed in release builds.

### Directory Layout
- `estwhi-core/`: Logic library.
- `estwhi/`: GUI Application.
- `tools/`: Utilities for asset extraction (`extract-res`, `card-normalizer`).
- `assets/`: Raw bitmaps and processed card images.
- `docs/`: Detailed design documents and modernization logs.

## Troubleshooting

| Issue | Solution |
| :--- | :--- |
| **`rc.exe` not found** | Install Visual Studio Build Tools or use `--features no-res`. |
| **Linker errors** | Ensure `rustup default stable-msvc` is set. |
| **Missing Cards** | Ensure `assets/cards/71x96/` contains the `.bmp` files. |
| **Test Failures** | Ensure no other instance is locking the Registry key `Software\Estwhi\Test_...`. |

## Credits

- **Original Game:** Turbo Pascal Windows 3.1 version (c. 1990s).
- **Rust Port:** Modern Win32 implementation (2025-2026).
- **Assets:** Original bitmap assets preserved for nostalgia.

---
**Built with Rust 🦀 & Win32 API**