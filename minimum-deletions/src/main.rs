use minimum_deletions::solve;
use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let tests = input.trim().parse::<usize>().unwrap();

    for _ in 0..tests {
        let mut total_numbers = String::new();
        let mut numbers = String::new();
        io::stdin().read_line(&mut total_numbers).unwrap();
        io::stdin().read_line(&mut numbers).unwrap();
        let total_numbers = total_numbers.trim().parse().unwrap();
        let numbers = numbers
            .trim()
            .split(" ")
            .map(|n| n.parse().unwrap())
            .collect();

        println!("{}", solve(total_numbers, numbers));
    }
}
