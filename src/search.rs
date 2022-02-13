use std::cmp;
use std::time::{Duration};
use chess::{Board, ChessMove, MoveGen};
use crate::search_context::SearchContext;
use crate::{evaluate, search_context};

pub fn iterative_deepening(max_time: Duration, board: &Board) -> ChessMove{
    println!("┌────────┐    ┌────────────┬────────────┬────────────┐    ┌──────┬────────┐");
    println!("│  Depth │    │   Time (s) │      Nodes │        NpS │    │ Move │   Eval │");
    println!("├────────┤    ├────────────┼────────────┼────────────┤    ├──────┼────────┤");
    let mut best_move = None;
    let mut depth = 1;
    let mut search_ctx = search_context::SearchContext::new(max_time);
    loop {
        let new_move = Some(get_move(depth, board, &mut search_ctx));
        if search_ctx.should_stop() {
            break;
        }

        best_move = new_move;
        depth += 1;
    }
    println!("└────────┘    └────────────┴────────────┴────────────┘    └──────┴────────┘");
    println!("Finished iterative deepening with {}, after {:8.3} seconds and {} nodes!",
    best_move.unwrap(), search_ctx.get_duration(), search_ctx.get_nodes());
    return best_move.unwrap();
}

fn get_move(depth: u16, board: &Board, search_ctx: &mut SearchContext) -> ChessMove {
    let mut best_value = -31_000;
    let mut best_move = None;
    let move_iter = MoveGen::new_legal(&board);
    for chess_move in move_iter {
        let new_board = Board::make_move_new(board, chess_move);
        let value = -search(depth - 1, -30_000, 30_000, &new_board, search_ctx);
        if value > best_value {
            best_value = value;
            best_move = Some(chess_move);
        }
    }

    if !search_ctx.should_stop() {
        println!("│ {:6} │    │ {:10.3} │ {:10} │ {:10.0} │    │ {} │ {:6} │",
            depth, search_ctx.get_duration(), search_ctx.get_nodes(),
            search_ctx.get_nodes() as f32 / search_ctx.get_duration(),
            best_move.unwrap(), best_value);
    }
    return best_move.unwrap();
}

fn search(depth_left: u16, mut alpha: i16, beta: i16, board: &Board, search_ctx: &mut SearchContext) -> i16 {
    if search_ctx.should_stop() {
        return 0;
    }
    if depth_left == 0  {
        return quiesence(alpha, beta, board, search_ctx);
    }

    let move_iter = MoveGen::new_legal(&board);
    for chess_move in move_iter {
        let temp_board = board.make_move_new(chess_move);
        let value = -search(depth_left - 1, -beta, -alpha, &temp_board, search_ctx);
        if value >= beta {
            return beta;
        }
        alpha = cmp::max(alpha, value);
    }
    return alpha;
}

pub fn quiesence(mut alpha: i16, beta: i16, board: &Board, search_ctx: &mut SearchContext) -> i16 {
    if search_ctx.should_stop() {
        return 0;
    }
    let stand_pat = evaluate::eval_board(board, search_ctx);
    if stand_pat >= beta {
        return beta;
    }
    alpha = cmp::max(alpha, stand_pat);
    
    let mut move_iter = MoveGen::new_legal(&board);
    let targets = board.color_combined(!board.side_to_move());
    move_iter.set_iterator_mask(*targets);
    for chess_move in move_iter {
        let temp_board = board.make_move_new(chess_move);
        let value = -quiesence(-beta, -alpha, &temp_board, search_ctx);
        if value >= beta {
            return beta;
        }
        alpha = cmp::max(alpha, value);
    }
    return alpha;
}