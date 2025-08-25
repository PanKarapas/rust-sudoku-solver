use crate::board::{cell::{CellPosition, CellValue, IncrementResult, IsCell}, Board};

#[derive(Copy, Clone, PartialEq)]
pub enum ValueConstraint {
    Allowed,
    FixedNotAllowed,
}

#[derive(Clone, PartialEq)]
pub struct ConstrainedCell {
    pub value: CellValue,
    pub fixed: bool,
    pub position: CellPosition,
    // pos 0 is ignored to reduce number of +1s
    pub value_constraint_map: [ValueConstraint; 10],
}

impl ConstrainedCell {
    // calculates bit maps for each row, column and square
    // the check_queue gets every cell that is not filled in added to it
    // the in_queue map indicates if a cell has been pushed to the queue
    pub fn calculate_forbidden_matrices(board: &Board<ConstrainedCell>, 
        row_forbidden: &mut [u16; 9], 
        col_forbidden: &mut [u16; 9], 
        square_forbidden: &mut [u16; 9],
        check_queue: &mut Vec<CellPosition>,
        in_queue: &mut [[bool; 9]; 9],
        fixed: &mut [[bool; 9]; 9],
    ) {
        // calculate the forbidden maps, and populate check quue
        for row_index in 0..=8 {
            for col_index in 0..=8 {
                // already has value, presumed fixed
                if let CellValue::Filled(val) = &board.0[row_index][col_index].value {
                    row_forbidden[row_index] |= 1u16 << val;
                    col_forbidden[col_index] |= 1u16 << val;
                    square_forbidden[((row_index / 3) * 3) + (col_index / 3)] |= 1u16 << val;
                    fixed[row_index][col_index] = true;
                } else {
                    // Otherwise we will need to check if it can be constrained
                    check_queue.push(board.0[row_index][col_index].position);
                    in_queue[row_index][col_index] = true;
                }
            }
        }
    }
    pub fn increment(&mut self) -> IncrementResult {
        match self.value {
            CellValue::Empty => match self.get_next_not_constrained(1) {
                // Can't be incremented, set to empty and tell the backtracker to find a new cell
                None => {
                    self.value = CellValue::Empty;
                    IncrementResult {
                        is_board_valid: false,
                        needs_revalidation: false,
                    }
                }
                // There is at least 1 more value to try
                Some(new_val) => {
                    self.value = CellValue::Filled(new_val);
                    IncrementResult {
                        is_board_valid: true,
                        needs_revalidation: true,
                    }
                }
            },
            CellValue::Filled(val) => {
                if val >= 9 {
                    self.value = CellValue::Empty;
                    IncrementResult {
                        is_board_valid: false,
                        needs_revalidation: false,
                    }
                } else {
                    match self.get_next_not_constrained(val + 1) {
                        // Can't be incremented, set to empty and tell the backtracker to find a new cell
                        None => {
                            self.value = CellValue::Empty;
                            IncrementResult {
                                is_board_valid: false,
                                needs_revalidation: false,
                            }
                        }
                        // There is at least 1 more value to try
                        Some(new_val) => {
                            self.value = CellValue::Filled(new_val);
                            IncrementResult {
                                is_board_valid: true,
                                needs_revalidation: true,
                            }
                        }
                    }
                }
            }
        }
    }

    fn get_next_not_constrained(&self, from: i8) -> Option<i8> {
        // Clamp to the valid digit domain [1, 9]; short-circuit if above range.
        let start = if from < 1 {
            1
        } else if from > 9 {
            return None;
        } else {
            from
        };
        (start..=9)
            .find(|&val| self.value_constraint_map[val as usize] == ValueConstraint::Allowed)
    }}

impl IsCell for ConstrainedCell {
    fn value(&self) -> &CellValue {
        &self.value
    }

    fn is_fixed(&self) -> bool {
        self.fixed
    }

    fn new(value: CellValue, fixed: bool, position: CellPosition) -> Self {
        ConstrainedCell {
            value,
            fixed,
            position,
            value_constraint_map: [ValueConstraint::Allowed; 10],
        }
    }
}
