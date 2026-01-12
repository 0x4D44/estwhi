//! Core logic for Estimation Whist — platform independent.
//!
//! This library provides the game logic for Estimation Whist (also known as "Oh Hell"),
//! a trick-taking card game. The implementation is completely platform-independent,
//! with no dependencies on any particular UI framework or operating system.
//!
//! # Example
//!
//! ```
//! use estwhi_core::{Deck, score_hand, ScoreMode};
//! use rand::thread_rng;
//!
//! // Create and shuffle a deck
//! let mut rng = thread_rng();
//! let deck = Deck::new_shuffled(&mut rng);
//!
//! // Score a hand
//! let calls = vec![2, 1, 0, 0];  // Player bids
//! let tricks = vec![2, 1, 0, 0]; // Tricks won
//! let deltas = score_hand(ScoreMode::Vanilla, false, &calls, &tricks, 3);
//! ```

use rand::Rng;

pub mod ai;
pub mod config;
pub mod state;

/// Card suit in a standard 52-card deck.
///
/// Explicit discriminants match legacy card numbering scheme where:
/// - Clubs = 0 (cards 1-13)
/// - Diamonds = 1 (cards 14-26)
/// - Spades = 2 (cards 27-39)
/// - Hearts = 3 (cards 40-52)
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Suit {
    Clubs = 0,
    Diamonds = 1,
    Spades = 2,
    Hearts = 3,
}

/// Card rank in a standard 52-card deck.
///
/// Note: Ace is ranked as 1 in the enum for legacy compatibility, but in
/// actual gameplay, Aces are high (beat Kings). Use [`rank_value`] to get
/// the correct ordering for trick resolution.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Rank {
    Ace = 1,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
}

/// A playing card with a suit and rank.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

impl Card {
    /// Creates a Card from a legacy card ID (1-52).
    ///
    /// Legacy numbering: 1..13 Clubs, 14..26 Diamonds, 27..39 Spades, 40..52 Hearts.
    /// Within each suit: 1=Ace, 2-10=numerics, 11=Jack, 12=Queen, 13=King.
    ///
    /// # Arguments
    /// * `id` - Legacy card ID (must be in range 1..=52)
    ///
    /// # Returns
    /// * `Some(Card)` if the ID is valid
    /// * `None` if the ID is 0 or greater than 52
    ///
    /// # Examples
    /// ```
    /// use estwhi_core::{Card, Suit, Rank};
    ///
    /// let ace_of_clubs = Card::from_legacy_id(1).unwrap();
    /// assert_eq!(ace_of_clubs.suit, Suit::Clubs);
    /// assert_eq!(ace_of_clubs.rank, Rank::Ace);
    ///
    /// let king_of_hearts = Card::from_legacy_id(52).unwrap();
    /// assert_eq!(king_of_hearts.suit, Suit::Hearts);
    /// assert_eq!(king_of_hearts.rank, Rank::King);
    /// ```
    pub fn from_legacy_id(id: u8) -> Option<Card> {
        if id == 0 || id > 52 {
            return None;
        }
        let zero = id - 1; // 0..51
        let suit = match zero / 13 {
            0 => Suit::Clubs,
            1 => Suit::Diamonds,
            2 => Suit::Spades,
            _ => Suit::Hearts,
        };
        let rank_idx = (zero % 13) + 1; // 1..13
        let rank = match rank_idx {
            1 => Rank::Ace,
            2 => Rank::Two,
            3 => Rank::Three,
            4 => Rank::Four,
            5 => Rank::Five,
            6 => Rank::Six,
            7 => Rank::Seven,
            8 => Rank::Eight,
            9 => Rank::Nine,
            10 => Rank::Ten,
            11 => Rank::Jack,
            12 => Rank::Queen,
            _ => Rank::King,
        };
        Some(Card { suit, rank })
    }

    /// Converts this Card to its legacy card ID (1-52).
    ///
    /// # Returns
    /// The legacy card ID, where:
    /// - 1-13 = Clubs (Ace through King)
    /// - 14-26 = Diamonds (Ace through King)
    /// - 27-39 = Spades (Ace through King)
    /// - 40-52 = Hearts (Ace through King)
    pub fn legacy_id(&self) -> u8 {
        let suit_base = match self.suit {
            Suit::Clubs => 0,
            Suit::Diamonds => 13,
            Suit::Spades => 26,
            Suit::Hearts => 39,
        };
        let r = match self.rank {
            Rank::Ace => 1,
            Rank::Two => 2,
            Rank::Three => 3,
            Rank::Four => 4,
            Rank::Five => 5,
            Rank::Six => 6,
            Rank::Seven => 7,
            Rank::Eight => 8,
            Rank::Nine => 9,
            Rank::Ten => 10,
            Rank::Jack => 11,
            Rank::Queen => 12,
            Rank::King => 13,
        };
        (suit_base + r) as u8
    }
}

/// Configuration for a game of Estimation Whist.
#[derive(Clone, Debug)]
pub struct GameConfig {
    /// Number of players (2-6)
    pub num_players: u8,
    /// Maximum cards dealt in any round (1-15, typically 13)
    pub max_cards: u8,
    /// Scoring mode (Vanilla or Squared)
    pub score_mode: ScoreMode,
    /// Whether to use "hard score" mode (penalty for under-bidding the table)
    pub hard_score: bool,
}

/// Scoring mode for calculating hand scores.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ScoreMode {
    /// Vanilla mode: score = tricks + bonus (if bid matched)
    Vanilla,
    /// Squared mode: score = tricks² + bonus (if bid matched), otherwise just tricks
    Squared,
}

/// A shuffled deck of 52 playing cards, represented by legacy card IDs.
#[derive(Clone, Debug)]
pub struct Deck([u8; 52]);

impl Deck {
    /// Creates a new shuffled deck using the Fisher-Yates algorithm.
    ///
    /// # Arguments
    /// * `rng` - A random number generator implementing the [`Rng`] trait
    ///
    /// # Examples
    /// ```
    /// use estwhi_core::Deck;
    /// use rand::thread_rng;
    ///
    /// let mut rng = thread_rng();
    /// let deck = Deck::new_shuffled(&mut rng);
    /// assert_eq!(deck.cards().len(), 52);
    /// ```
    pub fn new_shuffled<R: Rng + ?Sized>(rng: &mut R) -> Self {
        let mut a = [0u8; 52];
        for (i, v) in a.iter_mut().enumerate() {
            *v = (i as u8) + 1;
        }
        // Fisher–Yates shuffle
        for i in (1..52).rev() {
            let j = rng.gen_range(0..=i);
            a.swap(i, j);
        }
        Deck(a)
    }

    /// Returns a reference to the array of 52 shuffled card IDs.
    ///
    /// Card IDs are in the legacy format (1-52).
    pub fn cards(&self) -> &[u8; 52] {
        &self.0
    }
}

// ----- Scoring helpers -----

/// Computes per-player score deltas for a completed hand.
///
/// # Scoring Rules
/// - **AvailableBonus**: 10 points (or 0 if `hard_score` is true and total calls < dealt cards)
/// - **Vanilla mode**:
///   - If bid matched: score = tricks + AvailableBonus
///   - If bid missed: score = tricks
/// - **Squared mode**:
///   - If bid matched: score = AvailableBonus + tricks²
///   - If bid missed: score = tricks
///
/// # Arguments
/// * `score_mode` - The scoring mode to use (Vanilla or Squared)
/// * `hard_score` - Whether to apply the "hard score" penalty for under-bidding
/// * `calls` - Each player's bid (number of tricks they predict winning)
/// * `tricks` - Each player's actual tricks won
/// * `dealt_cards` - Number of cards dealt this hand
///
/// # Returns
/// A vector of score deltas (one per player), to be added to cumulative scores.
///
/// # Examples
/// ```
/// use estwhi_core::{score_hand, ScoreMode};
///
/// let calls = vec![2, 1, 0, 0];
/// let tricks = vec![2, 1, 0, 0];
/// let deltas = score_hand(ScoreMode::Vanilla, false, &calls, &tricks, 3);
/// assert_eq!(deltas, vec![12, 11, 10, 10]); // All matched their bids, get +10 bonus
/// ```
pub fn score_hand(
    score_mode: ScoreMode,
    hard_score: bool,
    calls: &[u32],
    tricks: &[u32],
    dealt_cards: u32,
) -> Vec<u32> {
    let n = calls.len().min(tricks.len());
    let total_calls: u32 = calls.iter().take(n).copied().sum();
    let mut deltas = vec![0u32; n];
    for i in 0..n {
        let call = calls[i];
        let t = tricks[i];
        // Matching any bid (including zero) earns the 10 bonus.
        // Legacy "hard score" mode removed the bonus when the table under-called.
        let available_bonus = if hard_score && total_calls < dealt_cards {
            0
        } else {
            10
        };
        let add = match score_mode {
            ScoreMode::Vanilla => {
                let base = t;
                if call == t {
                    base + available_bonus
                } else {
                    base
                }
            }
            ScoreMode::Squared => {
                if call == t {
                    available_bonus + t.saturating_mul(t)
                } else {
                    t
                }
            }
        };
        deltas[i] = add;
    }
    deltas
}

// ----- Trick rules (ported from legacy) -----

/// Converts a legacy card ID to its suit index.
///
/// # Arguments
/// * `id` - Legacy card ID (1-52)
///
/// # Returns
/// Suit index: 1=Clubs, 2=Diamonds, 3=Spades, 4=Hearts
///
/// Note: This returns 1-based indices (not 0-based) for legacy compatibility.
pub fn suit_index_from_legacy_id(id: u32) -> u32 {
    let zero = (id as i32) - 1;
    let suit = (zero / 13) as u32; // 0..3 => Clubs,Diamonds,Spades,Hearts
    suit + 1
}

/// Returns the rank value for trick comparison, with Ace ranked high.
///
/// # Arguments
/// * `id` - Legacy card ID (1-52)
///
/// # Returns
/// Rank value where Ace=14 (highest), King=13, Queen=12, ..., Two=2.
/// This is used for determining trick winners where Aces beat Kings.
pub fn rank_value(id: u32) -> u32 {
    let r = ((id - 1) % 13) + 1; // 1..13
    if r == 1 {
        14
    } else {
        r
    }
}

/// Determines if a card play is legal given the current trick state.
///
/// # Rules
/// - If no lead card has been played yet, any card is legal
/// - If a lead exists and the player holds cards in the lead suit, they must follow suit
/// - If the player has no cards in the lead suit, any card is legal
///
/// # Arguments
/// * `card_id` - The legacy ID of the card being played (1-52)
/// * `trick` - Current trick state (Some(card) for played cards, None for empty positions)
/// * `hand` - Player's current hand (legacy card IDs)
///
/// # Returns
/// `true` if the play is legal, `false` otherwise
///
/// # Examples
/// ```
/// use estwhi_core::is_legal_play;
///
/// // Hearts led (card 40 = Ace of Hearts), player holds hearts
/// let trick = vec![Some(40), None, None, None];
/// let hand = vec![41, 5, 10]; // Has 41 (2 of Hearts)
/// assert!(!is_legal_play(5, &trick, &hand));  // Can't play clubs when holding hearts
/// assert!(is_legal_play(41, &trick, &hand)); // Must follow suit
/// ```
pub fn is_legal_play(card_id: u32, trick: &[Option<u32>], hand: &[u32]) -> bool {
    // If no lead yet, any card is legal
    let lead_suit = trick
        .iter()
        .flatten()
        .next()
        .map(|&c| suit_index_from_legacy_id(c));
    if lead_suit.is_none() {
        return true;
    }
    let lead = lead_suit.unwrap();
    let has_lead = hand.iter().any(|&c| suit_index_from_legacy_id(c) == lead);
    if !has_lead {
        return true;
    }
    suit_index_from_legacy_id(card_id) == lead
}

/// Determines which player won a completed trick.
///
/// # Rules
/// - Trump cards beat all non-trump cards
/// - Among trump cards (or when no trump is played), highest rank wins
/// - Aces are high (beat Kings)
/// - If no trump is played, only cards matching the lead suit can win
///
/// # Arguments
/// * `trick` - Completed trick with one card per player (all entries must be Some)
/// * `trump` - Trump suit index (1=Clubs, 2=Diamonds, 3=Spades, 4=Hearts)
///
/// # Returns
/// * `Some(index)` - The seat index (0-based) of the winning player
/// * `None` - If the trick is incomplete or invalid
///
/// # Examples
/// ```
/// use estwhi_core::decide_trick_winner;
///
/// // Hearts led, Ace of Hearts (40) beats King of Hearts (52)
/// let trick = vec![Some(52), Some(40), Some(51), Some(41)];
/// let winner = decide_trick_winner(&trick, 1 /* Clubs trump */).unwrap();
/// assert_eq!(winner, 1); // Player 1 played the Ace
/// ```
pub fn decide_trick_winner(trick: &[Option<u32>], trump: u32) -> Option<usize> {
    let n = trick.len();
    if n == 0 || trick.iter().any(|c| c.is_none()) {
        return None;
    }
    let played: Vec<(usize, u32)> = trick
        .iter()
        .enumerate()
        .map(|(i, c)| (i, c.unwrap()))
        .collect();
    let lead_suit = suit_index_from_legacy_id(played[0].1);
    let trump_cards: Vec<(usize, u32)> = played
        .iter()
        .copied()
        .filter(|&(_, id)| suit_index_from_legacy_id(id) == trump)
        .collect();
    let candidates: Vec<(usize, u32)> = if !trump_cards.is_empty() {
        trump_cards
    } else {
        played
            .into_iter()
            .filter(|&(_, id)| suit_index_from_legacy_id(id) == lead_suit)
            .collect()
    };
    if candidates.is_empty() {
        return None;
    }
    let mut best = candidates[0];
    let mut best_val = rank_value(best.1);
    for &(p, id) in candidates.iter().skip(1) {
        let v = rank_value(id);
        if v > best_val {
            best = (p, id);
            best_val = v;
        }
    }
    Some(best.0)
}

/// Sorts a hand for display in the classic order.
///
/// # Sorting Order
/// - Suits are grouped: Clubs, Diamonds, Spades, Hearts
/// - Within each suit, ranks are ascending with Ace shown last (high)
/// - Example: 2♣, 3♣, K♣, A♣, 2♦, ... A♥
///
/// # Arguments
/// * `hand` - Mutable slice of legacy card IDs to sort in-place
///
/// # Examples
/// ```
/// use estwhi_core::sort_hand_for_display;
///
/// let mut hand = vec![1, 13, 12, 14, 26]; // Mixed cards
/// sort_hand_for_display(&mut hand);
/// // Now sorted: [12, 13, 1, 14, 26] = [Q♣, K♣, A♣, A♦, K♦]
/// ```
pub fn sort_hand_for_display(hand: &mut [u32]) {
    hand.sort_by_key(|&id| (suit_index_from_legacy_id(id), rank_value(id)));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn card_roundtrip() {
        for id in 1..=52u8 {
            let c = Card::from_legacy_id(id).unwrap();
            assert_eq!(c.legacy_id(), id);
        }
    }

    #[test]
    fn scoring_vanilla_soft() {
        // Zero calls get 10 bonus when matched
        let calls = vec![1, 0, 0, 0];
        let tricks = vec![1, 0, 0, 0];
        let d = score_hand(ScoreMode::Vanilla, false, &calls, &tricks, 1);
        assert_eq!(d, vec![11, 10, 10, 10]);
    }

    #[test]
    fn scoring_vanilla_hard_zero_bid_penalty() {
        // In hard_score mode when table under-calls, nobody gets bonus
        let calls = vec![0, 0, 0, 0];
        let tricks = vec![0, 0, 0, 0];
        let d = score_hand(ScoreMode::Vanilla, true, &calls, &tricks, 1);
        assert_eq!(d, vec![0, 0, 0, 0]); // total_calls(0) < dealt_cards(1), so no bonus
    }

    #[test]
    fn scoring_squared_exact() {
        let calls = vec![2, 0, 0, 0];
        let tricks = vec![2, 0, 0, 0];
        let d = score_hand(ScoreMode::Squared, false, &calls, &tricks, 2);
        assert_eq!(d, vec![14, 10, 10, 10]); // 10 + 2^2 for player 0; 10 + 0^2 for others
    }

    #[test]
    fn scoring_squared_miss() {
        let calls = vec![2, 0, 0, 0];
        let tricks = vec![1, 0, 0, 0];
        let d = score_hand(ScoreMode::Squared, false, &calls, &tricks, 2);
        assert_eq!(d, vec![1, 10, 10, 10]); // Player 0 missed → 1; others made 0 bid → 10 each
    }

    #[test]
    fn legality_follow_suit() {
        // Lead is hearts (id 40 = Ace of Hearts); player holds a heart
        let trick = vec![Some(40), None, None, None];
        let hand = vec![41, 5, 10];
        assert!(!is_legal_play(5, &trick, &hand));
        assert!(is_legal_play(41, &trick, &hand));
    }

    #[test]
    fn winner_no_trump_ace_high() {
        // Hearts led: 40 (Ace) should beat 52 (King) if both hearts
        let trick = vec![Some(52), Some(40), Some(51), Some(41)];
        // lead suit = hearts; highest rank = Ace(40)
        let w = decide_trick_winner(&trick, 1 /* Clubs trump (irrelevant) */).unwrap();
        assert_eq!(w, 1);
    }

    #[test]
    fn winner_with_trump() {
        // Diamonds led, but a spade (trump=3) is played
        let trick = vec![
            Some(20), /* D7 */
            Some(30), /* S4 */
            Some(22), /* D9 */
            Some(24), /* DJ */
        ];
        let w = decide_trick_winner(&trick, 3 /* Spades trump */).unwrap();
        assert_eq!(w, 1);
    }

    #[test]
    fn sort_hand_classic() {
        // Mixed suits and ranks; expect grouping by suit and Ace last within suit
        let mut h = vec![1, 13, 12, 14, 26, 25, 40, 39];
        sort_hand_for_display(&mut h);
        // Clubs (2..13, Ace last) -> [12,13,1] simplified sample yields [12,13,1]
        // Diamonds similarly; Hearts similarly.
        assert_eq!(h[0], 12); // C Queen
        assert_eq!(h[1], 13); // C King
        assert_eq!(h[2], 1); // C Ace at end
        assert_eq!(h.last().copied(), Some(40)); // H Ace at end of hearts block
    }

    #[test]
    fn invalid_card_ids() {
        assert!(Card::from_legacy_id(0).is_none());
        assert!(Card::from_legacy_id(53).is_none());
        assert!(Card::from_legacy_id(255).is_none());
    }

    #[test]
    fn deck_properties() {
        let mut rng = rand::thread_rng();
        let deck = Deck::new_shuffled(&mut rng);
        let cards = deck.cards();
        assert_eq!(cards.len(), 52);

        // Verify uniqueness
        let mut sorted = *cards;
        sorted.sort();
        for (i, &v) in sorted.iter().enumerate() {
            assert_eq!(v, (i as u8) + 1);
        }
    }

    #[test]
    fn empty_trick_winner() {
        // Empty trick
        assert!(decide_trick_winner(&[], 1).is_none());

        // Incomplete trick (has None)
        let trick = vec![Some(1), None, Some(2)];
        assert!(decide_trick_winner(&trick, 1).is_none());

        // Trick with all None
        let trick_all_none = vec![None, None, None, None];
        assert!(decide_trick_winner(&trick_all_none, 1).is_none());
    }

    #[test]
    fn trick_winner_no_candidates() {
        // This is theoretically impossible with valid cards if the list is non-empty,
        // because at least the first card sets the lead suit and is a candidate.
        // However, to cover the `candidates.is_empty()` check:
        // We need a scenario where `trump_cards` is empty AND `played.filter(lead)` is empty.
        // But `played[0]` DEFINES the lead suit, so it is always in the filter.
        // So `candidates` can only be empty if `played` is empty, which is covered by the initial check.
        // The only way to hit that line is if logic changes or `played` filtering is bugged.
        // But we can verify normal behavior.
        let trick = vec![Some(1)];
        assert_eq!(decide_trick_winner(&trick, 2).unwrap(), 0);
    }

    #[test]
    fn legal_play_edge_cases() {
        let hand = vec![1, 14, 27]; // A club, A diamond, A spade

        // No lead yet - anything goes
        let empty_trick = vec![None, None, None, None];
        assert!(is_legal_play(1, &empty_trick, &hand));
        assert!(is_legal_play(14, &empty_trick, &hand));

        // Lead is Hearts (40), we have none - anything goes
        let hearts_led = vec![Some(40), None, None, None];
        assert!(is_legal_play(1, &hearts_led, &hand)); // Discard Club
        assert!(is_legal_play(14, &hearts_led, &hand)); // Discard Diamond

        // Lead is Clubs (13), we have a Club (1) - must follow
        let clubs_led = vec![Some(13), None, None, None];
        assert!(is_legal_play(1, &clubs_led, &hand)); // Follow suit
        assert!(!is_legal_play(14, &clubs_led, &hand)); // Revoke (Diamond)
    }
}
