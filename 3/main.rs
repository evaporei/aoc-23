use std::fs::File;
use std::io::{self, BufRead};

fn is_digit(ch: u8) -> bool {
    ch >= 49 && ch <= 57
}

fn is_symbol(ch: u8) -> bool {
    !is_digit(ch) && ch != b'.'
}

type Pos = (usize, usize);

fn has_symbol_same_line(curr_line: &str, pos: Pos, n_digits: usize) -> bool {
    // prev
    if let Some(prev_pos) = pos.1.checked_sub(n_digits) {
        if let Some(ch) = curr_line.chars().nth(prev_pos) {
            return is_symbol(ch as u8);
        }
    }
    // next
    if let Some(ch) = curr_line.chars().nth(pos.1 + 1) {
        return is_symbol(ch as u8);
    }
    false
}

fn has_symbol_prev_line(prev: &Option<String>, pos: Pos, n_digits: usize) -> bool {
    let prev = match prev {
        Some(prev) => prev,
        None => return false,
    };
    for p in 0..n_digits + 1 {
        if let Some(prev_pos) = pos.1.checked_sub(n_digits) {
            if let Some(ch) = prev.chars().nth(prev_pos + p) {
                if is_symbol(ch as u8) {
                    return true;
                }
            }
        }
    }
    false
}

fn main() {
    let filename = "./easy_input_part_one"; // 4361
    // let filename = "./input";
    let file = File::open(filename).unwrap();
    let curr = io::BufReader::new(&file).lines();

    let mut peek = curr.enumerate().peekable();
    let mut prev = None;
    let mut next = peek.peek();

    // max number of digits = 3
    let mut str_n = String::with_capacity(3);
    // i, j (j == last digit of number)
    let mut n_pos: Pos = (0, 0);
    let mut n: u16;
    let mut total: u32 = 0;

    // .........
    // ...456...
    // .........
    // (1,5)
    // n_digits = 3
    // same line: (OK?!)
    // (1,5-3)->(1,2)
    // (1,5+1)->(1,6)
    // prev line: (OK?!)
    // for p in (0..n_digits+1):
    //   (1-1,5-3+p)->(0,2~6)
    // next line:
    // for p in (0..n_digits+1):
    //   (1+1,5-3+p)->(2,2~6)

    while let Some((i, line)) = peek.next() {
        let line = line.unwrap();
        for (j, cell) in line.bytes().enumerate() {
            if is_digit(cell) {
                str_n.push(cell as char);
                n_pos = (i, j);
            } else if !str_n.is_empty() {
                n = str_n.parse().unwrap();
                let n_digits = str_n.len();
                str_n = "".to_owned();
                println!("({},{}) {}", n_pos.0, n_pos.1, n);
                println!("prev {}", has_symbol_prev_line(&prev, n_pos, n_digits));
                if has_symbol_same_line(&line, n_pos, n_digits) ||
                   has_symbol_prev_line(&prev, n_pos, n_digits) {
                    total += n as u32;
                }
            }
        }
        prev = Some(line);
        next = peek.peek();
    }

    println!("{total}");
}
