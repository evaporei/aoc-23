use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    let mut lines = read_lines("./easy_input_part_one").unwrap(); // 35
    // let mut lines = read_lines("./input").unwrap();

    // parsing
    let time_str = lines.next().unwrap().unwrap();
    let times: Vec<u16> = time_str
        .split_whitespace()
        .skip(1)
        .map(str::parse)
        .map(Result::unwrap)
        .collect();
    let distance_str = lines.next().unwrap().unwrap();
    let distances: Vec<u16> = distance_str
        .split_whitespace()
        .skip(1)
        .map(str::parse)
        .map(Result::unwrap)
        .collect();

    dbg!(times);
    dbg!(distances);
}
