use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    let lines = read_lines("./simple_loop").unwrap();
    // let lines = read_lines("./complex_loop").unwrap();
    // let lines = read_lines("./input").unwrap();
    let s = find_starting_position(lines);

    println!("{s:?}");


}

fn find_starting_position(lines: io::Lines<io::BufReader<File>>) -> (usize, usize) {
    for (i, line) in lines.enumerate() {
        let line = line.unwrap();

        for (j, ch) in line.bytes().enumerate() {
            if ch == b'S' {
                return (i, j);
            }
        }
    }

    unreachable!("there should always be a S in the map");
}
