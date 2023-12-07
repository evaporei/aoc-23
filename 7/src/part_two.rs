use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

// type in the readings
#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
enum Kind {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

type CardMap = HashMap<u8, usize>;

// each entry has the count of that card
fn card_map(cards: &str) -> CardMap {
    let mut map = HashMap::new();

    for card in cards.bytes() {
        if card == b'J' {
            continue;
        }
        map.entry(card)
            .and_modify(|c| *c += 1)
            .or_insert(1);
    }

    // all 'J's
    if map.is_empty() {
        map.insert(b'J', 5);
    }

    map
}

impl Kind {
    fn is_five_of_a_kind(cards: &CardMap) -> bool {
        Self::is_n_of_a_kind(cards, 5)
    }
    fn is_four_of_a_kind(cards: &CardMap) -> bool {
        Self::is_n_of_a_kind(cards, 4)
    }
    fn is_full_house(cards: &CardMap) -> bool {
        Self::is_three_of_a_kind(cards)
            && Self::is_n_of_a_kind(cards, 2)
    }
    fn is_three_of_a_kind(cards: &CardMap) -> bool {
        Self::is_n_of_a_kind(cards, 3)
    }
    fn is_two_pair(cards: &CardMap) -> bool {
        Self::is_n_pair(cards, 2)
    }
    fn is_one_pair(cards: &CardMap) -> bool {
        Self::is_n_pair(cards, 1)
    }
    fn is_n_of_a_kind(map: &CardMap, n: usize) -> bool {
        map.values().any(|count| *count == n)
    }
    fn is_n_pair(map: &CardMap, n: usize) -> bool {
        let mut pair_count = 0;

        for count in map.values() {
            if *count == 2 {
                pair_count += 1;
            }
        }

        pair_count == n
    }
}

#[test]
fn test_kind_checks() {
    assert_eq!(Kind::from("JJJJJ"), Kind::FiveOfAKind);
    assert_eq!(Kind::from("AAAAA"), Kind::FiveOfAKind);
    assert_eq!(Kind::from("22222"), Kind::FiveOfAKind);
    assert_eq!(Kind::from("J2JJJ"), Kind::FiveOfAKind);
    assert_eq!(Kind::from("33JJJ"), Kind::FiveOfAKind);

    assert_eq!(Kind::from("22322"), Kind::FourOfAKind);
    assert_eq!(Kind::from("J2J44"), Kind::FourOfAKind);

    assert_eq!(Kind::from("22QQ2"), Kind::FullHouse);
    assert_eq!(Kind::from("33TTT"), Kind::FullHouse);
    assert_eq!(Kind::from("4T44T"), Kind::FullHouse);

    assert_eq!(Kind::from("223Q2"), Kind::ThreeOfAKind);
}

impl From<&str> for Kind {
    fn from(cards: &str) -> Self {
        let mut map = card_map(cards);

        if cards.contains('J') {
            let j_count = cards.bytes().filter(|ch| *ch == b'J').count();
            if j_count == 5 {
                return Self::FiveOfAKind;
            }

            let most_matches = map.values().max().copied().unwrap();

            for count in map.values_mut() {
                if *count == most_matches {
                    *count += j_count;
                    break;
                }
            }
        }

        if Self::is_five_of_a_kind(&map) {
            return Self::FiveOfAKind;
        }

        if Self::is_four_of_a_kind(&map) {
            return Self::FourOfAKind;
        }

        // has to be checked before three_of_a_kind
        if Self::is_full_house(&map) {
            return Self::FullHouse;
        }

        if Self::is_three_of_a_kind(&map) {
            return Self::ThreeOfAKind;
        }

        if Self::is_two_pair(&map) {
            return Self::TwoPair;
        }

        if Self::is_one_pair(&map) {
            return Self::OnePair;
        }

        Self::HighCard
    }
}

#[derive(Debug, Ord, Eq)]
struct Hand {
    // A, K, Q, J, T, 9, 8, 7, 6, 5, 4, 3, or 2
    cards: String,
    bid: u32,
    kind: Kind,
    rank: u32,
}

impl Hand {
    fn parse(line: &str) -> Self {
        let mut it = line.split_whitespace();
        let cards = it.next().unwrap().to_string();
        let bid = it.next().unwrap().parse().unwrap();
        let kind = Kind::from(cards.as_str());
        
        Self {
            cards,
            bid,
            kind,
            rank: 0,
        }
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind
    }
}

// A, K, Q, T, 9, 8, 7, 6, 5, 4, 3, 2 or J
#[derive(Eq, PartialEq, Ord, PartialOrd)]
enum Card {
    A,
    K,
    Q,
    T,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
    J,
}

impl From<u8> for Card {
    fn from(ch: u8) -> Self {
        match ch {
            b'A' => Self::A,
            b'K' => Self::K,
            b'Q' => Self::Q,
            b'T' => Self::T,
            b'9' => Self::Nine,
            b'8' => Self::Eight,
            b'7' => Self::Seven,
            b'6' => Self::Six,
            b'5' => Self::Five,
            b'4' => Self::Four,
            b'3' => Self::Three,
            b'2' => Self::Two,
            b'J' => Self::J,
            _ => unreachable!("k a b o o m"),
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let kind_cmp = self.kind.partial_cmp(&other.kind);
        if kind_cmp != Some(Ordering::Equal) {
            return kind_cmp;
        }
        for (card1, card2) in self.cards.bytes().zip(other.cards.bytes()) {
            let card1 = Card::from(card1);
            let card2 = Card::from(card2);

            let card_cmp = card1.partial_cmp(&card2);
            if card_cmp == Some(Ordering::Equal) {
                continue;
            } else {
                return card_cmp;
            }
        }
        kind_cmp
    }
}

pub fn run() {
    // let lines = read_lines("./easy_input_part_one").unwrap(); // 5905
    let lines = read_lines("./input").unwrap(); // 248191286, 246135914 (too high), 245672634 (too
                                                // low), 245794069 (weeee)
    let mut hands = vec![];

    for line in lines {
        let line = line.unwrap();

        let hand = Hand::parse(&line);
        hands.push(hand);
    }

    hands.sort();

    let mut total = 0;

    for (i, hand) in hands.iter().enumerate() {
        let rank = hands.len() - i;
        let winnings = hand.bid * rank as u32;
        total += winnings;
    }

    println!("part two {total}");
}
