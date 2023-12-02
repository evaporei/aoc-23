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
            let line_parser = LineParser::new(line.unwrap());
            self.sum += line_parser.parse();
        }

        self.sum
    }
}

fn is_digit(ch: u8) -> bool {
    ch >= 49 && ch <= 57
}

#[derive(Copy, Clone, Debug)]
enum Digit { One, Two, Three, Four, Five, Six, Seven, Eight, Nine }

impl Digit {
    fn from(n: u8) -> Self {
        match n - 48 {
            1 => Self::One,
            2 => Self::Two,
            3 => Self::Three,
            4 => Self::Four,
            5 => Self::Five,
            6 => Self::Six,
            7 => Self::Seven,
            8 => Self::Eight,
            9 => Self::Nine,
            _ => panic!("this ain't no digit pal"),
        }
    }

    fn to_char(self) -> char {
        match self {
            Self::One => '1',
            Self::Two => '2',
            Self::Three => '3',
            Self::Four => '4',
            Self::Five => '5',
            Self::Six => '6',
            Self::Seven => '7',
            Self::Eight => '8',
            Self::Nine => '9',
        }
    }
}

#[derive(Debug)]
struct DigitList(Vec<Digit>);

impl DigitList {
    fn to_n(self) -> usize {
        let mut it = self.0.iter();
        let first_digit = it.next().unwrap().to_char();
        let last_digit = it.next_back().unwrap().to_char();

        let mut s = String::with_capacity(self.0.len());
        s.push(first_digit);
        s.push(last_digit);
        s.parse().unwrap()
    }
}

struct LineParser {
    line: String,
}

impl LineParser {
    fn new(line: String) -> Self {
        Self {
            line,
        }
    }

    fn parse(self) -> usize {
        let mut digits = DigitList(vec![]);
        for ch in self.line.bytes() {
            if is_digit(ch) {
                digits.0.push(Digit::from(ch));
            }
        }

        if digits.0.len() == 1 {
            digits.0.push(digits.0[0]);
        }

        digits.to_n()
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
