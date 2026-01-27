use serde::{Deserialize, Serialize};
use std::fmt;
use thiserror::Error;

/// Board size constants
pub const COPENHAGEN_SIZE: usize = 11;
pub const BRANDUBH_SIZE: usize = 7;
pub const MAX_BOARD_SIZE: usize = 11;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Variant {
    Copenhagen, // 11x11, traditional Hnefatafl
    Brandubh,   // 7x7, Irish variant
}

impl Variant {
    pub fn board_size(&self) -> usize {
        match self {
            Variant::Copenhagen => COPENHAGEN_SIZE,
            Variant::Brandubh => BRANDUBH_SIZE,
        }
    }

    pub fn name(&self) -> &str {
        match self {
            Variant::Copenhagen => "Copenhagen Hnefatafl",
            Variant::Brandubh => "Brandubh",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Piece {
    Attacker,
    Defender,
    King,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Player {
    Attackers,
    Defenders,
}

impl Player {
    pub fn opponent(&self) -> Player {
        match self {
            Player::Attackers => Player::Defenders,
            Player::Defenders => Player::Attackers,
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Position {
    pub row: usize,
    pub col: usize,
}

impl Position {
    pub fn new(row: usize, col: usize) -> Self {
        Position { row, col }
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.row, self.col)
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Move {
    pub from: Position,
    pub to: Position,
}

impl Move {
    pub fn new(from: Position, to: Position) -> Self {
        Move { from, to }
    }
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} -> {}", self.from, self.to)
    }
}

#[derive(Debug, Error)]
pub enum GameError {
    #[error("Invalid move: {0}")]
    InvalidMove(String),
    #[error("Game already over")]
    GameOver,
    #[error("Not your turn")]
    NotYourTurn,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum GameResult {
    AttackersWin,
    DefendersWin,
    Draw,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameState {
    board: [[Option<Piece>; MAX_BOARD_SIZE]; MAX_BOARD_SIZE],
    variant: Variant,
    board_size: usize,
    current_player: Player,
    king_position: Option<Position>,
    move_count: usize,
    result: Option<GameResult>,
}

impl GameState {
    /// Create a new game with the specified variant
    pub fn new(variant: Variant) -> Self {
        let mut state = GameState {
            board: [[None; MAX_BOARD_SIZE]; MAX_BOARD_SIZE],
            variant,
            board_size: variant.board_size(),
            current_player: Player::Attackers,
            king_position: None,
            move_count: 0,
            result: None,
        };

        match variant {
            Variant::Copenhagen => state.setup_copenhagen(),
            Variant::Brandubh => state.setup_brandubh(),
        }

        state
    }

    /// Create a new game with Copenhagen Hnefatafl (default)
    pub fn new_copenhagen() -> Self {
        Self::new(Variant::Copenhagen)
    }

    /// Create a new game with Brandubh variant
    pub fn new_brandubh() -> Self {
        Self::new(Variant::Brandubh)
    }

    /// Setup Copenhagen Hnefatafl (11x11)
    fn setup_copenhagen(&mut self) {
        let board_size = COPENHAGEN_SIZE;

        // Place king in center
        let center = board_size / 2;
        self.board[center][center] = Some(Piece::King);
        self.king_position = Some(Position::new(center, center));

        // Place defenders around king (cross pattern)
        let defenders = [
            (center - 1, center),
            (center + 1, center),
            (center, center - 1),
            (center, center + 1),
            (center - 2, center),
            (center + 2, center),
            (center, center - 2),
            (center, center + 2),
        ];

        for &(r, c) in &defenders {
            self.board[r][c] = Some(Piece::Defender);
        }

        // Place attackers on edges (T-shape on each side)
        let attackers = [
            // Top
            (0, 3),
            (0, 4),
            (0, 5),
            (0, 6),
            (0, 7),
            (1, 5),
            // Bottom
            (10, 3),
            (10, 4),
            (10, 5),
            (10, 6),
            (10, 7),
            (9, 5),
            // Left
            (3, 0),
            (4, 0),
            (5, 0),
            (6, 0),
            (7, 0),
            (5, 1),
            // Right
            (3, 10),
            (4, 10),
            (5, 10),
            (6, 10),
            (7, 10),
            (5, 9),
        ];

        for &(r, c) in &attackers {
            self.board[r][c] = Some(Piece::Attacker);
        }
    }

    /// Setup Brandubh (7x7)
    fn setup_brandubh(&mut self) {
        let board_size = BRANDUBH_SIZE;
        let center = board_size / 2; // 3 for 7x7

        // Place king in center
        self.board[center][center] = Some(Piece::King);
        self.king_position = Some(Position::new(center, center));

        // Place 4 defenders around king
        let defenders = [
            (center - 1, center), // Above
            (center + 1, center), // Below
            (center, center - 1), // Left
            (center, center + 1), // Right
        ];

        for &(r, c) in &defenders {
            self.board[r][c] = Some(Piece::Defender);
        }

        // Place 8 attackers on edges (2 on each side)
        let attackers = [
            // Top
            (0, 3),
            (1, 3),
            // Bottom
            (5, 3),
            (6, 3),
            // Left
            (3, 0),
            (3, 1),
            // Right
            (3, 5),
            (3, 6),
        ];

        for &(r, c) in &attackers {
            self.board[r][c] = Some(Piece::Attacker);
        }
    }

    pub fn variant(&self) -> Variant {
        self.variant
    }

    pub fn board_size(&self) -> usize {
        self.board_size
    }

    pub fn current_player(&self) -> Player {
        self.current_player
    }

    pub fn result(&self) -> Option<&GameResult> {
        self.result.as_ref()
    }

    pub fn is_game_over(&self) -> bool {
        self.result.is_some()
    }

    pub fn move_count(&self) -> usize {
        self.move_count
    }

    pub fn get_piece(&self, pos: Position) -> Option<Piece> {
        if pos.row < self.board_size && pos.col < self.board_size {
            self.board[pos.row][pos.col]
        } else {
            None
        }
    }

    /// Check if a position is a corner (throne)
    fn is_corner(&self, pos: Position) -> bool {
        (pos.row == 0 && pos.col == 0)
            || (pos.row == 0 && pos.col == self.board_size - 1)
            || (pos.row == self.board_size - 1 && pos.col == 0)
            || (pos.row == self.board_size - 1 && pos.col == self.board_size - 1)
    }

    /// Check if a position is the throne (center)
    fn is_throne(&self, pos: Position) -> bool {
        let center = self.board_size / 2;
        pos.row == center && pos.col == center
    }

    /// Get all legal moves for the current player
    pub fn legal_moves(&self) -> Vec<Move> {
        if self.is_game_over() {
            return Vec::new();
        }

        let mut moves = Vec::new();

        for row in 0..self.board_size {
            for col in 0..self.board_size {
                let pos = Position::new(row, col);
                if let Some(piece) = self.get_piece(pos) {
                    if self.piece_belongs_to_current_player(piece) {
                        moves.extend(self.legal_moves_for_piece(pos));
                    }
                }
            }
        }

        moves
    }

    fn piece_belongs_to_current_player(&self, piece: Piece) -> bool {
        match (piece, self.current_player) {
            (Piece::Attacker, Player::Attackers) => true,
            (Piece::Defender | Piece::King, Player::Defenders) => true,
            _ => false,
        }
    }

    fn legal_moves_for_piece(&self, from: Position) -> Vec<Move> {
        let mut moves = Vec::new();
        let piece = self.get_piece(from).unwrap();

        // Try all four directions
        let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];

        for &(dr, dc) in &directions {
            let mut r = from.row as i32;
            let mut c = from.col as i32;

            loop {
                r += dr;
                c += dc;

                if r < 0 || r >= self.board_size as i32 || c < 0 || c >= self.board_size as i32 {
                    break;
                }

                let to = Position::new(r as usize, c as usize);

                // Can't move onto another piece
                if self.get_piece(to).is_some() {
                    break;
                }

                // Only king can move to throne or corners
                if piece != Piece::King {
                    if self.is_throne(to) || self.is_corner(to) {
                        break;
                    }
                }

                moves.push(Move::new(from, to));
            }
        }

        moves
    }

    /// Make a move and update the game state
    pub fn make_move(&mut self, mv: Move) -> Result<(), GameError> {
        if self.is_game_over() {
            return Err(GameError::GameOver);
        }

        // Validate the move
        if !self.legal_moves().contains(&mv) {
            return Err(GameError::InvalidMove(format!("Move {} is not legal", mv)));
        }

        // Move the piece
        let piece = self.board[mv.from.row][mv.from.col].unwrap();
        self.board[mv.from.row][mv.from.col] = None;
        self.board[mv.to.row][mv.to.col] = Some(piece);

        // Update king position
        if piece == Piece::King {
            self.king_position = Some(mv.to);
        }

        // Check for captures
        self.check_captures(mv.to);

        // Check win conditions
        self.check_game_end();

        // Switch player
        self.current_player = self.current_player.opponent();
        self.move_count += 1;

        Ok(())
    }

    fn check_captures(&mut self, moved_to: Position) {
        let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];

        for &(dr, dc) in &directions {
            let target_r = moved_to.row as i32 + dr;
            let target_c = moved_to.col as i32 + dc;

            if target_r < 0
                || target_r >= self.board_size as i32
                || target_c < 0
                || target_c >= self.board_size as i32
            {
                continue;
            }

            let target = Position::new(target_r as usize, target_c as usize);

            if let Some(target_piece) = self.get_piece(target) {
                // Check if we can capture this piece
                if self.can_capture(moved_to, target) {
                    self.board[target.row][target.col] = None;

                    // If king was captured, update king position
                    if target_piece == Piece::King {
                        self.king_position = None;
                    }
                }
            }
        }
    }

    fn can_capture(&self, attacker: Position, target: Position) -> bool {
        let attacker_piece = self.get_piece(attacker).unwrap();
        let target_piece = self.get_piece(target).unwrap();

        // Can't capture your own pieces
        match (attacker_piece, target_piece) {
            (Piece::Attacker, Piece::Attacker) => return false,
            (Piece::Defender, Piece::Defender) => return false,
            (Piece::King, Piece::Defender) => return false,
            (Piece::Defender, Piece::King) => return false,
            _ => {}
        }

        // King needs to be surrounded on all 4 sides
        if target_piece == Piece::King {
            return self.is_king_surrounded(target);
        }

        // Regular pieces need hostile piece on opposite side
        let dr = target.row as i32 - attacker.row as i32;
        let dc = target.col as i32 - attacker.col as i32;

        let opposite_r = target.row as i32 + dr;
        let opposite_c = target.col as i32 + dc;

        if opposite_r < 0
            || opposite_r >= self.board_size as i32
            || opposite_c < 0
            || opposite_c >= self.board_size as i32
        {
            return false;
        }

        let opposite = Position::new(opposite_r as usize, opposite_c as usize);

        // Throne and corners act as hostile
        if self.is_throne(opposite) || self.is_corner(opposite) {
            return true;
        }

        if let Some(opposite_piece) = self.get_piece(opposite) {
            // Check if opposite piece is hostile to target
            match (target_piece, opposite_piece) {
                (Piece::Attacker, Piece::Defender | Piece::King) => true,
                (Piece::Defender, Piece::Attacker) => true,
                _ => false,
            }
        } else {
            false
        }
    }

    fn is_king_surrounded(&self, king_pos: Position) -> bool {
        let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];

        for &(dr, dc) in &directions {
            let r = king_pos.row as i32 + dr;
            let c = king_pos.col as i32 + dc;

            if r < 0 || r >= self.board_size as i32 || c < 0 || c >= self.board_size as i32 {
                continue;
            }

            let pos = Position::new(r as usize, c as usize);

            // Must be surrounded by attackers or throne/corners
            if self.is_throne(pos) || self.is_corner(pos) {
                continue;
            }

            if let Some(piece) = self.get_piece(pos) {
                if piece != Piece::Attacker {
                    return false;
                }
            } else {
                return false;
            }
        }

        true
    }

    fn check_game_end(&mut self) {
        // Defenders win if king reaches a corner
        if let Some(king_pos) = self.king_position {
            if self.is_corner(king_pos) {
                self.result = Some(GameResult::DefendersWin);
                return;
            }
        } else {
            // King captured - attackers win
            self.result = Some(GameResult::AttackersWin);
            return;
        }

        // Check for draw (no legal moves)
        // This will be checked after switching player
    }

    /// Get a string representation of the board
    pub fn display_board(&self) -> String {
        let mut result = String::new();
        result.push_str(&format!("[{}]\n", self.variant.name()));
        result.push_str("   ");
        for col in 0..self.board_size {
            result.push_str(&format!("{:2} ", col));
        }
        result.push('\n');

        for row in 0..self.board_size {
            result.push_str(&format!("{:2} ", row));
            for col in 0..self.board_size {
                let pos = Position::new(row, col);
                let c = if self.is_corner(pos) {
                    'X'
                } else if self.is_throne(pos) && self.get_piece(pos).is_none() {
                    'T'
                } else {
                    match self.get_piece(pos) {
                        Some(Piece::Attacker) => 'A',
                        Some(Piece::Defender) => 'D',
                        Some(Piece::King) => 'K',
                        None => '.',
                    }
                };
                result.push_str(&format!(" {} ", c));
            }
            result.push('\n');
        }

        result
    }
}

impl Default for GameState {
    fn default() -> Self {
        Self::new_copenhagen()
    }
}
