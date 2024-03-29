use chess::{ChessMove, MoveGen, Board, Piece, EMPTY};

use crate::search_context::SearchContext;

// const MVV_LVA: [[i16; 6]; 6] = [
//     //K   Q   R   B   N   P
//     [ 0,  0,  0,  0,  0,  0], // K
//     [50, 51, 52, 53, 54, 55], // Q
//     [40, 41, 42, 43, 44, 45], // R
//     [30, 31, 32, 33, 34, 35], // B
//     [20, 21, 22, 23, 24, 25], // N
//     [10, 11, 12, 13, 14, 15], // P
// ];

#[inline]
pub fn get_moves(board: &Board, search_ctx: &SearchContext) -> Vec<ChessMove> {
    let mut move_iter = MoveGen::new_legal(&board);
    let num_moves = move_iter.len();
    let mut moves: Vec<ChessMove> = Vec::with_capacity(num_moves);

    match search_ctx.get_last_pv_move() {
        Some(last_pv_move) => {
            if board.legal(last_pv_move) {
                moves.push(last_pv_move);
                move_iter.remove_move(last_pv_move);
            }
        },
        None => (),
    }

    let targets = board.pieces(Piece::Queen);
    move_iter.set_iterator_mask(*targets);
    for chess_move in &mut move_iter {
        moves.push(chess_move);
    }

    let targets = board.pieces(Piece::Rook);
    move_iter.set_iterator_mask(*targets);
    for chess_move in &mut move_iter {
        moves.push(chess_move);
    }

    let targets = board.pieces(Piece::Bishop);
    move_iter.set_iterator_mask(*targets);
    for chess_move in &mut move_iter {
        moves.push(chess_move);
    }

    let targets = board.pieces(Piece::Knight);
    move_iter.set_iterator_mask(*targets);
    for chess_move in &mut move_iter {
        moves.push(chess_move);
    }

    let targets = board.pieces(Piece::Pawn);
    move_iter.set_iterator_mask(*targets);
    for chess_move in &mut move_iter {
        moves.push(chess_move);
    }

    move_iter.set_iterator_mask(!EMPTY);
    for chess_move in &mut move_iter {
        moves.push(chess_move);
    }

    return moves;

    // let num_moves = move_iter.len();
    // let mut moves: Vec<ChessMove> = Vec::with_capacity(num_moves);
    // let mut scores: Vec<i16> = Vec::with_capacity(num_moves);
    // for chess_move in move_iter {
    //     moves.push(chess_move);
    //     let victim_opt = board.piece_on(chess_move.get_dest());
    //     match victim_opt {
    //         Some(victim) => {
    //             let aggressor = board.piece_on(chess_move.get_source()).unwrap();
    //             let score = MVV_LVA[victim.to_index()][aggressor.to_index()];
    //             scores.push(score);
    //         },
    //         None => {
    //             moves.push(chess_move);
    //         }
    //     }
    // }
}

#[inline]
pub fn get_captures(board: &Board, search_ctx: &SearchContext) -> Vec<ChessMove> {
    let mut move_iter = MoveGen::new_legal(&board);
    let num_moves = move_iter.len();
    let mut moves: Vec<ChessMove> = Vec::with_capacity(num_moves);

    let targets = board.pieces(Piece::Queen);
    move_iter.set_iterator_mask(*targets);
    for chess_move in &mut move_iter {
        moves.push(chess_move);
    }

    let targets = board.pieces(Piece::Rook);
    move_iter.set_iterator_mask(*targets);
    for chess_move in &mut move_iter {
        moves.push(chess_move);
    }

    let targets = board.pieces(Piece::Bishop);
    move_iter.set_iterator_mask(*targets);
    for chess_move in &mut move_iter {
        moves.push(chess_move);
    }

    let targets = board.pieces(Piece::Knight);
    move_iter.set_iterator_mask(*targets);
    for chess_move in &mut move_iter {
        moves.push(chess_move);
    }

    let targets = board.pieces(Piece::Pawn);
    move_iter.set_iterator_mask(*targets);
    for chess_move in &mut move_iter {
        moves.push(chess_move);
    }

    return moves;
}