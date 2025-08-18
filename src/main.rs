/*notable puzzles :
 *
 * Most Time Needed: .....9.......4..5.68.........4....7....62........8.......9..8.6........3..3..52..
 * 					Best time: 6 s
 *
 * Least Time Needed: 8..45.6...3.68.......3.28.11.....2...6.....5...9.....89.78.6.......29.7...6.34..9
 * 					Best time: 2.4 ms
*/
use std::{
    process::ExitCode, time::Instant
};

mod board;
mod solvers;

use crate::solvers::get_solver;


fn main() -> ExitCode {
    // TODO: get these from args
    let puzzle =
        "8..45.6...3.68.......3.28.11.....2...6.....5...9.....89.78.6.......29.7...6.34..9";
    
    let solver = match get_solver("backtracking") {
        Err(msg) => {
            println!("{}", msg);
            return ExitCode::FAILURE;
        },
        Ok(val) => val
    };
    
    println!("Board:");
    print_puzzle(puzzle.to_string());


    let start = Instant::now();
    let solved = solver.solve(&puzzle);
    let duration = start.elapsed();
    println!("Time to solve is: {:?}", duration);

    match solved {
        Err(error) => println!("Error while solving: {error}"),
        Ok((solved, solved_board)) => {
            if !solved {
                println!("Unsolvable...");
            } else {
                println!("Solution:");
                print_puzzle(solved_board);
            }
        }
    }
    return ExitCode::SUCCESS;
}

fn print_puzzle(puzzle: String) {
    let chars: Vec<char> = puzzle.chars().collect();
    let size = 9;

    for row in 0..size {
        if row % 3 == 0 && row != 0 {
            println!("------+-------+------");
        }

        for col in 0..size {
            if col % 3 == 0 && col != 0 {
                print!("| ");
            }

            let idx = row * size + col;
            let ch = chars[idx];

            if ch == '.' {
                print!(". ");
            } else {
                print!("{} ", ch);
            }
        }
        println!();
    }
}

