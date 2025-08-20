use crate::{
    board::{
        Board,
        cell::{
            CellPosition, CellValue,
            constrained_cell::{ConstrainedCell, ValueConstraint},
        },
    },
    solvers::Solver,
};
pub struct ConstrainedBackTrackingSolver;

impl Solver for ConstrainedBackTrackingSolver {
    fn solve(&self, puzzle: &'static str) -> Result<(bool, String), &'static str> {
        let mut board = match Board::<ConstrainedCell>::parse_puzzle_string(puzzle) {
            Err(error) => return Err(error),
            Ok(b) => b,
        };

        ConstrainedBackTrackingSolver::calculate_fixed_board_constrains_until_stable(&mut board);

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

impl ConstrainedBackTrackingSolver {
    fn calculate_fixed_board_constrains_until_stable(board: &mut Board<ConstrainedCell>) {
        while ConstrainedBackTrackingSolver::calculate_fixed_board_constrains(board) {}
    }
    fn calculate_fixed_board_constrains(board: &mut Board<ConstrainedCell>) -> bool {
        let mut changed = false;
        let starting_board = &*board;
        let mut forbidden_map: [[[bool;10]; 9]; 9] = [[[false; 10]; 9]; 9];
        for row in &board.0 {
            for cell in row {
                if !cell.fixed {
                    let position = cell.position;
                    // Boolean array: forbidden[i] = true if value i is not allowed
                    let mut forbidden = [false; 10];

                    // Mark row constraints
                    for c in starting_board.get_row(position.row) {
                        if c.position.column != position.column {
                            if let CellValue::Filled(val) = c.value {
                                forbidden[val as usize] = true;
                            }
                        }
                    }

                    // Mark column constraints
                    for c in starting_board.get_col(position.column) {
                        if c.position.row != position.row {
                            if let CellValue::Filled(val) = c.value {
                                forbidden[val as usize] = true;
                            }
                        }
                    }
                    // Mark square constraints
                    for c in starting_board.get_square(position.column, position.row) {
                        if c.position != position {
                            if let CellValue::Filled(val) = c.value {
                                forbidden[val as usize] = true;
                            }
                        }
                    }
                    forbidden_map[cell.position.row as usize][cell.position.column as usize] = forbidden;
                }
            }
        }
        for row in &mut board.0 {
                 for cell in row {
                    let position = cell.position;
                    let forbidden = forbidden_map[position.row as usize][position.column as usize];

                    let mut allowed_val = None;
                    let mut only_one = true;
                    for i in 1..=9 {
                        if forbidden[i as usize] && cell.value_constraint_map[i as usize] == ValueConstraint::Allowed {
                            cell.value_constraint_map[i as usize] = ValueConstraint::FixedNotAllowed;
                            changed = true;
                        }
                        if only_one && cell.value_constraint_map[i as usize] == ValueConstraint::Allowed {
                            if allowed_val.is_some() {
                                only_one = false;
                            }
                            allowed_val = Some(i);
                        }
                    }

                    if only_one {
                        if let Some(val) = allowed_val {
                            cell.fixed = true;
                            cell.value = CellValue::Filled(val as i8);
                        }
                    }
                }
        }
        changed
    }
}
