use std::fs::File;
use std::io::{self, BufRead, Read};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

// const FILENAME: &str = "./simple_loop"; // 4
// const FILENAME: &str = "./complex_loop"; // 8
// const FILENAME: &str = "./area_input1";
// const FILENAME: &str = "./area_input2";
// const FILENAME: &str = "./area_input3";
const FILENAME: &str = "./input"; // 6860, 593 (too high), 343 (works, but I need to learn another
                                  // strategy other than split into quadrants

// natural, obviously
type Pos = (usize, usize);
type IntPos = (i32, i32);
type Map = Vec<Vec<u8>>;

fn nat_to_int(pos: Pos) -> IntPos {
    (pos.0 as i32, pos.1 as i32)
}

fn main() {
    let lines = read_lines(FILENAME).unwrap();
    let map = collect_map(lines);

    let s = find_starting_position(&map);
    let connected_pipes = find_first_pipes(&map, s);

    let mut loop_pipes = vec![];

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

        loop_pipes.push(nat_to_int(prev1));
        loop_pipes.push(nat_to_int(prev2));
    }

    println!("part one: {steps}");

    // push furthest position
    loop_pipes.push(nat_to_int(cursor1));

    // let mut inside_loop = 0;
    // // dbg!(&loop_pipes);
    // for (i, line) in map.iter().enumerate() {
    //     for (j, _ch) in line.iter().enumerate() {
    //         if is_inside_loop(&loop_pipes, nat_to_int((i, j))) {
    //             inside_loop += 1;
    //         }
    //     }
    // }

    // println!("part two: {inside_loop}");

    // let inside_loop = calculate_area(&loop_pipes);

    // let mut viz = file::open("./viz").unwrap();
    // let mut viz = String::with_capacity(140 * 140);
    // for (i, line) in map.iter().enumerate() {
    //     for (j, _ch) in line.iter().enumerate() {
    //         if loop_pipes.contains(&(i, j)) {
    //             // viz.write(b"V").unwrap();
    //             viz.push('V');
    //         } else {
    //             // viz.write(b".").unwrap();
    //             viz.push('.');
    //         }
    //     }
    //     // viz.write(b"\n").unwrap();
    //     viz.push('\n');
    // }
    //
    let mut viz = File::open("./quadrant").unwrap(); // 349, 345, 343
    // // let mut viz = File::open("./final_viz").unwrap();
    let mut contents = String::new();
    viz.read_to_string(&mut contents).unwrap();
    println!("part two: {}", contents.bytes().filter(|ch| *ch == b'.').count()); // 343
    // println!("{viz}");

    // type pipe = | or - or J or L or F or 7 # 6
    // type tile = S or . or pipe # 3
    // total (grid) area 140x140 = 19_600 -> n of pipes
    //
    // shape area = inside perimeter
    // result = total - perimeter - shape (inside)


    // .......
    // .VVVV..
    // .V..VV.
    // .VVVVV.

    // println!("part two: {inside_loop}");
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

// fn min(x: i32, y: i32) -> i32 {
//     if x < y { x } else { y }
// }
//
// fn max(x: i32, y: i32) -> i32 {
//     if x > y { x } else { y }
// }
//
// 8261
// fn is_inside_loop(perimeter: &Vec<IntPos>, point: IntPos) -> bool {
//     let len = perimeter.len();
//     let mut inside = false;
//     let (x, y) = point;
//     let (p1x, p1y) = perimeter[0];
//     for i in 0..len {
//         let (p2x, p2y) = perimeter[i % len];
//         if y > min(p1y, p2y) {
//             if y <= max(p1y, p2y) {
//                 if x <= max(p1x, p2x) {
//                     let mut xinters = None;
//                     if p1y != p2y {
//                         xinters = Some((y - p1y) * (p2x - p1x)
//                             / (p2y - p1y) + p1x);
//                     }
//                     if p1x == p2x || Some(x) <= xinters {
//                         inside = !inside;
//                     }
//                 }
//             }
//         }
//     }
//
//     inside
// }

// even odd algorithm, not working
// 5019
fn is_inside_loop(perimeter: &Vec<IntPos>, point: IntPos) -> bool {
    let mut inside = false;
    let mut j = perimeter.len() - 1;
    let (x, y) = point;
    for i in 0..perimeter.len() {
        if x == perimeter[i].0 && y == perimeter[i].1 {
            // point is in the loop line
            return false;
        }
        if (perimeter[i].1 > y) != (perimeter[j].1 > y) {
            let slope = (x - perimeter[i].0)
                * (perimeter[j].1 - perimeter[i].1)
                - (perimeter[j].0 - perimeter[i].0)
                * (y - perimeter[i].1); 
            if slope == 0 {
                // point is on boundary
                return false;
            }
            if (slope < 0) != (perimeter[j].1 < perimeter[i].1) {
                inside = !inside;
            }
        }
        j = i;
    }

    inside
}

// fn calculate_area(perimeter: &Vec<Pos>) -> usize {
//     0
// }
