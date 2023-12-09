use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    let lines = read_lines("./example_part_one").unwrap();
    // let lines = read_lines("./input").unwrap();

    for line in lines {
        let line = line.unwrap();

        let numbers: Vec<i32> = line
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();

        dbg!(&numbers);

        diff(&numbers);
    }
}

fn diff(numbers: &[i32]) {
    if numbers.iter().all(|n| *n == 0) {
        return;
    }

    let next: Vec<i32> = numbers
        .windows(2)
        .map(|ns| ns[1] - ns[0])
        .collect();

    dbg!(&next);

    diff(&next)
}
