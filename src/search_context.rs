use std::time::{Instant, Duration};

use chess::ChessMove;

pub struct SearchContext {
    start_time: Instant,
    max_time: Duration,
    num_nodes: u64,
    last_pv_move: Option<ChessMove>,
    debug_output: bool,
    uci_output: bool,
}

impl SearchContext {
    pub fn new(max_time: Duration) -> SearchContext{
        SearchContext {
            start_time: Instant::now(),
            max_time: max_time,
            num_nodes: 0,
            last_pv_move: None,
            debug_output: false,
            uci_output: true,
        }
    }

    pub fn should_stop(&self) -> bool {
        self.num_nodes % 1024 == 0 && 
        self.start_time.elapsed() >= self.max_time
    }

    pub fn get_duration(&self) -> f32 {
        self.start_time.elapsed().as_secs_f32()
    }

    pub fn get_nodes(&self) -> u64 {
        self.num_nodes
    }

    pub fn inc_nodes(&mut self) {
        self.num_nodes += 1;
    }

    pub fn get_last_pv_move(&self) -> Option<ChessMove> {
        self.last_pv_move
    }
    
    pub fn set_last_pv_move(&mut self, new_pv_move: ChessMove) {
        self.last_pv_move = Some(new_pv_move);
    }

    pub fn is_debug(&self) -> bool {
        self.debug_output
    }

    pub fn is_uci(&self) -> bool {
        self.uci_output
    }
}