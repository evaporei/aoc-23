use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse_seeds(line: &str) -> Vec<u32> {
    let numbers = line // "seeds: 79 14 55 13"
        .split(':') // ["seeds", " 79 14 55 13"]
        .skip(1) // [" 79 14 55 13"]
        .next() // " 79 14 55 13"
        .unwrap()
        .split_whitespace(); // ["79", "14", "55", "13"]

    numbers.fold(vec![], |mut acc, n| {
        acc.push(n.parse().unwrap());
        acc
    })
}

fn main() {
    let lines = read_lines("./easy_input_part_one").unwrap(); // 13
    // let lines = read_lines("./input").unwrap(); // 32609, 14624680
    let mut seeds = vec![];

    for line in lines {
        let line = line.unwrap();
        if line.starts_with("seeds:") {
            seeds = parse_seeds(&line);
        }
    }

    dbg!(seeds);
}
