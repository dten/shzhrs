extern crate ansi_term;
extern crate shzhrs;

use std::time::Instant;

fn main() {
    #[cfg(windows)]
    {
        ansi_term::enable_ansi_support().unwrap()
    };

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
    println!("Solved in {:?}\n", time);

    let moves = shzhrs::solution_to_moves(&solution);
    {
        use ansi_term::Style;
        use ansi_term::Colour::{Black, Green, Red, RGB};
        use shzhrs::{Card, Move, Suit};
        let bold = Style::new().bold();
        let black = Style::new().on(RGB(128, 128, 128)).fg(Black);
        let green = bold.fg(Green);
        let red = bold.fg(Red);
        let display_card = |c: &Card| {
            let card = format!("{}", c);
            match c.suit() {
                Some(&Suit::Black) => black.paint(card),
                Some(&Suit::Green) => green.paint(card),
                Some(&Suit::Red) => red.paint(card),
                None => Style::new().paint(card),
            }
        };
        for (m, b) in moves.iter().zip(solution.iter().skip(1)) {
            match *m {
                Move::PileToPile(ref card, ref from, ref to) => {
                    println!(
                        "{} {} from pile {} to pile {}",
                        bold.paint("Move"),
                        display_card(card),
                        bold.paint(format!("{}", from + 1)),
                        bold.paint(format!("{}", to + 1)),
                    );
                }
                Move::Place(ref card) => {
                    println!(
                        "{} {}",
                        bold.paint("Place"),
                        display_card(&Card::Value(card.clone()))
                    );
                }
                Move::PileToSpare(ref card, ref from) => {
                    println!(
                        "{} {} from pile {} to {}",
                        bold.paint("Move"),
                        display_card(card),
                        bold.paint(format!("{}", from + 1)),
                        bold.paint("spares"),
                    );
                }
                Move::SpareToPile(ref card, ref to) => {
                    println!(
                        "{} {} from {} to pile {}",
                        bold.paint("Move"),
                        display_card(card),
                        bold.paint("spares"),
                        bold.paint(format!("{}", to + 1)),
                    );
                }
                Move::Flower() => {
                    println!("{}", bold.paint("Flower power!"));
                }
                Move::DragonStack(ref suit) => {
                    let dragons = match *suit {
                        Suit::Black => black.paint("Black Dragons"),
                        Suit::Green => green.paint("Green Dragons"),
                        Suit::Red => red.paint("Red Dragons"),
                    };
                    println!("{} {}", bold.paint("Stack"), dragons);
                }
            }
            println!("{}\n", b.encode());
        }
    }
}
