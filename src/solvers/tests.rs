use crate::solvers::{constrained_backtracking::ConstrainedBackTrackingSolver, BackTrackingSolver, Solver};


fn check_all_solvers(puzzle: &'static str, solution: Option<&'static str>) {
    check_solver(BackTrackingSolver,puzzle, solution);
    check_solver(ConstrainedBackTrackingSolver,puzzle, solution);
}

fn check_solver<S: Solver>(solver: S, board: &'static str, expect: Option<&'static str>) {
    let res = solver.solve(board);
    match expect {
        None => {
            if res.is_ok_and(|ret| ret.0 == true) {
                assert!(false, "{} solved impossible puzzle: {board}", solver.name())
            }
        },
        Some(solution) => {
            match res {
                Err(error) => assert!(false, "{} failed to parse puzzle {board} with error: {error}", solver.name()),
                Ok((_, actual)) => assert_eq!(actual, solution, "{} failed to solve puzzle {}", solver.name(), board)
            }
        }
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
    