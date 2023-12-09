use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    // let lines = read_lines("./example_part_one").unwrap(); // 114, 2
    let lines = read_lines("./input").unwrap(); // 1930746032, 1154
    let mut extrapolated_sum = 0;
    let mut extrapolated_backwards = 0;

    for line in lines {
        let line = line.unwrap();

        let numbers: Vec<i32> = line
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();

        let mut lists = diff(vec![], numbers);

        add_new_history(&mut lists);

        let new_history = lists[0][lists[0].len() - 1];
        extrapolated_sum += new_history;

        add_history_backwards(&mut lists);

        let backwards_history = lists[0][0];
        extrapolated_backwards += backwards_history;
    }

    println!("part one {extrapolated_sum}");
    println!("part two {extrapolated_backwards}");
}

fn diff(mut start: Vec<Vec<i32>>, numbers: Vec<i32>) -> Vec<Vec<i32>> {
    if numbers.iter().all(|n| *n == 0) {
        // we don't need to push 'numbers'
        // cause they won't be used
        // (all zeroes)
        return start;
    }

    let next: Vec<i32> = numbers
        .windows(2)
        .map(|ns| ns[1] - ns[0])
        .collect();

    start.push(numbers);

    diff(start, next)
}

// adds a new history to the first list (input)
fn add_new_history(lists: &mut Vec<Vec<i32>>) {
    // push a repeating 3 or 2 or 1
    // they're all the same in the last list
    let last_elem_idx = lists.len() - 1;
    let last_list = &mut lists[last_elem_idx];
    let mut curr_to_add = last_list[0];
    last_list.push(curr_to_add);

    let old_len = lists[0].len();

    let mut rev_i = lists.len() - 2;

    // we push to the first list, this loop ends
    while old_len == lists[0].len() {
        let last_idx = lists[rev_i].len() - 1;
        let curr_last_elem = lists[rev_i][last_idx];
        let sum = curr_last_elem + curr_to_add;
        curr_to_add = sum;
        lists[rev_i].push(sum);
        if rev_i == 0 {
            break;
        }
        rev_i -= 1;
    }
}

// adds a history backwards to the first list (input)
fn add_history_backwards(lists: &mut Vec<Vec<i32>>) {
    // insert a repeating 3 or 2 or 1
    // they're all the same in the last list
    let last_elem_idx = lists.len() - 1;
    let last_list = &mut lists[last_elem_idx];
    let mut curr_to_sub = last_list[0];
    last_list.insert(0, curr_to_sub);

    let old_len = lists[0].len();

    let mut rev_i = lists.len() - 2;

    // we push to the first list, this loop ends
    while old_len == lists[0].len() {
        let curr_first_elem = lists[rev_i][0];
        let sub = curr_first_elem - curr_to_sub;
        curr_to_sub = sub;
        lists[rev_i].insert(0, sub);
        if rev_i == 0 {
            break;
        }
        rev_i -= 1;
    }
}
