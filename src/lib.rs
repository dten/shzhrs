#![allow(unused)]
#![allow(dead_code)]

extern crate rand;
use rand::Rng;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum Suit {
    Black,
    Green,
    Red,
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum Value {
    Dragon,
    Number(u8),
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
struct ValueCard(Suit, Value);

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
struct FlowerCard;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum Card {
    Value(ValueCard),
    Flower(FlowerCard),
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum Spare {
    Empty,
    Card(ValueCard),
    DragonStack(Suit),
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum Place {
    Empty,
    Card(ValueCard),
    DragonStack(Suit),
}

#[derive(Debug, Eq, PartialEq)]
struct Board {
    spares: [Spare; 3],
    flower: Option<FlowerCard>,
    places: [Place; 3],
    piles: [Vec<Card>; 8],
}

fn all_the_cards() -> Vec<Card> {
    let mut cards = Vec::with_capacity(1 + (9 + 4) * 3);

    cards.push(Card::Flower(FlowerCard));

    for &suit in &[Suit::Black, Suit::Green, Suit::Red] {
        for i in 0..9 {
            cards.push(Card::Value(ValueCard(suit, Value::Number(i + 1))));
        }
        cards.push(Card::Value(ValueCard(suit, Value::Dragon)));
        cards.push(Card::Value(ValueCard(suit, Value::Dragon)));
        cards.push(Card::Value(ValueCard(suit, Value::Dragon)));
        cards.push(Card::Value(ValueCard(suit, Value::Dragon)));
    }

    cards
}

fn new_game() -> Board {
    let mut board = Board {
        spares: [Spare::Empty; 3],
        flower: None,
        places: [Place::Empty; 3],
        piles: [
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
        ],
    };

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
        println!("{:#?}", board);
        assert_eq!(board.flower, None);
    }
}
