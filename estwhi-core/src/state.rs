//! Game state transition logic.

/// Calculates the number of cards to deal for a given round number (1-based).
///
/// Implements the "Mountain Deal" pattern:
/// - Counts up from 1 to `max_cards`
/// - Then counts down from `max_cards - 1` to 1
///
/// # Arguments
/// * `round_no` - Current round number (1-based).
/// * `max_cards` - Maximum cards to deal (peak of the mountain).
///
/// # Returns
/// The number of cards to deal. Returns 0 if the round number is out of bounds
/// (i.e., game over condition).
pub fn cards_to_deal(round_no: u32, max_cards: u32) -> u32 {
    let limit = 2 * max_cards - 1;
    if round_no > limit {
        return 0;
    }
    if round_no <= max_cards {
        round_no
    } else {
        (2 * max_cards) - round_no
    }
}

/// Calculates the next trump suit index.
///
/// Rotates 1 -> 2 -> 3 -> 4 -> 1.
///
/// # Arguments
/// * `current_trump` - Current trump (1=Clubs, 2=Diamonds, 3=Spades, 4=Hearts).
///   If 0 (start of game), returns 1.
pub fn next_trump(current_trump: u32) -> u32 {
    if current_trump == 0 || current_trump >= 4 {
        1
    } else {
        current_trump + 1
    }
}

/// Calculates the next starting player index (1-based).
///
/// Rotates 1 -> 2 -> ... -> num_players -> 1.
///
/// # Arguments
/// * `current_start` - Current starting player (1-based).
/// * `num_players` - Total number of players.
pub fn next_start_player(current_start: u32, num_players: u32) -> u32 {
    if current_start == 0 || current_start >= num_players {
        1
    } else {
        current_start + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mountain_deal() {
        let max = 3;
        // Limit = 2*3 - 1 = 5 rounds
        assert_eq!(cards_to_deal(1, max), 1);
        assert_eq!(cards_to_deal(2, max), 2);
        assert_eq!(cards_to_deal(3, max), 3); // Peak
        assert_eq!(cards_to_deal(4, max), 2);
        assert_eq!(cards_to_deal(5, max), 1);
        assert_eq!(cards_to_deal(6, max), 0); // End
    }

    #[test]
    fn trump_rotation() {
        assert_eq!(next_trump(0), 1);
        assert_eq!(next_trump(1), 2);
        assert_eq!(next_trump(4), 1);
    }

    #[test]
    fn player_rotation() {
        let n = 4;
        assert_eq!(next_start_player(0, n), 1);
        assert_eq!(next_start_player(1, n), 2);
        assert_eq!(next_start_player(4, n), 1);
    }
}
