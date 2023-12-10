use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

const FILENAME: &str = "./simple_loop";
// const FILENAME: &str = "./complex_loop";
// const FILENAME: &str = "./input";

type Pos = (usize, usize);
type Map = Vec<Vec<u8>>;

fn main() {
    let lines = read_lines(FILENAME).unwrap();
    let map = collect_map(lines);

    let s = find_starting_position(&map);

    println!("{s:?}");


}

fn collect_map(lines: io::Lines<io::BufReader<File>>) -> Map {
    let mut map = Vec::with_capacity(140);

    for line in lines {
        let line = line.unwrap();
        map.push(line.bytes().collect());
    }

    map
}

fn find_starting_position(map: &Map) -> Pos {
    for (i, line) in map.iter().enumerate() {
        for (j, ch) in line.iter().enumerate() {
            if *ch == b'S' {
                return (i, j);
            }
        }
    }

    unreachable!("there should always be a S in the map");
}
