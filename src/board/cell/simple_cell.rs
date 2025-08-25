use crate::board::cell::{CellPosition, CellValue, IncrementResult, IsCell};

#[derive(Clone, Debug)]
pub struct SimpleCell {
    pub value: CellValue,
    pub fixed: bool,
    pub position: CellPosition
}

impl IsCell for SimpleCell {
    fn value(&self) -> &CellValue {
        &self.value
    }

    fn is_fixed(&self) -> bool {
        self.fixed
    }

    fn new(value: CellValue, fixed: bool, position: CellPosition) -> Self {
        SimpleCell {
            value,
            fixed,
            position
        }
    }
}

impl SimpleCell {
    pub fn increment(&mut self) -> IncrementResult {
        match self.value {
            CellValue::Empty => {
                self.value = CellValue::Filled(1);
                IncrementResult {
                    is_board_valid: true,
                    needs_revalidation: true,
                }
            }
            CellValue::Filled(val) => {
                if val >= 9 {
                    self.value = CellValue::Empty;
                    IncrementResult {
                        is_board_valid: false,
                        needs_revalidation: false,
                    }
                } else {
                    self.value = CellValue::Filled(val + 1);
                    IncrementResult {
                        is_board_valid: false,
                        needs_revalidation: true,
                    }
                }
            }
        }
    }
}
