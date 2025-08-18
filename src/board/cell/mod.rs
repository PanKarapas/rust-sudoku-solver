
pub mod simple_cell;

pub trait IsCell: Clone {
    fn value(&self) -> &CellValue;
    fn is_fixed(&self) -> bool;
    fn init(value: CellValue, fixed: bool) -> Self;
}

#[derive(Clone, PartialEq)]
pub enum CellValue {
    Empty,
    Filled(i8),
}