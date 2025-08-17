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
    process::{ExitCode},
    time::Instant,
};
mod board;

use crate::board::Board;

use crate::board::{parse_puzzle_string};

fn main() -> ExitCode {
    let puzzle =
        "8..45.6...3.68.......3.28.11.....2...6.....5...9.....89.78.6.......29.7...6.34..9";

    let mut board: Board = match parse_puzzle_string(puzzle) {
        None => return ExitCode::FAILURE,
        Some(value) => value
    };
    
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

