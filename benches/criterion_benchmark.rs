#[macro_use]
extern crate criterion;
extern crate shzhrs;

use criterion::{Bencher, Criterion};
use shzhrs::*;

fn criterion_benchmark(c: &mut Criterion) {
    fn bench_solve(b: &mut Bencher, board: &str) {
        let board = Board::decode(board).unwrap();
        b.iter(|| solve(&board.clone()).expect("solve failed"))
    }

    c.bench_function("board 0", |b| {
        bench_solve(b, 
            ";;;;;;;b1b2r9g9g2;rDrDg7b8g3;r5g6ffb3rD;b9g5r2b7b4;r1g1r3gDrD;r8r7g4bDbD;b6gDgDbDb5;g8bDgDr6r4");
    });
    c.bench_function("board 1", |b| {
        bench_solve(b, 
            ";;;;;;;rDb6b3g5b5;r5r1g2rDrD;bDg3g4gDgD;b4b8g8g1ff;g6r7r3b1r2;r8b2bDr6b7;g7g9b9bDr4;gDbDr9gDrD");
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
