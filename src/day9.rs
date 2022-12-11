use std::collections::HashSet;
use std::str::Split;

fn move_tail(tail: &mut(i32, i32), head: (i32, i32)) {
    let diff = (head.0 - tail.0, head.1 - tail.1);
    let total = diff.0.abs() + diff.1.abs();
    match total {
        0 => (),
        1 => (),
        2 => {
            tail.0 += diff.0 / 2;
            tail.1 += diff.1 / 2;
        },
        3 => {
            if diff.0.abs() == 1 {
                tail.0 += diff.0;
                tail.1 += diff.1 / 2;
            }
            else {
                tail.0 += diff.0 / 2;
                tail.1 += diff.1;
            }
        }
        4 => {
            tail.0 += diff.0 / 2;
            tail.1 += diff.1 / 2;
        }
        _ => unreachable!()
    }
}

fn main() {
    // Read input from file
    let input = std::fs::read_to_string("data/day9.txt").unwrap();
    let input_data = input.split("\n");

    first(input_data.clone(), 2);
    first(input_data, 10);
}

fn first(input_data: Split<&str>, length:usize) {
    let mut positions = vec![(0, 0); length];

    let mut visited = HashSet::new();

    for line in input_data {
        if line.len() == 0 { break; }
        let direction = line.chars().nth(0).unwrap();
        let distance = line[2..].parse::<i32>().unwrap();

        match direction {
            'U' => {
                for i in 0..distance {
                    positions[0].1 += 1;
                    do_tail(&mut positions, &mut visited);
                }
            },
            'D' => {
                for i in 0..distance {
                    positions[0].1 -= 1;
                    do_tail(&mut positions, &mut visited);
                }
            },
            'L' => {
                for i in 0..distance {
                    positions[0].0 -= 1;
                    do_tail(&mut positions, &mut visited);
                }
            },
            'R' => {
                for i in 0..distance {
                    positions[0].0 += 1;
                    do_tail(&mut positions, &mut visited);
                }
            },
            _ => panic!("Invalid direction"),
        }
    }
    println!("{}", visited.len());
}

fn do_tail(mut positions: &mut Vec<(i32, i32)>, visited: &mut HashSet<(i32, i32)>) {
    for j in 0..(positions.len() - 1) {
        let head = positions[j];
        move_tail(&mut positions[j + 1], head);
    }
    visited.insert(positions[positions.len() - 1]);
}