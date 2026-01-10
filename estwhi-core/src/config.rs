//! Configuration validation and parsing logic.

use crate::ScoreMode;

/// Validates and clamps the number of players.
///
/// # Arguments
/// * `n` - Requested number of players.
///
/// # Returns
/// Clamped value between 2 and 6.
pub fn validate_players(n: u32) -> u32 {
    n.clamp(2, 6)
}

/// Validates and clamps the maximum cards per hand.
///
/// # Arguments
/// * `n` - Requested max cards.
///
/// # Returns
/// Clamped value between 1 and 15.
pub fn validate_max_cards(n: u32) -> u32 {
    n.clamp(1, 15)
}

/// Calculates the maximum allowed cards based on number of players.
///
/// # Arguments
/// * `num_players` - Number of players (2-6).
///
/// # Returns
/// The maximum number of cards that can be dealt (floor(52 / num_players)), clamped to 15.
pub fn calc_max_cards_for_players(num_players: u32) -> u32 {
    if num_players == 0 {
        return 15;
    }
    let max_from_deck = 52 / num_players;
    max_from_deck.min(15)
}

/// Parses score mode from integer representation.
///
/// # Arguments
/// * `val` - 0 for Vanilla, otherwise Squared.
pub fn parse_score_mode(val: u32) -> ScoreMode {
    if val == 0 {
        ScoreMode::Vanilla
    } else {
        ScoreMode::Squared
    }
}

/// Serializes score mode to integer representation.
pub fn serialize_score_mode(mode: ScoreMode) -> u32 {
    match mode {
        ScoreMode::Vanilla => 0,
        ScoreMode::Squared => 1,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn player_limits() {
        assert_eq!(validate_players(1), 2);
        assert_eq!(validate_players(2), 2);
        assert_eq!(validate_players(6), 6);
        assert_eq!(validate_players(100), 6);
    }

    #[test]
    fn max_cards_calculation() {
        assert_eq!(calc_max_cards_for_players(0), 15); // Safety check
        assert_eq!(calc_max_cards_for_players(4), 13); // 52 / 4 = 13
        assert_eq!(calc_max_cards_for_players(3), 15); // 52 / 3 = 17 -> clamp 15
        assert_eq!(calc_max_cards_for_players(2), 15); // 52 / 2 = 26 -> clamp 15
        assert_eq!(calc_max_cards_for_players(6), 8); // 52 / 6 = 8
    }

    #[test]
    fn max_cards_limits() {
        assert_eq!(validate_max_cards(0), 1);
        assert_eq!(validate_max_cards(13), 13);
        assert_eq!(validate_max_cards(15), 15);
        assert_eq!(validate_max_cards(99), 15);
    }

    #[test]
    fn score_mode_roundtrip() {
        assert_eq!(parse_score_mode(0), ScoreMode::Vanilla);
        assert_eq!(parse_score_mode(1), ScoreMode::Squared);
        assert_eq!(parse_score_mode(99), ScoreMode::Squared);

        assert_eq!(serialize_score_mode(ScoreMode::Vanilla), 0);
        assert_eq!(serialize_score_mode(ScoreMode::Squared), 1);
    }
}
