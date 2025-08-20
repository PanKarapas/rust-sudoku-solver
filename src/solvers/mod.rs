use crate::solvers::{backtracking::BackTrackingSolver, constrained_backtracking::ConstrainedBackTrackingSolver};

#[cfg(test)]
mod tests;


pub mod backtracking;
pub mod constrained_backtracking;

pub trait Solver {
    #[allow(dead_code)]
    fn name(&self) -> &'static str {
        std::any::type_name::<Self>()
    }
    fn solve(&self, board: &'static str) -> Result<(bool, String), &'static str>;
}


pub fn get_solver(s: &str) -> Result<Box<dyn Solver>, String> {
    match s.to_lowercase().as_str() {
        "backtracking" => Ok(Box::new(BackTrackingSolver)),
        "constrainedbacktracking" => Ok(Box::new(ConstrainedBackTrackingSolver)),
        _ => Err("Unknown solver type: ".to_owned() + s)
     }
}

