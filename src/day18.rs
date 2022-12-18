use std::collections::HashSet;
use std::fs;
use itertools::Itertools;

fn main() {
    let input = fs::read_to_string("data/day18.txt").unwrap();

    let mut points = HashSet::new();

    let mut minimum_coords = (i32::MAX, i32::MAX, i32::MAX);
    let mut maximum_coords = (0, 0, 0);

    for line in input.lines() {
        let a: (i32, i32, i32) = line.split(",").map(|x| x.parse::<i32>().unwrap()).collect_tuple().unwrap();
        minimum_coords = (minimum_coords.0.min(a.0), minimum_coords.1.min(a.1), minimum_coords.2.min(a.2));
        maximum_coords = (maximum_coords.0.max(a.0), maximum_coords.1.max(a.1), maximum_coords.2.max(a.2));
        points.insert(a);
    }

    let offsets = vec![
        (1, 0, 0),
        (0, 1, 0),
        (0, 0, 1),
        (-1, 0, 0),
        (0, -1, 0),
        (0, 0, -1),
    ];

    let a = points.iter()
        .map(|(x, y, z)| {
            offsets.iter()
                .filter(|(dx, dy, dz)| !points.contains(&(x + dx, y + dy, z + dz)))
                .count()
        }).sum::<usize>();
    println!("{}", a);

    let mut filled_points = points.clone();
    for point in points.iter() {
        let mut outside = vec![false; 6];
        for (side, (dx, dy, dz)) in offsets.iter().enumerate() {
            let mut temp = point.clone();
            temp = (temp.0 + dx, temp.1 + dy, temp.2 + dz);
            for i in 1.. {
                if points.contains(&temp) {
                    break;
                }

                if temp.0 < minimum_coords.0 || temp.1 < minimum_coords.1 || temp.2 < minimum_coords.2 ||
                    temp.0 > maximum_coords.0 || temp.1 > maximum_coords.1 || temp.2 > maximum_coords.2 {
                    outside[side] = true;
                    break;
                }
                temp = (temp.0 + dx, temp.1 + dy, temp.2 + dz);
            }
        }
        if outside.iter().any(|x| !*x) {
            for (x, y) in outside.iter().enumerate() {
                if *y {continue}
                let off = offsets[x];
                let mut temp = point.clone();
                for _ in 1.. {
                    temp = (temp.0 + off.0, temp.1 + off.1, temp.2 + off.2);
                    if !filled_points.contains(&temp) {
                        println!("{} {} {}", temp.0, temp.1, temp.2);
                        filled_points.insert(temp);
                    } else {
                        break;
                    }
                }
            }
        }
    }

    let a = filled_points.iter()
        .map(|(x, y, z)| {
            offsets.iter()
                .filter(|(dx, dy, dz)| !filled_points.contains(&(x + dx, y + dy, z + dz)))
                .count()
        }).sum::<usize>();
    println!("{}", a);
}