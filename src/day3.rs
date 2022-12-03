use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let reader = BufReader::new(File::open("data/day3.txt").unwrap());

    let mut score: usize = 0;
    let mut badge_score = 0;

    let mut badge_collection = HashSet::new();

    for (i, line) in reader.lines().enumerate() {
        let line = line.unwrap();

        if i % 3 == 0 {
            if i != 0 {
                let f = badge_collection.iter().next().unwrap();
                add_priority(&mut badge_score, f);
            }
            badge_collection = HashSet::from_iter(line.chars());
        }
        else {
            badge_collection = badge_collection.intersection(&HashSet::from_iter(line.chars())).map(|z| *z).collect::<HashSet<char>>();
        }

        let first_half: HashSet<char> = HashSet::from_iter(line.chars().take(line.len() / 2));
        let second_half:HashSet<char> = HashSet::from_iter(line.chars().skip(line.len() / 2));

        let a = first_half.intersection(&second_half).next().unwrap();

        add_priority(&mut score, a);
    }
    let f = badge_collection.iter().next().unwrap();
    add_priority(&mut badge_score, f);

    println!("{score} {badge_score}");
}

fn add_priority(score: &mut usize, a: &char) {
    if a.is_lowercase() {
        *score += (*a as u8 - 'a' as u8 + 1) as usize;
    } else {
        *score += (*a as u8 - 'A' as u8 + 27) as usize;
    }
}