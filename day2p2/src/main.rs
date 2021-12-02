use std::io::{BufRead, BufReader};
use std::fs::File;

fn main() {
    let reader = BufReader::new(File::open("input.txt").expect("Cannot open input.txt"));

    let mut aim: u32 = 0;
    let mut depth: u32 = 0;
    let mut horizontal: u32 = 0;

    for line in reader.lines() {
        let unwrapped = line.unwrap();
        let current: Vec<&str> = unwrapped.split(" ").collect();
        let value = current[1].parse::<u32>().unwrap();

        match current[0] {
            "up" => aim -= value,
            "down" => aim += value,
            "forward" => {
                horizontal += value;
                depth += aim * value
            }
            _ => {}
        }
    }

    println!("Result: {}", depth * horizontal);
}
