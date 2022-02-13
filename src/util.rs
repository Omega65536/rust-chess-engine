use std::time::{Instant};
use chess::{Board, MoveGen, Color, Square, Rank, File};

pub fn print_board(board: &Board) {
    let white_symbols = ['P', 'N', 'B', 'R', 'Q', 'K'];
    let black_symbols = ['p', 'n', 'b', 'r', 'q', 'k'];
    let mut result = String::new();

    for y in (0..8).rev() {
        for x in 0..8 {
            let square = Square::make_square(
                Rank::from_index(y),
                File::from_index(x));
            match board.piece_on(square) {
                Some(piece) =>
                match board.color_on(square) {
                    Some(color) =>
                    match color {
                        Color::White => result.push(white_symbols[piece.to_index()]),
                        Color::Black => result.push(black_symbols[piece.to_index()]),
                    }
                    None => result.push('?'),
                }
                None => result.push('.'),
            }
            result.push(' ');
        }
        result.push('\n');
    }
    println!("{}", result);
}

pub fn perft(depth: usize) {
    let board = Board::default();
    let now = Instant::now();
    MoveGen::movegen_perft_test(&board, depth);
    let elapsed_time = now.elapsed();
    println!("{} millis.", elapsed_time.as_millis());
}
