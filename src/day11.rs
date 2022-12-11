#[derive(PartialEq)]
enum Operation {
    Mul,
    Add,
}

struct Monkey {
    items: Vec<usize>,
    operation: Operation,
    operand: Option<usize>,
    test: usize,
    target: (usize, usize),
}

impl Monkey {
    fn new(d: &str) -> Monkey {
        let a = d.split("\n").collect::<Vec<&str>>();
        let items = a[1]
            .strip_prefix("  Starting items: ").unwrap()
            .split(", ")
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        let temp_row2 =  a[2].strip_prefix("  ").unwrap().split(" ").collect::<Vec<&str>>();
        let operation = match temp_row2[4] {
            "+" => Operation::Add,
            "*" => Operation::Mul,
            _ => unreachable!()
        };
        let operand = match temp_row2[5].parse::<usize>() {
            Ok(x) => Some(x),
            Err(_) => None,
        };
        let test = a[3].strip_prefix("  ").unwrap().split(" ").collect::<Vec<&str>>()[3].parse::<usize>().unwrap();
        let target = (
            a[4].strip_prefix("    ").unwrap().split(" ").collect::<Vec<&str>>()[5].parse::<usize>().unwrap(),
            a[5].strip_prefix("    ").unwrap().split(" ").collect::<Vec<&str>>()[5].parse::<usize>().unwrap()
        );
        return Monkey {
            items,
            operation,
            operand,
            test,
            target,
        };
    }
}


fn main() {
    let input = std::fs::read_to_string("data/day11.txt").unwrap();
    let input_data = input.split("\n\n");

    let mut monkeys = Vec::new();

    for item in input_data {
        monkeys.push(Monkey::new(item));
        if monkeys.len() == 8 {
            break;
        }
    }

    let mut thrown = vec![0; monkeys.len()];

    let modulo = monkeys.iter().map(|x| x.test).product::<usize>();

    // 20 for first one
    for _ in 0..10000 {
        for i in 0..monkeys.len() {
            thrown[i] += monkeys[i].items.len();
            let x1 = monkeys[i].items.clone();
            let target_true = monkeys[i].target.0;
            let target_false = monkeys[i].target.1;
            for item in x1 {
                let mut item = item;
                if monkeys[i].operation == Operation::Add {
                    item += monkeys[i].operand.unwrap();
                }
                else {
                    item *= match monkeys[i].operand {
                        Some(x) => x,
                        None => item,
                    };
                }
                // item /= 3; for first one instead of modulo
                item %= modulo;
                if item % monkeys[i].test == 0 {
                    monkeys[target_true].items.push(item);
                }
                else {
                    monkeys[target_false].items.push(item);
                }
            }
            monkeys[i].items.clear();
        }
    }
    thrown.sort();
    thrown.reverse();
    println!("{:?}", thrown.iter().take(2).fold(1, |acc, x| acc * x));
}