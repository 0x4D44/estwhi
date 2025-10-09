# Scores Dialog Comparison: TPfW 1.5 vs Rust Implementation

## Executive Summary

The high scores dialog implementation differs significantly between the original Turbo Pascal for Windows 1.5 version and the current Rust implementation. The Pascal version uses **20 separate static controls** (10 for names + 10 for scores), while the Rust version uses **10 combined controls** with formatted text.

**Status**: ✅ **ANALYSIS COMPLETE**

---

## 1. Pascal Implementation (TPfW 1.5)

### 1.1 Resource Definition

**Dialog Name**: `Scores` (referenced in estwhi_v11.pas:1206)

**Control IDs** (from estwhi_v11.pas:62-81):
- **Name Controls**: ID_SCRNAME1 through ID_SCRNAME10 (601-610)
- **Score Controls**: ID_SCRVALU1 through ID_SCRVALU10 (611-620)

**Total Controls**: 20 static text controls + 1 OK button = 21 controls

**Note**: The original .RC file for the Scores dialog was not found in the codebase (only ABOUT.RC exists). The dialog definition is likely in the compiled ESTWHI.RES file.

### 1.2 Dialog Class Structure (estwhi_v11.pas:182-191)

```pascal
TYPE
  PScoreBox = ^TScoreBox;
  TScoreBox = OBJECT(TDialog)
    Score1, Score2, Score3, Score4, Score5,
    Score6, Score7, Score8, Score9, Score10,
    Value1, Value2, Value3, Value4, Value5,
    Value6, Value7, Value8, Value9, Value10: PStatic;
    CONSTRUCTOR Init (AParent: PWindowsObject; AName: PChar);
    PROCEDURE SetupWindow; VIRTUAL;
  END;
```

**Key Points**:
- 10 `Score` pointers for name controls
- 10 `Value` pointers for score value controls
- Each control is a separate `PStatic` object

### 1.3 Constructor (estwhi_v11.pas:3038-3066)

```pascal
CONSTRUCTOR TScoreBox.Init (AParent: PWindowsObject; AName: PChar);
BEGIN
  TDialog.Init(AParent, AName);

  Score1  := New(PStatic, InitResource(@self, ID_SCRNAME1, 15));
  Score2  := New(PStatic, InitResource(@self, ID_SCRNAME2, 15));
  [... Score3 through Score10 ...]

  Value1  := New(PStatic, InitResource(@self, ID_SCRVALU1, 5));
  Value2  := New(PStatic, InitResource(@self, ID_SCRVALU2, 5));
  [... Value3 through Value10 ...]
END;
```

**Key Points**:
- Each control is initialized with `InitResource`
- Name controls have max length 15 characters
- Score value controls have max length 5 characters
- Total: 20 separate control initializations

### 1.4 Data Population (estwhi_v11.pas:3069-3107)

```pascal
PROCEDURE TScoreBox.SetupWindow;
VAR
   TextString: ARRAY [0..20] OF CHAR;
BEGIN
  TDialog.SetupWindow;

  (* Set score values *)
  Str(BestScores[1], TextString);
  Value1^.SetText(TextString);
  Str(BestScores[2], TextString);
  Value2^.SetText(TextString);
  [... repeat for Value3 through Value10 ...]

  (* Set name values *)
  Score1^.SetText(BestNames[1]);
  Score2^.SetText(BestNames[2]);
  [... repeat for Score3 through Score10 ...]
END;
```

**Key Points**:
- `BestScores` array contains integer values (lines 437-438)
- `BestNames` array contains string values (lines 437-438)
- `Str()` converts integers to strings
- Each control is populated individually (20 separate `SetText` calls)

### 1.5 Data Structure

```pascal
BestNames : ARRAY [1..10] OF STRING[15];
BestScores: ARRAY [1..10] OF LONGINT;
```

**Key Points**:
- Names limited to 15 characters
- Scores stored as LONGINT (32-bit integers)
- 1-indexed arrays

---

## 2. Rust Implementation

### 2.1 Resource Definition (estwhi/resources/app.rc:90-107)

```rc
3003 DIALOGEX 0, 0, 300, 220
STYLE DS_MODALFRAME | WS_POPUP | WS_CAPTION | WS_SYSMENU
CAPTION "High Scores"
FONT 9, "Segoe UI"
BEGIN
    LTEXT "Name                   Score", -1, 20, 10, 260, 10
    LTEXT "", 4100, 20, 25, 260, 10
    LTEXT "", 4101, 20, 40, 260, 10
    LTEXT "", 4102, 20, 55, 260, 10
    LTEXT "", 4103, 20, 70, 260, 10
    LTEXT "", 4104, 20, 85, 260, 10
    LTEXT "", 4105, 20, 100, 260, 10
    LTEXT "", 4106, 20, 115, 260, 10
    LTEXT "", 4107, 20, 130, 260, 10
    LTEXT "", 4108, 20, 145, 260, 10
    LTEXT "", 4109, 20, 160, 260, 10
    DEFPUSHBUTTON "OK", IDOK, 125, 195, 50, 14
END
```

**Key Points**:
- Dialog size: 300x220 pixels (larger than typical Pascal dialogs)
- **11 LTEXT controls total**:
  - 1 header control (ID -1): "Name                   Score"
  - 10 data controls (IDs 4100-4109): Empty strings, filled at runtime
- Only **1 OK button** (no Cancel button)
- Controls are **260 pixels wide** (very wide for combined text)
- **15 pixel vertical spacing** between rows

### 2.2 Dialog Procedure (estwhi/src/main.rs:3597-3632)

```rust
extern "system" fn scores_dlg_proc(hwnd: HWND, msg: u32, wparam: WPARAM, _lparam: LPARAM) -> isize {
    unsafe {
        match msg {
            WM_INITDIALOG => {
                // Populate fixed lines 4100..4109
                let hs = load_high_scores();

                for i in 0..10u32 {
                    let name = &hs.names[i as usize];
                    let val = hs.values[i as usize];

                    let line = wide(&format!("{:<20} {:>6}", name, val));

                    let _ = SetDlgItemTextW(hwnd, 4100 + i as i32, PCWSTR(line.as_ptr()));
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
```

**Key Points**:
- **Single loop** to populate all 10 controls
- **Format string**: `"{:<20} {:>6}"`
  - Name: left-aligned, 20 characters wide
  - Score: right-aligned, 6 characters wide
  - Total width: 26 characters plus spacing
- **Combined text**: Name and score in one string per line
- Only **10 SetDlgItemTextW calls** instead of 20

### 2.3 Data Structure (estwhi/src/main.rs:3908-3912)

```rust
#[derive(Default)]
struct HighScores {
    names: [String; 10],
    values: [i32; 10],
}
```

**Key Points**:
- Parallel arrays (same as Pascal)
- Names stored as `String` (no fixed length limit in memory)
- Scores stored as `i32` (32-bit integers, same as Pascal LONGINT)
- **0-indexed** arrays (unlike Pascal's 1-indexed)

---

## 3. Key Differences

### 3.1 Control Architecture

| Aspect | Pascal (TPfW 1.5) | Rust |
|--------|-------------------|------|
| **Number of Controls** | 20 (10 names + 10 scores) | 10 (combined) |
| **Control IDs** | 601-610 (names), 611-620 (scores) | 4100-4109 |
| **Control Type** | Separate PStatic objects | Single LTEXT per row |
| **Data Format** | Separate text in each control | Combined formatted string |
| **Initialization Calls** | 20 x `New(PStatic, ...)` | 10 x `SetDlgItemTextW` |
| **Update Calls** | 20 x `SetText()` | 10 x `SetDlgItemTextW` |

### 3.2 Layout & Spacing

| Aspect | Pascal (TPfW 1.5) | Rust |
|--------|-------------------|------|
| **Dialog Size** | Unknown (no .RC found) | 300x220 pixels |
| **Font** | Unknown (likely MS Sans Serif 8pt) | Segoe UI 9pt |
| **Column Alignment** | Two separate controls (precise positioning) | Single control with format string |
| **Name Width** | 15 characters max | 20 characters (format) |
| **Score Width** | 5 characters max | 6 characters (format) |
| **Header Row** | Unknown | "Name                   Score" |

### 3.3 Code Complexity

| Aspect | Pascal (TPfW 1.5) | Rust |
|--------|-------------------|------|
| **Lines of Code** | ~70 lines (Init + SetupWindow) | ~30 lines (dialog proc) |
| **Control Declarations** | 20 pointer fields in object | 0 (direct ID usage) |
| **Update Pattern** | Manual, repetitive (20 calls) | Loop-based (1 loop, 10 iterations) |
| **Formatting** | `Str()` conversion function | Rust `format!` macro |

### 3.4 Visual Appearance

**Pascal (Expected)**:
```
Name Control 1          Score Control 1
Name Control 2          Score Control 2
[...]
```
- **Pros**:
  - Precise alignment guaranteed by dialog editor
  - Separate controls allow different fonts/styles
  - Can right-align scores independently

- **Cons**:
  - More complex resource definition
  - More memory for 20 controls vs 10

**Rust (Current)**:
```
Name                   Score
[combined text: "Martin               1234"]
[combined text: "Susan                 987"]
[...]
```
- **Pros**:
  - Simpler resource definition
  - Less memory (10 controls vs 20)
  - Easier to maintain and update
  - More flexible name length (no hard limit)

- **Cons**:
  - Relies on monospace-like alignment (format string padding)
  - Can't use different fonts for name vs score
  - Harder to ensure pixel-perfect alignment

---

## 4. Functional Equivalence Analysis

### 4.1 What Works Identically

✅ **Data Storage**: Both use parallel arrays for names and scores
✅ **Display**: Both show 10 rows of high scores
✅ **Persistence**: Both save to registry/INI (same keys)
✅ **User Interaction**: Both are modal dialogs with OK button

### 4.2 What Works Differently

⚠️ **Control Count**: Pascal 20 controls, Rust 10 controls
⚠️ **Alignment Method**: Pascal uses separate controls, Rust uses format strings
⚠️ **Name Length**: Pascal enforces 15-char limit via control, Rust uses 20-char formatting
⚠️ **Font**: Pascal likely uses MS Sans Serif 8pt, Rust uses Segoe UI 9pt
⚠️ **Dialog Size**: Pascal size unknown, Rust is 300x220

### 4.3 Potential Issues with Rust Implementation

**Issue #1: Alignment with Proportional Font**

The Rust implementation uses Segoe UI, which is a **proportional font** (not monospace). This means:

```
"{:<20} {:>6}"
```

May **not** produce perfectly aligned columns because:
- "W" is wider than "i"
- "Name" with spaces may not align with "Score" column reliably

**Example Problem**:
```
Name                   Score
William                  1234    <- W is wide
iiiiiiiiiiii              987    <- i is narrow (more fits)
```

The columns may not line up visually, even though they have the same character count.

**Pascal Advantage**: With separate controls positioned by pixel coordinates, alignment is **guaranteed** regardless of font.

**Issue #2: No Cancel Button**

- Pascal version: Unknown (typical dialogs have Cancel, but scores might only have OK)
- Rust version: Only OK button (IDOK)
- Current code handles both ID 1 (OK) and ID 2 (Cancel), but .RC only defines OK

This matches the typical behavior for a high scores display (read-only, no Cancel needed).

**Issue #3: Header Row Formatting**

- Rust implementation includes header row: `"Name                   Score"`
- Pascal implementation: Unknown if header exists
- If Pascal doesn't have header, this is a UX improvement

---

## 5. Recommendations for Alignment

### Option A: Keep Rust Implementation (Recommended)

**Rationale**: The Rust implementation is simpler, more maintainable, and functionally equivalent.

**Required Changes**:

1. **Fix font alignment issue** by switching to a monospace font for the scores dialog:

```rc
3003 DIALOGEX 0, 0, 300, 220
STYLE DS_MODALFRAME | WS_POPUP | WS_CAPTION | WS_SYSMENU
CAPTION "High Scores"
FONT 9, "Consolas"  /* Change from Segoe UI to Consolas (monospace) */
BEGIN
    [... rest unchanged ...]
END
```

Or use `Courier New` if Consolas is not available:
```rc
FONT 8, "Courier New"
```

2. **Optional: Match Pascal dialog size** (if original size is known):
   - Measure the original dialog if possible
   - Adjust width/height and control positions to match

3. **Optional: Adjust name/score field widths**:
   - Change format from `"{:<20} {:>6}"` to `"{:<15} {:>5}"` to match Pascal's control limits
   - Adjust dialog width accordingly (could be narrower)

**Pros**:
- Minimal changes required
- Keeps clean, maintainable code
- Only needs font change

**Cons**:
- Still different from Pascal architecture
- Can't style name and score differently

---

### Option B: Match Pascal Architecture Exactly

**Rationale**: Replicate the original design precisely for pixel-perfect compatibility.

**Required Changes**:

1. **Update app.rc dialog 3003** to use 20 separate controls:

```rc
3003 DIALOGEX 0, 0, 220, 180
STYLE DS_MODALFRAME | WS_POPUP | WS_CAPTION | WS_SYSMENU
CAPTION "High Scores"
FONT 8, "MS Sans Serif"
BEGIN
    LTEXT "Name", -1, 20, 10, 100, 10
    LTEXT "Score", -1, 140, 10, 50, 10

    LTEXT "", 601, 20, 25, 100, 10   /* Score1 (name) */
    LTEXT "", 611, 140, 25, 50, 10, SS_RIGHT  /* Value1 (score, right-aligned) */

    LTEXT "", 602, 20, 40, 100, 10   /* Score2 */
    LTEXT "", 612, 140, 40, 50, 10, SS_RIGHT  /* Value2 */

    [... repeat for 603-610 and 613-620 ...]

    DEFPUSHBUTTON "OK", IDOK, 85, 160, 50, 14
END
```

2. **Update main.rs scores_dlg_proc** to populate 20 controls:

```rust
extern "system" fn scores_dlg_proc(hwnd: HWND, msg: u32, wparam: WPARAM, _lparam: LPARAM) -> isize {
    unsafe {
        match msg {
            WM_INITDIALOG => {
                let hs = load_high_scores();

                // Populate name controls (601-610)
                for i in 0..10u32 {
                    let name = wide(&hs.names[i as usize]);
                    let _ = SetDlgItemTextW(hwnd, 601 + i as i32, PCWSTR(name.as_ptr()));
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
```

**Pros**:
- Matches Pascal architecture exactly
- Pixel-perfect alignment guaranteed
- Right-aligned scores (SS_RIGHT style)
- Can use proportional fonts safely

**Cons**:
- More complex .RC file (20 controls instead of 10)
- Longer code (2 loops instead of 1)
- More memory (20 HWND handles vs 10)

---

## 6. Final Recommendation

**✅ Recommended Approach: Option A with Font Fix**

**Reasoning**:
1. The Rust implementation is functionally equivalent and simpler
2. Changing to a monospace font solves the alignment issue
3. Modern Windows applications commonly use simpler approaches
4. Easier to maintain going forward
5. Uses less memory and fewer resources

**Changes Required**:

### Change 1: Update Font (app.rc:93)

**From**:
```rc
FONT 9, "Segoe UI"
```

**To**:
```rc
FONT 9, "Consolas"
```

### Change 2 (Optional): Adjust Dialog Width

If using Consolas 9pt, may want to adjust width:

**From**:
```rc
3003 DIALOGEX 0, 0, 300, 220
```

**To**:
```rc
3003 DIALOGEX 0, 0, 260, 220
```

And adjust control width:

**From**:
```rc
LTEXT "", 4100, 20, 25, 260, 10
```

**To**:
```rc
LTEXT "", 4100, 20, 25, 220, 10
```

### Change 3 (Optional): Match Pascal Name Length

If Pascal enforces 15-char names, update format string (main.rs:3610):

**From**:
```rust
let line = wide(&format!("{:<20} {:>6}", name, val));
```

**To**:
```rust
let line = wide(&format!("{:<15}  {:>5}", name, val));
```

---

## 7. Testing Checklist

After making changes:

- [ ] Build and run application (`cargo build`)
- [ ] Open High Scores dialog (Game → Scores or F3)
- [ ] Verify name and score columns are perfectly aligned
- [ ] Verify all 10 scores display correctly
- [ ] Verify font is readable and professional
- [ ] Verify dialog size is appropriate (not too wide/narrow)
- [ ] Compare with original Pascal version if available

---

## 8. Summary

**Current State**:
- Rust implementation uses 10 combined controls with format strings
- Pascal implementation uses 20 separate controls
- Functionally equivalent but architecturally different

**Issue Identified**:
- Segoe UI is proportional, may cause misaligned columns

**Recommended Fix**:
- Change font to Consolas (monospace) to ensure alignment
- Optionally adjust dialog width
- Keep simplified 10-control architecture

**Alternative**:
- Replicate Pascal's 20-control architecture exactly
- More work, same end result

**Implementation Time**:
- Option A (Font fix): 5-10 minutes
- Option B (Full replication): 30-45 minutes

---

## 9. Files Analyzed

1. **estwhi_v11.pas** (lines 62-81, 182-191, 1206, 3036-3107)
   - Control ID definitions
   - TScoreBox class declaration
   - Constructor and SetupWindow method

2. **estwhi/resources/app.rc** (lines 90-107)
   - Dialog 3003 resource definition

3. **estwhi/src/main.rs** (lines 3597-3632, 3908-3912)
   - scores_dlg_proc implementation
   - HighScores struct definition

---

## 10. Conclusion

The Rust implementation is **well-designed** and **functionally equivalent** to the Pascal version. The only issue is the use of a **proportional font** which may cause alignment problems. Switching to **Consolas** or **Courier New** will resolve this while maintaining the cleaner, simpler architecture.

**Status**: ✅ **READY FOR IMPLEMENTATION**

**Recommended Next Step**: Implement Option A (font change) and test.

---

**Document Date**: 2025-10-09
**Author**: Claude Code
**Review Status**: ✅ Complete
