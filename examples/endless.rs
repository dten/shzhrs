extern crate rayon;
extern crate shzhrs;

use std::time::Instant;
use rayon::prelude::*;

fn main() {
    rayon::ThreadPoolBuilder::new()
        .num_threads(4)
        .build_global()
        .unwrap();

    let res = (0..100)
        .collect::<Vec<_>>()
        .par_iter()
        .map(|_| {
            let board = shzhrs::new_game();
            let encoded = board.encode();
            let start = Instant::now();
            match shzhrs::solve(&board) {
                None => panic!("couldn't solve {}", encoded),
                Some((path, cost)) => {
                    let time = Instant::now() - start;
                    let boards = path.iter().map(|b| b.encode()).collect::<Vec<_>>();
                    println!(
                        "solved {:?}\n{:#?}\ncost: {} time: {:?}",
                        encoded, boards, cost, time
                    );
                    return (time, encoded);
                }
            }
        })
        .max();

    if let Some((longest_time, board)) = res {
        println!("{} took the longest at {:?}", board, longest_time);
    } else {
        println!("Solved nothing");
    }
}
