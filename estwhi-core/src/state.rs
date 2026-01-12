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

/// Determines which player (0-based index) should act next in the current trick.
///
/// # Arguments
/// * `start_player` - The player who led the trick (1-based).
/// * `trick` - The current trick state (vector of options, where None means not played).
///
/// # Returns
/// * `Some(index)` - The 0-based index of the next player to act.
/// * `None` - If the trick is full or invalid (empty).
pub fn next_player_to_act(start_player: u32, trick: &[Option<u32>]) -> Option<usize> {
    let n = trick.len();
    if n == 0 {
        return None;
    }
    // Convert 1-based start_player to 0-based index
    let start0 = start_player.saturating_sub(1) as usize;

    // Check slots in turn order
    for i in 0..n {
        let p = (start0 + i) % n;
        if trick[p].is_none() {
            return Some(p);
        }
    }
    None
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
        assert_eq!(next_trump(5), 1); // Boundary > 4
        assert_eq!(next_trump(100), 1); // Far boundary
    }

    #[test]
    fn player_rotation() {
        let n = 4;
        assert_eq!(next_start_player(0, n), 1);
        assert_eq!(next_start_player(1, n), 2);
        assert_eq!(next_start_player(4, n), 1);
        assert_eq!(next_start_player(5, n), 1); // Boundary > n
        assert_eq!(next_start_player(100, n), 1); // Far boundary
    }

    #[test]
    fn next_player_logic() {
        // 4 players, start player 1
        let trick_empty = vec![None, None, None, None];
        assert_eq!(next_player_to_act(1, &trick_empty), Some(0));

        // Player 1 played
        let trick_p1 = vec![Some(1), None, None, None];
        assert_eq!(next_player_to_act(1, &trick_p1), Some(1));

        // Player 1 and 2 played
        let trick_p12 = vec![Some(1), Some(2), None, None];
        assert_eq!(next_player_to_act(1, &trick_p12), Some(2));

        // Full trick
        let trick_full = vec![Some(1), Some(2), Some(3), Some(4)];
        assert_eq!(next_player_to_act(1, &trick_full), None);

        // Start player 3 (index 2)
        // Order: 2, 3, 0, 1
        assert_eq!(next_player_to_act(3, &trick_empty), Some(2));

        let trick_p3 = vec![None, None, Some(1), None];
        assert_eq!(next_player_to_act(3, &trick_p3), Some(3));

        let trick_p34 = vec![None, None, Some(1), Some(2)];
        assert_eq!(next_player_to_act(3, &trick_p34), Some(0));

        // Edge case: Empty trick vec
        assert_eq!(next_player_to_act(1, &[]), None);
    }
}
