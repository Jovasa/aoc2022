fn main() {
    let input = std::fs::read_to_string("data/day10.txt").unwrap();

    let ops = input.strip_suffix("\n").unwrap().split("\n").collect::<Vec<&str>>();

    let mut total = 0;
    let mut executing = false;
    let mut pos = 0;

    let mut x = 1;

    for i in 1.. {
        if (i + 20) % 40 == 0{
            total += x * i;
            // println!("{} {} {} {} {} {}",i, x, x* (i), total, pos, executing);
        }
        let row_index = i % 40;
        if row_index - 2 <= x &&  x <= row_index {
            print!("#");
        }
        else {
            print!(".");
        }
        if row_index == 0 {
            println!();
        }
        if !executing {
            if ops[pos] != "noop" {
                executing = true;
            }
            else {
                pos += 1;
            }
        }
        else {
            let op = ops[pos].split(" ").collect::<Vec<&str>>();
            let arg = op[1].parse::<i32>().unwrap();
            x += arg;
            pos += 1;
            executing = false;
        }
        if pos == ops.len() {
            break;
        }
    }
    println!("{}", total);
}