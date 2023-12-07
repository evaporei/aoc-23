use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Debug)]
struct Hand {
    cards: String,
    bid: u16,
    rank: u16,
}

impl Hand {
    fn parse(line: &str) -> Self {
        let mut it = line.split_whitespace();
        let cards = it.next().unwrap().to_string();
        let bid = it.next().unwrap().parse().unwrap();
        
        Self {
            cards,
            bid,
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
