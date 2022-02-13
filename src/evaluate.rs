use chess::{Board, Piece, Color, Square, Rank, File};

use crate::search_context::SearchContext;

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

pub fn eval_board(board: &Board, search_ctx: &mut SearchContext) -> i16 {
    search_ctx.inc_nodes();
    let my_color = board.side_to_move();
    let mut value = 0;
    for x in 0..8 {
        for y in 0..8 {
            let square = Square::make_square(
                Rank::from_index(y),
                File::from_index(x));
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
    }
    //println!("Board: {} Value: {}", board, value);
    return if my_color == Color::White { value } else { -value };
}

fn eval_white_piece(piece: Piece, square: Square) -> i16 {
    match piece {
        Piece::Pawn =>   100 + PAWN_MG[square.to_index()],
        Piece::Knight => 310 + KNIGHT_MG[square.to_index()],
        Piece::Bishop => 330 + BISHOP_MG[square.to_index()],
        Piece::Rook =>   520,
        Piece::Queen =>  950,
        Piece::King => 10000 + KING_MG[square.to_index()],
    }
}

fn eval_black_piece(piece: Piece, square: Square) -> i16 {
    match piece {
        Piece::Pawn =>   100 + PAWN_MG[63 - square.to_index()],
        Piece::Knight => 310 + KNIGHT_MG[63 - square.to_index()],
        Piece::Bishop => 330 + BISHOP_MG[63 - square.to_index()],
        Piece::Rook =>   520,
        Piece::Queen =>  950,
        Piece::King => 10000 + KING_MG[63 - square.to_index()],
    }
}