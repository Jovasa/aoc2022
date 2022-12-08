fn check_direction<'a, I, B>(outer: I, inner: B, seen: &mut Vec<Vec<bool>>, grid: &Vec<Vec<i8>>, column_major: bool)
    where I: Iterator<Item=usize>, B: Iterator<Item=usize> + Clone
{
    for y in outer {
        let mut highest_so_far = -1;
        for x in inner.clone() {
            let current = if column_major { grid[y][x] } else { grid[x][y] };
            if current > highest_so_far {
                if column_major {
                    seen[y][x] = true;
                } else {
                    seen[x][y] = true;
                }
            }
            highest_so_far = std::cmp::max(highest_so_far, current);
        }
    }
}

fn count_direction<'a, I, B>(outer: I, inner: B, grid: &Vec<Vec<i8>>, column_major: bool, start: i8) -> usize
    where I: Iterator<Item=usize>, B: Iterator<Item=usize> + Clone
{
    let mut count = 0;
    for y in outer {
        for x in inner.clone() {
            let current = if column_major { grid[y][x] } else { grid[x][y] };

            count += 1;

            if current >= start {
                return count;
            }
        }
    }
    count
}

fn count_visibility(grid: &Vec<Vec<i8>>, x: usize, y: usize) -> usize {
    let first = count_direction(y + 1..grid.len(), x..x + 1, grid, true, grid[y][x]);
    let second = count_direction((0..y).rev(), x..x + 1, grid, true, grid[y][x]);
    let third = count_direction(x + 1..grid[0].len(), y..y + 1, grid, false, grid[y][x]);
    let fourth = count_direction((0..x).rev(), y..y + 1, grid, false, grid[y][x]);
    first * second * third * fourth
}

fn main() {
    // read input from file
    let input = std::fs::read_to_string("data/day8.txt").unwrap();
    let input_data = input.split("\n");

    let mut forest: Vec<Vec<i8>> = Vec::new();

    for line in input_data {
        if line.len() == 0 {
            break;
        }
        forest.push(line.chars().map(|c| c.to_digit(10).unwrap() as i8).collect());
    }

    let height = forest.len();
    let width = forest[0].len();
    let mut seen = vec![vec![false; width]; height];

    check_direction(0..height, 0..width, &mut seen, &forest, true);
    check_direction(0..height, (0..width).rev().into_iter(), &mut seen, &forest, true);
    check_direction(0..width, (0..height).into_iter(), &mut seen, &forest, false);
    check_direction(0..width, (0..height).rev().into_iter(), &mut seen, &forest, false);

    println!("{}", seen.iter().map(|row| row.iter().filter(|&x| *x).count()).sum::<usize>());

    let mut max_visibility = 0;
    for y in 0..height {
        for x in 0..width {
            let visibility = count_visibility(&forest, x, y);
            max_visibility = std::cmp::max(max_visibility, visibility);
        }
    }

    println!("{}", max_visibility);
}