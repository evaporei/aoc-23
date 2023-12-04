use std::collections::BTreeSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

// they're actually sets instead of lists
fn parse_number_list(list: &str) -> BTreeSet<u8> {
    let mut l = BTreeSet::new();
    // example input: " 41 48 83 86 17 "
    // ["41", "48", "83", "86", "17"]
    let numbers = list.trim().split_whitespace();

    // "41" | "48" | "83" | ...
    for n in numbers {
        l.insert(n.parse().unwrap());
    }

    l
}

#[derive(Debug)]
struct Card {
    id: u16,
    winning: BTreeSet<u8>, // [10; u8] (5 in example input)
    have: BTreeSet<u8>, // [25; u8] (8 in example input)
}

impl Card {
    fn parse(line: &str) -> Self {
        // example input: "Card   1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53"
        let id = line.split_whitespace() // ["Card", "1:", ...]
            .skip(1) // ["1:", ...]
            .next() // "1:"
            .unwrap();
        // remove `:`
        let id = id[..id.len() - 1].parse().unwrap();

        // ["Card 1: 41 48 83 86 17 ", " 83 86  6 31 17  9 48 53"]
        let mut two_sets = line.split('|');

        // "Card 1: 41 48 83 86 17 "
        let first_set = two_sets.next().unwrap().to_owned();
        // " 41 48 83 86 17 "
        let first_set = first_set.split(':').skip(1).next().unwrap();
        let first_set = parse_number_list(first_set);

        // " 83 86  6 31 17  9 48 53"
        let second_set = two_sets.next().unwrap().to_owned();
        let second_set = parse_number_list(&second_set);

        Self {
            id,
            winning: first_set,
            have: second_set,
        }
    }
}

fn main() {
    // let lines = read_lines("./easy_input_part_one").unwrap(); // 13
    let lines = read_lines("./input").unwrap(); // 32609
    let mut points = 0;
    let mut lens = vec![];

    for line in lines {
        let line = line.unwrap();
        let card = Card::parse(&line);

        let intersection: Vec<_> = card
            .winning
            .intersection(&card.have)
            .cloned()
            .collect();

        lens.push(intersection.len());
    }

    for len in lens {
        if let Some(l) = len.checked_sub(1) {
            points += 1 << l;
        }
    }

    println!("{points}");
}
