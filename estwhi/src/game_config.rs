use crate::registry;
use crate::ui_logic::RandomThingsConfig;
use estwhi_core::{
    config::{parse_score_mode, serialize_score_mode, validate_max_cards, validate_players},
    ScoreMode,
};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum NextNotify {
    Dialog = 0,
    Mouse = 1,
}

#[derive(Clone, Debug)]
pub struct UiConfig {
    pub num_players: u32, // 2..6
    pub max_cards: u32,   // 1..15
    pub score_mode: ScoreMode,
    pub next_notify: NextNotify,
    pub confirm_exit: bool,
    pub cheat_cards: bool,
}

impl Default for UiConfig {
    fn default() -> Self {
        Self {
            num_players: 4,
            max_cards: 13,
            score_mode: ScoreMode::Vanilla,
            next_notify: NextNotify::Mouse,
            confirm_exit: true,
            cheat_cards: false,
        }
    }
}

#[derive(Default)]
pub struct CheatWindowState {
    pub hwnd: Option<isize>,
    pub offset_x: i32, // Offset from main window left edge
    pub offset_y: i32, // Offset from main window top edge
}

pub fn load_config_from_registry() -> UiConfig {
    let mut cfg = UiConfig::default();

    let np = registry::get_u32("NumberOfPlayers", cfg.num_players);
    let mc = registry::get_u32("MaxCards", cfg.max_cards);
    let sm = registry::get_u32("ScoreMode", 0);
    let nn = registry::get_u32("NextCardNotify", 1);
    let ce = registry::get_u32("ConfirmExit", 1);
    let ch = registry::get_u32("CheatCards", 0);

    cfg.num_players = validate_players(np);
    cfg.max_cards = validate_max_cards(mc);
    cfg.score_mode = parse_score_mode(sm);
    cfg.next_notify = if nn == 0 {
        NextNotify::Dialog
    } else {
        NextNotify::Mouse
    };
    cfg.confirm_exit = ce != 0;
    cfg.cheat_cards = ch != 0;

    cfg
}

pub fn save_config_to_registry(cfg: &UiConfig) {
    let _ = registry::set_u32("NumberOfPlayers", cfg.num_players);
    let _ = registry::set_u32("MaxCards", cfg.max_cards);
    let _ = registry::set_u32("ScoreMode", serialize_score_mode(cfg.score_mode));
    let _ = registry::set_u32(
        "NextCardNotify",
        match cfg.next_notify {
            NextNotify::Dialog => 0,
            NextNotify::Mouse => 1,
        },
    );
    let _ = registry::set_u32("ConfirmExit", if cfg.confirm_exit { 1 } else { 0 });
    let _ = registry::set_u32("CheatCards", if cfg.cheat_cards { 1 } else { 0 });
}

pub fn load_cheat_window_state() -> CheatWindowState {
    CheatWindowState {
        hwnd: None,
        offset_x: registry::get_u32("CheatWindowOffsetX", 100) as i32,
        offset_y: registry::get_u32("CheatWindowOffsetY", 100) as i32,
    }
}

pub fn save_cheat_window_state(state: &CheatWindowState) {
    let _ = registry::set_u32("CheatWindowOffsetX", state.offset_x as u32);
    let _ = registry::set_u32("CheatWindowOffsetY", state.offset_y as u32);
}

pub fn load_random_things_config() -> RandomThingsConfig {
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

pub fn save_random_things_config(cfg: &RandomThingsConfig) {
    let _ = registry::rt_set_u32("Multiplier", cfg.multiplier as u32);
    let _ = registry::rt_set_u32("Number of", cfg.count as u32);
    let _ = registry::rt_set_u32("Time interval", cfg.interval_ms);
    let _ = registry::rt_set_u32("They exist", if cfg.enabled { 1 } else { 0 });
    let _ = registry::rt_set_u32("Icon twirl", if cfg.icon_twirl_enabled { 1 } else { 0 });
}
