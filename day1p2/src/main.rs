use std::io::{BufRead, BufReader};
use std::fs::File;

fn main() {
    let reader = BufReader::new(File::open("input.txt").expect("Cannot open input.txt"));

    let mut numbers: Vec<u32> = Vec::new();
    let mut increments = 0;
    let mut index = 0;

    for line in reader.lines() {
        numbers.push(line.unwrap().parse::<u32>().unwrap());

        if index < 3 {
            index += 1;
            continue;
        }

        let a = numbers[index - 3] + numbers[index - 2] + numbers[index - 1];
        let b = numbers[index - 2] + numbers[index - 1] + numbers[index];

        if b > a {
            increments += 1;
        }

        index += 1;
    }

    println!("Number of increments: {}", increments);

    assert_eq!(increments, 1252);
}
