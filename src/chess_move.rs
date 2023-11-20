use crate::piece::Piece;
use crate::piece::PieceType;
pub struct Move {
    from: (usize, usize),
    to: (usize, usize),
    piece_moved: Piece,
    piece_captured: Option<Piece>,
    // Fields for special moves
    is_castling_move: bool,
    is_en_passant_move: bool,
    promoted_to: Option<PieceType>, // None if not a promotion
}

impl Move {
    // Constructor for a regular move
    pub fn new(from: (usize, usize), to: (usize, usize), piece_moved: Piece, piece_captured: Option<Piece>) -> Self {
        Move {
            from,
            to,
            piece_moved,
            piece_captured,
            is_castling_move: false,
            is_en_passant_move: false,
            promoted_to: None,
        }
    }

    // Additional constructors or methods for special moves can be added here
}