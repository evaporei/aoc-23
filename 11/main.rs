use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

// I've always wanted to have a type w/ Space in it's name lol
type SpaceMap = Vec<Vec<u8>>;

// const FILENAME: &str = "./example_input";
const FILENAME: &str = "./input";

fn main() {
    let lines = read_lines(FILENAME).unwrap();
    let map = collect_map(lines);
}

fn collect_map(lines: io::Lines<io::BufReader<File>>) -> SpaceMap {
    let mut map = Vec::with_capacity(151);

    for line in lines {
        let line = line.unwrap();
        map.push(line.bytes().collect());
    }

    map
}
