extern crate rayon;
extern crate shzhrs;

use std::time::Instant;
use rayon::prelude::*;

fn main() {
    rayon::ThreadPoolBuilder::new()
        .num_threads(4)
        .build_global()
        .unwrap();

    (0..1000)
        .into_par_iter()
        .map(|_| {
            let board = shzhrs::new_game();
            let encoded = board.encode();
            let start = Instant::now();
            match shzhrs::solve(&board) {
                None => {
                    let time = Instant::now() - start;
                    (false, time, encoded)
                }
                Some((_path, _cost)) => {
                    let time = Instant::now() - start;
                    //let boards = path.iter().map(|b| b.encode()).collect::<Vec<_>>();
                    // println!(
                    //     "solved {:?}\n{:#?}\ncost: {} time: {:?}",
                    //     encoded, boards, cost, time
                    // );
                    (true, time, encoded)
                }
            }
        })
        .for_each(|(solved, time, encoded)| {
            println!("Solved: {:?}, Time: {:?}, Board:{} ", solved, time, encoded);
        });
}
