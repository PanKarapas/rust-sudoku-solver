use core::fmt;

pub mod simple_cell;
pub mod constrained_cell;

pub trait IsCell: Clone {
    fn value(&self) -> &CellValue;
    fn is_fixed(&self) -> bool;
    fn init(value: CellValue, fixed: bool, position: CellPosition) -> Self;
}
#[derive(Clone, PartialEq, Eq, Copy, Debug)]
pub struct CellPosition {
    pub row: i8,
    pub column: i8
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum CellValue {
    Empty,
    Filled(i8),
}

impl fmt::Display for CellValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            CellValue::Empty => ".".to_string(),
            CellValue::Filled(val) => val.to_string()
        })
    }
}

pub struct IncrementResult {
    pub is_board_valid: bool,
    pub needs_revalidation: bool,
}