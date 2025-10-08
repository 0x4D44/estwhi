# Cheat Cards Window - High-Level Design (Revised)

## 1. Overview

The **Cheat Cards Window** is a separate, floating window that displays the cards held by all opponent players (players 2-6) during an active game. This provides a "cheat mode" functionality that allows the human player (Player 1) to see what cards the computer opponents are holding.

### 1.1 Purpose
- **Primary Function**: Display all opponent players' current hands in a compact, readable format
- **Use Case**: Debugging, learning card combinations, or casual play where the user wants to see all cards
- **User Control**: Can be toggled on/off via a checkbox in the Options dialog

### 1.2 Existing Infrastructure

**IMPORTANT**: Significant infrastructure already exists in the Rust codebase:

✅ **Checkbox**: ID 4009 in app.rc (line 67): `AUTOCHECKBOX "&Show opponent cards", 4009, 10, 170, 100, 12`

✅ **Config Field**: `cheat_cards: bool` in `UiConfig` struct (main.rs:158)

✅ **Registry Persistence**:
- Load: `registry::get_u32("CheatCards", 0)` (main.rs:364)
- Save: `registry::set_u32("CheatCards", if cfg.cheat_cards { 1 } else { 0 })` (main.rs:420)

✅ **Dialog Integration**: Options dialog already reads/writes the checkbox (main.rs:2919, 2990-3009)

**What's Missing**:
- Window class registration and creation
- Window drawing implementation
- Window position persistence (X, Y coordinates)
- Post-dialog toggle logic in main window
- Update triggers during gameplay
- Startup window creation if flag is true

## 2. Original Implementation Analysis

### 2.1 Window Characteristics (from estwhi_v11.pas)

**Window Type**: Separate popup window (`WS_POPUP | WS_CAPTION | WS_SYSMENU`)
- **Title**: "Cheat Information - Cards"
- **Size**: 400 x 200 pixels (fixed size, not resizable) - **Note**: Should be DPI-scaled in Rust
- **Background**: System button face color (light gray/beige)
- **Persistence**: Position saved to INI file on close, restored on next open

**State Persistence** (lines 695-705, 897-900):
```pascal
- CheatCards: BOOLEAN          // Whether window should exist
- CheatX, CheatY: INTEGER       // Window position
- Saved to INI: 'Cheat Cards' section
  - 'Window exists': 0 or 1
  - 'X position': screen coordinate
  - 'Y position': screen coordinate
```

### 2.2 Display Layout (DrawScreen method, lines 3190-3287)

**For each opponent player (players 2-N):**

1. **Player Number**: Drawn at X=10, vertically spaced
2. **Cards**: Drawn starting at X=30, scaled and spaced horizontally

**Vertical Layout**:
- YIncrement = 57 pixels per player row (default)
- If more than 2 opponents: YIncrement = (WindowHeight - 81) / (NoPlayers - 2)
- Capped at maximum 57 pixels
- Player A's Y position = 14 + YIncrement * (A - 2)

**Horizontal Card Layout**:
```pascal
(* Count cards ONCE - all players have same card count *)
NoCards := 0;
FOR A := 1 TO MaxCards DO
  IF (HandCards[2, A] > 0) THEN INC(NoCards);

(* Calculate spacing based on card count *)
IF (NoCards > 1) THEN
  ActWidth := (360 - SmallCardWidth) DIV (NoCards - 1)
  IF ActWidth > SmallMinWidth THEN
    ActWidth := min(ActWidth, SmallCardWidth + 10)
    // Cards overlap with ActWidth spacing
  ELSE
    // Spacing too tight, cards overlap heavily
ELSE
  // Single card: draw at X=30
```

**Key Insight**: Card count is computed ONCE (from any player, they all have the same count), then that spacing is used for ALL player rows.

**Card Dimensions**:
- Full card: 71 x 96 pixels (`CardWidth` x `CardHeight`)
- Small card: 41 x 55 pixels (`SmallCardWidth` x `SmallCardHeight`)
- Minimum spacing: 25 pixels (`SmallMinWidth`)
- Cards are StretchBlt from full-size bitmaps to small size

### 2.3 Lifecycle Management

**Creation** (lines 932-941, 1178-1187):
```pascal
(* On startup or Options change *)
IF (CheatCards) THEN
  CheatCardsPntr := New(PChetBox, Init(@self, 'Cheat Information - Cards'));
  IF (Application^.MakeWindow(CheatCardsPntr) = NIL) THEN
    MessageBox(HWindow, 'Could not create window object!', 'Error:', MB_ICONHAND);
    CheatCards := FALSE;
```

**Post-Dialog Toggle** (lines 1173-1195):
```pascal
(* IMPORTANT: Toggle happens in MAIN WINDOW after dialog closes *)
Temp2 := CheatCards;  (* Save old state *)
Application^.ExecDialog(New(POptions, Init(@self, 'Options')));
IF (Temp2 <> CheatCards) THEN
BEGIN
  IF (CheatCards) THEN
    (* Create window *)
    CheatCardsPntr := New(PChetBox, Init(@self, 'Cheat Information - Cards'));
  ELSE
    (* Close window *)
    CheatCardsPntr^.CloseWindow;
END;
```

**Destruction** (lines 1190-1193, 2842-2861):
```pascal
(* When user closes window via X button *)
FUNCTION TChetBox.CanClose: BOOLEAN;
BEGIN
  PMainWindow(Parent)^.CheatX := Attr.X;          (* Save position *)
  PMainWindow(Parent)^.CheatY := Attr.Y;
  PMainWindow(Parent)^.CheatCards := FALSE;       (* Update flag! *)
  CanClose := TRUE;
END;

(* On main window close *)
IF (CheatCards) THEN
  CheatCardsPntr^.CloseWindow;
```

**Updates** (lines 1435, 1529, 1564):
```
Cheat window is redrawn on:
1. After dealing new hand (GameDeal)
2. After computer players play their cards (ComputersPlays)
3. After trick evaluation (EvalTrick)
```

### 2.4 Options Dialog Integration (lines 3338, 3383, 3427-3430)

**Control**: Checkbox with ID 460 (`ID_CHEATCARD`) - **In Rust: ID 4009**
- Located in Options dialog (resource 3002)
- Label: "&Show opponent cards"

**Behavior**:
```pascal
SetupWindow:
  IF (PMainWindow(Parent)^.CheatCards) THEN Chk1^.SetCheck(BF_CHECKED);

OK Handler:
  IF (Chk1^.GetCheck = BF_CHECKED) THEN
    PMainWindow(Parent)^.CheatCards := TRUE
  ELSE
    PMainWindow(Parent)^.CheatCards := FALSE;
```

**Note**: Dialog only reads/writes the flag. Toggle logic happens in main window post-dialog.

## 3. Rust Implementation Design

### 3.1 Data Structures

**Add to AppState** (main.rs):
```rust
struct CheatWindowState {
    hwnd: Option<HWND>,  // Handle to cheat window (if open)
    x: i32,              // Last known X position
    y: i32,              // Last known Y position
}

// In main AppState
struct AppState {
    config: UiConfig,     // Contains cheat_cards: bool (already exists)
    game: GameState,
    // ... other existing fields ...
    cheat_window: CheatWindowState,  // NEW
}
```

**Note**: `config.cheat_cards` already exists. We only add position tracking.

### 3.1.1 Constants and Imports

**Constants** (add to main.rs):
```rust
// Cheat window constants
const CHEAT_WINDOW_CLASS: &str = "EstwhiCheatCards";
const CHEAT_WINDOW_WIDTH_BASE: f32 = 400.0;   // Base 96 DPI
const CHEAT_WINDOW_HEIGHT_BASE: f32 = 200.0;
const SMALL_CARD_WIDTH_BASE: f32 = 41.0;
const SMALL_CARD_HEIGHT_BASE: f32 = 55.0;
const SMALL_MIN_WIDTH_BASE: f32 = 25.0;

// Already defined (verify):
// const CARD_W: i32 = 71;  // main.rs:86
// const CARD_H: i32 = 96;  // main.rs:88
```

**Imports Needed** (verify these are in scope):
```rust
// Most should already be imported, but verify:
use windows::Win32::UI::WindowsAndMessaging::GetSysColor;  // For COLOR_BTNFACE
use windows::Win32::Graphics::Gdi::{StretchBlt, SRCCOPY};  // For card scaling
```

### 3.2 Registry Persistence

**IMPORTANT**: Match existing registry key naming convention and function patterns.

**Existing**:
- `"CheatCards"` - boolean flag (already persisted in `save_config_to_registry()`)

**Add**:
- `"CheatWindowX"` - X position
- `"CheatWindowY"` - Y position

**Pattern**: Follow the RandomThingsConfig pattern - separate load/save functions for cheat window state.

```rust
// NEW: Cheat window state persistence functions
fn load_cheat_window_state() -> CheatWindowState {
    CheatWindowState {
        hwnd: None,  // Never persisted, always starts as None
        x: registry::get_u32("CheatWindowX", 100) as i32,
        y: registry::get_u32("CheatWindowY", 100) as i32,
    }
}

fn save_cheat_window_state(state: &CheatWindowState) {
    let _ = registry::set_u32("CheatWindowX", state.x as u32);
    let _ = registry::set_u32("CheatWindowY", state.y as u32);
}

// MODIFIED: Update app_state() initialization to load cheat window state
fn app_state() -> &'static Mutex<AppState> {
    APP_STATE.get_or_init(|| {
        let mut app = AppState {
            config: load_config_from_registry(),
            game: GameState::default(),
            random_things: RandomThings::default(),
            cheat_window: load_cheat_window_state(),  // NEW: Load position
        };
        app.random_things.config = load_random_things_config();
        app.random_things.validate_and_fix_config();
        Mutex::new(app)
    })
}

// Note: save_config_to_registry() already saves the CheatCards flag.
// Window position is saved separately via save_cheat_window_state().
```

### 3.3 Post-Dialog Toggle Logic

**CRITICAL**: This logic goes in the MAIN WINDOW after Options dialog closes, NOT in the dialog handler.

**Implementation**: Modify the existing Options menu command handler (ID 102) in window_proc.

**Current Code** (main.rs:1142-1144):
```rust
102 => {
    show_options_dialog(hwnd);  // Already implemented
    return LRESULT(0);
}
```

**Updated Code**:
```rust
102 => {
    // Options - with cheat window toggle logic
    let old_cheat_flag = {
        let app = app_state().lock().unwrap();
        app.config.cheat_cards
    };

    // Show Options dialog (already implemented helper function)
    show_options_dialog(hwnd);

    // Check if cheat cards flag changed
    let new_cheat_flag = {
        let app = app_state().lock().unwrap();
        app.config.cheat_cards
    };

    // Toggle window if flag changed
    if old_cheat_flag != new_cheat_flag {
        unsafe {
            if new_cheat_flag {
                // Create cheat window
                if let Err(_) = create_cheat_cards_window(hwnd) {
                    MessageBoxW(
                        hwnd,
                        PCWSTR(wide("Could not create cheat cards window!").as_ptr()),
                        PCWSTR(wide("Error").as_ptr()),
                        MB_ICONHAND | MB_OK,
                    );
                    app_state().lock().unwrap().config.cheat_cards = false;
                }
            } else {
                // Close cheat window
                close_cheat_cards_window();
            }
        }
    }

    return LRESULT(0);
}
```

**Note**: Uses existing `show_options_dialog(hwnd)` helper (main.rs:2584) instead of inline DialogBoxParamW call.

### 3.4 Window Creation with DPI Scaling

**Register Window Class**:
```rust
const CHEAT_WINDOW_CLASS: &str = "EstwhiCheatCards";

unsafe fn register_cheat_window_class(hinstance: HINSTANCE) -> windows::core::Result<()> {
    let class_name = wide(CHEAT_WINDOW_CLASS);

    let wc = WNDCLASSW {
        style: CS_HREDRAW | CS_VREDRAW,
        lpfnWndProc: Some(cheat_window_proc),
        cbClsExtra: 0,
        cbWndExtra: 0,
        hInstance: hinstance,
        hIcon: HICON::default(),
        hCursor: LoadCursorW(None, IDC_ARROW).unwrap(),
        hbrBackground: HBRUSH((COLOR_BTNFACE.0 + 1) as isize as *mut _),
        lpszMenuName: PCWSTR::null(),
        lpszClassName: PCWSTR(class_name.as_ptr()),
    };

    // Register and check for errors (matches main window pattern at main.rs:917)
    let atom = RegisterClassW(&wc);
    if atom == 0 {
        return Err(windows::core::Error::from_win32());
    }

    Ok(())
}
```

**Note**: Must be called during wWinMain initialization, before any window creation attempts.

**Create Window with DPI Scaling**:
```rust
// Constants (base 96 DPI)
const CHEAT_WINDOW_WIDTH_BASE: f32 = 400.0;
const CHEAT_WINDOW_HEIGHT_BASE: f32 = 200.0;

unsafe fn create_cheat_cards_window(parent_hwnd: HWND) -> windows::core::Result<HWND> {
    let (x, y) = {
        let app = app_state().lock().unwrap();
        (app.cheat_window.x, app.cheat_window.y)
    };

    // Get DPI for scaling
    let dpi = GetDpiForWindow(parent_hwnd) as f32;
    let scale = dpi / 96.0;

    let width = (CHEAT_WINDOW_WIDTH_BASE * scale).round() as i32;
    let height = (CHEAT_WINDOW_HEIGHT_BASE * scale).round() as i32;

    let hwnd = CreateWindowExW(
        WINDOW_EX_STYLE::default(),
        PCWSTR(wide(CHEAT_WINDOW_CLASS).as_ptr()),
        PCWSTR(wide("Cheat Information - Cards").as_ptr()),
        WS_POPUP | WS_CAPTION | WS_SYSMENU | WS_VISIBLE,
        x, y, width, height,
        parent_hwnd,
        None,
        GetModuleHandleW(None)?,
        None,
    )?;

    app_state().lock().unwrap().cheat_window.hwnd = Some(hwnd);
    Ok(hwnd)
}
```

**Close Window**:
```rust
unsafe fn close_cheat_cards_window() {
    let hwnd_opt = {
        let app = app_state().lock().unwrap();
        app.cheat_window.hwnd
    };

    if let Some(hwnd) = hwnd_opt {
        // Window will handle position saving in WM_CLOSE
        let _ = DestroyWindow(hwnd);
    }
}
```

### 3.5 Window Procedure

**CRITICAL**: Cleanup must happen in WM_DESTROY, not WM_CLOSE, because:
- User clicking X: DefWindowProc(WM_CLOSE) → DestroyWindow → WM_DESTROY
- Toggle off via Options: Directly calls DestroyWindow → WM_DESTROY (skips WM_CLOSE!)

```rust
unsafe extern "system" fn cheat_window_proc(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    match msg {
        WM_PAINT => {
            draw_cheat_cards(hwnd);
            return LRESULT(0);
        }

        WM_MOVE => {
            // Save window position as it moves
            let mut rect = RECT::default();
            if GetWindowRect(hwnd, &mut rect).is_ok() {
                let mut app = app_state().lock().unwrap();
                app.cheat_window.x = rect.left;
                app.cheat_window.y = rect.top;
            }
            // Fall through to DefWindowProc
        }

        WM_CLOSE => {
            // User clicked X button - just destroy the window
            // Cleanup happens in WM_DESTROY
            let _ = DestroyWindow(hwnd);
            return LRESULT(0);
        }

        WM_DESTROY => {
            // CRITICAL: All cleanup happens here (called in both close paths)
            let mut app = app_state().lock().unwrap();

            // Save final position
            let mut rect = RECT::default();
            if GetWindowRect(hwnd, &mut rect).is_ok() {
                app.cheat_window.x = rect.left;
                app.cheat_window.y = rect.top;
            }

            // Update state: window is now closed
            app.config.cheat_cards = false;  // Options checkbox reflects closure
            app.cheat_window.hwnd = None;

            return LRESULT(0);
        }

        _ => {}
    }

    DefWindowProcW(hwnd, msg, wparam, lparam)
}
```

### 3.6 Drawing Logic with Correct Indexing

**Constants** (base 96 DPI):
```rust
const SMALL_CARD_WIDTH_BASE: f32 = 41.0;
const SMALL_CARD_HEIGHT_BASE: f32 = 55.0;
const SMALL_MIN_WIDTH_BASE: f32 = 25.0;
```

**Drawing Function**:
```rust
unsafe fn draw_cheat_cards(hwnd: HWND) {
    let mut ps = PAINTSTRUCT::default();
    let hdc = BeginPaint(hwnd, &mut ps);

    // Clone data from AppState to avoid holding lock during paint
    let (hands, num_players, dpi) = {
        let app = app_state().lock().unwrap();
        (
            app.game.hands.clone(),
            app.game.hands.len(),
            GetDpiForWindow(hwnd) as f32,
        )
    };
    // Lock released here

    let scale = dpi / 96.0;

    // Get window rect
    let mut rect = RECT::default();
    GetClientRect(hwnd, &mut rect);

    // Fill background with button face color
    let gray_brush = CreateSolidBrush(COLORREF(GetSysColor(COLOR_BTNFACE)));
    FillRect(hdc, &rect, gray_brush);
    DeleteObject(gray_brush);

    // Only draw if game has players
    if num_players < 2 {
        EndPaint(hwnd, &ps);
        return;
    }

    // Calculate vertical spacing for player rows
    let mut y_increment = (57.0 * scale).round() as i32;
    if num_players > 2 {
        let available_height = rect.bottom - (81.0 * scale).round() as i32;
        let calculated_increment = available_height / (num_players - 2) as i32;
        if calculated_increment < y_increment {
            y_increment = calculated_increment;
        }
    }

    // Count cards ONCE (all players have same count)
    // Use player 2 (index 1 in 0-based Vec)
    let no_cards = if hands.len() > 1 {
        hands[1].iter().filter(|&&c| c > 0).count()
    } else {
        0
    };

    if no_cards == 0 {
        EndPaint(hwnd, &ps);
        return;
    }

    // Calculate horizontal card spacing (same for all players)
    let small_card_width = (SMALL_CARD_WIDTH_BASE * scale).round() as i32;
    let small_card_height = (SMALL_CARD_HEIGHT_BASE * scale).round() as i32;
    let small_min_width = (SMALL_MIN_WIDTH_BASE * scale).round() as i32;

    let act_width = if no_cards > 1 {
        let available_width = (360.0 * scale).round() as i32 - small_card_width;
        let calculated = available_width / (no_cards - 1) as i32;

        if calculated > small_min_width {
            calculated.min(small_card_width + (10.0 * scale).round() as i32)
        } else {
            small_min_width  // Fallback to minimum
        }
    } else {
        small_card_width  // Single card, no spacing needed
    };

    // Set up text drawing
    SetBkMode(hdc, TRANSPARENT);
    SetTextColor(hdc, COLORREF(GetSysColor(COLOR_WINDOWTEXT)));

    // Draw each opponent player (skip player 1 = index 0)
    for player_idx in 1..num_players {
        let player_number = player_idx + 1;  // Display number (2, 3, 4, ...)
        let row_index = player_idx - 1;      // Row offset (0, 1, 2, ...)

        let text_y = (14.0 * scale).round() as i32 + y_increment * row_index as i32;
        let card_y = (4.0 * scale).round() as i32 + y_increment * row_index as i32;

        // Draw player number
        let player_text = player_number.to_string();
        let player_wide = wide(&player_text);
        TextOutW(
            hdc,
            (10.0 * scale).round() as i32,
            text_y,
            &player_wide[..player_wide.len() - 1],
        );

        // Draw cards for this player
        let mut card_index = 0;
        for &card_id in &hands[player_idx] {
            if card_id > 0 {
                let card_x = (30.0 * scale).round() as i32 + card_index * act_width;

                // Draw card (scaled from full 71x96 to small size)
                draw_card_scaled(
                    hdc,
                    card_x,
                    card_y,
                    card_id,
                    small_card_width,
                    small_card_height,
                );

                card_index += 1;
            }
        }
    }

    EndPaint(hwnd, &ps);
}
```

**Card Drawing Helper**:
```rust
unsafe fn draw_card_scaled(
    hdc: HDC,
    x: i32,
    y: i32,
    card_id: u32,
    dest_width: i32,
    dest_height: i32,
) {
    // Load card bitmap from cache (use existing get_card_bitmap)
    let hbmp = match get_card_bitmap(card_id) {
        Some(bmp) => bmp,
        None => return,  // Card not found, skip drawing
    };

    let memdc = CreateCompatibleDC(hdc);
    let old_bmp = SelectObject(memdc, hbmp);

    // StretchBlt from full card size (71x96) to small size
    let _ = StretchBlt(
        hdc,
        x,
        y,
        dest_width,
        dest_height,
        memdc,
        0,
        0,
        CARD_W,  // 71
        CARD_H,  // 96
        SRCCOPY,
    );

    SelectObject(memdc, old_bmp);
    DeleteDC(memdc);
}
```

### 3.7 Update Triggers

Add calls to update the cheat window at these game events:

```rust
unsafe fn update_cheat_cards_window() {
    let hwnd_opt = {
        let app = app_state().lock().unwrap();
        app.cheat_window.hwnd
    };

    if let Some(hwnd) = hwnd_opt {
        InvalidateRect(hwnd, None, BOOL(1));
    }
}
```

**Call after**:
1. **deal_new_hand()** - New cards dealt to all players
2. **after computer plays** - In the computer turn logic after cards are played
3. **after trick evaluation** - After trick winner is determined and cards removed

### 3.8 Startup Window Creation

Add to `wWinMain` after main window created:

```rust
// After ShowWindow(hwnd, show_cmd);

// Create cheat window if flag is set
unsafe {
    let should_create = {
        let app = app_state().lock().unwrap();
        app.config.cheat_cards
    };

    if should_create {
        if let Err(_) = create_cheat_cards_window(hwnd) {
            MessageBoxW(
                hwnd,
                PCWSTR(wide("Could not create cheat cards window!").as_ptr()),
                PCWSTR(wide("Error").as_ptr()),
                MB_ICONHAND | MB_OK,
            );
            app_state().lock().unwrap().config.cheat_cards = false;
        }
    }
}
```

### 3.9 Cleanup

**On Main Window WM_DESTROY**:
```rust
unsafe fn cleanup_cheat_window() {
    // CRITICAL: Must release mutex before calling DestroyWindow to avoid deadlock
    // DestroyWindow sends WM_DESTROY, which also acquires the mutex
    let hwnd_opt = {
        let app = app_state().lock().unwrap();
        app.cheat_window.hwnd
    };  // Mutex released here

    if let Some(hwnd) = hwnd_opt {
        // Save position before destroying window
        let mut rect = RECT::default();
        if GetWindowRect(hwnd, &mut rect).is_ok() {
            let mut app = app_state().lock().unwrap();
            app.cheat_window.x = rect.left;
            app.cheat_window.y = rect.top;
        }  // Mutex released here

        // Destroy window (will trigger WM_DESTROY handler)
        // WM_DESTROY can now acquire the mutex without deadlock
        DestroyWindow(hwnd);
        // Note: WM_DESTROY will clear hwnd and update cheat_cards flag
    }

    // Save both config and cheat window state separately
    // (Following the pattern of save_config_to_registry + save_random_things_config)
    {
        let app = app_state().lock().unwrap();
        save_config_to_registry(&app.config);
        save_cheat_window_state(&app.cheat_window);
    }  // Mutex released
}
```

**Note**: This is called from main window's WM_DESTROY handler to ensure state is saved on app exit. The function carefully manages mutex locks to avoid deadlock when DestroyWindow triggers the cheat window's WM_DESTROY handler.

## 4. Implementation Checklist

### Phase 1: Window Position Persistence
- [ ] Add `CheatWindowState` struct to AppState
- [ ] Add load/save for "CheatWindowX" and "CheatWindowY" registry keys
- [ ] Initialize cheat_window state on startup
- [ ] Save cheat_window state on shutdown

### Phase 2: Window Infrastructure
- [ ] Register cheat window class in wWinMain (with error checking)
- [ ] Implement create_cheat_cards_window() with DPI scaling
- [ ] Implement close_cheat_cards_window()
- [ ] Implement cheat_window_proc()
  - [ ] WM_PAINT handler (calls draw_cheat_cards)
  - [ ] WM_MOVE handler (saves position as window moves)
  - [ ] WM_CLOSE handler (calls DestroyWindow only)
  - [ ] WM_DESTROY handler (ALL cleanup: saves position, updates config.cheat_cards, clears hwnd)

### Phase 3: Post-Dialog Toggle Logic
- [ ] Add toggle logic to Options menu command handler (ID 102)
- [ ] Save old cheat_cards flag before dialog
- [ ] Compare flags after dialog
- [ ] Create or close window based on change
- [ ] Handle creation errors

### Phase 4: Drawing Implementation
- [ ] Implement draw_cheat_cards() with:
  - [ ] Clone data before painting (don't hold lock)
  - [ ] DPI scaling for all dimensions
  - [ ] Correct 0-based indexing (skip player 0, draw 1..N)
  - [ ] Count cards ONCE, use for all rows
  - [ ] Calculate vertical spacing
  - [ ] Calculate horizontal card spacing
  - [ ] Draw player numbers
  - [ ] Draw cards for each player
- [ ] Implement draw_card_scaled() helper

### Phase 5: Update Triggers
- [ ] Add update_cheat_cards_window() calls after:
  - [ ] deal_new_hand()
  - [ ] computer plays cards
  - [ ] trick evaluation

### Phase 6: Startup & Cleanup
- [ ] Add startup creation logic in wWinMain
- [ ] Add cleanup_cheat_window() in main WM_DESTROY
- [ ] Test window position persistence across restarts

### Phase 7: Testing
- [ ] Test toggle on/off via Options
- [ ] Test window position persistence
- [ ] Test with 2, 3, 4, 5, 6 players
- [ ] Test with varying max_cards (1-15)
- [ ] Test window closure via X button updates checkbox
- [ ] Test DPI scaling on high-DPI display
- [ ] Test multi-monitor positioning
- [ ] Test app restart with window open flag

## 5. Edge Cases & Error Handling

1. **Window Creation Failure**:
   ```rust
   if let Err(_) = create_cheat_cards_window(hwnd) {
       MessageBoxW(hwnd, w!("Could not create cheat cards window!"),
                   w!("Error"), MB_ICONHAND | MB_OK);
       app.config.cheat_cards = false;
   }
   ```

2. **No Game in Progress**: Window shows empty rows (player numbers, no cards)

3. **Multi-Monitor**: Window position may be offscreen; Windows will auto-adjust

4. **DPI Scaling**: All dimensions scaled consistently using `GetDpiForWindow()`

5. **Mutex Deadlock**: Data cloned before painting to avoid holding lock

6. **User Closes Window**: WM_DESTROY updates `config.cheat_cards = false` so Options checkbox reflects state (cleanup must be in WM_DESTROY, not WM_CLOSE, to handle both close paths)

## 6. Index Translation Reference

**Pascal (1-based)**:
- HandCards[1, ...] = Player 1 (human)
- HandCards[2, ...] = Player 2 (first opponent)
- Loop: FOR A := 2 TO NoPlayers

**Rust (0-based)**:
- hands[0] = Player 1 (human) - SKIP in cheat window
- hands[1] = Player 2 (first opponent to display)
- Loop: for player_idx in 1..num_players

**Display Calculations**:
- player_number = player_idx + 1 (for text display: "2", "3", etc.)
- row_index = player_idx - 1 (for Y offset: 0, 1, 2, ...)

## 7. Testing Strategy

1. **Basic Toggle**: Open/close via Options checkbox multiple times
2. **Position Persistence**: Move window, close app, reopen - verify position restored
3. **User Close**: Close via X button, open Options - verify checkbox is unchecked
4. **Multi-Game**: Play multiple games, verify updates throughout
5. **Player Counts**: Test with 2, 3, 4, 5, 6 players - verify layout
6. **Card Counts**: Test with max_cards 1-15 - verify spacing adapts
7. **DPI Scaling**: Test on 96, 120, 144, 192 DPI displays
8. **Startup**: Close app with window open, reopen - verify window recreated
9. **Errors**: Test creation failure scenarios (low memory, etc.)
10. **Indexing**: Verify player 1's cards NEVER shown, players 2+ shown correctly
