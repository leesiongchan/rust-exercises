use std::collections::HashMap;
use std::io;

struct Date {
    year: u32,
    month: u32,
    day: u32,
}

fn solve(date: &str) {
    let date: Vec<_> = date.split(":").map(|d| d.parse::<u32>().unwrap()).collect();
    let date = Date {
        year: date[0],
        month: date[1],
        day: date[2],
    };
    let days_of_month = get_days_of_month(&date);
    let total_days = if days_of_month % 2 == 0 {
        days_of_month
            + get_days_of_month(&Date {
                month: get_next_month(date.month),
                ..date
            })
    } else {
        days_of_month
    };
    let total_pills = (total_days - date.day) / 2 + 1;

    println!("{}", total_pills);
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let tests = input.trim().parse::<usize>().unwrap();

    for _ in 0..tests {
        let mut date = String::new();
        io::stdin().read_line(&mut date).unwrap();
        solve(date.trim());
    }
}

fn get_days_of_month(date: &Date) -> u32 {
    let mut days_list: HashMap<u32, u32> = HashMap::new();
    days_list.insert(1, 31);
    days_list.insert(2, 28);
    days_list.insert(3, 31);
    days_list.insert(4, 30);
    days_list.insert(5, 31);
    days_list.insert(6, 30);
    days_list.insert(7, 31);
    days_list.insert(8, 31);
    days_list.insert(9, 30);
    days_list.insert(10, 31);
    days_list.insert(11, 30);
    days_list.insert(12, 31);

    if is_leap_year(date.year) {
        days_list.insert(2, 29);
    }

    *days_list.get(&date.month).unwrap()
}

fn get_next_month(month: u32) -> u32 {
    if month == 12 {
        1
    } else {
        month + 1
    }
}

fn is_leap_year(year: u32) -> bool {
    if year % 400 == 0 {
        return true;
    }
    if year % 100 == 0 {
        return false;
    }
    year % 4 == 0
}
