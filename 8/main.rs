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
    // let mut lines = read_lines("./example_part_two").unwrap(); // 6
    let mut lines = read_lines("./input").unwrap(); // 18157, 19783 (too low),
    // 201405124411 (lcm, wrong ;-;), 14299763833181 (yay!)

    // LLR
    let instructions = lines.next().unwrap().unwrap();

    // empty line
    let _ = lines.next();

    let mut map = BTreeMap::new();
    let mut a_steps = vec![];

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

        if origin.ends_with('A') {
            a_steps.push(origin.clone());
        }

        // we don't push Z into the map
        // so we don't loop forever in count_steps
        if !origin.ends_with('Z') {
            map.entry(origin)
                // just insert the first occurrence
                .or_insert((left, right));
        }
    }

    let n_steps = count_steps("AAA", &map, &instructions);
    println!("part one: {n_steps}");

    let mut everyone_at_z = 1;
    for a_step in a_steps {
        let n_steps = count_steps(&a_step, &map, &instructions);
        everyone_at_z = lcm(everyone_at_z, n_steps);
    }
    println!("part two: {everyone_at_z}");
}

// counts the steps until we find a Z in the right, aka (_, --Z)
// this only works if the map doesn't contain keys that end w/ Z
fn count_steps(start: &str, map: &BTreeMap<String, (String, String)>, instructions: &str) -> u64 {
    let (mut l, mut r) = map.get(start).unwrap().clone();
    let mut n_steps = 1;
    let mut i = 0;

    loop {
        if i == instructions.len() {
            i = 0;
        }
        let curr_instruction = instructions.bytes().nth(i).unwrap();
        (l, r) = match curr_instruction {
            // we can always unwrap, because it always ends in R (last Z)
            b'L' => map.get(&l).unwrap().clone(),
            b'R' => match map.get(&r) {
                Some(directions) => directions.clone(),
                // Z is not a key in the map, so we got to the end
                None => break,
            },
            _ => unreachable!("bad input in instructions, only L and R are allowed"),
        };
        n_steps += 1;
        i += 1;
    }

    n_steps
}

fn lcm(first: u64, second: u64) -> u64 {
    first * second / gcd(first, second)
}

fn gcd(first: u64, second: u64) -> u64 {
    let mut max = first;
    let mut min = second;
    if min > max {
        let val = max;
        max = min;
        min = val;
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}
