use std::io::{BufRead, BufReader};
use std::fs::File;

fn main() {
    let reader = BufReader::new(File::open("input.txt").expect("Cannot open input.txt"));
    let mut increments:i32 = -1; // ignore the "first" increment
    let mut previous:i32 = 0;

    for line in reader.lines() {
        let number = line.unwrap().parse::<i32>().unwrap();

        if number > previous {
            increments += 1;
        }

        previous = number;
    }

    println!("Number of increments: {}", increments);
}
