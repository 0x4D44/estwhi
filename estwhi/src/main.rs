#![windows_subsystem = "windows"]
#![allow(clippy::let_unit_value)]
#![allow(clippy::cmp_null)]
#![allow(dead_code)]

use windows::core::PCWSTR;

use windows::Win32::Foundation::{BOOL, HMODULE, HWND, LPARAM, LRESULT, RECT, WPARAM};

use windows::Win32::Graphics::Gdi::{
    BeginPaint, BitBlt, CreateCompatibleBitmap, CreateCompatibleDC,
    CreateCompatibleDC as CreateCDC, CreateSolidBrush, DeleteDC, DeleteDC as DeleteDc,
    DeleteObject, EndPaint, FillRect, GetSysColor, SelectObject, SelectObject as SelectObj,
    SetBkMode, SetTextColor, TextOutW, COLOR_WINDOWTEXT, HBITMAP, HBRUSH, HDC, NOTSRCCOPY,
    PAINTSTRUCT, SRCCOPY, TRANSPARENT,
};

use windows::Win32::Graphics::Gdi::{GetObjectW, SetStretchBltMode, StretchBlt, BITMAP, HALFTONE};

use windows::Win32::Graphics::Gdi::Rectangle as GdiRectangle;

use windows::Win32::Foundation::COLORREF;

use windows::Win32::UI::Input::KeyboardAndMouse::EnableWindow;
use windows::Win32::UI::WindowsAndMessaging::*;

use windows::Win32::Graphics::Gdi::FrameRect;

use windows::Win32::Graphics::Gdi::{CreateFontIndirectW, LOGFONTW};

use windows::Win32::UI::WindowsAndMessaging::WM_SETFONT;

use windows::Win32::Graphics::Gdi::{
    CreateDIBitmap, GetDC, ReleaseDC, BITMAPINFO, BITMAPINFOHEADER, CBM_INIT, DIB_RGB_COLORS,
};

use windows::Win32::Graphics::Gdi::InvalidateRect;

use windows::Win32::System::LibraryLoader::{
    FindResourceW, GetModuleHandleW, LoadResource, LockResource, SizeofResource,
};

use windows::Win32::System::Diagnostics::Debug::OutputDebugStringW;

use windows::Win32::UI::HiDpi::GetDpiForWindow;

use windows::Win32::UI::Controls::InitCommonControls;

use std::ffi::c_void;

use std::sync::{Mutex, OnceLock};

use estwhi_core::{
    decide_trick_winner, is_legal_play, score_hand, sort_hand_for_display,
    suit_index_from_legacy_id, Deck, ScoreMode,
};

use rand::prelude::*;

use std::cell::RefCell;

mod registry;

#[inline]
const fn make_int_resource(id: u16) -> PCWSTR {
    PCWSTR(id as usize as *const u16)
}

#[inline]
const fn loword(val: u32) -> u16 {
    (val & 0xffff) as u16
}

const fn hiword(val: u32) -> u16 {
    ((val >> 16) & 0xffff) as u16
}

const BST_CHECKED_U: usize = 1;

const BST_UNCHECKED_U: usize = 0;

// ----- Classic rendering constants & toggle (96-DPI absolute coords) -----

// ClassicLayout is now a runtime option (Options dialog + registry). Keep constants above.

// Card bitmap nominal size

const CARD_W: i32 = 71;

const CARD_H: i32 = 96;

// Cheat window constants
const CHEAT_WINDOW_CLASS: &str = "EstwhiCheatCards";
const CHEAT_WINDOW_WIDTH_BASE: f32 = 400.0;
const CHEAT_WINDOW_HEIGHT_BASE: f32 = 200.0;
const SMALL_CARD_WIDTH_BASE: f32 = 41.0;
const SMALL_CARD_HEIGHT_BASE: f32 = 55.0;
const SMALL_MIN_WIDTH_BASE: f32 = 25.0;

// Hand row absolute layout

const HAND_Y: i32 = 228; // top-left Y for hand cards

const HAND_X0: i32 = 10; // leftmost X for first hand card

const HAND_SPAN_X: i32 = 500; // span used to compute classic ActWidth

const MIN_WIDTH: i32 = 20; // minimum spacing threshold (classic message when too small)

// Trick row absolute layout

const TRICK_Y: i32 = 55;

const TRICK_X0: i32 = 10;

const TRICK_STEP: i32 = 30;

const CLIENT_WIDTH: i32 = 600;
const CLIENT_HEIGHT: i32 = 360;
const DEAL_BTN_X: f32 = 530.0;
const DEAL_BTN_Y: f32 = 232.0;
const EXIT_BTN_Y: f32 = 285.0;
const BUTTON_SIZE: f32 = 45.0;
const STATUS_HEIGHT: f32 = 24.0;

// Random Things constants
const ID_RNDTIMER: usize = 2000;
const ID_ICNTIMER: usize = 2001;
const THING_SIZE: i32 = 31;

// Random Things dialog control IDs
const IDC_RNDMULTSC: i32 = 660;
const IDC_RNDNUMBSC: i32 = 661;
const IDC_RNDTIMESC: i32 = 662;
const IDC_RNDMULTST: i32 = 663;
const IDC_RNDNUMBST: i32 = 664;
const IDC_RNDTIMEST: i32 = 665;
const IDC_RNDEXISCK: i32 = 666;
const IDC_RNDICONCK: i32 = 667;

// Legacy classic layout rectangles (mirrors DrawControls/DrawPlayedCards in estwhi.pas)
// Score panel: (340,20)-(585, 60 + 15 * players)
// Played cards caption: (20,20)-(130,47); Your hand caption: (20,198)-(130,225)
// Extra info box: (410,160)-(585,225); buttons remain at (530,232) and (530,285).

#[derive(Copy, Clone, Debug, PartialEq, Eq)]

enum NextNotify {
    Dialog = 0,
    Mouse = 1,
}

#[derive(Clone, Debug)]

struct UiConfig {
    num_players: u32, // 2..6

    max_cards: u32, // 1..15

    score_mode: ScoreMode,

    next_notify: NextNotify,

    confirm_exit: bool,

    hard_score: bool,

    cheat_cards: bool,

    classic_layout: bool,
}

impl Default for UiConfig {
    fn default() -> Self {
        Self {
            num_players: 4,

            max_cards: 13,

            score_mode: ScoreMode::Vanilla,

            next_notify: NextNotify::Mouse,

            confirm_exit: true,

            hard_score: false,

            cheat_cards: false,

            classic_layout: true,
        }
    }
}

#[derive(Clone, Debug, Default)]

struct GameState {
    in_progress: bool,

    round_no: u32,

    trump: u32, // 1..4

    start_player: u32,

    dealt_cards: u32,

    calls: Vec<u32>,

    tricks: Vec<u32>,

    scores: Vec<u32>,

    last_winner: Option<u32>,

    hand: Vec<u32>,

    hand_positions: Vec<RECT>,

    selected: Option<usize>,

    hands: Vec<Vec<u32>>, // all players' hands (legacy ids)

    trick: Vec<Option<u32>>, // current trick cards per player index

    waiting_for_human: bool,

    current_player: usize,

    cards_remaining: u32,

    // bidding helper for call dialog: forbidden call value for last bidder (if any)
    bidding_forbidden: Option<u32>,

    waiting_for_continue: bool,
}

#[derive(Clone, Debug)]
struct RandomThingsConfig {
    enabled: bool,
    icon_twirl_enabled: bool,
    multiplier: i32,
    count: usize,
    interval_ms: u32,
}

impl Default for RandomThingsConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            icon_twirl_enabled: true,
            multiplier: 6,
            count: 4,
            interval_ms: 200,
        }
    }
}

#[derive(Clone, Debug)]
struct RandomThingState {
    x: i32,
    y: i32,
    bitmap_index: usize,
}

#[derive(Default)]
struct RandomThings {
    config: RandomThingsConfig,
    things: Vec<RandomThingState>,
    random_timer_active: bool,
    icon_timer_active: bool,
    icon_count: usize,
}

impl RandomThings {
    fn validate_and_fix_config(&mut self) {
        self.config.multiplier = self.config.multiplier.clamp(1, 20);
        self.config.count = self.config.count.clamp(1, 4);
        self.config.interval_ms = self.config.interval_ms.clamp(20, 1000);
    }

    fn resize_things(&mut self, client_width: i32, client_height: i32) {
        let current_count = self.things.len();
        let new_count = self.config.count;

        match new_count.cmp(&current_count) {
            std::cmp::Ordering::Greater => {
                for i in current_count..new_count {
                    // Initialize in the table area (left side), not center of whole window
                    // The table is roughly 2/3 of width and most of height
                    let safe_x = (client_width / 3).max(100);
                    let safe_y = (client_height / 3).max(100);
                    self.things.push(RandomThingState {
                        x: safe_x,
                        y: safe_y,
                        bitmap_index: i % 4,
                    });
                }
            }
            std::cmp::Ordering::Less => {
                self.things.truncate(new_count);
            }
            std::cmp::Ordering::Equal => {}
        }
    }
}

#[derive(Default)]

struct CheatWindowState {
    hwnd: Option<isize>,
    offset_x: i32, // Offset from main window left edge
    offset_y: i32, // Offset from main window top edge
}

struct AppState {
    config: UiConfig,

    game: GameState,

    random_things: RandomThings,

    cheat_window: CheatWindowState,
}

#[derive(Copy, Clone)]

struct UiHandles {
    status_hwnd: HWND,

    status_font: windows::Win32::Graphics::Gdi::HFONT,

    hbr_green: HBRUSH,

    hbr_gray: HBRUSH,

    deal_btn: HWND,

    exit_btn: HWND,
}

thread_local! {



    static UI_HANDLES: RefCell<UiHandles> = const { RefCell::new(UiHandles { status_hwnd: HWND(0 as _), status_font: windows::Win32::Graphics::Gdi::HFONT(0 as _), hbr_green: HBRUSH(0 as _), hbr_gray: HBRUSH(0 as _), deal_btn: HWND(0 as _), exit_btn: HWND(0 as _) }) };



}

static APP_STATE: OnceLock<Mutex<AppState>> = OnceLock::new();

fn app_state() -> &'static Mutex<AppState> {
    APP_STATE.get_or_init(|| {
        let mut app = AppState {
            config: load_config_from_registry(),
            game: GameState::default(),
            random_things: RandomThings::default(),
            cheat_window: load_cheat_window_state(),
        };
        app.random_things.config = load_random_things_config();
        app.random_things.validate_and_fix_config();
        Mutex::new(app)
    })
}

fn load_config_from_registry() -> UiConfig {
    let mut cfg = UiConfig::default();

    let np = registry::get_u32("NumberOfPlayers", cfg.num_players);

    let mc = registry::get_u32("MaxCards", cfg.max_cards);

    let sm = registry::get_u32("ScoreMode", 0);

    let nn = registry::get_u32("NextCardNotify", 1);

    let ce = registry::get_u32("ConfirmExit", 1);

    let hs = registry::get_u32("HardScore", 0);

    let ch = registry::get_u32("CheatCards", 0);

    let cl = registry::get_u32("ClassicLayout", 1);

    cfg.num_players = np.clamp(2, 6);

    cfg.max_cards = mc.clamp(1, 15);

    cfg.score_mode = if sm == 0 {
        ScoreMode::Vanilla
    } else {
        ScoreMode::Squared
    };

    cfg.next_notify = if nn == 0 {
        NextNotify::Dialog
    } else {
        NextNotify::Mouse
    };

    cfg.confirm_exit = ce != 0;

    cfg.hard_score = hs != 0;

    cfg.cheat_cards = ch != 0;

    cfg.classic_layout = cl != 0;

    cfg
}

fn save_config_to_registry(cfg: &UiConfig) {
    let _ = registry::set_u32("NumberOfPlayers", cfg.num_players);

    let _ = registry::set_u32("MaxCards", cfg.max_cards);

    let _ = registry::set_u32(
        "ScoreMode",
        match cfg.score_mode {
            ScoreMode::Vanilla => 0,
            ScoreMode::Squared => 1,
        },
    );

    let _ = registry::set_u32(
        "NextCardNotify",
        match cfg.next_notify {
            NextNotify::Dialog => 0,
            NextNotify::Mouse => 1,
        },
    );

    let _ = registry::set_u32("ConfirmExit", if cfg.confirm_exit { 1 } else { 0 });

    let _ = registry::set_u32("HardScore", if cfg.hard_score { 1 } else { 0 });

    let _ = registry::set_u32("CheatCards", if cfg.cheat_cards { 1 } else { 0 });

    let _ = registry::set_u32("ClassicLayout", if cfg.classic_layout { 1 } else { 0 });
}

fn load_cheat_window_state() -> CheatWindowState {
    CheatWindowState {
        hwnd: None,
        offset_x: registry::get_u32("CheatWindowOffsetX", 100) as i32,
        offset_y: registry::get_u32("CheatWindowOffsetY", 100) as i32,
    }
}

fn save_cheat_window_state(state: &CheatWindowState) {
    let _ = registry::set_u32("CheatWindowOffsetX", state.offset_x as u32);
    let _ = registry::set_u32("CheatWindowOffsetY", state.offset_y as u32);
}

fn load_random_things_config() -> RandomThingsConfig {
    let def = RandomThingsConfig::default();
    let mut cfg = RandomThingsConfig {
        multiplier: registry::rt_get_u32("Multiplier", def.multiplier as u32) as i32,
        count: registry::rt_get_u32("Number of", def.count as u32) as usize,
        interval_ms: registry::rt_get_u32("Time interval", def.interval_ms),
        enabled: registry::rt_get_u32("They exist", if def.enabled { 1 } else { 0 }) != 0,
        icon_twirl_enabled: registry::rt_get_u32(
            "Icon twirl",
            if def.icon_twirl_enabled { 1 } else { 0 },
        ) != 0,
    };
    cfg.multiplier = cfg.multiplier.clamp(1, 20);
    cfg.count = cfg.count.clamp(1, 4);
    cfg.interval_ms = cfg.interval_ms.clamp(20, 1000);
    cfg
}

fn save_random_things_config(cfg: &RandomThingsConfig) {
    let _ = registry::rt_set_u32("Multiplier", cfg.multiplier as u32);
    let _ = registry::rt_set_u32("Number of", cfg.count as u32);
    let _ = registry::rt_set_u32("Time interval", cfg.interval_ms);
    let _ = registry::rt_set_u32("They exist", if cfg.enabled { 1 } else { 0 });
    let _ = registry::rt_set_u32("Icon twirl", if cfg.icon_twirl_enabled { 1 } else { 0 });
}

unsafe fn start_random_things(hwnd: HWND) {
    let mut app = app_state().lock().unwrap();
    if !app.random_things.config.enabled || app.game.in_progress {
        return;
    }
    let mut rc = RECT::default();
    let _ = GetClientRect(hwnd, &mut rc);
    app.random_things.resize_things(rc.right, rc.bottom);
    let _ = SetTimer(
        hwnd,
        ID_RNDTIMER,
        app.random_things.config.interval_ms,
        None,
    );
    app.random_things.random_timer_active = true;
}

unsafe fn stop_random_things(hwnd: HWND) {
    let mut app = app_state().lock().unwrap();
    if app.random_things.random_timer_active {
        let _ = KillTimer(hwnd, ID_RNDTIMER);
        app.random_things.random_timer_active = false;
        drop(app);
        let _ = InvalidateRect(hwnd, None, BOOL(1));
    }
}

unsafe fn start_icon_twirl(hwnd: HWND) {
    let mut app = app_state().lock().unwrap();
    if !app.random_things.config.icon_twirl_enabled {
        return;
    }
    let _ = SetTimer(hwnd, ID_ICNTIMER, 100, None);
    app.random_things.icon_timer_active = true;
    app.random_things.icon_count = 0;
}

unsafe fn stop_icon_twirl(hwnd: HWND) {
    let mut app = app_state().lock().unwrap();
    if app.random_things.icon_timer_active {
        let _ = KillTimer(hwnd, ID_ICNTIMER);
        app.random_things.icon_timer_active = false;
    }
}

unsafe fn on_random_timer(hwnd: HWND) {
    if IsIconic(hwnd).as_bool() {
        return;
    }
    let mut rc = RECT::default();
    let _ = GetClientRect(hwnd, &mut rc);

    let dpi = GetDpiForWindow(hwnd) as f32;
    let scale = dpi / 96.0;

    // When no game is in progress, entire screen is green - random things can roam anywhere
    // (Original behavior: clear green screen with only a logo at 285,80)
    let table_left = 0;
    let table_top = 0;
    let table_right = rc.right;
    let table_bottom = rc.bottom - (24.0 * scale).round() as i32; // Leave room for status bar

    let hdc = GetDC(hwnd);
    UI_HANDLES.with(|h| {
        let hh = h.borrow();
        let green_brush = hh.hbr_green;
        let mut app = app_state().lock().unwrap();

        // Phase 1: Clear old positions with green
        for thing in &app.random_things.things {
            let rect = RECT {
                left: thing.x,
                top: thing.y,
                right: thing.x + THING_SIZE,
                bottom: thing.y + THING_SIZE,
            };
            let _ = FillRect(hdc, &rect, green_brush);
        }

        // Phase 2: Update positions with random walk, constrained to table area
        let mult = app.random_things.config.multiplier;
        let mut rng = rand::thread_rng();
        for thing in &mut app.random_things.things {
            let dx = mult * rng.gen_range(-1..=1);
            let dy = mult * rng.gen_range(-1..=1);
            thing.x += dx;
            thing.y += dy;

            // Constrain to table area boundaries
            if thing.x < table_left {
                thing.x = table_left;
            }
            if thing.x > table_right - THING_SIZE {
                thing.x = table_right - THING_SIZE;
            }
            if thing.y < table_top {
                thing.y = table_top;
            }
            if thing.y > table_bottom - THING_SIZE {
                thing.y = table_bottom - THING_SIZE;
            }

            // Avoid logo at (285, 80) - original collision area was 254-316, 49-111
            let logo_x = (285.0 * scale).round() as i32;
            let logo_y = (80.0 * scale).round() as i32;
            let logo_left = logo_x - (31.0 * scale).round() as i32;
            let logo_right = logo_x + (31.0 * 2.0 * scale).round() as i32;
            let logo_top = logo_y - (31.0 * scale).round() as i32;
            let logo_bottom = logo_y + (31.0 * 2.0 * scale).round() as i32;

            if thing.x >= logo_left
                && thing.x <= logo_right
                && thing.y >= logo_top
                && thing.y <= logo_bottom
            {
                // Push to nearest edge of logo area
                if thing.x < logo_x {
                    thing.x = logo_left;
                } else {
                    thing.x = logo_right;
                }
                if thing.y < logo_y {
                    thing.y = logo_top;
                } else {
                    thing.y = logo_bottom;
                }
            }
        }

        // Phase 3: Draw new positions
        for thing in &app.random_things.things {
            let name = match thing.bitmap_index {
                0 => "CLUB",
                1 => "DIAMOND",
                2 => "SPADE",
                _ => "HEART",
            };
            if let Ok(obj) = LoadImageW(
                GetModuleHandleW(None).unwrap(),
                PCWSTR(wide(name).as_ptr()),
                IMAGE_BITMAP,
                0,
                0,
                LR_DEFAULTSIZE | LR_CREATEDIBSECTION,
            ) {
                blit_bitmap(
                    hdc,
                    HBITMAP(obj.0),
                    thing.x,
                    thing.y,
                    THING_SIZE,
                    THING_SIZE,
                );
            }
        }
    });
    let _ = ReleaseDC(hwnd, hdc);
}

unsafe fn on_icon_timer(hwnd: HWND) {
    if !IsIconic(hwnd).as_bool() {
        return;
    }
    let mut app = app_state().lock().unwrap();
    app.random_things.icon_count = (app.random_things.icon_count + 1) % 3;
    let hinst = GetModuleHandleW(None).unwrap();
    let icon_id = make_int_resource((1 + app.random_things.icon_count) as u16);
    if let Ok(icon) = LoadIconW(hinst, icon_id) {
        let hdc = GetDC(hwnd);
        let _ = DrawIcon(hdc, 0, 0, icon);
        let _ = ReleaseDC(hwnd, hdc);
    }
}

fn request_redraw(hwnd: HWND) {
    unsafe {
        let _ = InvalidateRect(hwnd, None, BOOL(1));
    }
}

fn set_status(text: &str) {
    // Add leading spaces for left padding in status bar
    let padded_text = format!("  {}", text);
    let w = wide(&padded_text);

    UI_HANDLES.with(|h| unsafe {
        let hh = h.borrow();

        if !hh.status_hwnd.0.is_null() {
            let _ = SetWindowTextW(hh.status_hwnd, PCWSTR(w.as_ptr()));
        }
    });
}

fn debug_out(s: &str) {
    let mut msg = String::new();

    msg.push_str("[ESTWHI] ");

    msg.push_str(s);

    msg.push('\n');

    let w = wide(&msg);

    unsafe {
        OutputDebugStringW(PCWSTR(w.as_ptr()));
    }
}

macro_rules! dbglog {
    ($($t:tt)*) => {{
        debug_out(&format!($($t)*));
    }};
}

fn next_player_to_act(start_player1: u32, trick: &[Option<u32>]) -> Option<usize> {
    let n = trick.len();
    if n == 0 {
        return None;
    }
    let start0 = start_player1.saturating_sub(1) as usize;
    for i in 0..n {
        let p = (start0 + i) % n;
        if trick[p].is_none() {
            return Some(p);
        }
    }
    None
}
static CARD_BITMAPS: OnceLock<Mutex<Vec<Option<isize>>>> = OnceLock::new();

fn card_bitmap_cache() -> &'static Mutex<Vec<Option<isize>>> {
    CARD_BITMAPS.get_or_init(|| Mutex::new(vec![None; 52]))
}

unsafe fn load_card_bitmap_resource(card_id: u32) -> Option<HBITMAP> {
    let hinst = GetModuleHandleW(None).ok()?;
    let res = FindResourceW(hinst, make_int_resource(card_id as u16), RT_BITMAP);
    if res.0.is_null() {
        return None;
    }
    let size = SizeofResource(hinst, res);
    if size == 0 {
        return None;
    }
    let handle = LoadResource(hinst, res).ok()?;
    let locked = LockResource(handle);
    if locked.is_null() {
        return None;
    }
    let data = locked as *const u8;
    let header_size = *(data as *const u32) as usize;
    if header_size == 0 || header_size >= size as usize {
        return None;
    }
    let header_ptr = data as *const BITMAPINFOHEADER;
    let bits_ptr = data.add(header_size);
    let hdc = GetDC(HWND(0 as _));
    if hdc.0.is_null() {
        return None;
    }
    let hbmp = CreateDIBitmap(
        hdc,
        Some(header_ptr),
        CBM_INIT as u32,
        Some(bits_ptr as *const c_void),
        Some(header_ptr as *const BITMAPINFO),
        DIB_RGB_COLORS,
    );
    let _ = ReleaseDC(HWND(0 as _), hdc);
    if hbmp.0.is_null() {
        None
    } else {
        Some(hbmp)
    }
}

fn get_card_bitmap(card_id: u32) -> Option<HBITMAP> {
    if card_id == 0 || card_id > 52 {
        return None;
    }
    let mut cache = card_bitmap_cache().lock().unwrap();
    let idx = (card_id - 1) as usize;
    if let Some(ptr) = cache[idx] {
        return Some(HBITMAP(ptr as *mut _));
    }
    let loaded = unsafe { load_card_bitmap_resource(card_id) };
    if let Some(hbmp) = loaded {
        cache[idx] = Some(hbmp.0 as isize);
        Some(hbmp)
    } else {
        dbglog!("Failed to load bitmap for card {}", card_id);
        None
    }
}

fn start_deal(hwnd: HWND) {
    let mut app = app_state().lock().unwrap();

    let n = app.config.num_players as usize;

    if app.game.round_no == 0 {
        // New game: reset scores and start at round 1

        app.game.scores = vec![0; n];

        app.game.round_no = 1;
    } else {
        app.game.round_no += 1;
    }

    let max = app.config.max_cards;

    let total_rounds = (max * 2).saturating_sub(1);

    if app.game.round_no > total_rounds {
        app.game.round_no = 1;
    }

    // Mountain deal

    let rn = app.game.round_no;

    app.game.dealt_cards = if rn <= max { rn } else { (2 * max) - rn };

    // Rotate trump and starting player each round (legacy parity)

    app.game.trump = if app.game.trump == 0 || app.game.trump == 4 {
        1
    } else {
        app.game.trump + 1
    };

    app.game.start_player = if app.game.start_player == 0 {
        1
    } else {
        let mut s = app.game.start_player + 1;
        if s > n as u32 {
            s = 1
        }
        s
    };

    app.game.in_progress = true;

    app.game.calls = vec![0; n];

    app.game.tricks = vec![0; n];

    if app.game.scores.len() != n {
        app.game.scores = vec![0; n];
    }

    app.game.last_winner = None;

    dbglog!(
        "Deal: round={} dealt={} trump(prev)={} start(prev)={} players={}",
        rn,
        app.game.dealt_cards,
        app.game.trump,
        app.game.start_player,
        n
    );

    // Deal real cards using estwhi-core Deck

    let mut rng = thread_rng();

    let deck = Deck::new_shuffled(&mut rng);

    let mut ptr = 0usize;

    app.game.hands = vec![vec![]; n];

    for p in 0..n {
        for _ in 0..(app.game.dealt_cards as usize) {
            app.game.hands[p].push(deck.cards()[ptr] as u32);

            ptr += 1;
        }

        sort_hand_for_display(&mut app.game.hands[p]);
    }

    app.game.hand = app.game.hands[0].clone();

    app.game.trick = vec![None; n];

    app.game.cards_remaining = app.game.dealt_cards;

    // Current player is whoever is first to act this trick (start seat)

    app.game.current_player =
        next_player_to_act(app.game.start_player, &app.game.trick).unwrap_or(0);

    app.game.waiting_for_human = app.game.current_player == 0;

    app.game.selected = None;

    app.game.waiting_for_continue = false;

    dbglog!(
        "Post-deal: trump={} start={} cur={} waiting={} hand0={:?}",
        app.game.trump,
        app.game.start_player,
        app.game.current_player,
        app.game.waiting_for_human,
        app.game.hand
    );

    drop(app);

    // Update cheat cards window after dealing
    unsafe {
        update_cheat_cards_window();
    }

    // Stop random things when game starts
    unsafe {
        stop_random_things(hwnd);
    }

    // Draw new hand before bidding (parity with legacy)

    request_redraw(hwnd);

    // Run bidding for all players (human + AI) with last-bidder constraint

    run_bidding(hwnd);

    // If an AI starts this trick, let them play until it's the human's turn

    advance_ai_until_human_or_trick_end(hwnd);
}

fn wide(s: &str) -> Vec<u16> {
    use std::os::windows::prelude::*;

    std::ffi::OsStr::new(s)
        .encode_wide()
        .chain(std::iter::once(0))
        .collect()
}

fn main() -> windows::core::Result<()> {
    unsafe {
        // Ensure common controls (UpDown) are registered

        InitCommonControls();

        let class_name = wide("ESTWHI_MAIN");

        let hinstance = GetModuleHandleW(None)?;

        // Load application icon from resources (ID = 1)
        // Use LoadImageW for better compatibility with custom icons
        let hicon = LoadImageW(
            hinstance,
            make_int_resource(1),
            IMAGE_ICON,
            0,
            0,
            LR_DEFAULTSIZE | LR_SHARED,
        )
        .ok()
        .map(|h| HICON(h.0))
        .unwrap_or_default();

        let wc = WNDCLASSW {
            style: CS_HREDRAW | CS_VREDRAW,

            lpfnWndProc: Some(wndproc),

            hInstance: hinstance.into(),

            hIcon: hicon,

            hCursor: LoadCursorW(None, IDC_ARROW).unwrap(),

            hbrBackground: HBRUSH(std::ptr::null_mut()),

            lpszClassName: PCWSTR(class_name.as_ptr()),

            ..Default::default()
        };

        let atom = RegisterClassW(&wc);

        if atom == 0 {
            return Err(windows::core::Error::from_win32());
        }

        // Register cheat cards window class
        register_cheat_window_class(hinstance)?;

        let title = wide("Estimation Whist");

        let style = WS_OVERLAPPED
            | WS_CAPTION
            | WS_SYSMENU
            | WS_MINIMIZEBOX
            | WS_CLIPCHILDREN
            | WS_CLIPSIBLINGS;

        let mut desired = RECT {
            left: 0,
            top: 0,
            right: CLIENT_WIDTH,
            bottom: CLIENT_HEIGHT,
        };

        AdjustWindowRectEx(&mut desired, style, BOOL(1), WINDOW_EX_STYLE::default())?;

        let width = desired.right - desired.left;
        let height = desired.bottom - desired.top;

        let hwnd = CreateWindowExW(
            WINDOW_EX_STYLE::default(),
            PCWSTR(class_name.as_ptr()),
            PCWSTR(title.as_ptr()),
            style,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            width,
            height,
            None,
            None,
            hinstance,
            None,
        )?;

        // Set window icon explicitly
        if !hicon.is_invalid() {
            let _ = SendMessageW(hwnd, WM_SETICON, WPARAM(ICON_BIG as usize), LPARAM(hicon.0 as isize));
            let _ = SendMessageW(hwnd, WM_SETICON, WPARAM(ICON_SMALL as usize), LPARAM(hicon.0 as isize));
        }

        // Load and attach menu from resources; if unavailable, create one in code

        if let Ok(hmenu) = LoadMenuW(hinstance, make_int_resource(2000)) {
            let _ = SetMenu(hwnd, hmenu);
        } else {
            create_default_menu(hwnd);
        }

        // Restore window position from registry before showing
        const NO_SAVED_POS: u32 = 0x80000000; // Sentinel value
        let saved_x = registry::get_u32("WindowX", NO_SAVED_POS);
        let saved_y = registry::get_u32("WindowY", NO_SAVED_POS);

        if saved_x != NO_SAVED_POS && saved_y != NO_SAVED_POS {
            // Convert u32 back to i32 (preserves bit pattern for negative positions)
            let x_pos = saved_x as i32;
            let y_pos = saved_y as i32;

            SetWindowPos(
                hwnd,
                None,
                x_pos,
                y_pos,
                0,
                0,
                SWP_NOSIZE | SWP_NOZORDER | SWP_NOACTIVATE,
            )?;
        }

        // Show the window
        let _ = ShowWindow(hwnd, SW_SHOW);

        // Initialize shared brushes and status child window

        {
            let mut rc = RECT::default();

            let _ = GetClientRect(hwnd, &mut rc);

            let dpi = GetDpiForWindow(hwnd) as i32;

            let scale = dpi as f32 / 96.0;
            let status_h = (STATUS_HEIGHT * scale).round() as i32;

            let shwnd = CreateWindowExW(
                WINDOW_EX_STYLE::default(),
                PCWSTR(wide("STATIC").as_ptr()),
                PCWSTR(wide("  Ready").as_ptr()), // Leading spaces for left padding
                WS_CHILD | WS_VISIBLE | WINDOW_STYLE(0x200), // SS_CENTERIMAGE for vertical centering
                0,
                rc.bottom - status_h,
                rc.right - rc.left,
                status_h,
                hwnd,
                None,
                hinstance,
                None,
            )
            .unwrap();

            let mut lf: LOGFONTW = core::mem::zeroed();

            lf.lfHeight = -((9 * dpi) / 72);

            let face = wide("Arial");

            let mut i = 0usize;
            while i < face.len() && i < lf.lfFaceName.len() {
                lf.lfFaceName[i] = face[i];
                i += 1;
            }

            let hfont = CreateFontIndirectW(&lf);

            let _ = SendMessageW(shwnd, WM_SETFONT, WPARAM(hfont.0 as usize), LPARAM(1));

            UI_HANDLES.with(|h| {
                let mut hh = h.borrow_mut();

                hh.status_hwnd = shwnd;

                hh.status_font = hfont;

                hh.hbr_green = CreateSolidBrush(COLORREF(128 << 8));

                hh.hbr_gray = CreateSolidBrush(COLORREF((240 << 16) | (240 << 8) | 240));

                let btn_w = (BUTTON_SIZE * scale).round() as i32;
                let btn_h = btn_w;
                let deal_x = (DEAL_BTN_X * scale).round() as i32;
                let deal_y = (DEAL_BTN_Y * scale).round() as i32;
                let exit_y = (EXIT_BTN_Y * scale).round() as i32;

                let deal_btn = CreateWindowExW(
                    WINDOW_EX_STYLE::default(),
                    PCWSTR(wide("BUTTON").as_ptr()),
                    PCWSTR(wide("&Deal").as_ptr()),
                    WINDOW_STYLE(WS_CHILD.0 | WS_VISIBLE.0 | (BS_PUSHBUTTON as u32)),
                    deal_x,
                    deal_y,
                    btn_w,
                    btn_h,
                    hwnd,
                    HMENU(100isize as _), // CM_GAMEDEAL as control id
                    hinstance,
                    None,
                )
                .unwrap();

                let _ = SendMessageW(deal_btn, WM_SETFONT, WPARAM(hfont.0 as usize), LPARAM(1));

                let exit_btn = CreateWindowExW(
                    WINDOW_EX_STYLE::default(),
                    PCWSTR(wide("BUTTON").as_ptr()),
                    PCWSTR(wide("E&xit").as_ptr()),
                    WINDOW_STYLE(WS_CHILD.0 | WS_VISIBLE.0 | (BS_PUSHBUTTON as u32)),
                    deal_x,
                    exit_y,
                    btn_w,
                    btn_h,
                    hwnd,
                    HMENU(104isize as _), // CM_GAMEEXIT
                    hinstance,
                    None,
                )
                .unwrap();

                let _ = SendMessageW(exit_btn, WM_SETFONT, WPARAM(hfont.0 as usize), LPARAM(1));

                hh.deal_btn = deal_btn;

                hh.exit_btn = exit_btn;
            });
        }

        let _ = ShowWindow(hwnd, SW_SHOW);

        // Create cheat window if flag is set
        {
            let should_create = {
                let app = app_state().lock().unwrap();
                app.config.cheat_cards
            };

            if should_create && create_cheat_cards_window(hwnd).is_err() {
                MessageBoxW(
                    hwnd,
                    PCWSTR(wide("Could not create cheat cards window!").as_ptr()),
                    PCWSTR(wide("Error").as_ptr()),
                    MB_ICONHAND | MB_OK,
                );
                app_state().lock().unwrap().config.cheat_cards = false;
            }
        }

        // Load accelerators

        let haccel = LoadAcceleratorsW(hinstance, make_int_resource(2001)).ok();

        let mut msg = MSG::default();

        while GetMessageW(&mut msg, None, 0, 0).into() {
            if let Some(acc) = haccel {
                if TranslateAcceleratorW(hwnd, acc, &msg) != 0 {
                    continue;
                }
            }

            let _ = TranslateMessage(&msg);

            DispatchMessageW(&msg);
        }
    }

    Ok(())
}

extern "system" fn wndproc(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    unsafe {
        match msg {
            WM_CREATE => {
                // Ensure APP_STATE is initialized from registry

                let _ = app_state();

                // Start Random Things if enabled
                start_random_things(hwnd);

                return LRESULT(0);
            }

            WM_COMMAND => {
                match loword(wparam.0 as u32) as u32 {
                    100 => {
                        start_deal(hwnd);
                        return LRESULT(0);
                    }
                    101 => {
                        show_scores_dialog(hwnd);
                        return LRESULT(0);
                    }
                    102 => {
                        // Options - with cheat window toggle logic
                        let old_cheat_flag = {
                            let app = app_state().lock().unwrap();
                            app.config.cheat_cards
                        };

                        show_options_dialog(hwnd);

                        // Check if cheat cards flag changed
                        let new_cheat_flag = {
                            let app = app_state().lock().unwrap();
                            app.config.cheat_cards
                        };

                        // Toggle window if flag changed
                        if old_cheat_flag != new_cheat_flag {
                            if new_cheat_flag {
                                // Create cheat window
                                if create_cheat_cards_window(hwnd).is_err() {
                                    MessageBoxW(
                                        hwnd,
                                        PCWSTR(
                                            wide("Could not create cheat cards window!").as_ptr(),
                                        ),
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

                        return LRESULT(0);
                    }
                    103 => {
                        show_random_things_dialog(hwnd);
                        return LRESULT(0);
                    }
                    104 => {
                        let _ = DestroyWindow(hwnd);
                        return LRESULT(0);
                    }
                    _ => {}
                }

                return LRESULT(0);
            }
            WM_LBUTTONDOWN => {
                let (mx, my) = get_xy(lparam);
                handle_click_play(hwnd, mx, my);
                request_redraw(hwnd);
                return LRESULT(0);
            }

            WM_MOVE => {
                // Move cheat window to maintain relative position
                let hwnd_opt = {
                    let app = app_state().lock().unwrap();
                    app.cheat_window.hwnd
                };

                if let Some(hwnd_raw) = hwnd_opt {
                    let cheat_hwnd = HWND(hwnd_raw as *mut _);

                    // Get new main window position
                    let mut parent_rect = RECT::default();
                    if GetWindowRect(hwnd, &mut parent_rect).is_ok() {
                        // Get offset
                        let (offset_x, offset_y) = {
                            let app = app_state().lock().unwrap();
                            (app.cheat_window.offset_x, app.cheat_window.offset_y)
                        };

                        // Calculate new absolute position
                        let new_x = parent_rect.left + offset_x;
                        let new_y = parent_rect.top + offset_y;

                        // Move cheat window
                        let _ = SetWindowPos(
                            cheat_hwnd,
                            None,
                            new_x,
                            new_y,
                            0,
                            0,
                            SWP_NOSIZE | SWP_NOZORDER | SWP_NOACTIVATE,
                        );
                    }
                }
            }

            WM_PAINT => {
                let mut ps = PAINTSTRUCT::default();
                let hdc = BeginPaint(hwnd, &mut ps);

                let mut rc = RECT::default();
                let _ = GetClientRect(hwnd, &mut rc);

                let memdc = CreateCompatibleDC(hdc);
                let bmp = CreateCompatibleBitmap(hdc, rc.right - rc.left, rc.bottom - rc.top);
                let oldbmp = SelectObject(memdc, bmp);

                let white = CreateSolidBrush(COLORREF(0x00FFFFFF));
                FillRect(memdc, &rc, white);

                let dpi = GetDpiForWindow(hwnd) as f32;
                let scale = dpi / 96.0;
                let status_h = (24.0 * scale).round() as i32;

                let hand_h = (CARD_H as f32 * scale).round() as i32;
                let hand_top = (rc.bottom - status_h - hand_h).max(0);
                let hand_rc = RECT {
                    left: 0,
                    top: hand_top,
                    right: rc.right,
                    bottom: rc.bottom - status_h,
                };

                let info_w = (240.0 * scale).round() as i32;
                let info_bottom = hand_rc.top;
                let info_left = (rc.right - info_w).max(0);
                let info_rc = RECT {
                    left: info_left,
                    top: 0,
                    right: rc.right,
                    bottom: info_bottom,
                };

                let table_rc = RECT {
                    left: 0,
                    top: 0,
                    right: info_left,
                    bottom: info_rc.bottom,
                };

                let green = CreateSolidBrush(COLORREF(128 << 8));

                let game_in_progress = app_state().lock().unwrap().game.in_progress;

                if game_in_progress {
                    // Game in progress: Draw all UI elements
                    FillRect(memdc, &table_rc, green);
                    draw_bevel_box(memdc, info_rc);
                    FillRect(memdc, &info_rc, green);
                    draw_info_panel(memdc, &info_rc, scale);
                    FillRect(memdc, &hand_rc, green);

                    let classic = app_state().lock().unwrap().config.classic_layout;
                    if classic {
                        draw_hand_classic(memdc);
                    } else {
                        draw_hand(memdc, &hand_rc, scale);
                    }

                    let pb_rc = RECT {
                        left: (20.0 * scale).round() as i32,
                        top: (20.0 * scale).round() as i32,
                        right: (130.0 * scale).round() as i32,
                        bottom: (47.0 * scale).round() as i32,
                    };
                    draw_bevel_box(memdc, pb_rc);
                    let played_lbl = wide("Played cards:");
                    let _ = TextOutW(
                        memdc,
                        (30.0 * scale).round() as i32,
                        (26.0 * scale).round() as i32,
                        &played_lbl[..played_lbl.len() - 1],
                    );

                    let yh_rc = RECT {
                        left: (20.0 * scale).round() as i32,
                        top: (198.0 * scale).round() as i32,
                        right: (130.0 * scale).round() as i32,
                        bottom: (225.0 * scale).round() as i32,
                    };
                    draw_bevel_box(memdc, yh_rc);
                    let hand_lbl = wide("Your hand:");
                    let _ = TextOutW(
                        memdc,
                        (30.0 * scale).round() as i32,
                        (204.0 * scale).round() as i32,
                        &hand_lbl[..hand_lbl.len() - 1],
                    );

                    let other_rc = RECT {
                        left: (410.0 * scale).round() as i32,
                        top: (160.0 * scale).round() as i32,
                        right: (585.0 * scale).round() as i32,
                        bottom: (225.0 * scale).round() as i32,
                    };
                    draw_bevel_box(memdc, other_rc);
                    draw_extra_info(memdc, &other_rc, scale);

                    if classic {
                        draw_trick_classic(memdc);
                    } else {
                        let trick_rc = RECT {
                            left: 0,
                            top: 0,
                            right: info_left,
                            bottom: info_rc.bottom,
                        };
                        draw_trick(memdc, &trick_rc, scale);
                    }
                } else {
                    // No game: Just green background with IC_LOGO and random things
                    FillRect(memdc, &rc, green);

                    // Draw IC_LOGO at fixed position (matching original at 285, 80)
                    if let Ok(obj) = LoadImageW(
                        GetModuleHandleW(None).unwrap(),
                        PCWSTR(wide("IC_LOGO").as_ptr()),
                        IMAGE_BITMAP,
                        0,
                        0,
                        LR_DEFAULTSIZE | LR_CREATEDIBSECTION,
                    ) {
                        blit_bitmap(
                            memdc,
                            HBITMAP(obj.0),
                            (285.0 * scale).round() as i32,
                            (80.0 * scale).round() as i32,
                            31,
                            31,
                        );
                    }

                    // Draw random things at their current positions
                    let app = app_state().lock().unwrap();
                    for thing in &app.random_things.things {
                        let name = match thing.bitmap_index {
                            0 => "CLUB",
                            1 => "DIAMOND",
                            2 => "SPADE",
                            _ => "HEART",
                        };
                        if let Ok(obj) = LoadImageW(
                            GetModuleHandleW(None).unwrap(),
                            PCWSTR(wide(name).as_ptr()),
                            IMAGE_BITMAP,
                            0,
                            0,
                            LR_DEFAULTSIZE | LR_CREATEDIBSECTION,
                        ) {
                            blit_bitmap(
                                memdc,
                                HBITMAP(obj.0),
                                thing.x,
                                thing.y,
                                THING_SIZE,
                                THING_SIZE,
                            );
                        }
                    }
                }

                let _ = BitBlt(
                    hdc,
                    0,
                    0,
                    rc.right - rc.left,
                    rc.bottom - rc.top,
                    memdc,
                    0,
                    0,
                    SRCCOPY,
                );

                let _ = SelectObject(memdc, oldbmp);
                let _ = DeleteObject(bmp);
                let _ = DeleteDC(memdc);
                let _ = DeleteObject(green);
                let _ = DeleteObject(white);
                let _ = EndPaint(hwnd, &ps);

                return LRESULT(0);
            }

            WM_TIMER => match wparam.0 {
                ID_RNDTIMER => {
                    on_random_timer(hwnd);
                    return LRESULT(0);
                }
                ID_ICNTIMER => {
                    on_icon_timer(hwnd);
                    return LRESULT(0);
                }
                _ => {}
            },

            WM_SIZE => {
                let size_type = wparam.0 as u32;
                match size_type {
                    SIZE_MINIMIZED => {
                        stop_random_things(hwnd);
                        start_icon_twirl(hwnd);
                    }
                    SIZE_RESTORED | SIZE_MAXIMIZED => {
                        stop_icon_twirl(hwnd);
                        start_random_things(hwnd);
                    }
                    _ => {}
                }
            }

            WM_DESTROY => {
                // Cleanup shared resources

                UI_HANDLES.with(|h| {
                    let mut hh = h.borrow_mut();

                    if hh.status_font.0 != core::ptr::null_mut() {
                        let _ = DeleteObject(hh.status_font);
                        hh.status_font = windows::Win32::Graphics::Gdi::HFONT(0 as _);
                    }

                    if hh.hbr_green.0 != core::ptr::null_mut() {
                        let _ = DeleteObject(hh.hbr_green);
                        hh.hbr_green = HBRUSH(0 as _);
                    }

                    if hh.hbr_gray.0 != core::ptr::null_mut() {
                        let _ = DeleteObject(hh.hbr_gray);
                        hh.hbr_gray = HBRUSH(0 as _);
                    }
                });

                if let Some(cache) = CARD_BITMAPS.get() {
                    let mut cache = cache.lock().unwrap();

                    for entry in cache.iter_mut() {
                        if let Some(ptr) = entry.take() {
                            let hbmp = HBITMAP(ptr as *mut _);
                            let _ = DeleteObject(hbmp);
                        }
                    }
                }

                // Clean up cheat cards window
                cleanup_cheat_window();

                let _st = app_state().lock().unwrap();

                // Save window position before destroying
                let mut rect = RECT::default();
                if GetWindowRect(hwnd, &mut rect).is_ok() {
                    // Cast i32 to u32 (preserves bit pattern for negative positions)
                    let _ = registry::set_u32("WindowX", rect.left as u32);
                    let _ = registry::set_u32("WindowY", rect.top as u32);
                }

                PostQuitMessage(0);

                return LRESULT(0);
            }

            _ => {}
        }

        DefWindowProcW(hwnd, msg, wparam, lparam)
    }
}

#[allow(dead_code)]
unsafe fn draw_status(hdc: HDC, rc: &RECT, text: &str) {
    SetBkMode(hdc, TRANSPARENT);

    SetTextColor(hdc, COLORREF(0x000000));

    let mut r = *rc;

    r.left += 8;

    r.top += 4;

    let w = wide(text);

    let sl = &w[..w.len() - 1];

    let _ = TextOutW(hdc, r.left, r.top, sl);
}

unsafe fn draw_info_panel(hdc: HDC, _rc: &RECT, _scale: f32) {
    // Classic absolute layout for info grid and footer lines

    // Draw main info box

    draw_bevel_box(
        hdc,
        RECT {
            left: 340,
            top: 20,
            right: 585,
            bottom: 60 + 15 * (app_state().lock().unwrap().config.num_players as i32),
        },
    );

    // Headings

    SetBkMode(hdc, windows::Win32::Graphics::Gdi::OPAQUE);

    SetTextColor(hdc, COLORREF(0x000000));

    // Match button-face background

    let btnface = COLORREF(0x00F0F0F0);

    windows::Win32::Graphics::Gdi::SetBkColor(hdc, btnface);

    let heads = [(420, "Calls:"), (470, "Tricks:"), (520, "Scores:")];

    for (x, s) in heads {
        let w = wide(s);
        let sl = &w[..w.len() - 1];
        let _ = TextOutW(hdc, x, 30, sl);
    }

    // Player labels and values

    let app = app_state().lock().unwrap();

    for i in 0..app.config.num_players as usize {
        let y = 35 + 15 * ((i + 1) as i32);

        let label = wide(&format!("Player {}:", i + 1));
        let sll = &label[..label.len() - 1];

        let _ = TextOutW(hdc, 350, y, sll);

        let calls = wide(&format!("{}", app.game.calls.get(i).copied().unwrap_or(0)));
        let slc = &calls[..calls.len() - 1];

        let _ = TextOutW(hdc, 430, y, slc);

        let tricks = wide(&format!("{}", app.game.tricks.get(i).copied().unwrap_or(0)));
        let slt = &tricks[..tricks.len() - 1];

        let _ = TextOutW(hdc, 490, y, slt);

        let score = wide(&format!("{}", app.game.scores.get(i).copied().unwrap_or(0)));
        let sls = &score[..score.len() - 1];

        let _ = TextOutW(hdc, 540, y, sls);
    }

    // Footer lines: Round number, Player to start, Last trick winner

    let total_rounds = {
        let max = app.config.max_cards;
        (max * 2).saturating_sub(1)
    };

    let w1 = wide(&format!(
        "Round number: {} of {}",
        app.game.round_no, total_rounds
    ));
    let s1 = &w1[..w1.len() - 1];

    let _ = TextOutW(hdc, 420, 170, s1);

    let w2 = wide(&format!(
        "Player to start: {}",
        app.game.start_player.max(1)
    ));
    let s2 = &w2[..w2.len() - 1];

    let _ = TextOutW(hdc, 420, 185, s2);

    let last = app.game.last_winner.unwrap_or(0);

    let w3 = if last > 0 {
        wide(&format!("Last trick won by: {}", last))
    } else {
        wide("Last trick won by:      ")
    };

    let s3 = &w3[..w3.len() - 1];

    let _ = TextOutW(hdc, 420, 200, s3);

    // Trump suit small icon at (285,80)

    // Trump mapping follows legacy: 1=Clubs,2=Diamonds,3=Spades,4=Hearts

    let name = match app.game.trump {
        1 => Some("CLUB"),
        2 => Some("DIAMOND"),
        3 => Some("SPADE"),
        4 => Some("HEART"),
        _ => None,
    };

    if let Some(id) = name {
        if let Ok(obj) = LoadImageW(
            GetModuleHandleW(None).unwrap(),
            PCWSTR(wide(id).as_ptr()),
            IMAGE_BITMAP,
            0,
            0,
            LR_DEFAULTSIZE | LR_CREATEDIBSECTION,
        ) {
            blit_bitmap(hdc, HBITMAP(obj.0), 285, 80, 31, 31);
        }
    }

    SetBkMode(hdc, TRANSPARENT);
}

unsafe fn draw_extra_info(hdc: HDC, rc: &RECT, _scale: f32) {
    SetBkMode(hdc, windows::Win32::Graphics::Gdi::OPAQUE);
    SetTextColor(hdc, COLORREF(0x000000));
    windows::Win32::Graphics::Gdi::SetBkColor(hdc, COLORREF(0x00F0F0F0));

    let app = app_state().lock().unwrap();

    let mut y = rc.top + 10;
    let x = rc.left + 10;
    let line_height = 15;

    // Calculate total rounds (matching Pascal: NumberofRounds = (NoMaxCards * 2) - 1)
    let total_rounds = (app.config.max_cards * 2).saturating_sub(1);

    // Round number: X of Y
    let round_text = wide(&format!(
        "Round number: {} of {}",
        app.game.round_no, total_rounds
    ));
    let _ = TextOutW(hdc, x, y, &round_text[..round_text.len() - 1]);
    y += line_height;

    // Player to start: X
    let start_text = wide(&format!("Player to start: {}", app.game.start_player));
    let _ = TextOutW(hdc, x, y, &start_text[..start_text.len() - 1]);
    y += line_height;

    // Last trick won by: X
    let winner_str = if let Some(w) = app.game.last_winner {
        format!("{}", w)
    } else {
        "      ".to_string()
    };
    let winner_text = wide(&format!("Last trick won by: {}", winner_str));
    let _ = TextOutW(hdc, x, y, &winner_text[..winner_text.len() - 1]);

    SetBkMode(hdc, TRANSPARENT);
}

// Clear shown trick, advance or score, and set up next actor/round.

// Returns true if the hand ended (and next round started or game ended).

fn finalize_trick_and_setup_next(hwnd: HWND) -> bool {
    // Hold one lock across logic to avoid races

    let mut app = app_state().lock().unwrap();

    // Clear trick and decrement remaining

    for c in app.game.trick.iter_mut() {
        *c = None;
    }

    if app.game.cards_remaining > 0 {
        app.game.cards_remaining -= 1;
    }

    app.game.waiting_for_continue = false;

    let cards_left = app.game.cards_remaining;

    dbglog!("Finalize: cleared trick, cards_remaining={}", cards_left);

    if cards_left == 0 {
        let n = app.config.num_players as usize;

        let deltas = score_hand(
            app.config.score_mode,
            app.config.hard_score,
            &app.game.calls,
            &app.game.tricks,
            app.game.dealt_cards,
        );

        for (i, &delta) in deltas.iter().enumerate().take(n) {
            app.game.scores[i] = app.game.scores[i].saturating_add(delta);
        }

        let max = app.config.max_cards;
        let total_rounds = (max * 2).saturating_sub(1);

        let final_round = app.game.round_no >= total_rounds;

        let best_score = *app.game.scores.iter().max().unwrap_or(&0);

        let human_best = app.game.scores.first().copied().unwrap_or(0);

        dbglog!(
            "End hand (finalize): round={} final_round={} scores={:?}",
            app.game.round_no,
            final_round,
            app.game.scores
        );

        if final_round {
            app.game.in_progress = false;
        }

        drop(app);

        // Restart random things when game ends
        if final_round {
            unsafe {
                start_random_things(hwnd);
            }
        }

        set_status("");

        if final_round {
            if human_best >= best_score {
                maybe_update_high_scores(hwnd, human_best);
            }

            request_redraw(hwnd);

            true
        } else {
            // Start next round automatically

            start_deal(hwnd);

            true
        }
    } else {
        // Prepare next trick

        // Set start_player to last winner for the next trick (legacy parity)

        if let Some(w) = app.game.last_winner {
            app.game.start_player = w;
        }

        if let Some(nextp) = next_player_to_act(app.game.start_player, &app.game.trick) {
            app.game.current_player = nextp;

            app.game.waiting_for_human = nextp == 0;

            dbglog!(
                "Finalize: next trick start={} cur={} waiting={}",
                app.game.start_player,
                app.game.current_player,
                app.game.waiting_for_human
            );
        }

        drop(app);

        // Update cheat cards window after trick cleared
        unsafe {
            update_cheat_cards_window();
        }

        set_status("");

        request_redraw(hwnd);

        false
    }
}

// Classic hand drawing (absolute 96-DPI coordinates; overlap and invert for illegal)

unsafe fn draw_hand_classic(hdc: HDC) {
    let app_snapshot = app_state().lock().unwrap();

    let n = app_snapshot.game.hand.len();

    if n == 0 {
        return;
    }

    let act_width = if n > 1 {
        let mut w = (HAND_SPAN_X - CARD_W) / ((n - 1) as i32);

        if w > (CARD_W + 10) {
            w = CARD_W + 10;
        }

        w
    } else {
        CARD_W
    };

    if n > 1 && act_width <= MIN_WIDTH {
        drop(app_snapshot);

        let _ = MessageBoxW(
            HWND(0 as _),
            PCWSTR(wide("Window too small to draw cards").as_ptr()),
            PCWSTR(wide("Estimation Whist").as_ptr()),
            MB_ICONINFORMATION,
        );

        return;
    }

    // Prepare text mode for any labels if needed

    SetBkMode(hdc, TRANSPARENT);

    SetTextColor(hdc, COLORREF(0x000000));

    let mut positions: Vec<RECT> = Vec::with_capacity(n);

    let mut x = HAND_X0;

    let y = HAND_Y;

    let mut last_idx: Option<usize> = None;

    for i in 0..n {
        let r = RECT {
            left: x,
            top: y,
            right: x + CARD_W,
            bottom: y + CARD_H,
        };

        // Overlap truncation for previous rect when spacing < full width

        if let Some(pi) = last_idx {
            if act_width < CARD_W {
                if let Some(prev) = positions.get_mut(pi) {
                    prev.right = prev.left + act_width;
                }
            }
        }

        // Draw card bitmap with invert if illegal

        let card_id = app_snapshot.game.hand[i];

        let legal = is_legal_play(card_id, &app_snapshot.game.trick, &app_snapshot.game.hand);

        if let Some(hbmp) = get_card_bitmap(card_id) {
            blit_card(hdc, hbmp, r.left, r.top, !legal);
        } else {
            // Fallback: simple frame and id

            let _ = GdiRectangle(hdc, r.left, r.top, r.right, r.bottom);

            let label = wide(&format!("{}", card_id));

            let sl = &label[..label.len() - 1];

            let _ = TextOutW(hdc, r.left + 6, r.top + 6, sl);
        }

        positions.push(r);

        last_idx = Some(i);

        x += if n > 1 { act_width } else { CARD_W };
    }

    drop(app_snapshot);

    let mut app = app_state().lock().unwrap();

    app.game.hand_positions = positions;
}

// Classic trick row drawing: absolute positions and region clear

unsafe fn draw_trick_classic(hdc: HDC) {
    // Clear trick area using classic rectangle

    let n_players = app_state().lock().unwrap().config.num_players as i32;

    let area = RECT {
        left: 39,
        top: 54,
        right: 41 + CARD_W + ((n_players - 1) * 30),
        bottom: 80 + CARD_H,
    };

    UI_HANDLES.with(|h| {
        let br = h.borrow().hbr_green;
        if br.0 != core::ptr::null_mut() {
            let _ = FillRect(hdc, &area, br);
        }
    });

    SetTextColor(hdc, COLORREF(0x000000));

    let app = app_state().lock().unwrap();

    let n = app.config.num_players as usize;

    if n == 0 {
        return;
    }

    let start = app.game.start_player.max(1) as usize; // 1-based in state

    let base = start - 1; // 0-based

    for a in 1..=n {
        let p = (base + (a - 1)) % n;

        if let Some(Some(card_id)) = app.game.trick.get(p) {
            if let Some(hbmp) = get_card_bitmap(*card_id) {
                let x = TRICK_X0 + (TRICK_STEP * (a as i32));

                let y = TRICK_Y;

                blit_card(hdc, hbmp, x, y, false);
            } else {
                let x = TRICK_X0 + (TRICK_STEP * (a as i32));

                let y = TRICK_Y;

                dbglog!("Failed to load trick card {}", card_id);

                let _ = GdiRectangle(hdc, x, y, x + CARD_W, y + CARD_H);

                let label = wide(&format!("{}", card_id));

                let sl = &label[..label.len() - 1];

                let _ = TextOutW(hdc, x + 6, y + 6, sl);
            }
        }

        // Numeric label beneath on green background

        let lx = 20 + (30 * (a as i32));

        let ly = 65 + CARD_H; // under the row

        let w = wide(&format!("{}", p + 1));

        let sl = &w[..w.len() - 1];

        windows::Win32::Graphics::Gdi::SetBkColor(hdc, COLORREF(128 << 8));

        SetBkMode(hdc, windows::Win32::Graphics::Gdi::OPAQUE);

        let _ = TextOutW(hdc, lx, ly, sl);
    }

    SetBkMode(hdc, TRANSPARENT);
}

// Classic exact-size blit with optional invert ROP

unsafe fn blit_card(hdc: HDC, hbmp: HBITMAP, x: i32, y: i32, invert: bool) {
    let mem = CreateCDC(hdc);

    let old = SelectObj(mem, hbmp);

    let mut bm: BITMAP = std::mem::zeroed();

    let _ = GetObjectW(
        hbmp,
        std::mem::size_of::<BITMAP>() as i32,
        Some(&mut bm as *mut _ as *mut _),
    );

    let sw = bm.bmWidth;
    let sh = bm.bmHeight;

    let rop = if invert { NOTSRCCOPY } else { SRCCOPY };

    let _ = BitBlt(hdc, x, y, sw, sh, mem, 0, 0, rop);

    let _ = SelectObj(mem, old);

    let _ = DeleteDc(mem);
}

unsafe fn draw_hand(hdc: HDC, rc: &RECT, scale: f32) {
    let card_w = (71.0 * scale) as i32;

    let card_h = (96.0 * scale) as i32;

    let min_overlap = (20.0 * scale) as i32;

    let left_pad = (10.0 * scale) as i32;

    let top_pad = (5.0 * scale) as i32;

    let app_snapshot = app_state().lock().unwrap();

    let n = app_snapshot.game.hand.len();

    if n == 0 {
        return;
    }

    let width = rc.right - rc.left;

    let hand_w = width - 2 * left_pad;

    let act_width = if n > 1 {
        let span = (hand_w - card_w).max(0);

        let w = span / ((n - 1) as i32);

        w.clamp(min_overlap, card_w + (10.0 * scale) as i32)
    } else {
        card_w
    };

    // Prepare brushes

    let white = CreateSolidBrush(COLORREF(0x00FFFFFF));

    let sel_brush = CreateSolidBrush(COLORREF(0x00FFF0AA));

    SetBkMode(hdc, TRANSPARENT);

    SetTextColor(hdc, COLORREF(0x000000));

    let mut positions: Vec<RECT> = Vec::with_capacity(n);

    let mut x = rc.left + left_pad;

    let y = rc.top + top_pad;

    for i in 0..n {
        let r = RECT {
            left: x,
            top: y,
            right: x + card_w,
            bottom: y + card_h,
        };

        // Fill background (highlight selection)

        let sel = app_snapshot.game.selected == Some(i);

        let brush = if sel { sel_brush } else { white };

        FillRect(hdc, &r, brush);

        // Try to draw bitmap if loaded

        let card_id = app_snapshot.game.hand[i];

        if let Some(hbmp) = get_card_bitmap(card_id) {
            blit_bitmap(hdc, hbmp, r.left, r.top, card_w, card_h);
        } else {
            // Border and label fallback

            let _ = GdiRectangle(hdc, r.left, r.top, r.right, r.bottom);

            let label = wide(&format!("{}", card_id));

            let sl = &label[..label.len() - 1];

            let tx = r.left + (8.0 * scale) as i32;

            let ty = r.top + (8.0 * scale) as i32;

            let _ = TextOutW(hdc, tx, ty, sl);
        }

        positions.push(r);

        x += act_width;
    }

    drop(app_snapshot);

    // Store positions

    let mut app = app_state().lock().unwrap();

    app.game.hand_positions = positions;
}

unsafe fn draw_trick(hdc: HDC, rc: &RECT, scale: f32) {
    let card_w = (71.0 * scale) as i32;

    let card_h = (96.0 * scale) as i32;

    let left_pad = (10.0 * scale) as i32;

    let top_pad = (10.0 * scale) as i32;

    let app = app_state().lock().unwrap();

    let n = app.config.num_players as usize;

    let start = app.game.start_player as usize;

    // Draw in player order starting from start_player

    let base = if start == 0 { 0 } else { start - 1 };

    for idx in 0..n {
        let p = ((base + idx) % n) as usize;

        if let Some(Some(card_id)) = app.game.trick.get(p) {
            let x = rc.left + left_pad + (idx as i32) * (card_w + (8.0 * scale) as i32);

            let y = rc.top + top_pad;

            if let Some(hbmp) = get_card_bitmap(*card_id) {
                blit_bitmap(hdc, hbmp, x, y, card_w, card_h);
            } else {
                let white = CreateSolidBrush(COLORREF(0x00FFFFFF));

                let r = RECT {
                    left: x,
                    top: y,
                    right: x + card_w,
                    bottom: y + card_h,
                };

                FillRect(hdc, &r, white);

                let _ = GdiRectangle(hdc, r.left, r.top, r.right, r.bottom);

                let label = wide(&format!("{}", card_id));

                let sl = &label[..label.len() - 1];

                let tx = r.left + (8.0 * scale) as i32;

                let ty = r.top + (8.0 * scale) as i32;

                let _ = TextOutW(hdc, tx, ty, sl);

                let _ = DeleteObject(white);
            }

            // Player label below the card

            let pl = wide(&format!(
                "P{}{}",
                p + 1,
                if app.game.current_player == p {
                    " <"
                } else {
                    ""
                }
            ));

            let pls = &pl[..pl.len() - 1];

            let ly = y + card_h + ((6.0 * scale) as i32);

            let _ = TextOutW(hdc, x, ly, pls);
        }
    }
}

// Draw a classic filled 3D box roughly matching Pascal DrawBox

unsafe fn draw_bevel_box(hdc: HDC, rc: RECT) {
    let black = CreateSolidBrush(COLORREF(0x000000));

    let ltgray = CreateSolidBrush(COLORREF(0x00F0F0F0)); // Match button-face grey

    let dark = CreateSolidBrush(COLORREF(0x00A0A0A0));

    let white = CreateSolidBrush(COLORREF(0x00FFFFFF));

    // Fill interior first

    let mut inner = rc;
    inner.left += 1;
    inner.top += 1;
    inner.right -= 1;
    inner.bottom -= 1;

    if inner.right > inner.left && inner.bottom > inner.top {
        let _ = FillRect(hdc, &inner, ltgray);
    }

    // Outer black frame

    let _ = FrameRect(hdc, &rc, black);

    // Inner dark/light frames for simple 3D effect

    inner.left += 1;
    inner.top += 1;
    inner.right -= 1;
    inner.bottom -= 1;

    if inner.right > inner.left && inner.bottom > inner.top {
        let _ = FrameRect(hdc, &inner, dark);
    }

    inner.left += 1;
    inner.top += 1;
    inner.right -= 1;
    inner.bottom -= 1;

    if inner.right > inner.left && inner.bottom > inner.top {
        let _ = FrameRect(hdc, &inner, white);
    }

    let _ = DeleteObject(black);
    let _ = DeleteObject(ltgray);
    let _ = DeleteObject(dark);
    let _ = DeleteObject(white);
}

fn get_xy(lparam: LPARAM) -> (i32, i32) {
    let lp = lparam.0 as u32;

    let x = (lp & 0xFFFF) as i16 as i32;

    let y = ((lp >> 16) & 0xFFFF) as i16 as i32;

    (x, y)
}

fn create_default_menu(hwnd: HWND) {
    unsafe {
        let hmenu = CreateMenu().unwrap();

        let game = CreatePopupMenu().unwrap();

        let helpm = CreatePopupMenu().unwrap();

        // Game menu

        let _ = AppendMenuW(game, MF_STRING, 100, PCWSTR(wide("&Deal").as_ptr())).unwrap();

        let _ = AppendMenuW(game, MF_STRING, 101, PCWSTR(wide("S&cores").as_ptr())).unwrap();

        let _ = AppendMenuW(game, MF_STRING, 102, PCWSTR(wide("&Options...").as_ptr())).unwrap();

        let _ = AppendMenuW(
            game,
            MF_STRING,
            103,
            PCWSTR(wide("&Random things...").as_ptr()),
        )
        .unwrap();

        let _ = AppendMenuW(game, MF_SEPARATOR, 0, PCWSTR(std::ptr::null())).unwrap();

        let _ = AppendMenuW(game, MF_STRING, 104, PCWSTR(wide("E&xit").as_ptr())).unwrap();

        // Help menu

        let _ = AppendMenuW(helpm, MF_STRING, 900, PCWSTR(wide("&Contents").as_ptr())).unwrap();

        let _ = AppendMenuW(
            helpm,
            MF_STRING,
            901,
            PCWSTR(wide("Help on &Help").as_ptr()),
        )
        .unwrap();

        let _ = AppendMenuW(helpm, MF_SEPARATOR, 0, PCWSTR(std::ptr::null())).unwrap();

        let _ = AppendMenuW(
            helpm,
            MF_STRING,
            999,
            PCWSTR(wide("&About Estimation Whist").as_ptr()),
        )
        .unwrap();

        // Top-level

        let _ = AppendMenuW(
            hmenu,
            MF_STRING | MF_POPUP,
            game.0 as usize,
            PCWSTR(wide("&Game").as_ptr()),
        )
        .unwrap();

        let _ = AppendMenuW(
            hmenu,
            MF_STRING | MF_POPUP,
            helpm.0 as usize,
            PCWSTR(wide("&Help").as_ptr()),
        )
        .unwrap();

        let _ = SetMenu(hwnd, hmenu).unwrap();

        let _ = DrawMenuBar(hwnd);
    }
}

fn advance_ai_until_human_or_trick_end(hwnd: HWND) {
    use rand::seq::SliceRandom;

    loop {
        let mut app = app_state().lock().unwrap();

        dbglog!(
            "Loop top: start={} cur={} waiting={} trick={:?}",
            app.game.start_player,
            app.game.current_player,
            app.game.waiting_for_human,
            app.game.trick
        );

        let n = app.config.num_players as usize;

        if n == 0 || app.game.trick.len() != n {
            break;
        }

        if app.game.waiting_for_human {
            break;
        }

        // Determine whose turn it is based on start seat and trick state

        if let Some(cur) = next_player_to_act(app.game.start_player, &app.game.trick) {
            dbglog!(
                "AI loop: next seat {} start={} trick={:?}",
                cur,
                app.game.start_player,
                app.game.trick
            );

            if cur == 0 {
                app.game.current_player = cur;

                app.game.waiting_for_human = true;

                drop(app);

                break;
            }

            // AI plays a legal random card at seat cur

            let lead_suit = app
                .game
                .trick
                .iter()
                .flatten()
                .next()
                .map(|&c| suit_index_from_legacy_id(c));

            let hand = app.game.hands[cur].clone();

            if hand.is_empty() {
                app.game.current_player = cur;
                drop(app);
                continue;
            }

            let mut legal_idx: Vec<usize> = Vec::new();

            if let Some(lead) = lead_suit {
                let has_lead = hand.iter().any(|&c| suit_index_from_legacy_id(c) == lead);

                if has_lead {
                    for (i, &c) in hand.iter().enumerate() {
                        if suit_index_from_legacy_id(c) == lead {
                            legal_idx.push(i);
                        }
                    }
                }
            }

            if legal_idx.is_empty() {
                legal_idx = (0..hand.len()).collect();
            }

            let mut rng = rand::thread_rng();

            if let Some(&pick) = legal_idx.choose(&mut rng) {
                let card = hand[pick];

                let real = &mut app.game.hands[cur];

                real.remove(pick);

                app.game.trick[cur] = Some(card);

                dbglog!("AI seat {} played card {}", cur, card);
            }

            // Update current player to next to act

            if let Some(nextp) = next_player_to_act(app.game.start_player, &app.game.trick) {
                app.game.current_player = nextp;
            }

            drop(app);

            // Update cheat cards window after AI plays
            unsafe {
                update_cheat_cards_window();
            }
        } else {
            // Trick full

            dbglog!("AI loop: trick full");

            drop(app);
        }

        // Check if trick complete

        {
            let app2 = app_state().lock().unwrap();

            if app2.game.trick.iter().filter(|c| c.is_some()).count()
                == (app2.config.num_players as usize)
            {
                drop(app2);

                dbglog!("Trick complete -> deciding winner");

                decide_winner_and_setup();

                // Pause per option: Dialog shows MB, Mouse waits for click

                let (notify, winner) = {
                    let a = app_state().lock().unwrap();
                    (a.config.next_notify, a.game.last_winner.unwrap_or(0))
                };

                match notify {
                    NextNotify::Dialog => unsafe {
                        let msg = if winner > 0 {
                            format!("Player {} won that trick.", winner)
                        } else {
                            "Trick complete.".to_string()
                        };

                        let w = wide(&msg);

                        let cap = wide("Estimation Whist");

                        let _ = MessageBoxW(
                            hwnd,
                            PCWSTR(w.as_ptr()),
                            PCWSTR(cap.as_ptr()),
                            MB_ICONINFORMATION,
                        );

                        // Now finalize and continue immediately

                        if finalize_trick_and_setup_next(hwnd) {
                            return;
                        }

                        continue;
                    },

                    NextNotify::Mouse => {
                        let mut a = app_state().lock().unwrap();

                        a.game.waiting_for_continue = true;

                        drop(a);

                        set_status("Click to continue");

                        break;
                    }
                }
            }
        }
    }

    request_redraw(hwnd);
}

fn decide_winner_and_setup() {
    let mut app = app_state().lock().unwrap();

    let n = app.config.num_players as usize;

    if app.game.trick.iter().filter(|c| c.is_some()).count() != n {
        return;
    }

    let trump = app.game.trump;

    if let Some(winner) = decide_trick_winner(&app.game.trick, trump) {
        app.game.tricks[winner] += 1;

        app.game.last_winner = Some((winner + 1) as u32);

        app.game.current_player = winner;

        // Do not clear or decrement here; we pause and finalize later

        dbglog!(
            "Trick decided: winner={} start_next={} cards_remaining={} tricks={:?}",
            winner,
            app.game.last_winner.unwrap_or(0),
            app.game.cards_remaining,
            app.game.tricks
        );
    }
}

fn handle_click_play(hwnd: HWND, mx: i32, my: i32) {
    {
        let mut app = app_state().lock().unwrap();

        if app.game.waiting_for_continue {
            app.game.waiting_for_continue = false;

            drop(app);

            let ended = finalize_trick_and_setup_next(hwnd);

            if !ended {
                advance_ai_until_human_or_trick_end(hwnd);
            }

            return;
        }

        let mut clicked: Option<usize> = None;

        for (i, r) in app.game.hand_positions.iter().enumerate() {
            if mx >= r.left && mx < r.right && my >= r.top && my < r.bottom {
                clicked = Some(i);
                break;
            }
        }

        let waiting = app.game.waiting_for_human;

        let cur = app.game.current_player;

        if let Some(i) = clicked {
            app.game.selected = Some(i);

            if waiting && cur == 0 {
                if let Some(card_id) = app.game.hand.get(i).copied() {
                    let legal = is_legal_play(card_id, &app.game.trick, &app.game.hand);

                    dbglog!(
                        "Human click: idx={} card={} legal={} start={} trick={:?}",
                        i,
                        card_id,
                        legal,
                        app.game.start_player,
                        app.game.trick
                    );

                    if legal {
                        let card = app.game.hand.remove(i);

                        app.game.trick[0] = Some(card);

                        if let Some(pos) = app.game.hands[0].iter().position(|&x| x == card) {
                            app.game.hands[0].remove(pos);
                        }

                        // Advance to the next seat to act

                        if let Some(nextp) =
                            next_player_to_act(app.game.start_player, &app.game.trick)
                        {
                            app.game.current_player = nextp;

                            app.game.waiting_for_human = nextp == 0;

                            dbglog!(
                                "After human: next={} waiting={} trick={:?}",
                                nextp,
                                app.game.waiting_for_human,
                                app.game.trick
                            );
                        } else {
                            app.game.waiting_for_human = false;
                        }
                    }
                }
            }
        }
    }

    advance_ai_until_human_or_trick_end(hwnd);
}

unsafe fn blit_bitmap(hdc: HDC, hbmp: HBITMAP, x: i32, y: i32, w: i32, h: i32) {
    let mem = CreateCDC(hdc);

    let old = SelectObj(mem, hbmp);

    // Determine source dimensions

    let mut bm: BITMAP = std::mem::zeroed();

    let _ = GetObjectW(
        hbmp,
        std::mem::size_of::<BITMAP>() as i32,
        Some(&mut bm as *mut _ as *mut _),
    );

    let sw = bm.bmWidth;
    let sh = bm.bmHeight;

    let oldmode = SetStretchBltMode(hdc, HALFTONE);

    let _ = StretchBlt(hdc, x, y, w, h, mem, 0, 0, sw, sh, SRCCOPY);

    let _ = SetStretchBltMode(
        hdc,
        windows::Win32::Graphics::Gdi::STRETCH_BLT_MODE(oldmode),
    );

    let _ = SelectObj(mem, old);

    let _ = DeleteDc(mem);
}

// ----- Dialogs (stubs) -----

const IDD_ABOUT: u16 = 3001;

const IDD_OPTIONS: u16 = 3002;

const IDD_SCORES: u16 = 3003;

const IDD_CALL: u16 = 3004;

#[allow(dead_code)]
const IDC_LIST_SCORES: u16 = 4010;

const IDC_CALL_BASE: u16 = 400; // 0..15 (ID_CALLZER through ID_CALLFIF in original)

unsafe fn show_about_dialog(parent: HWND) {
    let hinst = GetModuleHandleW(None).unwrap();

    DialogBoxParamW(
        hinst,
        make_int_resource(IDD_ABOUT),
        parent,
        Some(about_dlg_proc),
        LPARAM(0),
    );
}

unsafe fn show_options_dialog(parent: HWND) {
    let hinst = GetModuleHandleW(None).unwrap();

    DialogBoxParamW(
        hinst,
        make_int_resource(IDD_OPTIONS),
        parent,
        Some(options_dlg_proc),
        LPARAM(0),
    );
}

unsafe fn show_scores_dialog(parent: HWND) {
    let hinst = GetModuleHandleW(None).unwrap();

    DialogBoxParamW(
        hinst,
        make_int_resource(IDD_SCORES),
        parent,
        Some(scores_dlg_proc),
        LPARAM(0),
    );
}

// Cheat Cards Window Functions

unsafe fn register_cheat_window_class(hinstance: HMODULE) -> windows::core::Result<()> {
    let class_name = wide(CHEAT_WINDOW_CLASS);

    let wc = WNDCLASSW {
        style: CS_HREDRAW | CS_VREDRAW,
        lpfnWndProc: Some(cheat_window_proc),
        cbClsExtra: 0,
        cbWndExtra: 0,
        hInstance: hinstance.into(),
        hIcon: HICON::default(),
        hCursor: LoadCursorW(None, IDC_ARROW).unwrap(),
        hbrBackground: HBRUSH(std::ptr::null_mut()),
        lpszMenuName: PCWSTR::null(),
        lpszClassName: PCWSTR(class_name.as_ptr()),
    };

    let atom = RegisterClassW(&wc);
    if atom == 0 {
        return Err(windows::core::Error::from_win32());
    }

    Ok(())
}

unsafe fn create_cheat_cards_window(parent_hwnd: HWND) -> windows::core::Result<HWND> {
    // Get main window position
    let mut parent_rect = RECT::default();
    GetWindowRect(parent_hwnd, &mut parent_rect)?;

    // Get offset from app state
    let (offset_x, offset_y) = {
        let app = app_state().lock().unwrap();
        (app.cheat_window.offset_x, app.cheat_window.offset_y)
    };

    // Calculate absolute position: main window + offset
    let x = parent_rect.left + offset_x;
    let y = parent_rect.top + offset_y;

    let dpi = GetDpiForWindow(parent_hwnd) as f32;
    let scale = dpi / 96.0;

    let width = (CHEAT_WINDOW_WIDTH_BASE * scale).round() as i32;
    let height = (CHEAT_WINDOW_HEIGHT_BASE * scale).round() as i32;

    let hwnd = CreateWindowExW(
        WINDOW_EX_STYLE::default(),
        PCWSTR(wide(CHEAT_WINDOW_CLASS).as_ptr()),
        PCWSTR(wide("Cheat Information - Cards").as_ptr()),
        WS_POPUP | WS_CAPTION | WS_SYSMENU | WS_VISIBLE,
        x,
        y,
        width,
        height,
        parent_hwnd,
        None,
        GetModuleHandleW(None)?,
        None,
    )?;

    app_state().lock().unwrap().cheat_window.hwnd = Some(hwnd.0 as isize);
    Ok(hwnd)
}

unsafe fn close_cheat_cards_window() {
    let hwnd_opt = {
        let app = app_state().lock().unwrap();
        app.cheat_window.hwnd
    };

    if let Some(hwnd_raw) = hwnd_opt {
        let hwnd = HWND(hwnd_raw as *mut _);
        let _ = DestroyWindow(hwnd);
    }
}

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
            // Calculate and store offset relative to parent window
            if let Ok(parent) = GetParent(hwnd) {
                if !parent.is_invalid() {
                    let mut parent_rect = RECT::default();
                    let mut cheat_rect = RECT::default();
                    if GetWindowRect(parent, &mut parent_rect).is_ok()
                        && GetWindowRect(hwnd, &mut cheat_rect).is_ok()
                    {
                        let mut app = app_state().lock().unwrap();
                        app.cheat_window.offset_x = cheat_rect.left - parent_rect.left;
                        app.cheat_window.offset_y = cheat_rect.top - parent_rect.top;
                    }
                }
            }
        }

        WM_CLOSE => {
            let _ = DestroyWindow(hwnd);
            return LRESULT(0);
        }

        WM_DESTROY => {
            // Save final offset relative to parent window
            if let Ok(parent) = GetParent(hwnd) {
                if !parent.is_invalid() {
                    let mut parent_rect = RECT::default();
                    let mut cheat_rect = RECT::default();
                    if GetWindowRect(parent, &mut parent_rect).is_ok()
                        && GetWindowRect(hwnd, &mut cheat_rect).is_ok()
                    {
                        let mut app = app_state().lock().unwrap();
                        app.cheat_window.offset_x = cheat_rect.left - parent_rect.left;
                        app.cheat_window.offset_y = cheat_rect.top - parent_rect.top;
                    }
                }
            }

            let mut app = app_state().lock().unwrap();
            app.config.cheat_cards = false;
            app.cheat_window.hwnd = None;

            return LRESULT(0);
        }

        _ => {}
    }

    DefWindowProcW(hwnd, msg, wparam, lparam)
}

unsafe fn draw_cheat_cards(hwnd: HWND) {
    let mut ps = PAINTSTRUCT::default();
    let hdc = BeginPaint(hwnd, &mut ps);

    let (hands, num_players, dpi) = {
        let app = app_state().lock().unwrap();
        (
            app.game.hands.clone(),
            app.game.hands.len(),
            GetDpiForWindow(hwnd) as f32,
        )
    };

    let scale = dpi / 96.0;

    let mut rect = RECT::default();
    let _ = GetClientRect(hwnd, &mut rect);

    let green_brush = CreateSolidBrush(COLORREF(128 << 8));
    FillRect(hdc, &rect, green_brush);
    let _ = DeleteObject(green_brush);

    if num_players < 2 {
        let _ = EndPaint(hwnd, &ps);
        return;
    }

    let mut y_increment = (57.0 * scale).round() as i32;
    if num_players > 2 {
        let available_height = rect.bottom - (81.0 * scale).round() as i32;
        let calculated_increment = available_height / (num_players - 2) as i32;
        if calculated_increment < y_increment {
            y_increment = calculated_increment;
        }
    }

    let no_cards = if hands.len() > 1 {
        hands[1].iter().filter(|&&c| c > 0).count()
    } else {
        0
    };

    if no_cards == 0 {
        let _ = EndPaint(hwnd, &ps);
        return;
    }

    let small_card_width = (SMALL_CARD_WIDTH_BASE * scale).round() as i32;
    let small_card_height = (SMALL_CARD_HEIGHT_BASE * scale).round() as i32;
    let small_min_width = (SMALL_MIN_WIDTH_BASE * scale).round() as i32;

    let act_width = if no_cards > 1 {
        let available_width = (360.0 * scale).round() as i32 - small_card_width;
        let calculated = available_width / (no_cards - 1) as i32;

        if calculated > small_min_width {
            calculated.min(small_card_width + (10.0 * scale).round() as i32)
        } else {
            small_min_width
        }
    } else {
        small_card_width
    };

    SetBkMode(hdc, TRANSPARENT);
    SetTextColor(hdc, COLORREF(GetSysColor(COLOR_WINDOWTEXT)));

    #[allow(clippy::needless_range_loop)]
    for player_idx in 1..num_players {
        let player_number = player_idx + 1;
        let row_index = player_idx - 1;

        let text_y = (14.0 * scale).round() as i32 + y_increment * row_index as i32;
        let card_y = (4.0 * scale).round() as i32 + y_increment * row_index as i32;

        let player_text = player_number.to_string();
        let player_wide = wide(&player_text);
        let _ = TextOutW(
            hdc,
            (10.0 * scale).round() as i32,
            text_y,
            &player_wide[..player_wide.len() - 1],
        );

        let mut card_index = 0;
        for &card_id in &hands[player_idx] {
            if card_id > 0 {
                let card_x = (30.0 * scale).round() as i32 + card_index * act_width;

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

    let _ = EndPaint(hwnd, &ps);
}

unsafe fn draw_card_scaled(
    hdc: HDC,
    x: i32,
    y: i32,
    card_id: u32,
    dest_width: i32,
    dest_height: i32,
) {
    let hbmp = match get_card_bitmap(card_id) {
        Some(bmp) => bmp,
        None => return,
    };

    let memdc = CreateCompatibleDC(hdc);
    let old_bmp = SelectObject(memdc, hbmp);

    let _ = StretchBlt(
        hdc,
        x,
        y,
        dest_width,
        dest_height,
        memdc,
        0,
        0,
        CARD_W,
        CARD_H,
        SRCCOPY,
    );

    SelectObject(memdc, old_bmp);
    let _ = DeleteDC(memdc);
}

unsafe fn update_cheat_cards_window() {
    let hwnd_opt = {
        let app = app_state().lock().unwrap();
        app.cheat_window.hwnd
    };

    if let Some(hwnd_raw) = hwnd_opt {
        let hwnd = HWND(hwnd_raw as *mut _);
        let _ = InvalidateRect(hwnd, None, BOOL(1));
    }
}

unsafe fn cleanup_cheat_window() {
    let hwnd_opt = {
        let app = app_state().lock().unwrap();
        app.cheat_window.hwnd
    };

    if let Some(hwnd_raw) = hwnd_opt {
        let hwnd = HWND(hwnd_raw as *mut _);

        // Calculate and save offset before destroying
        if let Ok(parent) = GetParent(hwnd) {
            if !parent.is_invalid() {
                let mut parent_rect = RECT::default();
                let mut cheat_rect = RECT::default();
                if GetWindowRect(parent, &mut parent_rect).is_ok()
                    && GetWindowRect(hwnd, &mut cheat_rect).is_ok()
                {
                    let mut app = app_state().lock().unwrap();
                    app.cheat_window.offset_x = cheat_rect.left - parent_rect.left;
                    app.cheat_window.offset_y = cheat_rect.top - parent_rect.top;
                }
            }
        }

        let _ = DestroyWindow(hwnd);
    }

    {
        let app = app_state().lock().unwrap();
        save_config_to_registry(&app.config);
        save_cheat_window_state(&app.cheat_window);
    }
}

unsafe fn show_call_dialog(parent: HWND) -> i32 {
    let hinst = GetModuleHandleW(None).unwrap();

    let ret = DialogBoxParamW(
        hinst,
        make_int_resource(IDD_CALL),
        parent,
        Some(call_dlg_proc),
        LPARAM(0),
    );

    ret as i32 // returns selection (0..15)
}

// Run bidding phase for all players with last-bidder constraint and simple AI

fn run_bidding(hwnd: HWND) {
    unsafe {
        let n = app_state().lock().unwrap().config.num_players as usize;

        let no_cards = app_state().lock().unwrap().game.dealt_cards as u32;

        // Bidding order starts at StartPlayer and wraps; human is seat 0

        let sp1 = app_state().lock().unwrap().game.start_player as usize; // 1-based

        let start = if sp1 == 0 { 0 } else { sp1 - 1 };

        let mut sum_so_far: u32 = 0;

        dbglog!(
            "Bidding: start={} dealt={} players={}",
            start + 1,
            no_cards,
            n
        );

        for turn in 0..n {
            let seat = (start + turn) % n;

            let is_last = turn == n - 1;

            if seat == 0 {
                // Human call

                let mut app = app_state().lock().unwrap();

                app.game.bidding_forbidden = if is_last {
                    Some(no_cards.saturating_sub(sum_so_far))
                } else {
                    None
                };

                drop(app);

                let sel = show_call_dialog(hwnd);

                let mut app2 = app_state().lock().unwrap();

                let mut call = if sel < 0 { 0 } else { sel as u32 };

                if call > no_cards {
                    call = no_cards;
                }

                if is_last {
                    let forbidden = no_cards.saturating_sub(sum_so_far);

                    if call == forbidden {
                        call = if call == 0 { 1 } else { call - 1 };
                    }
                }

                app2.game.calls[seat] = call;

                app2.game.bidding_forbidden = None;

                sum_so_far += call;

                dbglog!(
                    "Call: seat={} (human) call={} sum_so_far={} last={}",
                    seat,
                    call,
                    sum_so_far,
                    is_last
                );
            } else {
                // Simple AI call

                let app = app_state().lock().unwrap();

                let hand = app.game.hands[seat].clone();

                let trump = app.game.trump;

                drop(app);

                let mut est: f32 = 0.0;

                for id in hand {
                    let r = ((id - 1) % 13) + 1; // 1..13, Ace=1

                    est += match r {
                        1 => 1.0,
                        13 => 0.8,
                        12 => 0.6,
                        11 => 0.5,
                        10 => 0.4,
                        _ => 0.0,
                    };

                    if suit_index_from_legacy_id(id) == trump {
                        est += 0.2;
                    }
                }

                let mut call = est.round() as u32;

                if call > no_cards {
                    call = no_cards;
                }

                if is_last {
                    let forbidden = no_cards.saturating_sub(sum_so_far);

                    if call == forbidden {
                        call = if call == 0 { 1 } else { call - 1 };
                    }
                }

                let mut app2 = app_state().lock().unwrap();

                app2.game.calls[seat] = call;

                sum_so_far += call;

                dbglog!(
                    "Call: seat={} (AI) call={} sum_so_far={} last={} ",
                    seat,
                    call,
                    sum_so_far,
                    is_last
                );
            }
        }
    }
}

extern "system" fn about_dlg_proc(hwnd: HWND, msg: u32, wparam: WPARAM, _lparam: LPARAM) -> isize {
    unsafe {
        match msg {
            WM_INITDIALOG => return 1,

            WM_COMMAND => {
                let id = loword(wparam.0 as u32);

                if id == 1 || id == 2 {
                    let _ = EndDialog(hwnd, wparam.0 as isize);
                    return 1;
                }
            }

            _ => {}
        }

        0
    }
}

// Calculate maximum cards allowed based on number of players
// Standard deck has 52 cards, so max cards = floor(52 / num_players)
// But never exceed 15 cards per player
fn calc_max_cards_for_players(num_players: u32) -> u32 {
    let max_from_deck = 52 / num_players;
    max_from_deck.min(15)
}

extern "system" fn options_dlg_proc(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> isize {
    unsafe {
        match msg {
            WM_INITDIALOG => {
                // Populate from current config
                let cfg = app_state().lock().unwrap().config.clone();

                // Initialize scrollbar for number of players (2-6)
                SendDlgItemMessageW(hwnd, 4001, SBM_SETRANGE, WPARAM(2), LPARAM(6));
                SendDlgItemMessageW(
                    hwnd,
                    4001,
                    SBM_SETPOS,
                    WPARAM(cfg.num_players as usize),
                    LPARAM(1),
                );

                // Update players value label
                let players_str = format!("{}\0", cfg.num_players);
                let _ = SetDlgItemTextW(hwnd, 4010, PCWSTR(wide(&players_str).as_ptr()));

                // Initialize scrollbar for max cards (always 1-15, fixed range)
                SendDlgItemMessageW(hwnd, 4002, SBM_SETRANGE, WPARAM(1), LPARAM(15));

                // Clamp current max cards to what's allowed for current player count
                let max_allowed = calc_max_cards_for_players(cfg.num_players);
                let max_cards = cfg.max_cards.min(max_allowed);
                SendDlgItemMessageW(
                    hwnd,
                    4002,
                    SBM_SETPOS,
                    WPARAM(max_cards as usize),
                    LPARAM(1),
                );

                // Update max cards value label
                let cards_str = format!("{}\0", max_cards);
                let _ = SetDlgItemTextW(hwnd, 4012, PCWSTR(wide(&cards_str).as_ptr()));

                // Radios

                let _ = SendDlgItemMessageW(
                    hwnd,
                    if matches!(cfg.next_notify, NextNotify::Dialog) {
                        4003
                    } else {
                        4004
                    },
                    BM_SETCHECK,
                    WPARAM(BST_CHECKED_U),
                    LPARAM(0),
                );

                let _ = SendDlgItemMessageW(
                    hwnd,
                    if matches!(cfg.score_mode, ScoreMode::Vanilla) {
                        4005
                    } else {
                        4006
                    },
                    BM_SETCHECK,
                    WPARAM(BST_CHECKED_U),
                    LPARAM(0),
                );

                // Checks

                let _ = SendDlgItemMessageW(
                    hwnd,
                    4007,
                    BM_SETCHECK,
                    WPARAM(if cfg.confirm_exit {
                        BST_CHECKED_U
                    } else {
                        BST_UNCHECKED_U
                    }),
                    LPARAM(0),
                );

                let _ = SendDlgItemMessageW(
                    hwnd,
                    4008,
                    BM_SETCHECK,
                    WPARAM(if cfg.hard_score {
                        BST_CHECKED_U
                    } else {
                        BST_UNCHECKED_U
                    }),
                    LPARAM(0),
                );

                let _ = SendDlgItemMessageW(
                    hwnd,
                    4009,
                    BM_SETCHECK,
                    WPARAM(if cfg.cheat_cards {
                        BST_CHECKED_U
                    } else {
                        BST_UNCHECKED_U
                    }),
                    LPARAM(0),
                );

                let _ = SendDlgItemMessageW(
                    hwnd,
                    4011,
                    BM_SETCHECK,
                    WPARAM(if cfg.classic_layout {
                        BST_CHECKED_U
                    } else {
                        BST_UNCHECKED_U
                    }),
                    LPARAM(0),
                );

                return 1;
            }

            WM_HSCROLL => {
                // Handle scrollbar changes
                let hwnd_scrollbar = HWND(lparam.0 as _);
                let scroll_code = loword(wparam.0 as u32) as i32;

                // Determine which scrollbar was moved
                let sb_players = GetDlgItem(hwnd, 4001).unwrap();
                let sb_cards = GetDlgItem(hwnd, 4002).unwrap();

                if hwnd_scrollbar == sb_players {
                    // Players scrollbar changed
                    let cur_pos =
                        SendMessageW(hwnd_scrollbar, SBM_GETPOS, WPARAM(0), LPARAM(0)).0 as i32;
                    let mut new_pos = cur_pos;

                    // Update position based on scroll action
                    if scroll_code == SB_LINEUP.0 || scroll_code == SB_LINELEFT.0 {
                        new_pos -= 1;
                    } else if scroll_code == SB_LINEDOWN.0 || scroll_code == SB_LINERIGHT.0 {
                        new_pos += 1;
                    } else if scroll_code == SB_PAGEUP.0 || scroll_code == SB_PAGELEFT.0 {
                        new_pos -= 1;
                    } else if scroll_code == SB_PAGEDOWN.0 || scroll_code == SB_PAGERIGHT.0 {
                        new_pos += 1;
                    } else if scroll_code == SB_THUMBTRACK.0 || scroll_code == SB_THUMBPOSITION.0 {
                        new_pos = hiword(wparam.0 as u32) as i32;
                    } else {
                        return 0;
                    }

                    // Clamp to range [2, 6]
                    new_pos = new_pos.clamp(2, 6);

                    if new_pos != cur_pos {
                        SendMessageW(
                            hwnd_scrollbar,
                            SBM_SETPOS,
                            WPARAM(new_pos as usize),
                            LPARAM(1),
                        );

                        // Update value label
                        let players_str = format!("{}\0", new_pos);
                        let _ = SetDlgItemTextW(hwnd, 4010, PCWSTR(wide(&players_str).as_ptr()));

                        // Clamp max cards position if it exceeds the new limit
                        let max_allowed = calc_max_cards_for_players(new_pos as u32);
                        let cur_cards =
                            SendDlgItemMessageW(hwnd, 4002, SBM_GETPOS, WPARAM(0), LPARAM(0)).0
                                as u32;

                        if cur_cards > max_allowed {
                            // Clamp to new maximum
                            SendDlgItemMessageW(
                                hwnd,
                                4002,
                                SBM_SETPOS,
                                WPARAM(max_allowed as usize),
                                LPARAM(1),
                            );

                            // Update max cards value label
                            let cards_str = format!("{}\0", max_allowed);
                            let _ = SetDlgItemTextW(hwnd, 4012, PCWSTR(wide(&cards_str).as_ptr()));
                        }
                    }
                } else if hwnd_scrollbar == sb_cards {
                    // Max cards scrollbar changed
                    let cur_pos =
                        SendMessageW(hwnd_scrollbar, SBM_GETPOS, WPARAM(0), LPARAM(0)).0 as i32;
                    let mut new_pos = cur_pos;

                    // Update position based on scroll action
                    if scroll_code == SB_LINEUP.0 || scroll_code == SB_LINELEFT.0 {
                        new_pos -= 1;
                    } else if scroll_code == SB_LINEDOWN.0 || scroll_code == SB_LINERIGHT.0 {
                        new_pos += 1;
                    } else if scroll_code == SB_PAGEUP.0 || scroll_code == SB_PAGELEFT.0 {
                        new_pos -= 1;
                    } else if scroll_code == SB_PAGEDOWN.0 || scroll_code == SB_PAGERIGHT.0 {
                        new_pos += 1;
                    } else if scroll_code == SB_THUMBTRACK.0 || scroll_code == SB_THUMBPOSITION.0 {
                        new_pos = hiword(wparam.0 as u32) as i32;
                    } else {
                        return 0;
                    }

                    // Get current range based on players
                    let num_players =
                        SendDlgItemMessageW(hwnd, 4001, SBM_GETPOS, WPARAM(0), LPARAM(0)).0 as u32;
                    let max_allowed = calc_max_cards_for_players(num_players);

                    // Clamp to range [1, max_allowed]
                    new_pos = new_pos.max(1).min(max_allowed as i32);

                    if new_pos != cur_pos {
                        SendMessageW(
                            hwnd_scrollbar,
                            SBM_SETPOS,
                            WPARAM(new_pos as usize),
                            LPARAM(1),
                        );

                        // Update value label
                        let cards_str = format!("{}\0", new_pos);
                        let _ = SetDlgItemTextW(hwnd, 4012, PCWSTR(wide(&cards_str).as_ptr()));
                    }
                }

                return 0;
            }

            WM_COMMAND => {
                let id = loword(wparam.0 as u32);

                if id == 1 {
                    // OK - Read values from scrollbars
                    let num_players =
                        SendDlgItemMessageW(hwnd, 4001, SBM_GETPOS, WPARAM(0), LPARAM(0)).0 as u32;
                    let max_cards =
                        SendDlgItemMessageW(hwnd, 4002, SBM_GETPOS, WPARAM(0), LPARAM(0)).0 as u32;

                    let next_notify =
                        if SendDlgItemMessageW(hwnd, 4003, BM_GETCHECK, WPARAM(0), LPARAM(0)).0
                            as usize
                            == BST_CHECKED_U
                        {
                            NextNotify::Dialog
                        } else {
                            NextNotify::Mouse
                        };

                    let score_mode =
                        if SendDlgItemMessageW(hwnd, 4005, BM_GETCHECK, WPARAM(0), LPARAM(0)).0
                            as usize
                            == BST_CHECKED_U
                        {
                            ScoreMode::Vanilla
                        } else {
                            ScoreMode::Squared
                        };

                    let confirm_exit =
                        SendDlgItemMessageW(hwnd, 4007, BM_GETCHECK, WPARAM(0), LPARAM(0)).0
                            as usize
                            == BST_CHECKED_U;

                    let hard_score =
                        SendDlgItemMessageW(hwnd, 4008, BM_GETCHECK, WPARAM(0), LPARAM(0)).0
                            as usize
                            == BST_CHECKED_U;

                    let cheat_cards =
                        SendDlgItemMessageW(hwnd, 4009, BM_GETCHECK, WPARAM(0), LPARAM(0)).0
                            as usize
                            == BST_CHECKED_U;

                    let classic_layout =
                        SendDlgItemMessageW(hwnd, 4011, BM_GETCHECK, WPARAM(0), LPARAM(0)).0
                            as usize
                            == BST_CHECKED_U;

                    let mut state = app_state().lock().unwrap();

                    state.config = UiConfig {
                        num_players,
                        max_cards,
                        score_mode,
                        next_notify,
                        confirm_exit,
                        hard_score,
                        cheat_cards,
                        classic_layout,
                    };

                    drop(state);

                    save_config_to_registry(&app_state().lock().unwrap().config);

                    let _ = EndDialog(hwnd, 1);

                    return 1;
                }

                if id == 2 {
                    let _ = EndDialog(hwnd, 2);
                    return 1;
                }
            }

            _ => {}
        }

        0
    }
}

extern "system" fn scores_dlg_proc(hwnd: HWND, msg: u32, wparam: WPARAM, _lparam: LPARAM) -> isize {
    unsafe {
        match msg {
            WM_INITDIALOG => {
                // Populate name controls (601-610) and score controls (611-620)
                // matching Pascal implementation with 20 separate controls

                let hs = load_high_scores();

                // Populate name controls (601-610)
                // Truncate names to 15 characters to prevent wrapping/clipping
                for i in 0..10u32 {
                    let name: String = hs.names[i as usize].chars().take(15).collect();
                    let name_wide = wide(&name);
                    let _ = SetDlgItemTextW(hwnd, 601 + i as i32, PCWSTR(name_wide.as_ptr()));
                }

                // Populate score controls (611-620)
                for i in 0..10u32 {
                    let score = wide(&format!("{}", hs.values[i as usize]));
                    let _ = SetDlgItemTextW(hwnd, 611 + i as i32, PCWSTR(score.as_ptr()));
                }

                return 1;
            }

            WM_COMMAND => {
                let id = loword(wparam.0 as u32);

                if id == 1 || id == 2 {
                    let _ = EndDialog(hwnd, wparam.0 as isize);
                    return 1;
                }
            }

            _ => {}
        }

        0
    }
}

extern "system" fn call_dlg_proc(hwnd: HWND, msg: u32, wparam: WPARAM, _lparam: LPARAM) -> isize {
    unsafe {
        match msg {
            WM_INITDIALOG => {
                // Disable invalid call buttons beyond dealt_cards and, if present, the forbidden value

                let app = app_state().lock().unwrap();

                let maxc = app.game.dealt_cards;

                let forbid = app.game.bidding_forbidden;

                drop(app);

                for v in 0..=15 {
                    let child = GetDlgItem(hwnd, (IDC_CALL_BASE + v as u16) as i32).unwrap();

                    let _ = EnableWindow(child, BOOL(1));
                }

                for v in (maxc + 1)..=15 {
                    let child = GetDlgItem(hwnd, (IDC_CALL_BASE + v as u16) as i32).unwrap();

                    let _ = EnableWindow(child, BOOL(0));
                }

                if let Some(fv) = forbid {
                    if fv <= 15 {
                        let child = GetDlgItem(hwnd, (IDC_CALL_BASE + fv as u16) as i32).unwrap();

                        let _ = EnableWindow(child, BOOL(0));
                    }
                }

                return 1;
            }

            WM_COMMAND => {
                let id = loword(wparam.0 as u32);

                if (IDC_CALL_BASE..=IDC_CALL_BASE + 15).contains(&id) {
                    let sel = (id - IDC_CALL_BASE) as isize;

                    let _ = EndDialog(hwnd, sel);

                    return 1;
                }
            }

            _ => {}
        }

        0
    }
}

extern "system" fn random_dlg_proc(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> isize {
    unsafe {
        match msg {
            WM_INITDIALOG => {
                let app = app_state().lock().unwrap();
                let cfg = &app.random_things.config;
                let mult_sb = GetDlgItem(hwnd, IDC_RNDMULTSC).unwrap();
                let _ = SendMessageW(mult_sb, SBM_SETRANGE, WPARAM(1), LPARAM(20));
                let _ = SendMessageW(
                    mult_sb,
                    SBM_SETPOS,
                    WPARAM(cfg.multiplier as usize),
                    LPARAM(1),
                );
                let numb_sb = GetDlgItem(hwnd, IDC_RNDNUMBSC).unwrap();
                let _ = SendMessageW(numb_sb, SBM_SETRANGE, WPARAM(1), LPARAM(4));
                let _ = SendMessageW(numb_sb, SBM_SETPOS, WPARAM(cfg.count), LPARAM(1));
                let time_sb = GetDlgItem(hwnd, IDC_RNDTIMESC).unwrap();
                let _ = SendMessageW(time_sb, SBM_SETRANGE, WPARAM(20), LPARAM(1000));
                let _ = SendMessageW(
                    time_sb,
                    SBM_SETPOS,
                    WPARAM(cfg.interval_ms as usize),
                    LPARAM(1),
                );
                let mult_text = wide(&format!("{}", cfg.multiplier));
                let _ = SetDlgItemTextW(hwnd, IDC_RNDMULTST, PCWSTR(mult_text.as_ptr()));
                let numb_text = wide(&format!("{}", cfg.count));
                let _ = SetDlgItemTextW(hwnd, IDC_RNDNUMBST, PCWSTR(numb_text.as_ptr()));
                let time_text = wide(&format!("{}", cfg.interval_ms));
                let _ = SetDlgItemTextW(hwnd, IDC_RNDTIMEST, PCWSTR(time_text.as_ptr()));
                let _ = SendDlgItemMessageW(
                    hwnd,
                    IDC_RNDEXISCK,
                    BM_SETCHECK,
                    WPARAM(if cfg.enabled {
                        BST_CHECKED_U
                    } else {
                        BST_UNCHECKED_U
                    }),
                    LPARAM(0),
                );
                let _ = SendDlgItemMessageW(
                    hwnd,
                    IDC_RNDICONCK,
                    BM_SETCHECK,
                    WPARAM(if cfg.icon_twirl_enabled {
                        BST_CHECKED_U
                    } else {
                        BST_UNCHECKED_U
                    }),
                    LPARAM(0),
                );
                1
            }
            WM_HSCROLL => {
                let scroll_code = loword(wparam.0 as u32) as i32;
                let scrollbar = HWND(lparam.0 as _);
                let sb_id = GetDlgCtrlID(scrollbar);

                // Get current position and range
                let mut pos = SendMessageW(scrollbar, SBM_GETPOS, WPARAM(0), LPARAM(0)).0 as i32;

                // Determine range based on which scrollbar
                let (min, max, step) = match sb_id {
                    IDC_RNDMULTSC => (1, 20, 1),
                    IDC_RNDNUMBSC => (1, 4, 1),
                    IDC_RNDTIMESC => (20, 1000, 10),
                    _ => return 0,
                };

                // Update position based on scroll code
                if scroll_code == SB_LINEUP.0 || scroll_code == SB_LINELEFT.0 {
                    pos = (pos - step).max(min);
                } else if scroll_code == SB_LINEDOWN.0 || scroll_code == SB_LINERIGHT.0 {
                    pos = (pos + step).min(max);
                } else if scroll_code == SB_PAGEUP.0 || scroll_code == SB_PAGELEFT.0 {
                    pos = (pos - step * 5).max(min);
                } else if scroll_code == SB_PAGEDOWN.0 || scroll_code == SB_PAGERIGHT.0 {
                    pos = (pos + step * 5).min(max);
                } else if scroll_code == SB_THUMBTRACK.0 || scroll_code == SB_THUMBPOSITION.0 {
                    pos = (wparam.0 >> 16) as i32;
                    pos = pos.clamp(min, max);
                } else if scroll_code == SB_TOP.0 || scroll_code == SB_LEFT.0 {
                    pos = min;
                } else if scroll_code == SB_BOTTOM.0 || scroll_code == SB_RIGHT.0 {
                    pos = max;
                } else {
                    return 0;
                }

                // Set new position
                let _ = SendMessageW(scrollbar, SBM_SETPOS, WPARAM(pos as usize), LPARAM(1));

                // Update corresponding static text
                let text = wide(&format!("{}", pos));
                match sb_id {
                    IDC_RNDMULTSC => {
                        let _ = SetDlgItemTextW(hwnd, IDC_RNDMULTST, PCWSTR(text.as_ptr()));
                    }
                    IDC_RNDNUMBSC => {
                        let _ = SetDlgItemTextW(hwnd, IDC_RNDNUMBST, PCWSTR(text.as_ptr()));
                    }
                    IDC_RNDTIMESC => {
                        let _ = SetDlgItemTextW(hwnd, IDC_RNDTIMEST, PCWSTR(text.as_ptr()));
                    }
                    _ => {}
                }
                0
            }
            WM_COMMAND => {
                let id = loword(wparam.0 as u32) as i32;
                match id {
                    1 => {
                        // IDOK
                        let main_hwnd = GetParent(hwnd).unwrap();
                        let mult_sb = GetDlgItem(hwnd, IDC_RNDMULTSC).unwrap();
                        let multiplier =
                            SendMessageW(mult_sb, SBM_GETPOS, WPARAM(0), LPARAM(0)).0 as i32;
                        let numb_sb = GetDlgItem(hwnd, IDC_RNDNUMBSC).unwrap();
                        let count =
                            SendMessageW(numb_sb, SBM_GETPOS, WPARAM(0), LPARAM(0)).0 as usize;
                        let time_sb = GetDlgItem(hwnd, IDC_RNDTIMESC).unwrap();
                        let interval_ms =
                            SendMessageW(time_sb, SBM_GETPOS, WPARAM(0), LPARAM(0)).0 as u32;
                        let enabled = SendDlgItemMessageW(
                            hwnd,
                            IDC_RNDEXISCK,
                            BM_GETCHECK,
                            WPARAM(0),
                            LPARAM(0),
                        )
                        .0 as usize
                            == BST_CHECKED_U;
                        let icon_twirl = SendDlgItemMessageW(
                            hwnd,
                            IDC_RNDICONCK,
                            BM_GETCHECK,
                            WPARAM(0),
                            LPARAM(0),
                        )
                        .0 as usize
                            == BST_CHECKED_U;
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
                            save_random_things_config(&app.random_things.config);
                            if app.random_things.random_timer_active && interval_ms != old_interval
                            {
                                let _ = KillTimer(main_hwnd, ID_RNDTIMER);
                                let _ = SetTimer(main_hwnd, ID_RNDTIMER, interval_ms, None);
                            }
                            if enabled != old_enabled {
                                if enabled && !app.game.in_progress {
                                    drop(app);
                                    start_random_things(main_hwnd);
                                    app = app_state().lock().unwrap();
                                } else if !enabled {
                                    drop(app);
                                    stop_random_things(main_hwnd);
                                    app = app_state().lock().unwrap();
                                }
                            }
                            if (count != old_count) || (enabled && !old_enabled) {
                                let mut rc = RECT::default();
                                let _ = GetClientRect(main_hwnd, &mut rc);
                                app.random_things.resize_things(rc.right, rc.bottom);
                            }
                            if icon_twirl != old_icon_twirl {
                                if icon_twirl && IsIconic(main_hwnd).as_bool() {
                                    drop(app);
                                    start_icon_twirl(main_hwnd);
                                } else if !icon_twirl {
                                    drop(app);
                                    stop_icon_twirl(main_hwnd);
                                }
                            }
                        }
                        let _ = EndDialog(hwnd, 1);
                        1
                    }
                    2 => {
                        // IDCANCEL
                        let _ = EndDialog(hwnd, 0);
                        1
                    }
                    _ => 0,
                }
            }
            _ => 0,
        }
    }
}

unsafe fn show_random_things_dialog(parent: HWND) {
    let hinst = GetModuleHandleW(None).unwrap();
    let _ = DialogBoxParamW(
        hinst,
        make_int_resource(3006),
        parent,
        Some(random_dlg_proc),
        LPARAM(0),
    );
}

// High scores persistence

#[derive(Clone, Debug, Default)]

struct HighScores {
    names: [String; 10],

    values: [u32; 10],
}

fn load_high_scores() -> HighScores {
    let mut hs = HighScores::default();

    for i in 0..10 {
        let idx = i + 1;

        let s_key = format!("Score{:02}", idx);

        let n_key = format!("Name{:02}", idx);

        hs.values[i] = registry::get_u32(&s_key, 0);

        hs.names[i] = registry::get_string(&n_key).unwrap_or_else(|| "-".to_string());
    }

    hs
}

fn save_high_scores(hs: &HighScores) {
    for i in 0..10 {
        let idx = i + 1;

        let s_key = format!("Score{:02}", idx);

        let n_key = format!("Name{:02}", idx);

        let _ = registry::set_u32(&s_key, hs.values[i]);

        let _ = registry::set_string(&n_key, &hs.names[i]);
    }
}

fn maybe_update_high_scores(hwnd: HWND, player_score: u32) {
    let mut hs = load_high_scores();

    if player_score <= hs.values[9] {
        return;
    }

    let name = unsafe { show_name_dialog(hwnd) }.unwrap_or_else(|| "Player".to_string());

    let mut pos = 10usize;

    for i in 0..10 {
        if player_score > hs.values[i] {
            pos = i;
            break;
        }
    }

    if pos < 10 {
        for i in (pos + 1..10).rev() {
            hs.values[i] = hs.values[i - 1];
            hs.names[i] = hs.names[i - 1].clone();
        }

        hs.values[pos] = player_score;
        // Truncate name to 15 characters (matching Pascal STRING[15])
        hs.names[pos] = name.chars().take(15).collect();

        save_high_scores(&hs);
    }
}

// Simple name dialog; returns Some(name) if OK

unsafe fn show_name_dialog(parent: HWND) -> Option<String> {
    const IDD_NAME: u16 = 3005;

    const IDC_NAME_EDIT: i32 = 5001;

    static NAME_BUF: OnceLock<Mutex<String>> = OnceLock::new();

    fn name_buf() -> &'static Mutex<String> {
        NAME_BUF.get_or_init(|| Mutex::new(String::new()))
    }

    extern "system" fn name_dlg_proc(
        hwnd: HWND,
        msg: u32,
        wparam: WPARAM,
        _lparam: LPARAM,
    ) -> isize {
        unsafe {
            match msg {
                WM_INITDIALOG => {
                    // Limit name to 15 characters (matching Pascal STRING[15])
                    const EM_LIMITTEXT: u32 = 0x00C5;
                    let _ = SendDlgItemMessageW(
                        hwnd,
                        IDC_NAME_EDIT,
                        EM_LIMITTEXT,
                        WPARAM(15),
                        LPARAM(0),
                    );
                    return 1;
                }

                WM_COMMAND => {
                    let id = loword(wparam.0 as u32);

                    if id == 1 {
                        // OK

                        let mut buf: [u16; 128] = [0; 128];

                        let _ = GetDlgItemTextW(hwnd, IDC_NAME_EDIT, &mut buf);

                        if let Ok(s) = String::from_utf16(
                            &buf.iter()
                                .copied()
                                .take_while(|&c| c != 0)
                                .collect::<Vec<u16>>(),
                        ) {
                            *name_buf().lock().unwrap() = s;
                        }

                        let _ = EndDialog(hwnd, 1);

                        return 1;
                    }

                    if id == 2 {
                        let _ = EndDialog(hwnd, 2);
                        return 1;
                    }
                }

                _ => {}
            }

            0
        }
    }

    let hinst = GetModuleHandleW(None).unwrap();

    let ret = DialogBoxParamW(
        hinst,
        make_int_resource(IDD_NAME),
        parent,
        Some(name_dlg_proc),
        LPARAM(0),
    );

    if ret == 1 {
        let s = name_buf().lock().unwrap().clone();

        if s.trim().is_empty() {
            None
        } else {
            Some(s)
        }
    } else {
        None
    }
}
