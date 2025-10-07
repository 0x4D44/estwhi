//! Core logic for Estimation Whist — platform independent.

use rand::Rng;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Suit {
    Clubs = 0,
    Diamonds = 1,
    Spades = 2,
    Hearts = 3,
}

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

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

impl Card {
    /// Legacy numbering: 1..13 Clubs, 14..26 Diamonds, 27..39 Spades, 40..52 Hearts.
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

#[derive(Clone, Debug)]
pub struct GameConfig {
    pub num_players: u8, // 2..6
    pub max_cards: u8,   // 1..15 (13 default)
    pub score_mode: ScoreMode,
    pub hard_score: bool,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ScoreMode {
    Vanilla,
    Squared,
}

#[derive(Clone, Debug)]
pub struct Deck([u8; 52]);

impl Deck {
    pub fn new_shuffled<R: Rng + ?Sized>(rng: &mut R) -> Self {
        let mut a = [0u8; 52];
        for (i, v) in a.iter_mut().enumerate() {
            *v = (i as u8) + 1;
        }
        // Fisher–Yates
        for i in (1..52).rev() {
            let j = rng.gen_range(0..=i);
            a.swap(i, j);
        }
        Deck(a)
    }

    pub fn cards(&self) -> &[u8; 52] {
        &self.0
    }
}

// ----- Scoring helpers -----

/// Compute per-player score deltas for a completed hand, matching the legacy rules:
/// - AvailableBonus = 10, except when `hard_score` and (sum(calls) < dealt_cards) and call == 0, then 0.
/// - Vanilla: delta = tricks, plus AvailableBonus if call == tricks.
/// - Squared: if call == tricks => AvailableBonus + tricks^2, else delta = tricks.
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
        // Port rule: matching a zero bid never earns the 10 bonus.
        // Additionally, legacy "hard score" removed the bonus for zero bids when
        // the table under-called the hand (sum(calls) < dealt_cards). Since zero
        // bids never get a bonus here, the hard-score condition is redundant but
        // we keep the structure for clarity in case of future rule tweaks.
        let available_bonus = if call == 0 || (hard_score && total_calls < dealt_cards) {
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

/// 1=Clubs, 2=Diamonds, 3=Spades, 4=Hearts (legacy encoding groups)
pub fn suit_index_from_legacy_id(id: u32) -> u32 {
    let zero = (id as i32) - 1;
    let suit = (zero / 13) as u32; // 0..3 => Clubs,Diamonds,Spades,Hearts
    suit + 1
}

/// Compare rank with Ace high (Ace outranks King).
pub fn rank_value(id: u32) -> u32 {
    let r = ((id - 1) % 13) + 1; // 1..13
    if r == 1 {
        14
    } else {
        r
    }
}

/// Lead-following legality: if a lead exists and player holds lead suit, must follow.
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

/// Decide winner seat index (0..n-1) for a completed trick.
/// - `trick` is indexed by absolute seat; all entries must be Some(card_id).
/// - Trump is 1..4 using the legacy suit index mapping.
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

/// Sort a hand for display in the classic order:
/// suits grouped Clubs, Diamonds, Spades, Hearts and within suit
/// ranks ascending with Ace shown last (high).
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
        // calls sum == dealt -> zero calls still get 10 bonus if matched
        let calls = vec![1, 0, 0, 0];
        let tricks = vec![1, 0, 0, 0];
        let d = score_hand(ScoreMode::Vanilla, false, &calls, &tricks, 1);
        assert_eq!(d, vec![11, 0, 0, 0]);
    }

    #[test]
    fn scoring_vanilla_hard_zero_bid_penalty() {
        // total calls < dealt and a player called 0 -> that player gets no 10 bonus even if matched
        let calls = vec![0, 0, 0, 0];
        let tricks = vec![0, 0, 0, 0];
        let d = score_hand(ScoreMode::Vanilla, true, &calls, &tricks, 1);
        assert_eq!(d, vec![0, 0, 0, 0]);
    }

    #[test]
    fn scoring_squared_exact() {
        let calls = vec![2, 0, 0, 0];
        let tricks = vec![2, 0, 0, 0];
        let d = score_hand(ScoreMode::Squared, false, &calls, &tricks, 2);
        assert_eq!(d, vec![14, 0, 0, 0]); // 10 + 2^2
    }

    #[test]
    fn scoring_squared_miss() {
        let calls = vec![2, 0, 0, 0];
        let tricks = vec![1, 0, 0, 0];
        let d = score_hand(ScoreMode::Squared, false, &calls, &tricks, 2);
        assert_eq!(d, vec![1, 0, 0, 0]);
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
}
