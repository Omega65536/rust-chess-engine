use chess::{Board, BoardStatus, ChessMove};
use std::io;
use std::str::FromStr;
use std::time::Duration;

mod util;
mod search_context;
mod search;
mod evaluate;

fn main() {
    test();
}

fn test() {
    let fens = [
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", // Starting pos
        "rn1qk2r/p2nbppp/bpp1p3/3pN3/2PP4/1PB3P1/P3PPBP/RN1QK2R w KQkq - 2 10", // QID
        "r2q1rk1/3nbppp/p2pbn2/4p1P1/1p2P3/1NN1BP2/PPPQ3P/2KR1B1R w - - 0 13", // Sicilian
        "1k2r3/ppp5/8/8/8/8/P4PPP/3R2K1 w - - 0 1", // Rook Endgame
        "4k3/8/8/8/8/8/8/3QK3 w - - 0 1", // KQvK
    ];
    for fen in fens {
        let board = Board::from_str(fen).expect("Invalid FEN");
        search::iterative_deepening(Duration::from_millis(500), &board);
        println!();
    }
}

fn vs() {
    let mut board = Board::default();

    while board.status() == BoardStatus::Ongoing {
        let engine_move = search::iterative_deepening(Duration::from_millis(10_000), &board);
        board = Board::make_move_new(&board, engine_move);
        util::print_board(&board);

        let user_move = get_user_move(&board);
        board = Board::make_move_new(&board, user_move);
        util::print_board(&board);
    }
}


fn get_user_move(board: &Board) -> ChessMove {
    let mut input = String::new();
    loop {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let user_move = ChessMove::from_san(&board, &input);
        match user_move {
            Ok(x) => return x,
            Err(_) => println!("Invalid move!"),
        }
    }
}
