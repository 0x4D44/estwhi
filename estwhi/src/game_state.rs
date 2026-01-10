use crate::game_config::{CheatWindowState, UiConfig};
use crate::ui_logic::RandomThings;
use windows::Win32::Foundation::RECT;

#[derive(Clone, Debug, Default)]
pub struct GameState {
    pub in_progress: bool,
    pub round_no: u32,
    pub trump: u32, // 1..4
    pub start_player: u32,
    pub dealt_cards: u32,
    pub calls: Vec<u32>,
    pub tricks: Vec<u32>,
    pub scores: Vec<u32>,
    pub last_winner: Option<u32>,
    pub hand: Vec<u32>,
    pub hand_positions: Vec<RECT>,
    pub selected: Option<usize>,
    pub hands: Vec<Vec<u32>>,    // all players' hands (legacy ids)
    pub trick: Vec<Option<u32>>, // current trick cards per player index
    pub waiting_for_human: bool,
    pub current_player: usize,
    pub cards_remaining: u32,
    // bidding helper for call dialog: forbidden call value for last bidder (if any)
    pub bidding_forbidden: Option<u32>,
    pub waiting_for_continue: bool,
}

pub struct AppState {
    pub config: UiConfig,
    pub game: GameState,
    pub random_things: RandomThings,
    pub cheat_window: CheatWindowState,
}
