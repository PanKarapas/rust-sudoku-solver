/*notable puzzles :
 *
 * Most Time Needed: .....9.......4..5.68.........4....7....62........8.......9..8.6........3..3..52..
 * 					Best time: 6 s
 *
 * Less Time Needed: 8..45.6...3.68.......3.28.11.....2...6.....5...9.....89.78.6.......29.7...6.34..9
 *
 * 					Best time: 2.4 ms
*/

use std::{
    array, fmt,
    process::{ExitCode, exit},
    time::Instant,
};

fn main() -> ExitCode {
    let puzzle =
        "8..45.6...3.68.......3.28.11.....2...6.....5...9.....89.78.6.......29.7...6.34..9";

    if !check_puzzle_string_valid(puzzle) {
        return ExitCode::FAILURE;
    }

    let mut board: Board = parse_puzzle_string(puzzle);
    println!("Board:");
    println!("{}", board);

    let start = Instant::now();
    let solved = board.solve();
    let duration = start.elapsed();
    println!("Time to solve is: {:?}", duration);

    if !solved {
        println!("Unsolvable...");
    } else {
        println!("Solution:");
        println!("{}", board);

    }
    return ExitCode::SUCCESS;
}

#[derive(Clone)]
struct Cell {
    value: CellValue,
    fixed: bool,
}

#[derive(Clone, PartialEq)]
enum CellValue {
    Empty,
    Filled(i8),
}
struct IncrementResult {
        is_board_valid: bool,
        needs_revalidation: bool,
}
impl Cell {
    fn increment(&mut self) -> IncrementResult {
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

impl fmt::Display for CellValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => write!(f, "_"),
            Self::Filled(val) => write!(f, "{}", val),
        }
    }
}

struct Board([[Cell; 9]; 9]);

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
impl Board {
    fn get_row(&self, y: i8) -> [&Cell; 9] {
        std::array::from_fn(|i| &self.0[y as usize][i])
    }

    fn get_col(&self, x: i8) -> [&Cell; 9] {
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

    fn get_square(&self, x: i8, y: i8) -> [&Cell; 9] {
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

    fn is_correct(&self) -> bool {
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

    fn get_first_non_fixed_zero(&mut self) -> Option<&mut Cell> {
        self.0
            .iter_mut()
            .flatten()
            .find(|cell| cell.value == CellValue::Empty && !cell.fixed)
    }

    fn get_last_non_fixed_non_zero(&mut self) -> Option<&mut Cell> {
        self.0
            .iter_mut()
            .flatten()
            .rfind(|cell| cell.value != CellValue::Empty && !cell.fixed)
    }

    fn solve(&mut self) -> bool {
        let mut curr_cell: &mut Cell;
        let mut is_valid = true;
        loop {
            // If the current board is valid (no duplicate values)
            curr_cell = if is_valid {
                if let Some(cell) = self.get_first_non_fixed_zero() {
                    cell
                } else {
                    if self.is_correct() {
                        return true;
                    } else {
                        if let Some(cell) = self.get_last_non_fixed_non_zero() {
                            cell
                        } else {
                            // no solution exists
                            return false;
                        }
                    }
                }
            } else {
                if let Some(cell) = self.get_last_non_fixed_non_zero() {
                    cell
                } else {
                    // no solution exists
                    return false;
                }
            };
            let increment_result = curr_cell.increment();

            if increment_result.needs_revalidation {
                is_valid = self.is_correct();
            } else {
                is_valid = increment_result.is_board_valid;
            }
        }
    }
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

fn parse_puzzle_string(puzzle: &str) -> Board {
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

    return board;
}

fn check_puzzle_string_valid(puzzle: &str) -> bool {
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
