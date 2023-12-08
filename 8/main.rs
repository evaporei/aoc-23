use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    let mut lines = read_lines("./example1").unwrap();
    // let lines = read_lines("./example2").unwrap();
    // let lines = read_lines("./input").unwrap();

    // LLR
    let steps = lines.next().unwrap().unwrap();

    dbg!(steps);

    // empty line
    let _ = lines.next();

    for line in lines {
        // CCC = (BBB, DDD)
        let line = line.unwrap();
        let mut line = line.split('=');

        let mut path = line.next().unwrap().to_owned();
        path.pop(); // \s

        let mut options = line.next().unwrap().split(',');
        let mut left = options.next().unwrap().to_owned();
        let mut right = options.next().unwrap().to_owned();

        left.remove(0); // (
        left.remove(0); // \s
        right.remove(4); // )
        right.remove(0); // \s

        println!("{left}, {right}");
    }
}
