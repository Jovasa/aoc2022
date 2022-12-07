use std::collections::{HashMap, VecDeque};
use itertools::Itertools;

fn main() {
    let input = std::fs::read_to_string("data/day7.txt").unwrap();
    let data = input.split("\n");

    let mut hierarchy = VecDeque::new();
    let mut sizes: HashMap<String, usize> = HashMap::new();
    for line in data {
        if line.len() == 0 {
            continue;
        }
        let parts = line.split(" ").collect::<Vec<&str>>();
        if parts[0] == "$" {
            if parts[1] == "cd" {
                if parts[2] == ".." {
                    hierarchy.pop_back();
                } else {
                    hierarchy.push_back(parts[2]);
                }
            }
        } else if parts[0] == "dir" {
            continue;
        } else {
            let size = parts[0].parse::<usize>().unwrap();
            for (depth, dir) in hierarchy.iter().enumerate() {
                let a = hierarchy.iter().take(depth + 1).join("/");
                let entry = sizes.entry(a).or_insert(0);
                *entry += size;
            }
        }
    }
    let mut total_size = 0;
    for (_key, value) in &sizes {
        if *value <= 100000 { total_size += value; }
    }
    println!("{}", total_size);

    let require_size = 30000000 - (70000000 - *sizes.entry("/".to_string()).or_insert(0));

    let mut smallest_available = usize::MAX;
    for (key, value) in &sizes {
        if *value >= require_size && *value < smallest_available {
            smallest_available = *value;
        }
    }
    println!("{}", smallest_available);
}