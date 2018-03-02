#![allow(unused)]
#![allow(dead_code)]

extern crate rand;
extern crate smallvec;

use rand::Rng;
use smallvec::SmallVec;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
enum Suit {
    Black,
    Green,
    Red,
}

#[derive(Debug, Eq, PartialEq, Hash)]
struct ValueCard(Suit, u8);

#[derive(Debug, Eq, PartialEq, Hash)]
struct FlowerCard;

#[derive(Debug, Eq, PartialEq, Hash)]
enum Card {
    Value(ValueCard),
    Dragon(Suit),
    Flower(FlowerCard),
}

#[derive(Debug, Eq, PartialEq, Hash)]
enum Spare {
    Empty,
    Card(Card),
    DragonStack(Suit),
}

#[derive(Debug, Eq, PartialEq, Hash)]
enum Place {
    Empty,
    Card(ValueCard),
}

#[derive(Debug, Eq, PartialEq, Hash)]
struct Board {
    spares: [Spare; 3],
    flower: Option<FlowerCard>,
    places: [Place; 3],
    piles: [SmallVec<[Card; 13]>; 8],
}

impl Default for Board {
    fn default() -> Board {
        Board {
            spares: [Spare::Empty, Spare::Empty, Spare::Empty],
            flower: None,
            places: [Place::Empty, Place::Empty, Place::Empty],
            piles: Default::default(),
        }
    }
}

impl Board {
    pub fn serialize(&self) -> String {
        "".into()
    }
}

fn all_the_cards() -> Vec<Card> {
    let mut cards = Vec::with_capacity(1 + (9 + 4) * 3);

    cards.push(Card::Flower(FlowerCard));

    for &suit in &[Suit::Black, Suit::Green, Suit::Red] {
        for i in 0..9 {
            cards.push(Card::Value(ValueCard(suit, i + 1)));
        }
        cards.push(Card::Dragon(suit));
        cards.push(Card::Dragon(suit));
        cards.push(Card::Dragon(suit));
        cards.push(Card::Dragon(suit));
    }

    cards
}

fn new_game() -> Board {
    let mut board = Board::default();

    let mut cards = all_the_cards();
    let mut rng = rand::thread_rng();
    rng.shuffle(&mut cards);

    let mut pile = 0;
    for card in cards {
        board.piles[pile].push(card);
        pile += 1;
        pile %= board.piles.len();
    }

    board
}

#[cfg(test)]
mod test {
    #[test]
    fn there_are_so_many_cards() {
        assert_eq!(super::all_the_cards().len(), 40);
    }

    #[test]
    fn new_games_are_fun() {
        let board = super::new_game();
        println!("{}", board.serialize());
        assert_eq!(board.flower, None);
    }
}
