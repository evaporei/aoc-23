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
    let mut lines = read_lines("./example_part_two").unwrap(); // 6
    // let mut lines = read_lines("./input").unwrap(); // 18157, 19783 (too low),
    // 201405124411 (lcm, wrong ;-;)

    // LLR
    let steps = lines.next().unwrap().unwrap();

    // empty line
    let _ = lines.next();

    // let a_steps = ["AAA", "RLA", "QLA", "QFA", "RXA", "JSA"];
    // let z_steps = ["QCZ", "LRZ", "ZZZ", "PQZ", "VHZ", "JJZ"];
    let z_steps = ["11Z", "22Z"];

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

        // if origin != "ZZZ" {
        if !z_steps.contains(&origin.as_str()) {
            // just insert the first one
            map.entry(origin)
                .or_insert((left, right));
        }
    }

    // let n_steps = find_steps("AAA", "ZZZ", &map, &steps);
    // let n_steps = find_steps("AAA", &["ZZZ"], &map, &steps);
    // println!("part one {n_steps}");

    // for a_step in a_steps {
    //     let n_steps = find_steps(a_step, &z_steps, &map, &steps);
    //     println!("part two {a_step}: {n_steps}");
    // }
    println!("part two:");
    // let n_steps = find_steps("AAA", &["ZZZ"], &map, &steps);
    // println!("{} - {}: {n_steps}", "AAA", "ZZZ"); // 18157
    // let n_steps = find_steps("RLA", &["JJZ"], &map, &steps);
    // println!("{} - {}: {n_steps}", "RLA", "JJZ"); // 14363
    // let n_steps = find_steps("QLA", &["VHZ"], &map, &steps);
    // println!("{} - {}: {n_steps}", "QLA", "VHZ"); // 16531
    // let n_steps = find_steps("QFA", &["PQZ"], &map, &steps);
    // println!("{} - {}: {n_steps}", "QFA", "PQZ"); // 12737
    // let n_steps = find_steps("RXA", &["QCZ"], &map, &steps);
    // println!("{} - {}: {n_steps}", "RXA", "QCZ"); // 19783 (max)
    // let n_steps = find_steps("AAA", &["LRZ"], &map, &steps);
    // println!("{} - {}: {n_steps}", "JSA", "LRZ"); // 18157

    // example part two
    let n_steps = find_steps("11A", &["11Z"], &map, &steps);
    println!("{} - {}: {n_steps}", "11A", "11Z"); // 18157
    let n_steps = find_steps("22A", &["22Z"], &map, &steps);
    println!("{} - {}: {n_steps}", "22A", "22Z"); // 18157
}

// fn find_steps(start: &str, end: &str, map: &BTreeMap<String, (String, String)>, steps: &str) -> u32 {
fn find_steps(start: &str, ends: &[&str], map: &BTreeMap<String, (String, String)>, steps: &str) -> u64 {
    let (mut l, mut r) = map.get(start).unwrap().clone();
    let mut n_steps = 1;
    let mut i = 0;

    // while l != end || r != end {
    while !ends.contains(&l.as_str()) || !ends.contains(&r.as_str()) {
    // while n_steps <= 201_405_124_411 {
        if i == steps.len() {
            i = 0;
        }
        let step = steps.bytes().nth(i).unwrap();
        (l, r) = match step {
            b'L' => match map.get(&l) {
                Some(directions) => directions.clone(),
                None => break,
            },
            b'R' => match map.get(&r) {
                Some(directions) => directions.clone(),
                None => break,
            },
            _ => unreachable!("bad input, only L and R are allowed"),
        };
        n_steps += 1;
        i += 1;
    }

    n_steps
}
