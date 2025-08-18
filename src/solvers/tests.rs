use crate::{board::{Board}, solvers::Solver, solvers::BackTrackingSolver};


fn check_all_solvers(puzzle: &'static str, solution: Option<&'static str>) {
    let board = crate::board::parse_puzzle_string(puzzle);
    match board {
        None => assert!(false, "Puzzle failed to be parsed into a board."),
        Some(board) => {
            check_solver(BackTrackingSolver, board, solution);

        }
    }
}

fn check_solver<S: Solver>(solver: S, puzzle: Board, expect: Option<&'static str>) {
    let mut board = puzzle.clone();
    let res = solver.solve(&mut board);
    match expect {
        None => {
            if res {
                assert!(!res, "{} solved impossible puzzle: {puzzle}", solver.name())
            }
        },
        Some(solution) => assert_eq!(board.to_str(), solution, "{} failed to solve puzzle {}", solver.name(), puzzle.to_str())
    }
}

#[test]
fn solves_puzzle() {
    check_all_solvers(
        "8..45.6...3.68.......3.28.11.....2...6.....5...9.....89.78.6.......29.7...6.34..9",
Some("891457632234681597675392841183975264762148953459263718947816325318529476526734189"));
}

#[test]
fn doesnt_solve_impossible_puzzle() {
    check_all_solvers("4..45.6...3.68.......3.28.11.....2...6.....5...9.....89.78.6.......29.7...6.34..9", None);
}
    