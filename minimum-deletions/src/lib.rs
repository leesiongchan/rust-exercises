fn gcd(a: i32, b: i32) -> i32 {
    if a == 0 {
        b
    } else {
        gcd(b % a, a)
    }
}

pub fn solve(total_numbers: i32, numbers: Vec<i32>) -> i32 {
    let mut check = -1;
    for nums in numbers.windows(2) {
        if gcd(nums[0], nums[1]) == 1 {
            check = 0;
        }
    }
    check
}
