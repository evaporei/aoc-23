use std::collections::BTreeMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    // let mut lines = read_lines("./example1").unwrap(); // 2
    // let mut lines = read_lines("./example2").unwrap(); // 6
    let mut lines = read_lines("./input").unwrap(); // 18157

    // LLR
    let steps = lines.next().unwrap().unwrap();

    // empty line
    let _ = lines.next();

    let mut map = BTreeMap::new();

    for line in lines {
        // CCC = (BBB, DDD)
        let line = line.unwrap();
        let mut line = line.split('=');

        let mut origin = line.next().unwrap().to_owned();
        origin.pop(); // \s

        let mut directions = line.next().unwrap().split(',');
        let mut left = directions.next().unwrap().to_owned();
        let mut right = directions.next().unwrap().to_owned();

        left.remove(0); // (
        left.remove(0); // \s
        right.remove(4); // )
        right.remove(0); // \s

        if origin != "ZZZ" {
            if !map.contains_key(&origin) {
                map.insert(origin, (left, right));
            }
        }
    }

    let (mut l, mut r) = map.get("AAA").unwrap().clone();
    let mut n_steps = 1;
    let mut i = 0;

    while l != "ZZZ" || r != "ZZZ" {
        if i == steps.len() {
            i = 0;
        }
        let step = steps.bytes().nth(i).unwrap();
        (l, r) = match step {
            b'L' => {
                if map.get(&l).is_none() {
                    break;
                }
                map.get(&l).unwrap().clone()
            },
            b'R' => {
                if map.get(&r).is_none() {
                    break;
                }
                map.get(&r).unwrap().clone()
            },
            _ => unreachable!("bad input, only L and R are allowed"),
        };
        n_steps += 1;
        i += 1;
    }

    println!("part one {n_steps}");
}
