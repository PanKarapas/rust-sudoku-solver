use crate::common::check_all_solvers;
mod common;

#[test]
fn solves_puzzle() {
    check_all_solvers(
        "8..45.6...3.68.......3.28.11.....2...6.....5...9.....89.78.6.......29.7...6.34..9",
        Some("891457632234681597675392841183975264762148953459263718947816325318529476526734189"),
    );
}

#[test]
fn doesnt_solve_impossible_puzzle() {
    check_all_solvers(
        "4..45.6...3.68.......3.28.11.....2...6.....5...9.....89.78.6.......29.7...6.34..9",
        None,
    );
}

#[cfg(feature = "heavy")]
mod heavy_test {
    use crate::common::check_all_solvers;

    use std::{
        fs::File,
        io::{BufRead, BufReader},
    };
    #[test]
    fn heavy_test() {
        println!("a");
        let file_names = ["easy", "medium", "hard", "diabolical"];
        let mut puzzles: Vec<String> = Vec::new();
        let mut solutions: Vec<String> = Vec::new();
        for file_name in file_names {
            let path = format!("heavy-data/{}", file_name);
            puzzles.append(&mut get_heavy_data_lines(&format!("{}.txt", path)));
            solutions.append(&mut get_heavy_data_lines(&format!(
                "{}_solutions.txt",
                path
            )));
        }
        if puzzles.len() != solutions.len() {
            assert!(false, "Puzzle and solution files had different lengths.")
        }

        // leaks but its not an issue since these are just tests
        let static_puzzles: &'static [String] = Box::leak(puzzles.into_boxed_slice());
        let static_solutions: &'static [String] = Box::leak(solutions.into_boxed_slice());

        for i in 0..static_puzzles.len() {
            check_all_solvers(
                static_puzzles
                    .get(i)
                    .expect("Failed to get puzzle.")
                    .as_str(),
                Some(
                    static_solutions
                        .get(i)
                        .expect("Failed to get solution.")
                        .as_str(),
                ),
            );
        }
    }

    pub fn get_heavy_data_lines(path: &str) -> Vec<String> {
        use std::fs::File;
        use std::io::{BufRead, BufReader};

        let file = File::open(path).expect(&format!("Unable to find file: {}", path));
        let buf = BufReader::new(file);
        buf.lines()
            .map(|l| l.expect("Could not parse line."))
            .collect()
    }
}
