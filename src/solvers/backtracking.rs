use crate::{board::{cell::simple_cell::SimpleCell, Board}, solvers::Solver};

pub struct BackTrackingSolver;
impl Solver for BackTrackingSolver {
    fn solve(&self, puzzle: &'static str) -> Result<(bool, String), &'static str> {
        let mut board = match Board::<SimpleCell>::parse_puzzle_string(puzzle) {
            Err(error) => return Err(error),
            Ok(b) => b
        };
        let mut curr_cell: &mut SimpleCell;
        let mut is_valid = true;
        loop {
            // If the current board is valid (no duplicate values)
            curr_cell = if is_valid {
                if let Some(cell) = board.get_first_non_fixed_zero() {
                    cell
                } else {
                    if board.is_correct() {
                        return Ok((true, board.to_str().clone()));
                    } else {
                        if let Some(cell) = board.get_last_non_fixed_non_zero() {
                            cell
                        } else {
                            // no solution exists
                            return Ok((false, "".to_string()));
                        }
                    }
                }
            } else {
                if let Some(cell) = board.get_last_non_fixed_non_zero() {
                    cell
                } else {
                    // no solution exists
                            return Ok((false, "".to_string()));
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