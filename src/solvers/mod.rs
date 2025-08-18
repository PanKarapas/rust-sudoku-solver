use crate::solvers::{backtracking::BackTrackingSolver};

#[cfg(test)]
mod tests;


pub mod backtracking;

pub trait Solver {
    fn name(&self) -> &'static str;
    fn solve(&self, board: &'static str) -> Result<(bool, String), &'static str>;
}


pub fn get_solver(s: &str) -> Result<Box<dyn Solver>, String> {
    match s.to_lowercase().as_str() {
        "backtracking" => Ok(Box::new(BackTrackingSolver)),
        _ => Err("Unknown solver type: ".to_owned() + s)
     }
}

