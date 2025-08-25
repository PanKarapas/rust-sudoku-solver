use rust_sudoku_solver::board::cell::simple_cell::SimpleCell;

#[test]
fn parse_valid_board() {
    let puzzle =
        "8..45.6...3.68.......3.28.11.....2...6.....5...9.....89.78.6.......29.7...6.34..9";
    let board = rust_sudoku_solver::board::Board::<SimpleCell>::parse_puzzle_string(puzzle);
    assert!(board.is_ok());
}
#[test]
fn dont_parse_invalid_board_invalid_char() {
    let puzzle =
        "8..45.6...3.68.......3.28.11.....2...6.....5...9.....89.78.6......29.7..a.6.34..9";
    let board = rust_sudoku_solver::board::Board::<SimpleCell>::parse_puzzle_string(puzzle);
    assert!(board.is_err());
}

#[test]
fn dont_parse_invalid_board_too_short() {
    let puzzle = "8..45.6...3.68.......3.28.11.....2...6.....5...9.....89.78.6......29.7...6.34..9";
    let board = rust_sudoku_solver::board::Board::<SimpleCell>::parse_puzzle_string(puzzle);
    assert!(board.is_err());
}

#[test]
fn dont_parse_invalid_board_too_long() {
    let puzzle =
        "8..45.6...3.68.......3.28.11.....2...6.....5...9.....89.78.6......29.7...6.34...9.";
    let board = rust_sudoku_solver::board::Board::<SimpleCell>::parse_puzzle_string(puzzle);
    assert!(board.is_err());
}
