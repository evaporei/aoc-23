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
#[derive(Debug)]
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
        map.entry(card)
            .and_modify(|c| *c += 1)
            .or_insert(1);
    }

    map
}

impl Kind {
    fn is_five_of_a_kind(cards: &str) -> bool {
        for pair in cards.as_bytes().windows(2) {
            if pair[0] != pair[1] {
                return false;
            }
        }

        true
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
    assert_eq!(Kind::is_five_of_a_kind("AAAAA"), true);
    assert_eq!(Kind::is_five_of_a_kind("22222"), true);
    assert_eq!(Kind::is_five_of_a_kind("22322"), false);

    assert_eq!(Kind::is_four_of_a_kind(&card_map("22322")), true);
    assert_eq!(Kind::is_four_of_a_kind(&card_map("AAAAA")), false);
    assert_eq!(Kind::is_four_of_a_kind(&card_map("J2J44")), false);
    assert_eq!(Kind::is_four_of_a_kind(&card_map("J2JJJ")), true);

    assert_eq!(Kind::is_full_house(&card_map("22QQ2")), true);
    assert_eq!(Kind::is_full_house(&card_map("22TQ2")), false);
    assert_eq!(Kind::is_full_house(&card_map("22322")), false);
    assert_eq!(Kind::is_full_house(&card_map("AAAAA")), false);
    assert_eq!(Kind::is_full_house(&card_map("J2J44")), false);
    assert_eq!(Kind::is_full_house(&card_map("33JJJ")), true);

    assert_eq!(Kind::is_three_of_a_kind(&card_map("223Q2")), true);
    assert_eq!(Kind::is_three_of_a_kind(&card_map("22322")), false);
    assert_eq!(Kind::is_three_of_a_kind(&card_map("AAAAA")), false);
    assert_eq!(Kind::is_three_of_a_kind(&card_map("J2J44")), false);
    assert_eq!(Kind::is_three_of_a_kind(&card_map("32JJJ")), true);

    assert_eq!(Kind::is_two_pair(&card_map("4242J")), true);
    assert_eq!(Kind::is_two_pair(&card_map("22322")), false);
    assert_eq!(Kind::is_two_pair(&card_map("22322")), false);
    assert_eq!(Kind::is_two_pair(&card_map("AAAAA")), false);
    assert_eq!(Kind::is_two_pair(&card_map("J44QJ")), true);

    assert_eq!(Kind::is_one_pair(&card_map("4K42J")), true);
    assert_eq!(Kind::is_one_pair(&card_map("4242J")), false);
    assert_eq!(Kind::is_one_pair(&card_map("22322")), false);
    assert_eq!(Kind::is_one_pair(&card_map("4KT2J")), false);
    assert_eq!(Kind::is_one_pair(&card_map("AAAAA")), false);
    assert_eq!(Kind::is_one_pair(&card_map("J4TQJ")), true);
}

impl From<&str> for Kind {
    fn from(cards: &str) -> Self {
        if Self::is_five_of_a_kind(cards) {
            return Self::FiveOfAKind;
        }

        let map = card_map(cards);

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

#[derive(Debug)]
struct Hand {
    // A, K, Q, J, T, 9, 8, 7, 6, 5, 4, 3, or 2
    cards: String,
    bid: u16,
    kind: Kind,
    rank: u16,
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

fn part_one() {
    let lines = read_lines("./easy_input_part_one").unwrap();
    // let mut lines = read_lines("./input").unwrap();

    for line in lines {
        let line = line.unwrap();

        let hand = Hand::parse(&line);
        dbg!(hand);
    }
}

fn main() {
    part_one();
}
