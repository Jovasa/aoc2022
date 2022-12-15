use std::cmp::Ordering::Equal;
use std::collections::HashSet;
use itertools::Itertools;
use regex::Regex;

fn main() {
    // read input file
    let input = std::fs::read_to_string("data/day15.txt").unwrap();
    let pattern = Regex::new(r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)").unwrap();

    let mut row_2_000_000 = HashSet::new();

    let mut beacons = HashSet::new();

    let mut data = Vec::new();

    for line in input.lines() {
        let captures = pattern.captures(line).unwrap();
        let x = captures[1].parse::<i32>().unwrap();
        let y = captures[2].parse::<i32>().unwrap();
        let bx = captures[3].parse::<i32>().unwrap();
        let by = captures[4].parse::<i32>().unwrap();

        data.push((x, y, bx, by));

        let dist = (bx - x).abs() + (by - y).abs();

        if by == 2_000_000 {
            beacons.insert(bx);
        }

        let distance_from_2_000_000 = (2_000_000 - y).abs();

        let extra_distance = dist -distance_from_2_000_000;

        for i in (x-extra_distance)..=(x+extra_distance) {
            row_2_000_000.insert(i);
        }

    }
    println!("{}", row_2_000_000.len() - beacons.len());


    for row_n in 0..4_000_000 {
        let mut ranges = Vec::new();
        for (x, y, bx, by) in &data {
            let dist = (bx - x).abs() + (by - y).abs();
            let distance_from_row_n = (row_n - y).abs();
            let extra_distance = dist - distance_from_row_n;
            if extra_distance >= 0 {
                ranges.push((x - extra_distance, x + extra_distance));
            }
        }
        ranges.sort_by(|a, b| if a.0.cmp(&b.0) == Equal { a.1.cmp(&b.1) } else { a.0.cmp(&b.0) });

        let mut max_end = -1;
        for (f, s) in ranges.iter().tuple_windows() {
            let end = f.1.max(max_end);
            if end + 2 == s.0 && f.1 >= 0 && s.0 <= 4_000_000 {
                println!("{}", (f.1 + 1) as usize * 4_000_000 + row_n as usize, );
                return;
            }
            max_end = end;
        }
    }
}