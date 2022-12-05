use itertools::Itertools;

fn main() {
    let data = include_str!("../data/day5.txt");

    do_thing(data, true);
    do_thing(data, false);
}

fn do_thing(data: &str, create_mover_9000: bool) {
    let mut stacks: Vec<Vec<String>> = vec![Vec::new(); 9];

    for line in data.split("\n") {
        if line.len() == 0 {
            continue;
        }
        if line.starts_with(" 1") {
            for i in 0..stacks.len() {
                let stack = &stacks[i];
                stacks[i] = stack.into_iter().rev().map(|x| x.to_owned()).collect();
            }
        } else if line.starts_with("move") {
            let split = line.split(" ").collect::<Vec<&str>>();
            if create_mover_9000 {
                for _ in 0..split[1].parse::<u32>().unwrap() {
                    let a = stacks[split[3].parse::<usize>().unwrap() - 1].pop().unwrap();
                    stacks[split[5].parse::<usize>().unwrap() - 1].push(a);
                }
            }
            else {
                let mut temp = Vec::new();
                for _ in 0..split[1].parse::<u32>().unwrap() {
                    let a = stacks[split[3].parse::<usize>().unwrap() - 1].pop().unwrap();
                    temp.push(a)
                }
                stacks[split[5].parse::<usize>().unwrap() - 1].extend(temp.into_iter().rev());
            }
        } else {
            line.chars()
                .chunks(4)
                .into_iter()
                .enumerate()
                .for_each(|(i, c)| {
                    let string = c.take(3).collect::<String>();
                    if string != "   " {
                        stacks[i].push(string)
                    }
                });
        }
    }

    for stack in stacks.iter() {
        print!("{}", stack.iter().rev().next().unwrap().chars().skip_while(|c| !c.is_alphanumeric()).next().unwrap());
    }
    println!();
}