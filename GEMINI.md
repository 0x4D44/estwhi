# Estimation Whist (EstWhi) - Project Context

## Project Overview
**Estimation Whist** is a modern Rust port of a classic Windows 3.1 card game (originally written in Turbo Pascal). The project aims to modernize the codebase while strictly preserving the original gameplay, rules, and "classic" Windows UI feel.

**Architecture:**
*   **Language:** Rust (2021 edition)
*   **Platform:** Windows (Win32 API via `windows` crate)
*   **Workspace Structure:**
    *   `estwhi-core/`: Platform-independent game logic (pure Rust, minimal dependencies).
    *   `estwhi/`: The main Windows GUI application (monolithic, heavily uses `unsafe` Win32 calls).
    *   `tools/`: Utilities for resource extraction and normalization (`extract-res`, `card-normalizer`).

## Status & Roadmap (as of Nov 2025)
**Overall Quality:** B+ (Good foundation, functional, but needs engineering improvements).
**Core Logic:** Excellent (100% score), well-tested.
**UI/System:** Functional but brittle.

**Current Active Tasks (Priority High):**
1.  **Refactoring `main.rs`:** The file is >4,000 lines. Needs splitting into modules (`window`, `rendering`, `dialogs`, `game_state`).
2.  **Safety Hardening:**
    *   Replace ~117 `unwrap()` calls with proper error handling (especially in UI/Resource loading).
    *   Document ~60 `unsafe` blocks with `// SAFETY:` comments.
3.  **Testing:** Add integration tests for full game flow and unit tests for UI logic (positioning/layout).
4.  **Registry:** Refactor `registry.rs` to reduce duplication and improve error handling.

## Building & Running

### Prerequisites
*   **OS:** Windows (Win32 API requirement).
*   **Tools:** Rust toolchain, Visual Studio Build Tools (for `rc.exe` / Windows SDK).

### Commands
*   **Build (Release):** `cargo build --release`
    *   *Note:* If Windows SDK is missing, use: `cargo build --release --features no-res` (skips icon/dialogs).
*   **Run:** `cargo run --release`
*   **Test:** `cargo test --all`
*   **Format/Lint:** `cargo fmt --all && cargo clippy --all -- -D warnings`

## Key Files & Directories

| Path | Description |
| :--- | :--- |
| `estwhi-core/src/lib.rs` | **Core Logic.** The "brain" of the game. Pure Rust. High quality. |
| `estwhi/src/main.rs` | **Main Application.** The "monolith." Contains WindowProc, GDI rendering, Game Loop. Needs refactoring. |
| `estwhi/src/registry.rs` | **Persistence.** Handles Windows Registry operations. Needs cleanup. |
| `estwhi/resources/` | **Assets.** `.rc` files and icons. |
| `wrk_docs/` | **Documentation.** extensive design docs and code reviews. **READ THESE BEFORE MAJOR CHANGES.** |

## Development Conventions

*   **Legacy Compatibility:** Changes MUST preserve original game rules, scoring algorithms (Vanilla/Squared), and Card ID schemes (1-52).
*   **Win32 Style:** The UI aims for a "pixel-perfect" or "classic" Windows 95/3.1 aesthetic.
*   **Safety:** New code should avoid `unwrap()`. All `unsafe` blocks must be justified.
*   **Testing:** Logic changes must be covered by tests in `estwhi-core`.

## Common Issues / Troubleshooting
*   **`rc.exe` not found:** Install Windows SDK or use `--features no-res`.
*   **Panics:** Often due to unhandled `unwrap()` calls in `main.rs` during resource creation failures.
