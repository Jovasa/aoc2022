use std::fs;

fn main() {
    let input = fs::read_to_string("data/day17.txt").unwrap();
    let pattern = input.trim_end().chars().collect::<Vec<_>>();

    let mut pattern_iter = pattern.iter().clone().cycle();

    let mut depths = vec![0usize; 7];

    let shapes = vec![
        vec![0, 0, 1, 1, 1, 1, 0],
        vec![0, 0, 2, 1, 2, 0, 0],
        vec![0, 0, 1, 1, 1, 0, 0],
        vec![0, 0, 1, 0, 0, 0, 0],
        vec![0, 0, 1, 1, 0, 0, 0],
    ];

    let heights = vec![
        vec![0, 0, 1, 1, 1, 1, 0],
        vec![0, 0, 2, 3, 2, 0, 0],
        vec![0, 0, 1, 1, 3, 0, 0],
        vec![0, 0, 4, 0, 0, 0, 0],
        vec![0, 0, 2, 2, 0, 0, 0],
    ];

    let widths = vec![4, 3, 3, 1, 2];

    let mut shape_index = 0;

    for _ in 0..20 {
        let max_depth = depths.iter().max().unwrap();

        let mut shape_offset: i32 = 0;
        let mut height_offset = *max_depth + 3;
        loop {
            let direction = pattern_iter.next().unwrap();
            if direction == &'<' {
                if shape_offset != -2 && (depths[(shape_offset - 1 + 2) as usize] < height_offset || height_offset == 0) {
                    shape_offset -= 1;
                }
            } else {
                match shape_index {
                    0 => {
                        if shape_offset != 1 && depths[(shape_offset + widths[shape_index] + 2) as usize] <= height_offset {
                            shape_offset += 1;
                        }
                    }
                    1 | 2 => {
                        if shape_offset != 2 && depths[(shape_offset + widths[shape_index] + 2) as usize] <= height_offset {
                            shape_offset += 1;
                        }
                    }
                    3 => {
                        if shape_offset != 4 && depths[(shape_offset + widths[shape_index] + 2) as usize] <= height_offset {
                            shape_offset += 1;
                        }
                    }
                    4 => {
                        if shape_offset != 3 && depths[(shape_offset + widths[shape_index] + 2) as usize] <= height_offset {
                            shape_offset += 1;
                        }
                    }
                    _ => panic!(),
                }
            }
            if shapes[shape_index]
                .iter()
                // This skips first elements if the shape is to the left of the starting position
                .skip(if shape_offset < 0 { shape_offset.abs() } else { 0 } as usize)
                .zip(
                    depths.iter()
                        // This skips first depths if the shape is to the right of the starting position
                        .skip(shape_offset.max(0) as usize),
                )
                .any(|(shape_off, height)| {
                    println!("{} {}", shape_off, height);
                    *shape_off != 0 && (height + 1) as i32 - shape_off == height_offset as i32
                })
                || height_offset == 0 {
                for i in 0..shapes[shape_index].len() {
                    if i as i32 + shape_offset >= depths.len() as i32 || i as i32 + shape_offset < 0 {
                        continue;
                    }
                    if heights[shape_index][i] != 0 {
                        depths[(i as i32 + shape_offset) as usize] = height_offset + heights[shape_index][i];
                    }
                }
                break;
            }
            height_offset -= 1;
        }
        println!("{}: {:?}", shape_index, depths);
        shape_index = (shape_index + 1) % shapes.len();
    }

    println!("{}", depths.iter().max().unwrap());
}