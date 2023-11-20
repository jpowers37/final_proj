

use crate::board::Cell;
use crate::board::Board;
use crate::board::BOARD_SIZE;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Piece {
    pub piece_type: PieceType,
    pub color: Color,
    pub has_moved: bool,

}
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum PieceType {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Color {
    White,
    Black,
}
impl Piece {
    pub fn valid_moves(&self, position: (usize, usize), board: &Board) -> Vec<(usize, usize)> {
        match self.piece_type {
            PieceType::Pawn => self.pawn_moves(position, board),
            PieceType::Rook => self.rook_moves(position, board),
            PieceType::Knight => self.knight_moves(position, board),
            PieceType::Bishop => self.bishop_moves(position, board),
            PieceType::Queen => self.queen_moves(position, board),
            PieceType::King => self.king_moves(position, board),
        }
    }
    fn pawn_moves(&self, position:(usize, usize), board: &Board) -> Vec<(usize, usize)> {
        let mut moves = Vec::new();
        let (x, y) = position;
        match self.color {
            Color::White => {
                if x < BOARD_SIZE - 1 && board.grid[x + 1][y] == Cell::Empty {
                    moves.push((x + 1, y));
                }
                if x == 1 && board.grid[x + 2][y] == Cell::Empty && board.grid[x + 1][y] == Cell::Empty {
                    moves.push((x + 2, y));
                }
                if x < BOARD_SIZE - 1 && y < BOARD_SIZE - 1 {
                    if let Cell::Occupied(piece) = board.grid[x + 1][y + 1] {
                        if piece.color != self.color {
                            moves.push((x + 1, y + 1));
                        }
                    }
                }
                if x < BOARD_SIZE - 1 && y > 0 {
                    if let Cell::Occupied(piece) = board.grid[x + 1][y - 1] {
                        if piece.color != self.color {
                            moves.push((x + 1, y - 1));
                        }
                    }
                }
            },
            Color::Black => {
                if x > 0 && board.grid[x - 1][y] == Cell::Empty {
                    moves.push((x - 1, y));
                }
                if x == 6 && board.grid[x - 2][y] == Cell::Empty && board.grid[x - 1][y] == Cell::Empty {
                    moves.push((x - 2, y));
                }
                if x > 0 && y < BOARD_SIZE - 1 {
                    if let Cell::Occupied(piece) = board.grid[x - 1][y + 1] {
                        if piece.color != self.color {
                            moves.push((x - 1, y + 1));
                        }
                    }
                }
                if x > 0 && y > 0 {
                    if let Cell::Occupied(piece) = board.grid[x - 1][y - 1] {
                        if piece.color != self.color {
                            moves.push((x - 1, y - 1));
                        }
                    }
                }
            },
        }
        moves    
    }
    fn rook_moves(&self, position: (usize, usize), board: &Board) -> Vec<(usize, usize)> {
        let mut moves = Vec::new();
        let (x, y) = position;
        let directions = [(1, 0), (0, 1), (-1, 0), (0, -1)];

        for &(dx, dy) in &directions {
            let mut current_x = x as i32;
            let mut current_y = y as i32;

            loop {
                current_x += dx;
                current_y += dy;
                if current_x < 0 || current_x >= BOARD_SIZE as i32 || current_y < 0 || current_y >= BOARD_SIZE as i32 {
                    break;
                }
                let new_position = (current_x as usize, current_y as usize);
                match board.grid[new_position.0][new_position.1] {
                    Cell::Empty => {
                        moves.push(new_position);
                    }
                    Cell::Occupied(piece) => {
                        if piece.color != self.color {
                            moves.push(new_position);
                        }
                        break;
                    }
                }
            }
        }
        moves
    }
    fn knight_moves(&self, position: (usize, usize), board: &Board) -> Vec<(usize, usize)> {
        let mut moves = Vec::new();
        let (x, y) = position;
        let possible_moves = [
            (x.wrapping_add(2), y.wrapping_add(1)),
            (x.wrapping_add(2), y.wrapping_sub(1)),
            (x.wrapping_sub(2), y.wrapping_add(1)),
            (x.wrapping_sub(2), y.wrapping_sub(1)),
            (x.wrapping_add(1), y.wrapping_add(2)),
            (x.wrapping_add(1), y.wrapping_sub(2)),
            (x.wrapping_sub(1), y.wrapping_add(2)),
            (x.wrapping_sub(1), y.wrapping_sub(2)),
        ];

        for &new_position in &possible_moves {
            if new_position.0 < BOARD_SIZE && new_position.1 < BOARD_SIZE {
                match board.grid[new_position.0][new_position.1] {
                    Cell::Empty => {
                        moves.push(new_position);
                    }
                    Cell::Occupied(piece) => {
                        if piece.color != self.color {
                            moves.push(new_position);
                        }
                    }
                }
            }
        }
        moves
    }
    fn bishop_moves(&self, position: (usize, usize), board: &Board) -> Vec<(usize, usize)> {
        let mut moves = Vec::new();
        let (x, y) = position;
        let directions = [(1, 1), (1, -1), (-1, 1), (-1, -1)];

        for &(dx, dy) in &directions {
            let mut current_x = x as i32;
            let mut current_y = y as i32;

            loop {
                current_x += dx;
                current_y += dy;
                if current_x < 0 || current_x >= BOARD_SIZE as i32 || current_y < 0 || current_y >= BOARD_SIZE as i32 {
                    break;
                }
                let new_position = (current_x as usize, current_y as usize);
                match board.grid[new_position.0][new_position.1] {
                    Cell::Empty => {
                        moves.push(new_position);
                    }
                    Cell::Occupied(piece) => {
                        if piece.color != self.color {
                            moves.push(new_position);
                        }
                        break;
                    }
                }
            }
        }
        moves
    }
    fn queen_moves(&self, position: (usize, usize), board: &Board) -> Vec<(usize, usize)> {
        let mut moves = Vec::new();
        let (x, y) = position;
        let directions = [(1, 0), (0, 1), (-1, 0), (0, -1), (1, 1), (1, -1), (-1, 1), (-1, -1)];

        for &(dx, dy) in &directions {
            let mut current_x = x as i32;
            let mut current_y = y as i32;

            loop {
                current_x += dx;
                current_y += dy;

                if current_x < 0 || current_x >= BOARD_SIZE as i32 || current_y < 0 || current_y >= BOARD_SIZE as i32 {
                    break;
                }

                let new_position = (current_x as usize, current_y as usize);
                match board.grid[new_position.0][new_position.1] {
                    Cell::Empty => {
                        moves.push(new_position);
                    }
                    Cell::Occupied(piece) => {
                        if piece.color != self.color {
                            moves.push(new_position);
                        }
                        break;
                    }
                }
            }
        }
        moves
    }
    fn king_moves(&self, position: (usize, usize), board: &Board) -> Vec<(usize, usize)> {
        let mut moves = Vec::new();
        let (x, y) = position;
        let possible_moves = [
            (x.wrapping_add(1), y),
            (x.wrapping_sub(1), y),
            (x, y.wrapping_add(1)),
            (x, y.wrapping_sub(1)),
            (x.wrapping_add(1), y.wrapping_add(1)),
            (x.wrapping_add(1), y.wrapping_sub(1)),
            (x.wrapping_sub(1), y.wrapping_add(1)),
            (x.wrapping_sub(1), y.wrapping_sub(1)),
        ];

        for &new_position in &possible_moves {
            if new_position.0 < BOARD_SIZE && new_position.1 < BOARD_SIZE {
                match board.grid[new_position.0][new_position.1] {
                    Cell::Empty => {
                        moves.push(new_position);
                    }
                    Cell::Occupied(piece) => {
                        if piece.color != self.color {
                            moves.push(new_position);
                        }
                    }
                }
            }
        }
        moves
    }

}