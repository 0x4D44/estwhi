# Random Things - Reimplementation Design (REVISED)

## Overview
Random Things is a whimsical feature that displays small suit icons (clubs, diamonds, hearts, spades) that randomly "walk" around the green playing table area when no game is in progress. Additionally, an "Icon Twirl" feature animates the application icon when the window is minimized.

## Original Implementation Analysis

### Data Structures (Pascal)
```pascal
XPos, YPos: ARRAY [1..10] OF INTEGER;  (* Timer points locations *)
Multiplier: INTEGER;                    (* Random walk multiplier: 1-20 *)
NoThings: INTEGER;                      (* Number of random things: 1-6 *)
TimeRnd: WORD;                          (* Time interval (ms): 20-1000 *)
ExistRnd: BOOLEAN;                      (* Random things enabled *)
ExistIcn: BOOLEAN;                      (* Icon twirl enabled *)
IconCount: INTEGER;                     (* Current icon in animation: 1-3 *)
TimeIco: WORD;                          (* Icon animation interval *)
```

### Dialog Configuration
- **Dialog Size**: 192×88 dialog units
- **Position**: (25, 20)
- **Controls**:
  1. **Multiplier Scrollbar** (ID 660): Range 1-20, controls movement speed
  2. **Multiplier Static Text** (ID 663): Displays current multiplier value
  3. **Number of Things Scrollbar** (ID 661): Range 1-6, controls how many things
  4. **Number Static Text** (ID 664): Displays current count
  5. **Time Interval Scrollbar** (ID 662): Range 20-1000ms, controls timer interval
  6. **Time Static Text** (ID 665): Displays current interval
  7. **"They exist" Checkbox** (ID 666): Enables/disables random things
  8. **"Icon twirl on" Checkbox** (ID 667): Enables/disables icon animation

### Animation Logic

#### Movement Algorithm (per timer tick)
For each "thing" (1 to NoThings):

1. **Erase old position**: Fill 31×31 rectangle with green at (XPos[i], YPos[i])

2. **Update position** using random walk:
   ```pascal
   XPos[i] := XPos[i] + Multiplier * (RANDOM(3) - 1)  // -1, 0, or +1
   YPos[i] := YPos[i] + Multiplier * (RANDOM(3) - 1)  // -1, 0, or +1
   ```

3. **Boundary checks**:
   - Keep within window: `0 <= X <= (WindowWidth - 31)`, `0 <= Y <= (WindowHeight - 95)`
   - Avoid trump logo area: `(254..316, 49..111)` - push to edge if collision
   - Avoid button area: `(>500, >200)` - push back if collision

4. **Draw new position**: Display appropriate bitmap (31×31) at new coordinates

#### Bitmap Assignment
Things 1-4 use suit icon bitmaps (we only have 4, not 6):
1. Club suit icon
2. Diamond suit icon
3. Heart suit icon
4. Spade suit icon

Note: Original had MD_Logo and IC_Logo at positions 5-6, but these aren't in extracted resources.

#### Timer Management
- **Random Things Timer** (ID 2000): Set when ExistRnd is true and no game in progress
- **Icon Twirl Timer** (ID 2001): Set when ExistIcn is true and window is minimized
- Both timers killed when game starts
- Random things timer restarted when game ends (if ExistRnd is true)

### Icon Twirl Feature
When window is minimized and icon twirl is enabled:
- Cycles through 3 application icons (ICON1, ICON2, ICON3)
- Updates every TimeIco milliseconds (default 100ms)
- Uses IconCount to track current icon (1-3)

### State Persistence
Settings saved to Registry under `HKEY_CURRENT_USER\Software\Estwhi\Random Things`:
- `Multiplier` (DWORD, default: 6)
- `Number of` (DWORD, default: 6)
- `Time interval` (DWORD, default: 20)
- `They exist` (DWORD, default: 1/true)
- `Icon twirl` (DWORD, default: 1/true)

## Rust Reimplementation Design

### Dependencies
Add to `Cargo.toml`:
```toml
rand = "0.8"
winreg = "0.52"
```

### Data Structures

```rust
use rand::Rng;

const ID_RNDTIMER: usize = 2000;  // Random things timer
const ID_ICNTIMER: usize = 2001;  // Icon animation timer
const THING_SIZE: i32 = 31;       // Icon size (31x31 pixels)

#[derive(Clone, Debug)]
struct RandomThingsConfig {
    enabled: bool,              // ExistRnd
    icon_twirl_enabled: bool,   // ExistIcn
    multiplier: i32,            // 1-20, movement speed
    count: usize,               // 1-4, number of things (limited by available bitmaps)
    interval_ms: u32,           // 20-1000, timer interval
}

impl Default for RandomThingsConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            icon_twirl_enabled: true,
            multiplier: 6,
            count: 4,               // Default to 4 (was 6 in original, but we only have 4 bitmaps)
            interval_ms: 200,       // Changed from 20ms (too fast) to 200ms default
        }
    }
}

#[derive(Clone, Debug)]
struct RandomThingState {
    x: i32,
    y: i32,
    bitmap_index: usize,     // 0-3 for which suit icon to use
}

struct RandomThings {
    config: RandomThingsConfig,
    things: Vec<RandomThingState>,
    random_timer_active: bool,
    icon_timer_active: bool,
    icon_count: usize,       // 0-2 for icon animation
}

impl RandomThings {
    fn new() -> Self {
        Self {
            config: RandomThingsConfig::default(),
            things: Vec::new(),
            random_timer_active: false,
            icon_timer_active: false,
            icon_count: 0,
        }
    }

    fn validate_and_fix_config(&mut self) {
        // Clamp values to valid ranges
        self.config.multiplier = self.config.multiplier.clamp(1, 20);
        self.config.count = self.config.count.clamp(1, 4);  // Max 4 (we only have 4 bitmaps)
        self.config.interval_ms = self.config.interval_ms.clamp(20, 1000);
    }

    fn resize_things(&mut self, client_width: i32, client_height: i32) {
        let current_count = self.things.len();
        let new_count = self.config.count;

        if new_count > current_count {
            // Add new things at center position
            for i in current_count..new_count {
                self.things.push(RandomThingState {
                    x: client_width / 2,
                    y: client_height / 2,
                    bitmap_index: i % 4,  // Cycle through 4 suit icons
                });
            }
        } else if new_count < current_count {
            // Remove excess things
            self.things.truncate(new_count);
        }
    }
}
```

### Configuration Dialog

Dialog resource (app.rc):
```rc
3006 DIALOG 25, 20, 192, 88
STYLE DS_MODALFRAME | WS_POPUP | WS_CAPTION | WS_SYSMENU
CAPTION "Random things"
FONT 8, "MS Sans Serif"
BEGIN
    GROUPBOX "Settings", -1, 8, 6, 176, 56

    LTEXT "Multiplier (1-20):", -1, 16, 18, 70, 8
    CONTROL "", 660, "SCROLLBAR", SBS_HORZ | WS_CHILD | WS_VISIBLE | WS_TABSTOP, 90, 16, 80, 12
    LTEXT "6", 663, 174, 18, 10, 8

    LTEXT "Number of (1-4):", -1, 16, 32, 70, 8
    CONTROL "", 661, "SCROLLBAR", SBS_HORZ | WS_CHILD | WS_VISIBLE | WS_TABSTOP, 90, 30, 80, 12
    LTEXT "4", 664, 174, 32, 10, 8

    LTEXT "Time interval (ms):", -1, 16, 46, 70, 8
    CONTROL "", 662, "SCROLLBAR", SBS_HORZ | WS_CHILD | WS_VISIBLE | WS_TABSTOP, 90, 44, 80, 12
    LTEXT "200", 665, 174, 46, 16, 8

    AUTOCHECKBOX "They &exist", 666, 16, 68, 70, 10
    AUTOCHECKBOX "Icon &twirl on", 667, 96, 68, 70, 10

    DEFPUSHBUTTON "OK", IDOK, 52, 70, 50, 14
    PUSHBUTTON "Cancel", IDCANCEL, 108, 70, 50, 14
END
```

### Dialog Procedure

```rust
const IDC_RNDMULTSC: i32 = 660;
const IDC_RNDNUMBSC: i32 = 661;
const IDC_RNDTIMESC: i32 = 662;
const IDC_RNDMULTST: i32 = 663;
const IDC_RNDNUMBST: i32 = 664;
const IDC_RNDTIMEST: i32 = 665;
const IDC_RNDEXISCK: i32 = 666;
const IDC_RNDICONCK: i32 = 667;

extern "system" fn random_dlg_proc(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> isize {
    unsafe {
        match msg {
            WM_INITDIALOG => {
                let app = app_state().lock().unwrap();
                let cfg = &app.random_things.config;

                // Set scrollbar ranges and positions
                let mult_sb = GetDlgItem(hwnd, IDC_RNDMULTSC).unwrap();
                SendMessageW(mult_sb, SBM_SETRANGE, WPARAM(1), LPARAM(20));
                SendMessageW(mult_sb, SBM_SETPOS, WPARAM(cfg.multiplier as usize), LPARAM(1));

                let numb_sb = GetDlgItem(hwnd, IDC_RNDNUMBSC).unwrap();
                SendMessageW(numb_sb, SBM_SETRANGE, WPARAM(1), LPARAM(4));
                SendMessageW(numb_sb, SBM_SETPOS, WPARAM(cfg.count), LPARAM(1));

                let time_sb = GetDlgItem(hwnd, IDC_RNDTIMESC).unwrap();
                SendMessageW(time_sb, SBM_SETRANGE, WPARAM(20), LPARAM(1000));
                SendMessageW(time_sb, SBM_SETPOS, WPARAM(cfg.interval_ms as usize), LPARAM(1));

                // Set static text controls
                let mult_text = wide(&format!("{}", cfg.multiplier));
                SetDlgItemTextW(hwnd, IDC_RNDMULTST, PCWSTR(mult_text.as_ptr()));

                let numb_text = wide(&format!("{}", cfg.count));
                SetDlgItemTextW(hwnd, IDC_RNDNUMBST, PCWSTR(numb_text.as_ptr()));

                let time_text = wide(&format!("{}", cfg.interval_ms));
                SetDlgItemTextW(hwnd, IDC_RNDTIMEST, PCWSTR(time_text.as_ptr()));

                // Set checkboxes
                CheckDlgButton(hwnd, IDC_RNDEXISCK, if cfg.enabled { BST_CHECKED } else { BST_UNCHECKED });
                CheckDlgButton(hwnd, IDC_RNDICONCK, if cfg.icon_twirl_enabled { BST_CHECKED } else { BST_UNCHECKED });

                1
            }

            WM_HSCROLL => {
                // Get which scrollbar sent the message
                let scrollbar = HWND(lparam.0 as _);
                let pos = SendMessageW(scrollbar, SBM_GETPOS, WPARAM(0), LPARAM(0)).0 as i32;

                // Determine which scrollbar and update corresponding static text
                let sb_id = GetDlgCtrlID(scrollbar);

                match sb_id {
                    IDC_RNDMULTSC => {
                        let text = wide(&format!("{}", pos));
                        SetDlgItemTextW(hwnd, IDC_RNDMULTST, PCWSTR(text.as_ptr()));
                    }
                    IDC_RNDNUMBSC => {
                        let text = wide(&format!("{}", pos));
                        SetDlgItemTextW(hwnd, IDC_RNDNUMBST, PCWSTR(text.as_ptr()));
                    }
                    IDC_RNDTIMESC => {
                        let text = wide(&format!("{}", pos));
                        SetDlgItemTextW(hwnd, IDC_RNDTIMEST, PCWSTR(text.as_ptr()));
                    }
                    _ => {}
                }
                0
            }

            WM_COMMAND => {
                let id = loword(wparam.0 as u32) as i32;
                match id {
                    IDOK => {
                        // Get scrollbar positions
                        let mult_sb = GetDlgItem(hwnd, IDC_RNDMULTSC).unwrap();
                        let multiplier = SendMessageW(mult_sb, SBM_GETPOS, WPARAM(0), LPARAM(0)).0 as i32;

                        let numb_sb = GetDlgItem(hwnd, IDC_RNDNUMBSC).unwrap();
                        let count = SendMessageW(numb_sb, SBM_GETPOS, WPARAM(0), LPARAM(0)).0;

                        let time_sb = GetDlgItem(hwnd, IDC_RNDTIMESC).unwrap();
                        let interval_ms = SendMessageW(time_sb, SBM_GETPOS, WPARAM(0), LPARAM(0)).0 as u32;

                        // Get checkbox states
                        let enabled = IsDlgButtonChecked(hwnd, IDC_RNDEXISCK) == BST_CHECKED;
                        let icon_twirl = IsDlgButtonChecked(hwnd, IDC_RNDICONCK) == BST_CHECKED;

                        // Save to app state
                        let main_hwnd = GetParent(hwnd);
                        {
                            let mut app = app_state().lock().unwrap();
                            let old_interval = app.random_things.config.interval_ms;
                            let old_enabled = app.random_things.config.enabled;
                            let old_count = app.random_things.config.count;
                            let old_icon_twirl = app.random_things.config.icon_twirl_enabled;

                            app.random_things.config.multiplier = multiplier;
                            app.random_things.config.count = count;
                            app.random_things.config.interval_ms = interval_ms;
                            app.random_things.config.enabled = enabled;
                            app.random_things.config.icon_twirl_enabled = icon_twirl;
                            app.random_things.validate_and_fix_config();

                            // Save to registry
                            save_random_things_config(&app.random_things.config);

                            // Handle timer restart if interval changed
                            if app.random_things.random_timer_active && interval_ms != old_interval {
                                KillTimer(main_hwnd, ID_RNDTIMER);
                                SetTimer(main_hwnd, ID_RNDTIMER, interval_ms, None);
                            }

                            // Handle enabled state change
                            if enabled != old_enabled {
                                if enabled && !app.game.in_progress {
                                    start_random_things(main_hwnd);
                                } else if !enabled {
                                    stop_random_things(main_hwnd);
                                }
                            }

                            // Handle count change
                            if (count != old_count) || (enabled && !old_enabled) {
                                let mut rc = RECT::default();
                                GetClientRect(main_hwnd, &mut rc);
                                app.random_things.resize_things(rc.right, rc.bottom);
                            }

                            // Handle icon twirl change
                            if icon_twirl != old_icon_twirl {
                                if icon_twirl && IsIconic(main_hwnd).as_bool() {
                                    start_icon_twirl(main_hwnd);
                                } else if !icon_twirl {
                                    stop_icon_twirl(main_hwnd);
                                }
                            }
                        }

                        EndDialog(hwnd, 1);
                        1
                    }
                    IDCANCEL => {
                        EndDialog(hwnd, 0);
                        1
                    }
                    _ => 0
                }
            }

            _ => 0
        }
    }
}

unsafe fn show_random_things_dialog(parent: HWND) {
    let hinst = GetModuleHandleW(None).unwrap();
    DialogBoxParamW(
        hinst,
        PCWSTR(make_int_resource(3006).0),
        parent,
        Some(random_dlg_proc),
        LPARAM(0),
    );
}
```

### Registry Persistence

```rust
use winreg::enums::*;
use winreg::RegKey;

fn save_random_things_config(cfg: &RandomThingsConfig) {
    if let Ok(hkcu) = RegKey::predef(HKEY_CURRENT_USER).create_subkey("Software\\Estwhi\\Random Things") {
        let (key, _) = hkcu;
        let _ = key.set_value("Multiplier", &(cfg.multiplier as u32));
        let _ = key.set_value("Number of", &(cfg.count as u32));
        let _ = key.set_value("Time interval", &cfg.interval_ms);
        let _ = key.set_value("They exist", &(cfg.enabled as u32));
        let _ = key.set_value("Icon twirl", &(cfg.icon_twirl_enabled as u32));
    }
}

fn load_random_things_config() -> RandomThingsConfig {
    let mut cfg = RandomThingsConfig::default();

    if let Ok(key) = RegKey::predef(HKEY_CURRENT_USER).open_subkey("Software\\Estwhi\\Random Things") {
        if let Ok(v) = key.get_value::<u32, _>("Multiplier") {
            cfg.multiplier = v as i32;
        }
        if let Ok(v) = key.get_value::<u32, _>("Number of") {
            cfg.count = v as usize;
        }
        if let Ok(v) = key.get_value::<u32, _>("Time interval") {
            cfg.interval_ms = v;
        }
        if let Ok(v) = key.get_value::<u32, _>("They exist") {
            cfg.enabled = v != 0;
        }
        if let Ok(v) = key.get_value::<u32, _>("Icon twirl") {
            cfg.icon_twirl_enabled = v != 0;
        }
    }

    // Validate and clamp values
    cfg.multiplier = cfg.multiplier.clamp(1, 20);
    cfg.count = cfg.count.clamp(1, 4);
    cfg.interval_ms = cfg.interval_ms.clamp(20, 1000);

    cfg
}
```

### Timer and Animation Implementation

```rust
unsafe fn start_random_things(hwnd: HWND) {
    let mut app = app_state().lock().unwrap();

    if !app.random_things.config.enabled || app.game.in_progress {
        return;
    }

    // Initialize thing positions if needed
    let mut rc = RECT::default();
    GetClientRect(hwnd, &mut rc);
    app.random_things.resize_things(rc.right, rc.bottom);

    // Start timer
    SetTimer(hwnd, ID_RNDTIMER, app.random_things.config.interval_ms, None);
    app.random_things.random_timer_active = true;
}

unsafe fn stop_random_things(hwnd: HWND) {
    let mut app = app_state().lock().unwrap();
    if app.random_things.random_timer_active {
        KillTimer(hwnd, ID_RNDTIMER);
        app.random_things.random_timer_active = false;

        // Clear things from screen
        InvalidateRect(hwnd, None, TRUE);
    }
}

unsafe fn start_icon_twirl(hwnd: HWND) {
    let mut app = app_state().lock().unwrap();

    if !app.random_things.config.icon_twirl_enabled {
        return;
    }

    SetTimer(hwnd, ID_ICNTIMER, 100, None);  // 100ms interval for icon animation
    app.random_things.icon_timer_active = true;
    app.random_things.icon_count = 0;
}

unsafe fn stop_icon_twirl(hwnd: HWND) {
    let mut app = app_state().lock().unwrap();
    if app.random_things.icon_timer_active {
        KillTimer(hwnd, ID_ICNTIMER);
        app.random_things.icon_timer_active = false;
    }
}

unsafe fn on_random_timer(hwnd: HWND) {
    // Don't animate when minimized
    if IsIconic(hwnd).as_bool() {
        return;
    }

    let mut rc = RECT::default();
    GetClientRect(hwnd, &mut rc);
    let client_width = rc.right;
    let client_height = rc.bottom;

    let hdc = GetDC(hwnd);

    UI_HANDLES.with(|h| {
        let hh = h.borrow();
        let green_brush = hh.hbr_green;

        let mut app = app_state().lock().unwrap();

        // Phase 1: Clear old positions
        for thing in &app.random_things.things {
            let rect = RECT {
                left: thing.x,
                top: thing.y,
                right: thing.x + THING_SIZE,
                bottom: thing.y + THING_SIZE,
            };
            FillRect(hdc, &rect, green_brush);
        }

        // Phase 2: Update positions with random walk
        let mult = app.random_things.config.multiplier;
        let mut rng = rand::thread_rng();

        for thing in &mut app.random_things.things {
            // Random walk: -1, 0, or +1, scaled by multiplier
            let dx = mult * rng.gen_range(-1..=1);
            let dy = mult * rng.gen_range(-1..=1);

            thing.x += dx;
            thing.y += dy;

            // Boundary checks - keep within window
            if thing.x < 0 {
                thing.x += mult;
            }
            if thing.x > client_width - THING_SIZE {
                thing.x -= mult;
            }
            if thing.y < 0 {
                thing.y += mult;
            }
            // Leave room for status bar at bottom
            if thing.y > client_height - THING_SIZE - 24 {
                thing.y -= mult;
            }

            // Avoid trump logo area (absolute coordinates from original - may not apply to new layout)
            // We'll skip this for now since layout is different
            // Original: if (((XPos[A] < 316) AND (XPos[A] > 254)) AND ((YPos[A] < 111) AND (YPos[A] > 49)))

            // Avoid button area (absolute coordinates from original - may not apply to new layout)
            // We'll skip this for now since buttons are in different location
            // Original: IF ((YPos[A] > 200) AND (XPos[A] > 500))
        }

        // Phase 3: Draw new positions
        for thing in &app.random_things.things {
            // Get the appropriate suit bitmap
            if let Some(bitmaps) = SUIT_BITMAPS.get() {
                let bitmaps = bitmaps.lock().unwrap();
                if thing.bitmap_index < 4 {
                    // CLUB=0, DIAMOND=1, SPADE=2, HEART=3
                    if let Some(bmp_ptr) = bitmaps[thing.bitmap_index] {
                        let hbmp = HBITMAP(bmp_ptr as *mut _);
                        blit_bitmap(hdc, hbmp, thing.x, thing.y, THING_SIZE, THING_SIZE);
                    }
                }
            }
        }
    });

    ReleaseDC(hwnd, hdc);
}

unsafe fn on_icon_timer(hwnd: HWND) {
    // Only animate when minimized
    if !IsIconic(hwnd).as_bool() {
        return;
    }

    let hdc = GetDC(hwnd);

    let mut app = app_state().lock().unwrap();

    // Cycle through 3 icons
    app.random_things.icon_count = (app.random_things.icon_count + 1) % 3;

    // Load and draw the appropriate icon
    let hinst = GetModuleHandleW(None).unwrap();
    let icon_id = make_int_resource((1 + app.random_things.icon_count) as u16);  // ICON1, ICON2, ICON3

    if let Ok(icon) = LoadIconW(hinst, icon_id) {
        DrawIcon(hdc, 0, 0, icon);
    }

    ReleaseDC(hwnd, hdc);
}
```

### WM_TIMER Handler Integration

In the main window procedure:

```rust
WM_TIMER => {
    match wparam.0 {
        ID_RNDTIMER => {
            on_random_timer(hwnd);
            LRESULT(0)
        }
        ID_ICNTIMER => {
            on_icon_timer(hwnd);
            LRESULT(0)
        }
        _ => DefWindowProcW(hwnd, msg, wparam, lparam)
    }
}
```

### WM_SIZE Handler Integration

Handle window minimize/restore for icon twirl:

```rust
WM_SIZE => {
    let size_type = wparam.0 as u32;

    match size_type {
        SIZE_MINIMIZED => {
            // Window minimized - start icon twirl if enabled
            stop_random_things(hwnd);
            start_icon_twirl(hwnd);
        }
        SIZE_RESTORED | SIZE_MAXIMIZED => {
            // Window restored - stop icon twirl, restart random things
            stop_icon_twirl(hwnd);
            start_random_things(hwnd);
        }
        _ => {}
    }

    DefWindowProcW(hwnd, msg, wparam, lparam)
}
```

### Game State Integration

```rust
// In start_game() or when dealing cards:
unsafe fn start_game(hwnd: HWND) {
    stop_random_things(hwnd);
    // ... rest of game start logic
}

// When game ends:
unsafe fn end_game(hwnd: HWND) {
    // ... game end logic
    start_random_things(hwnd);
}
```

### Menu Integration

Add to `app.rc`:
```rc
POPUP "&Game"
BEGIN
    MENUITEM "&Deal\tF2", 100
    MENUITEM "S&cores\tF3", 101
    MENUITEM "&Options...", 102
    MENUITEM "&Random things...", 103
    MENUITEM SEPARATOR
    MENUITEM "E&xit", 104
END
```

Handle in WM_COMMAND:
```rust
103 => {  // CM_GAMERAND
    show_random_things_dialog(hwnd);
    LRESULT(0)
}
```

### Initialization at Startup

```rust
// In WinMain or window creation:
unsafe fn initialize_app(hwnd: HWND) {
    let mut app = app_state().lock().unwrap();

    // Load config from registry
    app.random_things.config = load_random_things_config();
    app.random_things.validate_and_fix_config();

    drop(app);  // Release lock before calling functions

    // Start random things if enabled
    start_random_things(hwnd);
}
```

## Implementation Checklist

- [ ] Add `rand` and `winreg` dependencies to Cargo.toml
- [ ] Add RandomThingsConfig, RandomThingState, and RandomThings structs to AppState
- [ ] Add timer constants (ID_RNDTIMER, ID_ICNTIMER)
- [ ] Create RANDOM dialog resource (ID 3006) in app.rc
- [ ] Implement random_dlg_proc with WM_INITDIALOG, WM_HSCROLL, WM_COMMAND handlers
- [ ] Implement save_random_things_config and load_random_things_config
- [ ] Implement start/stop functions for both random things and icon twirl
- [ ] Add WM_TIMER handler with cases for both timers
- [ ] Add WM_SIZE handler for minimize/restore
- [ ] Implement on_random_timer with random walk algorithm
- [ ] Implement on_icon_timer for icon animation
- [ ] Add menu item "Random things..." (ID 103) and handler
- [ ] Call load_random_things_config and start_random_things at startup
- [ ] Integrate with game start/end in deal/scoring code
- [ ] Load 3 application icons (ICON1, ICON2, ICON3) as resources
- [ ] Test with various configurations and edge cases

## Key Fixes from Original Design

1. **Timer ID Management**: Use boolean flags instead of storing timer IDs
2. **Icon Twirl**: Fully implemented with separate timer and icon cycling
3. **Bitmap Limit**: Restricted to 4 things max (we only have 4 suit bitmaps)
4. **Random Generation**: Use `rand::thread_rng().gen_range(-1..=1)` instead of modulo
5. **Dialog Syntax**: Proper CONTROL syntax for scrollbars
6. **WM_HSCROLL**: Full implementation with GetDlgCtrlID to identify scrollbar
7. **WM_INITDIALOG**: Full implementation with SBM_SETRANGE and SBM_SETPOS
8. **Registry**: Use `winreg` crate with proper key path
9. **Validation**: Clamp values when loading from registry
10. **Timer Restart**: Kill and restart timer when interval changes
11. **Resize Logic**: Properly handle count changes
12. **Type Safety**: Proper i32/usize conversions for Windows APIs
13. **Cancel Button**: Added to dialog
14. **Boundary Checks**: Use dynamic window size, not hardcoded coordinates
15. **Icon Twirl Config**: Added to data structures and persistence

## Notes

- Default interval changed from 20ms to 200ms (20ms is too fast on modern systems)
- Hardcoded collision avoidance removed (layout is different in new version)
- Limited to 4 things max since MD_Logo and IC_Logo bitmaps not available
- Icon animation requires 3 icon resources (ICON1, ICON2, ICON3)
- Uses existing SUIT_BITMAPS static for drawing
- Timer IDs must not conflict with other timers in the application
