use windows::core::PCWSTR;
use windows::Win32::Foundation::{COLORREF, HWND, RECT};
use windows::Win32::Graphics::Gdi::{
    BitBlt, CreateCompatibleDC, CreateSolidBrush, DeleteDC, DeleteObject, FillRect, FrameRect,
    GetObjectW, Rectangle as GdiRectangle, SelectObject, SetBkColor, SetBkMode, SetStretchBltMode,
    SetTextColor, StretchBlt, TextOutW, BITMAP, HALFTONE, HBITMAP, HBRUSH, HDC, NOTSRCCOPY, OPAQUE,
    SRCCOPY, TRANSPARENT,
};
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::UI::WindowsAndMessaging::{
    LoadImageW, MessageBoxW, IMAGE_BITMAP, LR_CREATEDIBSECTION, LR_DEFAULTSIZE, MB_ICONINFORMATION,
};

use crate::game_state::GameState;
use estwhi_core::is_legal_play;
use std::sync::{Mutex, OnceLock};

// Constants
pub const CARD_W: i32 = 71;
pub const CARD_H: i32 = 96;
pub const HAND_Y: i32 = 228;
pub const HAND_X0: i32 = 10;
pub const HAND_SPAN_X: i32 = 500;
pub const MIN_WIDTH: i32 = 20;
pub const TRICK_Y: i32 = 55;
pub const TRICK_X0: i32 = 10;
pub const TRICK_STEP: i32 = 30;
pub const THING_SIZE: i32 = 31;

static CARD_BITMAPS: OnceLock<Mutex<Vec<Option<isize>>>> = OnceLock::new();

#[inline]
pub const fn make_int_resource(id: u16) -> PCWSTR {
    PCWSTR(id as usize as *const u16)
}

pub fn wide(s: &str) -> Vec<u16> {
    use std::os::windows::prelude::*;
    std::ffi::OsStr::new(s)
        .encode_wide()
        .chain(std::iter::once(0))
        .collect()
}

pub fn get_card_bitmap(card_id: u32) -> Option<HBITMAP> {
    // legacy resource IDs: 1..52
    // If card_id is 0 or > 52, return None
    if !(1..=52).contains(&card_id) {
        return None;
    }

    let cache_mutex = CARD_BITMAPS.get_or_init(|| Mutex::new(vec![None; 52]));
    // Handle poisoned mutex by recovering the inner guard
    let mut cache = match cache_mutex.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    };

    let idx = (card_id - 1) as usize;

    if let Some(val) = cache[idx] {
        return Some(HBITMAP(val as *mut _));
    }

    // SAFETY: GetModuleHandleW(None) gets the handle for the current process.
    // We check for error (0 handle) implicitly via Ok() check on LoadImageW.
    // make_int_resource creates a valid resource identifier pointer.
    unsafe {
        let hinst = GetModuleHandleW(None).ok()?;

        if let Ok(handle) = LoadImageW(
            hinst,
            make_int_resource(card_id as u16),
            IMAGE_BITMAP,
            0,
            0,
            LR_DEFAULTSIZE | LR_CREATEDIBSECTION,
        ) {
            let hbmp = HBITMAP(handle.0);
            cache[idx] = Some(hbmp.0 as isize);
            Some(hbmp)
        } else {
            None
        }
    }
}

pub fn cleanup_resources() {
    if let Some(cache) = CARD_BITMAPS.get() {
        // Recover from poison to ensure cleanup
        let mut cache = match cache.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };

        for entry in cache.iter_mut() {
            if let Some(ptr) = entry.take() {
                let hbmp = HBITMAP(ptr as *mut _);
                // SAFETY: We own the bitmap handle from the cache and are deleting it.
                unsafe {
                    let _ = DeleteObject(hbmp);
                }
            }
        }
    }
}

/// Helper: Safely create a compatible DC or return early.
unsafe fn create_mem_dc(hdc: HDC) -> Option<windows::Win32::Graphics::Gdi::HDC> {
    let memdc = CreateCompatibleDC(hdc);
    if memdc.0.is_null() {
        None
    } else {
        Some(memdc)
    }
}

pub unsafe fn blit_bitmap(hdc: HDC, hbmp: HBITMAP, x: i32, y: i32, w: i32, h: i32) {
    // SAFETY: Check for valid memory DC creation.
    let memdc = match create_mem_dc(hdc) {
        Some(dc) => dc,
        None => return,
    };

    let old = SelectObject(memdc, hbmp);
    let mut bm = BITMAP::default();

    // SAFETY: GDI object validation.
    let _ = GetObjectW(
        windows::Win32::Graphics::Gdi::HGDIOBJ(hbmp.0),
        std::mem::size_of::<BITMAP>() as i32,
        Some(&mut bm as *mut _ as *mut _),
    );

    let _ = SetStretchBltMode(hdc, HALFTONE);
    let _ = StretchBlt(
        hdc,
        x,
        y,
        w,
        h,
        memdc,
        0,
        0,
        bm.bmWidth,
        bm.bmHeight,
        SRCCOPY,
    );

    let _ = SelectObject(memdc, old);
    let _ = DeleteDC(memdc);
}

pub unsafe fn blit_card(hdc: HDC, hbmp: HBITMAP, x: i32, y: i32, invert: bool) {
    let memdc = match create_mem_dc(hdc) {
        Some(dc) => dc,
        None => return,
    };

    let old = SelectObject(memdc, hbmp);

    let rop = if invert { NOTSRCCOPY } else { SRCCOPY };

    let _ = BitBlt(hdc, x, y, CARD_W, CARD_H, memdc, 0, 0, rop);

    let _ = SelectObject(memdc, old);
    let _ = DeleteDC(memdc);
}

pub unsafe fn draw_bevel_box(hdc: HDC, rc: RECT) {
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

pub unsafe fn draw_status(hdc: HDC, rc: &RECT, text: &str) {
    SetBkMode(hdc, TRANSPARENT);
    SetTextColor(hdc, COLORREF(0x000000));

    let mut r = *rc;
    r.left += 8;
    r.top += 4;

    let w = wide(text);
    let sl = &w[..w.len() - 1];

    let _ = TextOutW(hdc, r.left, r.top, sl);
}

pub unsafe fn draw_info_panel(hdc: HDC, num_players: u32, max_cards: u32, game: &GameState) {
    // Classic absolute layout for info grid and footer lines

    // Draw main info box
    draw_bevel_box(
        hdc,
        RECT {
            left: 340,
            top: 20,
            right: 585,
            bottom: 60 + 15 * (num_players as i32),
        },
    );

    // Headings
    SetBkMode(hdc, OPAQUE);
    SetTextColor(hdc, COLORREF(0x000000));

    // Match button-face background
    let btnface = COLORREF(0x00F0F0F0);
    SetBkColor(hdc, btnface);

    let heads = [(420, "Calls:"), (470, "Tricks:"), (520, "Scores:")];

    for (x, s) in heads {
        let w = wide(s);
        let sl = &w[..w.len() - 1];
        let _ = TextOutW(hdc, x, 30, sl);
    }

    // Player labels and values
    for i in 0..num_players as usize {
        let y = 35 + 15 * ((i + 1) as i32);

        let label = wide(&format!("Player {}:", i + 1));
        let sll = &label[..label.len() - 1];
        let _ = TextOutW(hdc, 350, y, sll);

        let calls = wide(&format!("{}", game.calls.get(i).copied().unwrap_or(0)));
        let slc = &calls[..calls.len() - 1];
        let _ = TextOutW(hdc, 430, y, slc);

        let tricks = wide(&format!("{}", game.tricks.get(i).copied().unwrap_or(0)));
        let slt = &tricks[..tricks.len() - 1];
        let _ = TextOutW(hdc, 490, y, slt);

        let score = wide(&format!("{}", game.scores.get(i).copied().unwrap_or(0)));
        let sls = &score[..score.len() - 1];
        let _ = TextOutW(hdc, 540, y, sls);
    }

    // Footer lines: Round number, Player to start, Last trick winner
    let total_rounds = (max_cards * 2).saturating_sub(1);

    let w1 = wide(&format!(
        "Round number: {} of {}",
        game.round_no, total_rounds
    ));
    let s1 = &w1[..w1.len() - 1];
    let _ = TextOutW(hdc, 420, 170, s1);

    let w2 = wide(&format!("Player to start: {}", game.start_player.max(1)));
    let s2 = &w2[..w2.len() - 1];
    let _ = TextOutW(hdc, 420, 185, s2);

    let last = game.last_winner.unwrap_or(0);
    let w3 = if last > 0 {
        wide(&format!("Last trick won by: {}", last))
    } else {
        wide("Last trick won by:      ")
    };
    let s3 = &w3[..w3.len() - 1];
    let _ = TextOutW(hdc, 420, 200, s3);

    // Trump suit small icon at (285,80)
    let name = match game.trump {
        1 => Some("CLUB"),
        2 => Some("DIAMOND"),
        3 => Some("SPADE"),
        4 => Some("HEART"),
        _ => None,
    };

    if let Some(id) = name {
        // SAFETY: Safe resource loading with error checking (ok()?).
        if let Ok(hinst) = GetModuleHandleW(None) {
            if let Ok(obj) = LoadImageW(
                hinst,
                PCWSTR(wide(id).as_ptr()),
                IMAGE_BITMAP,
                0,
                0,
                LR_DEFAULTSIZE | LR_CREATEDIBSECTION,
            ) {
                blit_bitmap(hdc, HBITMAP(obj.0), 285, 80, 31, 31);
            }
        }
    }

    SetBkMode(hdc, TRANSPARENT);
}

pub unsafe fn draw_extra_info(hdc: HDC, rc: &RECT, max_cards: u32, game: &GameState) {
    SetBkMode(hdc, OPAQUE);
    SetTextColor(hdc, COLORREF(0x000000));
    SetBkColor(hdc, COLORREF(0x00F0F0F0));

    let mut y = rc.top + 10;
    let x = rc.left + 10;
    let line_height = 15;

    let total_rounds = (max_cards * 2).saturating_sub(1);

    let round_text = wide(&format!(
        "Round number: {} of {}",
        game.round_no, total_rounds
    ));
    let _ = TextOutW(hdc, x, y, &round_text[..round_text.len() - 1]);
    y += line_height;

    let start_text = wide(&format!("Player to start: {}", game.start_player));
    let _ = TextOutW(hdc, x, y, &start_text[..start_text.len() - 1]);
    y += line_height;

    let winner_str = if let Some(w) = game.last_winner {
        format!("{}", w)
    } else {
        "      ".to_string()
    };
    let winner_text = wide(&format!("Last trick won by: {}", winner_str));
    let _ = TextOutW(hdc, x, y, &winner_text[..winner_text.len() - 1]);

    SetBkMode(hdc, TRANSPARENT);
}

pub fn calculate_hand_layout(num_cards: usize) -> Vec<RECT> {
    if num_cards == 0 {
        return Vec::new();
    }

    let act_width = if num_cards > 1 {
        let mut w = (HAND_SPAN_X - CARD_W) / ((num_cards - 1) as i32);
        if w > (CARD_W + 10) {
            w = CARD_W + 10;
        }
        w
    } else {
        CARD_W
    };

    // Note: MessageBox for too small window is a UI side effect,
    // we'll leave it in the draw function or return an error/empty here?
    // For pure logic, we just return the rects. The caller decides if they are too squeezed.

    let mut positions: Vec<RECT> = Vec::with_capacity(num_cards);
    let mut x = HAND_X0;
    let y = HAND_Y;
    let mut last_idx: Option<usize> = None;

    for i in 0..num_cards {
        let r = RECT {
            left: x,
            top: y,
            right: x + CARD_W,
            bottom: y + CARD_H,
        };

        if let Some(pi) = last_idx {
            if act_width < CARD_W {
                if let Some(prev) = positions.get_mut(pi) {
                    prev.right = prev.left + act_width;
                }
            }
        }

        positions.push(r);
        last_idx = Some(i);
        x += if num_cards > 1 { act_width } else { CARD_W };
    }
    positions
}

pub unsafe fn draw_hand_classic(hdc: HDC, game: &GameState) -> Vec<RECT> {
    let n = game.hand.len();

    if n == 0 {
        return Vec::new();
    }

    let positions = calculate_hand_layout(n);

    // Check min width constraint check (legacy logic)
    let act_width = if n > 1 {
        // Re-calculate or derive from positions?
        // Let's just re-calculate for the check
        let mut w = (HAND_SPAN_X - CARD_W) / ((n - 1) as i32);
        if w > (CARD_W + 10) {
            w = CARD_W + 10;
        }
        w
    } else {
        CARD_W
    };

    if n > 1 && act_width <= MIN_WIDTH {
        let _ = MessageBoxW(
            HWND(0 as _),
            PCWSTR(wide("Window too small to draw cards").as_ptr()),
            PCWSTR(wide("Estimation Whist").as_ptr()),
            MB_ICONINFORMATION,
        );
        return Vec::new();
    }

    SetBkMode(hdc, TRANSPARENT);
    SetTextColor(hdc, COLORREF(0x000000));

    for (i, r) in positions.iter().enumerate() {
        let card_id = game.hand[i];
        let legal = is_legal_play(card_id, &game.trick, &game.hand);

        if let Some(hbmp) = get_card_bitmap(card_id) {
            blit_card(hdc, hbmp, r.left, r.top, !legal);
        } else {
            let _ = GdiRectangle(hdc, r.left, r.top, r.right, r.bottom);
            let label = wide(&format!("{}", card_id));
            let sl = &label[..label.len() - 1];
            let _ = TextOutW(hdc, r.left + 6, r.top + 6, sl);
        }
    }
    positions
}

pub unsafe fn draw_trick_classic(
    hdc: HDC,
    num_players: u32,
    start_player: u32,
    trick: &[Option<u32>],
    hbr_green: HBRUSH,
) {
    let n_players = num_players as i32;

    let area = RECT {
        left: 39,
        top: 54,
        right: 41 + CARD_W + ((n_players - 1) * 30),
        bottom: 80 + CARD_H,
    };

    if hbr_green.0 != core::ptr::null_mut() {
        let _ = FillRect(hdc, &area, hbr_green);
    }

    SetTextColor(hdc, COLORREF(0x000000));

    let n = num_players as usize;
    if n == 0 {
        return;
    }

    let start = start_player.max(1) as usize; // 1-based in state
    let base = start - 1; // 0-based

    for a in 1..=n {
        let p = (base + (a - 1)) % n;

        if let Some(Some(card_id)) = trick.get(p) {
            if let Some(hbmp) = get_card_bitmap(*card_id) {
                let x = TRICK_X0 + (TRICK_STEP * (a as i32));
                let y = TRICK_Y;
                blit_card(hdc, hbmp, x, y, false);
            } else {
                let x = TRICK_X0 + (TRICK_STEP * (a as i32));
                let y = TRICK_Y;
                // dbglog! not available here, skip log
                let _ = GdiRectangle(hdc, x, y, x + CARD_W, y + CARD_H);
            }
        }

        // Numeric label beneath on green background
        let lx = 20 + (30 * (a as i32));
        let ly = 65 + CARD_H; // under the row
        let w = wide(&format!("{}", p + 1));
        let sl = &w[..w.len() - 1];

        SetBkColor(hdc, COLORREF(128 << 8));
        SetBkMode(hdc, OPAQUE);
        let _ = TextOutW(hdc, lx, ly, sl);
    }

    SetBkMode(hdc, TRANSPARENT);
}

pub unsafe fn draw_card_scaled(
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

    let memdc = match create_mem_dc(hdc) {
        Some(dc) => dc,
        None => return,
    };

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

    let _ = SelectObject(memdc, old_bmp);
    let _ = DeleteDC(memdc);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hand_layout_empty() {
        assert!(calculate_hand_layout(0).is_empty());
    }

    #[test]
    fn test_hand_layout_single() {
        let rects = calculate_hand_layout(1);
        assert_eq!(rects.len(), 1);
        assert_eq!(rects[0].left, HAND_X0);
        assert_eq!(rects[0].right, HAND_X0 + CARD_W);
    }

    #[test]
    fn test_hand_layout_full() {
        let rects = calculate_hand_layout(13);
        assert_eq!(rects.len(), 13);
        // Verify overlap logic (previous rect right side is adjusted)
        // Except for the last one
        let first = rects[0];
        let second = rects[1];
        // act_width for 13 cards: (500 - 71) / 12 = 429 / 12 = 35.
        // CARD_W is 71. So they overlap.
        // first.right should be first.left + 35 = 10 + 35 = 45.
        // Original logic: "prev.right = prev.left + act_width"
        assert_eq!(first.right - first.left, 35);
        assert_eq!(second.left, first.left + 35);

        // Last card should be full width
        let last = rects[12];
        assert_eq!(last.right - last.left, CARD_W);
    }
}
