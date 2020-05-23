use minimum_deletions::solve;

#[test]
fn simple() {
    let input = vec![2, 3];
    let expected = 0;
    assert_eq!(solve(input.len() as i32, input), expected);
}

#[test]
fn not_found() {
    let input = vec![2, 4];
    let expected = -1;
    assert_eq!(solve(input.len() as i32, input), expected);
}

#[test]
fn complex_1() {
    let input = vec![2, 4, 8, 15];
    let expected = 0;
    assert_eq!(solve(input.len() as i32, input), expected);
}

#[test]
fn complex_2() {
    let input = vec![21, 64, 86, 124];
    let expected = 0;
    assert_eq!(solve(input.len() as i32, input), expected);
}
