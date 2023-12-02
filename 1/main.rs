use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct Parser {
    lines: io::Lines<io::BufReader<File>>,
    sum: usize,
}

impl Parser {
    fn new(lines: io::Lines<io::BufReader<File>>) -> Self {
        Self {
            lines,
            sum: 0,
        }
    }

    fn parse(mut self) -> usize {
        for line in self.lines {
            let line = line.unwrap();
            let mut n = 0;
            let mut first = true;
            let mut last = 0;
            for ch in line.bytes() {
                if ch >= 49 && ch <= 57 {
                    if first {
                        n += (ch - 48) * 10;
                        first = false;
                    }

                    last = ch - 48;
                }
            }
            n += last;
            self.sum += n as usize;
        }

        self.sum
    }
}

fn main() {
    let lines = read_lines("./easy_input_part_one").unwrap();
    let parser = Parser::new(lines);
    let sum = parser.parse();
    println!("{sum}");
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
