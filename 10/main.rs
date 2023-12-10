use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

// const FILENAME: &str = "./simple_loop"; // 4
// const FILENAME: &str = "./complex_loop"; // 8
const FILENAME: &str = "./input"; // 6860

type Pos = (usize, usize);
type Map = Vec<Vec<u8>>;

fn main() {
    let lines = read_lines(FILENAME).unwrap();
    let map = collect_map(lines);

    let s = find_starting_position(&map);
    let connected_pipes = find_first_pipes(&map, s);

    let mut steps = 1;
    let mut cursor1 = connected_pipes.0;
    let mut cursor2 = connected_pipes.1;
    // both start at S
    let mut prev1 = s;
    let mut prev2 = s;
    // if they match, we found the end of the loop (furthest from S)
    while cursor1 != cursor2 {
        // save for prev
        let (tmp1, tmp2) = (cursor1, cursor2);

        cursor1 = find_next(cursor1, prev1, map[cursor1.0][cursor1.1]);
        cursor2 = find_next(cursor2, prev2, map[cursor2.0][cursor2.1]);

        (prev1, prev2) = (tmp1, tmp2);
        steps += 1;
    }

    println!("part one: {steps}");
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

#[derive(Clone, Copy, Debug)]
enum Dir { Left, Right, Up, Down }

impl Dir {
    // returns where previous is compared to current
    //
    // ...
    // .S-
    // ...
    //
    // in the case above, S is to the Left of -
    fn compare(curr: Pos, prev: Pos) -> Self {
        // same line
        if curr.0 == prev.0 {
            if curr.1 > prev.1 {
                Self::Left
            } else {
                Self::Right
            }
        } else {
            if curr.0 > prev.0 {
                Self::Up
            } else {
                Self::Down
            }
        }
    }
    fn opposite(self) -> Self {
        match self {
            Self::Left => Self::Right,
            Self::Right => Self::Left,
            Self::Up => Self::Down,
            Self::Down => Self::Up,
        }
    }
    // increment pos to the given direction
    fn new_pos(self, pos: Pos) -> Pos {
        match self {
            Self::Left => (pos.0, pos.1 - 1),
            Self::Right => (pos.0, pos.1 + 1),
            Self::Up => (pos.0 - 1, pos.1),
            Self::Down => (pos.0 + 1, pos.1),
        }
    }
}

// | is a vertical pipe connecting north and south.
// - is a horizontal pipe connecting east and west.
// L is a 90-degree bend connecting north and east.
// J is a 90-degree bend connecting north and west.
// 7 is a 90-degree bend connecting south and west.
// F is a 90-degree bend connecting south and east.
#[derive(Clone, Copy, Debug)]
enum Pipe { Vertical, Horizontal, L, J, Seven, F }

impl From<u8> for Pipe {
    fn from(ch: u8) -> Self {
        match ch {
            b'|' => Self::Vertical,
            b'-' => Self::Horizontal,
            b'L' => Self::L,
            b'J' => Self::J,
            b'7' => Self::Seven,
            b'F' => Self::F,
            _ => unreachable!("somehow we went to a non pipe position (. or S)"),
        }
    }
}

impl Pipe {
    // get the other side of the pipe
    fn next(self, prev: Dir) -> Dir {
        match (self, prev) {
            (Self::Vertical, dir) => dir.opposite(),
            (Self::Horizontal, dir) => dir.opposite(),

            (Self::L, Dir::Right) => Dir::Up,
            (Self::L, Dir::Up) => Dir::Right,

            (Self::J, Dir::Left) => Dir::Up,
            (Self::J, Dir::Up) => Dir::Left,

            (Self::Seven, Dir::Left) => Dir::Down,
            (Self::Seven, Dir::Down) => Dir::Left,

            (Self::F, Dir::Right) => Dir::Down,
            (Self::F, Dir::Down) => Dir::Right,

            _ => unreachable!("something is wrong {self:?}, {prev:?}"),
        }
    }
}

// prev here is used to find which side of the pipe should we go to
fn find_next(pos: Pos, prev: Pos, ch: u8) -> Pos {
    // we can't go this way (where we were at)
    let origin = Dir::compare(pos, prev);
    let pipe = Pipe::from(ch);
    let new_dir = pipe.next(origin);
    new_dir.new_pos(pos)
}
