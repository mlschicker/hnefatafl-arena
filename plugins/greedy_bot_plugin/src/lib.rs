use hnefatafl_arena::{Bot, GameState, Move, Player, Piece, Position};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;

/// A greedy bot that tries to capture pieces and uses pondering
pub struct GreedyBotPlugin {
    name: String,
    pondering: Arc<AtomicBool>,
    last_state: Option<GameState>,
}

impl Default for GreedyBotPlugin {
    fn default() -> Self {
        Self {
            name: "GreedyPlugin".to_string(),
            pondering: Arc::new(AtomicBool::new(false)),
            last_state: None,
        }
    }
}

impl GreedyBotPlugin {
    fn evaluate_move(&self, state: &GameState, mv: Move) -> i32 {
        let mut temp_state = state.clone();
        let _ = temp_state.make_move(mv);

        // Count pieces for each side
        let mut attacker_count = 0;
        let mut defender_count = 0;
        let mut king_alive = false;

        for row in 0..state.board_size() {
            for col in 0..state.board_size() {
                let pos = Position::new(row, col);
                match temp_state.get_piece(pos) {
                    Some(Piece::Attacker) => attacker_count += 1,
                    Some(Piece::Defender) => defender_count += 1,
                    Some(Piece::King) => king_alive = true,
                    None => {}
                }
            }
        }

        // Simple evaluation based on piece count
        match state.current_player() {
            Player::Attackers => {
                if !king_alive {
                    return 1000; // King captured is winning
                }
                attacker_count - defender_count * 2
            }
            Player::Defenders => {
                if !king_alive {
                    return -1000; // King lost is losing
                }
                defender_count * 2 - attacker_count
            }
        }
    }
}

impl Bot for GreedyBotPlugin {
    fn name(&self) -> &str {
        &self.name
    }

    fn get_move(&mut self, state: &GameState, _time_limit: Duration) -> Option<Move> {
        let moves = state.legal_moves();
        if moves.is_empty() {
            return None;
        }

        // Find the move with the best evaluation
        moves
            .into_iter()
            .max_by_key(|&mv| self.evaluate_move(state, mv))
    }

    fn game_start(&mut self, _player: Player) {
        // Reset pondering state
        self.pondering.store(false, Ordering::Relaxed);
        self.last_state = None;
    }

    fn notify_move(&mut self, _mv: Move) {
        // Could update internal state here
    }

    fn game_end(&mut self) {
        self.pondering.store(false, Ordering::Relaxed);
    }

    fn opponent_thinking(&mut self, state: &GameState) {
        // Start pondering - in a real implementation, this would run in a background thread
        self.pondering.store(true, Ordering::Relaxed);
        self.last_state = Some(state.clone());
        
        // Example: Pre-compute evaluations for possible opponent moves
        // In a real bot, you'd do this in a loop checking the pondering flag
        let moves = state.legal_moves();
        for mv in moves.iter().take(5) {
            if !self.pondering.load(Ordering::Relaxed) {
                break;
            }
            // Pre-compute evaluation
            let _ = self.evaluate_move(state, *mv);
        }
    }

    fn stop_pondering(&mut self) {
        self.pondering.store(false, Ordering::Relaxed);
    }
}

// Export the bot plugin using the macro
hnefatafl_arena::export_bot!(GreedyBotPlugin);
