use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

const FILENAME: &str = "./simple_loop";
// const FILENAME: &str = "./complex_loop";
// const FILENAME: &str = "./input";

type Pos = (usize, usize);
type Map = Vec<Vec<u8>>;

fn main() {
    let lines = read_lines(FILENAME).unwrap();
    let map = collect_map(lines);

    let s = find_starting_position(&map);
    let connected_pipes = find_first_pipes(&map, s);

    println!("{s:?}");
    println!("{connected_pipes:?}");
}

fn collect_map(lines: io::Lines<io::BufReader<File>>) -> Map {
    let mut map = Vec::with_capacity(140);

    for line in lines {
        let line = line.unwrap();
        map.push(line.bytes().collect());
    }

    map
}

fn find_starting_position(map: &Map) -> Pos {
    for (i, line) in map.iter().enumerate() {
        for (j, ch) in line.iter().enumerate() {
            if *ch == b'S' {
                return (i, j);
            }
        }
    }

    unreachable!("there should always be a S in the map");
}

const LEFT_PIPES: [u8; 3] = [b'-', b'L', b'F'];
const RIGHT_PIPES: [u8; 3] = [b'-', b'J', b'7'];
const UP_PIPES: [u8; 3] = [b'|', b'F', b'7'];
const DOWN_PIPES: [u8; 3] = [b'|', b'L', b'J'];

// There could be four pipes around an S (left,right,up,down),
// two of them must be connected.
// ...
// .S.
// ...
//
// left_pos  = (s.0    , s.1 - 1)
// right_pos = (s.0    , s.1 + 1)
// up_pos    = (s.0 - 1, s.1    )
// down_pos  = (s.0 + 1, s.1    )
//
// this function is disgusting, this should be done with a Graph 😭
fn find_first_pipes(map: &Map, s: (usize, usize)) -> (Pos, Pos) {
    let mut pipes = Vec::with_capacity(2);

    // address out of bounds
    // TODO: refactor to min/max/clamp function
    let s_0_minus = if s.0 == 0 { s.0 } else { s.0 - 1 };
    let s_0_plus = if s.0 == map.len() - 1 { s.0 } else { s.0 + 1 };
    let s_1_minus = if s.1 == 0 { s.1 } else { s.1 - 1 };
    let s_1_plus = if s.1 == map.len() - 1 { s.1 } else { s.1 + 1 };

    let left_pos = (s.0, s_1_minus);
    let right_pos = (s.0, s_1_plus);
    let up_pos = (s_0_minus, s.1);
    let down_pos = (s_0_plus, s.1);

    if LEFT_PIPES.contains(&map[left_pos.0][left_pos.1]) {
        pipes.push(left_pos);
    }
    if RIGHT_PIPES.contains(&map[right_pos.0][right_pos.1]) {
        pipes.push(right_pos);
    }
    if UP_PIPES.contains(&map[up_pos.0][up_pos.1]) {
        pipes.push(up_pos);
    }
    if DOWN_PIPES.contains(&map[down_pos.0][down_pos.1]) {
        pipes.push(down_pos);
    }

    (pipes[0], pipes[1])
}
