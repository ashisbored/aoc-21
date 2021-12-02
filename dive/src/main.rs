use std::io::BufRead;

fn main() {
    let stdin = std::io::stdin();

    let mut horizontal = 0_i64;
    let mut depth = 0_i64;
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let (cmd, value) = line.split_once(" ").unwrap();
        match (cmd, value.parse::<i64>().unwrap()) {
            ("forward", value) => horizontal += value,
            ("down", value) => depth += value,
            ("up", value) => depth -= value,
            _ => panic!("uh oh"),
        }
    }

    println!("{}", horizontal * depth);
}
