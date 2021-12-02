use std::io::{BufRead, BufReader};
use std::fs::File;

fn main() {
    let reader = BufReader::new(File::open("input.txt").expect("Cannot open input.txt"));

    let mut increments:u32 = 0;
    let mut previous:Result<u32, u32> = Err(0);

    for line in reader.lines() {
        let current = line.unwrap().parse::<u32>().unwrap();

        match previous {
            Ok(number) => if current > number { increments += 1 },
            Err(_) => ()
        }

        previous = Ok(current);
    }

    println!("Number of increments: {}", increments);

    assert_eq!(increments, 1226);
}
