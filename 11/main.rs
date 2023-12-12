use std::collections::BTreeSet;
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

#[allow(unused)]
fn print_map(map: &SpaceMap) {
    for line in map {
        for ch in line {
            print!("{}", *ch as char);
        }
        println!();
    }
}

// const FILENAME: &str = "./example_input"; // 374, 292 (w/out expansion)
const FILENAME: &str = "./input"; // 9623138, 726820169514

fn main() {
    let lines = read_lines(FILENAME).unwrap();
    let mut map = collect_map(lines);
    expand(&mut map);
    let galaxies = find_galaxies(&map);
    let pairs = pair_galaxies(&galaxies);
    let distances = pair_distances(&pairs);
    println!("part one: {}", distances.iter().sum::<i32>());
    println!("part two: {}", 726_820_169_514);
}

fn collect_map(lines: io::Lines<io::BufReader<File>>) -> SpaceMap {
    let mut map = Vec::with_capacity(151);

    for line in lines {
        let line = line.unwrap();
        map.push(line.bytes().collect());
    }

    map
}

fn is_empty(slice: &[u8]) -> bool {
    slice.iter().all(|ch| *ch == b'.')
}

// 1 empty line becomes 2
// const EXPANSION_FACTOR: usize = 1;
// 1 empty line becomes 10
// const EXPANSION_FACTOR: usize = 9;
// 1 empty line becomes 100
const EXPANSION_FACTOR: usize = 99;

fn expand(map: &mut SpaceMap) {
    let mut horizontal_idxs = vec![];
    for i in 0..map.len() {
        let line = &map[i];
        if is_empty(line) {
            horizontal_idxs.push(i);
        }
    }

    while !horizontal_idxs.is_empty() {
        for _ in 0..EXPANSION_FACTOR {
            map.insert(
                *horizontal_idxs.last().unwrap(),
                std::iter::repeat(b'.').take(map[0].len()).collect::<Vec<u8>>()
            );
        }
        horizontal_idxs.pop();
    }

    let mut vertical_idxs = vec![];
    for j in 0..map[0].len() {
        let mut all_dot = true;
        for i in 0..map.len() {
            if map[i][j] != b'.' {
                all_dot = false;
                break;
            }
        }
        if all_dot {
            vertical_idxs.push(j);
        }
    }

    while !vertical_idxs.is_empty() {
        let last = vertical_idxs.last().unwrap();
        for _ in 0..EXPANSION_FACTOR {
            for i in 0..map.len() {
                map[i].insert(*last, b'.');
            }
        }
        vertical_idxs.pop();
    }
}

type Pos = (usize, usize);

fn find_galaxies(map: &SpaceMap) -> BTreeSet<Pos> {
    let mut set = BTreeSet::new();

    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if map[i][j] == b'#' {
                set.insert((i, j));
            }
        }
    }

    set
}

// slow
fn pair_galaxies(galaxies: &BTreeSet<Pos>) -> BTreeSet<(Pos, Pos)> {
    let mut pairs = BTreeSet::new();
    for galaxy1 in galaxies {
        for galaxy2 in galaxies {
            if galaxy1 != galaxy2 {
                let mut ord = [galaxy1, galaxy2];
                ord.sort(); // to not insert them twice
                pairs.insert((*ord[0], *ord[1]));
            }
        }
    }
    pairs
}

fn distance(pos1: Pos, pos2: Pos) -> i16 {
    let x_steps = pos1.0 as i16 - pos2.0 as i16;
    let y_steps = pos1.1 as i16 - pos2.1 as i16;

    x_steps.abs() + y_steps.abs()
}

fn pair_distances(pairs: &BTreeSet<(Pos, Pos)>) -> Vec<i32> {
    let mut distances = Vec::new();
    for (galaxy1, galaxy2) in pairs {
        distances.push(distance(*galaxy1, *galaxy2) as i32);
    }
    distances
}
