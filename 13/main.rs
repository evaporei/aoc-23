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

// const FILENAME: &str = "./example_input";
// const MAX_LINE_LEN: usize = 9;

const FILENAME: &str = "./input";
const MAX_LINE_LEN: usize = 17;

fn main() {
    let lines = read_lines(FILENAME).unwrap();
    let mut h_patterns = vec![vec![]];
    let mut v_patterns = vec![vec![String::with_capacity(20); MAX_LINE_LEN]];

    for line in lines {
        let line = line.unwrap();

        if !line.is_empty() {
            if let Some(p) = h_patterns.last_mut() {
                p.push(encode(&line));
            }
            if let Some(p) = v_patterns.last_mut() {
                for (j, ch) in line.chars().enumerate() {
                    p[j].push(ch);
                }
            }
        } else {
            h_patterns.push(vec![]);
            v_patterns.push(vec![String::with_capacity(20); MAX_LINE_LEN]);
        }
    }

    // remove empty strings because we made a square
    // matrix based of the maximum line length
    let v_patterns: Vec<Vec<u32>> = v_patterns.into_iter()
        .map(|l| l.into_iter().filter_map(|s| {
            if s.is_empty() {
                None
            } else {
                Some(encode(&s))
            }
        }).collect())
        .collect();

    dbg!(&v_patterns);
    dbg!(&v_patterns.len());
}
