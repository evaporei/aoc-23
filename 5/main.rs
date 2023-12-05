use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

// input could be seed, soil, etc
fn dst_from_src(input: u64, dst: u64, src: u64, range: u64) -> Option<u64> {
    if src > input || src + range - 1 < input {
        return None;
    }

    // it could be (input - src) == 0
    if src == input {
        return Some(dst);
    }

    let diff = input - src;
    Some(dst + diff)
}

#[test]
fn test_src_to_dst() {
    let seed = 100;
    // 120 > 100, there's nothing to range, input returns
    assert_eq!(dst_from_src(seed, 13, 120, 50), None);
    // 100 == 100, return first dst
    assert_eq!(dst_from_src(seed, 50, 100, 2), Some(50));
    // 97 + 2 = 99, still lower than 100, return input
    assert_eq!(dst_from_src(seed, 50, 97, 2), None);

    // succeeds
    assert_eq!(dst_from_src(seed, 13, 55, 50), Some(58));

    // boundary checks
    assert_eq!(dst_from_src(seed, 13, 95, 5), None);

    // some from sample/easy input

    let seed = 79;
    assert_eq!(dst_from_src(seed, 50, 98, 2), None);
    assert_eq!(dst_from_src(seed, 52, 50, 48), Some(81));
    let seed = 14;
    assert_eq!(dst_from_src(seed, 50, 98, 2), None);
    assert_eq!(dst_from_src(seed, 52, 50, 48), None);
    let seed = 55;
    assert_eq!(dst_from_src(seed, 50, 98, 2), None);
    assert_eq!(dst_from_src(seed, 52, 50, 48), Some(57));
    let seed = 13;
    assert_eq!(dst_from_src(seed, 50, 98, 2), None);
    assert_eq!(dst_from_src(seed, 52, 50, 48), None);

    let seed = 81;
    assert_eq!(dst_from_src(seed, 0, 15, 37), None);
    assert_eq!(dst_from_src(seed, 37, 52, 2), None);
    assert_eq!(dst_from_src(seed, 39, 0, 15), None);
}

fn parse_seeds(line: &str) -> Vec<u64> {
    let numbers = line // "seeds: 79 14 55 13"
        .split(':') // ["seeds", " 79 14 55 13"]
        .skip(1) // [" 79 14 55 13"]
        .next() // " 79 14 55 13"
        .unwrap()
        .split_whitespace(); // ["79", "14", "55", "13"]

    numbers.fold(vec![], |mut acc, n| {
        acc.push(n.parse().unwrap());
        acc
    })
}

// dst, src, range
#[derive(Debug)]
struct Map(u64, u64, u64);

fn parse_map(line: &str) -> Map {
    let mut numbers = line // "0 15 37"
        .split_whitespace(); // ["0", "15", "37"]

    let one = numbers.next().unwrap();
    let two = numbers.next().unwrap();
    let three = numbers.next().unwrap();

    Map(one.parse().unwrap(), two.parse().unwrap(), three.parse().unwrap())
}

fn main() {
    // let lines = read_lines("./easy_input_part_one").unwrap(); // 35
    let lines = read_lines("./input").unwrap(); // 177942185
    let mut seeds = vec![];
    let mut all_maps = vec![vec![]];
    let mut parsing_map = false;
    let map_strs = [
        "seed-to-soil map:",
        "soil-to-fertilizer map:",
        "fertilizer-to-water map:",
        "water-to-light map:",
        "light-to-temperature map:",
        "temperature-to-humidity map:",
        "humidity-to-location map:",
    ];

    for line in lines {
        let line = line.unwrap();
        if line.starts_with("seeds:") {
            seeds = parse_seeds(&line);
        }

        if map_strs.iter().any(|s| line.starts_with(s)) {
            parsing_map = true;
            continue;
        }

        if parsing_map {
            // this is beautiful, I don't need to
            // replicate this logic outside of the loop
            if line.is_empty() {
                parsing_map = false;
                all_maps.push(vec![]);
                continue;
            }

            if let Some(m) = all_maps.last_mut() {
                m.push(parse_map(&line));
            }
        }
    }

    for maps in all_maps {
        for seed in &mut seeds {
            for Map(dst, src, range) in &maps {
                if let Some(d) = dst_from_src(*seed, *dst, *src, *range) {
                    *seed = d;
                    break;
                }
            }
        }
    }

    println!("part one {}", seeds.iter().min().unwrap());
}
