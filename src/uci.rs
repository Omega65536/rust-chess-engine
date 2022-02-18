use std::{io::{self}, ops::Index, time::Duration};

use chess::{Board, ChessMove};

use crate::{util, search};

const NAME: &str = "Rust Chess Engine";
const AUTHOR: &str = "https://github.com/Omega65536";

pub fn start() {
    let mut board: Board = Board::default();
    loop {
        let (command, args) = get_input();
        let mut arg_iter = args.iter();
        match command.as_str() {
            "uci" => {
                println!("id name {}", NAME);
                println!("id author {}", AUTHOR);
                println!("uciok");
            },
            "isready" => {
                println!("reayok");
            },
            "quit" => {
                return;
            },
            "ucinewgame" => {

            },
            "position" => {
                arg_iter.next();
                arg_iter.next();
                for move_notation in arg_iter {
                    let chess_move = ChessMove::from_san(&board, &move_notation).unwrap();
                    board = board.make_move_new(chess_move);
                }
                util::print_board(&board);
            }
            "go" => {
                util::print_board(&board);
                search::iterative_deepening(Duration::from_millis(1_000), &mut board);
            }
            _ => panic!("Unknown command {}", command),
        }
    }
}

fn get_input() -> (String, Vec<String>) {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    if buffer.ends_with('\n') {
        buffer.pop();
    }
    if buffer.ends_with('\r') {
        buffer.pop();
    }
    let mut tokens = buffer.split_whitespace();
    let command = tokens.next().unwrap().to_string();
    let args = Vec::from_iter(tokens.map(String::from));
    return (command, args);
    // let i = buffer.find(" ");
    // match i {
    //     Some(i) => (buffer.split_at(i).0.to_string(), buffer.split_at(i).1.to_string()),
    //     None => (buffer, String::from("")),
    // }
}