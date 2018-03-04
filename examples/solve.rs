extern crate shzhrs;

use std::time::Instant;

fn main() {
    println!("Enter board:");

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    let board = match shzhrs::Board::decode(&input.trim()) {
        Ok(b) => b,
        Err(s) => {
            println!("{}", s);
            std::process::exit(1);
        }
    };

    let start = Instant::now();
    let solution = match shzhrs::solve(&board) {
        None => {
            println!("Could not solve board");
            std::process::exit(1);
        }
        Some((path, _cost)) => path,
    };

    let time = Instant::now() - start;
    println!("Solved in {:?}", time);

    let moves = shzhrs::solution_to_moves(&solution);
    {
        for (m, b) in moves.iter().zip(solution.iter().skip(1)) {
            println!("{:?}\n{}\n", m, b.encode());
        }
    }
}
