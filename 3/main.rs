use std::fs::File;
use std::io::{self, BufRead};

fn is_digit(ch: u8) -> bool {
    ch >= 49 && ch <= 57
}

fn main() {
    let filename = "./easy_input_part_one"; // 4361
    // let filename = "./input";
    let file = File::open(filename).unwrap();
    let curr = io::BufReader::new(&file).lines();
    let mut prev = io::BufReader::new(&file).lines().into_iter();
    let mut next = io::BufReader::new(&file).lines().into_iter().skip(1);

    let mut str_n = String::with_capacity(3);
    // i, j (j == last digit of number)
    let mut n_pos: (usize, usize) = (0, 0);
    let mut n: u16;

    for (i, line) in curr.enumerate() {
        let line = line.unwrap();
        for (j, cell) in line.bytes().enumerate() {
            if is_digit(cell) {
                str_n.push(cell as char);
                n_pos = (i, j);
            } else if !str_n.is_empty() {
                n = str_n.parse().unwrap();
                str_n = "".to_owned();
                println!("({},{}) {}", n_pos.0, n_pos.1, n);
            }
        }
        prev.next();
        next.next();
    }
}
