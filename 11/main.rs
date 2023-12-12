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

// const FILENAME: &str = "./example_input";
const FILENAME: &str = "./input";

fn main() {
    let lines = read_lines(FILENAME).unwrap();
    let mut map = collect_map(lines);
    expand(&mut map);
    let galaxies = find_galaxies(&map);
    dbg!(&galaxies);
    dbg!(galaxies.len());
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

fn expand(map: &mut SpaceMap) {
    let mut horizontal_idxs = vec![];
    for i in 0..map.len() {
        let line = &map[i];
        if is_empty(line) {
            horizontal_idxs.push(i);
        }
    }

    while !horizontal_idxs.is_empty() {
        map.insert(
            *horizontal_idxs.last().unwrap(),
            std::iter::repeat(b'.').take(map[0].len()).collect::<Vec<u8>>()
        );
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
        for i in 0..map.len() {
            map[i].insert(*last, b'.');
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
