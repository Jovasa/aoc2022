use std::io::BufRead;

fn main() {
    let input = std::fs::read_to_string("data/day14.txt").unwrap();

    let width = 1000;
    let height = 200;

    let mut grid = vec![vec![false; width]; height];

    let mut max_height = 0;

    for line in input.lines() {
        let parts = line.split(" -> ").collect::<Vec<&str>>();

        for a in parts.windows(2) {

            let start = a[0].split(",").map(|x| x.parse::<usize>().unwrap()).collect::<Vec<usize>>();
            let end = a[1].split(",").map(|x| x.parse::<usize>().unwrap()).collect::<Vec<usize>>();
            if start[0] == end[0] {
                for y in start[1].min(end[1])..=end[1].max(start[1]) {
                    grid[y][start[0]] = true;
                }
            }
            else {
                for x in start[0].min(end[0])..=end[0].max(start[0]) {
                    grid[start[1]][x] = true;
                }
            }
            max_height = max_height.max(start[1]).max(end[1]);
        }
    }

    // False for first part
    if true {
        for x in 0..width {
            grid[max_height + 2][x] = true;
        }
    }

    for i in 0.. {
        let mut location = (500, 0);

        let mut stopped = false;
        while location.1 < height - 1 {
            if !grid[location.1 + 1][location.0] {
                location.1 += 1;
            }
            else if !grid[location.1 + 1][location.0 - 1] {
                location.0 -= 1;
                location.1 += 1;
            }
            else if !grid[location.1 + 1][location.0 + 1] {
                location.0 += 1;
                location.1 += 1;
            }
            else {
                grid[location.1][location.0] = true;
                stopped = true;
                break;
            }
        }
        if !stopped || location.1 == 0 {
            // For second part the answer is one too little
            println!("{}", i);
            break;
        }
    }
}