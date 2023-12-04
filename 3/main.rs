use std::fs::File;
use std::io::{self, BufRead};

fn is_digit(ch: u8) -> bool {
    ch >= 48 && ch <= 57
}

fn is_symbol(ch: u8) -> bool {
    !is_digit(ch) && ch != b'.'
}

fn is_asterisk(ch: u8) -> bool {
    ch == b'*'
}

fn is_dot(ch: u8) -> bool {
    ch == b'.'
}

type Pos = (usize, usize);

#[derive(Copy, Clone)]
struct Number {
    value: u32,
    // pos: Pos,
    start: usize,
    end: usize,
    digits: usize,
}

#[derive(Copy, Clone, Debug)]
struct Symbol {
    value: u8, // char
    col: usize,
    // pos: Pos,
}

impl Symbol {
    fn is_valid_part_number(&self, n: Number) -> bool {
        // for i in self.pos.1 - 1..self.pos.1 + 2 {
        //     for j in n.pos.1 - n.digits..n.pos.1 {
        //         if i == j {
        //             return true;
        //         }
        //     }
        // }
        for i in self.col - 1..self.col + 2 {
            for j in n.start..n.end {
                if i == j {
                    return true;
                }
            }
        }
        false
    }
}

#[derive(Debug)]
struct Part {
    symbol: Symbol,
    numbers: Vec<u32>,
}

fn main() {
    // let filename = "./easy_input_part_one"; // 4361, 467835
    let filename = "./input"; // 520_019, 44_997_877 (too low), 67_869_269 (too low), 75477702 (still wrong ;-;), 75519888 (FINALLY CORRECT)
    let file = File::open(filename).unwrap();
    let curr = io::BufReader::new(&file).lines();

    // max number of digits = 3
    let mut str_n = String::with_capacity(3);
    // i, j (j == last digit of number)
    // let mut n_pos: Pos = (0, 0);
    let mut n: u32;

    let mut n_matrix = vec![];
    let mut s_matrix = vec![];

    for (_i, line) in curr.enumerate() {
        let line = line.unwrap();
        let mut numbers = vec![];
        let mut symbols = vec![];
        let mut start = 0;
        let mut first_digit = true;
        for (j, cell) in line.bytes().enumerate() {
            if is_digit(cell) {
                if first_digit {
                    first_digit = false;
                    start = j;
                }
                str_n.push(cell as char);
                // n_pos = (i, j);
                continue;
            } else if !str_n.is_empty() {
                n = str_n.parse().unwrap();
                let n_digits = str_n.len();
                // numbers.push(Number { value: n, pos: n_pos, digits: n_digits });
                numbers.push(Number { value: n, start, end: j, digits: n_digits });
                str_n = "".to_owned();
                first_digit = true;
            }

            if !is_dot(cell) {
                // symbols.push(Symbol { value: cell, pos: (i, j) });
                symbols.push(Symbol { value: cell, col: j });
            }
        }

        if !str_n.is_empty() {
            n = str_n.parse().unwrap();
            let n_digits = str_n.len();
            // numbers.push(Number { value: n, pos: n_pos, digits: n_digits });
            numbers.push(Number { value: n, start, end: line.bytes().len(), digits: n_digits });
            str_n = "".to_owned();
        }

        n_matrix.push(numbers);
        s_matrix.push(symbols);
    }

    let mut parts = vec![];

    for (i, symbols) in s_matrix.iter().enumerate() {
        let start = if i == 0 { 0 } else { i - 1 };
        let end = if i == s_matrix.len() - 1 { i + 1 } else { i + 2 };
        for symbol in symbols {
            let mut valid_numbers = vec![];
            for j in start..end {
                for number in &n_matrix[j] {
                    if symbol.is_valid_part_number(*number) {
                        valid_numbers.push(number.value);
                    }
                }
            }
            parts.push(Part { symbol: *symbol, numbers: valid_numbers });
        }
    }

    // dbg!(&numbers);

    let mut total: u32 = 0;
    for part in &parts {
        for n in &part.numbers {
            total += *n;
        }
    }

    println!("part one {total}");

    let mut total2: u32 = 0;
    for part in &parts {
        if is_asterisk(part.symbol.value) && part.numbers.len() == 2 {
            total2 += part.numbers[0] * part.numbers[1];
        }
    }

    // dbg!(parts);

    println!("part two {total2}");
}
