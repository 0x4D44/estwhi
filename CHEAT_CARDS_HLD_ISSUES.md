# Issues Found in Cheat Cards HLD

## Critical Issues

### 1. **Infrastructure Already Exists**
**Status**: MAJOR OVERSIGHT

The HLD suggests creating all infrastructure from scratch, but significant parts already exist:

- ✅ **Checkbox exists** (ID 4009 in app.rc line 67): `AUTOCHECKBOX "&Show opponent cards", 4009, 10, 170, 100, 12`
- ✅ **Config field exists** (main.rs:158): `cheat_cards: bool` in `UiConfig` struct
- ✅ **Registry persistence exists** (main.rs:364, 420):
  - Loaded: `registry::get_u32("CheatCards", 0)`
  - Saved: `registry::set_u32("CheatCards", if cfg.cheat_cards { 1 } else { 0 })`
- ✅ **Dialog wiring exists** (main.rs:2919, 2990-3009): Checkbox is read/written in Options dialog

**What's Actually Missing**:
- Window creation/destruction logic
- Window drawing implementation
- Update triggers during gameplay
- Window position persistence (currently only the flag is persisted)

**Impact**: Phase 1 (Data & Persistence) and Phase 2 (Options Dialog) are mostly complete. Implementation can jump to Phase 3.

---

### 2. **Window Position Not Persisted**
**Status**: DESIGN GAP

The existing code persists `CheatCards` flag but NOT window position (CheatX, CheatY).

**Current State**:
```rust
// Only this exists:
registry::get_u32("CheatCards", 0)
```

**What's Needed**:
```rust
registry::get_u32("CheatWindowX", 100)  // or "CheatCardsX"
registry::get_u32("CheatWindowY", 100)  // or "CheatCardsY"
```

**Fix**: Add position persistence to registry save/load functions.

---

### 3. **Incorrect Card Counting in Drawing Logic**
**Status**: ALGORITHMIC ERROR

HLD Section 3.6 shows counting cards per-player inside the player loop:
```rust
// WRONG: Counting for each player individually
for player in 2..=no_players {
    for card in &hands[player_idx] {
        if *card > 0 {
            no_cards += 1;  // This recounts for each player
        }
    }
}
```

**Original Pascal Logic** (lines 3222-3227):
```pascal
(* Recalculate NoCards to make sure it is OK *)
NoCards := 0;
FOR A := 1 TO MaxCards DO
BEGIN
  IF (HandCards[2, A] > 0) THEN
    INC(NoCards);
END;
```

**Correct Approach**: Count cards ONCE (from any player, they all have the same count), then use that spacing for ALL player rows. In Estimation Whist, all players always have the same number of cards per hand.

```rust
// Count cards once (from player 2 = hands[1])
let no_cards = hands[1].iter().filter(|&&c| c > 0).count();

// Use this count for spacing calculations for ALL players
let act_width = if no_cards > 1 {
    (360 - SMALL_CARD_WIDTH) / (no_cards - 1)
} else {
    SMALL_CARD_WIDTH
};

// Then iterate all players with same spacing
for player_idx in 1..no_players {
    // Draw cards with consistent spacing
}
```

**Impact**: Without this fix, spacing will be incorrect or calculated redundantly.

---

### 4. **Indexing Confusion: Pascal 1-based vs Rust 0-based**
**Status**: CLARITY ISSUE

HLD is unclear about index translation between Pascal (1-based) and Rust (0-based).

**Pascal**: `HandCards[A, B]` where A = 1 to NoPlayers
- Player 1 = HandCards[1, ...]
- Player 2 = HandCards[2, ...]

**Rust**: `hands: Vec<Vec<u32>>` (main.rs:212)
- Player 1 = hands[0]
- Player 2 = hands[1]

**Drawing Loop Should Be**:
```rust
// Iterate from player 2 onwards (skip player 1 = human)
for player_idx in 1..game.hands.len() {
    let player_number = player_idx + 1; // For display
    let y_offset = 14 + y_increment * (player_idx - 1);

    // Draw player number
    draw_text(hdc, 10, y_offset, &player_number.to_string());

    // Draw cards from hands[player_idx]
    for &card_id in &game.hands[player_idx] {
        if card_id > 0 {
            // Draw card
        }
    }
}
```

**Fix**: Clarify indexing in all pseudocode examples.

---

### 5. **Missing Post-Dialog Toggle Logic**
**Status**: DESIGN GAP

HLD shows toggle logic inside Options dialog, but Pascal code does it AFTER dialog closes.

**Original Pattern** (lines 1173-1195):
```pascal
Temp2 := CheatCards;
Application^.ExecDialog(New(POptions, Init(@self, 'Options')));
IF (Temp2 <> CheatCards) THEN
BEGIN
  IF (CheatCards) THEN
    (* Open a new window *)
    CheatCardsPntr := New(PChetBox, Init(@self, 'Cheat Information - Cards'));
  ELSE
    (* Close the window *)
    CheatCardsPntr^.CloseWindow;
END;
```

**Correct Flow**:
1. Main window saves old `cheat_cards` value
2. Options dialog executes (reads/writes config)
3. Main window compares old vs new value
4. If changed, create or destroy window

**HLD Says**: "Implement toggle logic to create/destroy window" in Phase 2 (Options Dialog)
**Should Say**: "Implement toggle logic in main window AFTER Options dialog closes"

**Fix**: Add this logic to the command handler for Options menu item, not in the dialog proc.

---

### 6. **Window Closure by User Not Handled**
**Status**: INCOMPLETE DESIGN

When user closes cheat window directly (clicks X), Pascal code updates main flag:

```pascal
FUNCTION TChetBox.CanClose: BOOLEAN;
BEGIN
  PMainWindow(Parent)^.CheatX := Attr.X;
  PMainWindow(Parent)^.CheatY := Attr.Y;
  PMainWindow(Parent)^.CheatCards := FALSE;
```

**HLD Section 3.5** only mentions:
```rust
WM_CLOSE:
    - Save window position
    - Set window_hwnd to None
    - Set window_open to FALSE
    - DestroyWindow()
```

**Missing**: Need to update `app.config.cheat_cards = false` so the Options checkbox reflects the closure.

**Fix**:
```rust
WM_CLOSE => {
    let mut app = app_state().lock().unwrap();

    // Save position
    let mut rect = RECT::default();
    if GetWindowRect(hwnd, &mut rect).is_ok() {
        app.cheat_window_x = rect.left;
        app.cheat_window_y = rect.top;
    }

    // Update flag so Options dialog shows correct state
    app.config.cheat_cards = false;
    app.cheat_window_hwnd = None;

    DestroyWindow(hwnd);
    return 0;
}
```

---

### 7. **DPI Scaling Not Designed In**
**Status**: INCOMPLETE

HLD mentions DPI scaling in "Edge Cases" but doesn't integrate it into the design.

**Issue**: Window is fixed at 400x200 pixels, but should scale with DPI like the main window does.

**Pascal Code**: Fixed pixel sizes (no DPI awareness in Windows 3.1 era)
**Modern Rust Code**: Already DPI-aware (see main.rs:996: `GetDpiForWindow`)

**Fix**: Scale window size, card dimensions, and positions:
```rust
let dpi = GetDpiForWindow(parent_hwnd);
let scale = dpi as f32 / 96.0;

let window_width = (400.0 * scale).round() as i32;
let window_height = (200.0 * scale).round() as i32;

const SMALL_CARD_WIDTH_BASE: f32 = 41.0;
const SMALL_CARD_HEIGHT_BASE: f32 = 55.0;

let small_card_width = (SMALL_CARD_WIDTH_BASE * scale).round() as i32;
let small_card_height = (SMALL_CARD_HEIGHT_BASE * scale).round() as i32;
```

---

### 8. **Background Color Syntax Incorrect**
**Status**: TECHNICAL ERROR

HLD Section 3.4 shows:
```rust
hbrBackground: HBRUSH((COLOR_BTNFACE.0 + 1) as isize),
```

**Problem**: Incomplete cast, doesn't create proper pointer.

**Correct Syntax** (windows-rs):
```rust
hbrBackground: HBRUSH((COLOR_BTNFACE.0 + 1) as isize as *mut _),
```

Or reference existing code pattern from main window creation.

---

### 9. **Mutex Lock Held During Entire Paint**
**Status**: PERFORMANCE CONCERN

HLD Section 3.6 shows:
```rust
let app = app_state().lock().unwrap();
let game = &app.game;
// ... lots of drawing code ...
EndPaint(hwnd, &ps);
// Lock held until here
```

**Problem**: Holds mutex for entire paint operation, blocking other threads.

**Better Pattern**:
```rust
let (hands, no_players, max_cards) = {
    let app = app_state().lock().unwrap();
    (
        app.game.hands.clone(),
        app.game.hands.len(),
        app.config.max_cards,
    )
}; // Lock released here

// Now paint with cloned data
```

**Trade-off**: Cloning has cost, but paint may take longer than clone. Depends on data size.

---

### 10. **Missing Startup Window Creation**
**Status**: DESIGN GAP

HLD mentions in Section 2.4 that window should be created on startup if flag is TRUE, but doesn't specify WHERE in initialization sequence.

**Pascal Code** (lines 931-941):
```pascal
(* If cheat box is meant to exist then so it should *)
IF (CheatCards) THEN
BEGIN
  CheatCardsPntr := New(PChetBox, Init(@self, 'Cheat Information - Cards'));
```

**This happens in main window initialization, AFTER main window is created.**

**Fix**: Add to implementation checklist:
```rust
// In wWinMain, after main window created but before message loop:
unsafe {
    let should_create_cheat = {
        let app = app_state().lock().unwrap();
        app.config.cheat_cards
    };

    if should_create_cheat {
        if let Err(e) = create_cheat_cards_window(main_hwnd) {
            MessageBoxW(
                main_hwnd,
                w!("Could not create cheat cards window"),
                w!("Error"),
                MB_ICONHAND | MB_OK,
            );
            app_state().lock().unwrap().config.cheat_cards = false;
        }
    }
}
```

---

## Minor Issues

### 11. **Registry Key Naming Inconsistency**
**Status**: STYLE ISSUE

HLD suggests multiple naming schemes:
- "CheatCardsOpen" (inconsistent with existing "CheatCards")
- "CheatCardsX/Y" (consistent)
- "CheatWindowExists" (verbose)

**Existing Code**: Uses "CheatCards" for the flag.

**Recommendation**: Match existing style:
- "CheatCards" - flag (already exists)
- "CheatWindowX" - X position
- "CheatWindowY" - Y position

---

### 12. **Card Drawing Helper Not Specified**
**Status**: INCOMPLETE

HLD references `draw_card_scaled()` but doesn't define it.

**What's Needed**: Check existing card drawing code in main window and reuse pattern.

**Likely Implementation**:
```rust
unsafe fn draw_card_scaled(hdc: HDC, x: i32, y: i32, card_id: u32,
                           dest_w: i32, dest_h: i32) {
    let hbmp = load_card_bitmap(card_id);
    let memdc = CreateCompatibleDC(hdc);
    let old = SelectObject(memdc, hbmp);

    StretchBlt(
        hdc, x, y, dest_w, dest_h,
        memdc, 0, 0, CARD_W, CARD_H,
        SRCCOPY
    );

    SelectObject(memdc, old);
    DeleteDC(memdc);
}
```

---

### 13. **No Handling for "No Game in Progress"**
**Status**: EDGE CASE

What should cheat window show when `game.in_progress == false`?

**Options**:
1. Show player numbers with no cards (current Pascal behavior)
2. Show "No game in progress" message
3. Disable/gray out the window

**Recommendation**: Keep simple - show empty rows like Pascal does.

---

### 14. **Error Handling Not Specified**
**Status**: INCOMPLETE

HLD mentions error handling but doesn't specify message box text or recovery.

**Should Match Pascal** (lines 1184-1186):
```pascal
MessageBox(HWindow, 'Could not create window object!',
           'Error:', MB_ICONHAND);
CheatCards := FALSE;
```

**Rust Equivalent**:
```rust
MessageBoxW(
    parent_hwnd,
    w!("Could not create cheat cards window!"),
    w!("Error"),
    MB_ICONHAND | MB_OK,
);
app.config.cheat_cards = false;
```

---

### 15. **Window Class Name Not Consistent**
**Status**: STYLE ISSUE

HLD suggests: `"EstwhiCheatCards"`
Pascal uses: `"Cheat card window"` (line 3312)

**Recommendation**: Use `"EstwhiCheatCards"` (more specific, avoids spaces).

---

## Summary of Required Fixes

| Issue | Severity | Fix Required |
|-------|----------|--------------|
| Infrastructure already exists | Critical | Remove redundant phases, start at Phase 3 |
| Window position not persisted | Critical | Add X/Y registry persistence |
| Incorrect card counting | Critical | Count once, use for all players |
| Indexing confusion | High | Clarify 0-based indexing throughout |
| Post-dialog toggle missing | High | Add toggle logic to main window |
| Window closure flag not updated | High | Update config.cheat_cards on WM_CLOSE |
| DPI scaling not designed | Medium | Scale all dimensions and positions |
| Background color syntax | Medium | Fix HBRUSH cast syntax |
| Mutex held during paint | Medium | Clone data before paint |
| Startup creation missing | Medium | Add to initialization sequence |
| Registry key naming | Low | Use consistent naming scheme |
| Card drawing helper | Low | Define or reference existing function |
| No game edge case | Low | Document behavior (show empty) |
| Error handling | Low | Specify message box text |
| Window class name | Low | Use consistent naming |

## Revised Implementation Order

1. **Phase 1**: Window position registry persistence
2. **Phase 2**: Window class registration and creation logic
3. **Phase 3**: Drawing implementation (with correct indexing and counting)
4. **Phase 4**: Post-dialog toggle logic in main window
5. **Phase 5**: WM_CLOSE handler with flag update
6. **Phase 6**: Startup creation logic
7. **Phase 7**: DPI scaling integration
8. **Phase 8**: Update triggers after game events
9. **Phase 9**: Testing and polish
