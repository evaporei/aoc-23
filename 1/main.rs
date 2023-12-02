use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct Parser {
    sum: usize,
}

impl Parser {
    fn new() -> Self {
        Self {
            sum: 0,
        }
    }

    fn parse(mut self, lines: io::Lines<io::BufReader<File>>) -> usize {
        for line in lines {
            self.sum += self.parse_line(&line.unwrap());
        }

        self.sum
    }

    fn parse_line(&self, line: &str) -> usize {
        let mut n = 0;
        let mut first = true;
        let mut last = 0;
        for ch in line.bytes() {
            if self.is_digit(ch) {
                if first {
                    n += (ch - 48) * 10;
                    first = false;
                }

                last = ch - 48;
            }
        }
        n += last;
        n as usize
    }

    fn is_digit(&self, ch: u8) -> bool {
        ch >= 49 && ch <= 57
    }
}

fn main() {
    let parser = Parser::new();
    let lines = read_lines("./easy_input_part_one").unwrap();
    let sum = parser.parse(lines);
    println!("{sum}");
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
