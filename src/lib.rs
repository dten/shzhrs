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
    pub fn encode(&self) -> String {
        fn encode_suit (s: &Suit) -> &'static str {
            match *s {
                Suit::Black => "b",
                Suit::Green => "g",
                Suit::Red => "r",
            }
        }

        fn encode_dragon_stack (suit: &Suit) -> String {
            format!("{s}D{s}D{s}D{s}D", s = encode_suit(suit))
        }

        fn encode_flower (flower: &FlowerCard) -> String {
            "ff".to_string()
        }

        fn encode_valuecard (vc: &ValueCard) -> String {
            let &ValueCard(suit, value) = vc;
            format!("{}{}", encode_suit(&suit), value)
        }

        fn encode_card (card: &Card) -> String {
            match *card {
                Card::Flower(ref f) => encode_flower(f),
                Card::Dragon(ref suit) => format!("{}D", encode_suit(suit)),
                Card::Value(ref v) => encode_valuecard(&v),
            }
        }

        fn encode_spare (spare: &Spare) -> String {
            match *spare {
                Spare::Empty => String::new(),
                Spare::Card(ref c) => encode_card(&c),
                Spare::DragonStack(ref suit) => encode_dragon_stack(&suit),
            }
        }

        fn encode_place (place: &Place) -> String {
            match *place {
                Place::Empty => String::new(),
                Place::Card(ref vc) => encode_valuecard(&vc),
            }            
        }

        fn encode_pile (pile: &SmallVec<[Card; 13]>) -> String {
            pile.iter().map(encode_card).fold(String::new(), |a, c| a + &c)
        }

        [
            encode_spare(&self.spares[0]),
            encode_spare(&self.spares[1]),
            encode_spare(&self.spares[2]),
            self.flower.as_ref().map(encode_flower).unwrap_or_else(String::new),
            encode_place(&self.places[0]),
            encode_place(&self.places[1]),
            encode_place(&self.places[2]),
            encode_pile(&self.piles[0]),
            encode_pile(&self.piles[1]),
            encode_pile(&self.piles[2]),
            encode_pile(&self.piles[3]),
            encode_pile(&self.piles[4]),
            encode_pile(&self.piles[5]),
            encode_pile(&self.piles[6]),
            encode_pile(&self.piles[7]),
        ].join(";")
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
        println!("{}", board.encode());
        assert_eq!(board.flower, None);
    }
}
