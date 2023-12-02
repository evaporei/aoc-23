use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Debug, Default)]
struct Set {
    red: Option<u8>,
    green: Option<u8>,
    blue: Option<u8>,
}

impl Set {
    fn red(&mut self, amount: u8) {
        self.red = Some(amount);
    }
    fn green(&mut self, amount: u8) {
        self.green = Some(amount);
    }
    fn blue(&mut self, amount: u8) {
        self.blue = Some(amount);
    }
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

        let mut str_sets = line.split(';');
        let first_set = str_sets.next().unwrap().to_owned();
        let first_set = first_set.split(':').skip(1).next().unwrap();

        let mut cubes = first_set.split(',');

        let mut set = Set::default();

        while let Some(cube) = cubes.next() {
            if cube.contains("red") {
                let amount = cube.split(' ').skip(1).next().unwrap();
                set.red(amount.parse().unwrap());
            }
            if cube.contains("green") {
                let amount = cube.split(' ').skip(1).next().unwrap();
                set.green(amount.parse().unwrap());
            }
            if cube.contains("blue") {
                let amount = cube.split(' ').skip(1).next().unwrap();
                set.blue(amount.parse().unwrap());
            }
        }

        let mut sets = vec![set];

        for str_set in str_sets {
            let mut cubes = str_set.split(',');

            let mut set = Set::default();
            while let Some(cube) = cubes.next() {
                if cube.contains("red") {
                    let amount = cube.split(' ').skip(1).next().unwrap();
                    set.red(amount.parse().unwrap());
                }
                if cube.contains("green") {
                    let amount = cube.split(' ').skip(1).next().unwrap();
                    set.green(amount.parse().unwrap());
                }
                if cube.contains("blue") {
                    let amount = cube.split(' ').skip(1).next().unwrap();
                    set.blue(amount.parse().unwrap());
                }
            }

            sets.push(set);
        }

        Self {
            id,
            sets,
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
