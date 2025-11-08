# Estimation Whist

A modern Rust port of the classic Windows 3.1 card game "Estimation Whist" (also known as "Oh Hell").

## About

Estimation Whist is a trick-taking card game where players must predict exactly how many tricks they will win each hand. This project modernizes the original Turbo Pascal Windows 3.1 application while preserving its classic look and gameplay.

## Features

- Classic Windows 3.1-style UI using Win32 APIs
- 2-6 player support
- Configurable game rules:
  - Vanilla or Squared scoring modes
  - Optional "Hard Score" penalty mode
  - Maximum cards per hand (1-15)
- High score persistence via Windows Registry
- DPI-aware rendering for modern displays
- Optional "cheat cards" debug window
- Legacy "Random Things" Easter egg feature

## Prerequisites

### Required

- **Rust** (1.70 or newer)
  - Install from [https://rustup.rs/](https://rustup.rs/)
- **Windows 7 or newer**
  - This is a Win32 application and requires Windows
- **Windows SDK**
  - Required for resource compilation (`.rc` files)
  - Typically installed with Visual Studio Build Tools or full Visual Studio

### Optional

- **Visual Studio Build Tools** (recommended)
  - Download from: [https://visualstudio.microsoft.com/downloads/](https://visualstudio.microsoft.com/downloads/)
  - Select "Desktop development with C++" workload
  - This provides the Windows SDK and `rc.exe` resource compiler

## Building

### Standard Build

```bash
# Clone the repository
git clone https://github.com/0x4D44/estwhi.git
cd estwhi

# Build the project
cargo build --release

# The executable will be at: target/release/estwhi.exe
```

### Build Without Resources (No Windows SDK Required)

If you don't have the Windows SDK installed, you can build without resource compilation:

```bash
cargo build --release --features no-res
```

**Note:** This will skip embedding the application icon and dialogs. The game will create menus programmatically and function normally, but won't have custom dialogs or an icon.

### Running Tests

```bash
# Run all tests
cargo test --all

# Run only core library tests
cargo test -p estwhi-core

# Run with output
cargo test -- --nocapture
```

## Project Structure

```
estwhi/
├── estwhi-core/          # Platform-independent game logic library
│   └── src/lib.rs        # Core game algorithms, scoring, trick resolution
├── estwhi/               # Windows GUI application
│   ├── src/
│   │   ├── main.rs       # Win32 UI and game state
│   │   ├── registry.rs   # Windows Registry persistence
│   │   └── build.rs      # Resource compilation build script
│   ├── resources/
│   │   ├── app.rc        # Dialog and menu definitions
│   │   ├── cards.rcinc   # Card bitmap includes
│   │   └── app.ico       # Application icon
│   └── assets/           # Card bitmaps and graphics
├── tools/                # Utility programs
│   ├── extract-res/      # Extract bitmaps from legacy .RES files
│   ├── card-normalizer/  # Normalize card backgrounds
│   └── test-res-load/    # Test resource loading
└── docs/                 # Design documentation
```

## Architecture

The project is organized as a Cargo workspace with two main components:

### estwhi-core (Library)

Platform-independent game logic with minimal dependencies:

- **Card and Deck management** - Fisher-Yates shuffling
- **Scoring algorithms** - Vanilla and Squared modes
- **Trick resolution** - Lead suit following, trump handling
- **Legal play validation** - Suit-following rules
- **No platform dependencies** - Pure Rust with only `rand` crate

### estwhi (Executable)

Windows-specific UI layer:

- **Win32 GUI** - Classic Windows look and feel
- **GDI rendering** - Double-buffered card drawing
- **Registry persistence** - Game settings and high scores
- **DPI awareness** - Scales properly on high-DPI displays
- **Dialog management** - Options, scores, bidding, etc.

## Usage

### Starting a Game

1. Launch `estwhi.exe`
2. Use **Game → Options** to configure:
   - Number of players (2-6)
   - Maximum cards per hand (1-15)
   - Scoring mode (Vanilla/Squared)
   - Other preferences
3. Click **Deal** (or press F2) to start a new hand

### Gameplay

- **Bidding:** When prompted, select how many tricks you predict you'll win
- **Playing:** Click a card from your hand to play it
  - You must follow suit if you have cards in the lead suit
  - If you can't follow suit, you may play any card
- **Scoring:** After all tricks are played, scores are updated
  - Match your bid exactly to get bonus points
  - Vanilla mode: tricks + 10 bonus (if matched)
  - Squared mode: tricks² + 10 bonus (if matched)

### Keyboard Shortcuts

- **F2** - Deal new hand
- **F3** - View high scores

## Configuration

Game settings are persisted in the Windows Registry:

```
HKEY_CURRENT_USER\Software\Estwhi
```

Settings include:
- Number of players
- Maximum cards
- Scoring mode
- Window position
- Various gameplay options

## Development

### Code Style

This project follows standard Rust conventions:

```bash
# Format code
cargo fmt --all

# Run linter
cargo clippy --all -- -D warnings

# Check without building
cargo check --all
```

### Contributing

Contributions are welcome! Please:

1. Follow Rust conventions and idioms
2. Add tests for new functionality
3. Update documentation as needed
4. Run `cargo test` and `cargo clippy` before submitting

### Recent Improvements

See `wrk_docs/` for detailed code review and improvement plans.

## Legacy Compatibility

This Rust implementation maintains compatibility with the original Pascal version:

- **Card numbering** - Legacy 1-52 ID system preserved
- **Scoring rules** - Exact match of original algorithms
- **UI layout** - Classic mode matches pixel-perfect positions
- **Game logic** - Faithful reproduction of all rules

Original Pascal source files are included in the repository for reference.

## Troubleshooting

### Build Errors

**Problem:** `rc.exe not found`
**Solution:** Install Windows SDK or build with `--features no-res`

**Problem:** Linker errors
**Solution:** Ensure you have the MSVC toolchain: `rustup default stable-msvc`

### Runtime Errors

**Problem:** Application won't start
**Solution:** Ensure you're running on Windows 7 or newer

**Problem:** Cards don't display
**Solution:** Check that `assets/cards/71x96/` contains card bitmaps

### Performance Issues

**Problem:** Slow rendering
**Solution:** Enable double-buffering in Options (should be default)

## License

This project modernizes a classic card game. See original author credits in source files.

## Credits

- **Original Game:** Turbo Pascal Windows 3.1 version (1990s)
- **Rust Port:** Modern Win32 implementation
- **Card Graphics:** Classic bitmap assets

## Links

- **Repository:** https://github.com/0x4D44/estwhi
- **Documentation:** See `/docs` directory for design documents
- **Issues:** Report bugs via GitHub Issues

---

**Built with Rust 🦀 | Win32 API | Classic Gaming ♠️♥️♦️♣️**
