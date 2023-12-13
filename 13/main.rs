use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

// . -> 0
// # -> 1
fn encode(s: &str) -> u32 {
    // lol easy
    let bin = s.replace('.', "0").replace('#', "1");
    u32::from_str_radix(&bin, 2).unwrap()
}

#[test]
fn test_encode() {
    assert_eq!(encode(".###.#."), 0b0111010);
}

const FILENAME: &str = "./example_input";
// const FILENAME: &str = "./input";

fn main() {
    let lines = read_lines(FILENAME).unwrap();
    let mut h_patterns = vec![vec![]];

    for line in lines {
        let line = line.unwrap();

        if !line.is_empty() {
            if let Some(p) = h_patterns.last_mut() {
                p.push(encode(&line));
            }
        } else {
            h_patterns.push(vec![]);
        }
    }

    dbg!(&h_patterns);
    dbg!(&h_patterns.len());
}
