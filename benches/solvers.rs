use std::time::Duration;

use criterion::{
    BenchmarkGroup, Criterion, criterion_group, criterion_main, measurement::WallTime,
};
use {
    rust_sudoku_solver::board::parse_puzzle_string,
    rust_sudoku_solver::solvers::{Solver, backtracking::BackTrackingSolver},
};

fn bench_solvers(c: &mut Criterion) {
    let mut group = c.benchmark_group("sudoku_solvers");

    // Backtracking
    bench_solver(
        &mut group,
        Box::new(BackTrackingSolver),
        "_simple",
        "8..45.6...3.68.......3.28.11.....2...6.....5...9.....89.78.6.......29.7...6.34..9"
    );
    // Set sample size for harder puzzle to lower to save time
    group.sample_size(10);
    group.measurement_time(Duration::from_secs(2));

    bench_solver(
        &mut group,
        Box::new(BackTrackingSolver),
        "_hard",
        ".....9.......4..5.68.........4....7....62........8.......9..8.6........3..3..52.."
    );

    group.finish();
}

fn bench_solver(
    group: &mut BenchmarkGroup<'_, WallTime>,
    solver: Box<dyn Solver>,
    name_extension: &'static str,
    puzzle: &'static str,

) {
    let board = match parse_puzzle_string(puzzle) {
        None => {
            assert!(false);
            return;
        }
        Some(val) => val,
    };

    group.bench_function(solver.name().to_string() + name_extension, |b| {
        b.iter(|| assert!(solver.solve(&mut board.clone())))
    });
}

criterion_group!(benches, bench_solvers);
criterion_main!(benches);
