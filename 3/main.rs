use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    let lines = read_lines("./easy_input_part_one").unwrap(); // 4361
    // let lines = read_lines("./input").unwrap();

    for line in lines {
        let line = line.unwrap();
        for cell in line.bytes() {
            print!("{}", cell as char);
        }
        println!();
    }
}
