use crate::game_config::UiConfig;
use crate::game_state::GameState;
use estwhi_core::{
    ai::select_card_to_play, decide_trick_winner, score_hand, state::next_player_to_act,
};
use rand::Rng;

#[derive(Debug, PartialEq)]
pub enum StepResult {
    /// Human must act (click card)
    WaitHuman,
    /// AI played a card
    AiMoved { seat: usize, card: u32 },
    /// Trick is full, winner decided
    TrickComplete { winner: usize, winner_1based: u32 },
    /// Hand is over (all cards played), scores updated
    HandComplete {
        final_scores: Vec<u32>,
        game_over: bool,
    },
    /// No operation (e.g. invalid state)
    NoOp,
}

/// Advances the game state by one step (one AI move, or checking conditions).
/// Returns what happened.
pub fn advance_game_step(
    game: &mut GameState,
    config: &UiConfig,
    rng: &mut impl Rng,
) -> StepResult {
    let n = config.num_players as usize;
    if n == 0 || game.trick.len() != n {
        return StepResult::NoOp;
    }

    // 1. Check if trick is full (all cards played)
    if game.trick.iter().filter(|c| c.is_some()).count() == n {
        // Trick is complete. Decide winner if not already decided?
        // In main.rs, "decide_winner_and_setup" runs when trick is full.
        // We assume this function is called repeatedly.

        let trump = game.trump;
        if let Some(winner) = decide_trick_winner(&game.trick, trump) {
            // Logic from decide_winner_and_setup
            // Note: main.rs increments tricks immediately but does NOT clear the trick yet.
            // It waits for user interaction (MsgBox or Click).
            // However, to make progress, we must signal TrickComplete only ONCE per trick.
            // Check if we already processed this trick?
            // game.last_winner is updated when winner is decided.
            // But game.last_winner persists across tricks.
            // We can check if `game.tricks` sum equals `game.dealt_cards - game.cards_remaining` + 1?
            // Or simpler: The caller handles the loop. If we return TrickComplete, the caller should pause/finalize.

            // BUT, this function updates state. If we update state, next call will see updated state.
            // If we increment tricks here, we must ensure we don't do it twice.
            // Ideally this function should be idempotent or the caller stops calling it when TrickComplete.

            // Let's perform the update here.
            game.tricks[winner] += 1;
            let w1 = (winner + 1) as u32;
            game.last_winner = Some(w1);
            game.current_player = winner;

            return StepResult::TrickComplete {
                winner,
                winner_1based: w1,
            };
        }
        return StepResult::NoOp;
    }

    // 2. Check human turn
    if game.waiting_for_human {
        return StepResult::WaitHuman;
    }

    // 3. Determine next actor
    if let Some(cur) = next_player_to_act(game.start_player, &game.trick) {
        if cur == 0 {
            game.current_player = 0;
            game.waiting_for_human = true;
            return StepResult::WaitHuman;
        }

        // AI Turn
        let hand = game.hands[cur].clone();
        let trick = game.trick.clone();

        if hand.is_empty() {
            return StepResult::NoOp;
        }

        let card = select_card_to_play(&hand, &trick, rng);

        // Update state (play card)
        let real = &mut game.hands[cur];
        if let Some(pos) = real.iter().position(|&c| c == card) {
            real.remove(pos);
        }
        game.trick[cur] = Some(card);

        // Advance current player pointer immediately for UI/Debug
        if let Some(nextp) = next_player_to_act(game.start_player, &game.trick) {
            game.current_player = nextp;
        }

        return StepResult::AiMoved { seat: cur, card };
    }

    StepResult::NoOp
}

/// Finalizes the current trick (clears it) and prepares for next trick or scores hand.
/// Corresponds to `finalize_trick_and_setup_next` in main.rs.
pub fn finalize_trick(game: &mut GameState, config: &UiConfig) -> StepResult {
    // Clear trick
    for c in game.trick.iter_mut() {
        *c = None;
    }

    if game.cards_remaining > 0 {
        game.cards_remaining -= 1;
    }

    game.waiting_for_continue = false;

    if game.cards_remaining == 0 {
        // Hand complete - score it
        let n = config.num_players as usize;
        let deltas = score_hand(
            config.score_mode,
            false,
            &game.calls,
            &game.tricks,
            game.dealt_cards,
        );

        for (i, &delta) in deltas.iter().enumerate().take(n) {
            game.scores[i] = game.scores[i].saturating_add(delta);
        }

        let max = config.max_cards;
        let total_rounds = (max * 2).saturating_sub(1);
        let final_round = game.round_no >= total_rounds;

        StepResult::HandComplete {
            final_scores: game.scores.clone(),
            game_over: final_round,
        }
    } else {
        // Setup next trick
        if let Some(w) = game.last_winner {
            game.start_player = w;
        }

        if let Some(nextp) = next_player_to_act(game.start_player, &game.trick) {
            game.current_player = nextp;
            game.waiting_for_human = nextp == 0;
        }

        StepResult::NoOp // Just continued
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    fn make_test_game() -> (GameState, UiConfig) {
        let g = GameState {
            round_no: 1,
            trump: 1,        // Clubs
            start_player: 1, // Player 1 starts
            dealt_cards: 1,
            cards_remaining: 1,
            trick: vec![None, None, None, None],
            hands: vec![vec![1], vec![2], vec![3], vec![4]], // A, 2, 3, 4 of Clubs
            calls: vec![0; 4],
            tricks: vec![0; 4],
            scores: vec![0; 4],
            ..Default::default()
        };

        let c = UiConfig {
            num_players: 4,
            max_cards: 1,
            score_mode: estwhi_core::ScoreMode::Vanilla,
            next_notify: crate::game_config::NextNotify::Dialog,
            confirm_exit: true,
            cheat_cards: false,
        };
        (g, c)
    }

    #[test]
    fn test_human_turn_start() {
        let (mut g, c) = make_test_game();
        g.start_player = 1; // Player 1 (human) starts
        let mut rng = StdRng::seed_from_u64(42);

        let res = advance_game_step(&mut g, &c, &mut rng);
        assert_eq!(res, StepResult::WaitHuman);
        assert!(g.waiting_for_human);
    }

    #[test]
    fn test_ai_turn() {
        let (mut g, c) = make_test_game();
        g.start_player = 2; // Player 2 starts
        let mut rng = StdRng::seed_from_u64(42);

        let res = advance_game_step(&mut g, &c, &mut rng);
        match res {
            StepResult::AiMoved { seat, card } => {
                assert_eq!(seat, 1); // Index 1 = Player 2
                assert_eq!(card, 2); // 2 of Clubs
            }
            _ => panic!("Expected AiMoved"),
        }

        assert_eq!(g.trick[1], Some(2));
        assert!(g.hands[1].is_empty());
    }

    #[test]
    fn test_trick_completion() {
        let (mut g, c) = make_test_game();
        // Simulate full trick: P1=A(1), P2=2(2), P3=3(3), P4=4(4). All Clubs.
        // A beats all.
        g.trick = vec![Some(1), Some(2), Some(3), Some(4)];
        let mut rng = StdRng::seed_from_u64(42);

        let res = advance_game_step(&mut g, &c, &mut rng);
        match res {
            StepResult::TrickComplete {
                winner,
                winner_1based,
            } => {
                assert_eq!(winner, 0); // P1
                assert_eq!(winner_1based, 1);
            }
            _ => panic!("Expected TrickComplete"),
        }

        assert_eq!(g.tricks[0], 1);
    }

    #[test]
    fn test_finalize_hand() {
        let (mut g, c) = make_test_game();
        g.cards_remaining = 1;
        g.calls = vec![1, 0, 0, 0];
        g.tricks = vec![1, 0, 0, 0]; // P1 matched

        let res = finalize_trick(&mut g, &c);
        match res {
            StepResult::HandComplete {
                final_scores,
                game_over,
            } => {
                assert!(game_over); // dealt=1, max=1 -> round 1 is final (limit 1)
                assert_eq!(final_scores[0], 11); // 1 trick + 10 bonus
            }
            _ => panic!("Expected HandComplete"),
        }

        assert_eq!(g.cards_remaining, 0);
        assert!(g.trick.iter().all(|c| c.is_none()));
    }
}
