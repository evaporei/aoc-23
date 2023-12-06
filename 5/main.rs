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

    let dst = numbers.next().unwrap();
    let src = numbers.next().unwrap();
    let range = numbers.next().unwrap();

    Map(dst.parse().unwrap(), src.parse().unwrap(), range.parse().unwrap())
}

fn locations(mut seeds: Vec<u64>) {
    // let lines = read_lines("./easy_input_part_one").unwrap(); // 35
    let lines = read_lines("./input").unwrap(); // 177942185, 69841803
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

    // parsing
    for line in lines {
        let line = line.unwrap();

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

    // calculation
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

    // to get the `i`s
    // println!("{:?}", seeds.iter().enumerate().min_by(|(_, x), (_, y)| x.cmp(y)).unwrap());
    println!("{:?}", seeds.iter().min().unwrap());
}

fn main() {
    let lines = read_lines("./input").unwrap();
    let mut seeds = vec![];
    for line in lines {
        let line = line.unwrap();
        if line.starts_with("seeds:") {
            seeds = parse_seeds(&line);
        }
    }
    // part one
    locations(seeds);

    // part two
    // (i, location)

    // fun fact: if you uncomment them all, it takes 3 min on a 11 gen thinkpad

    // (180_368_864,   317_412_499)
    // locations((279_234_546..=279_234_546 + 382_175_449).collect::<Vec<u64>>());
    // // (  2_795_998,   120_063_312)
    // locations((689_152_391..=689_152_391 + 244_427_042).collect::<Vec<u64>>());
    // // (          0, 3_247_724_559)
    // locations((1_105_311_711..=1_105_311_711 + 2_036_236).collect::<Vec<u64>>());
    // // ( 40_919_163,   384_367_353)
    // locations((1_450_749_684..=1_450_749_684 + 123_906_789).collect::<Vec<u64>>());
    // // ( 59_963_967, 1_765_298_118)
    // locations((1_609_835_129..=1_609_835_129 + 60_050_954).collect::<Vec<u64>>());

    // (454_652_699,    69_841_803) -> WINNER
    locations((2_044_765_513..=2_044_765_513 + 620_379_445).collect::<Vec<u64>>());
    // // (          0, 3_117_212_723)
    // locations((2_906_422_699..=2_906_422_699 + 6_916_147).collect::<Vec<u64>>());
    // // (          0,   210_388_587)
    // locations((3_075_226_163..=3_075_226_163 + 146_720_986).collect::<Vec<u64>>());
    // // (101_953_205,   455_176_213)
    // locations((3_650_753_915..=3_650_753_915 + 127_044_950).collect::<Vec<u64>>());
    // // ( 32_942_963,   291_248_263)
    // locations((3_994_686_181..3_994_686_181 + 93_904_335).collect::<Vec<u64>>());
}
