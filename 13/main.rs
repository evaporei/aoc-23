// . -> 0
// # -> 1
fn encode(s: &str) -> u32 {
    // lol easy
    let bin = s.replace('.', "0").replace('#', "1");
    u32::from_str_radix(&bin, 2).unwrap()
}

#[test]
fn test_encode() {
    assert_eq!(encode(".###.#."), 0b0111010);
}

fn main() {
}
