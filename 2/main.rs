use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Debug)]
struct Set {
    red: u8,
    green: u8,
    blue: u8,
}

#[derive(Debug)]
struct Game {
    id: u8,
    sets: Vec<Set>,
}

impl Game {
    fn parse(line: &str) -> Self {
        let id = line.split(' ').skip(1).next().unwrap();
        let id = id[..id.len() - 1].parse().unwrap(); // remove `:`
        Self {
            id,
            sets: vec![],
        }
    }
}

fn main() {
    let lines = read_lines("./easy_input_part_one").unwrap(); // 8

    for line in lines {
        let line = line.unwrap();
        println!("{line}");
        let game = Game::parse(&line);
        println!("{game:?}");
    }
}
