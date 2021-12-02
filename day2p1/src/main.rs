use std::io::{BufRead, BufReader};
use std::fs::File;

fn main() {
    let reader = BufReader::new(File::open("input.txt").expect("Cannot open input.txt"));

    let mut depth: u32 = 0;
    let mut horizontal: u32 = 0;

    for line in reader.lines() {
        let unwrapped = line.unwrap();
        let current: Vec<&str> = unwrapped.split(" ").collect();
        let value = current[1].parse::<u32>().unwrap();

        match current[0] {
            "up" => depth -= value,
            "down" => depth += value,
            "forward" => horizontal += value,
            _ => {}
        }
    }

    println!("Result: {}", depth * horizontal);
}
