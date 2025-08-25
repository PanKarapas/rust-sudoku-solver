use std::{
    fs::File, io::{BufRead, BufReader}, time::{Instant}
};

use criterion::{
    BenchmarkGroup, Criterion, criterion_group, criterion_main, measurement::WallTime,
};
use rust_sudoku_solver::solvers::{
    Solver, backtracking::BacktrackingSolver,
    cell_eliminated_backtracking::CellEliminatedBacktrackingSolver,
    group_eliminated_backtracking::GroupEliminatedBacktrackingSolver,
};

fn bench_solvers(c: &mut Criterion) {
    let mut group = c.benchmark_group("sudoku_solvers");
    #[cfg(not(feature = "heavy"))]
    {
        use std::time::Duration;

        bench_all_solvers(
            &mut group,
            "_simple",
            "8..45.6...3.68.......3.28.11.....2...6.....5...9.....89.78.6.......29.7...6.34..9",
        );

        // Set sample size for harder puzzle to lower to save time
        group.sample_size(10);
        group.measurement_time(Duration::from_secs(2));

        bench_all_solvers(
            &mut group,
            "_hard",
            ".....9.......4..5.68.........4....7....62........8.......9..8.6........3..3..52..",
        );

        bench_all_solvers(
            &mut group,
            "_very_hard",
            "..3.8.......35.....7....6....5.......2...94.7........1.......8..6.....3.1....4...",
        );
    }
    #[cfg(feature = "heavy")]
    bench_heavy(&mut group);

    group.finish();
}

 #[cfg(feature = "heavy")]
fn bench_heavy(group: &mut BenchmarkGroup<'_, WallTime>) {
    let file_names = ["easy.txt", "medium.txt", "hard.txt", "diabolical.txt"];
    let mut puzzles: Vec<String> = Vec::new();
    for file_name in file_names {
        let file = File::open("heavy-data/".to_string() + file_name)
            .expect(("Unable to find file: ".to_string() + file_name).as_str());
        let buf = BufReader::new(file);
        let mut lines: Vec<String> = buf
            .lines()
            .map(|l| l.expect("Could not parse line."))
            .collect();
        puzzles.append(&mut lines);
    }
    let static_puzzles: &'static [String] = Box::leak(puzzles.into_boxed_slice());

    let solvers: &[Box<dyn Solver>] = &[
        Box::new(BacktrackingSolver),
        Box::new(CellEliminatedBacktrackingSolver),
        Box::new(GroupEliminatedBacktrackingSolver),
    ];
    for solver in solvers {
        group.bench_function(solver.name().to_string() + "_heavy", |b| {
            
            b.iter_custom(|iters| {
                let start = Instant::now();
                for _ in 0..iters {
            for puzzle in static_puzzles.iter() {
                assert!(solver.solve(puzzle.as_str()).is_ok_and(|ret| ret.0));
            }
        }
                start.elapsed()
            });
        });
    }
    
}

fn bench_all_solvers(
    group: &mut BenchmarkGroup<'_, WallTime>,
    name_extension: &'static str,
    puzzle: &'static str,
) {
    let solvers: &[Box<dyn Solver>] = &[
        Box::new(BacktrackingSolver),
        Box::new(CellEliminatedBacktrackingSolver),
        Box::new(GroupEliminatedBacktrackingSolver),
    ];

    for solver in solvers {
        group.bench_function(solver.name().to_string() + name_extension, |b| {
            b.iter(|| assert!(solver.solve(puzzle).is_ok_and(|ret| ret.0 == true)))
        });
    }
}

criterion_group!(benches, bench_solvers);

criterion_main!(benches);
