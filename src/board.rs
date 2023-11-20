
use crate::piece::Piece;
use crate::piece::{PieceType, Color};

pub const BOARD_SIZE: usize = 8;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Cell {
    Empty,
    Occupied(Piece),
}
#[derive(Clone)]
pub struct Board {
    pub grid: [[Cell; BOARD_SIZE]; BOARD_SIZE],
}

impl Board {
    pub fn new() -> Self {
        let mut grid = [[Cell::Empty; BOARD_SIZE]; BOARD_SIZE];
        grid[0] = [
            Cell::Occupied(Piece { piece_type: PieceType::Rook, color: Color::White, has_moved: false }),
            Cell::Occupied(Piece { piece_type: PieceType::Knight, color: Color::White, has_moved: false }),
            Cell::Occupied(Piece { piece_type: PieceType::Bishop, color: Color::White, has_moved: false }),
            Cell::Occupied(Piece { piece_type: PieceType::Queen, color: Color::White, has_moved: false }),
            Cell::Occupied(Piece { piece_type: PieceType::King, color: Color::White, has_moved: false }),
            Cell::Occupied(Piece { piece_type: PieceType::Bishop, color: Color::White, has_moved: false }),
            Cell::Occupied(Piece { piece_type: PieceType::Knight, color: Color::White, has_moved: false }),
            Cell::Occupied(Piece { piece_type: PieceType::Rook, color: Color::White, has_moved: false }),
        ];
        grid[1] = [Cell::Occupied(Piece { piece_type: PieceType::Pawn, color: Color::White, has_moved: false }); BOARD_SIZE];
        grid[6] = [Cell::Occupied(Piece { piece_type: PieceType::Pawn, color: Color::Black, has_moved: false }); BOARD_SIZE];
        grid[7] = [
            Cell::Occupied(Piece { piece_type: PieceType::Rook, color: Color::Black, has_moved: false }),
            Cell::Occupied(Piece { piece_type: PieceType::Knight, color: Color::Black, has_moved: false }),
            Cell::Occupied(Piece { piece_type: PieceType::Bishop, color: Color::Black, has_moved: false }),
            Cell::Occupied(Piece { piece_type: PieceType::Queen, color: Color::Black, has_moved: false }),
            Cell::Occupied(Piece { piece_type: PieceType::King, color: Color::Black, has_moved: false }),
            Cell::Occupied(Piece { piece_type: PieceType::Bishop, color: Color::Black, has_moved: false }),
            Cell::Occupied(Piece { piece_type: PieceType::Knight, color: Color::Black, has_moved: false }),
            Cell::Occupied(Piece { piece_type: PieceType::Rook, color: Color::Black, has_moved: false }),
        ];
        Self { grid }
    }
    
    pub fn print_board(board: &Board) {
        println!("  a b c d e f g h");
        for (i, row) in board.grid.iter().enumerate() {
            print!("{} ", 8 - i); // Row label
            for cell in row {
                let piece_symbol = match cell {
                    Cell::Empty => "·", // Use a dot or space for empty cells
                    Cell::Occupied(piece) => &piece_to_ascii(piece),
                };
                print!("{} ", piece_symbol); // Print piece with space
            }
            println!(" {}", 8 - i); // Row label at the end
        }
        println!("  a b c d e f g h");
    

    pub fn piece_to_ascii(piece: &Piece) -> &str {
        match (piece.color, piece.piece_type) {
            (Color::White, PieceType::King) => "♔",
            (Color::White, PieceType::Queen) => "♕",
            (Color::White, PieceType::Rook) => "♖",
            (Color::White, PieceType::Bishop) => "♗",
            (Color::White, PieceType::Knight) => "♘",
            (Color::White, PieceType::Pawn) => "♙",
            (Color::Black, PieceType::King) => "♚",
            (Color::Black, PieceType::Queen) => "♛",
            (Color::Black, PieceType::Rook) => "♜",
            (Color::Black, PieceType::Bishop) => "♝",
            (Color::Black, PieceType::Knight) => "♞",
            (Color::Black, PieceType::Pawn) => "♟︎",
            _ => " ", // Default case, should not occur
        } 
    }  } 

    pub fn is_within_bounds(&self, position: (isize, isize)) -> bool {
        position.0 >= 0 && position.0 < BOARD_SIZE as isize && position.1 >= 0 && position.1 < BOARD_SIZE as isize
    }

    pub fn get_piece_at(&self, position: (usize, usize)) -> Option<Piece> {
        match self.grid[position.0][position.1] {
            Cell::Occupied(piece) => Some(piece),
            Cell::Empty => None,
        }
    }
    pub fn set_piece_at(&mut self, position: (usize, usize), piece: Piece) {
        // Set the piece at the given 'position'
        self.grid[position.0][position.1] = Cell::Occupied(piece);
    }
    // Get the cell at the given position
    pub fn cell_at(&self, position: (isize, isize)) -> &Cell {
        &self.grid[position.0 as usize][position.1 as usize]
    }

    // Get the next square in the given direction from the given position
    pub fn next_square_in_direction(&self, position: (isize, isize), direction: (isize, isize)) -> Option<(isize, isize)> {
        let new_position = (position.0 + direction.0, position.1 + direction.1);
        if self.is_within_bounds(new_position) {
            Some(new_position)
        } else {
            None
        }
    }
    pub fn make_move(&mut self, from: (usize, usize), to: (usize, usize)) {
        // Assuming 'from' and 'to' are valid positions and the move is legal
        // Move the piece from 'from' to 'to'
        let piece = self.grid[from.0][from.1];
        self.grid[from.0][from.1] = Cell::Empty;
        self.grid[to.0][to.1] = piece;
    }
    pub fn move_piece(&mut self, from: (usize, usize), to: (usize, usize)) {
        let mut piece_to_move = match self.grid[from.0][from.1] {
            Cell::Occupied(piece) => piece,
            Cell::Empty => return,
        };

        // Move the piece
        self.grid[to.0][to.1] = Cell::Occupied(piece_to_move);
        self.grid[from.0][from.1] = Cell::Empty;

        // Mark the piece as having moved
        if let Cell::Occupied(ref mut moved_piece) = self.grid[to.0][to.1] {
            moved_piece.has_moved = true;
        }

    // Add other methods related to the board here.
    }
     pub fn remove_piece_at(&mut self, position: (usize, usize)) {
        self.grid[position.0][position.1] = Cell::Empty;
    }
}


