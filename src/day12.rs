fn main() {
    let input = std::fs::read_to_string("data/day12.txt").unwrap();

    let mut height_map: Vec<Vec<u8>> = Vec::new();

    let mut start = (0, 0);
    let mut end = (0, 0);

    for (y, line) in input.lines().enumerate() {
        height_map.push(line.chars().enumerate().map(|(x, c)|
            if c.is_ascii_lowercase() {
                c as u8 - 'a' as u8
            }
            else {
                if c == 'S' {
                    start = (x, y);
                    0
                }
                else {
                    end = (x, y);
                    'z' as u8 - 'a' as u8
                }
            }
        ).collect());
    }

    let mut visited = vec![false; height_map.len() * height_map[0].len()];
    let mut queue: Vec<(usize, usize)> = Vec::new();
    queue.push(end);
    visited[end.1 * height_map[0].len() + end.0] = true;
    let mut steps = 0;

    while !queue.is_empty() {
        let mut new_queue = Vec::new();
        for (x, y) in queue {
            // Remove second condition for part 1
            if x == start.0 && y == start.1 || height_map[y][x] == 0 {
                println!("{}", steps);
                return;
            }
            if x != 0 {
                let left = (x - 1, y);
                check_and_mark(&mut visited, &mut new_queue, &height_map, left, (x, y))
            }
            if x != height_map[0].len() - 1 {
                let right = (x + 1, y);
                check_and_mark(&mut visited, &mut new_queue, &height_map, right, (x, y))
            }
            if y != 0 {
                let up = (x, y - 1);
                check_and_mark(&mut visited, &mut new_queue, &height_map, up, (x, y))
            }
            if y != height_map.len() - 1 {
                let down = (x, y + 1);
                check_and_mark(&mut visited, &mut new_queue, &height_map, down, (x, y))
            }
        }
        queue = new_queue;
        steps += 1;
    }
}

fn check_and_mark(visited: &mut Vec<bool>, queue: &mut Vec<(usize, usize)>, height_map: &Vec<Vec<u8>>, pos: (usize, usize), prev: (usize, usize)) {
    let index = pos.1 * height_map[0].len() + pos.0;
    if visited[index] == false {
        let height = height_map[pos.1][pos.0];
        let prev_height = height_map[prev.1][prev.0];
        if prev_height as i32 - height as i32 <= 1 {
            visited[index] = true;
            queue.push(pos);
        }
    }
}