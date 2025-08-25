use crate::{board::{cell::{simple_cell::SimpleCell, CellPosition}, Board}, solvers::Solver};

pub struct BacktrackingSolver;
impl Solver for BacktrackingSolver {
    fn solve(&self, puzzle: &'static str) -> Result<(bool, String), &'static str> {
        let mut board = match Board::<SimpleCell>::parse_puzzle_string(puzzle) {
            Err(error) => return Err(error),
            Ok(b) => b
        };
        let mut curr_cell_pos: CellPosition;
        let mut is_valid = true;
        loop {
            // If the current board is valid (no duplicate values)
            curr_cell_pos = if is_valid {
                if let Some(cell) = board.get_first_non_fixed_zero() {
                    cell.position.clone()
                } else {
                    if board.is_correct() {
                        return Ok((true, board.to_str().clone()));
                    } else {
                        if let Some(cell) = board.get_last_non_fixed_non_zero() {
                            cell.position.clone()
                        } else {
                            return Ok((false, "".to_string()));
                        }
                    }
                }
            } else {
                if let Some(cell) = board.get_last_non_fixed_non_zero() {
                    cell.position.clone()
                } else {
                    return Ok((false, "".to_string()));
                }
            };

            let mut_cell = board.get_mut_cell(&curr_cell_pos);
            let increment_result = mut_cell.increment();

            if increment_result.needs_revalidation {
                is_valid = board.is_cell_valid(curr_cell_pos);
            } else {
                is_valid = increment_result.is_board_valid;
            }
        }
    }
}