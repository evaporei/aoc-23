use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

// const FILENAME: &str = "./example_input";
const FILENAME: &str = "./input";

fn main() {
    let lines = read_lines(FILENAME).unwrap();

    for line in lines {
        let line = line.unwrap();

        println!("{line}");
    }
}
