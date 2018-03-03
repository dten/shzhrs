extern crate shzhrs;

use std::time::Instant;

fn main() {
    let total = 100;
    let threads = 4;
    let per_thread = total / threads;
    let res = (0..threads)
        .map(|_| {
            std::thread::spawn(move || {
                (0..per_thread)
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
                    .max()
            })
        })
        .collect::<Vec<_>>()
        .into_iter()
        .filter_map(|h| h.join().ok())
        .filter_map(|m| m)
        .max();

    if let Some((longest_time, board)) = res {
        println!("{} took the longest at {:?}", board, longest_time);
    } else {
        println!("Solved nothing");
    }
}
