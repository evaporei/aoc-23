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

impl From<u8> for Digit {
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
}

impl From<&Digit> for char {
    fn from(digit: &Digit) -> Self {
        match digit {
            Digit::One => '1',
            Digit::Two => '2',
            Digit::Three => '3',
            Digit::Four => '4',
            Digit::Five => '5',
            Digit::Six => '6',
            Digit::Seven => '7',
            Digit::Eight => '8',
            Digit::Nine => '9',
        }
    }
}

// can't implement From<&str> for Option<Digit>
// `str` is not defined in the current crate
impl Digit {
    fn from_written(s: &str) -> Option<Self> {
        if s.starts_with("one") {
            Some(Self::One)
        } else if s.starts_with("two") {
            Some(Self::Two)
        } else if s.starts_with("three") {
            Some(Self::Three)
        } else if s.starts_with("four") {
            Some(Self::Four)
        } else if s.starts_with("five") {
            Some(Self::Five)
        } else if s.starts_with("six") {
            Some(Self::Six)
        } else if s.starts_with("seven") {
            Some(Self::Seven)
        } else if s.starts_with("eight") {
            Some(Self::Eight)
        } else if s.starts_with("nine") {
            Some(Self::Nine)
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct DigitList(Vec<Digit>);

impl From<DigitList> for usize {
    fn from(digits: DigitList) -> usize {
        let mut it = digits.iter();
        let first_digit = it.next().unwrap().into();
        let last_digit = it.next_back().unwrap().into();

        let mut s = String::with_capacity(digits.len());
        s.push(first_digit);
        s.push(last_digit);
        s.parse().unwrap()
    }
}

impl std::ops::Deref for DigitList {
    type Target = Vec<Digit>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for DigitList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
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
        for (i, ch) in self.line.bytes().enumerate() {
            if is_digit(ch) {
                digits.push(Digit::from(ch));
            } else if let Some(digit) = Digit::from_written(&self.line[i..]) {
                digits.push(digit);
            }
        }

        if digits.len() == 1 {
            let first = digits[0];
            digits.push(first);
        }

        digits.into()
    }
}

fn main() {
    let parser = Parser::new();
    // let lines = read_lines("./easy_input_part_one").unwrap(); // 142
    // let lines = read_lines("./easy_input_part_two").unwrap(); // 281
    let lines = read_lines("./input").unwrap(); // 55218
    let sum = parser.parse(lines);
    println!("{sum}");
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
