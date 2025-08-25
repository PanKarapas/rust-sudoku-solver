use crate::solvers::{backtracking::BacktrackingSolver, cell_eliminated_backtracking::CellEliminatedBacktrackingSolver, group_eliminated_backtracking::GroupEliminatedBacktrackingSolver};

pub mod backtracking;
pub mod cell_eliminated_backtracking;
pub mod group_eliminated_backtracking;

pub trait Solver {
    #[allow(dead_code)]
    fn name(&self) -> &'static str {
        std::any::type_name::<Self>()
    }
    fn solve(&self, board: &'static str) -> Result<(bool, String), &'static str>;
}


pub fn get_solver(s: &str) -> Result<Box<dyn Solver>, String> {
    match s.to_lowercase().as_str() {
        "backtracking" => Ok(Box::new(BacktrackingSolver)),
        "celleliminated" => Ok(Box::new(CellEliminatedBacktrackingSolver)),
        "groupeliminated" => Ok(Box::new(GroupEliminatedBacktrackingSolver)),
        _ => Err("Unknown solver type: ".to_owned() + s)
     }
}

