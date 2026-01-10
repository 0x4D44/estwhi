//! AI logic for Estimation Whist.
//!
//! Provides algorithms for computer players to:
//! 1. Calculate bids (estimates) based on hand strength.
//! 2. Choose cards to play (currently random legal moves).

use crate::{is_legal_play, suit_index_from_legacy_id};
use rand::Rng;

/// Calculates a bid (estimate) for a hand.
///
/// # Logic
/// Sums estimated trick-taking probability for each card:
/// - Ace: 1.0
/// - King: 0.8
/// - Queen: 0.6
/// - Jack: 0.5
/// - Ten: 0.4
/// - Others: 0.0
///
/// Adds 0.2 for each trump card.
/// Results are rounded to the nearest integer.
///
/// If `is_last_bidder` is true, the bid is adjusted to avoid the "forbidden" total
/// (sum of bids cannot equal number of cards dealt).
///
/// # Arguments
/// * `hand` - The player's hand (legacy card IDs).
/// * `trump` - The trump suit index (1=Clubs, 2=Diamonds, 3=Spades, 4=Hearts).
/// * `cards_dealt` - Number of cards dealt in this round.
/// * `sum_so_far` - Sum of bids from previous players (only used if `is_last_bidder` is true).
/// * `is_last_bidder` - Whether this is the last player to bid.
pub fn calculate_bid(
    hand: &[u32],
    trump: u32,
    cards_dealt: u32,
    sum_so_far: u32,
    is_last_bidder: bool,
) -> u32 {
    let mut est: f32 = 0.0;

    for &id in hand {
        // Legacy: 1=Ace, 13=King, 12=Queen, etc.
        // rank_idx calculation:
        let r = ((id - 1) % 13) + 1;

        est += match r {
            1 => 1.0,  // Ace
            13 => 0.8, // King
            12 => 0.6, // Queen
            11 => 0.5, // Jack
            10 => 0.4, // Ten
            _ => 0.0,
        };

        if suit_index_from_legacy_id(id) == trump {
            est += 0.2;
        }
    }

    let mut call = est.round() as u32;

    if call > cards_dealt {
        call = cards_dealt;
    }

    if is_last_bidder {
        let forbidden = cards_dealt.saturating_sub(sum_so_far);
        if call == forbidden {
            call = if call == 0 { 1 } else { call - 1 };
        }
    }

    call
}

/// Selects a card to play from the hand.
///
/// # Logic
/// Currently implements a "Random Legal" strategy:
/// 1. Identifies all legal cards to play (must follow suit if able).
/// 2. Picks one at random.
///
/// # Arguments
/// * `hand` - The player's hand (legacy card IDs).
/// * `trick` - The current trick state (cards played so far).
/// * `rng` - Random number generator.
///
/// # Returns
/// The legacy ID of the selected card. Panics if hand is empty.
pub fn select_card_to_play<R: Rng + ?Sized>(
    hand: &[u32],
    trick: &[Option<u32>],
    rng: &mut R,
) -> u32 {
    if hand.is_empty() {
        panic!("AI cannot play from empty hand");
    }

    // Filter for legal cards
    let legal: Vec<u32> = hand
        .iter()
        .copied()
        .filter(|&c| is_legal_play(c, trick, hand))
        .collect();

    // Should never be empty if hand is not empty (at least one card is always legal)
    if legal.is_empty() {
        // Fallback (should be unreachable logic-wise)
        return hand[0];
    }

    let idx = rng.gen_range(0..legal.len());
    legal[idx]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bid_calculation_high_cards() {
        // Ace of Clubs(1), King of Clubs(13)
        // Trump = Diamonds (2)
        // Est: 1.0 + 0.8 = 1.8 -> rounds to 2
        let hand = vec![1, 13];
        let bid = calculate_bid(&hand, 2, 2, 0, false);
        assert_eq!(bid, 2);
    }

    #[test]
    fn bid_calculation_trumps() {
        // 2 of Clubs(2), 3 of Clubs(3) -> normally 0
        // Trump = Clubs (1) -> +0.2 each = 0.4 -> rounds to 0
        let hand = vec![2, 3];
        let bid = calculate_bid(&hand, 1, 2, 0, false);
        assert_eq!(bid, 0);

        // 5 trumps -> 1.0 -> rounds to 1
        let hand5 = vec![2, 3, 4, 5, 6];
        let bid5 = calculate_bid(&hand5, 1, 5, 0, false);
        assert_eq!(bid5, 1);
    }

    #[test]
    fn bid_last_player_constraint() {
        // Hand estimates 1.
        // Cards dealt = 1.
        // Sum so far = 0.
        // Forbidden = 1 - 0 = 1.
        // Call calculated as 1, matches forbidden.
        // Should adjust to 0.
        let hand = vec![1]; // Ace -> 1.0
        let bid = calculate_bid(&hand, 2, 1, 0, true);
        assert_eq!(bid, 0);
    }

    #[test]
    fn bid_last_player_constraint_zero_case() {
        // Hand estimates 0.
        // Cards dealt = 1.
        // Sum so far = 1.
        // Forbidden = 1 - 1 = 0.
        // Call calculated as 0, matches forbidden.
        // Should adjust to 1.
        let hand = vec![2]; // 2 of Clubs -> 0.0
        let bid = calculate_bid(&hand, 2, 1, 1, true);
        assert_eq!(bid, 1);
    }

    #[test]
    fn bid_calculation_all_ranks() {
        // Ace(1), King(13), Queen(12), Jack(11), Ten(10), Nine(9)
        // Trump = Spades(3)
        // Est: 1.0 + 0.8 + 0.6 + 0.5 + 0.4 + 0.0 = 3.3 -> rounds to 3
        let hand = vec![1, 13, 12, 11, 10, 9];
        let bid = calculate_bid(&hand, 3, 6, 0, false);
        assert_eq!(bid, 3);
    }

    #[test]
    fn bid_calculation_clamp() {
        // Hand estimates 10, but only 5 cards dealt
        let hand = vec![1; 10]; // 10 Aces
        let bid = calculate_bid(&hand, 2, 5, 0, false);
        assert_eq!(bid, 5);
    }

    #[test]
    fn bid_last_player_no_collision() {
        // Hand estimates 2, cards dealt 5, sum 0. Forbidden = 5.
        // No collision, should remain 2.
        let hand = vec![1, 1]; // 2 Aces
        let bid = calculate_bid(&hand, 2, 5, 0, true);
        assert_eq!(bid, 2);
    }

    #[test]
    fn ai_select_card_simple() {
        let hand = vec![1, 14, 27, 40]; // One of each suit
        let trick = vec![None, None, None, None];
        let mut rng = rand::thread_rng();
        let card = select_card_to_play(&hand, &trick, &mut rng);
        assert!(hand.contains(&card));
    }

    #[test]
    fn ai_select_card_follow_suit() {
        let hand = vec![1, 2, 14]; // 2 Clubs, 1 Diamond
        let trick = vec![Some(13)]; // Club led (King of Clubs)
        let mut rng = rand::thread_rng();
        let card = select_card_to_play(&hand, &trick, &mut rng);
        assert!(card == 1 || card == 2);
    }

    #[test]
    #[should_panic(expected = "AI cannot play from empty hand")]
    fn ai_select_card_empty_panic() {
        let hand = vec![];
        let trick = vec![];
        let mut rng = rand::thread_rng();
        let _ = select_card_to_play(&hand, &trick, &mut rng);
    }
}
