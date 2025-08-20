use crate::board::cell::{CellPosition, CellValue, IncrementResult, IsCell};

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

    fn init(value: CellValue, fixed: bool, position: CellPosition) -> Self {
        ConstrainedCell {
            value,
            fixed,
            position,
            value_constraint_map: [ValueConstraint::Allowed; 10],
        }
    }
}
