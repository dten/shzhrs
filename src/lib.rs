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
        fn encode_suit(s: &Suit) -> &'static str {
            match *s {
                Suit::Black => "b",
                Suit::Green => "g",
                Suit::Red => "r",
            }
        }

        fn encode_dragon_stack(suit: &Suit) -> String {
            format!("{s}D{s}D{s}D{s}D", s = encode_suit(suit))
        }

        fn encode_flower(flower: &FlowerCard) -> String {
            "ff".to_string()
        }

        fn encode_valuecard(vc: &ValueCard) -> String {
            let &ValueCard(suit, value) = vc;
            format!("{}{}", encode_suit(&suit), value)
        }

        fn encode_card(card: &Card) -> String {
            match *card {
                Card::Flower(ref f) => encode_flower(f),
                Card::Dragon(ref suit) => format!("{}D", encode_suit(suit)),
                Card::Value(ref v) => encode_valuecard(v),
            }
        }

        fn encode_spare(spare: &Spare) -> String {
            match *spare {
                Spare::Empty => String::new(),
                Spare::Card(ref c) => encode_card(c),
                Spare::DragonStack(ref suit) => encode_dragon_stack(suit),
            }
        }

        fn encode_place(place: &Place) -> String {
            match *place {
                Place::Empty => String::new(),
                Place::Card(ref vc) => encode_valuecard(vc),
            }
        }

        fn encode_pile(pile: &SmallVec<[Card; 13]>) -> String {
            pile.iter()
                .map(encode_card)
                .fold(String::new(), |a, c| a + &c)
        }

        let spares = self.spares.iter().map(encode_spare);
        let flower = [&self.flower];
        let flower = flower
            .iter()
            .map(|f| f.as_ref().map(encode_flower).unwrap_or_else(String::new));
        let places = self.places.iter().map(encode_place);
        let piles = self.piles.iter().map(encode_pile);

        spares
            .chain(flower)
            .chain(places)
            .chain(piles)
            .collect::<Vec<_>>()
            .join(";")
    }

    pub fn decode(s: &str) -> Result<Board, &'static str> {
        let split = s.split(';').collect::<Vec<_>>();

        if split.len() != 15 {
            Err("split.length() != 15")?
        }

        fn decode_card(s: &str) -> Result<Card, &'static str> {
            if s.len() != 2 {
                Err("bad length of card")?;
            }
            if s == "ff" {
                return Ok(Card::Flower(FlowerCard));
            }

            let suit = match s.as_bytes()[0] {
                b'b' => Suit::Black,
                b'g' => Suit::Green,
                b'r' => Suit::Red,
                b => Err("bad suit")?,
            };

            Ok(match s.as_bytes()[1] {
                v @ b'1'...b'9' => Card::Value(ValueCard(suit, v - b'0')),
                b'D' => Card::Dragon(suit),
                _ => Err("bad value")?,
            })
        }

        fn decode_spare(s: &str) -> Result<Spare, &'static str> {
            Ok(match s {
                "" => Spare::Empty,
                "bDbDbDbD" => Spare::DragonStack(Suit::Black),
                "gDgDgDgD" => Spare::DragonStack(Suit::Green),
                "rDrDrDrD" => Spare::DragonStack(Suit::Red),
                _ => Spare::Card(decode_card(s)?),
            })
        }

        fn decode_flower(s: &str) -> Result<Option<FlowerCard>, &'static str> {
            Ok(match s {
                "" => None,
                "ff" => Some(FlowerCard),
                _ => Err("that isn't a flower")?,
            })
        }

        fn decode_place(s: &str) -> Result<Place, &'static str> {
            Ok(match s {
                "" => Place::Empty,
                _ => match decode_card(s)? {
                    Card::Dragon(_) => Err("Dragons can't placed")?,
                    Card::Flower(_) => Err("Flowers can't placed")?,
                    Card::Value(v) => Place::Card(v),
                },
            })
        }

        fn decode_pile(s: &str) -> Result<SmallVec<[Card; 13]>, &'static str> {
            if s.len() % 2 != 0 {
                Err("Piles must be even length")?
            }

            let mut pile = SmallVec::new();

            for i in 0..s.len() / 2 {
                let idx = i * 2;
                let st = &s[idx..idx + 2];
                pile.push(decode_card(st)?)
            }

            Ok(pile)
        }

        Ok(Board {
            spares: [
                decode_spare(split[0])?,
                decode_spare(split[1])?,
                decode_spare(split[2])?,
            ],
            flower: decode_flower(split[3])?,
            places: [
                decode_place(split[4])?,
                decode_place(split[5])?,
                decode_place(split[6])?,
            ],
            piles: [
                decode_pile(split[7])?,
                decode_pile(split[8])?,
                decode_pile(split[9])?,
                decode_pile(split[10])?,
                decode_pile(split[11])?,
                decode_pile(split[12])?,
                decode_pile(split[13])?,
                decode_pile(split[14])?,
            ],
        })
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
    use super::*;

    #[test]
    fn there_are_so_many_cards() {
        assert_eq!(super::all_the_cards().len(), 40);
    }

    #[test]
    fn new_games_are_fun() {
        let board = new_game();
        let encoded = board.encode();
        std::thread::spawn(|| {});
        let decoded = Board::decode(&encoded);
        assert_eq!(&decoded.unwrap(), &board);
        assert_eq!(board.flower, None);
    }

    #[test]
    fn empty_board() {
        let b = ";;;;;;;;;;;;;;";
        let board = Board::decode(b).unwrap();
        assert_eq!(b, board.encode());
    }

    #[test]
    fn board_of_interest_0() {
        let b = "gDgDgDgD;;;ff;;;;r6b5;r4g9bDr7;bDg5rDb6;b3g2r9b8;r3rDg8g6bD;rDg1b1r8b7;g7g4b4bDg3b2;b9r2r5r1rD";
        let board = Board::decode(b).unwrap();
        assert_eq!(b, board.encode());
    }
}
