use std::fs;

fn main() {
    let input = fs::read_to_string("../data/perm50e3.txt")
        .expect("Could not read file")
        .lines()
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    println!("{}", input.len());
}
