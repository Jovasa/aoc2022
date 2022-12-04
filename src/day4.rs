use std::fs::File;
use std::io::{BufRead, BufReader};
use itertools::Itertools;

fn main() {
    let reader = BufReader::new(File::open("data/day4.txt").unwrap());

    let mut complete_overlaps = 0;
    let mut overlaps = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        let (f, s) = line.split(",").collect_tuple().unwrap();
        let (first_low, first_high) = f.split("-")
            .map(|x| x.parse::<u32>().unwrap())
            .collect_tuple().unwrap();
        let (second_low, second_high) = s.split("-")
            .map(|x| x.parse::<u32>().unwrap())
            .collect_tuple().unwrap();

        if (first_low >= second_low && first_high <= second_high)
            || (first_low <= second_low && first_high >= second_high) {
            complete_overlaps += 1;
        }
        if !(first_high < second_low || second_high < first_low) {
            overlaps += 1;
        }
    }
    println!("{complete_overlaps} {overlaps}");
}