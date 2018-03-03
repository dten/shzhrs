#![allow(unused)]
#![allow(dead_code)]

extern crate pathfinding;
extern crate rand;
extern crate smallvec;

use rand::Rng;
use smallvec::SmallVec;
use pathfinding::astar;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
enum Suit {
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
struct ValueCard(Suit, u8);

trait GoesOn {
    fn goes_on(&self, b: &Self) -> bool;
}

impl GoesOn for ValueCard {
    fn goes_on(&self, b: &ValueCard) -> bool {
        use Card::*;
        let success = match (self, b) {
            (&ValueCard(suit_a, v_a), &ValueCard(suit_b, v_b)) => {
                suit_a != suit_b && v_a + 1 == v_b
            }
            _ => false,
        };
        success
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct FlowerCard;

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
enum Card {
    Value(ValueCard),
    Dragon(Suit),
    Flower(FlowerCard),
}

impl Card {
    pub fn suit(&self) -> Option<&Suit> {
        use Card::*;
        match *self {
            Value(ValueCard(ref suit, _)) => Some(suit),
            Dragon(ref suit) => Some(suit),
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

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
enum Spare {
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
enum Place {
    Empty,
    Card(ValueCard),
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
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

    pub fn neighbours(&self) -> Vec<Board> {
        let mut n: Vec<Board> = vec![];

        // move flower to flowerspot
        if self.flower == None {
            // Look for flower on top of piles
            for (i, pile) in self.piles.iter().enumerate() {
                match pile.last() {
                    Some(&Card::Flower(..)) => {
                        let mut new_board = self.clone();
                        new_board.piles[i].pop();
                        new_board.flower = Some(FlowerCard);
                        n.push(new_board);
                        return n; // Flower is the only choice
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
            n.push(new_board);
        }

        // cards to top right
        for (j, place) in self.places.iter().enumerate() {
            // Each place has a card of interest
            let target = match *place {
                Place::Empty => ValueCard(Suit::from_usize(j), 1),
                Place::Card(ValueCard(suit, i)) => ValueCard(suit, i + 1),
            };
            // Maybe what we want is in piles
            for (i, pile) in self.piles.iter().enumerate() {
                match pile.last() {
                    Some(&Card::Value(ref v)) if v == &target => {
                        let mut new_board = self.clone();
                        new_board.piles[i].pop();
                        new_board.places[j] = Place::Card(target.clone());
                        n.push(new_board)
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
                        n.push(new_board);
                        return n; // Flower is the only choice
                    }
                    _ => {}
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
                            new_board.spares[j] = Spare::Card(moved);
                            n.push(new_board)
                        }
                        _ => {}
                    }
                    break; // Only care about first empty spare
                }
            }
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
                                n.push(new_board);
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
                                n.push(new_board);
                                moved_to_empty = true;
                            } else if let Some(&Card::Value(ValueCard(p_suit, p_value))) =
                                pile.last()
                            {
                                // Can place on value cards of different suit + 1
                                if s_suit != p_suit && s_value + 1 == p_value {
                                    let mut new_board = self.clone();
                                    new_board.spares[j] = Spare::Empty;
                                    new_board.piles[i].push(card.clone());
                                    n.push(new_board)
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
                match *card {
                    Card::Value(..) => {
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
                                n.push(new_board);
                                moved_to_empty = true;
                            } else if card.goes_on(b_pile.last().unwrap()) {
                                let mut new_board = self.clone();
                                let from_pile = &self.piles[i];
                                new_board.piles[j].extend(from_pile[p..].iter().cloned());
                                new_board.piles[i].truncate(p);
                                n.push(new_board);
                            }
                        }
                        // If the stack is no longer valid give up on this pile
                        if p > 0 && !card.goes_on(&a_pile[p - 1]) {
                            break;
                        }
                    }
                    _ => break,
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
        let missing_value_cards = self.places
            .iter()
            .map(|p| match *p {
                Place::Empty => 9 as i64,
                Place::Card(ValueCard(_, v)) => (9 - v) as i64,
            })
            .sum::<i64>();
        let missing_dragon_stacks = self.spares.len() as i64
            - self.spares
                .iter()
                .filter_map(Spare::dragonstack_suit)
                .count() as i64;
        missing_value_cards + missing_dragon_stacks
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

fn solve(board: &Board) -> Option<(Vec<Board>, i64)> {
    let mut neighbours = |b: &Board| {
        println!("neighbours of {}", b.encode());
        Board::neighbours(b).into_iter().map(|b| (b, 1))
    };
    let mut heuristic = |b: &Board| b.work_to_do();
    let mut success = |b: &Board| b.is_a_goodn();
    astar(board, neighbours, heuristic, success)
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
            .iter()
            .map(Board::encode)
            .collect::<Vec<String>>();
        assert_eq!(
            neighbours, expected,
            "actual != expected for neighbours of {}",
            board
        );
    }

    #[test]
    fn whats_next() {
        assert_neighours(
            ";;;;;;;;;;;;b2b1;;",
            vec![
                ";;;;b1;;;;;;;;b2;;", // place b1
                "b1;;;;;;;;;;;;b2;;", //spare b1
                ";;;;;;;b1;;;;;b2;;", // unstack b1
            ],
        );
        assert_neighours("b1;;;;;;;;;;;;;;", vec![";;;;b1;;;;;;;;;;"]);
        assert_neighours(";;;ff;;;;;;;;;;;", vec![]);
        // Must move flower, so can't be in spare
        assert_neighours(";;;;;;;ff;;;;;;;", vec![";;;ff;;;;;;;;;;;"]);
        // Moving from spare onto a thing or blank
        assert_neighours(
            "b5;g5;;;;;;g6;;;;;;;",
            vec![
                "b5;g5;g6;;;;;;;;;;;;", // g6 from pile to remaining spare
                ";g5;;;;;;g6b5;;;;;;;", // stack b5 on g6
                ";g5;;;;;;g6;b5;;;;;;", // b5 to empty pile
                "b5;;;;;;;g6;g5;;;;;;", // g5 to empty pile
            ],
        );
    }

    #[test]
    fn stacking() {
        assert_neighours(
            ";;;;;;;;;;;;r2b1;g3;",
            vec![
                ";;;;b1;;;;;;;;r2;g3;",
                "b1;;;;;;;;;;;;r2;g3;",
                "g3;;;;;;;;;;;;r2b1;;",
                ";;;;;;;b1;;;;;r2;g3;",
                ";;;;;;;;;;;;;g3r2b1;", // stack r2b1 onto g3
            ],
        );
    }

    #[test]
    fn dragonstack() {
        assert_neighours(
            ";;rD;;;;;;;rD;rD;rD;r2b1;g3;",
            vec![
                "rDrDrDrD;;;;;;;;;;;;r2b1;g3;", // stack em up
                ";;rD;;b1;;;;;rD;rD;rD;r2;g3;",
                "rD;;rD;;;;;;;;rD;rD;r2b1;g3;",
                "rD;;rD;;;;;;;rD;;rD;r2b1;g3;",
                "rD;;rD;;;;;;;rD;rD;;r2b1;g3;",
                "b1;;rD;;;;;;;rD;rD;rD;r2;g3;",
                "g3;;rD;;;;;;;rD;rD;rD;r2b1;;",
                ";;;;;;;rD;;rD;rD;rD;r2b1;g3;",
                ";;rD;;;;;b1;;rD;rD;rD;r2;g3;",
                ";;rD;;;;;;;rD;rD;rD;;g3r2b1;",
            ],
        );
    }

    #[test]
    fn dragonstack_full_spares() {
        assert_neighours(
            "bD;bD;bD;;;;;;;;;r2bD;;;",
            vec![
                "bDbDbDbD;;;;;;;;;;;r2;;;", // stack em up
                ";bD;bD;;;;;bD;;;;r2bD;;;",
                "bD;;bD;;;;;bD;;;;r2bD;;;",
                "bD;bD;;;;;;bD;;;;r2bD;;;",
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
                assert_eq!(cost, 1);
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
                assert_eq!(cost, 38);
                assert_eq!(
                    path.iter().map(Board::encode).collect::<Vec<_>>(),
                    vec![
                        "gDgDgDgD;;;ff;;;;r6b5;r4g9bDr7;bDg5rDb6;b3g2r9b8;r3rDg8g6bD;rDg1b1r8b7;g7g4b4bDg3b2;b9r2r5r1rD", "gDgDgDgD;;;ff;;;;r6b5;r4g9bDr7b6;bDg5rD;b3g2r9b8;r3rDg8g6bD;rDg1b1r8b7;g7g4b4bDg3b2;b9r2r5r1rD", "gDgDgDgD;rD;;ff;;;;r6b5;r4g9bDr7b6;bDg5;b3g2r9b8;r3rDg8g6bD;rDg1b1r8b7;g7g4b4bDg3b2;b9r2r5r1rD", "gDgDgDgD;rD;;ff;;;;r6b5;r4g9bDr7b6g5;bD;b3g2r9b8;r3rDg8g6bD;rDg1b1r8b7;g7g4b4bDg3b2;b9r2r5r1rD", "gDgDgDgD;rD;bD;ff;;;;r6b5;r4g9bDr7b6g5;;b3g2r9b8;r3rDg8g6bD;rDg1b1r8b7;g7g4b4bDg3b2;b9r2r5r1rD", "gDgDgDgD;rD;bD;ff;;;;r6b5;r4g9bDr7b6g5;r8b7;b3g2r9b8;r3rDg8g6bD;rDg1b1;g7g4b4bDg3b2;b9r2r5r1rD", "gDgDgDgD;rD;bD;ff;b1;;;r6b5;r4g9bDr7b6g5;r8b7;b3g2r9b8;r3rDg8g6bD;rDg1;g7g4b4bDg3b2;b9r2r5r1rD", "gDgDgDgD;rD;bD;ff;b2;;;r6b5;r4g9bDr7b6g5;r8b7;b3g2r9b8;r3rDg8g6bD;rDg1;g7g4b4bDg3;b9r2r5r1rD", "gDgDgDgD;rD;bD;ff;b2;g1;;r6b5;r4g9bDr7b6g5;r8b7;b3g2r9b8;r3rDg8g6bD;rD;g7g4b4bDg3;b9r2r5r1rD", "gDgDgDgD;rD;bD;ff;b2;g1;;;r4g9bDr7b6g5;r8b7r6b5;b3g2r9b8;r3rDg8g6bD;rD;g7g4b4bDg3;b9r2r5r1rD", "gDgDgDgD;rD;bD;ff;b2;g1;;r9b8;r4g9bDr7b6g5;r8b7r6b5;b3g2;r3rDg8g6bD;rD;g7g4b4bDg3;b9r2r5r1rD", "gDgDgDgD;rD;bD;ff;b2;g2;;r9b8;r4g9bDr7b6g5;r8b7r6b5;b3;r3rDg8g6bD;rD;g7g4b4bDg3;b9r2r5r1rD", "gDgDgDgD;rD;bD;ff;b3;g2;;r9b8;r4g9bDr7b6g5;r8b7r6b5;;r3rDg8g6bD;rD;g7g4b4bDg3;b9r2r5r1rD", "gDgDgDgD;rD;bD;ff;b3;g3;;r9b8;r4g9bDr7b6g5;r8b7r6b5;;r3rDg8g6bD;rD;g7g4b4bD;b9r2r5r1rD", "gDgDgDgD;rD;bD;ff;b3;g3;;r9b8;r4g9bD;r8b7r6b5;r7b6g5;r3rDg8g6bD;rD;g7g4b4bD;b9r2r5r1rD", "gDgDgDgD;rD;bDbDbDbD;ff;b3;g3;;r9b8;r4g9;r8b7r6b5;r7b6g5;r3rDg8g6;rD;g7g4b4;b9r2r5r1rD", "gDgDgDgD;rD;bDbDbDbD;ff;b4;g3;;r9b8;r4g9;r8b7r6b5;r7b6g5;r3rDg8g6;rD;g7g4;b9r2r5r1rD", "gDgDgDgD;rD;bDbDbDbD;ff;b5;g3;;r9b8;r4g9;r8b7r6;r7b6g5;r3rDg8g6;rD;g7g4;b9r2r5r1rD", "gDgDgDgD;rD;bDbDbDbD;ff;b5;g4;;r9b8;r4g9;r8b7r6;r7b6g5;r3rDg8g6;rD;g7;b9r2r5r1rD", "gDgDgDgD;rD;bDbDbDbD;ff;b5;g5;;r9b8;r4g9;r8b7r6;r7b6;r3rDg8g6;rD;g7;b9r2r5r1rD", "gDgDgDgD;rD;bDbDbDbD;ff;b6;g5;;r9b8;r4g9;r8b7r6;r7;r3rDg8g6;rD;g7;b9r2r5r1rD", "gDgDgDgD;rD;bDbDbDbD;ff;b6;g6;;r9b8;r4g9;r8b7r6;r7;r3rDg8;rD;g7;b9r2r5r1rD", "gDgDgDgD;rD;bDbDbDbD;ff;b6;g7;;r9b8;r4g9;r8b7r6;r7;r3rDg8;rD;;b9r2r5r1rD", "gDgDgDgD;rD;bDbDbDbD;ff;b6;g8;;r9b8;r4g9;r8b7r6;r7;r3rD;rD;;b9r2r5r1rD", "gDgDgDgD;rDrDrDrD;bDbDbDbD;ff;b6;g8;;r9b8;r4g9;r8b7r6;r7;r3;;;b9r2r5r1", "gDgDgDgD;rDrDrDrD;bDbDbDbD;ff;b6;g9;;r9b8;r4;r8b7r6;r7;r3;;;b9r2r5r1", "gDgDgDgD;rDrDrDrD;bDbDbDbD;ff;b6;g9;r1;r9b8;r4;r8b7r6;r7;r3;;;b9r2r5", "gDgDgDgD;rDrDrDrD;bDbDbDbD;ff;b6;g9;r1;r9b8;r4;r8b7r6;r7;r3;r5;;b9r2", "gDgDgDgD;rDrDrDrD;bDbDbDbD;ff;b6;g9;r2;r9b8;r4;r8b7r6;r7;r3;r5;;b9", "gDgDgDgD;rDrDrDrD;bDbDbDbD;ff;b6;g9;r3;r9b8;r4;r8b7r6;r7;;r5;;b9", "gDgDgDgD;rDrDrDrD;bDbDbDbD;ff;b6;g9;r4;r9b8;;r8b7r6;r7;;r5;;b9", "gDgDgDgD;rDrDrDrD;bDbDbDbD;ff;b6;g9;r5;r9b8;;r8b7r6;r7;;;;b9", "gDgDgDgD;rDrDrDrD;bDbDbDbD;ff;b6;g9;r6;r9b8;;r8b7;r7;;;;b9", "gDgDgDgD;rDrDrDrD;bDbDbDbD;ff;b7;g9;r6;r9b8;;r8;r7;;;;b9", "gDgDgDgD;rDrDrDrD;bDbDbDbD;ff;b8;g9;r6;r9;;r8;r7;;;;b9", "gDgDgDgD;rDrDrDrD;bDbDbDbD;ff;b9;g9;r6;r9;;r8;r7;;;;", "gDgDgDgD;rDrDrDrD;bDbDbDbD;ff;b9;g9;r7;r9;;r8;;;;;", "gDgDgDgD;rDrDrDrD;bDbDbDbD;ff;b9;g9;r8;r9;;;;;;;", "gDgDgDgD;rDrDrDrD;bDbDbDbD;ff;b9;g9;r9;;;;;;;;"
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

    // #[test]
    // fn solve_random() {
    //     let board = new_game();
    //     println!("solving {:?}", board.encode());
    //     match solve(&board) {
    //         None => panic!("couldn't solve {}", board.encode()),
    //         Some((path, cost)) => {}
    //     }
    // }
}
