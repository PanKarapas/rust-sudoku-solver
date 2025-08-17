// src/board.rs
use std::{array, fmt};

pub struct Board(pub [[Cell; 9]; 9]);

#[derive(Clone)]
pub struct Cell {
    pub value: CellValue,
    pub fixed: bool,
}

#[derive(Clone, PartialEq)]
pub enum CellValue {
    Empty,
    Filled(i8),
}

impl Board {
    pub fn get_row(&self, y: i8) -> [&Cell; 9] {
        std::array::from_fn(|i| &self.0[y as usize][i])
    }

    pub fn get_col(&self, x: i8) -> [&Cell; 9] {
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

    pub fn get_square(&self, x: i8, y: i8) -> [&Cell; 9] {
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

    pub fn is_correct(&self) -> bool {
        if (0..9).any(|row| !is_group_correct(self.get_row(row))) {
            return false;
        }
        if (0..9).any(|col| !is_group_correct(self.get_col(col))) {
            return false;
        }

        if (0..9).any(|i| !is_group_correct(self.get_square((i % 3) * 3, (i / 3) * 3))) {
            return false;
        }

        return true;
    }

    pub fn get_first_non_fixed_zero(&mut self) -> Option<&mut Cell> {
        self.0
            .iter_mut()
            .flatten()
            .find(|cell| cell.value == CellValue::Empty && !cell.fixed)
    }

    pub fn get_last_non_fixed_non_zero(&mut self) -> Option<&mut Cell> {
        self.0
            .iter_mut()
            .flatten()
            .rfind(|cell| cell.value != CellValue::Empty && !cell.fixed)
    }
}

pub struct IncrementResult {
        pub is_board_valid: bool,
        pub needs_revalidation: bool,
}

impl Cell {
    pub fn increment(&mut self) -> IncrementResult {
        match self.value {
            CellValue::Empty => {
                self.value = CellValue::Filled(1);
                IncrementResult {is_board_valid: true, needs_revalidation: true}
            },
            CellValue::Filled(val) => {
                if val >= 9 {
                    self.value = CellValue::Empty;
                    IncrementResult {is_board_valid: false, needs_revalidation: false}
                } else {
                    self.value = CellValue::Filled(val + 1);
                    IncrementResult {is_board_valid: false, needs_revalidation: true}
                }
            }
        }
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.0 {
            for cell in row {
                write!(f, "{}", cell.value)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl fmt::Display for CellValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => write!(f, "_"),
            Self::Filled(val) => write!(f, "{}", val),
        }
    }
}

pub fn check_puzzle_string_valid(puzzle: &str) -> bool {
    if puzzle.len() != 81 {
        print!("Input string has the wrong length.");
        return false;
    }

    if puzzle.find(|c: char| !char::is_numeric(c) && c != '.') != None {
        print!("Invalid char in input string.");
        return false;
    }

    if puzzle.find(|c: char| c == '0') != None {
        print!("Found 0 in input string.");
        return false;
    }

    return true;
}

pub fn parse_puzzle_string(puzzle: &str) -> Option<Board> {
    if !check_puzzle_string_valid(puzzle) {
        return None;
    }
    let flat = puzzle
        .chars()
        .map(|c| Cell {
            value: {
                if c == '.' {
                    CellValue::Empty
                } else {
                    CellValue::Filled(c.to_digit(10).unwrap_or_default() as i8)
                }
            },
            fixed: c != '.',
        })
        .collect::<Vec<Cell>>();

    let board: Board = Board(core::array::from_fn(|y| {
        core::array::from_fn(|x| flat[(y * 9) + x].clone())
    }));

    return Some(board);
}

// Checks if any group (9 cells) has any duplicates excluding 0s
fn is_group_correct(group: [&Cell; 9]) -> bool {
    let mut seen = [false; 10];
    for cell in group {
        match cell.value {
            CellValue::Empty => continue,
            CellValue::Filled(val) => {
                if seen[val as usize] {
                    return false;
                }
                seen[val as usize] = true;
            }
        }
    }
    return true;
}