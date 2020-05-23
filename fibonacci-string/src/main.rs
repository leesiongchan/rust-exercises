use std::collections::HashMap;
use std::io;

fn solve(input: &str) {
    let mut map = HashMap::new();

    for i in input.chars() {
        let count = match map.get(&i) {
            Some(c) => c + 1,
            None => 1,
        };
        map.insert(i, count);
    }

    let mut is_dynamic = false;

    if map.keys().count() < 3 {
        is_dynamic = true;
    }

    let mut fib_nums: Vec<_> = map.iter().map(|(_, v)| *v).collect();
    fib_nums.sort();

    for nums in fib_nums.windows(3) {
        is_dynamic = nums[0] + nums[1] == nums[2];
    }

    println!("{}", if is_dynamic { "Dynamic" } else { "Not" });
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let tests = input.trim().parse::<usize>().unwrap();

    for _ in 0..tests {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        solve(input.trim());
    }
}
