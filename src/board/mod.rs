use std::array;

use crate::board::cell::{CellPosition, CellValue, IsCell};

pub mod cell;
#[cfg(test)]
mod tests;

#[derive(Clone, Debug)]
pub struct Board<T>(pub [[T; 9]; 9])
where
    T: IsCell;

impl<T> Board<T>
where
    T: IsCell,
{
    pub fn get_row(&self, y: i8) -> [&T; 9] {
        std::array::from_fn(|i| &self.0[y as usize][i])
    }

    pub fn get_col(&self, x: i8) -> [&T; 9] {
        [
            &self.0[0][x as usize],
            &self.0[1][x as usize],
            &self.0[2][x as usize],
            &self.0[3][x as usize],
            &self.0[4][x as usize],
            &self.0[5][x as usize],
            &self.0[6][x as usize],
            &self.0[7][x as usize],
            &self.0[8][x as usize],
        ]
    }

    pub fn get_square(&self, x: i8, y: i8) -> [&T; 9] {
        let x_in_square = x % 3;

        let y_in_square = y % 3;

        let square_corner_x = x - x_in_square;
        let square_corner_y = y - y_in_square;

        array::from_fn(|i| {
            let y_diff = i / 3;
            let x_diff = i % 3;

            &self.0[(square_corner_y) as usize + y_diff][(square_corner_x) as usize + x_diff]
        })
    }

    pub fn get_mut_cell(&mut self, position: &CellPosition) -> &mut T {
        &mut self.0[position.row as usize][position.column  as usize]
    }

    pub fn is_cell_valid(&self, cell_position: CellPosition) -> bool {

        if !Self::is_group_correct(self.get_row(cell_position.row)) {
            return false;
        }

        if !Self::is_group_correct(self.get_col(cell_position.column)) {
            return false;
        }

        if !Self::is_group_correct(self.get_square(cell_position.column, cell_position.row)) {
            return false;
        }

        return true;
    }

    pub fn is_correct(&self) -> bool {

        if (0..9).any(|row| !Self::is_group_correct(self.get_row(row))) {
            return false;
        }

        if (0..9).any(|col| !Self::is_group_correct(self.get_col(col))) {
            return false;
        }

        if (0..9).any(|i| !Self::is_group_correct(self.get_square((i % 3) * 3, (i / 3) * 3))) {
            return false;
        }

        return true;
    }

    pub fn get_first_non_fixed_zero(&self) -> Option<&T> {
        self.0
            .iter()
            .flatten()
            .find(|cell| *cell.value() == CellValue::Empty && !cell.is_fixed())
    }

    pub fn get_last_non_fixed_non_zero(&self) -> Option<&T> {
        self.0
            .iter()
            .flatten()
            .rfind(|cell| *cell.value() != CellValue::Empty && !cell.is_fixed())
    }

    pub fn to_str(&self) -> String {
        self.0
            .iter()
            .map(|row| {
                row.clone()
                    .map(|cell| match cell.value() {
                        CellValue::Empty => ".".to_string(),
                        CellValue::Filled(val) => val.to_string(),
                    })
                    .concat()
            })
            .reduce(|coll, substr| coll + substr.as_str())
            .expect("Failed to parse board to str")
    }
    pub fn parse_puzzle_string(puzzle: &str) -> Result<Board<T>, &'static str> {
        match Self::check_puzzle_string_valid(puzzle) {
            Err(error) => return Err(error),
            Ok(_) => {}
        }

        let flat = puzzle
            .chars()
            .enumerate()
            .map(|(i, c)| {
                T::init(
                    {
                        if c == '.' {
                            CellValue::Empty
                        } else {
                            CellValue::Filled(c.to_digit(10).unwrap_or_default() as i8)
                        }
                    },
                    c != '.',
                    CellPosition {
                        row: (i / 9) as i8,
                        column: (i % 9) as i8
                    }
                )
            })
            .collect::<Vec<T>>();

        let board: Board<T> = Board::<T>(core::array::from_fn(|y| {
            core::array::from_fn(|x| flat[(y * 9) + x].clone())
        }));

        return Ok(board);
    }

    fn check_puzzle_string_valid(puzzle: &str) -> Result<(), &'static str> {
        if puzzle.len() != 81 {
            return Err("Input string has the wrong length.");
        }

        if puzzle.find(|c: char| !char::is_numeric(c) && c != '.') != None {
            return Err("Invalid char in input string.");
        }

        if puzzle.find(|c: char| c == '0') != None {
            return Err("Found 0 in input string.");
        }

        return Ok(());
    }

    // Checks if any group (9 cells) has any duplicates excluding 0s
    fn is_group_correct(group: [&T; 9]) -> bool
    where
        T: IsCell,
    {
        let mut seen = [false; 10];
        for cell in group {
            match cell.value() {
                CellValue::Empty => continue,
                CellValue::Filled(val) => {
                    if seen[*val as usize] {
                        return false;
                    }
                    seen[*val as usize] = true;
                }
            }
        }
        return true;
    }
}
