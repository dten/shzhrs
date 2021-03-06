#![allow(unused)]
#![allow(dead_code)]

extern crate pathfinding;
extern crate rand;
extern crate smallvec;

use rand::Rng;
use smallvec::SmallVec;
use pathfinding::astar;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
pub enum Suit {
    Black,
    Green,
    Red,
}

impl Suit {
    pub fn to_usize(&self) -> usize {
        match *self {
            Suit::Black => 0,
            Suit::Green => 1,
            Suit::Red => 2,
        }
    }
    pub fn from_usize(i: usize) -> Suit {
        match i {
            0 => Suit::Black,
            1 => Suit::Green,
            2 => Suit::Red,
            _ => panic!("oi what you playin at"),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct ValueCard(Suit, u8);

trait GoesOn {
    fn goes_on(&self, b: &Self) -> bool;
}

impl GoesOn for ValueCard {
    fn goes_on(&self, b: &ValueCard) -> bool {
        use Card::*;
        match (self, b) {
            (&ValueCard(suit_a, v_a), &ValueCard(suit_b, v_b)) => {
                suit_a != suit_b && v_a + 1 == v_b
            }
            _ => false,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct FlowerCard;

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub enum Card {
    Value(ValueCard),
    Dragon(Suit),
    Flower(FlowerCard),
}

impl Card {
    pub fn suit(&self) -> Option<&Suit> {
        use Card::*;
        match *self {
            Value(ValueCard(ref suit, _)) | Dragon(ref suit) => Some(suit),
            _ => None,
        }
    }
}

impl GoesOn for Card {
    fn goes_on(&self, b: &Self) -> bool {
        use Card::*;
        match (self, b) {
            (&Value(ref vc_a), &Value(ref vc_b)) => vc_a.goes_on(vc_b),
            _ => false,
        }
    }
}

use std::fmt;
impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Card::Flower(..) => write!(f, "Flower"),
            _ => {
                match self.suit() {
                    Some(&Suit::Black) => write!(f, "Black ")?,
                    Some(&Suit::Green) => write!(f, "Green ")?,
                    Some(&Suit::Red) => write!(f, "Red ")?,
                    _ => {}
                }
                match *self {
                    Card::Dragon(..) => write!(f, "Dragon"),
                    Card::Value(ValueCard(ref _s, ref v)) => write!(f, "{}", v),
                    _ => unreachable!(),
                }
            }
        }
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub enum Spare {
    Empty,
    Card(Card),
    DragonStack(Suit),
}

impl Spare {
    pub fn is_empty(&self) -> bool {
        match *self {
            Spare::Empty => true,
            _ => false,
        }
    }

    pub fn dragonstack_suit(&self) -> Option<&Suit> {
        match *self {
            Spare::DragonStack(ref suit) => Some(suit),
            _ => None,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub enum Place {
    Empty,
    Card(ValueCard),
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Move {
    Place(ValueCard),
    PileToSpare(Card, usize),
    SpareToPile(Card, usize),
    PileToPile(Card, usize, usize),
    DragonStack(Suit),
    Flower(),
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct Board {
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

    pub fn neighbours(&self) -> Vec<(Move, Board)> {
        let mut n: Vec<(Move, Board)> = vec![];

        // move flower to flowerspot
        if self.flower == None {
            // Look for flower on top of piles
            for (i, pile) in self.piles.iter().enumerate() {
                match pile.last() {
                    Some(&Card::Flower(..)) => {
                        let mut new_board = self.clone();
                        new_board.piles[i].pop();
                        new_board.flower = Some(FlowerCard);
                        n.push((Move::Flower(), new_board));
                        return n; // Flower is the only choice
                    }
                    _ => {}
                }
            }
        }

        let all_placed = self.places
            .iter()
            .map(|p| match *p {
                Place::Empty => 0,
                Place::Card(ValueCard(_, v)) => v,
            })
            .min()
            .unwrap_or(0);

        // cards to top right
        for (j, place) in self.places.iter().enumerate() {
            // Each place has a card of interest, and might be forced up
            let (target, force) = match *place {
                Place::Empty => (ValueCard(Suit::from_usize(j), 1), true),
                Place::Card(ValueCard(suit, i)) => (ValueCard(suit, i + 1), i <= all_placed + 2),
            };
            // Maybe what we want is in piles
            for (i, pile) in self.piles.iter().enumerate() {
                match pile.last() {
                    Some(&Card::Value(ref v)) if v == &target => {
                        let mut new_board = self.clone();
                        new_board.piles[i].pop();
                        new_board.places[j] = Place::Card(target.clone());
                        if force {
                            return vec![(Move::Place(target), new_board)];
                        } else {
                            n.push((Move::Place(target.clone()), new_board));
                        }
                    }
                    _ => {}
                }
            }
            // Maybe what we want is in spares
            for (i, spare) in self.spares.iter().enumerate() {
                match *spare {
                    Spare::Card(Card::Value(ref v)) if v == &target => {
                        let mut new_board = self.clone();
                        new_board.spares[i] = Spare::Empty;
                        new_board.places[j] = Place::Card(target.clone());
                        if force {
                            return vec![(Move::Place(target), new_board)];
                        } else {
                            n.push((Move::Place(target.clone()), new_board));
                        }
                    }
                    _ => {}
                }
            }
        }

        // stack dragons
        for i in 0..3 {
            let suit_of_desire = Suit::from_usize(i);
            if self.spares
                .iter()
                .filter_map(Spare::dragonstack_suit)
                .any(|suit| suit == &suit_of_desire)
            {
                // already stacked
                continue;
            }

            let card_of_desire = Card::Dragon(suit_of_desire);
            let space_to_stack_to = match self.spares
                .iter()
                .enumerate()
                .find(|&(_, spare)| {
                    spare.is_empty() || *spare == Spare::Card(card_of_desire.clone())
                })
                .map(|(s, spare)| s)
            {
                Some(s) => s,
                None => continue, // there's nowhere to stack this dragon
            };

            let mut visible = 0;
            visible += self.spares
                .iter()
                .filter(|spare| **spare == Spare::Card(Card::Dragon(suit_of_desire)))
                .count();
            visible += self.piles
                .iter()
                .filter_map(|p| p.last())
                .filter(|card| **card == Card::Dragon(suit_of_desire))
                .count();
            if visible != 4 {
                // sad times, dragons not on top
                continue;
            }

            // Wooh stack em up
            let mut new_board = self.clone();
            // Purge spares
            for s in 0..new_board.spares.len() {
                if new_board.spares[s] == Spare::Card(card_of_desire.clone()) {
                    new_board.spares[s] = Spare::Empty;
                }
            }
            // Purge piles
            for p in 0..new_board.piles.len() {
                if new_board.piles[p].last() == Some(&card_of_desire.clone()) {
                    new_board.piles[p].pop();
                }
            }
            // Stack stack stack
            new_board.spares[space_to_stack_to] = Spare::DragonStack(suit_of_desire);
            n.push((Move::DragonStack(suit_of_desire), new_board));
        }

        // move from spares to piles
        for (j, spare) in self.spares.iter().enumerate() {
            if let Spare::Card(ref card) = *spare {
                match *card {
                    Card::Flower(..) => continue,
                    Card::Dragon(..) => {
                        for (i, pile) in self.piles.iter().enumerate() {
                            if pile.is_empty() {
                                // Empty stack for the dragon king
                                let mut new_board = self.clone();
                                new_board.spares[j] = Spare::Empty;
                                new_board.piles[i].push(card.clone());
                                n.push((Move::SpareToPile(card.clone(), i), new_board));
                                break; // Only care about first empty
                            }
                        }
                    }
                    Card::Value(ValueCard(s_suit, s_value)) => {
                        let mut moved_to_empty = false;
                        for (i, pile) in self.piles.iter().enumerate() {
                            if pile.is_empty() {
                                if moved_to_empty {
                                    continue;
                                }
                                // Empty piles love cards
                                let mut new_board = self.clone();
                                new_board.spares[j] = Spare::Empty;
                                new_board.piles[i].push(card.clone());
                                n.push((Move::SpareToPile(card.clone(), i), new_board));
                                moved_to_empty = true;
                            } else if let Some(&Card::Value(ValueCard(p_suit, p_value))) =
                                pile.last()
                            {
                                // Can place on value cards of different suit + 1
                                if s_suit != p_suit && s_value + 1 == p_value {
                                    let mut new_board = self.clone();
                                    new_board.spares[j] = Spare::Empty;
                                    new_board.piles[i].push(card.clone());
                                    n.push((Move::SpareToPile(card.clone(), i), new_board));
                                }
                            }
                        }
                    }
                }
            }
        }

        // move from pile to another pile
        for (i, a_pile) in self.piles.iter().enumerate() {
            for (p, card) in a_pile.iter().enumerate().rev() {
                let mut moved_to_empty = p == 0; // don't move base card to empty pile
                for (j, b_pile) in self.piles.iter().enumerate() {
                    if i == j {
                        continue;
                    }
                    if b_pile.is_empty() {
                        if moved_to_empty {
                            continue;
                        }
                        let mut new_board = self.clone();
                        let from_pile = &self.piles[i];
                        new_board.piles[j].extend(from_pile[p..].iter().cloned());
                        new_board.piles[i].truncate(p);
                        n.push((Move::PileToPile(card.clone(), i, j), new_board));
                        moved_to_empty = true;
                    } else if card.goes_on(b_pile.last().unwrap()) {
                        let mut new_board = self.clone();
                        let from_pile = &self.piles[i];
                        new_board.piles[j].extend(from_pile[p..].iter().cloned());
                        new_board.piles[i].truncate(p);
                        n.push((Move::PileToPile(card.clone(), i, j), new_board));
                    }
                }
                // If the stack is no longer valid give up on this pile
                if p > 0 && !card.goes_on(&a_pile[p - 1]) {
                    break;
                }
            }
        }

        // move to spare from piles
        for (i, pile) in self.piles.iter().enumerate() {
            for (j, spare) in self.spares.iter().enumerate() {
                if *spare == Spare::Empty {
                    match pile.last() {
                        Some(card) => {
                            let mut new_board = self.clone();
                            let moved = new_board.piles[i].pop().unwrap();
                            new_board.spares[j] = Spare::Card(moved.clone());
                            n.push((Move::PileToSpare(moved, i), new_board));
                        }
                        _ => {}
                    }
                    break; // Only care about first empty spare
                }
            }
        }

        n
    }

    pub fn is_a_goodn(&self) -> bool {
        if self.flower.is_none() {
            return false;
        }
        if self.piles.iter().any(|s| !s.is_empty()) {
            return false;
        }

        let ds_0 = self.spares[0].dragonstack_suit();
        let ds_1 = self.spares[1].dragonstack_suit();
        let ds_2 = self.spares[2].dragonstack_suit();
        if ds_0.is_none() || ds_1.is_none() || ds_2.is_none() {
            return false;
        }
        if ds_0 == ds_1 || ds_0 == ds_2 || ds_1 == ds_2 {
            return false;
        }

        if self.places[0] != Place::Card(ValueCard(Suit::Black, 9)) {
            return false;
        }
        if self.places[1] != Place::Card(ValueCard(Suit::Green, 9)) {
            return false;
        }
        if self.places[2] != Place::Card(ValueCard(Suit::Red, 9)) {
            return false;
        }

        true
    }

    pub fn work_to_do(&self) -> i64 {
        let misordered_piles: i64 = self.piles
            .iter()
            .map(|p| {
                let mut n = 0;
                for i in 1..p.len() {
                    if !p[i].goes_on(&p[i - 1]) {
                        n += 1
                    }
                }
                n
            })
            .sum();

        misordered_piles
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

pub fn new_game() -> Board {
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

pub fn solve(board: &Board) -> Option<(Vec<Board>, i64)> {
    use std::rc::Rc;
    let mut neighbours = |b: &Rc<Board>| {
        //println!("neighbours of {}", b.encode());
        let n = Board::neighbours(b);
        let count = n.len();
        n.into_iter()
            .map(move |(_m, b)| (Rc::new(b), if count == 1 { 0 } else { 1 }))
    };
    let mut heuristic = |b: &Rc<Board>| b.work_to_do();
    let mut success = |b: &Rc<Board>| b.is_a_goodn();
    astar(&Rc::new(board.clone()), neighbours, heuristic, success)
        .map(|(b, c)| (b.into_iter().map(|b| (*b).clone()).collect(), c))
}

pub fn solution_to_moves(boards: &Vec<Board>) -> Vec<Move> {
    let mut moves = vec![];
    for i in 0..boards.len() - 1 {
        let m = boards[i]
            .neighbours()
            .into_iter()
            .find(|&(_, ref b)| b == &boards[i + 1])
            .map(|p| p.0);
        match m {
            Some(m) => moves.push(m),
            None => panic!("you lied this isn't a solution!"),
        }
    }
    moves
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

    #[test]
    fn suitability() {
        let a = ValueCard(Suit::Black, 1);
        let b = ValueCard(Suit::Red, 2);
        assert!(a.goes_on(&b));
        assert!(!b.goes_on(&a));
        let a = Card::Value(a);
        let b = Card::Value(b);
        assert!(a.goes_on(&b));
        assert!(!b.goes_on(&a));
    }

    fn assert_neighours(board: &str, expected: Vec<&str>) {
        let neighbours = Board::decode(board)
            .unwrap()
            .neighbours()
            .into_iter()
            .map(|(_m, b)| b.encode())
            .collect::<Vec<String>>();
        assert_eq!(
            neighbours, expected,
            "actual != expected for neighbours of {}",
            board
        );
    }

    fn assert_moves(board: &str, expected: Vec<(&str, &str)>) {
        let neighbours = Board::decode(board)
            .unwrap()
            .neighbours()
            .into_iter()
            .map(|(m, b)| (format!("{:?}", m), b.encode()))
            .collect::<Vec<(String, String)>>();
        let neighbours = neighbours
            .iter()
            .map(|&(ref a, ref b)| (b.as_str(), a.as_str()))
            .collect::<Vec<_>>();
        assert_eq!(
            neighbours, expected,
            "actual != expected for neighbours of {}",
            board
        );
    }

    #[test]
    fn whats_next() {
        assert_moves(
            ";;;;;;;;;;;;b4b3;;",
            vec![
                (
                    ";;;;;;;b3;;;;;b4;;",
                    "PileToPile(Value(ValueCard(Black, 3)), 5, 0)",
                ),
                (
                    "b3;;;;;;;;;;;;b4;;",
                    "PileToSpare(Value(ValueCard(Black, 3)), 5)",
                ),
            ],
        );
        assert_moves(
            ";;;;;;;;;;;;b2b1;;",
            vec![(";;;;b1;;;;;;;;b2;;", "Place(ValueCard(Black, 1))")],
        );
        assert_neighours("b1;;;;;;;;;;;;;;", vec![";;;;b1;;;;;;;;;;"]);
        assert_neighours(";;;ff;;;;;;;;;;;", vec![]);
        // Must move flower, so can't be in spare
        assert_neighours(";;;;;;;ff;;;;;;;", vec![";;;ff;;;;;;;;;;;"]);
        // Moving from spare onto a thing or blank
        assert_neighours(
            "b5;g5;;;;;;g6;;;;;;;",
            vec![
                ";g5;;;;;;g6b5;;;;;;;", // stack b5 on g6
                ";g5;;;;;;g6;b5;;;;;;", // b5 to empty pile
                "b5;;;;;;;g6;g5;;;;;;", // g5 to empty pile
                "b5;g5;g6;;;;;;;;;;;;", // g6 from pile to remaining spare
            ],
        );
    }

    #[test]
    fn stacking() {
        assert_neighours(
            ";;;;;;;;;;;;r5b4;g6;",
            vec![
                ";;;;;;;b4;;;;;r5;g6;",
                ";;;;;;;;;;;;;g6r5b4;", // create stack of 3 cards
                "b4;;;;;;;;;;;;r5;g6;",
                "g6;;;;;;;;;;;;r5b4;;",
            ],
        );
    }

    #[test]
    fn dragonstack() {
        assert_moves(
            ";;rD;;;;;;;rD;rD;rD;r5b4;g6;",
            vec![
                ("rDrDrDrD;;;;;;;;;;;;r5b4;g6;", "DragonStack(Red)"),
                (
                    ";;;;;;;rD;;rD;rD;rD;r5b4;g6;",
                    "SpareToPile(Dragon(Red), 0)",
                ),
                (
                    ";;rD;;;;;b4;;rD;rD;rD;r5;g6;",
                    "PileToPile(Value(ValueCard(Black, 4)), 5, 0)",
                ),
                (
                    ";;rD;;;;;;;rD;rD;rD;;g6r5b4;",
                    "PileToPile(Value(ValueCard(Red, 5)), 5, 6)",
                ),
                (
                    "rD;;rD;;;;;;;;rD;rD;r5b4;g6;",
                    "PileToSpare(Dragon(Red), 2)",
                ),
                (
                    "rD;;rD;;;;;;;rD;;rD;r5b4;g6;",
                    "PileToSpare(Dragon(Red), 3)",
                ),
                (
                    "rD;;rD;;;;;;;rD;rD;;r5b4;g6;",
                    "PileToSpare(Dragon(Red), 4)",
                ),
                (
                    "b4;;rD;;;;;;;rD;rD;rD;r5;g6;",
                    "PileToSpare(Value(ValueCard(Black, 4)), 5)",
                ),
                (
                    "g6;;rD;;;;;;;rD;rD;rD;r5b4;;",
                    "PileToSpare(Value(ValueCard(Green, 6)), 6)",
                ),
            ],
        );
    }

    #[test]
    fn dragonstack_full_spares() {
        assert_neighours(
            "bD;bD;bD;;;;;;;;;r2bD;;;",
            vec![
                "bDbDbDbD;;;;;;;;;;;r2;;;", // stack em up
                ";bD;bD;;;;;bD;;;;r2bD;;;", // from spare 0
                "bD;;bD;;;;;bD;;;;r2bD;;;", // from spare 1
                "bD;bD;;;;;;bD;;;;r2bD;;;", // from spare 2
                "bD;bD;bD;;;;;bD;;;;r2;;;", // from pile 4
            ],
        );
    }

    #[test]
    fn winner_winner() {
        assert!(
            Board::decode("rDrDrDrD;bDbDbDbD;gDgDgDgD;ff;b9;g9;r9;;;;;;;;")
                .unwrap()
                .is_a_goodn()
        );
        assert!(
            !Board::decode("rDrDrDrD;bDbDbDbD;gDgDgDgD;ff;b9;g9;r8;;;;;;;;r9")
                .unwrap()
                .is_a_goodn()
        );
    }

    #[test]
    fn solve_simple() {
        let b = "rDrDrDrD;bDbDbDbD;gDgDgDgD;ff;b9;g9;r8;;;;;;;;r9";
        let board = Board::decode(b).unwrap();
        match solve(&board) {
            None => panic!("couldn't solve {}", b),
            Some((path, cost)) => {
                assert_eq!(cost, 0);
                assert_eq!(
                    path.iter().map(Board::encode).collect::<Vec<_>>(),
                    vec![
                        "rDrDrDrD;bDbDbDbD;gDgDgDgD;ff;b9;g9;r8;;;;;;;;r9",
                        "rDrDrDrD;bDbDbDbD;gDgDgDgD;ff;b9;g9;r9;;;;;;;;",
                    ]
                );
            }
        }
    }

    #[test]
    fn solve_pascals_game() {
        let b = "gDgDgDgD;;;ff;;;;r6b5;r4g9bDr7;bDg5rDb6;b3g2r9b8;r3rDg8g6bD;rDg1b1r8b7;g7g4b4bDg3b2;b9r2r5r1rD";
        let board = Board::decode(b).unwrap();
        match solve(&board) {
            None => panic!("couldn't solve {}", b),
            Some((path, cost)) => {
                assert_eq!(cost, 18);
                assert_eq!(
                    path.iter().map(Board::encode).collect::<Vec<_>>(),
                    vec![
                        "gDgDgDgD;;;ff;;;;r6b5;r4g9bDr7;bDg5rDb6;b3g2r9b8;r3rDg8g6bD;rDg1b1r8b7;g7g4b4bDg3b2;b9r2r5r1rD",
                        "gDgDgDgD;;;ff;;;;r6b5;r4g9bD;bDg5rDb6;b3g2r9b8r7;r3rDg8g6bD;rDg1b1r8b7;g7g4b4bDg3b2;b9r2r5r1rD",
                        "gDgDgDgD;;;ff;;;;r6b5;r4g9bD;bDg5rD;b3g2r9b8r7b6;r3rDg8g6bD;rDg1b1r8b7;g7g4b4bDg3b2;b9r2r5r1rD",
                        "gDgDgDgD;bD;;ff;;;;r6b5;r4g9;bDg5rD;b3g2r9b8r7b6;r3rDg8g6bD;rDg1b1r8b7;g7g4b4bDg3b2;b9r2r5r1rD",
                        "gDgDgDgD;bD;;ff;;;;r6b5;r4g9r8b7;bDg5rD;b3g2r9b8r7b6;r3rDg8g6bD;rDg1b1;g7g4b4bDg3b2;b9r2r5r1rD",
                        "gDgDgDgD;bD;;ff;b1;;;r6b5;r4g9r8b7;bDg5rD;b3g2r9b8r7b6;r3rDg8g6bD;rDg1;g7g4b4bDg3b2;b9r2r5r1rD",
                        "gDgDgDgD;bD;;ff;b2;;;r6b5;r4g9r8b7;bDg5rD;b3g2r9b8r7b6;r3rDg8g6bD;rDg1;g7g4b4bDg3;b9r2r5r1rD",
                        "gDgDgDgD;bD;;ff;b2;g1;;r6b5;r4g9r8b7;bDg5rD;b3g2r9b8r7b6;r3rDg8g6bD;rD;g7g4b4bDg3;b9r2r5r1rD",
                        "gDgDgDgD;bD;rD;ff;b2;g1;;r6b5;r4g9r8b7;bDg5;b3g2r9b8r7b6;r3rDg8g6bD;rD;g7g4b4bDg3;b9r2r5r1rD",
                        "gDgDgDgD;bD;rD;ff;b2;g1;;r6b5;r4g9r8b7;bD;b3g2r9b8r7b6g5;r3rDg8g6bD;rD;g7g4b4bDg3;b9r2r5r1rD",
                        "gDgDgDgD;bD;rD;ff;b2;g1;;;r4g9r8b7r6b5;bD;b3g2r9b8r7b6g5;r3rDg8g6bD;rD;g7g4b4bDg3;b9r2r5r1rD",
                        "gDgDgDgD;bD;rD;ff;b2;g1;;g9r8b7r6b5;r4;bD;b3g2r9b8r7b6g5;r3rDg8g6bD;rD;g7g4b4bDg3;b9r2r5r1rD",
                        "gDgDgDgD;bD;rD;ff;b2;g1;;g9r8b7r6b5;r4g3;bD;b3g2r9b8r7b6g5;r3rDg8g6bD;rD;g7g4b4bD;b9r2r5r1rD",
                        "gDgDgDgD;bDbDbDbD;rD;ff;b2;g1;;g9r8b7r6b5;r4g3;;b3g2r9b8r7b6g5;r3rDg8g6;rD;g7g4b4;b9r2r5r1rD",
                        "gDgDgDgD;bDbDbDbD;rD;ff;b2;g1;;g9r8b7r6b5;r4g3;r9b8r7b6g5;b3g2;r3rDg8g6;rD;g7g4b4;b9r2r5r1rD",
                        "gDgDgDgD;bDbDbDbD;rD;ff;b2;g2;;g9r8b7r6b5;r4g3;r9b8r7b6g5;b3;r3rDg8g6;rD;g7g4b4;b9r2r5r1rD",
                        "gDgDgDgD;bDbDbDbD;rD;ff;b3;g2;;g9r8b7r6b5;r4g3;r9b8r7b6g5;;r3rDg8g6;rD;g7g4b4;b9r2r5r1rD",
                        "gDgDgDgD;bDbDbDbD;rD;ff;b3;g3;;g9r8b7r6b5;r4;r9b8r7b6g5;;r3rDg8g6;rD;g7g4b4;b9r2r5r1rD",
                        "gDgDgDgD;bDbDbDbD;rD;ff;b4;g3;;g9r8b7r6b5;r4;r9b8r7b6g5;;r3rDg8g6;rD;g7g4;b9r2r5r1rD",
                        "gDgDgDgD;bDbDbDbD;rD;ff;b4;g4;;g9r8b7r6b5;r4;r9b8r7b6g5;;r3rDg8g6;rD;g7;b9r2r5r1rD",
                        "gDgDgDgD;bDbDbDbD;rD;ff;b4;g4;;g9r8b7r6b5;r4;r9b8r7b6g5;rD;r3rDg8g6;rD;g7;b9r2r5r1",
                        "gDgDgDgD;bDbDbDbD;rD;ff;b4;g4;r1;g9r8b7r6b5;r4;r9b8r7b6g5;rD;r3rDg8g6;rD;g7;b9r2r5",
                        "gDgDgDgD;bDbDbDbD;rD;ff;b4;g4;r1;g9r8b7r6b5;r4;r9b8r7b6g5;rD;r3rDg8g6r5;rD;g7;b9r2",
                        "gDgDgDgD;bDbDbDbD;rD;ff;b4;g4;r2;g9r8b7r6b5;r4;r9b8r7b6g5;rD;r3rDg8g6r5;rD;g7;b9",
                        "gDgDgDgD;bDbDbDbD;rD;ff;b5;g4;r2;g9r8b7r6;r4;r9b8r7b6g5;rD;r3rDg8g6r5;rD;g7;b9",
                        "gDgDgDgD;bDbDbDbD;rD;ff;b5;g5;r2;g9r8b7r6;r4;r9b8r7b6;rD;r3rDg8g6r5;rD;g7;b9",
                        "gDgDgDgD;bDbDbDbD;rD;ff;b6;g5;r2;g9r8b7r6;r4;r9b8r7;rD;r3rDg8g6r5;rD;g7;b9",
                        "gDgDgDgD;bDbDbDbD;rD;ff;b6;g5;r2;g9r8b7r6;r4;r9b8r7g6r5;rD;r3rDg8;rD;g7;b9",
                        "gDgDgDgD;bDbDbDbD;rD;ff;b6;g5;r2;g9r8b7r6;r4;r9b8r7g6r5;rD;r3rD;rD;g7;b9g8",
                        "gDgDgDgD;bDbDbDbD;rDrDrDrD;ff;b6;g5;r2;g9r8b7r6;r4;r9b8r7g6r5;;r3;;g7;b9g8",
                        "gDgDgDgD;bDbDbDbD;rDrDrDrD;ff;b6;g5;r3;g9r8b7r6;r4;r9b8r7g6r5;;;;g7;b9g8",
                        "gDgDgDgD;bDbDbDbD;rDrDrDrD;ff;b6;g5;r4;g9r8b7r6;;r9b8r7g6r5;;;;g7;b9g8",
                        "gDgDgDgD;bDbDbDbD;rDrDrDrD;ff;b6;g5;r5;g9r8b7r6;;r9b8r7g6;;;;g7;b9g8",
                        "gDgDgDgD;bDbDbDbD;rDrDrDrD;ff;b6;g6;r5;g9r8b7r6;;r9b8r7;;;;g7;b9g8",
                        "gDgDgDgD;bDbDbDbD;rDrDrDrD;ff;b6;g7;r5;g9r8b7r6;;r9b8r7;;;;;b9g8",
                        "gDgDgDgD;bDbDbDbD;rDrDrDrD;ff;b6;g8;r5;g9r8b7r6;;r9b8r7;;;;;b9",
                        "gDgDgDgD;bDbDbDbD;rDrDrDrD;ff;b6;g8;r6;g9r8b7;;r9b8r7;;;;;b9",
                        "gDgDgDgD;bDbDbDbD;rDrDrDrD;ff;b7;g8;r6;g9r8;;r9b8r7;;;;;b9",
                        "gDgDgDgD;bDbDbDbD;rDrDrDrD;ff;b7;g8;r7;g9r8;;r9b8;;;;;b9",
                        "gDgDgDgD;bDbDbDbD;rDrDrDrD;ff;b8;g8;r7;g9r8;;r9;;;;;b9",
                        "gDgDgDgD;bDbDbDbD;rDrDrDrD;ff;b9;g8;r7;g9r8;;r9;;;;;",
                        "gDgDgDgD;bDbDbDbD;rDrDrDrD;ff;b9;g8;r8;g9;;r9;;;;;",
                        "gDgDgDgD;bDbDbDbD;rDrDrDrD;ff;b9;g9;r8;;;r9;;;;;",
                        "gDgDgDgD;bDbDbDbD;rDrDrDrD;ff;b9;g9;r9;;;;;;;;"
                    ]
                );
                let moves = solution_to_moves(&path)
                    .into_iter()
                    .map(|m| format!("{:?}", m))
                    .collect::<Vec<_>>();
                assert_eq!(
                    moves,
                    vec![
                        "PileToPile(Value(ValueCard(Red, 7)), 1, 3)",
                        "PileToPile(Value(ValueCard(Black, 6)), 2, 3)",
                        "PileToSpare(Dragon(Black), 1)",
                        "PileToPile(Value(ValueCard(Red, 8)), 5, 1)",
                        "Place(ValueCard(Black, 1))",
                        "Place(ValueCard(Black, 2))",
                        "Place(ValueCard(Green, 1))",
                        "PileToSpare(Dragon(Red), 2)",
                        "PileToPile(Value(ValueCard(Green, 5)), 2, 3)",
                        "PileToPile(Value(ValueCard(Red, 6)), 0, 1)",
                        "PileToPile(Value(ValueCard(Green, 9)), 1, 0)",
                        "PileToPile(Value(ValueCard(Green, 3)), 6, 1)",
                        "DragonStack(Black)",
                        "PileToPile(Value(ValueCard(Red, 9)), 3, 2)",
                        "Place(ValueCard(Green, 2))",
                        "Place(ValueCard(Black, 3))",
                        "Place(ValueCard(Green, 3))",
                        "Place(ValueCard(Black, 4))",
                        "Place(ValueCard(Green, 4))",
                        "PileToPile(Dragon(Red), 7, 3)",
                        "Place(ValueCard(Red, 1))",
                        "PileToPile(Value(ValueCard(Red, 5)), 7, 4)",
                        "Place(ValueCard(Red, 2))",
                        "Place(ValueCard(Black, 5))",
                        "Place(ValueCard(Green, 5))",
                        "Place(ValueCard(Black, 6))",
                        "PileToPile(Value(ValueCard(Green, 6)), 4, 2)",
                        "PileToPile(Value(ValueCard(Green, 8)), 4, 7)",
                        "DragonStack(Red)",
                        "Place(ValueCard(Red, 3))",
                        "Place(ValueCard(Red, 4))",
                        "Place(ValueCard(Red, 5))",
                        "Place(ValueCard(Green, 6))",
                        "Place(ValueCard(Green, 7))",
                        "Place(ValueCard(Green, 8))",
                        "Place(ValueCard(Red, 6))",
                        "Place(ValueCard(Black, 7))",
                        "Place(ValueCard(Red, 7))",
                        "Place(ValueCard(Black, 8))",
                        "Place(ValueCard(Black, 9))",
                        "Place(ValueCard(Red, 8))",
                        "Place(ValueCard(Green, 9))",
                        "Place(ValueCard(Red, 9))",
                    ]
                );
            }
        }
    }

    #[test]
    fn solve_full_game() {
        let b = "gD;gD;gD;;;;;r6b5gDff;r4g9bDr7;bDg5rDb6;b3g2r9b8;r3rDg8g6bD;rDg1b1r8b7;g7g4b4bDg3b2;b9r2r5r1rD";

        assert_neighours(
            "gD;gD;gD;;;;;r6b5gDff;r4g9bDr7;bDg5rDb6;b3g2r9b8;r3rDg8g6bD;rDg1b1r8b7;g7g4b4bDg3b2;b9r2r5r1rD",
            vec![
                "gD;gD;gD;ff;;;;r6b5gD;r4g9bDr7;bDg5rDb6;b3g2r9b8;r3rDg8g6bD;rDg1b1r8b7;g7g4b4bDg3b2;b9r2r5r1rD", // must place flower
            ],
        );
        assert_neighours(
            "gD;gD;gD;ff;;;;r6b5gD;r4g9bDr7;bDg5rDb6;b3g2r9b8;r3rDg8g6bD;rDg1b1r8b7;g7g4b4bDg3b2;b9r2r5r1rD",
            vec![
                "gDgDgDgD;;;ff;;;;r6b5;r4g9bDr7;bDg5rDb6;b3g2r9b8;r3rDg8g6bD;rDg1b1r8b7;g7g4b4bDg3b2;b9r2r5r1rD", // goal is stack dragons
                "gD;gD;gD;ff;;;;r6b5gD;r4g9bD;bDg5rDb6;b3g2r9b8r7;r3rDg8g6bD;rDg1b1r8b7;g7g4b4bDg3b2;b9r2r5r1rD",
                "gD;gD;gD;ff;;;;r6b5gD;r4g9bDr7b6;bDg5rD;b3g2r9b8;r3rDg8g6bD;rDg1b1r8b7;g7g4b4bDg3b2;b9r2r5r1rD"
            ],
        );

        let board = Board::decode(b).unwrap();
        match solve(&board) {
            None => panic!("couldn't solve {}", board.encode()),
            Some((path, cost)) => {}
        }
    }

    #[test]
    fn wasteful_dragon_moving() {
        let b = ";;;;;;;b1b2r9g9g2;rDrDg7b8g3;r5g6ffb3rD;b9g5r2b7b4;r1g1r3gDrD;r8r7g4bDbD;b6gDgDbDb5;g8bDgDr6r4";
        let board = Board::decode(b).unwrap();
        match solve(&board) {
            None => panic!("couldn't solve {}", b),
            Some((path, cost)) => {
                let moves = solution_to_moves(&path)
                    .into_iter()
                    .map(|m| format!("{:?}", m))
                    .collect::<Vec<_>>();
                assert_eq!(
                    moves,
                    vec![
                        "PileToPile(Value(ValueCard(Green, 3)), 1, 3)",
                        "PileToPile(Value(ValueCard(Red, 4)), 7, 6)",
                        "PileToPile(Value(ValueCard(Black, 5)), 6, 7)",
                        "PileToSpare(Value(ValueCard(Green, 2)), 0)",
                        "PileToPile(Value(ValueCard(Black, 8)), 1, 0)",
                        "PileToPile(Value(ValueCard(Green, 7)), 1, 0)",
                        "PileToPile(Value(ValueCard(Red, 6)), 7, 0)",
                        "PileToSpare(Dragon(Red), 1)",
                        "DragonStack(Red)",
                        "PileToPile(Value(ValueCard(Green, 9)), 0, 1)",
                        "PileToPile(Value(ValueCard(Black, 3)), 2, 1)",
                        "Flower",
                        "PileToSpare(Value(ValueCard(Red, 9)), 0)",
                        "PileToPile(Value(ValueCard(Black, 2)), 0, 3)",
                        "Place(ValueCard(Black, 1))",
                        "Place(ValueCard(Black, 2))",
                        "Place(ValueCard(Black, 3))",
                        "PileToPile(Dragon(Green), 4, 0)",
                        "PileToPile(Value(ValueCard(Green, 3)), 3, 1)",
                        "PileToPile(Value(ValueCard(Red, 3)), 4, 3)",
                        "Place(ValueCard(Green, 1))",
                        "Place(ValueCard(Green, 2))",
                        "Place(ValueCard(Green, 3))",
                        "Place(ValueCard(Red, 1))",
                        "PileToPile(Value(ValueCard(Green, 6)), 2, 4)",
                        "PileToPile(Value(ValueCard(Black, 4)), 3, 2)",
                        "PileToSpare(Value(ValueCard(Black, 7)), 3)",
                        "Place(ValueCard(Red, 2))",
                        "Place(ValueCard(Red, 3))",
                        "Place(ValueCard(Black, 4))",
                        "Place(ValueCard(Red, 4))",
                        "Place(ValueCard(Black, 5))",
                        "Place(ValueCard(Red, 5))",
                        "Place(ValueCard(Red, 6))",
                        "SpareToPile(Value(ValueCard(Black, 7)), 2)",
                        "PileToSpare(Dragon(Black), 5)",
                        "PileToPile(Value(ValueCard(Green, 6)), 4, 2)",
                        "PileToPile(Dragon(Black), 5, 4)",
                        "Place(ValueCard(Green, 4))",
                        "Place(ValueCard(Green, 5))",
                        "Place(ValueCard(Green, 6))",
                        "Place(ValueCard(Green, 7))",
                        "Place(ValueCard(Red, 7))",
                        "Place(ValueCard(Red, 8))",
                        "PileToPile(Dragon(Black), 6, 5)",
                        "Place(ValueCard(Red, 9))",
                        "PileToSpare(Dragon(Green), 6)",
                        "DragonStack(Green)",
                        "Place(ValueCard(Black, 6))",
                        "Place(ValueCard(Black, 7))",
                        "Place(ValueCard(Black, 8))",
                        "Place(ValueCard(Black, 9))",
                        "DragonStack(Black)",
                        "Place(ValueCard(Green, 8))",
                        "Place(ValueCard(Green, 9))",
                    ]
                );
            }
        }
    }
}
