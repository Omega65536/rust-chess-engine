use std::time::{Instant, Duration};

pub struct SearchContext {
    start_time: Instant,
    max_time: Duration,
    num_nodes: u64,
    debug_output: bool,
}

impl SearchContext {
    pub fn new(max_time: Duration) -> SearchContext{
        SearchContext {
            start_time: Instant::now(),
            max_time: max_time,
            num_nodes: 0,
            debug_output: true,
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
}