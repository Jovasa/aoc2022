use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let reader = BufReader::new(File::open("data/day1.txt").unwrap());

    let mut data = Vec::new();
    let mut total = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        match line.parse::<u32>() {
            Ok(t) => total += t,
            Err(_) => {
                data.push(total);
                total = 0;
            }
        }
    }
    data.push(total);
    data.sort();
    println!("{}", data.iter().rev().next().unwrap());
    println!("{}", data.iter().rev().take(3).fold(0, |x, o| x + o))
}
