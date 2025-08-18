use crate::board::cell::{CellValue, IsCell};

#[derive(Clone)]
pub struct SimpleCell {
    pub value: CellValue,
    pub fixed: bool,
}

impl IsCell for SimpleCell {
    fn value(&self) -> &CellValue {
        &self.value
    }

    fn is_fixed(&self) -> bool {
        self.fixed
    }

    fn init(value: CellValue, fixed: bool) -> Self {
        SimpleCell {
            value: value,
            fixed: fixed,
        }
    }
}

pub struct IncrementResult {
    pub is_board_valid: bool,
    pub needs_revalidation: bool,
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
