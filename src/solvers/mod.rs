use crate::{board::Board, solvers::backtracking::BackTrackingSolver};

pub mod backtracking;

pub trait Solver {
    fn solve(&self, board: &mut Board) -> bool;
}

pub fn get_solver(s: &str) -> Result<Box<dyn Solver>, String> {
    match s.to_lowercase().as_str() {
        "backtracking" => Ok(Box::new(BackTrackingSolver)),
        _ => Err("Unknown solver type: ".to_owned() + s)
     }
}

