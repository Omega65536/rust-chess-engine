use std::cmp;
use chess::{Board, ChessMove, MoveGen, Piece, Color, Square, Rank, File};

pub struct Engine {
    num_nodes: u64,
}

impl Engine {
    pub fn new() -> Engine{
        Engine {
            num_nodes: 0,
        }
    }

    pub fn get_move(&self, board: &Board) -> ChessMove {
        println!("boardeval {}", self.evaluate(&board));
        let mut best_value = -31_000;
        let mut best_move = ChessMove::new(Square::A1, Square::A1, None);
        let move_iter = MoveGen::new_legal(&board);
        for chess_move in move_iter {
            let new_board = Board::make_move_new(board, chess_move);
            let value = -self.negamax(4, -30_000, 30_000, &new_board);
            if value > best_value {
                best_value = value;
                best_move = chess_move;
            }
            println!("BestMove {} Move {} Eval: {}", best_move, chess_move, value);
        }

        println!("MOVE {} EVAL {}", best_move, best_value);
        return best_move;
    }

    fn negamax(&self, depth_left: u16, mut alpha: i16, beta: i16, board: &Board) -> i16 {
        if depth_left == 0  {
            return self.evaluate(board);
        }

        let mut value = -30_000;
        let move_iter = MoveGen::new_legal(&board);
        for chess_move in move_iter {
            //println!("Value: {}, Alpha: {}, Beta: {}", value, alpha, beta);
            let new_board = board.make_move_new(chess_move);
            value = cmp::max(value, -self.negamax(depth_left - 1, -beta, -alpha, &new_board));
            alpha = cmp::max(alpha, value);
            if alpha >= beta {
                break;
            }
        }
        return value;
    }

    fn evaluate(&self, board: &Board) -> i16 {
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
                        if my_color == color {
                            value += self.eval_piece(piece);
                        }
                        else {
                            value -= self.eval_piece(piece);
                        }
                    }
                    None => {},
                }
            }
        }
        //println!("Board: {} Value: {}", board, value);
        return value
    }

    fn eval_piece(&self, piece: Piece) -> i16 {
        match piece {
            Piece::Pawn => 100,
            Piece::Knight => 310,
            Piece::Bishop => 330,
            Piece::Rook => 520,
            Piece::Queen => 950,
            Piece::King => 10000,
        }
    }
}
