use chess::{Board, Piece, Color, Square};

use crate::search_context::SearchContext;

const PAWN_VALUE: i16 = 100;
const KNIGHT_VALUE: i16 = 310;
const BISHOP_VALUE: i16 = 330;
const ROOK_VALUE: i16 = 520;
const QUEEN_VALUE: i16 = 950;

type Psqt = [i16; 64];

const PAWN_MG: Psqt = [
    0,    0,     0,     0,    0,    0,    0,    0,
    5,    5,     5,   -40,  -40,    5,    5,    5,
    0,    0,   -10,     0,    0,  -10,    0,    0,
    0,    0,    20,    30,   30,   20,    0,    0,
    0,    0,     0,    50,   50,    0,    0,    0,
   50,   50,    50,    70,   70,   60,   50,   50,
  100,  100,   100,   100,  100,  100,  100,  100,
    0,    0,     0,     0,    0,    0,    0,    0,
];

const KNIGHT_MG: Psqt = [
  -50,  -30,   -30,   -30,  -30,  -30,  -30,  -50,
  -30,  -30,     0,     0,    0,    0,  -30,  -30,
  -30,    0,    10,    15,   15,   10,    0,  -30,
  -30,    0,    15,    20,   20,   15,    0,  -30,
  -30,    0,    15,    20,   20,   15,    0,  -30,
  -30,    0,    10,    15,   15,   10,    0,  -30,
  -30,  -30,     0,     0,    0,    0,  -30,  -30,
  -50,  -30,   -30,   -30,  -30,  -30,  -30,  -50,
];

const BISHOP_MG: Psqt = [
  -20,  -10,   -10,   -10,  -10,  -10,  -10,  -20,
  -10,   10,     0,     0,    0,    0,   10,  -10,
  -10,    0,    10,    15,   15,   10,    0,  -10,
  -10,    0,    15,    15,   15,   15,    0,  -10,
  -10,    0,    15,    15,   15,   15,    0,  -10,
  -10,    0,    10,    15,   15,   10,    0,  -10,
  -10,   10,     0,     0,    0,    0,   10,  -10,
  -20,  -10,   -10,   -10,  -10,  -10,  -10,  -20,
];

const KING_MG: Psqt = [
    0,   20,   -10,   -20,  -20,  -10,   20,    0,
    0,    0,     0,     0,    0,    0,    0,    0,
    0,    0,     0,     0,    0,    0,    0,    0,
    0,    0,     0,     0,    0,    0,    0,    0,
    0,    0,     0,     0,    0,    0,    0,    0,
    0,    0,     0,     0,    0,    0,    0,    0,
    0,    0,     0,     0,    0,    0,    0,    0,
    0,    0,     0,     0,    0,    0,    0,    0,
];

pub fn eval_board(board: &mut Board, search_ctx: &mut SearchContext) -> i16 {
    search_ctx.inc_nodes();
    let my_color = board.side_to_move();
    let mut value = 0;
    for square in board.combined().into_iter() {
        match board.color_on(square) {
            Some(color) => {
                let piece = board.piece_on(square).unwrap();
                if color == Color::White {
                    value += eval_white_piece(piece, square);
                }
                else {
                    value -= eval_black_piece(piece, square);
                }
            }
            None => {},
        }
    }
    return if my_color == Color::White { value } else { -value };
}

fn eval_white_piece(piece: Piece, square: Square) -> i16 {
    match piece {
        Piece::Pawn =>   PAWN_VALUE   + PAWN_MG[square.to_index()],
        Piece::Knight => KNIGHT_VALUE + KNIGHT_MG[square.to_index()],
        Piece::Bishop => BISHOP_VALUE + BISHOP_MG[square.to_index()],
        Piece::Rook =>   ROOK_VALUE,
        Piece::Queen =>  QUEEN_VALUE,
        Piece::King =>                  KING_MG[square.to_index()],
    }
}

fn eval_black_piece(piece: Piece, square: Square) -> i16 {
    match piece {
        Piece::Pawn =>   PAWN_VALUE   + PAWN_MG[63 - square.to_index()],
        Piece::Knight => KNIGHT_VALUE + KNIGHT_MG[63 - square.to_index()],
        Piece::Bishop => BISHOP_VALUE + BISHOP_MG[63 - square.to_index()],
        Piece::Rook =>   ROOK_VALUE,
        Piece::Queen =>  QUEEN_VALUE,
        Piece::King =>                  KING_MG[63 - square.to_index()],
    }
}