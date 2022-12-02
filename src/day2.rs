use std::fs::File;
use std::io::{BufRead, BufReader};

fn main () {
    let reader = BufReader::new(File::open("data/day2.txt").unwrap());

    let mut score = 0;
    let mut second_score = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let opponent = line.chars().nth(0).unwrap();
        let mine = line.chars().nth(2).unwrap();
        match opponent {
            'A' => match mine {
                'X' => { score += 4; second_score += 3},
                'Y' => { score += 8; second_score += 4},
                'Z' => { score += 3; second_score += 8},
                _ => unreachable!()
            }
            'B' => match mine {
                'X' => { score += 1; second_score += 1},
                'Y' => { score += 5; second_score += 5 },
                'Z' => { score += 9; second_score += 9 },
                _ => unreachable!()
            }
            'C' => match mine {
                'X' => { score += 7; second_score += 2},
                'Y' => { score += 2; second_score += 6 },
                'Z' => { score += 6; second_score += 7 },
                _ => unreachable!()
            }
            _ => unreachable!()
        }
    }
    println!("{score} {second_score}")
}