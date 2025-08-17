use crate::{board::{Board, Cell}, solvers::Solver};
pub struct BackTrackingSolver;
impl Solver for BackTrackingSolver {
    fn solve(&self, board: &mut Board) -> bool {
        let mut curr_cell: &mut Cell;
        let mut is_valid = true;
        loop {
            // If the current board is valid (no duplicate values)
            curr_cell = if is_valid {
                if let Some(cell) = board.get_first_non_fixed_zero() {
                    cell
                } else {
                    if board.is_correct() {
                        return true;
                    } else {
                        if let Some(cell) = board.get_last_non_fixed_non_zero() {
                            cell
                        } else {
                            // no solution exists
                            return false;
                        }
                    }
                }
            } else {
                if let Some(cell) = board.get_last_non_fixed_non_zero() {
                    cell
                } else {
                    // no solution exists
                    return false;
                }
            };
            let increment_result = curr_cell.increment();

            if increment_result.needs_revalidation {
                is_valid = board.is_correct();
            } else {
                is_valid = increment_result.is_board_valid;
            }
        }
    }
    
    fn name(&self) -> &'static str {
        "Backtracking"
    }
}