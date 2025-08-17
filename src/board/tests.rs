
#[test]
fn parse_valid_board() {
    let puzzle =
        "8..45.6...3.68.......3.28.11.....2...6.....5...9.....89.78.6.......29.7...6.34..9";
    let board = crate::board::parse_puzzle_string(puzzle);
    assert_eq!(board.is_some(), true);
}
#[test]
fn dont_parse_invalid_board_invalid_char() {
    let puzzle =
        "8..45.6...3.68.......3.28.11.....2...6.....5...9.....89.78.6......29.7..a.6.34..9";
    let board = crate::board::parse_puzzle_string(puzzle);
    assert_eq!(board.is_some(), false);
}

#[test]
fn dont_parse_invalid_board_too_short() {
    let puzzle = "8..45.6...3.68.......3.28.11.....2...6.....5...9.....89.78.6......29.7...6.34..9";
    let board = crate::board::parse_puzzle_string(puzzle);
    assert_eq!(board.is_some(), false);
}

#[test]
fn dont_parse_invalid_board_too_long() {
    let puzzle =
        "8..45.6...3.68.......3.28.11.....2...6.....5...9.....89.78.6......29.7...6.34...9.";
    let board = crate::board::parse_puzzle_string(puzzle);
    assert_eq!(board.is_some(), false);
}
