use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn n_ways_to_beat(time: u64, record: u64) -> u64 {
    let mut n = 0;

    for hold in 1..time {
        let time_left = time - hold;
        let distance_travelled = hold * time_left;
        if distance_travelled > record {
            n += 1;
        }
    }

    n
}

const fn comptime<const TIME: u64>(record: u64) -> u64 {
    let mut n = 0;
    let mut hold = 1;

    while hold < TIME {
        let time_left = TIME - hold;
        let distance_travelled = hold * time_left;
        if distance_travelled > record {
            n += 1;
        }
        hold += 1;
    }

    n
}

#[test]
fn test_n_ways_to_beat() {
    assert_eq!(n_ways_to_beat(7, 9), 4);
    assert_eq!(n_ways_to_beat(15, 40), 8);
    assert_eq!(n_ways_to_beat(30, 200), 9);
}

fn part_one() {
    // let mut lines = read_lines("./easy_input_part_one").unwrap(); // 288
    let mut lines = read_lines("./input").unwrap(); // 840336

    // parsing
    let time_str = lines.next().unwrap().unwrap();
    let times: Vec<u64> = time_str
        .split_whitespace()
        .skip(1)
        .map(str::parse)
        .map(Result::unwrap)
        .collect();
    let distance_str = lines.next().unwrap().unwrap();
    let distances: Vec<u64> = distance_str
        .split_whitespace()
        .skip(1)
        .map(str::parse)
        .map(Result::unwrap)
        .collect();
    let mut total: u64 = 1;

    for (time, distance) in times.iter().zip(distances.iter()) {
        total *= n_ways_to_beat(*time, *distance);
    }

    println!("part one {total}");
}

fn part_two() {
    const TIME: u64 = 62_649_190;
    let distance = 553_101_014_731_074;

    let total: u64 = comptime::<TIME>(distance);

    println!("part two {total}"); // 41382569
}

fn main() {
    part_one();
    part_two();
}
