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

    fn parse(str_set: &str) -> Self {
        // example input: " 1 red, 2 green, 6 blue"
        // [" 1 red", " 2 green", " 6 blue"]
        let mut cubes = str_set.split(',');
        let mut set = Set::default();

        // " 1 red" | " 2 green" | " 6 blue"
        while let Some(cube) = cubes.next() {
            if cube.contains("red") {
                let amount = cube.split(' ') // [" ", "1", "red"]
                    .skip(1) // ["1", "red"]
                    .next() // "1"
                    .unwrap();
                set.red(amount.parse().unwrap());
            }
            if cube.contains("green") {
                let amount = cube.split(' ')
                    .skip(1)
                    .next()
                    .unwrap();
                set.green(amount.parse().unwrap());
            }
            if cube.contains("blue") {
                let amount = cube.split(' ')
                    .skip(1)
                    .next()
                    .unwrap();
                set.blue(amount.parse().unwrap());
            }
        }

        set
    }
}

#[derive(Debug)]
struct Game {
    id: u8,
    sets: Vec<Set>,
}

impl Game {
    fn parse(line: &str) -> Self {
        // example input: "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"
        let id = line.split(' ') // ["Game", "1:", ...]
            .skip(1) // ["1:", ...]
            .next() // "1:"
            .unwrap();
        // remove `:`
        let id = id[..id.len() - 1].parse().unwrap();

        // ["Game 1: 3 blue, 4 red", "1 red, 2 green, 6 blue", "2 green"]
        let mut str_sets = line.split(';');

        // "Game 1: 3 blue, 4 red"
        let first_set = str_sets.next().unwrap().to_owned();
        // " 3 blue, 4 red"
        let first_set = first_set.split(':').skip(1).next().unwrap();

        let first_set = Set::parse(first_set);

        let mut sets = vec![first_set];

        for str_set in str_sets {
            sets.push(Set::parse(str_set));
        }

        Self {
            id,
            sets,
        }
    }
}

fn main() {
    // let lines = read_lines("./easy_input_part_one").unwrap(); // 8, 2286
    let lines = read_lines("./input").unwrap(); // 2369, 66363
    let mut id_sum: u16 = 0;
    let mut power_sum = 0;

    'outer: for line in lines {
        let line = line.unwrap();
        let game = Game::parse(&line);

        // part two
        let biggest = game.sets.iter().fold((0, 0, 0), |(mut red, mut green, mut blue), curr| {
            if curr.red > Some(red) {
                red = curr.red.unwrap();
            }
            if curr.green > Some(green) {
                green = curr.green.unwrap();
            }
            if curr.blue > Some(blue) {
                blue = curr.blue.unwrap();
            }
            (red, green, blue)
        });
        let power = biggest.0 as u32 * biggest.1 as u32 * biggest.2 as u32;
        power_sum += power;

        // part one
        for set in &game.sets {
            if set.red > Some(12) || set.green > Some(13) || set.blue > Some(14) {
                continue 'outer;
            }
        }

        id_sum += game.id as u16;
    }

    println!("part one: {id_sum}");
    println!("part two: {power_sum}");
}
