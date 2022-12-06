use itertools::Itertools;

fn main() {
    // Read the input file
    let input = std::fs::read_to_string("data/day6.txt").unwrap();
    find_code(&input, 4);
    find_code(&input, 14);
}

fn find_code(input: &str, amount: usize) {
    for (i, a) in input.chars().collect::<Vec<char>>().windows(amount).enumerate() {
        if !a.iter().permutations(2).fold(false, |n, o| n || o[0] == o[1]) {
            println!("{}", i + amount);
            break;
        }
    }
}