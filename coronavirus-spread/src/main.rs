use std::io;

fn solve(people: i32, distances: Vec<i32>) {
    // Finds breakpoints
    let mut breakpoints = vec![];

    let mut last_person = distances[0];
    for (idx, person) in distances.iter().enumerate() {
        if person - last_person > 2 {
            breakpoints.push(idx);
        }
        last_person = *person;
    }
    // We need to add a breakpoint at last
    breakpoints.push(distances.len());

    let mut infected_list = vec![];
    let mut last_breakpoint = 0;
    for idx in breakpoints {
        let infected_people = distances[last_breakpoint..idx].iter().count();
        infected_list.push(infected_people);
        last_breakpoint = idx;
    }

    let min = infected_list.iter().min().unwrap();
    let max = infected_list.iter().max().unwrap();

    println!("{} {}", min, max);
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let tests = input.trim().parse::<usize>().unwrap();

    for _ in 0..tests {
        let mut people = String::new();
        let mut distances = String::new();
        io::stdin().read_line(&mut people).unwrap();
        io::stdin().read_line(&mut distances).unwrap();
        let people: i32 = people.trim().parse().unwrap();
        let distances: Vec<i32> = distances
            .trim()
            .split(" ")
            .map(|d| d.parse::<i32>().unwrap())
            .collect();
        solve(people, distances);
    }
}
