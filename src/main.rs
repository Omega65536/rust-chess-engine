use chess::{Board, BoardStatus, ChessMove};
use std::io;

mod util;
mod engine;

fn main() {
    test();
}

fn test() {
    let mut board = Board::default();
    let engine = engine::Engine::new();

    while board.status() == BoardStatus::Ongoing {
        let user_move = get_user_move(&board);
        board = Board::make_move_new(&board, user_move);
        util::print_board(&board);

        let engine_move = engine.get_move(&board);
        board = Board::make_move_new(&board, engine_move);
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
            Err(e) => println!("Invalid move!"),
        }
    }
}
