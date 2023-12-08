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
    let path = lines.next().unwrap().unwrap();

    println!("{path}");

    // empty line
    let _ = lines.next();

    for line in lines {
        // CCC = (BBB, DDD)
        let line = line.unwrap();
        println!("{line}");
    }
}
