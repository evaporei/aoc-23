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

type Pos = (usize, usize);

fn has_symbol_same_line(curr_line: &str, pos: Pos, n_digits: usize) -> bool {
    // prev
    if let Some(prev_pos) = pos.1.checked_sub(n_digits) {
        if let Some(ch) = curr_line.chars().nth(prev_pos) {
            if is_symbol(ch as u8) {
                return true;
            }
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
    for p in 0..n_digits + 2 {
        if let Some(prev_pos) = (pos.1 + p).checked_sub(n_digits) {
            if let Some(ch) = prev.chars().nth(prev_pos) {
                if is_symbol(ch as u8) {
                    return true;
                }
            }
        }
    }
    false
}

fn has_symbol_next_line(next: &Option<&String>, pos: Pos, n_digits: usize) -> bool {
    let next = match next {
        Some(next) => next,
        None => return false,
    };
    for p in 0..n_digits + 2 {
        if let Some(next_pos) = (pos.1 + p).checked_sub(n_digits) {
            if let Some(ch) = next.chars().nth(next_pos) {
                if is_symbol(ch as u8) {
                    return true;
                }
            }
        }
    }
    false
}

// it should be a list instead of one Pos
// prev:
// if len(3):
//   1 number, parse
// if len(2):
//   if together?
//     1 number, search and get n
//   else
//     2 numbers, search each and get their n's
// if len(1):
//   1 number, search and get n

fn has_digit_same_line(curr_line: &str, pos: Pos) -> Vec<(Pos, char)> {
    let mut l = vec![];
    // prev
    if let Some(prev_pos) = pos.1.checked_sub(1) {
        if let Some(ch) = curr_line.chars().nth(prev_pos) {
            if is_digit(ch as u8) {
                l.push(((pos.0, pos.1 - 1), ch));
            }
        }
    }
    // next
    if let Some(ch) = curr_line.chars().nth(pos.1 + 1) {
        if is_digit(ch as u8) {
            l.push(((pos.0, pos.1 + 1), ch));
        }
    }
    l
}

fn has_digit_prev_line(prev: &Option<String>, pos: Pos) -> Vec<(Pos, char)> {
    let mut l = vec![];
    let prev = match prev {
        Some(prev) => prev,
        None => return l,
    };
    for p in 0..1 + 2 {
        if let Some(prev_pos) = (pos.1 + (p)).checked_sub(1) {
            if let Some(ch) = prev.chars().nth(prev_pos) {
                if is_digit(ch as u8) {
                    l.push(((pos.0 - 1, pos.1 + p - 1), ch));
                }
            }
        }
    }
    l
}

fn has_digit_next_line(next: &Option<&String>, pos: Pos) -> Vec<(Pos, char)> {
    let mut l = vec![];
    let next = match next {
        Some(next) => next,
        None => return l,
    };
    for p in 0..1 + 2 {
        if let Some(next_pos) = (pos.1 + p).checked_sub(1) {
            if let Some(ch) = next.chars().nth(next_pos) {
                if is_digit(ch as u8) {
                    l.push(((pos.0 + 1, pos.1 + p - 1), ch));
                }
            }
        }
    }
    l
}

type Data = (Pos, u16, String);

#[derive(Debug)]
struct Numbers {
    data: Vec<Data>,
}

impl Numbers {
    fn new() -> Self {
        Self { data: Vec::new() }
    }

    fn insert(&mut self, pos: Pos, n: u16, n_str: String) {
        self.data.push((pos, n, n_str));
    }

    fn get(&self, may_pos: Pos, ch: char) -> Option<u16> {
        self.data.iter().find(|(pos, _, n_str)| {
            // should be in the same line
            if pos.0 != may_pos.0 {
                return false;
            }

            if let Some(res) = pos.1.checked_sub(may_pos.1) {
                // try to check if character in str version
                // matches w/ the one in the position received
                let c = n_str.chars().rev().nth(res);
                return c == Some(ch)
            }

            false
        })
        .map(|(_, n, _)| n)
        .copied()
    }
}

fn main() {
    // let filename = "./easy_input_part_one"; // 4361, 467835
    let filename = "./input"; // 520_019, 44_997_877 (too low), 67_869_269 (too low?)
    let file = File::open(filename).unwrap();
    let file2 = File::open(filename).unwrap();
    let curr = io::BufReader::new(&file).lines();
    let mut next_it = io::BufReader::new(&file2).lines().skip(1);

    let mut prev = None;
    let mut next = next_it.next();

    // max number of digits = 3
    let mut str_n = String::with_capacity(3);
    // i, j (j == last digit of number)
    let mut n_pos: Pos = (0, 0);
    let mut n: u16;
    let mut total: u32 = 0;

    let mut numbers = Numbers::new();

    // .........
    // ...456...
    // .........
    // (1,5)
    // n_digits = 3
    // same line: (OK?!)
    // (1,5-3)->(1,2)
    // (1,5+1)->(1,6)
    // prev line: (OK?!)
    // for p in (0..n_digits+2):
    //   (1-1,5-3+p)->(0,2~6)
    // next line: (OK?!)
    // for p in (0..n_digits+2):
    //   (1+1,5-3+p)->(2,2~6)

    // part one
    for (i, line) in curr.enumerate() {
        let line = line.unwrap();
        for (j, cell) in line.bytes().enumerate() {
            if is_digit(cell) {
                str_n.push(cell as char);
                n_pos = (i, j);
            } else if !str_n.is_empty() {
                n = str_n.parse().unwrap();
                let n_digits = str_n.len();
                // println!("({},{}) {}", n_pos.0, n_pos.1, n);
                // println!("curr {}", has_symbol_same_line(&line, n_pos, n_digits));
                // println!("prev {}", has_symbol_prev_line(&prev, n_pos, n_digits));
                // println!("next {}", has_symbol_next_line(&next.as_ref().map(|n| n.as_ref().unwrap()), n_pos, n_digits));
                if has_symbol_same_line(&line, n_pos, n_digits) ||
                   has_symbol_prev_line(&prev, n_pos, n_digits) ||
                   has_symbol_next_line(&next.as_ref().map(|n| n.as_ref().unwrap()), n_pos, n_digits) {
                    numbers.insert(n_pos, n, str_n);
                    total += n as u32;
                }
                str_n = "".to_owned();
            }

        }
        prev = Some(line);
        next = next_it.next();
    }

    // dbg!(&numbers);

    println!("part one {total}");

    // yeah this is ugly, 4 file descriptiors...
    let file3 = File::open(filename).unwrap();
    let file4 = File::open(filename).unwrap();
    let curr = io::BufReader::new(&file3).lines();
    let mut next_it = io::BufReader::new(&file4).lines().skip(1);

    let mut prev = None;
    let mut next = next_it.next();
    let mut total2: u32 = 0;

    // part two
    for (i, line) in curr.enumerate() {
        let line = line.unwrap();
        for (j, cell) in line.bytes().enumerate() {
            if is_asterisk(cell) {
                let ast_pos = dbg!((i, j));
                // println!("({},{})", ast_pos.0, ast_pos.1);
                let cur = has_digit_same_line(&line, ast_pos);
                let pre = has_digit_prev_line(&prev, ast_pos);
                let nxt = has_digit_next_line(&next.as_ref().map(|n| n.as_ref().unwrap()), ast_pos);
                println!("curr {:?}", cur);
                println!("prev {:?}", pre);
                println!("next {:?}", nxt);
                let mut ratios = vec![];
                if cur.len() == 2 {
                    println!("cur = 2");
                    // find
                    if let Some(found) = numbers.get(cur[0].0, cur[0].1) {
                        ratios.push(found as u32);
                    }
                    if let Some(found) = numbers.get(cur[1].0, cur[1].1) {
                        ratios.push(found as u32);
                    }
                } else if cur.len() == 1 {
                    println!("cur = 1");
                    if let Some(found) = numbers.get(cur[0].0, cur[0].1) {
                        ratios.push(found as u32);
                    }
                }

                // it's one number w/ three digits
                if pre.len() == 3 {
                    println!("pre = 3");
                    // we could directly parse here
                    if let Some(found) = numbers.get(pre[2].0, pre[2].1) {
                        ratios.push(found as u32);
                    }
                } else if pre.len() == 2 {
                    println!("pre = 2");
                    // together?
                    if pre[1].0.1 - pre[0].0.1 == 1 {
                        if let Some(found) = numbers.get(pre[1].0, pre[1].1) {
                            ratios.push(found as u32);
                        }
                    } else {
                        if let Some(found) = numbers.get(pre[0].0, pre[0].1) {
                            ratios.push(found as u32);
                        }
                        if let Some(found) = numbers.get(pre[1].0, pre[1].1) {
                            ratios.push(found as u32);
                        }
                    }
                } else if pre.len() == 1 {
                    if let Some(found) = numbers.get(pre[0].0, pre[0].1) {
                        ratios.push(found as u32);
                    }
                }

                // it's one number w/ three digits
                if nxt.len() == 3 {
                    println!("nxt = 3");
                    // we could directly parse here
                    if let Some(found) = numbers.get(nxt[2].0, nxt[2].1) {
                        ratios.push(found as u32);
                    }
                } else if nxt.len() == 2 {
                    println!("nxt = 2");
                    // together?
                    if nxt[1].0.1 - nxt[0].0.1 == 1 {
                        if let Some(found) = numbers.get(nxt[1].0, nxt[1].1) {
                            ratios.push(found as u32);
                        }
                    } else {
                        if let Some(found) = numbers.get(nxt[0].0, nxt[0].1) {
                            ratios.push(found as u32);
                        }
                        if let Some(found) = numbers.get(nxt[1].0, nxt[1].1) {
                            ratios.push(found as u32);
                        }
                    }
                } else if nxt.len() == 1 {
                    println!("nxt = 1");
                    if let Some(found) = numbers.get(nxt[0].0, nxt[0].1) {
                        ratios.push(found as u32);
                    }
                }
                // at least two
                if (!cur.is_empty() && !pre.is_empty()) || (!cur.is_empty() && !nxt.is_empty()) || (!pre.is_empty() && !nxt.is_empty()) {
                    if ratios.len() == 2 {
                        let mut f = 1;
                        for ratio in ratios {
                            println!("f o i");
                            f *= ratio;
                        }
                        total2 += f;
                    }
                }
            }
        }
        prev = Some(line);
        next = next_it.next();
    }

    println!("part two {total2}");
}
