use crate::board::{Board, Cell};
use crate::board::BOARD_SIZE;
use crate::piece::Piece;
use crate::piece::{PieceType, Color};


#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GameState {
    Ongoing,
    Check,
    Checkmate,
    Stalemate,
    Draw,
}
#[derive(Clone)]
pub struct Game {
    pub board: Board,
    pub current_player: Color,
    pub state: GameState,
    
}

impl Game {
    pub fn new() -> Self {
        // initialize new game
        Self {
            board: Board::new(),
            current_player: Color::Black, 
            state: GameState::Ongoing,
        }
    }
    //swap player after each turn
    pub fn switch_player(&mut self) {
        self.current_player = match self.current_player {
            Color::White => Color::Black,
            Color::Black => Color::White,
        };
    }
    //Links to valid_moves in Piece to determine individual piece moves are correct
    //Handles bounds checking external to the piece.rs rules
    pub fn is_valid_move(&self, from: (usize, usize), to: (usize, usize)) -> bool {
        if !self.is_within_bounds(from) || !self.is_within_bounds(to) {
            return false;
        }
    
        if let Some(piece) = self.board.get_piece_at(from) {
            if piece.color != self.current_player {
                return false;
            }
    
            let valid_moves = piece.valid_moves(from, &self.board);
            if valid_moves.contains(&to) {
                return true;
            }
        }
    
        false
    }
    //prevents pieces from moving off the board
    fn is_within_bounds(&self, position: (usize, usize)) -> bool {
        position.0 < BOARD_SIZE && position.1 < BOARD_SIZE
    }
    pub fn is_in_check(&self) -> bool {
        // Find the king's position
        let king_pos = self.find_king(self.current_player);
        if king_pos.is_none() {
            return false; // This should not happen in a regular game
        }
        let king_pos = king_pos.unwrap();

        // Check if any of the opponent's pieces can move to the king's position
        for i in 0..BOARD_SIZE {
            for j in 0..BOARD_SIZE {
                if let Some(piece) = self.board.get_piece_at((i, j)) {
                    if piece.color != self.current_player {
                        let valid_moves = piece.valid_moves((i, j), &self.board);
                        if valid_moves.contains(&king_pos) {
                            return true;
                        }
                    }
                }
            }
        }

        false
    }
    // Helper function to find the king's position for check and checkmate
    fn find_king(&self, color: Color) -> Option<(usize, usize)> {
        for i in 0..BOARD_SIZE {
            for j in 0..BOARD_SIZE {
                if let Some(piece) = self.board.get_piece_at((i, j)) {
                    if piece.color == color && piece.piece_type == PieceType::King {
                        return Some((i, j));
                    }
                }
            }
        }
        None
    }
    pub fn is_in_checkmate(&self) -> bool {
        if !self.is_in_check() {
            return false;
        }

        // Check if there are any legal moves left that can get the king out of check
        for i in 0..BOARD_SIZE {
            for j in 0..BOARD_SIZE {
                if let Some(piece) = self.board.get_piece_at((i, j)) {
                    if piece.color == self.current_player {
                        let valid_moves = piece.valid_moves((i, j), &self.board);
                        for &move_pos in &valid_moves {
                            //initialize a temp board with all the pieces in the same place to see if there are moves to save the king
                            // Make the move and check if the king is still in check
                            let mut temp_board = self.board.clone();
                            temp_board.make_move((i, j), move_pos);
                            let temp_game = Game { board: temp_board, current_player: self.current_player, state:self.state};
                            if !temp_game.is_in_check() {
                                return false;
                            }
                        }
                    }
                }
            }
        }

        true
    }
    //pawn promotion and castling were the only special moves implemented but these moves can be made the same way as regular moves
    pub fn handle_special_moves(&mut self, from: (usize, usize), to: (usize, usize)) {
        if let Some(piece) = self.board.get_piece_at(from) {
            match piece.piece_type {
                PieceType::King => self.handle_castling(from, to),
                PieceType::Pawn => {
                    //self.handle_en_passant(from, to);
                    self.handle_pawn_promotion(to);
                }
                _ => {}
            }
        }
    }
    fn handle_castling(&mut self, from: (usize, usize), to: (usize, usize)) {
        let y = from.1;
        let (rook_from, rook_to, king_to) = if to.0 > from.0 {
            // King-side castling
            ((BOARD_SIZE - 1, y), (from.0 + 1, y), (to.0, y))
        } else {
            // Queen-side castling
            ((0, y), (from.0 - 1, y), (to.0, y))
        };
        if !self.can_castle(from, to) {
            return;
        }

        // Move the king
        if let Some(king) = self.board.get_piece_at(from) {
            self.board.set_piece_at(king_to, king);
            self.board.remove_piece_at(from);
        }

        // Move the rook
        if let Some(rook) = self.board.get_piece_at(rook_from) {
            self.board.set_piece_at(rook_to, rook);
            self.board.remove_piece_at(rook_from);
        }

    
    }
    //helper function for handle_castling 
    fn can_castle(&self, from: (usize, usize), to: (usize, usize)) -> bool {
        // Check if the piece at 'from' is a king and it hasn't moved
        if let Some(piece) = self.board.get_piece_at(from) {
            if piece.piece_type != PieceType::King || piece.has_moved {
                return false;
            }
        } else {
            return false;
        }
        
        // Determine if it's king-side or queen-side castling
        let king_side = to.0 > from.0;
        let y = from.1;
        let (rook_x, clear_path_start, clear_path_end) = if king_side {
            // King-side castling
            (7, from.0 + 1, 6)
        } else {
            // Queen-side castling
            (0, 3, from.0 - 1)
        };

        // Check if the rook has moved
        if let Some(rook) = self.board.get_piece_at((rook_x, y)) {
            if rook.piece_type != PieceType::Rook || rook.has_moved {
                return false;
            }
        } else {
            return false;
        }

        // Check if the path between the king and rook is clear
        for x in clear_path_start..=clear_path_end {
            if self.board.get_piece_at((x, y)).is_some() {
                return false;
            }
        }
        let path: Vec<(usize, usize)> = (clear_path_start..=clear_path_end).map(|x| (x, y)).collect();
        // Check if the king is in check or the path is under attack
        if self.is_in_check() || self.is_path_under_attack(&path, self.current_player) {
            return false;
        }

        true
    }
    fn is_path_under_attack(&self, path: &[(usize, usize)], player_color: Color) -> bool {
        for &square in path {
            if self.is_square_under_attack(square, player_color) {
                return true;
            }
        }
        false
    }

    fn is_square_under_attack(&self, square: (usize, usize), player_color: Color) -> bool {
        // Check if any of the opponent's pieces can move to 'square'
        for x in 0..BOARD_SIZE {
            for y in 0..BOARD_SIZE {
                if let Some(piece) = self.board.get_piece_at((x, y)) {
                    if piece.color != player_color {
                        let potential_moves = piece.valid_moves((x, y), &self.board);
                        if potential_moves.contains(&square) {
                            return true;
                        }
                    }
                }
            }
        }
        false
    }
    fn handle_pawn_promotion(&mut self, to: (usize, usize)) {
        if self.is_pawn_promotion(to) {
            // Promote the pawn to Queen.
            let promoted_piece = Piece {
                piece_type: PieceType::Queen, 
                color: self.current_player,
                has_moved: true
            };
            self.board.set_piece_at(to, promoted_piece);
        }
    }
    fn is_pawn_promotion(&self, to: (usize, usize)) -> bool {
        if let Some(piece) = self.board.get_piece_at(to) {
            // Check if the piece is a pawn
            if piece.piece_type == PieceType::Pawn {
                // Check if the pawn has reached the opposite end of the board
                match piece.color {
                    Color::White => to.1 == BOARD_SIZE - 1, // White pawns promote at the last row
                    Color::Black => to.1 == 0,              // Black pawns promote at the first row
                }
            } else {
                false
            }
        } else {
            false
        }
    }
  
}