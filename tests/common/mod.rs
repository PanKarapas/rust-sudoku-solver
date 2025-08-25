use rust_sudoku_solver::solvers::backtracking::BacktrackingSolver;
use rust_sudoku_solver::solvers::{
    Solver, cell_eliminated_backtracking::CellEliminatedBacktrackingSolver,
    group_eliminated_backtracking::GroupEliminatedBacktrackingSolver,
};

pub fn check_all_solvers(puzzle: &'static str, solution: Option<&'static str>) {
    check_solver(BacktrackingSolver, puzzle, solution);
    check_solver(CellEliminatedBacktrackingSolver, puzzle, solution);
    check_solver(GroupEliminatedBacktrackingSolver, puzzle, solution);
}


fn check_solver<S: Solver>(solver: S, board: &'static str, expect: Option<&'static str>) {
    let res = solver.solve(board);
    match expect {
        None => {
            if res.is_ok_and(|ret| ret.0 == true) {
                assert!(false, "{} solved impossible puzzle: {board}", solver.name())
            }
        }
        Some(solution) => match res {
            Err(error) => assert!(
                false,
                "{} failed to parse puzzle {board} with error: {error}",
                solver.name()
            ),
            Ok((_, actual)) => assert_eq!(
                actual,
                solution,
                "{} failed to solve puzzle {}",
                solver.name(),
                board
            ),
        },
    }
}