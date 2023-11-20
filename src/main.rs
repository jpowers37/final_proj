mod game;
mod board;
mod piece;
mod chess_move;
use std::io::{self};





fn main() {
    let mut game = game::Game::new();

    loop {
        board::Board::print_board(&game.board);
        
        // Check if the game has ended
        if game.state != game::GameState::Ongoing {
            println!("Game Over!");
            break;
        }
        if game.is_in_check() {
            println!("{} is in check!", if game.current_player == piece::Color::White { "Black" } else { "White" });
        }

        if game.is_in_checkmate() {
            println!("Checkmate! {} wins!", if game.current_player == piece::Color::White { "Black" } else { "White" });
            break;  // Exit the loop if the game is over
        }
        // Get the current player's move
        println!("{} Move: ", if game.current_player == piece::Color::White { "Black" } else { "White" });
        let from = ask_for_move_input("Enter the source square (e.g., 'e2'): ");
        let to = ask_for_move_input("Enter the destination square (e.g., 'e4'): ");

        // Validate and make the move
        if game.is_valid_move(from, to) {
            game.board.move_piece(from, to);
            game.handle_special_moves(from, to);
            game.switch_player();
        } else {
            println!("Invalid move. Please try again.");
        }
    }
}

fn ask_for_move_input(prompt: &str) -> (usize, usize) {
    loop {
        println!("{}", prompt);
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        if let Some(position) = parse_chess_notation(&input.trim()) {
            return position;
        } else {
            println!("Invalid input. Please try again.");
        }
    }
}

fn parse_chess_notation(notation: &str) -> Option<(usize, usize)> {
    if notation.len() == 2 {
        let col = notation.chars().next().unwrap().to_ascii_lowercase() as usize - 'a' as usize;
        let row = notation.chars().nth(1).unwrap().to_digit(10).unwrap() as usize - 1;

        if col < 8 && row < 8 {
            return Some((7 - row, col)); // Convert to 0-indexed and flip row for typical chessboard layout
        }
    }
    None
}
