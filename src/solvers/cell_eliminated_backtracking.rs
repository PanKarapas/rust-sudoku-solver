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
pub struct CellEliminatedBacktrackingSolver;

impl Solver for CellEliminatedBacktrackingSolver {
    fn solve(&self, puzzle: &'static str) -> Result<(bool, String), &'static str> {
        let mut board = match Board::<ConstrainedCell>::parse_puzzle_string(puzzle) {
            Err(error) => return Err(error),
            Ok(b) => b,
        };

        CellEliminatedBacktrackingSolver::calculate_fixed_board_constrains_until_stable(&mut board);

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

impl CellEliminatedBacktrackingSolver {
    // Only works on fully fixed boards
    // if any cells are not fixed but have values, it will not work as expected
    fn calculate_fixed_board_constrains_until_stable(board: &mut Board<ConstrainedCell>) {
        // Each of these represents the values that are disallowed (because a fixed cell already has them)
        // in the row, column, and square
        let mut row_forbidden: [u16; 9] = [0u16; 9];
        let mut col_forbidden: [u16; 9] = [0u16; 9];
        let mut square_forbidden: [u16; 9] = [0u16; 9];
        // Cells that need their individual constrains re-checked, initially all non fixed cells
        let mut check_queue: Vec<CellPosition> = Vec::with_capacity(81);
        // so we don't push the same pos in twice
        let mut in_queue = [[false; 9]; 9];

        ConstrainedCell::calculate_forbidden_matrices(
            board,
            &mut row_forbidden,
            &mut col_forbidden,
            &mut square_forbidden,
            &mut check_queue,
            &mut in_queue,
            // unused
            &mut [[false; 9]; 9]
        );

        while let Some(pos_to_check) = check_queue.pop() {
            let cell = &mut board.0[pos_to_check.row as usize][pos_to_check.column as usize];
            let position = cell.position;
            in_queue[position.row as usize][position.column as usize] = false;
            let square_index = (((position.row / 3) * 3) + (position.column / 3)) as usize;
            let forbidden = row_forbidden[position.row as usize]
                | col_forbidden[position.column as usize]
                | square_forbidden[square_index];

            // Update cell constraints
            for i in 1..=9 {
                if ((forbidden >> i) & 1u16) > 0
                    && cell.value_constraint_map[i as usize] == ValueConstraint::Allowed
                {
                    cell.value_constraint_map[i as usize] = ValueConstraint::FixedNotAllowed;
                }
            }
            // If there is only 1 value the cell can take
            // 7 for the unused bits + 1 for the available
            if forbidden.count_zeros() == 8 {
                cell.fixed = true;
                let new_value = ((!forbidden) >> 1).trailing_zeros() as i8 + 1;
                cell.value = CellValue::Filled(new_value);
                // Update forbitten masks with new value
                row_forbidden[position.row as usize] |= 1u16 << new_value;
                col_forbidden[position.column as usize] |= 1u16 << new_value;
                square_forbidden[square_index] |= 1u16 << new_value;

                // Get all non fixed cells that might be affected by this, push them to the queue to be re-checked
                let square_start_row = (pos_to_check.row / 3) * 3;
                let square_start_col = (pos_to_check.column / 3) * 3;
                for i in 0..=8 {
                    if i != pos_to_check.column
                        && !board.0[pos_to_check.row as usize][i as usize].fixed
                        && !in_queue[pos_to_check.row as usize][i as usize]
                    {
                        check_queue.push(CellPosition {
                            row: pos_to_check.row,
                            column: i,
                        });
                        in_queue[pos_to_check.row as usize][i as usize] = true;
                    }

                    if i != pos_to_check.row
                        && !board.0[i as usize][pos_to_check.column as usize].fixed
                        && !in_queue[i as usize][pos_to_check.column as usize]
                    {
                        check_queue.push(CellPosition {
                            row: i,
                            column: pos_to_check.column,
                        });
                        in_queue[i as usize][pos_to_check.column as usize] = true;
                    }
                    let square_cell_row = square_start_row + (i / 3);
                    let square_cell_column = square_start_col + (i % 3);
                    if (square_cell_row != pos_to_check.row
                        || square_cell_column != pos_to_check.column)
                        && !board.0[square_cell_row as usize][square_cell_column as usize].fixed
                        && !in_queue[square_cell_row as usize][square_cell_column as usize]
                    {
                        check_queue.push(CellPosition {
                            row: square_cell_row,
                            column: square_cell_column,
                        });
                        in_queue[square_cell_row as usize][square_cell_column as usize] = true;
                    }
                }
            }
        }
    }
}
