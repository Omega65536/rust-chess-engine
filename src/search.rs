use std::cmp;
use std::time::{Duration};
use chess::{Board, ChessMove, Square};
use crate::search_context::SearchContext;
use crate::{evaluate, search_context, move_ordering};

pub fn iterative_deepening(max_time: Duration, board: &mut Board) -> ChessMove{
    // println!("┌────────┐    ┌────────────┬────────────┬────────────┐    ┌──────┬────────┐");
    // println!("│  Depth │    │   Time (s) │      Nodes │        NpS │    │ Move │   Eval │");
    // println!("├────────┤    ├────────────┼────────────┼────────────┤    ├──────┼────────┤");
    let mut best_move = None;
    let mut depth = 1;
    let mut search_ctx = search_context::SearchContext::new(max_time);
    search_ctx.set_last_pv_move(ChessMove::new(Square::E2, Square::E4, None));
    loop {
        let new_move = Some(get_move(depth, board, &mut search_ctx));
        if search_ctx.should_stop() {
            break;
        }
        best_move = new_move;
        //search_ctx.set_last_pv_move(best_move.unwrap());
        depth += 1;
    }
    // println!("└────────┘    └────────────┴────────────┴────────────┘    └──────┴────────┘");
    // println!("Finished iterative deepening , after {:8.3} seconds!", search_ctx.get_duration());
    println!("bestmove {}", best_move.unwrap());
    return best_move.unwrap();
}

fn get_move(depth: u16, board: &mut Board, search_ctx: &mut SearchContext) -> ChessMove {
    let mut best_value = -31_000;
    let mut best_move = None;
    let moves = move_ordering::get_moves(&board, &search_ctx);
    for chess_move in moves {
        let mut new_board = Board::make_move_new(board, chess_move);
        let value = -search(depth - 1, -30_000, 30_000, &mut new_board, search_ctx);
        if value > best_value {
            best_value = value;
            best_move = Some(chess_move);
        }
    }

    if !search_ctx.should_stop() {
        if search_ctx.is_debug() {
            println!("│ {:6} │    │ {:10.3} │ {:10} │ {:10.0} │    │ {} │ {:6} │",
            depth, search_ctx.get_duration(), search_ctx.get_nodes(),
            search_ctx.get_nodes() as f32 / search_ctx.get_duration(),
            best_move.unwrap(), best_value);
        }
        else if search_ctx.is_uci() {
            println!("info depth {} nodes {}", depth, search_ctx.get_nodes());
        }
        
    }
    return best_move.unwrap();
}

fn search(depth_left: u16, mut alpha: i16, beta: i16, board: &mut Board, search_ctx: &mut SearchContext) -> i16 {
    if search_ctx.should_stop() {
        return 0;
    }
    if depth_left <= 0  {
        return quiesence(alpha, beta, board, search_ctx);
    }

    // Null move pruning
    const R: u16 = 2;
    if depth_left > R {
        let temp_board_opt = board.null_move();
        match temp_board_opt {
            Some(mut temp_board) => {
                let value = -search(depth_left - R - 1, -beta, -beta + 1, &mut temp_board, search_ctx);
                if  value >= beta {
                    return value;
                }
            },
            None => (), 
        }
    }

    let moves = move_ordering::get_moves(&board, &search_ctx);
    for chess_move in moves {
        let mut temp_board = board.make_move_new(chess_move);
        let value = -search(depth_left - 1, -beta, -alpha, &mut temp_board, search_ctx);
        if value >= beta {
            return beta;
        }
        alpha = cmp::max(alpha, value);
    }
    return alpha;
}

fn quiesence(mut alpha: i16, beta: i16, board: &mut Board, search_ctx: &mut SearchContext) -> i16 {
    if search_ctx.should_stop() {
        return 0;
    }
    let stand_pat = evaluate::eval_board(board, search_ctx);
    if stand_pat >= beta {
        return beta;
    }

    // Delta pruning
    const BIG_DELTA: i16 = 975;
    if stand_pat < alpha - BIG_DELTA {
        return alpha;
    }
    alpha = cmp::max(alpha, stand_pat);
    
    let captures = move_ordering::get_captures(&board, &search_ctx);
    for chess_move in captures {
        let mut temp_board = board.make_move_new(chess_move);
        let value = -quiesence(-beta, -alpha, &mut temp_board, search_ctx);
        if value >= beta {
            return beta;
        }
        alpha = cmp::max(alpha, value);
    }
    return alpha;
}