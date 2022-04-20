use std::fs;

fn main() {
    let input = fs::read_to_string("../data/perm50e6.txt")
        .expect("Could not read file")
        .lines()
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    // println!("{}", input.len());
    // let input = vec![100, 32, 57, 10];
}

fn Cqsort<T>(s: &Vec<T>) {
    if s.len() <= 1 {
        return;
    }
}
