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
    let mut value = 0;

    let white_pieces = board.color_combined(Color::White);
    let black_pieces = board.color_combined(Color::Black);
    let white_king = board.king_square(Color::White);
    let black_king = board.king_square(Color::Black);

    let pawns = board.pieces(Piece::Pawn);
    let knights = board.pieces(Piece::Knight);
    let bishops = board.pieces(Piece::Bishop);
    let rooks = board.pieces(Piece::Rook);
    let queens = board.pieces(Piece::Queen);

    for square in white_pieces & pawns {
        value += PAWN_VALUE   + PAWN_MG[square.to_index()];
    }
    for square in white_pieces & knights {
        value += KNIGHT_VALUE + KNIGHT_MG[square.to_index()];
    }
    for square in white_pieces & bishops {
        value += BISHOP_VALUE + BISHOP_MG[square.to_index()];
    }
    for square in white_pieces & rooks {
        value += ROOK_VALUE   ;//+ ROOK_MG[square.to_index()];
    }
    for square in white_pieces & queens {
        value += QUEEN_VALUE  ;//+ QUEEN_MG[square.to_index()];
    }
    value += KING_MG[white_king.to_index()];
    

    for square in black_pieces & pawns {
        value -= PAWN_VALUE   + PAWN_MG[63 - square.to_index()];
    }
    for square in black_pieces & knights {
        value -= KNIGHT_VALUE + KNIGHT_MG[63 - square.to_index()];
    }
    for square in black_pieces & bishops {
        value -= BISHOP_VALUE + BISHOP_MG[63 - square.to_index()];
    }
    for square in black_pieces & rooks {
        value -= ROOK_VALUE   ;//+ ROOK_MG[63 - square.to_index()];
    }
    for square in black_pieces & queens {
        value -= QUEEN_VALUE  ;//+ QUEEN_MG[63 - square.to_index()];
    }
    value -= KING_MG[63 - black_king.to_index()];

    // for square in board.color_combined(Color::White).into_iter() {
    //     let piece = board.piece_on(square).unwrap();
    //     value += eval_white_piece(piece, square);
    // }
    // for square in board.color_combined(Color::Black).into_iter() {
    //     let piece = board.piece_on(square).unwrap();
    //     value -= eval_black_piece(piece, square);
    // }
    return if board.side_to_move() == Color::White{ value } else { -value };
}
